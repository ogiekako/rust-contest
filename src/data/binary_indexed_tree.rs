// Binary indexed tree
// Referred to http://hos.ac/slides/20140319_bit.pdf

use std::ops::AddAssign;

pub struct BIT<T> {
    bit: Vec<T>,
}

impl<T> BIT<T>
    where T: AddAssign + Copy + Default
{
    pub fn new(n: usize) -> BIT<T> {
        BIT { bit: vec![T::default(); n + 1] }
    }

    // add w to the i-th element (0-origin)
    pub fn add(&mut self, i: usize, w: T) {
        if i + 1 >= self.bit.len() {
            panic!("Index out of bounds: {}", i);
        }
        let mut x = (i + 1) as i32;
        while x < self.bit.len() as i32 {
            self.bit[x as usize] += w;
            x += x & -x;
        }
    }

    // get the [0, i) sum.
    pub fn sum(&self, i: usize) -> T {
        if i >= self.bit.len() {
            panic!("Index out of bounds: {}", i);
        }
        let mut res = T::default();
        let mut x = i as i32;
        while x > 0 {
            res += self.bit[x as usize];
            x -= x & -x;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::BIT;
    #[test]
    fn test() {
        let mut bit = BIT::new(5);
        bit.add(0, 10);
        bit.add(1, -20);
        bit.add(3, 30);
        bit.add(4, 40);

        assert_eq!(0, bit.sum(0));
        assert_eq!(10, bit.sum(1));
        assert_eq!(-10, bit.sum(2));
        assert_eq!(-10, bit.sum(3));
        assert_eq!(20, bit.sum(4));
        assert_eq!(60, bit.sum(5));

        let mut bit = BIT::new(1);
        bit.add(0, 0.5);
        assert_eq!(0.5, bit.sum(1));
    }
}
