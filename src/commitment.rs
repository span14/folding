use crate::utils::get_random;
use crate::vector::Vector;

use halo2_curves::bn256::Fr;
use halo2_curves::bn256::G1;
use halo2_curves::group::Group;



#[derive(Debug, Clone)]
pub struct Commitment<const NUM: usize> {
    elems: Vec<G1>,
}


impl<const NUM: usize> Commitment<NUM> {
    pub fn init() -> Commitment<NUM> {
        assert!(NUM > 0);
        let g = get_random();
        let mut gs = g;
        let mut elems = vec![];
        elems.push(G1::generator());
        for _ in 1..NUM {
            elems.push(G1::generator() * g);
            gs *= g;
        }

        Commitment::<NUM> {
            elems
        }
    }

    pub fn commit(&self, coeffs: &Vector) -> G1 {
        assert!(coeffs.elems.len() == NUM);
        coeffs.elems.iter().zip(&self.elems).fold(G1::identity(), |x, (y, z)| x + z * y )
    }
}