# Type-Safe Builder Example

A Rust builder struct which is type-safe, i.e. provision of required field is
checked at compile time.

The following code snippet will not compile if the `name` method is missing, or
called twice. Likewise for the `age` method.

```rs
let person = Person::builder()
    .name("John")
    .age(42)
    .build();
println!("{:#?}", person);
```

