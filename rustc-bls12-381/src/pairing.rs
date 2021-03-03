use super::{LENGTH_UNCOMPRESSED_G1_BYTES, LENGTH_UNCOMPRESSED_G2_BYTES};
use bls12_381::multi_miller_loop;

#[cfg(not(feature = "wasm"))]
use libc::c_uchar;

#[cfg(feature = "wasm")]
#[allow(non_camel_case_types)]
#[cfg(feature = "wasm")]
type c_uchar = u8;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_pairing_check_1(
    g1: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g2: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
) -> bool {
    let g1 = unsafe { &*g1 };
    let g2 = unsafe { &*g2 };
    let g1 = bls12_381::G1Affine::from_uncompressed_unchecked(g1).unwrap();
    let g2 = bls12_381::G2Affine::from_uncompressed_unchecked(g2).unwrap();
    let g2 = bls12_381::G2Prepared::from(g2);
    let res = multi_miller_loop(&[(&g1, &g2)]).final_exponentiation();
    res == bls12_381::Gt::identity()
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_pairing_check_2(
    g1_1: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_2: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g2_1: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_2: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
) -> bool {
    let g1_1 = unsafe { &*g1_1 };
    let g1_2 = unsafe { &*g1_2 };
    let g2_1 = unsafe { &*g2_1 };
    let g2_2 = unsafe { &*g2_2 };
    let g1_1 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_1).unwrap();
    let g1_2 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_2).unwrap();
    let g2_1 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_1).unwrap();
    let g2_1 = bls12_381::G2Prepared::from(g2_1);
    let g2_2 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_2).unwrap();
    let g2_2 = bls12_381::G2Prepared::from(g2_2);
    let res = multi_miller_loop(&[(&g1_1, &g2_1), (&g1_2, &g2_2)]).final_exponentiation();
    res == bls12_381::Gt::identity()
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_pairing_check_3(
    g1_1: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_2: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_3: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g2_1: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_2: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_3: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
) -> bool {
    let g1_1 = unsafe { &*g1_1 };
    let g1_2 = unsafe { &*g1_2 };
    let g1_3 = unsafe { &*g1_3 };
    let g2_1 = unsafe { &*g2_1 };
    let g2_2 = unsafe { &*g2_2 };
    let g2_3 = unsafe { &*g2_3 };
    let g1_1 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_1).unwrap();
    let g1_2 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_2).unwrap();
    let g1_3 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_3).unwrap();
    let g2_1 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_1).unwrap();
    let g2_1 = bls12_381::G2Prepared::from(g2_1);
    let g2_2 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_2).unwrap();
    let g2_2 = bls12_381::G2Prepared::from(g2_2);
    let g2_3 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_3).unwrap();
    let g2_3 = bls12_381::G2Prepared::from(g2_3);
    let res =
        multi_miller_loop(&[(&g1_1, &g2_1), (&g1_2, &g2_2), (&g1_3, &g2_3)]).final_exponentiation();
    res == bls12_381::Gt::identity()
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_pairing_check_4(
    g1_1: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_2: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_3: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_4: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g2_1: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_2: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_3: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_4: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
) -> bool {
    let g1_1 = unsafe { &*g1_1 };
    let g1_2 = unsafe { &*g1_2 };
    let g1_3 = unsafe { &*g1_3 };
    let g1_4 = unsafe { &*g1_4 };
    let g2_1 = unsafe { &*g2_1 };
    let g2_2 = unsafe { &*g2_2 };
    let g2_3 = unsafe { &*g2_3 };
    let g2_4 = unsafe { &*g2_4 };
    let g1_1 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_1).unwrap();
    let g1_2 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_2).unwrap();
    let g1_3 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_3).unwrap();
    let g1_4 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_4).unwrap();
    let g2_1 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_1).unwrap();
    let g2_1 = bls12_381::G2Prepared::from(g2_1);
    let g2_2 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_2).unwrap();
    let g2_2 = bls12_381::G2Prepared::from(g2_2);
    let g2_3 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_3).unwrap();
    let g2_3 = bls12_381::G2Prepared::from(g2_3);
    let g2_4 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_4).unwrap();
    let g2_4 = bls12_381::G2Prepared::from(g2_4);
    let res = multi_miller_loop(&[
        (&g1_1, &g2_1),
        (&g1_2, &g2_2),
        (&g1_3, &g2_3),
        (&g1_4, &g2_4),
    ])
    .final_exponentiation();
    res == bls12_381::Gt::identity()
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_pairing_check_5(
    g1_1: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_2: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_3: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_4: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_5: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g2_1: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_2: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_3: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_4: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_5: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
) -> bool {
    let g1_1 = unsafe { &*g1_1 };
    let g1_2 = unsafe { &*g1_2 };
    let g1_3 = unsafe { &*g1_3 };
    let g1_4 = unsafe { &*g1_4 };
    let g1_5 = unsafe { &*g1_5 };
    let g2_1 = unsafe { &*g2_1 };
    let g2_2 = unsafe { &*g2_2 };
    let g2_3 = unsafe { &*g2_3 };
    let g2_4 = unsafe { &*g2_4 };
    let g2_5 = unsafe { &*g2_5 };
    let g1_1 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_1).unwrap();
    let g1_2 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_2).unwrap();
    let g1_3 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_3).unwrap();
    let g1_4 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_4).unwrap();
    let g1_5 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_5).unwrap();
    let g2_1 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_1).unwrap();
    let g2_1 = bls12_381::G2Prepared::from(g2_1);
    let g2_2 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_2).unwrap();
    let g2_2 = bls12_381::G2Prepared::from(g2_2);
    let g2_3 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_3).unwrap();
    let g2_3 = bls12_381::G2Prepared::from(g2_3);
    let g2_4 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_4).unwrap();
    let g2_4 = bls12_381::G2Prepared::from(g2_4);
    let g2_5 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_5).unwrap();
    let g2_5 = bls12_381::G2Prepared::from(g2_5);
    let res = multi_miller_loop(&[
        (&g1_1, &g2_1),
        (&g1_2, &g2_2),
        (&g1_3, &g2_3),
        (&g1_4, &g2_4),
        (&g1_5, &g2_5),
    ])
    .final_exponentiation();
    res == bls12_381::Gt::identity()
}

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn rustc_bls12_381_pairing_check_6(
    g1_1: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_2: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_3: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_4: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_5: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g1_6: *const [c_uchar; LENGTH_UNCOMPRESSED_G1_BYTES],
    g2_1: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_2: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_3: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_4: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_5: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
    g2_6: *const [c_uchar; LENGTH_UNCOMPRESSED_G2_BYTES],
) -> bool {
    let g1_1 = unsafe { &*g1_1 };
    let g1_2 = unsafe { &*g1_2 };
    let g1_3 = unsafe { &*g1_3 };
    let g1_4 = unsafe { &*g1_4 };
    let g1_5 = unsafe { &*g1_5 };
    let g1_6 = unsafe { &*g1_6 };
    let g2_1 = unsafe { &*g2_1 };
    let g2_2 = unsafe { &*g2_2 };
    let g2_3 = unsafe { &*g2_3 };
    let g2_4 = unsafe { &*g2_4 };
    let g2_5 = unsafe { &*g2_5 };
    let g2_6 = unsafe { &*g2_6 };
    let g1_1 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_1).unwrap();
    let g1_2 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_2).unwrap();
    let g1_3 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_3).unwrap();
    let g1_4 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_4).unwrap();
    let g1_5 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_5).unwrap();
    let g1_6 = bls12_381::G1Affine::from_uncompressed_unchecked(g1_6).unwrap();
    let g2_1 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_1).unwrap();
    let g2_1 = bls12_381::G2Prepared::from(g2_1);
    let g2_2 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_2).unwrap();
    let g2_2 = bls12_381::G2Prepared::from(g2_2);
    let g2_3 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_3).unwrap();
    let g2_3 = bls12_381::G2Prepared::from(g2_3);
    let g2_4 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_4).unwrap();
    let g2_4 = bls12_381::G2Prepared::from(g2_4);
    let g2_5 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_5).unwrap();
    let g2_5 = bls12_381::G2Prepared::from(g2_5);
    let g2_6 = bls12_381::G2Affine::from_uncompressed_unchecked(g2_6).unwrap();
    let g2_6 = bls12_381::G2Prepared::from(g2_6);
    let res = multi_miller_loop(&[
        (&g1_1, &g2_1),
        (&g1_2, &g2_2),
        (&g1_3, &g2_3),
        (&g1_4, &g2_4),
        (&g1_5, &g2_5),
        (&g1_6, &g2_6),
    ])
    .final_exponentiation();
    res == bls12_381::Gt::identity()
}
