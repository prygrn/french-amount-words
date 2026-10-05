# Journal des modifications

Toutes les modifications notables de ce crate sont consignées dans ce fichier.

Le format suit [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/) et le crate respecte le
[versionnage sémantique](https://semver.org/lang/fr/). Chaque version publiée correspond au tag
git `vX.Y.Z`.

## [Unreleased]

## [0.1.1] - 2026-10-05

### Added

- Lien vers la documentation docs.rs dans les métadonnées du paquet.
- README : badges (version, documentation, licence, CI).
- README : positionnement par rapport aux crates `french-numbers` et `nb2fr`.

## [0.1.0] - 2026-10-05

### Added

- `number_to_words` : écriture en toutes lettres d'un entier, en graphie traditionnelle.
- `euro_amount_to_words` : écriture en toutes lettres d'un montant en euros exprimé en
  centimes.
- `MAX_VALUE` : plus grande valeur convertible (`999_999_999_999`).
- `AmountWordsError::ValueTooLarge` : erreur renvoyée au-delà de `MAX_VALUE`.
- `AmountWordsError` est `#[non_exhaustive]` : un `match` hors du crate doit prévoir un bras `_`,
  ce qui permet d'ajouter des variantes sans rupture de compatibilité.

[Unreleased]: https://github.com/prygrn/french-amount-words/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/prygrn/french-amount-words/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/prygrn/french-amount-words/releases/tag/v0.1.0
