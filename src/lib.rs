use std::mem::MaybeUninit;

/// Example type
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub partner: Option<String>,
    pub friends: Vec<String>,
    pub is_dead: bool,
}

/// Type-safe builder type for `Person`
// Individual const parameters are required; cannot use aggregate type
pub struct PersonBuilder<const NAME: bool, const AGE: bool, const PARTNER: bool> {
    // Two fields which each must be given exactly once
    name: MaybeUninit<String>,
    age: MaybeUninit<u8>,
    // A field which may be given once (defaults to `None`)
    partner: Option<String>,
    // Two fields which can be updated any number of times and aren't affected by const parameters
    friends: Vec<String>,
    is_dead: bool,
}

impl Person {
    /// Create a new `Person` builder
    pub fn builder() -> PersonBuilder<false, false, false> {
        PersonBuilder::new()
    }
}

impl PersonBuilder<false, false, false> {
    /// Create a new `Person` builder
    pub fn new() -> Self {
        Self {
            name: MaybeUninit::uninit(),
            age: MaybeUninit::uninit(),
            partner: None,
            friends: Vec::new(),
            is_dead: false,
        }
    }
}

impl<const PARTNER: bool> PersonBuilder<true, true, PARTNER> {
    /// Convert builder to a valid `Person` instance
    pub fn build(self) -> Person {
        Person {
            name: unsafe { self.name.assume_init() },
            age: unsafe { self.age.assume_init() },
            partner: self.partner,
            friends: self.friends,
            is_dead: self.is_dead,
        }
    }
}

impl<const AGE: bool, const PARTNER: bool> PersonBuilder<false, AGE, PARTNER> {
    /// Set the person's name
    pub fn name(self, name: String) -> PersonBuilder<true, AGE, PARTNER> {
        PersonBuilder {
            name: MaybeUninit::new(name),
            age: self.age,
            partner: self.partner,
            friends: self.friends,
            is_dead: self.is_dead,
        }
    }
}

impl<const NAME: bool, const PARTNER: bool> PersonBuilder<NAME, false, PARTNER> {
    /// Set the person's age
    pub fn age(self, age: u8) -> PersonBuilder<NAME, true, PARTNER> {
        PersonBuilder {
            name: self.name,
            age: MaybeUninit::new(age),
            partner: self.partner,
            friends: self.friends,
            is_dead: self.is_dead,
        }
    }
}

impl<const NAME: bool, const AGE: bool> PersonBuilder<NAME, AGE, false> {
    /// Set the person's partner
    pub fn partner(self, partner: String) -> PersonBuilder<NAME, AGE, true> {
        PersonBuilder {
            name: self.name,
            age: self.age,
            partner: Some(partner),
            friends: self.friends,
            is_dead: self.is_dead,
        }
    }
}

impl<const NAME: bool, const AGE: bool, const PARTNER: bool> PersonBuilder<NAME, AGE, PARTNER> {
    /// Add a friend of the person
    pub fn push_friend(mut self, friend_name: String) -> Self {
        self.friends.push(friend_name);
        self
    }

    /// Set the person's status to dead
    pub fn dead(mut self) -> Self {
        self.is_dead = true;
        self
    }
}
