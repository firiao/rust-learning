//! Структуры и методы.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Создать точку.
    pub fn new(x: f64, y: f64) -> Self {
        todo!()
    }

    /// Евклидово расстояние до другой точки.
    pub fn distance_to(&self, other: &Point) -> f64 {
        todo!()
    }

    /// Сдвинуть точку на месте.
    pub fn translate(&mut self, dx: f64, dy: f64) {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        todo!()
    }

    pub fn area(&self) -> f64 {
        todo!()
    }

    pub fn perimeter(&self) -> f64 {
        todo!()
    }

    pub fn is_square(&self) -> bool {
        todo!()
    }

    /// Увеличить обе стороны в `factor` раз. Требует `&mut self`.
    pub fn scale(&mut self, factor: f64) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_distance() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 4.0);
        assert_eq!(a.distance_to(&b), 5.0);
        assert_eq!(b.distance_to(&a), 5.0);
    }

    #[test]
    fn point_translate() {
        let mut p = Point::new(1.0, 2.0);
        p.translate(3.0, -1.0);
        assert_eq!(p, Point::new(4.0, 1.0));
    }

    #[test]
    fn rectangle_measurements() {
        let r = Rectangle::new(3.0, 4.0);
        assert_eq!(r.area(), 12.0);
        assert_eq!(r.perimeter(), 14.0);
        assert!(!r.is_square());
        assert!(Rectangle::new(5.0, 5.0).is_square());
    }

    #[test]
    fn rectangle_scale() {
        let mut r = Rectangle::new(2.0, 3.0);
        r.scale(2.0);
        assert_eq!(r, Rectangle::new(4.0, 6.0));
        assert_eq!(r.area(), 24.0);
    }

    #[test]
    fn derived_traits_work() {
        let r = Rectangle::new(1.0, 1.0);
        let copy = r; // Copy: r остаётся живым
        assert_eq!(r, copy);
        assert_eq!(format!("{r:?}"), "Rectangle { width: 1.0, height: 1.0 }");
    }
}
