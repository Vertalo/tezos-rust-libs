#[cfg(feature = "wasm")]
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub const LENGTH_FR_BYTES: usize = 32;
pub const LENGTH_FR_U64: usize = 4; // used for pow
pub const LENGTH_UNCOMPRESSED_G1_BYTES: usize = 96;
pub const LENGTH_UNCOMPRESSED_G2_BYTES: usize = 192;

pub mod fr;
pub mod g1;
pub mod g2;
pub mod pairing;
