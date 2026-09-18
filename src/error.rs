use crate::schedule::FieldKind;

pub enum ParseError {
	// A string alias was invalid
	InvalidFieldName(String),

	// Too many/few fields were provided
	InvalidFieldCount(u8),

	OutOfRange {
		offending_field: FieldKind,
		value: u8
	},
}