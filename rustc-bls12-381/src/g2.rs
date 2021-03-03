use std::ops::Neg;

use bls12_381::{G2Affine, G2Projective, Scalar};
use group::Group;

use super::{LENGTH_FR_BYTES, LENGTH_UNCOMPRESSED_G2_BYTES};

use rand::rngs::OsRng;

#[cfg(not(feature = "wasm"))]
use libc::c_uchar;

#[cfg(feature = "wasm")]
#[allow(non_camel_case_types)]
#[cfg(feature = "wasm")]
type c_uchar = u8;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

fn write_affine_g2(buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES], point: G2Affine) {
    let buffer = unsafe { &mut *buffer };
    for (d, s) in buffer.iter_mut().zip(point.to_uncompressed().iter()) {
        *d = *s;
    }
}

// Check that a byte array represents a valid uncompressed point.
#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g2_check_bytes(
    g: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
) -> bool {
    let g = unsafe { &*g };
    G2Affine::from_uncompressed_unchecked(&g)
        .is_some()
        .unwrap_u8()
        == 1
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g2_zero(buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES]) {
    let g = G2Affine::identity();
    write_affine_g2(buffer, g);
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g2_one(buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES]) {
    let g = G2Affine::generator();
    write_affine_g2(buffer, g);
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g2_random(buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES]) {
    let g = G2Projective::random(&mut OsRng);
    let g = G2Affine::from(g);
    write_affine_g2(buffer, g);
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g2_add(
    buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g1: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
) {
    let g1 = unsafe { &*g1 };
    let g2 = unsafe { &*g2 };
    let g1 = G2Affine::from_uncompressed_unchecked(&g1).unwrap();
    let g2 = G2Affine::from_uncompressed_unchecked(&g2).unwrap();
    let res = G2Projective::from(g1) + G2Projective::from(g2);
    write_affine_g2(buffer, G2Affine::from(res));
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g2_negate(
    buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
) {
    let g = unsafe { &*g };
    let g = G2Affine::from_uncompressed_unchecked(g).unwrap();
    write_affine_g2(buffer, g.neg())
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g2_eq(
    g1: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
) -> bool {
    let g1 = unsafe { &*g1 };
    let g2 = unsafe { &*g2 };
    let g1 = G2Affine::from_uncompressed_unchecked(g1).unwrap();
    let g2 = G2Affine::from_uncompressed_unchecked(g2).unwrap();
    g1 == g2
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g2_is_zero(
    g: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
) -> bool {
    let g = unsafe { &*g };
    let g = G2Affine::from_uncompressed_unchecked(g).unwrap();
    g.is_identity().unwrap_u8() == 1
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g2_double(
    buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
) {
    let g = unsafe { &*g };
    let g = G2Affine::from_uncompressed_unchecked(g).unwrap();
    let g = G2Projective::from(g);
    let res = g.double();
    let res = G2Affine::from(res);
    write_affine_g2(buffer, res);
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g2_mul(
    buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    s: *const [c_uchar; LENGTH_FR_BYTES],
) {
    let g = unsafe { &*g };
    let s = unsafe { &*s };
    let g = G2Affine::from_uncompressed_unchecked(g).unwrap();
    let s = Scalar::from_bytes(s).unwrap();
    let res = g * s;
    write_affine_g2(buffer, G2Affine::from(res));
}
