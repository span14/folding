use crate::vector::Vector;
use halo2_curves::bn256::Fr;
use halo2_curves::ff::Field;
use rand_core::OsRng;


pub fn get_random() -> Fr {
    Fr::random(OsRng)
}

pub fn convert(a: i32) -> Fr {
    if a < 0 {
        -Fr::from_raw([(-a) as u64, 0, 0, 0])
    } else {
        Fr::from_raw([a as u64, 0, 0, 0])
    }
}

pub fn circuit_verification(
    ql: &Vector, 
    qr: &Vector,
    qm: &Vector,
    qc: &Vector,
    q3: &Vector,
    qo: &Vector,
    a: &Vector,
    b: &Vector,
    o: &Vector,
) -> Vector {
    ql.clone() * a.clone() + 
    qr.clone() * b.clone() + qm * a.clone() * b.clone() + qc.clone() + q3.clone() * a.exp(3) + qo.clone() * o.clone()
}

pub fn folding_circuit_verification(
    ql: &Vector, 
    qr: &Vector,
    qm: &Vector,
    qc: &Vector,
    q3: &Vector,
    qo: &Vector,
    a: &Vector,
    b: &Vector,
    o: &Vector,
    e: &Vector,
    u: &Fr
) -> Vector {
    (ql.clone() * a.clone() + qr.clone() * b.clone() + qo.clone() * o.clone()) * u.square() + 
    qm * a.clone() * b.clone() * u + 
    qc.clone() * exp(u, 3) + 
    q3.clone() * a.exp(3) +
    e.clone()
}

pub fn exp(base: &Fr, num: i32) -> Fr{
    let mut temp = num;
    let mut multiplier = base.clone();
    let mut result = Fr::one();
    while temp > 0 {
        if temp & 1 == 1{
            result *= multiplier;
        }
        multiplier = multiplier.square();
        temp >>= 1;
    }
    result
}