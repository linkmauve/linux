// SPDX-License-Identifier: GPL-2.0 or MIT

use crate::impl_bounded;
use kernel::{
    io::Region,
    register, //
};

impl_bounded! {
    #[repr(u32)]
    #[bits(4)]
    enum TextureFormat {
        I4     = 0,
        I8     = 1,
        Ia4    = 2,
        Ia8    = 3,
        Rgb565 = 4,
        Rgb5a3 = 5,
        Rgba8  = 6,
        Ci4    = 8,
        Ci8    = 9,
        Ci14   = 10,

        /// Compressed
        Cmpr   = 14,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(4)]
    // The EFB Copy pipeline looks like:
    //
    //   1. Read EFB -> 2. Select color/depth -> 3. Downscale (optional)
    //   -> 4. YUV conversion (optional) -> 5. Encode Tiles -> 6. Write RAM
    //
    // The "Encode Tiles" stage receives RGBA8 texels from previous stages and encodes them to various
    // formats. EFBCopyFormat is the tile encoder mode. Note that the tile encoder does not care about
    // color vs. depth or intensity formats - it only sees RGBA8 texels.
    enum EfbCopyFormat {
        // These values represent EFB copy format in GX registers.
        // Most (but not all) of these values correspond to values of TextureFormat.
        R4 = 0,  // R4, I4, Z4

        // FIXME: Does 0x1 (Z8) have identical results to 0x8 (Z8H)?
        //        Is either or both of 0x1 and 0x8 used in games?
        R8_0x1 = 1,  // R8, I8, Z8H (?)

        Ra4 = 2,  // RA4, IA4

        // FIXME: Earlier versions of this file named the value 0x3 "GX_TF_Z16", which does not reflect
        //        the results one would expect when copying from the depth buffer with this format.
        //        For reference: When copying from the depth buffer, R should receive the top 8 bits of
        //                       the Z value, and A should be either 0xFF or 0 (please investigate).
        //        Please test original hardware and make sure dolphin-emu implements this format
        //        correctly.
        Ra8 = 3,  // RA8, IA8, (FIXME: Z16 too?)

        Rgb565 = 4,
        Rgb5A3 = 5,
        Rgba8 = 6,  // RGBA8, Z24
        A8 = 7,
        R8 = 8,   // R8, I8, Z8H
        G8 = 9,   // G8, Z8M
        B8 = 10,   // B8, Z8L
        Rg8 = 11,  // RG8, Z16R (Note: G and R are reversed)
        Gb8 = 12,  // GB8, Z16L

        // Special texture format used to represent YUVY xfb copies.
        // They aren't really textures, but they share so much hardware and usecases that it makes sense
        // to emulate them as part of texture cache.
        // This isn't a real value that can be used on console; it only exists for ease of implementation.
        Xfb = 15,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum TlutFormat {
        Ia8 = 0,
        Rgb565 = 1,
        Rgb5A3 = 2,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum TevScale {
        Scale1 = 0,
        Scale2 = 1,
        Scale4 = 2,
        Divide2 = 3,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(1)]
    enum TevOp {
        Add = 0,
        Sub = 1,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum TevCompareMode {
        R8 = 0,
        Gr16 = 1,
        Bgr24 = 2,
        Rgb8 = 3,
        // TODO: Allow one value to be defined more than once, perhaps.
        //A8 = 3,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(1)]
    enum TevComparison {
        Gt = 0,
        Eq = 1,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(4)]
    enum TevColorArg {
        PrevColor = 0,
        PrevAlpha = 1,
        Color0 = 2,
        Alpha0 = 3,
        Color1 = 4,
        Alpha1 = 5,
        Color2 = 6,
        Alpha2 = 7,
        TexColor = 8,
        TexAlpha = 9,
        RasColor = 10,
        RasAlpha = 11,
        One = 12,
        Half = 13,
        Konst = 14,
        Zero = 15,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(3)]
    enum TevAlphaArg {
        PrevAlpha = 0,
        Alpha0 = 1,
        Alpha1 = 2,
        Alpha2 = 3,
        TexAlpha = 4,
        RasAlpha = 5,
        Konst = 6,
        Zero = 7,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum TevOutput {
        Prev = 0,
        Color0 = 1,
        Color1 = 2,
        Color2 = 3,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum ZTexFormat {
        U8 = 0,
        U16 = 1,
        U24 = 2,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum ZTexOp {
        Disabled = 0,
        Add = 1,
        Replace = 2,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum TevBias {
        Zero = 0,
        AddHalf = 1,
        SubHalf = 2,
        Compare = 3,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum IndTexFormat {
        Itf8 = 0,
        Itf5 = 1,
        Itf4 = 2,
        Itf3 = 3,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(3)]
    enum IndTexBias {
        None = 0,
        S = 1,
        T = 2,
        St = 3,
        U = 4,
        Su = 5,
        Tu = 6,
        Stu = 7,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum IndMtxIndex {
        Off = 0,
        Matrix0 = 1,
        Matrix1 = 2,
        Matrix2 = 3,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum IndMtxId {
        Indirect = 0,
        S = 1,
        T = 2,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum IndTexBumpAlpha {
        Off = 0,
        S = 1,
        T = 2,
        U = 3,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(3)]
    enum IndTexWrap {
        ItwOff = 0,
        Itw256 = 1,
        Itw128 = 2,
        Itw64 = 3,
        Itw32 = 4,
        Itw16 = 5,
        Itw0 = 6,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(3)]
    enum RasColorChan {
        Color0 = 0,
        Color1 = 1,

        AlphaBump = 5,
        NormalizedAlphaBump = 6,
        Zero = 7,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum WrapMode {
        Clamp = 0,
        Repeat = 1,
        Mirror = 2,
        // Hardware testing indicates that WrapMode set to 3 behaves the same as clamp, though this is an
        // invalid value.
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum MipMode {
        None = 0,
        Point = 1,
        Linear = 2,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(1)]
    enum FilterMode {
        Nearest = 0,
        Linear = 1,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(1)]
    enum LodType {
        Edge = 0,
        Diagonal = 1,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum MaxAniso {
        One = 0,
        Two = 1,
        Four = 2,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum CullMode {
        /// Don’t cull any primitive
        None = 0,
        /// Cull back-facing primitives
        Back = 1,
        /// Cull front-facing primitives
        Front = 2,
        /// Cull all primitives
        All = 3,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(1)]
    enum AspectRatioAdjustment {
        DontAdjust = 0,
        Adjust = 1,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(3)]
    enum SrcBlendFactor {
        /// 0.0
        Zero        = 0,
        /// 1.0
        One         = 1,
        /// framebuffer color
        DstClr      = 2,
        /// 1.0 - (framebuffer color)
        InvDstClr   = 3,
        /// Source alpha
        SrcAlpha    = 4,
        /// 1.0 - (source alpha)
        InvSrcAlpha = 5,
        /// Framebuffer alpha
        DstAlpha    = 6,
        /// 1.0 - (framebuffer alpha)
        InvDstAlpha = 7,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(3)]
    enum DstBlendFactor {
        /// 0.0
        Zero        = 0,
        /// 1.0
        One         = 1,
        /// Source color
        SrcClr      = 2,
        /// 1.0 - (source color)
        InvSrcClr   = 3,
        /// Source alpha
        SrcAlpha    = 4,
        /// 1.0 - (source alpha)
        InvSrcAlpha = 5,
        /// Framebuffer alpha
        DstAlpha    = 6,
        /// 1.0 - (framebuffer alpha)
        InvDstAlpha = 7,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(4)]
    enum LogicOp {
        /// 0
        Clear   = 0,
        /// src & dst
        And     = 1,
        /// src & !dst
        RevAnd  = 2,
        /// src
        Copy    = 3,
        /// !src & dst
        InvAnd  = 4,
        /// dst
        Noop    = 5,
        /// src ^ dst
        Xor     = 6,
        /// src | dst
        Or      = 7,
        /// !(src | dst)
        Nor     = 8,
        /// !(src ^ dst)
        Equiv   = 9,
        /// !dst
        Inv     = 10,
        /// src | !dst
        RevOr   = 11,
        /// !src
        InvCopy = 12,
        /// !src | dst
        InvOr   = 13,
        /// !(src & dst)
        Nand    = 14,
        /// 1
        Set     = 15,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(1)]
    enum FogProjection {
        Perspective = 0,
        Orthographic = 1,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(3)]
    enum FogType {
        Off = 0,

        Linear = 2,

        Exp = 4,
        ExpSq = 5,
        BackwardsExp = 6,
        BackwardsExpSq = 7,
    }
}

impl_bounded! {
    #[repr(u32)]
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
    #[repr(u32)]
    #[bits(1)]
    enum FieldMaskState {
        Skip = 0,
        Write = 1,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(3)]
    #[expect(non_camel_case_types)]
    enum PixelFormat {
        Rgb8_Z24 = 0,
        Rgba6_Z24 = 1,
        Rgb565_Z16 = 2,
        Z24 = 3,
        Y8 = 4,
        U8 = 5,
        V8 = 6,
        Yuv420 = 7,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(3)]
    enum DepthFormat {
        Linear = 0,
        Near = 1,
        Mid = 2,
        Far = 3,

        // It seems these Z formats aren't supported/were removed ?
        InvLinear = 4,
        InvNear = 5,
        InvMid = 6,
        InvFar = 7,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(1)]
    enum TevRegType {
        Color = 0,
        Constant = 1,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum ColorChannel {
        Red = 0,
        Green = 1,
        Blue = 2,
        Alpha = 3,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(5)]
    #[expect(non_camel_case_types)]
    enum KonstSel {
        /// 1
        V1 = 0,
        /// 7/8
        V7_8 = 1,
        /// 3/4
        V3_4 = 2,
        /// 5/8
        V5_8 = 3,
        /// 1/2
        V1_2 = 4,
        /// 3/8
        V3_8 = 5,
        /// 1/4
        V1_4 = 6,
        /// 1/8
        V1_8 = 7,

        // 8-11 are invalid values that output 0 (8-15 for alpha)

        /// Konst 0 rgb (invalid for alpha)
        K0 = 12,
        /// Konst 1 rgb (invalid for alpha)
        K1 = 13,
        /// Konst 2 rgb (invalid for alpha)
        K2 = 14,
        /// Konst 3 rgb (invalid for alpha)
        K3 = 15,

        /// Konst 0 red
        K0_R = 16,
        /// Konst 1 red
        K1_R = 17,
        /// Konst 2 red
        K2_R = 18,
        /// Konst 3 red
        K3_R = 19,
        /// Konst 0 green
        K0_G = 20,
        /// Konst 1 green
        K1_G = 21,
        /// Konst 2 green
        K2_G = 22,
        /// Konst 3 green
        K3_G = 23,
        /// Konst 0 blue
        K0_B = 24,
        /// Konst 1 blue
        K1_B = 25,
        /// Konst 2 blue
        K2_B = 26,
        /// Konst 3 blue
        K3_B = 27,
        /// Konst 0 alpha
        K0_A = 28,
        /// Konst 1 alpha
        K1_A = 29,
        /// Konst 2 alpha
        K2_A = 30,
        /// Konst 3 alpha
        K3_A = 31,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum AlphaTestOp {
        And = 0,
        Or = 1,
        Xor = 2,
        Xnor = 3,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum FrameToField {
        Progressive = 0,
        InterlacedEven = 1,
        InterlacedOdd = 2,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(2)]
    enum GammaCorrection {
        Gamma1_0 = 0,
        Gamma1_7 = 1,
        Gamma2_2 = 2,
        // Hardware testing indicates this behaves the same as Gamma2_2
        Invalid2_2 = 3,
    }
}

impl_bounded! {
    #[repr(u32)]
    #[bits(3)]
    enum EmulatedZ {
        Disabled = 0,
        Early = 1,
        Late = 2,
        ForcedEarly = 3,
        EarlyWithFBFetch = 4,
        EarlyWithZComplocHack = 5,
    }
}

type u24 = u32;

register! {
    base: Region<0x100>;

    pub GEN_MODE(u24) @ 0x00 {
        3:0     num_tex_gens;
        7:4     num_chans;

        9:9     multisampling => bool;
        13:10   num_tev_stages_minus_1;
        15:14   cull_mode => CullMode;
        18:16   num_ind_stages;
        19:19   zfreeze => bool;
    }

    pub DISPLAY_COPY_FILTERS(u24)[4] @ 0x01 {
        // TODO: Figure out what the contents here mean!
        23:0    unknown;
    }

    pub IND_MATRIX(u24)[9] @ 0x06 {
        10:0    ma;
        21:11   mb;
        23:22   s0;
    }

    pub IND_I_MASK(u24) @ 0x0f {
        23:0    mask;
    }

    pub TEV_STAGE_INDIRECT(u24)[16] @ 0x10 {
        /// Indirect tex stage ID
        1:0     bt;
        3:2     format => IndTexFormat;
        6:4     bias => IndTexBias;
        8:7     bs => IndTexBumpAlpha;
        10:9    matrix_index => IndMtxIndex;
        12:11   matrix_id => IndMtxId;
        15:13   sw => IndTexWrap;
        18:16   tw => IndTexWrap;
        19:19   lb_utclod => bool;
        20:20   fb_addprev => bool;
    }

    pub SCISSOR_TOP_LEFT(u24) @ 0x20 {
        10:0    top_plus_0x156;
        22:12   left_plus_0x156;
    }

    pub SCISSOR_BOTTOM_RIGHT(u24) @ 0x21 {
        10:0    bottom_plus_0x155;
        22:12   right_plus_0x155;
    }

    pub LINE_POINT(u24) @ 0x22 {
        // In 1/6th of pixels.
        7:0     line_width => u8;
        // In 1/6th of pixels.
        15:8    point_size => u8;
        18:16   line_format;
        21:19   point_format;
        22:22   adjust_for_aspect_ratio => AspectRatioAdjustment;
    }

    pub PERF_0_TRIANGLE(u24) @ 0x23 {
        // TODO: Figure out what the contents here mean!
        23:0    unknown;
    }

    pub PERF_0_QUAD(u24) @ 0x24 {
        // TODO: Figure out what the contents here mean!
        23:0    unknown;
    }

    // TODO: Dolphin calls it RAS1_SS0
    pub IND_TEX_COOR_SCALE_1(u24) @ 0x25 {
        3:0     stage_0_scale_s;
        7:4     stage_0_scale_t;
        11:8    stage_1_scale_s;
        15:12   stage_1_scale_t;
    }

    // TODO: Dolphin calls it RAS1_SS1
    pub IND_TEX_COOR_SCALE_2(u24) @ 0x26 {
        3:0     stage_2_scale_s;
        7:4     stage_2_scale_t;
        11:8    stage_3_scale_s;
        15:12   stage_3_scale_t;
    }

    pub I_REF(u24) @ 0x27 {
        // TODO: Figure out what the contents here mean!
        23:0    unknown;
    }

    pub T_REF(u24)[8] @ 0x28 {
        2:0     even_texmap;
        5:3     even_texcoord;
        6:6     even_texmap_enabled => bool;
        9:7     even_colid => RasColorChan;

        14:12   odd_texmap;
        17:15   odd_texcoord;
        18:18   odd_texmap_enabled => bool;
        21:19   odd_colid => RasColorChan;
    }

    // TODO: Shouldn’t stride be 2 here?
    pub SU_S_SIZE(u24)[8, stride = 8] @ 0x30 {
        15:0    s_coord_scale_minus_1;
        16:16   s_coord_range_bias => bool;
        17:17   s_coord_cylindric_wrap => bool;
        18:18   use_line_offset => bool;
        19:19   use_point_offset => bool;
    }

    // TODO: Shouldn’t stride be 2 here?
    pub SU_T_SIZE(u24)[8, stride = 8] @ 0x31 {
        15:0    t_coord_scale_minus_1;
        16:16   t_coord_range_bias => bool;
        17:17   t_coord_cylindric_wrap => bool;
    }

    pub Z_MODE(u24) @ 0x40 {
        0:0 test_enable => bool;
        3:1 func => CompareMode;
        4:4 update_enable => bool;
    }

    pub BLEND_MODE(u24) @ 0x41 {
        0:0     enable => bool;
        1:1     logic_ops => bool;
        2:2     dither => bool;
        3:3     color_update => bool;
        4:4     alpha_update => bool;
        7:5     dest_factor => DstBlendFactor;
        10:8    src_factor => SrcBlendFactor;
        11:11   subtract => bool;
        15:12   logic_mode => LogicOp;
    }

    pub CONSTANT_ALPHA(u24) @ 0x42 {
        7:0     dst_alpha;
        8:8     enable => bool;
    }

    pub Z_COMPARE(u24) @ 0x43 {
        2:0 efb_pixel_format => PixelFormat;
        5:3 depth_format => DepthFormat;
        6:6 early_depth_dest => bool;
    }

    pub FIELD_MASK(u24) @ 0x44 {
        0:0 odd => FieldMaskState;
        1:1 even => FieldMaskState;
    }

    pub SET_DRAW_DONE(u24) @ 0x45 {
        // TODO: Figure out what the contents here mean!
        23:0    unknown;
    }

    pub BUS_CLOCK_0(u24) @ 0x46 {
        // TODO: Figure out what the contents here mean!
        23:0    unknown;
    }

    pub PE_TOKEN_ID(u24) @ 0x47 {
        15:0    token => u16;
    }

    pub PE_TOKEN_INT_ID(u24) @ 0x48 {
        15:0    token => u16;
    }

    pub COPY_TEX_SOURCE_TOP_LEFT(u24) @ 0x49 {
        9:0     top;
        19:10   left;
    }

    pub COPY_TEX_SOURCE_WIDTH_HEIGHT(u24) @ 0x4a {
        9:0     width_minus_1;
        19:10   height_minus_1;
    }

    pub COPY_TEX_DEST(u24) @ 0x4b {
        23:0    address_rshift_5;
    }

    pub COPY_TEX_DEST_STRIDE(u24) @ 0x4d {
        23:0    stride_rshift_5;
    }

    pub COPY_Y_SCALE(u24) @ 0x4e {
        // TODO: Figure out how this field is interpreted, the limits, mask, etc.
        23:0    scale;
    }

    pub CLEAR_AR(u24) @ 0x4f {
        7:0     red => u8;
        15:8    alpha => u8;
    }

    pub CLEAR_GB(u24) @ 0x50 {
        7:0     blue => u8;
        15:8    green => u8;
    }

    pub CLEAR_Z(u24) @ 0x51 {
        23:0    depth;
    }

    pub TRIGGER_EFB_COPY(u24) @ 0x52 {
        0:0     clamp_top => bool;
        1:1     clamp_bottom => bool;

        // realformat is (fmt/2)+((fmt&1)*8).... for some reason
        // the msb is the lsb (pattern: cycling right shift)
        6:3     target_pixel_format;
        8:7     gamma => GammaCorrection;
        9:9     half_scale => bool;
        10:10   scale_invert => bool;
        11:11   clear => bool;
        13:12   frame_to_field => FrameToField;
        14:14   copy_to_xfb => bool;
        15:15   intensity_fmt => bool;
        16:16   auto_conversion => bool;
    }

    pub COPY_FILTER_0(u24) @ 0x53 {
        5:0     w0;
        11:6    w1;
        17:12   w2;
        23:18   w3;
    }

    pub COPY_FILTER_1(u24) @ 0x54 {
        5:0     w4;
        11:6    w5;
        17:12   w6;
    }

    pub CLEAR_BBOX_1(u24) @ 0x55 {
        9:0     a;
        19:10   b;
    }

    pub CLEAR_BBOX_2(u24) @ 0x56 {
        9:0     c;
        19:10   d;
    }

    pub CLEAR_PIXEL_PERF(u24) @ 0x57 {
        // TODO: Figure out what the contents here mean!
        23:0    unknown;
    }

    pub REV_BITS(u24) @ 0x58 {
        // TODO: Figure out what the contents here mean!
        23:0    unknown;
    }

    pub SCISSOR_OFFSET(u24) @ 0x59 {
        9:0     x;
        19:10   y;
    }

    pub PRELOAD_ADDRESS(u24) @ 0x60 {
        23:0    addr;
    }

    pub PRELOAD_TMEM_EVEN(u24) @ 0x61 {
        23:0    addr;
    }

    pub PRELOAD_TMEM_ODD(u24) @ 0x62 {
        23:0    addr;
    }

    pub PRELOAD_MODE(u24) @ 0x63 {
        14:0    count_rshift_5;
        16:15   type_;
    }

    pub TLUT_SOURCE(u24) @ 0x64 {
        23:0    source;
    }

    pub TLUT_DEST(u24) @ 0x65 {
        9:0     tmem_address;
        20:10   tmem_line_count;
    }

    pub TEX_INVALIDATE(u24) @ 0x66 {
        // TODO: Figure out what the contents here mean!
        23:0    unknown;
    }

    pub PERF_1(u24) @ 0x67 {
        // TODO: Figure out what the contents here mean!
        23:0    unknown;
    }

    pub FIELD_MODE(u24) @ 0x68 {
        /// Adjust vertex tex LOD computation to account for interlacing.
        0:0 tex_lod => AspectRatioAdjustment;
    }

    pub BUS_CLOCK_1(u24) @ 0x69 {
        // TODO: Figure out what the contents here mean!
        23:0    unknown;
    }

    // TODO: All these duplicated at 0xa0

    pub TEX_SET_MODE_0(u24)[4] @ 0x80 {
        1:0     wrap_s => WrapMode;
        3:2     wrap_t => WrapMode;
        4:4     mag_filter => FilterMode;
        6:5     mipmap_filter => MipMode;
        7:7     min_filter => FilterMode;
        8:8     lod_type => LodType;
        18:9    load_bias;
        20:19   max_anisotropic_filtering => MaxAniso;
        21:21   lod_bias_clamp => bool;
    }

    pub TEX_SET_MODE_1(u24)[4] @ 0x84 {
        7:0     min_lod => u8;
        15:8    max_lod => u8;
    }

    pub TEX_SET_IMAGE_0(u24)[4] @ 0x88 {
        9:0     width_minus_1;
        19:10   height_minus_1;
        23:20   format => TextureFormat;
    }

    pub TEX_SET_IMAGE_1(u24)[4] @ 0x8c {
        14:0    tmem_even;
        17:15   cache_width;
        20:18   cache_height;
        21:21   cache_is_manually_managed => bool;
    }

    pub TEX_SET_IMAGE_2(u24)[4] @ 0x90 {
        14:0    tmem_odd;
        17:15   cache_width;
        20:18   cache_height;
    }

    pub TEX_SET_IMAGE_3(u24)[4] @ 0x94 {
        23:0    source_address_rshift_5;
    }

    pub TEX_SET_TLUT(u24)[4] @ 0x98 {
        9:0     tmem_offset;
        11:10   tlut_format => TlutFormat;
    }

    // TODO: Shouldn’t stride be 2 here?
    pub TEV_COLOR(u24)[16, stride = 8] @ 0xc0 {
        3:0     a => TevColorArg;
        7:4     b => TevColorArg;
        11:8    c => TevColorArg;
        15:12   d => TevColorArg;
        17:16   bias => TevBias;
        18:18   operation => TevOp;
        18:18   comparison => TevComparison;
        19:19   clamp => bool;
        21:20   scale => TevScale;
        21:20   compare_mode => TevCompareMode;
        23:22   dest => TevOutput;
    }

    // TODO: Shouldn’t stride be 2 here?
    pub TEV_ALPHA(u24)[16, stride = 8] @ 0xc1 {
        1:0     rswap;
        3:2     tswap;
        6:4     a => TevAlphaArg;
        9:7     b => TevAlphaArg;
        12:10   c => TevAlphaArg;
        15:13   d => TevAlphaArg;
        17:16   bias => TevBias;
        18:18   operation => TevOp;
        18:18   comparison => TevComparison;
        19:19   clamp => bool;
        21:20   scale => TevScale;
        21:20   compare_mode => TevCompareMode;
        23:22   dest => TevOutput;
    }

    pub TEV_COLOR_RA(u24)[4] @ 0xe0 {
        10:0    red;

        22:12   alpha;
        23:23   type_ => TevRegType;
    }

    pub TEV_COLOR_GB(u24)[4] @ 0xe1 {
        10:0    blue;

        22:12   green;
        23:23   type_ => TevRegType;
    }

    pub FOG_RANGE_BASE(u24) @ 0xe8 {
        9:0     center;
        10:10   enabled => bool;
    }

    pub FOG_RANGE_K(u24)[5] @ 0xe9 {
        11:0    hi;
        23:12   lo;
    }

    pub FOG_PARAM_0(u24) @ 0xee {
        10:0    mantissa;
        18:11   exponent;
        19:19   sign;
    }

    pub FOG_MAGNITUDE(u24) @ 0xef {
        23:0    magnitude;
    }

    pub FOG_EXPONENT(u24) @ 0xf0 {
        // b's exp + 1?
        23:0    shift;
    }

    pub FOG_PARAM_3(u24) @ 0xf1 {
        10:0    mantissa;
        18:11   exponent;
        19:19   sign;
        20:20   proj => FogProjection;
        23:21   fsel => FogType;
    }

    pub FOG_COLOR(u24) @ 0xf2 {
        7:0     blue => u8;
        15:8    green => u8;
        23:16   red => u8;
    }

    pub ALPHA_TEST(u24) @ 0xf3 {
        7:0     ref0 => u8;
        15:8    ref1 => u8;
        18:16   comp0 => CompareMode;
        21:19   comp1 => CompareMode;
        23:22   logic => AlphaTestOp;
    }

    pub BIAS(u24) @ 0xf4 {
        23:0    bias;
    }

    pub Z_TEX_2(u24) @ 0xf5 {
        1:0 type_ => ZTexFormat;
        3:2 op => ZTexOp;
    }

    pub TEV_K_SEL(u24)[8] @ 0xf6 {
        1:0     swap_rb => ColorChannel;
        3:2     swap_ga => ColorChannel;
        8:4     const_color_even => KonstSel;
        13:9    const_alpha_even => KonstSel;
        18:14   const_color_odd => KonstSel;
        23:19   const_alpha_odd => KonstSel;
    }

    /// Masks the next BP register write(s?).
    pub MASK(u24) @ 0xfe {
        /// Only the bits set in this register will be written on the next write.
        23:0    mask;
    }
}
