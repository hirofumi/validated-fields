use validated_fields::ValidatedFields;

#[derive(Debug, PartialEq, ValidatedFields)]
struct Foo<X> {
    x: X,
}
