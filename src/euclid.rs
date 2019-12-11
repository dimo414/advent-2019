// Referenced https://docs.rs/rusttype/0.5.2/src/rusttype/geometry.rs.html
// Other resources:
//   https://crates.io/crates/euclid - https://doc.servo.org/src/euclid/point.rs.html
mod point {
    use super::*;
    use std::fmt;
    use std::ops::{Add,AddAssign,Sub};
    use regex::Regex;
    use std::str::FromStr;
    use crate::error::ParseError;

    #[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    #[inline]
    pub const fn point(x: i32, y: i32) -> Point {
        Point { x, y }
    }


    impl Point {
        pub const ORIGIN: Point = point(0, 0);

        pub fn bounding_box(points: impl IntoIterator<Item = Point>) -> Option<(Point, Point)> {
            points.into_iter().fold(None, |r , c|
                match r {
                    Some((min, max)) => {
                        Some((
                            point(std::cmp::min(min.x, c.x), std::cmp::min(min.y, c.y)),
                            point(std::cmp::max(max.x, c.x), std::cmp::max(max.y, c.y))
                        ))
                    },
                    None => Some((c, c)),
                }
            )
        }

        pub fn in_bounds(&self, min: Point, max: Point) -> bool {
            assert!(min.x <= max.x);
            assert!(min.y <= max.y);
            min.x <= self.x && min.y <= self.y && max.x >= self.x && max.y >= self.y
        }
    }

    impl Add<&Vector> for Point {
        type Output = Point;

        fn add(self, vec: &Vector) -> Point {
            point(self.x + vec.x, self.y + vec.y)
        }
    }

    impl Add<&Vector> for &Point {
        type Output = Point;

        fn add(self, vec: &Vector) -> Point {
            point(self.x + vec.x, self.y + vec.y)
        }
    }

    impl Add<Vector> for &Point {
        type Output = Point;

        fn add(self, vec: Vector) -> Point {
            point(self.x + vec.x, self.y + vec.y)
        }
    }

    impl Add<Vector> for Point {
        type Output = Point;

        fn add(self, vec: Vector) -> Point {
            point(self.x + vec.x, self.y + vec.y)
        }
    }

    impl AddAssign<Vector> for Point {
        fn add_assign(&mut self, vec: Vector) {
            *self = point(self.x + vec.x, self.y + vec.y);
        }
    }

    impl Sub for Point {
        type Output = Vector;

        fn sub(self, point: Point) -> Vector { vector(self.x - point.x, self.y - point.y) }
    }

    impl Sub for &Point {
        type Output = Vector;

        fn sub(self, point: &Point) -> Vector { vector(self.x - point.x, self.y - point.y) }
    }

    impl FromStr for Point {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, ParseError> {
            lazy_static! {
                // r"^([^,]+),([^,]+)$" would be more strict - worth it?
                static ref RE: Regex = Regex::new(r"^\(?([^(,]+),([^),]+)\)?$").unwrap();
            }

            let caps = regex_captures!(RE, s)?;
            let x: i32 = capture_group!(caps, 1).trim().parse()?;
            let y: i32 = capture_group!(caps, 2).trim().parse()?;
            return Ok(point(x, y));
        }
    }

    impl fmt::Debug for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse() {
            assert_eq!("3, 4".parse::<Point>(), Ok(point(3, 4)));
            assert_eq!("-3,-4".parse::<Point>(), Ok(point(-3, -4)));
            assert_eq!("(40,30)".parse::<Point>(), Ok(point(40, 30)));
            assert_eq!("(-3, -5)".parse::<Point>(), Ok(point(-3, -5)));

            assert!("abc".parse::<Point>().is_err());
        }

        #[test]
        fn bounding() {
            let points = vec!(point(1, 2), point(2, 3), point(0, 5));
            assert_eq!(Point::bounding_box(points), Some((point(0, 2), point(2, 5))));
        }

        #[test]
        fn in_bounds_() {
            let zero_zero = point(0, 0);
            let two_two = point(2, 2);
            let five_six = point(5, 6);
            assert!(two_two.in_bounds(zero_zero, two_two));
            assert!(!five_six.in_bounds(zero_zero, two_two));
        }

        #[test]
        fn add() {
            assert_eq!(point(1, 0) + super::super::vector(2, 3), point(3, 3));
        }
        #[test]
        fn sub() {
            assert_eq!(point(3, 3) - point(1, 0), super::super::vector(2, 3));
        }
    }
}
pub use self::point::{Point,point};

mod vector {
    use std::fmt;
    use std::str::FromStr;
    use std::ops::Mul;
    use crate::error::ParseError;

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Vector {
        pub x: i32,
        pub y: i32,
    }

    #[inline]
    pub const fn vector(x: i32, y: i32) -> Vector {
        Vector { x, y }
    }

    impl Vector {
        pub fn abs(&self) -> Vector {
            vector(self.x.abs(), self.y.abs())
        }

        pub fn len(&self) -> f64 {
            (self.x as f64).hypot(self.y as f64)
        }

        pub fn grid_len(&self) -> u32 {
            (self.x.abs() + self.y.abs()) as u32
        }
    }

    impl Mul<i32> for Vector {
        type Output = Vector;

        fn mul(self, m: i32) -> Vector {
            vector(self.x * m, self.y * m)
        }
    }

    impl FromStr for Vector {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, ParseError> {
            // Just reuse point's parser
            let p: super::Point = s.parse()?;
            Ok(vector(p.x, p.y))
        }
    }

    impl fmt::Debug for Vector {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl fmt::Display for Vector {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::point;
        use super::*;

        #[test]
        fn parse() {
            assert_eq!("3, 4".parse::<Vector>(), Ok(vector(3, 4)));
            assert_eq!("-3,-4".parse::<Vector>(), Ok(vector(-3, -4)));
        }

        #[test]
        fn len() {
            assert_eq!(vector(3, -4).len(), 5_f64);
        }

        parameterized_test!{ grid_lens, (p1, p2, d), {
            assert_eq!((p1 - p2).grid_len(), d);
            assert_eq!((p2 - p1).grid_len(), d);
        }}
        grid_lens! {
            a: (point(1,1), point(1,1), 0),
            b: (point(1,1), point(1,2), 1),
            c: (point(1,1), point(2,2), 2),
            d: (point(1,1), point(1,5), 4),
            e: (point(1,1), point(8,3), 9),
            f: (point(1,1), point(-1,-1), 4),
        }
    }
}
pub use self::vector::{Vector,vector};
