use crate::{AmountWordsError, MAX_VALUE};

pub(crate) const MILLION: u64 = 1_000_000;

const MILLIARD: u64 = 1_000_000_000;
const THOUSAND: u64 = 1_000;
const HUNDRED: u64 = 100;
const SMALLEST_PLURAL_QUANTITY: u64 = 2;

const ZERO_WORD: &str = "zéro";
const HUNDRED_WORD: &str = "cent";
const THOUSAND_WORD: &str = "mille";
const PLURAL_MARK: &str = "s";
const WORD_SEPARATOR: &str = " ";

/// Words from 0 to 19, indexed by their value; zero never appears in a compound number.
const BELOW_TWENTY_WORDS: [&str; 20] = [
    "", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf", "dix", "onze",
    "douze", "treize", "quatorze", "quinze", "seize", "dix-sept", "dix-huit", "dix-neuf",
];

/// Remainders joined to a ten by "et" (and): "vingt et un", "soixante et onze".
const REMAINDERS_JOINED_WITH_ET: [u64; 2] = [1, 11];

/// "quatre-vingt" multiplies "vingt": it takes the plural and never takes "et".
const MULTIPLIED_VINGT: u64 = 80;

/// Base tens in increasing order; 70 and 90 are built from "soixante" and
/// "quatre-vingt" followed by 10 to 19, hence no base for 70 and 90.
const TENS: [(u64, &str); 6] = [
    (20, "vingt"),
    (30, "trente"),
    (40, "quarante"),
    (50, "cinquante"),
    (60, "soixante"),
    (MULTIPLIED_VINGT, "quatre-vingt"),
];

/// Scale nouns: unlike "mille", they agree in number and let "vingt" and "cent" agree
/// before them.
const SCALE_NOUNS: [(u64, &str); 2] = [(MILLIARD, "milliard"), (MILLION, "million")];

/// Grammatical number of a word, which decides its plural mark.
#[derive(Clone, Copy)]
enum GrammaticalNumber {
    Singular,
    Plural,
}

impl GrammaticalNumber {
    fn of_quantity(quantity: u64) -> Self {
        if quantity >= SMALLEST_PLURAL_QUANTITY {
            Self::Plural
        } else {
            Self::Singular
        }
    }

    fn plural_mark(self) -> &'static str {
        match self {
            Self::Singular => "",
            Self::Plural => PLURAL_MARK,
        }
    }
}

/// Spells out an integer in words, following the traditional spelling.
///
/// # Errors
///
/// Returns [`AmountWordsError::ValueTooLarge`] if `value` exceeds [`crate::MAX_VALUE`].
pub fn number_to_words(value: u64) -> Result<String, AmountWordsError> {
    if value > MAX_VALUE {
        return Err(AmountWordsError::ValueTooLarge { value });
    }
    if value == 0 {
        return Ok(ZERO_WORD.to_owned());
    }

    let mut parts: Vec<String> = Vec::new();
    for (scale, noun) in SCALE_NOUNS {
        let count = value / scale % THOUSAND;
        if count > 0 {
            parts.push(format!(
                "{} {noun}{}",
                group_words(count, GrammaticalNumber::Plural),
                GrammaticalNumber::of_quantity(count).plural_mark()
            ));
        }
    }

    let thousands = value / THOUSAND % THOUSAND;
    match thousands {
        0 => {}
        1 => parts.push(THOUSAND_WORD.to_owned()),
        // Before "mille", a numeral adjective, "vingt" and "cent" stay invariable:
        // "quatre-vingt mille", "deux cent mille".
        _ => parts.push(format!(
            "{} {THOUSAND_WORD}",
            group_words(thousands, GrammaticalNumber::Singular)
        )),
    }

    let units = value % THOUSAND;
    if units > 0 {
        parts.push(group_words(units, GrammaticalNumber::Plural));
    }

    Ok(parts.join(WORD_SEPARATOR))
}

/// Spells out a group from 1 to 999; `ending_number` is the grammatical number that
/// multiplied "vingt" and "cent" take when they end the group.
fn group_words(value: u64, ending_number: GrammaticalNumber) -> String {
    let hundreds = value / HUNDRED;
    let below_hundred = value % HUNDRED;

    let mut parts: Vec<String> = Vec::new();
    match hundreds {
        0 => {}
        1 => parts.push(HUNDRED_WORD.to_owned()),
        _ => {
            let hundred_number = if below_hundred == 0 {
                ending_number
            } else {
                GrammaticalNumber::Singular
            };
            parts.push(format!(
                "{} {HUNDRED_WORD}{}",
                below_twenty_word(hundreds),
                hundred_number.plural_mark()
            ));
        }
    }
    if below_hundred > 0 {
        parts.push(below_hundred_words(below_hundred, ending_number));
    }

    parts.join(WORD_SEPARATOR)
}

/// Spells out a number from 1 to 99, joining tens and units with a hyphen or with "et".
fn below_hundred_words(value: u64, ending_number: GrammaticalNumber) -> String {
    let Some(&(tens_value, tens_word)) = TENS
        .iter()
        .rev()
        .find(|(tens_value, _)| *tens_value <= value)
    else {
        return below_twenty_word(value).to_owned();
    };

    let is_multiplied_vingt = tens_value == MULTIPLIED_VINGT;
    let remainder = value - tens_value;
    if remainder == 0 {
        let tens_number = if is_multiplied_vingt {
            ending_number
        } else {
            GrammaticalNumber::Singular
        };
        return format!("{tens_word}{}", tens_number.plural_mark());
    }

    let remainder_word = below_twenty_word(remainder);
    if !is_multiplied_vingt && REMAINDERS_JOINED_WITH_ET.contains(&remainder) {
        format!("{tens_word} et {remainder_word}")
    } else {
        format!("{tens_word}-{remainder_word}")
    }
}

fn below_twenty_word(value: u64) -> &'static str {
    BELOW_TWENTY_WORDS[value as usize]
}

#[cfg(test)]
mod tests;
