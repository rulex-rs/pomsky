# Changelog

All notable changes to the _Pomsky regular expression language_ will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2022-07-04

This is the first release since [Rulex was renamed to Pomsky](https://pomsky-lang.org/blog/renaming-rulex/).

If you are using the `rulex` crate, replace it with `pomsky`. The `rulex-macro` crate should be replaced with `pomsky-macro`. To install the new binary, [see instructions](https://github.com/rulex-rs/pomsky/releases/tag/v0.5). If you installed rulex with cargo, you can remove it with

```sh
rm $(type -P rulex)
```

### Added

- Deprecation warnings for `<%`, `%>`, `[codepoint]`, `[cp]` and `[.]`. These were deprecated
  before, but Pomsky wasn't able to show warnings until now.

### Changed

- Improved codegen for Unicode chars between 128 and 255

- Some diagnostics involving built-in variables were improved

- The words `atomic`, `if`, `else` and `recursion` are now reserved

### Fixed

- `Grapheme` is now only allowed in the PCRE, Java and Ruby flavors. Previously, it was accepted by
  Pomsky for some flavors that don't support `\X`.
- Keywords and reserved words are no longer accepted as variable names

### Library changes

- The `Rulex` struct was renamed to `Expr`, and `RulexFeatures` was renamed to `PomskyFeatures`
- `Span::range()` now returns an `Option<Range<usize>>` instead of a `Range<usize>`
- `Expr::parse` and `Expr::parse_and_compile` now return a `(String, Vec<Warning>)` tuple

## [0.4.3] - 2022-06-19

### Added

- Add libFuzzer and AFL fuzzing boilerplate to find panics

- Add artificial recursion limit during parsing to prevent stack exhaustion.
  _This means that groups can be nested by at most 127 levels. I don't think you'll ever run into this limitation, but if you do, you can refactor your expression into variables._

### Fixed

- Fixed crash caused by slicing into a multi-byte UTF-8 code point after a backslash or in a string
- Fixed crash caused by stack exhaustion when parsing a very deeply nested expression

## [0.4.2] - 2022-06-16

### Added

- Built-in variables were added:

  - `Start` as an alias for `<%`, which matches the start of the string
  - `End` as an alias for `%>`, which matches the end of the string
  - `Codepoint` and `C` as aliases for `[codepoint]`, matching a single code point
  - `G` as an alias for `Grapheme`, matching an extended grapheme cluster

- `Grapheme` was turned from a keyword into a built-in variable.

- The repository now has issue templates and a pull request template.

### Important note

`<%`, `%>`, `[codepoint]`, `[cp]` and `[.]` will be deprecated in the future. It is recommended
to use `Start`, `End` and `Codepoint`/`C` instead.

There won't be a replacement for `[.]`, but you can use `![n]` to match any code point except
the ASCII line break.

### Fixed/improved

- [#29](https://github.com/rulex-rs/pomsky/pull/29): Fix a miscompilation of a repeated empty group,
  e.g. `()?`. Thanks, [sebastiantoh](https://github.com/sebastiantoh)!

- Make the parser more permissive to parse arbitrary negated expressions. This results in better
  error messages.

- Add missing help messages to diagnostics and fix a few that were broken:

  - When parsing `^`: _Use `Start` to match the start of the string_
  - When parsing `$`: _Use `End` to match the end of the string_
  - When parsing e.g. `(?<grp>)`: _Named capturing groups use the `:name(...)` syntax. Try `:grp(...)` instead_
  - When parsing e.g. `\4`: _Replace `\\4` with `::4`_
  - When parsing e.g. `(?<=test)`: _Lookbehind uses the `<<` syntax. For example, `<< 'bob'` matches if the position is preceded with bob._
  - When parsing e.g. `(?<!test)`: _Negative lookbehind uses the `!<<` syntax. For example, `!<< 'bob'` matches if the position is not preceded with bob._

- Improve test suite: Help messages are now tested as well, and failing tests can be "blessed" when
  the output has changed. Test coverage was also improved.

- The entire public API is now documented.

## [0.4.1] - 2022-06-03

### Fixed

- Fixed a miscompilation in situations where a variable followed by a `?` expands to a repetition

## [0.4.0] - 2022-06-03

The repository was moved to its own organization! ???? It also has a new website with an
[online playground](https://playground.pomsky-lang.org/)!

### Added

- API to selectively disable some language features

- [Online playground](https://playground.pomsky-lang.org/) to try out Pomsky. You can write
  pomsky expressions on the left and immediately see the output on the right.

### Changed

- Ranges now have a maximum number of digits. The default is 6, but can be configured.

  This prevents DoS attacks when compiling untrusted input, since compiling ranges has exponential
  runtime with regard to the number of digits.

### Library changes

- `ParseOptions` was moved out of `CompileOptions`. This means that the
  [`parse_and_compile`](https://docs.rs/rulex/0.4.0/rulex/struct.Rulex.html#method.parse_and_compile)
  method now expects three parameters instead of two.

## [0.3.0] - 2022-03-29

### Added

- A [**book**](https://pomsky-lang.org/docs/), with instructions, a language tour and a formal
  grammar!

- **Variables**! For example, `let x = 'test';` declares a variable `x` that can be used below. Read
  [this chapter](https://pomsky-lang.org/docs/language-tour/variables) from the book to find
  out more.

- **Number range expressions**! For example, `range '0'-'255'` generates this regex:

  ```regexp
  0|1[0-9]{0,2}|2(?:[0-4][0-9]?|5[0-5]?|[6-9])?|[3-9][0-9]?
  ```

- **Relative references**: `::-1` refers to the previous capturing group, `::+1` to the next one

- `w`, `d`, `s`, `h`, `v` and `X` now have aliases: `word`, `digit`, `space`, `horiz_space`,
  `vert_space` and `Grapheme`.

- `enable lazy;` and `disable lazy;` to enable or disable lazy matching by default at the global
  scope or in a group.

### Changed

- **Made `greedy` the default** for repetitions. You can opt into lazy matching with the `lazy`
  keyword or globally with `enable lazy;`.

- **POSIX classes (e.g. `alnum`) have been renamed** to start with `ascii_`, since they only support
  Basic Latin

- Double quoted strings can now contain escaped quotes, e.g. `"\"test\""`. Backslashes now must be
  escaped. Single quoted strings were not changed.

- Improved Unicode support

  - In addition to Unicode general categories and scripts, pomsky now supports blocks and other
    boolean properties
  - Pomsky now validates properties and tells you when a property isn't supported by the target
    regex flavor
  - Shorthands (`[h]` and `[v]`) are substituted with character classes when required to support
    Unicode everywhere

- Named references compile to numeric references (like relative references), which are better
  supported

- A `?` after a repetition is now forbidden, because it easy confuse to with a lazy quantifier.
  The error can be silenced by wrapping the inner expression in parentheses, e.g. `([w]{3})?`.

### Removed

- `R` was removed, because it didn't work properly, and I'm still unsure about the best syntax
  and behavior.

### Fixed

- A `?` following a repetition no longer miscompiles: `([w]{3})?` now correctly emits `(?:\w{3})?`
  instead of `\w{3}?`.
- A `{0,42}` repetition no longer miscompiles (it previously emitted `{,42}`).

## [0.2.0] - 2022-03-12

### Changed

- Improved the Rust macro; pomsky expressions are written directly in the Rust source code, not in a
  string literal:
  ```rs
  let regex: &str = rulex!("hello" | "world" '!'+);
  ```
- There are a few limitations in the Rust macro due to the way Rust tokenizes code:
  - Strings with more than 1 code point must be enclosed in double quotes, single quotes don't work
  - Strings can't contain backslashes; this will be fixed in a future release
  - Code points must be written without the `+`, e.g. `U10FFFF` instead of `U+10FFFF`
  - Pomsky expressions can contain Rust comments; they can't contain comments starting with `#`

## [0.1.0] - 2022-03-11

Initial release

[unreleased]: https://github.com/rulex-rs/pomsky/compare/v0.5...HEAD
[0.5.0]: https://github.com/rulex-rs/pomsky/compare/v0.4.3...v0.5
[0.4.3]: https://github.com/rulex-rs/pomsky/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/rulex-rs/pomsky/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/rulex-rs/pomsky/compare/v0.4...v0.4.1
[0.4.0]: https://github.com/rulex-rs/pomsky/compare/v0.3...v0.4
[0.3.0]: https://github.com/rulex-rs/pomsky/compare/v0.2...v0.3
[0.2.0]: https://github.com/rulex-rs/pomsky/compare/v0.1...v0.2
[0.1.0]: https://github.com/rulex-rs/pomsky/releases/tag/v0.1
