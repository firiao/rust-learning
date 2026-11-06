// 05. Pattern matching.
//
// Запуск:
//   rustc --edition 2024 examples/05_match.rs -o /tmp/match
//   /tmp/match
//
// Темы: match, guards, диапазоны, деструктуризация, if let, while let, let-else.

#[derive(Debug)]
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn main() {
    // Match по числам с диапазоном и guard.
    let x = 7;
    match x {
        1 => println!("один"),
        2..=5 => println!("от 2 до 5"),
        n if n % 2 == 0 => println!("чётное {n}"),
        n => println!("что-то другое: {n}"),
    }

    // Деструктуризация enum с данными.
    let commands = vec![
        Command::Move { x: 1, y: 2 },
        Command::Write(String::from("hello")),
        Command::ChangeColor(255, 0, 0),
        Command::Quit,
    ];

    for command in commands {
        match command {
            Command::Quit => println!("выход"),
            Command::Move { x, y } => println!("движение в ({x}, {y})"),
            Command::Write(text) => println!("текст: {text}"),
            Command::ChangeColor(r, g, b) => println!("цвет: ({r}, {g}, {b})"),
        }
    }

    // if let — короткая форма для одного варианта.
    let some = Some(42);
    if let Some(value) = some {
        println!("if let: {value}");
    }

    // while let — пока pop возвращает Some.
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("stack pop: {top}");
    }

    // let-else: либо достали значение, либо выходим.
    let value: Option<i32> = Some(10);
    let Some(v) = value else {
        println!("значения нет");
        return;
    };
    println!("let-else: {v}");

    // Деструктуризация кортежа.
    let pair = (1, "два");
    let (a, b) = pair;
    println!("pair = ({a}, {b})");

    // Срезы в match.
    let numbers: &[i32] = &[1, 2, 3, 4, 5];
    match numbers {
        [first, .., last] => println!("first = {first}, last = {last}"),
        [only] => println!("одно число: {only}"),
        [] => println!("пусто"),
    }

    // Упражнение: добавь в Command вариант Ping и обработай его.
    // Компилятор проверит, что все варианты покрыты.
}
