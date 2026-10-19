// SPDX-License-Identifier: GPL-2.0 or MIT

// We don't expect that all the registers and fields will be used, even in the
// future.
//#![allow(dead_code)]

use crate::impl_bounded;
use kernel::{
    io::Region,
    register, //
};

impl_bounded! {
    #[repr(u16)]
    #[bits(3)]
    enum CompareMode {
        Never = 0,
        Less = 1,
        Equal = 2,
        LEqual = 3,
        Greater = 4,
        NEqual = 5,
        GEqual = 6,
        Always = 7,
    }
}

impl_bounded! {
    #[repr(u16)]
    #[bits(3)]
    enum SrcBlendFactor {
        Zero = 0,
        One = 1,
        DstClr = 2,
        InvDstClr = 3,
        SrcAlpha = 4,
        InvSrcAlpha = 5,
        DstAlpha = 6,
        InvDstAlpha = 7,
    }
}

impl_bounded! {
    #[repr(u16)]
    #[bits(3)]
    enum DstBlendFactor {
        Zero = 0,
        One = 1,
        SrcClr = 2,
        InvSrcClr = 3,
        SrcAlpha = 4,
        InvSrcAlpha = 5,
        DstAlpha = 6,
        InvDstAlpha = 7,
    }
}

impl_bounded! {
    #[repr(u16)]
    #[bits(4)]
    enum LogicOp {
        Clear = 0,
        And = 1,
        AndReverse = 2,
        Copy = 3,
        AndInverted = 4,
        NoOp = 5,
        Xor = 6,
        Or = 7,
        Nor = 8,
        Equiv = 9,
        Invert = 10,
        OrReverse = 11,
        CopyInverted = 12,
        OrInverted = 13,
        Nand = 14,
        Set = 15,
    }
}

impl_bounded! {
    #[repr(u16)]
    #[bits(2)]
    enum AlphaReadMode {
        /// Always read 0x00.
        Read00 = 0,

        /// Always read 0xFF.
        ReadFF = 1,

        /// Always read the real alpha value
        ReadNone = 2,

        /// TODO
        Unknown = 3,
    }
}

register! {
    base: Region<0x100>;

    /// Depth configuration
    pub(crate) ZCONF(u16) @ 0x00 {
        4:4 z_update_enable => bool;
        3:1 function => CompareMode;
        0:0 z_comparator_enable => bool;
    }

    /// Alpha configuration
    pub(crate) ALPHA_CONF(u16) @ 0x02 {
        15:12   logic_op => LogicOp;
        11:11   subtract => bool;
        10:8    src_factor => SrcBlendFactor;
        7:5     dst_factor => DstBlendFactor;
        4:4     alpha_update_enable => bool;
        3:3     color_update_enable => bool;
        2:2     dither => bool;
        1:1     logic => bool;
        0:0     blend => bool;
    }

    /// TODO
    pub(crate) DST_ALPHA_CONF(u16) @ 0x04 {
        8:8 enable => bool;
        7:0 alpha => u8;
    }

    /// Alpha mode configuration
    pub(crate) ALPHA_MODE(u16) @ 0x06 {
        10:8    compare_mode => CompareMode;
        7:0     threshold => u8;
    }

    /// Alpha read
    pub(crate) ALPHA_READ(u16) @ 0x08 {
        1:0 read_mode => AlphaReadMode;
        2:2 unknown => bool;
    }

    /// TODO
    pub(crate) CONTROL(u16) @ 0x0a {
        3:3 finish => bool;
        2:2 token => bool;
        1:1 finish_enable => bool;
        0:0 token_enable => bool;
    }

    // TODO: Figure out if there is a register 0x0c.

    /// TODO
    pub(crate) TOKEN(u16) @ 0x0e {
        15:0    token;
    }

    /// The leftmost pixel drawn since the previous bounding-box clear.
    ///
    /// Since the PE works only on 2×2 blocks, this value is always even.
    pub(crate) BBOX_LEFT(u16) @ 0x10 {
        9:0 bbox;
    }

    /// The rightmost pixel drawn since the previous bounding-box clear.
    ///
    /// Since the PE works only on 2×2 blocks, this value is always even.
    pub(crate) BBOX_RIGHT(u16) @ 0x12 {
        9:0 bbox;
    }

    /// The topmost pixel drawn since the previous bounding-box clear.
    ///
    /// Since the PE works only on 2×2 blocks, this value is always odd.
    pub(crate) BBOX_TOP(u16) @ 0x14 {
        9:0 bbox;
    }

    /// The bottommost pixel drawn since the previous bounding-box clear.
    ///
    /// Since the PE works only on 2×2 blocks, this value is always odd.
    pub(crate) BBOX_BOTTOM(u16) @ 0x16 {
        9:0 bbox;
    }

    // TODO: Add the remaining perf registers.
}
