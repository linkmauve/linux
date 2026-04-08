// SPDX-License-Identifier: GPL-2.0 or MIT

// We don't expect that all the registers and fields will be used, even in the
// future.
//#![allow(dead_code)]

/*
use kernel::register;

register! {
    /// TODO
    pub(crate) STATUS(u16) @ 0x0 {
        /// Indicates status of Blitting Processor interrupt?
        4:4 breakpoint: bool;
        /// CP is not doing anything
        3:3 command_idle: bool;
        /// CP is not reading anything (??)
        2:2 read_idle: bool;
        /// FIFO underflow (Determined by watermark registers?)
        1:1 underflow_lo_watermark: bool;
        /// FIFO overflow
        0:0 overflow_hi_watermark: bool;
    }

    /// TODO
    pub(crate) CONTROL(u16) @ 0x2 {
        /// breakpoint enable (default 0)
        5:5 bpe: bool;
        /// FIFO link enable (YAGCD says CP->PE?) (default 1)
        4:4 gple: bool;
        /// FIFO underflow interrupt (default 0)
        3:3 uf_int: bool;
        /// FIFO overflow interrupt (default 1)
        2:2 of_int: bool;
        /// Command Processor interrupt (default 0)
        1:1 cp_int: bool;
        /// FIFO read enable (disable while setting up) (default 1)
        0:0 fifo_enable: bool;
    }

    /// TODO
    pub(crate) CLEAR(u16) @ 0x4 {
        /// Write 1 to clear metrics
        2:2 metrics: bool;
        /// Write 1 to clear FIFO underflow
        1:1 fifo_underflow: bool;
        /// Write 1 to clear FIFO overflow
        0:0 fifo_overflow: bool;
    }

    /// TODO
    pub(crate) PERF_SELECT(u16) @ 0x6 {
        /// TODO
        15:0    unknown;
    }

    /// TODO
    pub(crate) UNKNOWN_0A(u16) @ 0xa {
        /// TODO
        15:0    unknown;
    }

    /// This register contains the last token that was read from the FIFO.
    pub(crate) TOKEN(u16) @ 0xe {
        /// The token
        15:0    token;
    }

    pub(crate) FIFO_BASE_LO(u16) @ 0x20 {
        15:0    address;
    }

    pub(crate) FIFO_BASE_HI(u16) @ 0x22 {
        15:0    address;
    }

    pub(crate) FIFO_END_LO(u16) @ 0x24 {
        15:0    address;
    }

    pub(crate) FIFO_END_HI(u16) @ 0x26 {
        15:0    address;
    }

    pub(crate) FIFO_HI_WATERMARK_LO(u16) @ 0x28 {
        15:0    watermark;
    }

    pub(crate) FIFO_HI_WATERMARK_HI(u16) @ 0x2a {
        15:0    watermark;
    }

    pub(crate) FIFO_LO_WATERMARK_LO(u16) @ 0x2c {
        15:0    watermark;
    }

    pub(crate) FIFO_LO_WATERMARK_HI(u16) @ 0x2e {
        15:0    watermark;
    }

    pub(crate) FIFO_RW_DISTANCE_LO(u16) @ 0x30 {
        15:0    distance;
    }

    pub(crate) FIFO_RW_DISTANCE_HI(u16) @ 0x32 {
        15:0    distance;
    }

    pub(crate) FIFO_WRITE_POINTER_LO(u16) @ 0x34 {
        15:0    address;
    }

    pub(crate) FIFO_WRITE_POINTER_HI(u16) @ 0x36 {
        15:0    address;
    }

    pub(crate) FIFO_READ_POINTER_LO(u16) @ 0x38 {
        15:0    address;
    }

    pub(crate) FIFO_READ_POINTER_HI(u16) @ 0x3a {
        15:0    address;
    }

    pub(crate) FIFO_BP_LO(u16) @ 0x3c {
        15:0    address;
    }

    pub(crate) FIFO_BP_HI(u16) @ 0x3e {
        15:0    address;
    }
}
*/
