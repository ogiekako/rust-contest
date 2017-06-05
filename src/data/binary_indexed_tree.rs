// Binary indexed tree
// Referred to http://hos.ac/slides/20140319_bit.pdf

use std::ops::Add;

pub struct BIT<T> {
    bit: Vec<Option<T>>,
}

impl<'a, T: 'a> BIT<T>
    where T: Add<Output = T> + Copy
{
    pub fn new(n: usize) -> BIT<T> {
        let mut bit = BIT { bit: vec![] };
        for i in 0..n + 1 {
            bit.bit.push(None);
        }
        bit
    }

    // add w to the i-th element (0-origin)
    pub fn add(&'a mut self, i: usize, w: T) {
        if i + 1 >= self.bit.len() {
            panic!("Index out of bounds: {}", i);
        }
        let mut x = (i + 1) as i32;
        while x < self.bit.len() as i32 {
            self.bit[x as usize] = Some(self.bit[x as usize].map_or(w, |v| v + w));
            x += x & -x;
        }
    }

    // get the [0, i) sum.
    pub fn sum(&self, i: usize) -> Option<T> {
        let mut res = None;
        let mut x = i as i32;
        while x > 0 {
            res = match (self.bit[x as usize], res) {
                (Some(u), Some(v)) => Some(u + v),
                (v, None) => v,
                _ => res,
            };
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
        let mut bit = BIT::new(4);
        bit.add(0, 10);
        bit.add(1, 20);
        bit.add(2, 30);
        bit.add(3, 40);

        assert_eq!(None, bit.sum(0));
        assert_eq!(Some(10), bit.sum(1));
        assert_eq!(Some(30), bit.sum(2));
        assert_eq!(Some(60), bit.sum(3));
        assert_eq!(Some(100), bit.sum(4));
    }
}
