# Liquidity Check

A rust library for checking if a string represents a valid monetary value.

```rust
assert_eq!(validate("$50"), true);
assert_eq!(validate("€ 50"), true);
assert_eq!(validate("50 EUR"), true);
assert_eq!(validate("50.0 ¥"), true);
assert_eq!(validate("50"), false);
assert_eq!(validate("50 ER"), false);
assert_eq!(validate("50_$"), false);
assert_eq!(validate("50,000 PAB"), true);
```

## Note

The current implementation is quite simple, so in some edge cases it's possible to get false positives.