/// Scanner
///
/// # Example
/// ```
/// use contest::scanner;
/// let mut sc = scanner::new("1 2 \n\n \r\t \n 3.5".as_bytes());
/// assert_eq!("1".to_string(), sc.next::<String>());
/// assert_eq!(2, sc.next());
/// assert_eq!(3.5, sc.next());
///
/// // To create a scanner from stdin:
/// scanner::new(std::io::stdin());
/// ```

use std;
use std::io;
use std::io::BufRead;

pub struct Scanner<R: io::Read> {
    br: io::BufReader<R>,
    // Read tokens are stored in reversed order per line.
    buf: Vec<String>,
}

pub fn new<R: io::Read>(r: R) -> Scanner<R> {
    Scanner::new(r)
}

impl<R: io::Read> Scanner<R> {
    #[inline]
    fn new(r: R) -> Scanner<R> {
        Scanner {
            br: io::BufReader::new(r),
            buf: vec![],
        }
    }
    #[inline]
    pub fn next<T>(&mut self) -> T
        where T: std::str::FromStr,
              T::Err: std::fmt::Debug
    {
        self.next_string().map(|s| s.parse::<T>().expect("Parse failed: ")).expect("Unexpected EOF")
    }
    fn next_string(&mut self) -> Option<String> {
        self.buf.pop().or_else(|| match self.update() {
            true => self.next_string(),
            false => None,
        })
    }
    #[inline]
    fn update(&mut self) -> bool {
        let mut s = String::new();
        let res = self.br.read_line(&mut s);
        match res.expect("I/O error.") {
            0 => false,
            _ => {
                self.buf = s.split_whitespace().map(|x| x.to_string()).rev().collect();
                true
            }
        }
    }
}

#[test]
fn next() {
    let mut sc = new("hoge".as_bytes());
    let s: String = sc.next();
    assert_eq!("hoge", s);
}
