//! Разбор аргументов командной строки через `clap`.

use clap::Parser;

/// Считает статистику по файлу.
#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Путь к файлу
    pub path: String,

    /// Сколько строк показать
    #[arg(short, long, default_value_t = 10)]
    pub top: usize,
}

/// Разбирает аргументы (без имени программы в начале списка).
pub fn parse_args(items: &[&str]) -> Result<Args, clap::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_path_with_default_top() {
        let args = parse_args(&["file.txt"]).unwrap();
        assert_eq!(args.path, "file.txt");
        assert_eq!(args.top, 10);
    }

    #[test]
    fn parses_long_flag() {
        let args = parse_args(&["file.txt", "--top", "5"]).unwrap();
        assert_eq!(args.top, 5);
    }

    #[test]
    fn parses_short_flag() {
        let args = parse_args(&["-t", "3", "file.txt"]).unwrap();
        assert_eq!(args.top, 3);
    }

    #[test]
    fn missing_path_is_error() {
        assert!(parse_args(&[]).is_err());
        assert!(parse_args(&["--top", "5"]).is_err());
    }

    #[test]
    fn bad_number_is_error() {
        assert!(parse_args(&["file.txt", "--top", "много"]).is_err());
    }
}
