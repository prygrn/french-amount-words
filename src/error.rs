use std::fmt;

use crate::MAX_VALUE;

/// Failure to spell out a number or an amount in words.
///
/// A `match` outside the crate must include a `_` arm, so that new variants can be added
/// without breaking compatibility:
///
/// ```compile_fail,E0004
/// use french_amount_words::AmountWordsError;
///
/// fn too_large_value(error: &AmountWordsError) -> u64 {
///     match error {
///         AmountWordsError::ValueTooLarge { value } => *value,
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AmountWordsError {
    /// The value exceeds [`crate::MAX_VALUE`].
    ValueTooLarge {
        /// Value compared with [`crate::MAX_VALUE`]: the input of [`crate::number_to_words`],
        /// but the euro part (not the input in cents) for
        /// [`crate::euro_amount_to_words`].
        value: u64,
    },
}

impl fmt::Display for AmountWordsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValueTooLarge { value } => write!(
                formatter,
                "value {value} exceeds the maximum convertible value {MAX_VALUE}"
            ),
        }
    }
}

impl std::error::Error for AmountWordsError {}
