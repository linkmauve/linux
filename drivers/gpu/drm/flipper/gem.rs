// SPDX-License-Identifier: GPL-2.0 or MIT

use crate::driver::{
    GxDevice,
    GxDriver, //
};
use kernel::{
    drm::gem,
    prelude::*, //
};

/// GEM Object inner driver data
#[pin_data]
pub(crate) struct GxObject {}

impl gem::DriverObject for GxObject {
    type Driver = GxDriver;

    fn new(_dev: &GxDevice, _size: usize) -> impl PinInit<Self, Error> {
        try_pin_init!(GxObject {})
    }
}
