use std::vec;

use halo2_curves::bn256::Fr;

mod commitment;
mod utils;
mod vector;


use commitment::Commitment;
use utils::{convert, circuit_verification, exp, folding_circuit_verification, get_random};
use vector::Vector;


// C1
// x1 + x2 - x3 = 0
// x3 * x4 - x5 = 0
// x5^3 - x6 = 0

// ql * a + qr * b + qm * a * b + qc + q3 * a^3 + qo * o = 0


fn main() {
    // query values in C1
    let ql = Vector::from_i(vec![1, 0, 0]);
    let qr = Vector::from_i(vec![1, 0, 0]);
    let qo = Vector::from_i(vec![-1, -1, -1]);
    let qm = Vector::from_i(vec![0, 1, 0]);
    let qc = Vector::from_i(vec![0, 0, 0]);
    let q3 = Vector::from_i(vec![0, 0, 1]);

    // assigned values in C1
    let a1 = Vector::from_i(vec![1, 3, 12]);
    let b1 = Vector::from_i(vec![2, 4, 0]);
    let o1 = Vector::from_i(vec![3, 12, 1728]);
    // randomly assigned
    let e1 = Vector::from_i(vec![5065, 293489, 9814]);

    // copy constraint in C1 ???
    

    // lookup values in C1 ???


    assert_eq!(
        circuit_verification(&ql, &qr, &qm, &qc, &q3, &qo, &a1, &b1, &o1), 
        Vector::from_i(vec![0, 0, 0])
    );

    
    // query values in C2
    // let ql2 = Vector::from_i(vec![1, 0, 0]);
    // let qr2 = Vector::from_i(vec![1, 0, 0]);
    // let qo2 = Vector::from_i(vec![-1, -1, -1]);
    // let qm2 = Vector::from_i(vec![0, 1, 0]);
    // let qc2 = Vector::from_i(vec![0, 0, 0]);
    // let q32 = Vector::from_i(vec![0, 0, 1]);

    // assigned values in C2
    let a2 = Vector::from_i(vec![3, 12, 24]);
    let b2 = Vector::from_i(vec![9, 2, 0]);
    let o2 =Vector::from_i(vec![12, 24, 13824]);
    let e2 = Vector::from_i(vec![93450, 92845, 9329480]);

    // copy constraint in C2 ???
    

    // lookup values in C2 ???

    assert_eq!(
        circuit_verification(&ql, &qr, &qm, &qc, &q3, &qo, &a2, &b2, &o2), 
        Vector::from_i(vec![0, 0, 0])
    );

    // Fold
    let commit = Commitment::<3>::init();
    let u1 = get_random();
    let u2 = get_random();
    let r = get_random();
    

    let u = u1 + r * u2;
    let a = a1.clone() + a2.clone() * r;
    let b = b1.clone() + b2.clone() * r;
    let o = o1.clone() + o2.clone() * r;

    let wa1 = commit.commit(&a1);
    let wa2 = commit.commit(&a2);
    let wa = wa1 + wa2 * r;

    let wb1 = commit.commit(&b1);
    let wb2 = commit.commit(&b2);
    let wb = wb1 + wb2 * r;

    let wo1 = commit.commit(&o1);
    let wo2 = commit.commit(&o2);
    let wo = wo1 + wo2 * r;


    let lin1 = ql.clone() * a1.clone() + qr.clone() * b1.clone() + qo.clone() * o1.clone();
    let lin2 = ql.clone() * a2.clone() + qr.clone() * b2.clone() + qo.clone() * o2.clone();
    let t1 = 
        lin1.clone() * convert(2) * u1 * u2 +
        lin2.clone() * u1.square() +
        (
            a1.clone() * b2.clone() * u1 +
            a2.clone() * b1.clone() * u1 +
            a1.clone() * b1.clone() * u2
        ) * qm.clone() +
        a1.exp(2) * a2.clone() * convert(3) * q3.clone() +
        qc.clone() * u1.square() * u2 * convert(3);
    
    let t2 = 
        lin1.clone() * u2.square() +
        lin2.clone() * u1 * u2 * convert(2) +
        (
            a2.clone() * b2.clone() * u1 +
            a1.clone() * b2.clone() * u2 +
            a2.clone() * b1.clone() * u1
        ) * qm.clone() +
        a1.clone() * a2.exp(2) * convert(3) * q3.clone() +
        qc.clone() * u1 * u2.square() * convert(3);

    
    
    // Sangaria
    // let we1 = commit.commit(&e1);
    // let we2 = commit.commit(&e2);
    // let wt1 = commit.commit(&t1);
    // let wt2 = commit.commit(&t2);
    // let we = we1 + wt1 * (-r) + wt2 * (-r.square()) + we2 * exp(&r, 3);
    
    // Protostar
    let e = e1.clone() - t1.clone() * r - t2.clone() * r.square() + e2.clone() * exp(&r, 3);
    // Commit e at the very last round. So far this example only fold two circuits
    let we = commit.commit(&e);
    
    // Internal Testing
    assert_eq!(
        folding_circuit_verification(
            &ql, &qr, &qm, &qc, &q3, &qo, &a, &b, &o, &e, &u
        ),
        folding_circuit_verification(
            &ql, &qr, &qm, &qc, &q3, &qo, &a1, &b1, &o1, &e1, &u1
        ) + 
        folding_circuit_verification(
            &ql, &qr, &qm, &qc, &q3, &qo, &a2, &b2, &o2, &e2, &u2
        ) * exp(&r, 3)
    );


}
