//! Трейты, `dyn Trait`.

pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str;

    /// Метод по умолчанию: переопределять не обязательно.
    fn describe(&self) -> String {
        format!("{} площадью {:.2}", self.name(), self.area())
    }
}

pub struct Circle {
    pub radius: f64,
}

pub struct Square {
    pub side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!()
    }

    fn name(&self) -> &'static str {
        "круг"
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        todo!()
    }

    fn name(&self) -> &'static str {
        "квадрат"
    }
}

/// Суммарная площадь разных фигур в одном списке — только через `dyn`.
pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    todo!()
}

/// Описания всех фигур.
pub fn describe_all(shapes: &[&dyn Shape]) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> (Circle, Square) {
        (Circle { radius: 1.0 }, Square { side: 2.0 })
    }

    #[test]
    fn areas_are_correct() {
        let (c, s) = sample();
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-9);
        assert_eq!(s.area(), 4.0);
    }

    #[test]
    fn default_method_works() {
        let (c, _) = sample();
        assert_eq!(c.describe(), "круг площадью 3.14");
    }

    #[test]
    fn sums_heterogeneous_list() {
        let (c, s) = sample();
        let shapes: Vec<&dyn Shape> = vec![&c, &s];
        let total = total_area(&shapes);
        assert!((total - (std::f64::consts::PI + 4.0)).abs() < 1e-9);
        assert_eq!(total_area(&[]), 0.0);
    }

    #[test]
    fn describes_heterogeneous_list() {
        let (c, s) = sample();
        let shapes: Vec<&dyn Shape> = vec![&c, &s];
        let descriptions = describe_all(&shapes);
        assert_eq!(descriptions.len(), 2);
        assert!(descriptions[0].starts_with("круг"));
        assert!(descriptions[1].starts_with("квадрат"));
    }
}
