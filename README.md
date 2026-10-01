# morphing-string

Morph one string into another, one edit at a time.

`morphing-string` computes a minimal sequence of single-character edits —
insertions, deletions, and substitutions — that turns one string into another,
then applies them one per call. Driving it from a render loop gives you a
text-morphing effect:

```rust
use morphing_string::MorphingString;

let mut string = MorphingString::new("kitten");
string.set_target("mittens");

while !string.progress().is_complete() {
    println!("{}", string.value());
    string.advance();
}

assert_eq!(string.value(), "mittens");
```

## Usage

Call `advance()` to apply one edit and get back a `Progress` telling you how
far along the morph is. Each call does a constant amount of work, independent of
how different the two strings are, so you can advance on whatever cadence suits
your app — one edit per frame, or a burst per tick.

[`advance`] is a no-op once the morph completes, so a loop of the form
`while !string.advance().is_complete()` is safe to run to exhaustion.

## Example

A terminal demo that morphs between lines of a poem, one edit per frame:

```sh
cargo run --example tui_poem
```

## Notes

- Zero runtime dependencies.
- Operates on `char`s, so multi-byte UTF-8 is handled correctly.
- Where the shortest sequence of edits is ambiguous, substitutions are
  preferred over insertions, and insertions over deletions, which keeps
  unchanged characters in place for as long as possible.

## License

Licensed under [the fuck around and find out license v0.1](https://owly.fans/license/fafol/).
