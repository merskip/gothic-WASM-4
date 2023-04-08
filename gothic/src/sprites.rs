use wasm4::sprite::Sprite;
use wasm4::geometry::Size;
use wasm4::sprite::Flags::*;

/// Player
pub const PLAYER_SPRITE: &Sprite = &Sprite::new(
    Size::new(12, 16),
    BLIT_2BPP,
    &[0xfe,0xaa,0xaf,0xf9,0x55,0x5b,0xe5,0x55,0x56,0x95,0x55,0x56,0x95,0x00,0x56,0xe0,0x82,0x0b,0xf8,0x82,0x2b,0xf8,0x00,0xaf,0xea,0x82,0xaf,0xa0,0x00,0x2a,0x82,0x00,0x82,0xea,0x00,0xab,0xfe,0x00,0xbf,0xfe,0x00,0xbf,0xfe,0x08,0xbf,0xfe,0xaa,0xbf],
);

/// King_Rhobar_2
pub const KING__RHOBAR_2_SPRITE: &Sprite = &Sprite::new(
    Size::new(16, 18),
    BLIT_1BPP,
    &[0x0d,0xb0,0x0a,0x50,0x18,0x18,0x38,0x1c,0x7f,0xfe,0x67,0xe6,0x60,0x06,0x6c,0x36,0x66,0x66,0x64,0x26,0x60,0x06,0x33,0xcc,0x4e,0x72,0x4d,0xb2,0x3d,0xbc,0x11,0x88,0x1f,0xf8,0x0e,0x70],
);
