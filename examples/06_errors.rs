// 06. Ошибки: Result, ?, свои типы ошибок.
//
// Запуск:
//   rustc --edition 2024 examples/06_errors.rs -o /tmp/errors
//   /tmp/errors
//
// Темы: Result<T, E>, ?, From, Display, Error, panic vs Result.

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    Negative(i64),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(error) => write!(f, "ошибка разбора: {error}"),
            AppError::Negative(value) => write!(f, "отрицательное число: {value}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Parse(error) => Some(error),
            AppError::Negative(_) => None,
        }
    }
}

// Позволяет `?` автоматически преобразовать ParseIntError в AppError.
impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

fn parse_positive(text: &str) -> Result<u64, AppError> {
    let value: i64 = text.parse()?; // ? вернёт AppError::Parse
    if value < 0 {
        return Err(AppError::Negative(value));
    }
    Ok(value as u64)
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let value = parse_positive("10")?;
    println!("run: value = {value}");
    Ok(())
}

fn main() {
    for input in ["42", "-7", "не число"] {
        match parse_positive(input) {
            Ok(value) => println!("{input:?} -> Ok({value})"),
            Err(error) => println!("{input:?} -> Err({error})"),
        }
    }

    // unwrap/expect — для прототипов и тестов, не для пользовательского ввода.
    let number: i32 = "123".parse().expect("это должно быть числом");
    println!("number = {number}");

    // Комбинаторы Option.
    let length = Some("hello").map(|text| text.len()).unwrap_or(0);
    println!("length = {length}");

    let maybe: Option<i32> = None;
    let value = maybe.unwrap_or(42);
    println!("value = {value}");

    // main может обработать ошибку, вернувшуюся из run.
    if let Err(error) = run() {
        eprintln!("run завершился с ошибкой: {error}");
    }

    // Упражнение: добавь вариант AppError::TooLarge(u64) и проверку
    // value > 1_000_000. Не забудь обновить Display.
}
