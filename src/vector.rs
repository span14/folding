use crate::utils::convert;
use std::ops::{Add, Mul, Neg, Sub};
use std::cmp::PartialEq;
use halo2_curves::bn256::Fr;


#[derive(Debug, Clone)]
pub struct Vector {
    pub elems: Vec<Fr>
}

impl Vector {
    pub fn from_f(elems: Vec<Fr>) -> Vector {
        Vector {
            elems
        }
    }

    pub fn from_i(elems: Vec<i32>) -> Vector {
        let nelems = elems.iter().map(|x| convert(*x)).collect::<Vec<Fr>>();
        Vector { elems: nelems }
    }

    pub fn exp(&self, num: i32) -> Vector {
        let elems = self.elems.iter().map(|x| {
            let mut temp = num;
            let mut multiplier = x.clone();
            let mut result = Fr::one();
            while temp > 0 {
                if temp & 1 == 1{
                    result *= multiplier;
                }
                multiplier = multiplier.square();
                temp >>= 1;
            }
            result
        }).collect::<Vec<Fr>>();

        Vector { elems }
    }

}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        assert!(self.elems.len() == rhs.elems.len());
        let elems = self.elems.iter().zip(&rhs.elems).map(|(x, y)| x + y).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Add<Vector> for &Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        assert!(self.elems.len() == rhs.elems.len());
        let elems = self.elems.iter().zip(&rhs.elems).map(|(x, y)| x + y).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Add<&Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: &Vector) -> Self::Output {
        assert!(self.elems.len() == rhs.elems.len());
        let elems = self.elems.iter().zip(&rhs.elems).map(|(x, y)| x + y).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;
    fn add(self, rhs: &Vector) -> Self::Output {
        assert!(self.elems.len() == rhs.elems.len());
        let elems = self.elems.iter().zip(&rhs.elems).map(|(x, y)| x + y).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        assert!(self.elems.len() == rhs.elems.len());
        let elems = self.elems.iter().zip(&rhs.elems).map(|(x, y)| x - y).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Sub<Vector> for &Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        assert!(self.elems.len() == rhs.elems.len());
        let elems = self.elems.iter().zip(&rhs.elems).map(|(x, y)| x - y).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Sub<&Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: &Vector) -> Self::Output {
        assert!(self.elems.len() == rhs.elems.len());
        let elems = self.elems.iter().zip(&rhs.elems).map(|(x, y)| x - y).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Sub<&Vector> for &Vector {
    type Output = Vector;
    fn sub(self, rhs: &Vector) -> Self::Output {
        assert!(self.elems.len() == rhs.elems.len());
        let elems = self.elems.iter().zip(&rhs.elems).map(|(x, y)| x - y).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}


impl Mul<Vector> for Vector {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        assert!(self.elems.len() == rhs.elems.len());
        let elems = self.elems.iter().zip(&rhs.elems).map(|(x, y)| x * y).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Mul<Vector> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        assert!(self.elems.len() == rhs.elems.len());
        let elems = self.elems.iter().zip(&rhs.elems).map(|(x, y)| x * y).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Mul<&Vector> for Vector {
    type Output = Vector;
    fn mul(self, rhs: &Vector) -> Self::Output {
        assert!(self.elems.len() == rhs.elems.len());
        let elems = self.elems.iter().zip(&rhs.elems).map(|(x, y)| x * y).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Mul<&Vector> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: &Vector) -> Self::Output {
        assert!(self.elems.len() == rhs.elems.len());
        let elems = self.elems.iter().zip(&rhs.elems).map(|(x, y)| x * y).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}


impl Mul<Fr> for Vector {
    type Output = Vector;
    fn mul(self, rhs: Fr) -> Self::Output {
        let elems = self.elems.iter().map(|x| x * rhs).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Mul<Fr> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: Fr) -> Self::Output {
        let elems = self.elems.iter().map(|x| x * rhs).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Mul<&Fr> for Vector {
    type Output = Vector;
    fn mul(self, rhs: &Fr) -> Self::Output {
        let elems = self.elems.iter().map(|x| x * rhs).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Mul<&Fr> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: &Fr) -> Self::Output {
        let elems = self.elems.iter().map(|x| x * rhs).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        let elems = self.elems.iter().map(|x| -x).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}

impl Neg for &Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        let elems = self.elems.iter().map(|x| -x).collect::<Vec<Fr>>();
        Vector {
            elems
        }
    }
}


impl PartialEq for Vector {

    fn eq(&self, other: &Self) -> bool {
        if self.elems.len() == other.elems.len() {
            self.elems.iter().zip(&other.elems).fold(true, |x, (y, z)| x & (y == z) )
        } else {
            false
        }
    }
}



#[cfg(test)]
mod tests {

    use super::Vector;

    #[test]
    fn test_vector_multiplication() {
        let a = vec![1, 2, 3, 4];
        let b = vec![5, 6, 7, 8];
        let c = vec![5, 12, 21, 32];

        let av = Vector::from_i(a);
        let bv = Vector::from_i(b);
        let cv = Vector::from_i(c);

        assert_eq!(av * bv, cv);

    }

    #[test]
    fn test_vector_addition() {
        let a = vec![1, 2, 3, 4];
        let b = vec![5, 6, 7, 8];
        let c = vec![6, 8, 10, 12];

        let av = Vector::from_i(a);
        let bv = Vector::from_i(b);
        let cv = Vector::from_i(c);

        assert_eq!(av + bv, cv);
    }

    #[test]
    fn test_vector_subtraction() {
        let a = vec![1, 2, 3, 4];
        let b = vec![5, 6, 7, 8];
        let c = vec![-4, -4, -4, -4];

        let av = Vector::from_i(a);
        let bv = Vector::from_i(b);
        let cv = Vector::from_i(c);

        assert_eq!(av - bv, cv);
    }

    #[test]
    fn test_vector_negation() {
        let a = vec![1, 2, 3, 4];
        let b = vec![-1, -2, -3, -4];

        let av = Vector::from_i(a);
        let bv = Vector::from_i(b);

        assert_eq!(-av, bv);
    }



    #[test]
    fn test_vector_exp() {
        let a = vec![1, 2, 3, 4];
        let b = vec![1, 8, 27, 64];

        let av = Vector::from_i(a);
        let bv = Vector::from_i(b);

        assert_eq!(av.exp(3), bv);
    }

    

}
