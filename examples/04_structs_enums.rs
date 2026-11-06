// 04. Structs, enums, методы, Option.
//
// Запуск:
//   rustc --edition 2024 examples/04_structs_enums.rs -o /tmp/structs
//   /tmp/structs
//
// Темы: struct, impl, enum с данными, match, Option, derive.

use std::f64::consts::PI;

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u8,
    active: bool,
}

impl User {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
            active: true,
        }
    }

    fn birthday(&mut self) {
        self.age += 1;
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    fn into_name(self) -> String {
        self.name
    }
}

#[derive(Debug)]
struct Point(i32, i32);

#[derive(Debug)]
enum Shape {
    Circle {
        radius: f64,
    },
    Rectangle {
        width: f64,
        height: f64,
    },
    Triangle(f64, f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

fn main() {
    let mut user = User::new("Аня", 20);
    user.birthday();
    println!("user = {user:?}");
    println!("is_adult = {}", user.is_adult());
    println!("active = {}", user.active);

    // Struct update syntax: копируем остальные поля.
    let user2 = User {
        name: String::from("Борис"),
        ..user.clone()
    };
    println!("user2 = {user2:?}");

    let name = user2.into_name();
    println!("name = {name}");

    let point = Point(3, 4);
    println!("point = {point:?}, x = {}, y = {}", point.0, point.1);

    // Option<T> вместо null.
    let maybe: Option<&str> = Some("значение");
    match maybe {
        Some(value) => println!("Some({value})"),
        None => println!("None"),
    }

    let nothing: Option<&str> = None;
    if let Some(value) = nothing {
        println!("не должно выполниться: {value}");
    } else {
        println!("nothing = None");
    }

    let shapes = vec![
        Shape::Circle { radius: 1.0 },
        Shape::Rectangle {
            width: 3.0,
            height: 4.0,
        },
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for shape in &shapes {
        println!("{shape:?} -> area = {:.2}", shape.area());
    }

    // Упражнение: добавь вариант Shape::Square { side } и обработай его
    // в area(). Компилятор напомнит, если забыть вариант.
}
