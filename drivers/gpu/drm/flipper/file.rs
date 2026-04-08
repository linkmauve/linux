// SPDX-License-Identifier: GPL-2.0 or MIT

use crate::driver::{
    GxDevice,
    GxDriver, //
};
use kernel::{
    drm,
    io::Io,
    prelude::*,
    uaccess::UserSlice,
    uapi, //
};

#[pin_data]
pub(crate) struct File {}

/// Convenience type alias for our DRM `File` type
pub(crate) type DrmFile = drm::file::File<File>;

impl drm::file::DriverFile for File {
    type Driver = GxDriver;

    fn open(_dev: &drm::Device<Self::Driver>) -> Result<Pin<KBox<Self>>> {
        KBox::try_pin_init(try_pin_init!(Self {}), GFP_KERNEL)
    }
}

impl File {
    pub(crate) fn write_mem(
        tdev: &GxDevice,
        cmd: &mut uapi::drm_flipper_write_mem,
        _file: &DrmFile,
    ) -> Result<u32> {
        //dev_info!(tdev, "{cmd:?}");
        let pointer = cmd.pointer as usize;
        let size = cmd.size as usize;
        let mut reader = UserSlice::new(UserPtr::from_addr(pointer), size).reader();
        dev_info!(
            tdev,
            "Writing memory {} from 0x{pointer:08x} for {size}!\n",
            cmd.mem_type
        );

        let wg_pipe = tdev.wg_pipe.access(unsafe { tdev.as_ref().as_bound() })?;

        match cmd.mem_type {
            uapi::mem_type_BP => {
                for reg in 0..size >> 2 {
                    // For some reason, this format stores data in little-endian…
                    let mut value = reader.read::<u32>()?.swap_bytes();
                    if (value & 0xff000000) != 0 {
                        return Err(EINVAL);
                    }
                    if reg == 0x4b {
                        // Patching the XFB address.
                        dev_info!(tdev, "Got XFB set to 0x{:08x}, patching!\n", value << 5);
                        value = 0x0177_4000 >> 5;
                    }
                    dev_info!(tdev, "BP reg 0x{reg:02x} to 0x{value:06x}\n");
                    /*
                    let data = ((reg as u32) << 24) | value;
                    wg_pipe.write8(0x61, 0x0);
                    wg_pipe.write32(data, 0x0);
                    */
                }
                //dev_info!(tdev, "Wrote everything!\n");

                Ok(0)
            }
            uapi::mem_type_CP => {
                for reg in 0..size >> 2 {
                    let value = reader.read::<u32>()?.swap_bytes();
                    dev_info!(tdev, "CP reg 0x{reg:02x} to 0x{value:08x}\n");
                    /*
                    wg_pipe.write8(0x08, 0x0);
                    wg_pipe.write8(reg as u8, 0x0);
                    wg_pipe.write32(value, 0x0);
                    */
                }
                //dev_info!(tdev, "Wrote everything!\n");

                Ok(0)
            }
            uapi::mem_type_XF => {
                /*
                wg_pipe.write8(0x10, 0x0);
                wg_pipe.write16((size >> 2) as u16 - 1, 0x0);
                wg_pipe.write16(0, 0x0);
                */
                for reg in 0..size >> 2 {
                    let value = reader.read::<u32>()?.swap_bytes();
                    //wg_pipe.write32(value, 0x0);
                    dev_info!(tdev, "XF reg 0x{reg:04x} to 0x{value:08x}\n");
                }
                //dev_info!(tdev, "Wrote everything!\n");

                Ok(0)
            }
            uapi::mem_type_XF_REGS => {
                /*
                wg_pipe.write8(0x10, 0x0);
                wg_pipe.write16((size >> 2) as u16 - 1, 0x0);
                wg_pipe.write16(0x1000_u16, 0x0);
                */
                for reg in 0x1000..0x1000 + (size >> 2) {
                    let value = reader.read::<u32>()?.swap_bytes();
                    //wg_pipe.write32(value, 0x0);
                    dev_info!(tdev, "XF reg 0x{reg:04x} to 0x{value:08x}\n");
                }
                //dev_info!(tdev, "Wrote everything!\n");

                Ok(0)
            }
            uapi::mem_type_TEX_MEM => {
                dev_info!(tdev, "Would write texture memory… but doesn’t!\n");

                Ok(0)
            }
            _ => Err(EINVAL),
        }
    }

