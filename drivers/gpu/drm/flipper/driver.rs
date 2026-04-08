// SPDX-License-Identifier: GPL-2.0 or MIT

use crate::{
    file::File,
    gem::GxObject,
    regs, //
};
use core::arch::asm;
use kernel::{
    device::Core,
    devres::Devres,
    drm,
    drm::ioctl,
    io::mem::IoMem,
    io::Io,
    of,
    platform,
    prelude::*,
    sync::aref::ARef,
    sync::Arc, //
};

/// Convenience type alias for the DRM device type for this driver.
pub(crate) type GxDevice = drm::Device<GxDriver>;

pub(crate) struct GxDriver {
    _device: ARef<GxDevice>,
}

#[pin_data]
pub(crate) struct GxData {
    pub(crate) pdev: ARef<platform::Device>,
    pub(crate) cp_regs: Arc<Devres<IoMem<256>>>,
    pub(crate) gx_regs: Arc<Devres<IoMem<256>>>,
    pub(crate) pi_regs: Arc<Devres<IoMem<256>>>,
    pub(crate) wg_pipe: Arc<Devres<IoMem<4>>>,
}

macro_rules! mtspr {
    ($R:tt, $v:tt) => {{
        let input: u32 = $v;
        unsafe { asm!(concat!("mtspr ", stringify!($R), ", {0}"), in(reg) input, options(nostack)) };
    }}
}

fn setup_fifo(
    pdev: &platform::Device<Core>,
    cp_regs: &Arc<Devres<IoMem<256>>>,
    pi_regs: &Arc<Devres<IoMem<256>>>,
    wg_pipe: &Arc<Devres<IoMem<4>>>,
) -> Result<u32> {
    // Hardcoded in the device-tree…
    mtspr!(921, 0x0c008000);
    const FIFO_START: u32 = 0x01734000;
    const FIFO_END: u32 = 0x01774000;
    const HI_WATERMARK: u32 = (FIFO_END - FIFO_START) * 3 / 4;

    let cp = cp_regs.access(pdev.as_ref())?;
    let pi = pi_regs.access(pdev.as_ref())?;
    let wg_pipe = wg_pipe.access(pdev.as_ref())?;

    // TODO: Replace those with register!() once it is in mainline.
    // FIFO Control
    cp.write16(0, 0x0);

    // FIFO Clear
    cp.write16(0x7, 0x4);

    // FIFO Start
    cp.write16((FIFO_START & 0xffff) as u16, 0x20);
    cp.write16((FIFO_START >> 16) as u16, 0x22);

    // FIFO End
    cp.write16((FIFO_END & 0xffff) as u16, 0x24);
    cp.write16((FIFO_END >> 16) as u16, 0x26);

    // Set watermarks, high at 75%, low at 0%
    cp.write16((HI_WATERMARK & 0xffff) as u16, 0x28);
    cp.write16((HI_WATERMARK >> 16) as u16, 0x2a);
    cp.write16(0, 0x2c);
    cp.write16(0, 0x2e);

    // Set R/W pointers to fifo start
    cp.write16(0, 0x30);
    cp.write16(0, 0x32);
    cp.write16((FIFO_START & 0xffff) as u16, 0x34);
    cp.write16((FIFO_START >> 16) as u16, 0x36);
    cp.write16((FIFO_START & 0xffff) as u16, 0x38);
    cp.write16((FIFO_START >> 16) as u16, 0x3a);

    // Set fifo bounds
    pi.write32(FIFO_START, 0xc);
    pi.write32(FIFO_END, 0x10);

    // Set write pointer
    pi.write32(FIFO_START, 0x14);
    for _ in 0..7 {
        wg_pipe.write32(0, 0);
    }
    wg_pipe.write16(0, 0);
    wg_pipe.write8(0, 0);
    pi.write32(FIFO_START, 0x14);

    // enable read & GP link
    cp.write16(17, 0x0);

    Ok(0)
}

kernel::of_device_table!(
    OF_TABLE,
    MODULE_OF_TABLE,
    <GxDriver as platform::Driver>::IdInfo,
    [
        (of::DeviceId::new(c"nintendo,hollywood-gx"), ()),
        (of::DeviceId::new(c"nintendo,flipper-gx"), ())
    ]
);

impl platform::Driver for GxDriver {
    type IdInfo = ();
    const OF_ID_TABLE: Option<of::IdTable<Self::IdInfo>> = Some(&OF_TABLE);

    fn probe(
        pdev: &platform::Device<Core>,
        _info: Option<&Self::IdInfo>,
    ) -> impl PinInit<Self, Error> {
        let request = pdev.io_request_by_index(0).ok_or(ENODEV)?;
        let cp_regs = Arc::pin_init(request.iomap_sized::<256>(), GFP_KERNEL)?;
        let request = pdev.io_request_by_index(1).ok_or(ENODEV)?;
        let gx_regs = Arc::pin_init(request.iomap_sized::<256>(), GFP_KERNEL)?;
        let request = pdev.io_request_by_index(2).ok_or(ENODEV)?;
        let pi_regs = Arc::pin_init(request.iomap_sized::<256>(), GFP_KERNEL)?;
        let request = pdev.io_request_by_index(3).ok_or(ENODEV)?;
        let wg_pipe = Arc::pin_init(request.iomap_sized::<4>(), GFP_KERNEL)?;

        setup_fifo(pdev, &cp_regs, &pi_regs, &wg_pipe)?;

        let platform: ARef<platform::Device> = pdev.into();

        let data = try_pin_init!(GxData {
            pdev: platform.clone(),
            cp_regs,
            gx_regs,
            pi_regs,
            wg_pipe,
        });

        let tdev: ARef<GxDevice> = drm::Device::new(pdev.as_ref(), data)?;
        drm::driver::Registration::new_foreign_owned(&tdev, pdev.as_ref(), 0)?;

        Ok(GxDriver { _device: tdev })
    }
}

const INFO: drm::DriverInfo = drm::DriverInfo {
    major: 0,
    minor: 1,
    patchlevel: 0,
    name: c"flipper",
    desc: c"Nintendo Flipper DRM driver",
};

#[vtable]
impl drm::Driver for GxDriver {
    type Data = GxData;
    type File = File;
    type Object = drm::gem::Object<GxObject>;

    const INFO: drm::DriverInfo = INFO;

    kernel::declare_drm_ioctls! {
        (FLIPPER_WRITE_MEM, drm_flipper_write_mem, ioctl::RENDER_ALLOW, File::write_mem),
        (FLIPPER_RUN_CMDS, drm_flipper_run_cmds, ioctl::RENDER_ALLOW, File::run_cmds),
    }
}
