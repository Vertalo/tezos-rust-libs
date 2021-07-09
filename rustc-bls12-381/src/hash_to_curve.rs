use std::convert::TryInto;

use super::writer::{write_uncompressed_g1, write_uncompressed_g2};
use super::{LENGTH_UNCOMPRESSED_G1_BYTES, LENGTH_UNCOMPRESSED_G2_BYTES};
use pairing_plus::bls12_381::{G1Affine, G1Uncompressed, G2Affine, G2Uncompressed, G1, G2};
use pairing_plus::hash_to_curve::HashToCurve;
use pairing_plus::hash_to_field::ExpandMsgXmd;
use pairing_plus::CurveProjective;
use pairing_plus::EncodedPoint;
use sha2::Sha256;

#[cfg(not(feature = "wasm"))]
use libc::c_uchar;

#[cfg(feature = "wasm")]
#[allow(non_camel_case_types)]
#[cfg(feature = "wasm")]
type c_uchar = u8;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

const MESSAGE_SIZE: usize = 512;
// 43 looks enough, it will one of those:
// BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_NUL_
// BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_
// BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_AUG_
// BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_AUG_
// BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_POP_
// BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_POP_
const DST_SIZE: usize = 48;

fn hash_to_curve<G>(
    dst: &[u8; DST_SIZE],
    message: &[u8; MESSAGE_SIZE],
    message_length: usize,
    dst_length: usize,
) -> G
where
    G: CurveProjective + HashToCurve<ExpandMsgXmd<Sha256>>,
{
    let message = &message[0..message_length];
    let dst = &dst[0..dst_length];
    G::hash_to_curve(message, dst)
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_hash_to_curve_g1(
    buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    message: *const [c_uchar; MESSAGE_SIZE],
    dst: *const [c_uchar; DST_SIZE],
    message_length: usize,
    dst_length: usize,
) {
    let message = unsafe { &*message };
    let dst = unsafe { &*dst };
    let message = &message[..MESSAGE_SIZE];
    let dst = &dst[..DST_SIZE];
    let res = hash_to_curve::<G1>(
        dst.try_into().unwrap(),
        message.try_into().unwrap(),
        message_length,
        dst_length,
    );
    let res = G1Affine::from(res);
    let res = G1Uncompressed::from_affine(res);
    write_uncompressed_g1(buffer, res)
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_hash_to_curve_g2(
    buffer: *mut [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    message: *const [c_uchar; MESSAGE_SIZE],
    dst: *const [c_uchar; DST_SIZE],
    message_length: usize,
    dst_length: usize,
) {
    let message = unsafe { &*message };
    let dst = unsafe { &*dst };
    let message = &message[..MESSAGE_SIZE];
    let dst = &dst[..DST_SIZE];
    let res = hash_to_curve::<G2>(
        dst.try_into().unwrap(),
        message.try_into().unwrap(),
        message_length,
        dst_length,
    );
    let res = G2Affine::from(res);
    let res = G2Uncompressed::from_affine(res);
    write_uncompressed_g2(buffer, res)
}
