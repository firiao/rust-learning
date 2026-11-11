//! Лаба 7. структуры и методы.
//!
//! Реализация — твоя задача. Спека лежит в `tests/spec.rs`.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Money {
    kopecks: u64,          
}

impl Money {
    pub fn new(rubles: u64, kopecks: u64) -> Self {
        Self { kopecks: rubles * 100 + kopecks }
    }
    pub fn rubles(&self) -> u64 { self.kopecks / 100 }
    pub fn kopecks(&self) -> u64 { self.kopecks % 100 }
    pub fn in_kopecks(&self) -> u64 { self.kopecks }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self{
        Self {x, y}
    }
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self{
        Self {width, height}
    }
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    pub fn is_square(&self) -> bool {
        if self.width == self.height {
            return true;
        }
        false
    }
    pub fn scale(&mut self, x: f64) {
        self.width *= x;
        self.height *= x;
    }
}
