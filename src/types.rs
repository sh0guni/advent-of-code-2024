use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vector {
    pub dx: isize,
    pub dy: isize,
}

impl Point {
    pub fn new((x, y): (usize, usize)) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl Add<Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl Sub<Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        let neg = -rhs;
        self + neg
    }
}

impl Mul<usize> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: usize) -> Self::Output {
        Vector {
            dx: self.dx * rhs as isize,
            dy: self.dy * rhs as isize,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector {
            dx: -self.dx,
            dy: -self.dy,
        }
    }
}

impl Mul<isize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: isize) -> Self::Output {
        Vector {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}
