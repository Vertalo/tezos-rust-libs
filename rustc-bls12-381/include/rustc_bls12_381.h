#ifndef RUSTC_BLS12_381_INCLUDE_H
#define RUSTC_BLS12_381_INCLUDE_H
#include <stdbool.h>

// Fr
extern bool rustc_bls12_381_fr_check_bytes(const unsigned char *element);

extern bool rustc_bls12_381_fr_is_zero(const unsigned char *element);

extern bool rustc_bls12_381_fr_is_one(const unsigned char *x);

extern void rustc_bls12_381_fr_zero(unsigned char *buffer);

extern void rustc_bls12_381_fr_one(unsigned char *buffer);

extern void rustc_bls12_381_fr_random(unsigned char *buffer);

extern void rustc_bls12_381_fr_add(unsigned char *buffer,
                                   const unsigned char *x,
                                   const unsigned char *y);

extern void rustc_bls12_381_fr_mul(unsigned char *buffer,
                                   const unsigned char *x,
                                   const unsigned char *y);

extern void rustc_bls12_381_fr_unsafe_inverse(unsigned char *buffer,
                                              const unsigned char *x);

extern void rustc_bls12_381_fr_negate(unsigned char *buffer,
                                      const unsigned char *x);

extern bool rustc_bls12_381_fr_eq(const unsigned char *x,
                                  const unsigned char *y);

extern void rustc_bls12_381_fr_square(unsigned char *buffer,
                                      const unsigned char *x);

extern void rustc_bls12_381_fr_double(unsigned char *buffer,
                                      const unsigned char *x);

extern void rustc_bls12_381_fr_pow(unsigned char *buffer,
                                   const unsigned char *x,
                                   const unsigned char *n);

// G1 uncompressed
extern bool rustc_bls12_381_g1_check_bytes(const unsigned char *element);

extern void rustc_bls12_381_g1_one(unsigned char *buffer);

extern void rustc_bls12_381_g1_zero(unsigned char *buffer);

extern void rustc_bls12_381_g1_random(unsigned char *element);

extern void rustc_bls12_381_g1_add(unsigned char *buffer,
                                   const unsigned char* g1,
                                   const unsigned char *g2);

extern void rustc_bls12_381_g1_negate(unsigned char *buffer,
                                      const unsigned char *g);

extern bool rustc_bls12_381_g1_eq(const unsigned char *g1,
                                  const unsigned char *g2);

extern bool rustc_bls12_381_g1_is_zero(const unsigned char *g);

extern void rustc_bls12_381_g1_double(unsigned char *buffer,
                                      const unsigned char *g);

extern void rustc_bls12_381_g1_mul(unsigned char *buffer,
                                   const unsigned char *g,
                                   const unsigned char *a);

// G2
extern bool rustc_bls12_381_g2_check_bytes(const unsigned char *element);

extern void rustc_bls12_381_g2_one(unsigned char *buffer);

extern void rustc_bls12_381_g2_zero(unsigned char *buffer);

extern void rustc_bls12_381_g2_random(unsigned char *element);

extern void rustc_bls12_381_g2_add(unsigned char *buffer,
                                   const unsigned char* g1,
                                   const unsigned char *g2);

extern void rustc_bls12_381_g2_negate(unsigned char *buffer,
                                      const unsigned char *g);

extern bool rustc_bls12_381_g2_eq(const unsigned char *g1,
                                  const unsigned char *g2);

extern bool rustc_bls12_381_g2_is_zero(const unsigned char *g);

extern void rustc_bls12_381_g2_double(unsigned char *buffer,
                                      const unsigned char *g);

extern void rustc_bls12_381_g2_mul(unsigned char *buffer,
                                  const unsigned char *g,
                                  const unsigned char *a);

// Pairing checks
extern bool rustc_bls12_381_pairing_check_1(const unsigned char *g1,
                                            const unsigned char *g2);

extern bool rustc_bls12_381_pairing_check_2(const unsigned char *g1_1,
                                            const unsigned char *g1_2,
                                            const unsigned char *g2_1,
                                            const unsigned char *g2_2);

extern bool rustc_bls12_381_pairing_check_3(
    const unsigned char *g1_1, const unsigned char *g1_2,
    const unsigned char *g1_3, const unsigned char *g2_1,
    const unsigned char *g2_2, const unsigned char *g2_3);

extern bool rustc_bls12_381_pairing_check_4(
    const unsigned char *g1_1, const unsigned char *g1_2,
    const unsigned char *g1_3, const unsigned char *g1_4, const unsigned char *g2_1,
    const unsigned char *g2_2, const unsigned char *g2_3, const unsigned char *g2_4);

extern bool rustc_bls12_381_pairing_check_5(
    const unsigned char *g1_1, const unsigned char *g1_2,
    const unsigned char *g1_3, const unsigned char *g1_4, const unsigned char *g1_5,
    const unsigned char *g2_1, const unsigned char *g2_2,
    const unsigned char *g2_3, const unsigned char *g2_4, const unsigned char *g2_5);

extern bool rustc_bls12_381_pairing_check_6(
    const unsigned char *g1_1, const unsigned char *g1_2,
    const unsigned char *g1_3, const unsigned char *g1_4,
    const unsigned char *g1_5, const unsigned char *g1_6, const unsigned char *g2_1,
    const unsigned char *g2_2, const unsigned char *g2_3,
    const unsigned char *g2_4, const unsigned char *g2_5, const unsigned char *g2_6);
#endif
