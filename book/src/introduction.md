<div style="text-align: center">

![Rulex Logo](./assets/logo.svg)

# Rulex

</div>

Rulex is a language that compiles to regular expressions. It is currently in an alpha stage and
will likely change substantially in the next few releases.

## Usage

Rulex can be used with a CLI or a Rust macro. See
[installation instructions](installation-instructions.md).

You should also enable Unicode support in your regex engine if it isn't supported by default.
[See instructions](./enabling-unicode-support.md).

## Basics

Rulex expressions (_rulexes_ for short) describe the syntactical structure of a text. There are
several kinds of expressions, which will be explained now.

This introduction assumes basic knowledge of regexes. If you aren't familiar with them, I highly
recommend [this introduction](https://www.regular-expressions.info/quickstart.html).

### Table of contents:

- [Summary](#summary)
- [Strings](#strings)
- [Concatenate expressions](#concatenate-expressions)
- [Alternatives](#alternatives)
- [Groups](#groups)
- [Repetitions](#repetitions)
  - [Greedy and lazy matching](#greedy-and-lazy-matching)
  - [Variants of repetition](#variants-of-repetition)
- [Character classes](#character-classes)
  - [About Unicode ranges](#about-unicode-ranges)
- [Unicode support](#unicode-support)
- [Negation](#negation)
- [Special character classes](#special-character-classes)
- [Non-printable characters](#non-printable-characters)
- [Boundaries](#boundaries)
- [Lookaround](#lookaround)
- [Range](#range)
- [Grapheme](#grapheme)

### Summary

Here you can see all the features at a glance. Don't worry, they will be explained in more detail
below.

On the left are rulex expressions, on the right are the equivalent regexes:

```rulex
# String
'hello world'                 # hello world

# Lazy repetition
'hello'{1,5}                  # (?:hello){1,5}?
'hello'*                      # (?:hello)*?
'hello'+                      # (?:hello)+?

# Greedy repetition
'hello'{1,5} greedy           # (?:hello){1,5}
'hello'* greedy               # (?:hello)*
'hello'+ greedy               # (?:hello)+

# Alternation
'hello' | 'world'             # hello|world

# Character classes
['aeiou']                     # [aeiou]
['p'-'s']                     # [p-s]

# Named character classes
[.] [w] [s] [n]               # .\w\s\n

# Combined
[w 'a' 't'-'z' U+15]          # [\wat-z\x15]

# Negated character classes
!['a' 't'-'z']                # [^at-z]

# Unicode
[Greek] U+30F Grapheme        # \p{Greek}\u030F\X

# Boundaries
<% %>                         # ^$
% 'hello' !%                  # \bhello\B

# Non-capturing groups
'terri' ('fic' | 'ble')       # terri(?:fic|ble)

# Capturing groups
:('test')                     # (test)
:name('test')                 # (?P<name>test)

# Lookahead/lookbehind
>> 'foo' | 'bar'              # (?=foo|bar)
<< 'foo' | 'bar'              # (?<=foo|bar)
!>> 'foo' | 'bar'             # (?!foo|bar)
!<< 'foo' | 'bar'             # (?<!foo|bar)

# Backreferences
:('test') ::1                 # (test)\1
:name('test') ::name          # (?P<name>test)\k<name>

# Ranges
range '0'-'999'               # 0|[1-9][0-9]{0,2}
range '0'-'255'               # 0|1[0-9]{0,2}|2(?:[0-4][0-9]?|5[0-5]?|[6-9])?|[3-9][0-9]?
```

### Strings

In Rulex, characters that should be matched as-is, are always wrapped in quotes. We can use
double quotes (`""`) or single quotes (`''`). Text wrapped in quotes is an expression we call a
_string_. It matches the exact content of the string:

```rulex
"test"
```

### Concatenate expressions

If we write several expressions in a row, they are matched one after the other:

```rulex
'hello' 'world' '!'     # matches the string "helloworld!"
```

In Rulex, whitespace is insignificant, except between quotes. This means that can add spaces
and line breaks to make it look clearer. We can also add comments to explain what the expressions
are doing. They start with a `#` and span until the end of the line:

```rulex
# this is a comment
'hello'     # this is also a comment
'world'     # and this
```

### Alternatives

What if we want to match multiple strings? In a regex, we can enumerate multiple alternatives,
divided by a `|`:

```regexp
one|two|three|four|five
```

The same works in Rulex:

```rulex
'one' | 'two' | 'three' | 'four' | 'five'
```

### Groups

Multiple expressions can be grouped together by wrapping them in `()`. This is useful when we have
multiple alternatives that all start or end with the same thing:

```rulex
'tang' ('ible' | 'ent' | 'o')
```

This matches the words _tangible_, _tangent_ and _tango_.

Groups can also be used to _capture_ their content, e.g. to replace it with something else. In
regexes, every group is a capturing group by default. This is not the case in rulex: Capturing
groups must be prefixed with `:`.

```rulex
:('foo')
```

Capturing groups are consecutively numbered, to be able to refer to them later:

```rulex
:('Max' | 'Laura') (' is ' | ' was ') :('asleep' | 'awake')
```

The first group, containing the name, has index **1**, the third group with the adverb has the index
**2**. The second group is skipped because it isn't capturing (it isn't prefixed with `:`).

This means that you can add non-capturing groups freely without accidentally changing the capturing
group numbers. However, it's usually better to use _named capturing groups_, so you don't need to
count groups and instead refer to each group by a name:

```rulex
:name('Max' | 'Laura') (' is ' | ' was ') :adverb('asleep' | 'awake')
```

### Repetitions

When we want to match an expression multiple times, it would be cumbersome to repeat our expression.
Instead, we can specify how often the expression should occur:

```rulex
('r' | 'w' | 'x' | '-'){9}
```

This matches an `r`, `w`, `x` or `-` character 9 times. For example, it would match the string
`rwxr-xr--`, or `xxrr-xr-w`.

What if we want to match strings of different lengths? Repetitions are quite flexible, so we can
specify a lower and upper bound for the number of repetitions:

```rulex
('r' | 'w' | 'x' | '-'){3,9}
```

#### Greedy and lazy matching

This matches at least 3 times and at most 9 times. The default repetition mode in rulex is _lazy_,
unlike regexes (which are greedy by default).

This means that rulex always tries to match an expression as few times as possible. This means that,
since rulexes are usually allowed to match only _part_ of the text, the above expression will always
stop after the third repetition.

> I'm considering to change this.

This is obviously not very useful in this case. So we can opt into greedy matching with the `greedy`
keyword:

```rulex
('r' | 'w' | 'x' | '-'){3,9} greedy
```

Now it will greedily match the expression as often as possible, up to 9 times.

#### Variants of repetition

If we want to match an expression arbitrarily often, without an upper bound, we can just omit it:

```rulex
'test'{3,} greedy
```

There are three repetitions that are very common: `{0,}` (zero or more), `{1,}` (one or more) and
`{0,1}` (zero or one). These have dedicated symbols, `*`, `+` and `?`:

```rulex
'test'*     # match zero times or more
'test'+     # match one time or more
'test'?     # match zero or one time
```

Note that these also require the `greedy` keyword to opt into greedy matching.

### Character classes

What if we want to match an arbitrary word? Enumerating every single word is obviously not feasible,
so what to do instead? We can simply enumerate the characters and repeat them:

```rulex
('a' | 'b' | 'c' | 'd' | 'e' |
 'f' | 'g' | 'h' | 'i' | 'j' |
 'k' | 'l' | 'm' | 'n' | 'o' |
 'p' | 'q' | 'r' | 's' | 't' |
 'u' | 'v' | 'w' | 'x' | 'y' | 'z')+
```

This is pretty verbose, but it could be worse. But this only matches lowercase letters. Also, we
programmers tend to be lazy, so there's a more convenient solution:

```rulex
['a'-'z' 'A'-'Z']+
```

What is this? The square brackets indicate that this is a _character class_. A character class
always matches exactly 1 character (more precisely, a Unicode code point). This character class
contains two ranges, one for lowercase letters and one for uppercase letters. Together, this
matches any character that is either a lowercase or uppercase letter.

It's also possible to add single characters, for example:

```rulex
['$' '_' 'a'-'z' 'A'-'Z']
```

When we have several characters in a character class that aren't part of a range, we can simply
put them into the same quotes:

```rulex
['$_' 'a'-'z' 'A'-'Z']
```

#### About Unicode ranges

What is a range, exactly? Let's see with an example:

```rulex
['0'-'z']
```

This doesn't seem to make sense, but does work. If you compile it to a regex and
[try it out](https://regexr.com/6hagq), you'll notice that it matches numbers, lowercase and uppercase
letters. However, it also matches a few other characters, e.g. the question mark `?`.

The reason is that rulex uses Unicode, a standard that assigns every character a numeric value.
When we write `'0'-'z'`, rulex assumes that we want to match any character whose numeric value
is somewhere between the value of `'0'` and the value of `'z'`. This works well for letters (e.g.
`'a'-'Z'`) and numbers (`'0'-'9'`), because these have consecutive numbers in Unicode. However,
there are some special characters between digits, uppercase letters and lowercase letters:

```rulex
Character       Unicode value
=============================
'0'             48
'1'             49
'2'             50
      ...
'9'             57
':'             58
';'             59
'<'             60
'='             61
'>'             62
'?'             63
'@'             64
'A'             65
'B'             66
      ...
'Z'             90
'['             91
'\'             92
']'             93
'^'             94
'_'             95
'`'             96
'a'             97
      ...
'z'             122
```

Why, you might ask? This is for [historical](https://en.wikipedia.org/wiki/ASCII#Overview)
[reasons](https://en.wikipedia.org/wiki/Unicode#History).

### Unicode support

The reason why Unicode was invented is that most people in the world don't speak English, and many
of them use languages with different alphabets. To support them, Unicode includes 144,697 characters
covering 159 different scripts. Since we have a standard that makes it really easy to support
different languages, there's no excuse for not use it.

The character class `['a'-'z' 'A'-'Z']` only recognizes Latin characters. What should we do instead?
We should use a
[Unicode category](https://en.wikipedia.org/wiki/Unicode_character_property#General_Category).
In this case, the obvious candidate is `Letter`. Rulex makes it very easy to use Unicode categories:

```rulex
[Letter]
```

That's it. This matches any letter from all 159 scripts! It's also possible to match any character
in a specific script:

```rulex
[Cyrillic Hebrew]
```

This matches a Cyrillic or Hebrew character. Not sure why you'd want to do that.

Some regex engines can also match Unicode properties other than categories and scripts. Probably
the most useful ones are

- `Alphabetic` (includes letters and marks that can appear in a word)
- `White_Space`
- `Uppercase`, `Lowercase`
- `Emoji`

You can see the full list of Unicode properties [here](./unicode-properties.md).

### Negation

Character classes are negated by putting a `!` in front of it. For example, `!['a'-'f']` matches
anything except a letter in the range from `a` to `f`.

It's also possible to negate Unicode properties individually. For example, `[Latin !Alphabetic]`
matches a code point that is either in the Latin script, or is not alphabetic.

### Special character classes

There are a few _shorthand character classes_: `word`, `digit`, `space`, `horiz_space` and
`vert_space`. They can be abbreviated with their first, letter: `w`, `d`, `s`, `h` and `v`. Like
Unicode properties, they must appear in square brackets.

The character classes `word`, `digit` and `space` exist to be compatible with regex engines, but
using them is not always a good idea, because their behavior is not consistent across all regex
engines:

- `word` matches a _word character_, i.e. a letter, digit or underscore. On regex engines with
  Unicode support, this should be equivalent to `[Alphabetic Mark Decimal_Number Connector_Punctuation Join_Control]`.
- `digit` matches a digit. Equivalent to `Decimal_Number` if the regex engine supports Unicode.
- `space` matches whitespace. Equivalent to `White_Space` if the regex engine supports Unicode.

The `vert_space` and `horiz_space` shorthands are consistent across regex engines:

- `horiz_space` matches horizontal whitespace (tabs and spaces). This is equivalent to
  `[U+09 Space_Separator]`.
- `vert_space` matches vertical whitespace. This is equivalent to `[U+0A-U+0D U+85 U+2028 U+2029]`.

There are two more shorthands: `[codepoint]` (or `[cp]` for short), matches any Unicode code point;
`[.]` matches any Unicode code point, _except_ the ASCII line break `\n`.

#### What if I don't need Unicode support?

You don't have to use `[word]`, `[digit]` or `[space]` if you know that the input is only ASCII.
Unicode-aware matching can be considerably slower. For example, the `[word]` character class
includes more than 100,000 code points, so matching a `[ascii_word]`, which includes only 63 code
points, is faster.

Rulex supports a number of ASCII-only shorthands:

| Character class  | Equivalent                              |
| ---------------- | --------------------------------------- |
| `[ascii]`        | `[U+00-U+7F]`                           |
| `[ascii_alpha]`  | `['a'-'z' 'A'-'Z']`                     |
| `[ascii_alnum]`  | `['0'-'9' 'a'-'z' 'A'-'Z']`             |
| `[ascii_blank]`  | `[' ' U+09],`                           |
| `[ascii_cntrl]`  | `[U+00-U+1F U+7F]`                      |
| `[ascii_digit]`  | `['0'-'9']`                             |
| `[ascii_graph]`  | `['!'-'~']`                             |
| `[ascii_lower]`  | `['a'-'z']`                             |
| `[ascii_print]`  | `[' '-'~']`                             |
| `[ascii_punct]`  | `` ['!'-'/' ':'-'@' '['-'`' '{'-'~'] `` |
| `[ascii_space]`  | `[' ' U+09-U+0D]`                       |
| `[ascii_upper]`  | `['A'-'Z']`                             |
| `[ascii_word]`   | `['0'-'9' 'a'-'z' 'A'-'Z' '_']`         |
| `[ascii_xdigit]` | `['0'-'9' 'a'-'f' 'A'-'F']`             |

Using them can improve performance, but be careful when you use them. If you aren't sure if the
input will ever contain non-ASCII characters, it's better to err on the side of correctness, and
use Unicode-aware character classes.

### Non-printable characters

Characters that can't be printed should be replaced with their hexadecimal Unicode code point. For
example, you may write `U+FEFF` to match the
[Zero Width No-Break Space](https://www.compart.com/en/unicode/U+FEFF).

There are also 6 non-printable characters with a name:

- `[n]` is equivalent to `[U+0A]`, the `\n` line feed.
- `[r]` is equivalent to `[U+0D]`, the `\r` carriage return.
- `[f]` is equivalent to `[U+0C]`, the `\f` form feed.
- `[a]` is equivalent to `[U+07]`, the "alert" or "bell" control character.
- `[e]` is equivalent to `[U+0B]`, the "escape" control character.

Other characters have to be written in their hexadecimal form. Note that you don't need to write
leading zeroes, i.e. `U+0` is just as ok as `U+0000`. However, it is conventional to write ASCII
characters with two digits and non-ASCII characters with 4, 5 or 6 digits depending on their length.

### Boundaries

Boundaries match a position in a string without consuming any code points. There are 4 boundaries:

- `%` matches a word boundary. It matches successfully if it is preceded, but not succeeded by a
  word character, or vice versa. For example, `[cp] % [cp]` matches `A;` and `;A`, but not `AA` or
  `;;`.
- `!%` matches a position that is _not_ a word boundary. For example, `[cp] !% [cp]` matches `aa`
  and `::`, but not `a:` or `:a`.
- `<%` matches the start of the string.
- `%>` matches the end of the string.

A word character is anything that matches `[word]`. If the regex engine is Unicode-aware, this is
`[Alphabetic Mark Decimal_Number Connector_Punctuation]`. For some regex engines, Unicode-aware
matching has to be enabled first ([see here](./enabling-unicode-support.md)).

In JavaScript, `%` and `!%` is _never_ Unicode-aware, even when the `u` flag is set.
[See here](./enabling-unicode-support.md#javascript) for more information.

### Lookaround

Lookarounds provide the ability to see if the characters before or after the current position
match a certain expression. There are four variants:

- `>>`, a positive lookahead. For example, `(>> [w])` matches if the position is followed by a word
  character. That character isn't included in the match.
- `<<`, a positive lookbehind. For example, `(<< [w])` matches if the position is directly after
  a word character.
- `!>>`, a negative lookahead. For example `(!>> [w])` matches if the position is _not_ followed by
  a word character. Note that this also matches at the end of the string, so it's not the same as
  `(>> ![w])`, which would require that the position is followed by at least one character.
- `!<<`, a negative lookbehind. For example `(!<< [w])` matches if the position is _not_ directly
  after a word character. This also matches at the start of the string, so it's not the same as
  `(<< ![w])`.

Lookaround makes it possible to match a string multiple times. For example,
<code class="language-rulex">(!>> ('_' | 'for' | 'while' | 'if') %) [w]+ %></code> matches a string
consisting of word characters, but not one of the keywords `_`, `for`, `while`and`if`. Be careful when using this technique, because the lookahead might not match the same length as the expression after it. Here, we ensured that both match until the end of the word with `%`.

Some regex engines don't allow arbitrary expressions in a lookbehind. For example, they might
forbid repetitions or expressions with an unknown length, such as `'hi' | 'world'`. The reason for
this is that they don't support backwards matching; instead, when they see a lookbehind such as
`(<< 'foo')`, they see that it has a length of 3 code points, so they go back 3 characters in the
string and match the expression `'foo'` forwards. This requires that the length of the match is
known. Rulex currently doesn't validates this for regex engines with such a requirement.

### Range

TODO

### Grapheme

TODO