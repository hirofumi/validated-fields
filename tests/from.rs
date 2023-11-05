use validated::Validated;
use validated::Validated::Good;

use validated_fields::ValidatedFields;

#[derive(Clone, Debug, PartialEq, ValidatedFields)]
struct Foo {
    a: i32,
    b: i32,
}

#[test]
fn round_trip() {
    let original = Foo { a: 1, b: 2 };

    assert_eq!(
        Validated::from(FooValidatedFields::from(original.clone())),
        Good(original),
    );
}
