use std::convert::Infallible;

use nonempty_collections::nev;
use validated::Validated;
use validated::Validated::{Fail, Good};

use validated_fields::ValidatedFields;

#[derive(Debug, PartialEq, ValidatedFields)]
struct Outer {
    a: i32,
    b: i32,
    inner: Inner,
}

#[derive(Debug, PartialEq, ValidatedFields)]
struct Inner {
    x: i32,
    y: i32,
}

#[test]
fn nested_good() {
    let validated: Validated<_, Infallible> = OuterValidatedFields {
        a: Good(1),
        b: Good(2),
        inner: InnerValidatedFields {
            x: Good(3),
            y: Good(4),
        }
        .into(),
    }
    .into();

    assert_eq!(
        validated,
        Good(Outer {
            a: 1,
            b: 2,
            inner: Inner { x: 3, y: 4 },
        })
    );
}

#[test]
fn nested_fail() {
    let validated: Validated<_, _> = OuterValidatedFields {
        a: Good(1),
        b: Fail(nev!["invalid b".to_string()]),
        inner: InnerValidatedFields {
            x: Good(3),
            y: Fail(nev!["invalid y", "extra error"]),
        }
        .map_err(|e| format!("inner: {e}"))
        .into(),
    }
    .into();

    assert_eq!(
        validated,
        Fail(nev![
            "invalid b".to_string(),
            "inner: invalid y".to_string(),
            "inner: extra error".to_string()
        ]),
    );
}
