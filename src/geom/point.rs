use std::ops::Add;

#[derive(Debug)]
pub struct Point<T>(T, T);

impl<T> Add for Point<T>
    where T: Add<Output = T>
{
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl<'a, T> Add for &'a Point<T>
    where &'a T: Add<Output = T>
{
    type Output = Point<T>;

    fn add(self, other: &'a Point<T>) -> Point<T> {
        Point(&self.0 + &other.0, &self.1 + &other.1)
    }
}

impl<'a, T> Add<&'a Point<T>> for Point<T>
    where T: Add<&'a T, Output = T>
{
    type Output = Point<T>;

    fn add(self, other: &'a Point<T>) -> Point<T> {
        Point(self.0 + &other.0, self.1 + &other.1)
    }
}

impl<T> PartialEq for Point<T>
    where T: Eq
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    #[test]
    fn test() {
        assert_eq!(Point(1, 0) + Point(2, 3), Point(3, 3));

        let p = &Point(1, 2);
        assert_eq!(p + p + p, Point(3, 6));
    }
}
