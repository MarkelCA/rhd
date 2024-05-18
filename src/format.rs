use clap::{builder::PossibleValue, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LineNumberFormat {
    Hexadecimal,
    Decimal,
}

// Can also be derived with feature flag `derive`
impl ValueEnum for LineNumberFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[LineNumberFormat::Hexadecimal, LineNumberFormat::Decimal]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            LineNumberFormat::Hexadecimal => PossibleValue::new("hex").help("Hexadecimal format"),
            LineNumberFormat::Decimal => PossibleValue::new("dec").help("Decimal format"),
        })
    }
}

impl std::fmt::Display for LineNumberFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for LineNumberFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}
