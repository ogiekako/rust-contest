// Segment tree
// Referred implementation: http://arc008.contest.atcoder.jp/submissions/1231257
// Algorithm explained in http://codeforces.com/blog/entry/18051
// Segment tree problems: http://hamayanhamayan.hatenablog.jp/entry/2017/04/06/105732

/// Point update + range query
pub struct SegTree<T, F> {
    n: usize,
    // root: 1
    // left: i * 2,  right: i * 2 + 1
    // parent: i / 2
    nodes: Vec<T>,
    id: T,
    op: F,
}

impl<T: Clone, F: Fn(&T, &T) -> T> SegTree<T, F> {
    /// n doesn't have to be a power of 2.
    pub fn new(n: usize, id: T, op: F) -> SegTree<T, F> {
        SegTree {
            n: n,
            nodes: vec![id.clone(); n * 2],
            id: id,
            op: op,
        }
    }

    pub fn set(&mut self, mut i: usize, x: T) {
        if i >= self.n {
            panic!("Index out of bounds: {}", i);
        }
        i += self.n;
        self.nodes[i] = x;
        while {
            i >>= 1;
            i > 0
        } {
            self.nodes[i] = (self.op)(&self.nodes[i * 2], &self.nodes[i * 2 + 1]);
        }
    }

    pub fn query(&self, mut l: usize, mut r: usize) -> T {
        if r > self.n {
            panic!("Index out of bounds: {}", r);
        }
        let (mut al, mut ar) = (self.id.clone(), self.id.clone());
        l += self.n;
        r += self.n;
        while l < r {
            if l & 1 == 1 {
                al = (self.op)(&al, &self.nodes[l]);
                l += 1;
            }
            if r & 1 == 1 {
                ar = (self.op)(&self.nodes[r - 1], &ar);
            }
            r >>= 1;
            l >>= 1;
        }
        return (self.op)(&al, &ar);
    }
}

#[cfg(test)]
mod test {
    use super::SegTree;

    #[test]
    fn test() {
        let mut seg = SegTree::new(5, 1, |&x, &y| x * y);

        seg.set(0, 2);
        seg.set(4, 3);
        assert_eq!(6, seg.query(0, 5));
        assert_eq!(2, seg.query(0, 4));
        assert_eq!(3, seg.query(1, 5));
        assert_eq!(1, seg.query(2, 4));
        seg.set(2, 0);
        assert_eq!(0, seg.query(0, 5));
        assert_eq!(1, seg.query(1, 0));
        assert_eq!(1, seg.query(0, 0));
    }

    #[test]
    fn rmq() {
        use std;
        let max = std::i32::MAX;
        let mut seg = SegTree::new(3, max, |&x, &y| std::cmp::min(x, y));

        seg.set(0, 2);
        seg.set(2, 3);
        assert_eq!(2, seg.query(0, 3));
        assert_eq!(2, seg.query(0, 2));
        assert_eq!(3, seg.query(1, 3));
        assert_eq!(max, seg.query(1, 2));
        seg.set(1, 0);
        assert_eq!(0, seg.query(0, 3));
        assert_eq!(max, seg.query(1, 0));
        assert_eq!(max, seg.query(0, 0));
    }
}
