// The fields a cron entry contains
#[derive(Clone, Copy, PartialEq)]
pub enum FieldKind {
    Minute,
    Hour,
    DayOfMonth,
    Month,
    DayOfWeek,
}

impl std::fmt::Display for FieldKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldKind::Minute => {
                write!(f, "minute")
            }

            FieldKind::Hour => {
                write!(f, "hour")
            }

            FieldKind::DayOfMonth => {
                write!(f, "day of month")
            }

            FieldKind::Month => {
                write!(f, "month")
            }

            FieldKind::DayOfWeek => {
                write!(f, "day of week")
            }
        }
    }
}

// The fundamental terminals of cron fields
#[derive(PartialEq)]
pub enum FieldTerminal {
    // * wildcard for any value
    Star,

    // A singular value
    Single(u8),

    // A range of values: (start, end)
    Range(u8, u8),
}

impl std::fmt::Display for FieldTerminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldTerminal::Star => write!(f, "*"),
            FieldTerminal::Single(value) => write!(f, "{}", value),
            FieldTerminal::Range(start, end) => write!(f, "{}-{}", start, end),
        }
    }
}

//
#[derive(PartialEq)]
pub enum FieldItem {
    // A single terminal
    Plain(FieldTerminal),

    // A step of values (terminal, step_size)
    Step(FieldTerminal, u8),
}

impl std::fmt::Display for FieldItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldItem::Plain(terminal) => write!(f, "{}", terminal),
            FieldItem::Step(terminal, step) => write!(f, "{}/{}", terminal, step),
        }
    }
}

// A singular value is treated as a list of length 1
#[derive(PartialEq)]
pub struct Field(Vec<FieldItem>);

// Allow Field struct to be built from an iterator
impl FromIterator<FieldItem> for Field {
    fn from_iter<T: IntoIterator<Item = FieldItem>>(iter: T) -> Self {
        Field(iter.into_iter().collect())
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items: Vec<String> = self.0.iter().map(|item| item.to_string()).collect();

        write!(f, "{}", items.join(","))
    }
}

#[derive(PartialEq)]
pub struct Schedule {
    minute: Field,
    hour: Field,
    day_of_month: Field,
    month: Field,
    day_of_week: Field,
}

impl Schedule {
    pub fn new(
        minute: Field,
        hour: Field,
        day_of_month: Field,
        month: Field,
        day_of_week: Field,
    ) -> Self {
        Self {
            minute,
            hour,
            day_of_month,
            month,
            day_of_week,
        }
    }
}

impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = [
            "--- Cron Schedule ---".to_string(),
            format!("{}: {}", FieldKind::Minute, self.minute),
            format!("{}: {}", FieldKind::Hour, self.hour),
            format!("{}: {}", FieldKind::DayOfMonth, self.day_of_month),
            format!("{}: {}", FieldKind::Month, self.month),
            format!("{}: {}", FieldKind::DayOfWeek, self.day_of_week),
        ];

        write!(f, "{}", lines.join("\n"))
    }
}
