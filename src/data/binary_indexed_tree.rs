// Binary indexed tree
// Referred to http://hos.ac/slides/20140319_bit.pdf

pub struct BIT<T, F> {
    bit: Vec<T>,
    id: T,
    op: F,
}

impl<T, F> BIT<T, F>
    where T: Clone,
          F: Fn(&T, &T) -> T
{
    pub fn new(n: usize, id: T, op: F) -> BIT<T, F> {
        BIT {
            bit: vec![id.clone(); n + 1],
            id: id,
            op: op,
        }
    }

    // add w to the i-th element (0-origin)
    pub fn add(&mut self, mut i: usize, w: T) {
        i += 1;
        if i >= self.bit.len() {
            panic!("Index out of bounds: {}", i);
        }
        while i < self.bit.len() {
            self.bit[i] = (self.op)(&self.bit[i], &w);
            i += i & i.wrapping_neg();
        }
    }

    // get the [0, i) sum.
    pub fn sum(&self, mut i: usize) -> T {
        if i >= self.bit.len() {
            panic!("Index out of bounds: {}", i);
        }
        let mut res = self.id.clone();
        while i > 0 {
            res = (self.op)(&res, &self.bit[i]);
            i -= i & i.wrapping_neg();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::BIT;
    #[test]
    fn test() {
        let mut bit = BIT::new(5, 0, |&x, &y| x + y);
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

        let mut bit = BIT::new(1, 0.0, |&x, &y| x + y);
        bit.add(0, 0.5);
        assert_eq!(0.5, bit.sum(1));
    }
}
