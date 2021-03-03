use std::ops::Neg;

use bls12_381::{G1Affine, Scalar};
use group::Group;

use super::{LENGTH_FR_BYTES, LENGTH_UNCOMPRESSED_G1_BYTES};

use rand::rngs::OsRng;

#[cfg(not(feature = "wasm"))]
use libc::c_uchar;

#[cfg(feature = "wasm")]
#[allow(non_camel_case_types)]
#[cfg(feature = "wasm")]
type c_uchar = u8;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

fn write_affine_g1(buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES], point: G1Affine) {
    let buffer: &mut [u8; LENGTH_UNCOMPRESSED_G1_BYTES] = unsafe { &mut *buffer };
    for (d, s) in buffer.iter_mut().zip(point.to_uncompressed().iter()) {
        *d = *s;
    }
}

// Check that a byte array represents a valid uncompressed point.
#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g1_check_bytes(
    g: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
) -> bool {
    let g = unsafe { &*g };
    G1Affine::from_uncompressed_unchecked(&g)
        .is_some()
        .unwrap_u8()
        == 1
}

// Get the zero of the curve
#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g1_zero(buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES]) {
    let zero = bls12_381::G1Affine::identity();
    write_affine_g1(buffer, zero);
}

// Get a fixed generator of the curve
#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g1_one(buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES]) {
    let one = bls12_381::G1Affine::generator();
    write_affine_g1(buffer, one);
}

// Get a random point on the curve
#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g1_random(buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES]) {
    let g = bls12_381::G1Affine::from(bls12_381::G1Projective::random(&mut OsRng));
    write_affine_g1(buffer, g);
}

// Add two points of the curve.
// !! The two points must be valid. Undefined behaviors otherwise.
#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g1_add(
    buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g2: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
) {
    let g1 = unsafe { &*g1 };
    let g2 = unsafe { &*g2 };
    let g1 = bls12_381::G1Affine::from_uncompressed_unchecked(g1).unwrap();
    let g2 = bls12_381::G1Affine::from_uncompressed_unchecked(g2).unwrap();
    let res = bls12_381::G1Projective::from(g1) + bls12_381::G1Projective::from(g2);
    write_affine_g1(buffer, bls12_381::G1Affine::from(res));
}

// Compute the opposite of the point.
// !! The point must be valid. Undefined behaviors otherwise.
#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g1_negate(
    buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
) {
    let g = unsafe { &*g };
    let g = bls12_381::G1Affine::from_uncompressed_unchecked(g).unwrap();
    write_affine_g1(buffer, g.neg())
}

// Check if two points are algebraically equals
// !! The points must be valid. Undefined behaviors otherwise.
#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g1_eq(
    g1: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g2: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
) -> bool {
    let g1 = unsafe { &*g1 };
    let g2 = unsafe { &*g2 };
    let g1 = bls12_381::G1Affine::from_uncompressed_unchecked(g1).unwrap();
    let g2 = bls12_381::G1Affine::from_uncompressed_unchecked(g2).unwrap();
    g1 == g2
}

// Check if the given point is the zero of the curve
// !! The point must be valid. Undefined behaviors otherwise.
#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g1_is_zero(
    g: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
) -> bool {
    let g = unsafe { &*g };
    let g = bls12_381::G1Affine::from_uncompressed_unchecked(g).unwrap();
    g.is_identity().unwrap_u8() == 1
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g1_double(
    buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
) {
    let g = unsafe { &*g };
    let g = bls12_381::G1Affine::from_uncompressed_unchecked(g).unwrap();
    let g = bls12_381::G1Projective::from(g);
    let res = g.double();
    let res = bls12_381::G1Affine::from(res);
    write_affine_g1(buffer, res);
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_g1_mul(
    buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    s: *const [c_uchar; LENGTH_FR_BYTES],
) {
    let g = unsafe { &*g };
    let s = unsafe { &*s };
    let g = bls12_381::G1Affine::from_uncompressed_unchecked(g).unwrap();
    let s = Scalar::from_bytes(s).unwrap();
    let res = g * s;
    write_affine_g1(buffer, bls12_381::G1Affine::from(res));
}
