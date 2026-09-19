use crate::{config::CRON_FIELD_COUNT, schedule::FieldKind};

pub enum ParseError {
    // The syntax could not be parsed appropriately
    MalformedSyntax(String),

    // A string alias was invalid
    InvalidFieldName(String),

    // Too many/few fields were provided
    InvalidFieldCount(usize),

    // A range was malformed
    InvalidRange {
        start: u8,
        end: u8,
    },

    // A value falls outside the fields range
    OutOfRange {
        offending_field: FieldKind,
        value: u8,
    },
}

// Add display for all variants of ParseError
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::MalformedSyntax(s) => {
                write!(f, "MalformedSyntax: {} is not valid cron syntax", s,)
            }

            ParseError::InvalidFieldCount(field_count) => {
                write!(
                    f,
                    "InvalidFieldCount: expected {} received {}",
                    CRON_FIELD_COUNT, field_count,
                )
            }

            ParseError::InvalidFieldName(field_name) => {
                write!(
                    f,
                    "InvalidFieldName: field name {} is not valid",
                    field_name,
                )
            }

            ParseError::InvalidRange { start, end } => {
                write!(f, "InvalidRange: the range {}-{} is not valid", start, end,)
            }

            ParseError::OutOfRange {
                offending_field,
                value,
            } => {
                write!(
                    f,
                    "OutOfRange: the {} value '{}' is out of range",
                    offending_field, value,
                )
            }
        }
    }
}
