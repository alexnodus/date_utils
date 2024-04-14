# Date Utils

## description ## Описание
`date_utils` is a Rust library that provides functions for working with dates. It allows parsing dates from strings and calculating the difference between two dates. 
— это Rust библиотека, предназначенная для работы с датами. Она позволяет парсить даты из строк и вычислять разницу между двумя датами.

## functionality ## Функциональность
library functions: Библиотека предоставляет следующие функции:

- `parse_date_from_datetime(datetime: &str) -> (i32, u32, u32)`: is a function that takes a string with a date and time and returns a tuple with the year, month, and day. : принимает строку с датой и временем и возвращает кортеж с годом, месяцем и днем. 
- `calculate_date_difference(start_year: i32, start_month: u32, start_day: u32, end_year: i32, end_month: u32, end_day: u32) -> (i32, i32, i32)`: is a function that calculates the difference between two dates and returns the difference in years, months, and days. : вычисляет разницу между двумя датами и возвращает разницу в годах, месяцах и днях.

## example usage ## Пример использования
```rust
use date_utils::date_utils::{parse_date_from_datetime, calculate_date_difference};

fn main() {
    let date_time_start = "2020-01-15T15:30:00";
    let date_time_end = "2023-04-14T15:30:00";

    let (start_year, start_month, start_day) = parse_date_from_datetime(date_time_start);
    let (end_year, end_month, end_day) = parse_date_from_datetime(date_time_end);
    
    let (years, months, days) = calculate_date_difference(start_year, start_month, start_day, end_year, end_month, end_day);
    println!("Difference is {} years, {} months, and {} days", years, months, days);
}
