use algebra::fields::bn_382::{Fr, Fq12Parameters, Fq2Parameters, Fq6Parameters};

use crate::fields::{fp::FpGadget, fp12::Fp12Gadget, fp2::Fp2Gadget, fp6_3over2::Fp6Gadget};

pub type FqGadget = FpGadget<Fr>;
pub type Fq2Gadget = Fp2Gadget<Fq2Parameters, Fr>;
pub type Fq6Gadget = Fp6Gadget<Fq6Parameters, Fr>;
pub type Fq12Gadget = Fp12Gadget<Fq12Parameters, Fr>;

#[test]
fn bn382_g_field_gadgets_test() {
    use super::*;
    use crate::fields::tests::*;
    use algebra::fields::bn_382::{Fq, Fq12, Fq2, Fq6};

    field_test::<_, Fq, FqGadget>();
    frobenius_tests::<Fq, Fq, FqGadget>(13);
    equ_verdict_fp_gadget_test::<Fq>();
    from_bits_fp_gadget_test::<Fq>();
    bit_fp_gadgets_test::<Fq>();

    field_test::<_, Fq, Fq2Gadget>();
    frobenius_tests::<Fq2, Fq, Fq2Gadget>(13);

    field_test::<_, Fq, Fq6Gadget>();
    frobenius_tests::<Fq6, Fq, Fq6Gadget>(13);

    field_test::<_, Fq, Fq12Gadget>();
    frobenius_tests::<Fq12, Fq, Fq12Gadget>(13);
}