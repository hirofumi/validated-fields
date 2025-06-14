# validated-fields

A Rust procedural macro for generating structs that accumulate validation errors across all fields rather than stopping at the first failure.

## Overview

When validating structs with multiple fields, we often want to collect all validation errors rather than stopping at the first one.
`ValidatedFields` solves this by generating companion structs with `Validated<T, E>` fields for collecting validation errors across multiple fields.

## Example

```rust
use anyhow::Context;
use validated::Validated;
use validated::Validated::Good;
use validated_fields::ValidatedFields;

#[derive(ValidatedFields)]
struct Person {
    name: String,
    age: u8,
    address: Address,
}

#[derive(ValidatedFields)]
struct Address {
    street: String,
    city: String,
    floor: u32,
}

let person = PersonValidatedFields {
    name: Good("Alex".to_string()),
    age: "not_a_number".parse::<u8>()
        .context("invalid age")
        .into(),
    address: AddressValidatedFields {
        street: Good("123 Main St".to_string()),
        city: Good("Somewhere".to_string()),
        floor: "not_a_number".parse::<u32>()
            .context("invalid floor")
            .into(),
    }
    .into(),
};

let result: Validated<Person, anyhow::Error> = person.into();
```
