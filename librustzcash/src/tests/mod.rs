use zcash_primitives::jubjub::{FixedGenerators, JubjubParams};

use super::JUBJUB;

mod key_agreement;
mod key_components;
mod mmr;
mod notes;
mod signatures;

#[test]
fn sapling_generators() {
    struct SaplingGenerators {
        skb: [u8; 32],
        pkb: [u8; 32],
        npb: [u8; 32],
        wprb: [u8; 32],
        vcvb: [u8; 32],
        vcrb: [u8; 32],
    };

    // From https://github.com/zcash-hackworks/zcash-test-vectors/blob/master/sapling_generators.py
    let sapling_generators = SaplingGenerators {
        skb: [
            0x30, 0xb5, 0xf2, 0xaa, 0xad, 0x32, 0x56, 0x30, 0xbc, 0xdd, 0xdb, 0xce, 0x4d, 0x67,
            0x65, 0x6d, 0x05, 0xfd, 0x1c, 0xc2, 0xd0, 0x37, 0xbb, 0x53, 0x75, 0xb6, 0xe9, 0x6d,
            0x9e, 0x01, 0xa1, 0xd7,
        ],
        pkb: [
            0xe7, 0xe8, 0x5d, 0xe0, 0xf7, 0xf9, 0x7a, 0x46, 0xd2, 0x49, 0xa1, 0xf5, 0xea, 0x51,
            0xdf, 0x50, 0xcc, 0x48, 0x49, 0x0f, 0x84, 0x01, 0xc9, 0xde, 0x7a, 0x2a, 0xdf, 0x18,
            0x07, 0xd1, 0xb6, 0xd4,
        ],
        npb: [
            0x65, 0x00, 0x2b, 0xc7, 0x36, 0xfa, 0xf7, 0xa3, 0x42, 0x2e, 0xff, 0xff, 0xe8, 0xb8,
            0x55, 0xe1, 0x8f, 0xba, 0x96, 0xa0, 0x15, 0x8a, 0x9e, 0xfc, 0xa5, 0x84, 0xbf, 0x40,
            0x54, 0x9d, 0x36, 0xe1,
        ],
        wprb: [
            0xac, 0x77, 0x6c, 0x79, 0x65, 0x63, 0xfc, 0xd4, 0x4c, 0xc4, 0x9c, 0xfa, 0xea, 0x8b,
            0xb7, 0x96, 0x95, 0x2c, 0x26, 0x6e, 0x47, 0x77, 0x9d, 0x94, 0x57, 0x4c, 0x10, 0xad,
            0x01, 0x75, 0x4b, 0x11,
        ],
        vcvb: [
            0xd7, 0xc8, 0x67, 0x06, 0xf5, 0x81, 0x7a, 0xa7, 0x18, 0xcd, 0x1c, 0xfa, 0xd0, 0x32,
            0x33, 0xbc, 0xd6, 0x4a, 0x77, 0x89, 0xfd, 0x94, 0x22, 0xd3, 0xb1, 0x7a, 0xf6, 0x82,
            0x3a, 0x7e, 0x6a, 0xc6,
        ],
        vcrb: [
            0x8b, 0x6a, 0x0b, 0x38, 0xb9, 0xfa, 0xae, 0x3c, 0x3b, 0x80, 0x3b, 0x47, 0xb0, 0xf1,
            0x46, 0xad, 0x50, 0xab, 0x22, 0x1e, 0x6e, 0x2a, 0xfb, 0xe6, 0xdb, 0xde, 0x45, 0xcb,
            0xa9, 0xd3, 0x81, 0xed,
        ],
    };

    {
        let mut vec = Vec::new();
        let p = JUBJUB.generator(FixedGenerators::SpendingKeyGenerator);
        p.write(&mut vec).unwrap();
        assert_eq!(&vec, &sapling_generators.skb);
    }

    {
        let mut vec = Vec::new();
        let p = JUBJUB.generator(FixedGenerators::ProofGenerationKey);
        p.write(&mut vec).unwrap();
        assert_eq!(&vec, &sapling_generators.pkb);
    }

    {
        let mut vec = Vec::new();
        let p = JUBJUB.generator(FixedGenerators::NullifierPosition);
        p.write(&mut vec).unwrap();
        assert_eq!(&vec, &sapling_generators.npb);
    }

    {
        let mut vec = Vec::new();
        let p = JUBJUB.generator(FixedGenerators::NoteCommitmentRandomness);
        p.write(&mut vec).unwrap();
        assert_eq!(&vec, &sapling_generators.wprb);
    }

    {
        let mut vec = Vec::new();
        let p = JUBJUB.generator(FixedGenerators::ValueCommitmentValue);
        p.write(&mut vec).unwrap();
        assert_eq!(&vec, &sapling_generators.vcvb);
    }

    {
        let mut vec = Vec::new();
        let p = JUBJUB.generator(FixedGenerators::ValueCommitmentRandomness);
        p.write(&mut vec).unwrap();
        assert_eq!(&vec, &sapling_generators.vcrb);
    }
}
