use anyhow::Context;
use itertools::Itertools;
use validated::Validated;

use validated_fields::ValidatedFields;

#[derive(ValidatedFields)]
struct Foo {
    x: i32,
    y: i32,
    bar: Bar,
}

#[derive(ValidatedFields)]
struct Bar {
    z: i32,
}

#[test]
fn anyhow() {
    let validated: Validated<_, _> = FooValidatedFields {
        x: "X".parse().context("invalid x").into(),
        y: "2".parse().context("invalid y").into(),
        bar: BarValidatedFields {
            z: "Z".parse().context("invalid z").into(),
        }
        .into(),
    }
    .into();

    assert_eq!(
        validated
            .ok()
            .err()
            .map(|es| es.into_iter().map(|e| e.to_string()).join("; ")),
        Some("invalid x; invalid z".to_string()),
    );
}