    pub(crate) fn run_cmds(
        tdev: &GxDevice,
        cmd_list: &mut uapi::drm_flipper_run_cmds,
        _file: &DrmFile,
    ) -> Result<u32> {
        let cmds_len = cmd_list.cmds_len as usize;
        //dev_info!(tdev, "Got {cmds_len} commands: {cmd_list:?}\n");
        dev_info!(tdev, "Got {cmds_len} commands\n");

        let wg_pipe = tdev.wg_pipe.access(unsafe { tdev.as_ref().as_bound() })?;

        for i in 0..cmds_len {
            let cmd = cmd_list.cmds[i];
            //dev_info!(tdev, "{cmd:?}");

            match cmd.command {
                uapi::command_FIFO => {
                    let mut reader =
                        UserSlice::new(UserPtr::from_addr(cmd.pointer as usize), cmd.size as usize)
                            .reader();

                    loop {
                        let command: u8 = match reader.read() {
                            Ok(command) => command,
                            Err(_) => break,
                        };
                        match command {
                            // NOP
                            0x00 => {
                                dev_info!(tdev, "FIFO: NOP\n");
                                //wg_pipe.write8(command, 0x0);
                            }
                            // XF
                            0x10 => {
                                let num: u16 = reader.read()?;
                                let reg: u16 = reader.read()?;
                                dev_info!(
                                    tdev,
                                    "FIFO: Running {} XF commands, starting at {reg:04x}\n",
                                    num + 1
                                );
                                //wg_pipe.write8(command, 0x0);
                                //wg_pipe.write16(num, 0x0);
                                //wg_pipe.write16(reg, 0x0);
                                for i in 0..num + 1 {
                                    let data: u32 = reader.read()?;
                                    //wg_pipe.write32(data, 0x0);
                                }
                            }
                            // BP
                            0x48 => {
                                dev_info!(tdev, "FIFO: Invalidate vertex cache\n");
                                //wg_pipe.write8(command, 0x0);
                            }
                            // BP
                            0x61 => {
                                let data: u32 = reader.read()?;
                                let reg = data >> 24;
                                let mut value = data & 0x00_ffffff;
                                if reg == 0x4b {
                                    value = 0x17740000 >> 5;
                                    dev_info!(tdev, "Patching BP command 0x4b!\n");
                                }
                                dev_info!(
                                    tdev,
                                    "FIFO: Running BP command {reg:02x} with value 0x{value:06x}\n"
                                );
                                //wg_pipe.write8(command, 0x0);
                                //wg_pipe.write32((reg << 24) | value, 0x0);
                            }
                            0x90 => {
                                let len: u16 = reader.read()?;
                                //wg_pipe.write8(command, 0x0);
                                //wg_pipe.write16(len, 0x0);
                                dev_info!(tdev, "FIFO: Draw triangles ({len} vertices)");
                                for i in 0..len {
                                    let a: u8 = reader.read()?;
                                    let b: u8 = reader.read()?;
                                    dev_info!(tdev, "{a:02x} {b:02x}");
                                    //wg_pipe.write8(a, 0x0);
                                    //wg_pipe.write8(b, 0x0);
                                }
                            }
                            _ => {
                                dev_err!(
                                    tdev,
                                    "FIFO: Warning: Running unknown command {command:02x}\n"
                                );
                                //wg_pipe.write8(command, 0x0);
                            }
                        }
                    }
                }
                uapi::command_UPDATE_VERTEX => {
                    let reader =
                        UserSlice::new(UserPtr::from_addr(cmd.pointer as usize), cmd.size as usize)
                            .reader();
                    //let mut data = KVec::new();
                    //reader.read_all(&mut data, GFP_KERNEL)?;
                    //dev_info!(tdev, "Running command {} with data {data:?}\n", cmd.command);
                }
                _ => return Err(EINVAL),
            }
        }

        Ok(0)
    }
}
