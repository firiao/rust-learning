// 07. Трейты, generics, trait objects, операторные трейты.
//
// Запуск:
//   rustc --edition 2024 examples/07_traits.rs -o /tmp/traits
//   /tmp/traits
//
// Темы: trait, default methods, impl Trait for Type, generics, dyn Trait,
//        Add, Display.

use std::fmt;
use std::ops::Add;

trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;

    fn species(&self) -> &str {
        "животное"
    }

    // Default-метод использует другие методы трейта.
    fn speak(&self) -> String {
        format!("{} по имени {} говорит: {}", self.species(), self.name(), self.sound())
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn sound(&self) -> &str {
        "гав"
    }

    fn species(&self) -> &str {
        "собака"
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn sound(&self) -> &str {
        "мяу"
    }

    fn species(&self) -> &str {
        "кошка"
    }
}

// Статическая диспетчеризация: компилятор создаёт версию под каждый T.
fn introduce<T: Animal>(animal: &T) {
    println!("{}", animal.speak());
}

// Динамическая диспетчеризация: разные типы в одной коллекции.
fn zoo(animals: &[Box<dyn Animal>]) {
    for animal in animals {
        println!("{}", animal.speak());
    }
}

// impl Trait в возвращаемом типе: конкретный тип скрыт.
fn make_animal() -> impl Animal {
    Dog {
        name: String::from("Рекс"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Рекс"),
    };
    let cat = Cat {
        name: String::from("Мурка"),
    };

    // Generic-функция.
    introduce(&dog);
    introduce(&cat);

    // Trait objects.
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: String::from("Шарик"),
        }),
        Box::new(Cat {
            name: String::from("Барсик"),
        }),
    ];
    zoo(&animals);

    // impl Trait.
    println!("{}", make_animal().speak());

    // Операторные трейты.
    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = Vec2 { x: 3.0, y: 4.0 };
    let v3 = v1 + v2;
    println!("v1 + v2 = {v3}");
    println!("Debug: {v3:?}");

    // Упражнение: реализуй Sub для Vec2 и Mul<f64>.
    // Затем добавь трейт Shape с методом area() и две реализации.
}
