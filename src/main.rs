use typesafe_builder::{Person, PersonBuilder};

fn main() {
    // Chained methods
    let person = Person::builder()
        .name("John".to_string())
        .age(42)
        .push_friend("Bob".to_string())
        .partner("Jane".to_string())
        .dead()
        .dead()
        .build();
    println!("{:#?}", person);

    // Separate statements
    let builder: PersonBuilder<false, false, false> = PersonBuilder::new();
    let builder: PersonBuilder<true, false, false> = builder.name("John".to_string());
    let mut builder: PersonBuilder<true, true, false> = builder.age(42);
    builder = builder.push_friend("Bob".to_string());
    let person = builder.build();
    println!("{:#?}", person);
}
