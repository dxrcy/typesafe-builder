use typesafe_builder::{Person, PersonBuilder};

fn main() {
    // Chained methods
    let person = Person::builder()
        .name("John")
        .age(42)
        .push_friend("Bob")
        .partner("Jane")
        .dead()
        .dead()
        .build();
    println!("{:#?}", person);

    // Separate statements
    let builder: PersonBuilder<false, false, false> = PersonBuilder::new();
    let builder: PersonBuilder<true, false, false> = builder.name("John");
    let mut builder: PersonBuilder<true, true, false> = builder.age(42);
    builder = builder.push_friend("Bob");
    let person = builder.build();
    println!("{:#?}", person);
}
