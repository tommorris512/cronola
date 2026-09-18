use crate::error::ParseError;

pub enum Day {
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
	Sunday,
}

pub fn parse_day_name(s: &str) -> Result<Day, ParseError> {
	match s {
		"MON" => Ok(Day::Monday),
		"TUE" => Ok(Day::Tuesday),
		"WED" => Ok(Day::Wednesday),
		"THU" => Ok(Day::Thursday),
		"FRI" => Ok(Day::Friday),
		"SAT" => Ok(Day::Saturday),
		"SUN" => Ok(Day::Sunday),

		_ => Err(ParseError::InvalidFieldName(s.to_string())),
	}
}

pub fn day_to_index(day: Day) -> u8 {
	match day {
		Day::Sunday => 0, //or 7, handle this
		Day::Monday => 1,
		Day::Tuesday => 2,
		Day::Wednesday => 3,
		Day::Thursday => 4,
		Day::Friday => 5,
		Day::Saturday => 6,
	}
}

pub enum Month {
	January,
	February,
	March,
	April,
	May,
	June,
	July,
	August,
	September,
	October,
	November,
	December,
}

pub fn parse_month_name(s: &str) -> Result<Month, ParseError> {
	match s {
		"JAN" => Ok(Month::January),
		"FEB" => Ok(Month::February),
		"MAR" => Ok(Month::March),
		"APR" => Ok(Month::April),
		"MAY" => Ok(Month::May),
		"JUN" => Ok(Month::June),
		"JUL" => Ok(Month::July),
		"AUG" => Ok(Month::August),
		"SEP" => Ok(Month::September),
		"OCT" => Ok(Month::October),
		"NOV" => Ok(Month::November),
		"DEC" => Ok(Month::December),

		_ => Err(ParseError::InvalidFieldName(s.to_string())),
	}
}

pub fn month_to_index(month: Month) -> u8 {
	match month {
		Month::January => 1,
		Month::February => 2,
		Month::March => 3,
		Month::April => 4,
		Month::May => 5,
		Month::June => 6,
		Month::July => 7,
		Month::August => 8,
		Month::September => 9,
		Month::October => 10,
		Month::November => 11,
		Month::December => 12,
	}
}