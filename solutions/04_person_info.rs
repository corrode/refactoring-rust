// ANCHOR: listing

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

/// Format a description using the exercise's fixed age bands.
pub fn person_info(person: &Person) -> String {
    match person {
        Person { name, age: 0..=19 } => format!("{name} is a teenager."),
        Person { name, age: 20..=39 } => format!("{name} is an adult."),
        Person { name, .. } => format!("{name} is middle-aged."),
    }
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn person_age_boundaries() {
        for (age, label) in [
            (0, "a teenager"),
            (19, "a teenager"),
            (20, "an adult"),
            (39, "an adult"),
            (40, "middle-aged"),
            (u8::MAX, "middle-aged"),
        ] {
            let person = Person {
                name: "Alice".into(),
                age,
            };
            assert_eq!(
                person_info(&person),
                format!("Alice is {label}."),
                "age {age}"
            );
        }
    }

    #[test]
    fn person_names_are_formatted_verbatim() {
        for name in ["", "Zoë 🦀", "{name}"] {
            let person = Person {
                name: name.into(),
                age: 20,
            };
            assert_eq!(person_info(&person), format!("{name} is an adult."));
        }
    }

    #[test]
    fn person_remains_usable_after_formatting() {
        let person = Person {
            name: "Alice".into(),
            age: 40,
        };
        let description = person_info(&person);
        assert_eq!(description, "Alice is middle-aged.");
        assert_eq!(person.name, "Alice");
        assert_eq!(person.age, 40);
        assert_eq!(person_info(&person), description);
    }
}
// ANCHOR_END: tests
