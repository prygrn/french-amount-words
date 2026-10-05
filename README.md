# french-amount-words

[![crates.io](https://img.shields.io/crates/v/french-amount-words.svg)](https://crates.io/crates/french-amount-words)
[![docs.rs](https://docs.rs/french-amount-words/badge.svg)](https://docs.rs/french-amount-words)
[![License: MIT](https://img.shields.io/crates/l/french-amount-words.svg)](LICENSE)
[![CI](https://github.com/prygrn/french-amount-words/actions/workflows/ci.yml/badge.svg)](https://github.com/prygrn/french-amount-words/actions/workflows/ci.yml)

Spells out integers and euro amounts in French words, following the traditional spelling.

The crate is self-contained: it depends on nothing but the standard library.
A typical use is writing a sum out in words on a rent receipt, an invoice or a cheque.

What sets it apart is the spelling of euro amounts ("un million d'euros", "un euro",
"cinquante centimes"…). To spell out integers in general, see also
[`french-numbers`](https://crates.io/crates/french-numbers) (MIT/Apache-2.0, spellings from
before and after the 1990 reform, feminine forms) or [`nb2fr`](https://crates.io/crates/nb2fr)
(GPL-3.0).

## Installation

```sh
cargo add french-amount-words
```

## Usage

```rust
use french_amount_words::{AmountWordsError, euro_amount_to_words, number_to_words};

fn main() -> Result<(), AmountWordsError> {
    assert_eq!(number_to_words(1234)?, "mille deux cent trente-quatre");
    assert_eq!(number_to_words(80)?, "quatre-vingts");
    assert_eq!(number_to_words(2_000_000)?, "deux millions");

    // Amounts are given in cents to avoid floating-point numbers.
    assert_eq!(
        euro_amount_to_words(123_456)?,
        "mille deux cent trente-quatre euros et cinquante-six centimes"
    );
    assert_eq!(euro_amount_to_words(100)?, "un euro");
    assert_eq!(euro_amount_to_words(50)?, "cinquante centimes");
    assert_eq!(euro_amount_to_words(100_000_000)?, "un million d'euros");
    Ok(())
}
```

## Upper bound

The constant `MAX_VALUE` is `999_999_999_999` ("neuf cent quatre-vingt-dix-neuf milliards…").

- `number_to_words` returns `AmountWordsError::ValueTooLarge` above `MAX_VALUE`.
- `euro_amount_to_words` returns the same error when the euro part exceeds `MAX_VALUE`;
  the error's `value` field then holds the euro part, not the input in cents.

## Spelling rules applied

The crate follows the traditional spelling described by the Académie française, not the 1990
spelling reform (which joins every element with hyphens).

- Hyphens only between elements below one hundred: `vingt-deux`, `dix-sept`,
  `quatre-vingt-dix-neuf`, but `deux cent un`, `mille cent`.
- "et" (and), without hyphens, for 21, 31, 41, 51, 61 and 71: `vingt et un`,
  `soixante et onze`. No "et" for 81 and 91: `quatre-vingt-un`, `quatre-vingt-onze`.
- `vingt` (twenty) and `cent` (hundred) take an "s" when they are multiplied and end the
  number: `quatre-vingts`, `deux cents`, but `quatre-vingt-un`, `deux cent un`.
- Before `mille` (thousand), a numeral adjective, they stay invariable: `quatre-vingt mille`,
  `deux cent mille`.
- `mille` is invariable and is never preceded by "un" (one): `mille`, `deux mille`.
  Likewise, `cent` is never preceded by "un".
- `million` and `milliard` (billion) are nouns: they take the plural mark and do not prevent
  `vingt` and `cent` from agreeing: `un million`, `deux millions`, `quatre-vingts millions`,
  `deux cents milliards`.
- Euro amounts:
  - `euro` and `centime` (cent) stay singular for zero and one: `zéro euro`, `un euro`,
    `un centime`;
  - "zéro centime" is never written (`deux euros`), nor is "zéro euro" when there are
    cents (`cinquante centimes`);
  - after `million` or `milliard` ending the euro part, the unit becomes "d'euros" (of
    euros): `un million d'euros`, but `un million deux cents euros`.

## Development

```sh
git clone --recurse-submodules https://github.com/prygrn/french-amount-words.git
cd french-amount-words
make setup  # installs the git hooks
make test   # unit tests and doctests
```

## License

MIT, see the `LICENSE` file.
