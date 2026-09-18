pub enum FieldTerminal {
	// * wildcard for any value
	Star,

	// A singular value
	Single(u8),

	// A range of values: (start, end)
	Range(u8, u8)
}

pub enum FieldItem {
	// A single terminal
	Plain(FieldTerminal),

	// A step of values (terminal, step_size)
	Step(FieldTerminal, u8),
}

// A singular value is treated as a list of length 1
pub struct Field(Vec<FieldItem>);

pub struct Schedule {
	minute: Field,
	hour: Field,
	dom: Field,
	month: Field,
	dow: Field,
}

pub enum FieldKind {
	Minute,
	Hour,
	DayOfMonth,
	Month,
	DayOfWeek,
}