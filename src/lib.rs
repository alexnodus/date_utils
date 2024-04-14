pub mod date_utils {
    pub fn parse_date_from_datetime(datetime: &str) -> (i32, u32, u32) {
        let date_part = datetime.split('T').next().unwrap();
        let parts: Vec<&str> = date_part.split('-').collect();
        let year = parts[0].parse::<i32>().unwrap();
        let month = parts[1].parse::<u32>().unwrap();
        let day = parts[2].parse::<u32>().unwrap();

        (year, month, day)
    }

    pub fn calculate_date_difference(start_year: i32, start_month: u32, start_day: u32,
                                     end_year: i32, end_month: u32, end_day: u32) -> (i32, i32, i32) {
        let mut years = end_year - start_year;
        let mut months = end_month as i32 - start_month as i32;
        let mut days = end_day as i32 - start_day as i32;

        if days < 0 {
            months -= 1;
            let prev_month = if end_month == 1 { 12 } else { end_month - 1 };
            let prev_year = if prev_month == 12 { end_year - 1 } else { end_year };
            let days_in_prev_month = match prev_month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => if (prev_year % 4 == 0 && prev_year % 100 != 0) || (prev_year % 400 == 0) { 29 } else { 28 },
                _ => unreachable!("Invalid month"),
            };
            days += days_in_prev_month;
        }
        if months < 0 {
            years -= 1;
            months += 12;
        }

        (years, months, days)
    }
}

#[cfg(test)]
mod tests {
    use super::date_utils::*;

    #[test]
    fn test_date_parsing() {
        assert_eq!(parse_date_from_datetime("2023-04-14T15:30:00"), (2023, 4, 14));
    }

    #[test]
    fn test_date_difference() {
        assert_eq!(calculate_date_difference(2024, 04, 01, 2024, 4, 01), (0, 0, 0));
        assert_eq!(calculate_date_difference(2024, 04, 01, 2024, 4, 02), (0, 0, 1));

        assert_eq!(calculate_date_difference(2024, 01, 29, 2024, 3, 28), (0, 1, 28));
        assert_eq!(calculate_date_difference(2024, 02, 20, 2024, 3, 28), (0, 1, 8));

        assert_eq!(calculate_date_difference(2023, 01, 29, 2023, 3, 28), (0, 1, 27));
        assert_eq!(calculate_date_difference(2023, 02, 20, 2023, 3, 28), (0, 1, 8));

        assert_eq!(calculate_date_difference(2020, 1, 15, 2023, 4, 14), (3, 2, 30));
    }
}
