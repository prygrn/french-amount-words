//! Helpers shared by the crate's table-driven tests.

use crate::AmountWordsError;

pub(crate) type Conversion = fn(u64) -> Result<String, AmountWordsError>;

/// Checks each (input, expected output) pair, reporting the failing input.
pub(crate) fn assert_conversions(convert: Conversion, cases: &[(u64, &str)]) {
    for &(input, expected) in cases {
        let actual = convert(input);

        assert_eq!(actual.as_deref(), Ok(expected), "input: {input}");
    }
}
