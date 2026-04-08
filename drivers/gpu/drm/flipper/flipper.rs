// SPDX-License-Identifier: GPL-2.0 or MIT

//! Nintendo Flipper DRM driver.

use crate::driver::GxDriver;

mod driver;
mod file;
mod gem;
mod regs;

kernel::module_platform_driver! {
    type: GxDriver,
    name: "flipper",
    authors: ["Link Mauve <linkmauve@linkmauve.fr>"],
    description: "Nintendo Flipper DRM driver",
    license: "Dual MIT/GPL",
}
