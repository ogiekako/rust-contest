// Referred to http://hos.ac/slides/20140319_bit.pdf

pub struct BIT {
    bit: Vec<i32>,
}

impl BIT {
    pub fn new(n: usize) -> BIT {
        BIT { bit: vec![0;n + 1] }
    }
    // add w to the i-th element (0-origin)
    pub fn add(&mut self, i: usize, w: i32) {
        if i + 1 > self.bit.len() {
            panic!("Index out of bounds: {}", i);
        }
        let mut x = (i + 1) as i32;
        while x < self.bit.len() as i32 {
            self.bit[x as usize] += w;
            x += x & -x;
        }
    }

    // get the [0, i) sum.
    pub fn sum(&self, i: usize) -> i32 {
        let mut res = 0;
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
        let mut bit = BIT::new(4);
        bit.add(0, 10);
        bit.add(1, 20);
        bit.add(2, 30);
        bit.add(3, 40);

        assert_eq!(0, bit.sum(0));
        assert_eq!(10, bit.sum(1));
        assert_eq!(30, bit.sum(2));
        assert_eq!(60, bit.sum(3));
        assert_eq!(100, bit.sum(4));
    }
}
