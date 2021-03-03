use bls12_381::Scalar;

use ff::Field;
use rand::rngs::OsRng;

use super::{LENGTH_FR_BYTES, LENGTH_FR_U64};
use std::convert::TryInto;

#[cfg(not(feature = "wasm"))]
use libc::c_uchar;

#[cfg(feature = "wasm")]
#[allow(non_camel_case_types)]
#[cfg(feature = "wasm")]
type c_uchar = u8;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

fn write_fr(buffer: *mut [c_uchar; LENGTH_FR_BYTES], element: Scalar) {
    let buffer: &mut [u8; LENGTH_FR_BYTES] = unsafe { &mut *buffer };
    for (d, s) in buffer.as_mut().iter_mut().zip(element.to_bytes().iter()) {
        *d = *s
    }
}

// Check that a byte array represents a valid point.
#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_check_bytes(
    element: *const [c_uchar; LENGTH_FR_BYTES],
) -> bool {
    let s = Scalar::from_bytes(unsafe { &*element });
    s.is_some().unwrap_u8() == 1
}

// From here, the parameters are supposed to be field elements.
#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_is_zero(element: *const [c_uchar; LENGTH_FR_BYTES]) -> bool {
    let s = Scalar::from_bytes(unsafe { &*element }).unwrap();
    s == Scalar::zero()
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_is_one(element: *const [c_uchar; LENGTH_FR_BYTES]) -> bool {
    let s = Scalar::from_bytes(unsafe { &*element }).unwrap();
    s == Scalar::one()
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_zero(buffer: *mut [c_uchar; LENGTH_FR_BYTES]) {
    let zero = Scalar::zero();
    write_fr(buffer, zero)
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_one(buffer: *mut [c_uchar; LENGTH_FR_BYTES]) {
    let one = Scalar::one();
    write_fr(buffer, one)
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_random(buffer: *mut [c_uchar; LENGTH_FR_BYTES]) {
    let random_x = Scalar::random(&mut OsRng);
    write_fr(buffer, random_x);
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_add(
    buffer: *mut [c_uchar; LENGTH_FR_BYTES],
    x: *const [c_uchar; LENGTH_FR_BYTES],
    y: *const [c_uchar; LENGTH_FR_BYTES],
) {
    let x = Scalar::from_bytes(unsafe { &*x }).unwrap();
    let y = Scalar::from_bytes(unsafe { &*y }).unwrap();
    let z = x + y;
    write_fr(buffer, z);
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_mul(
    buffer: *mut [c_uchar; LENGTH_FR_BYTES],
    x: *const [c_uchar; LENGTH_FR_BYTES],
    y: *const [c_uchar; LENGTH_FR_BYTES],
) {
    let x = Scalar::from_bytes(unsafe { &*x }).unwrap();
    let y = Scalar::from_bytes(unsafe { &*y }).unwrap();
    let z = x * y;
    write_fr(buffer, z);
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_eq(
    x: *const [c_uchar; LENGTH_FR_BYTES],
    y: *const [c_uchar; LENGTH_FR_BYTES],
) -> bool {
    let x = Scalar::from_bytes(unsafe { &*x }).unwrap();
    let y = Scalar::from_bytes(unsafe { &*y }).unwrap();
    x == y
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_unsafe_inverse(
    buffer: *mut [c_uchar; LENGTH_FR_BYTES],
    x: *const [c_uchar; LENGTH_FR_BYTES],
) {
    let x = Scalar::from_bytes(unsafe { &*x }).unwrap();
    let x = x.invert().unwrap();
    write_fr(buffer, x)
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_negate(
    buffer: *mut [c_uchar; LENGTH_FR_BYTES],
    x: *const [c_uchar; LENGTH_FR_BYTES],
) {
    let x = Scalar::from_bytes(unsafe { &*x }).unwrap();
    let x = x.neg();
    write_fr(buffer, x)
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_square(
    buffer: *mut [c_uchar; LENGTH_FR_BYTES],
    x: *const [c_uchar; LENGTH_FR_BYTES],
) {
    let x = Scalar::from_bytes(unsafe { &*x }).unwrap();
    let x = x.square();
    write_fr(buffer, x)
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_double(
    buffer: *mut [c_uchar; LENGTH_FR_BYTES],
    x: *const [c_uchar; LENGTH_FR_BYTES],
) {
    let x = Scalar::from_bytes(unsafe { &*x }).unwrap();
    let x = x.double();
    write_fr(buffer, x)
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_fr_pow(
    buffer: *mut [c_uchar; LENGTH_FR_BYTES],
    x: *const [c_uchar; LENGTH_FR_BYTES],
    n: *const [c_uchar; LENGTH_FR_BYTES],
) {
    let x = Scalar::from_bytes(unsafe { &*x }).unwrap();
    let n = unsafe { *n };
    let mut n_u64: [u64; LENGTH_FR_U64] = [0; LENGTH_FR_U64];
    for i in 0..LENGTH_FR_U64 {
        n_u64[i] = u64::from_le_bytes(n[i * 8..(i + 1) * 8].try_into().unwrap())
    }
    let res = x.pow(&n_u64);
    write_fr(buffer, res)
}
