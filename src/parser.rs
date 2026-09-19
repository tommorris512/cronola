use crate::config::CRON_FIELD_COUNT;
use crate::error::ParseError;
use crate::names::{day_to_index, month_to_index, parse_day_name, parse_month_name};
use crate::schedule::{Field, FieldItem, FieldKind, FieldTerminal, Schedule};

fn bounds(field: FieldKind) -> (u8, u8) {
    match field {
        FieldKind::Minute => (0, 59),
        FieldKind::Hour => (0, 23),
        FieldKind::DayOfMonth => (1, 31),
        FieldKind::Month => (1, 12),
        FieldKind::DayOfWeek => (0, 7),
    }
}

fn resolve_value(s: &str, field: FieldKind) -> Result<u8, ParseError> {
    if let Ok(value) = s.parse::<u8>() {
        return Ok(value);
    }

    match field {
        FieldKind::Month => Ok(month_to_index(parse_month_name(s)?)),
        FieldKind::DayOfWeek => Ok(day_to_index(parse_day_name(s)?)),

        // All other fields have no valid textual alias
        _ => Err(ParseError::MalformedSyntax(s.to_string())),
    }
}

fn parse_value(s: &str, field: FieldKind) -> Result<u8, ParseError> {
    let value = resolve_value(s, field)?;
    let (min, max) = bounds(field);

    // Ensure the parsed value falls within the valid bounds
    if value < min || value > max {
        return Err(ParseError::OutOfRange {
            offending_field: field,
            value,
        });
    }

    Ok(value)
}

fn parse_field_terminal(s: &str, field: FieldKind) -> Result<FieldTerminal, ParseError> {
    // Trivial case of a wildcard
    if s == "*" {
        return Ok(FieldTerminal::Star);
    }

    // Attempt to split a range into a start and end
    if let Some((start, end)) = s.split_once("-") {
        let start = parse_value(start, field)?;
        let end = parse_value(end, field)?;

        if start > end {
            return Err(ParseError::InvalidRange { start, end });
        }

        return Ok(FieldTerminal::Range(start, end));
    }

    // By exhaustion, this ought to be a singular numerical value
    Ok(FieldTerminal::Single(parse_value(s, field)?))
}

fn parse_field_item(s: &str, field: FieldKind) -> Result<FieldItem, ParseError> {
    // Attempt to split a step into a prefix and step count
    if let Some((prefix, step)) = s.split_once("/") {
        let field_terminal = parse_field_terminal(prefix, field)?;
        let step: u8 = step
            .parse()
            .map_err(|_| ParseError::MalformedSyntax(step.to_string()))?;

        return Ok(FieldItem::Step(field_terminal, step));
    }

    // By exhaustion, this ought to be a plain terminal
    Ok(FieldItem::Plain(parse_field_terminal(s, field)?))
}

fn parse_field(s: &str, field: FieldKind) -> Result<Field, ParseError> {
    s.split(",").map(|a| parse_field_item(a, field)).collect()
}

pub fn parse(s: &str) -> Result<Schedule, ParseError> {
    let fields: Vec<&str> = s.split(" ").collect();

    // TODO: change to allow the script(s) at the end too
    if fields.len() != CRON_FIELD_COUNT {
        return Err(ParseError::InvalidFieldCount(fields.len()));
    }

    let minute = parse_field(fields[0], FieldKind::Minute)?;
    let hour = parse_field(fields[1], FieldKind::Hour)?;
    let day_of_month = parse_field(fields[2], FieldKind::DayOfMonth)?;
    let month = parse_field(fields[3], FieldKind::Month)?;
    let day_of_week = parse_field(fields[4], FieldKind::DayOfWeek)?;

    Ok(Schedule::new(
        minute,
        hour,
        day_of_month,
        month,
        day_of_week,
    ))
}
