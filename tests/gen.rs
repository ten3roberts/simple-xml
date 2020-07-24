#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;
    const NAMES: [&str; 21] = [
        "Jacob",
        "Emily",
        "Michael",
        "Madison",
        "Joshua",
        "Emma",
        "Matthew",
        "Olivia",
        "Daniel",
        "Hannah",
        "Christopher",
        "Abigail",
        "Andrew",
        "Isabella",
        "Ethan",
        "Samantha",
        "Joseph",
        "Elizabeth",
        "William",
        "Ashley",
        "Anthony",
    ];

    fn create_person(num_friends: usize, depth: usize) -> xml::Node {
        let mut person = xml::new("person", String::new());
        person.add_node(xml::new(
            "name",
            NAMES.choose(&mut rand::thread_rng()).unwrap().to_string(),
        ));
        person.add_node(xml::new("address", "Rose Walk 3".to_owned()));
        let mut balance = xml::new("balance", "5".to_owned());
        balance.add_attribute("currency", "pound");
        person.add_node(balance);

        let mut friends = xml::new("friends", String::new());

        if depth > 0 {
            for _ in 0..num_friends {
                friends.add_node(create_person(num_friends, depth - 1));
            }
        }
        person.add_node(friends);
        person
    }
    #[test]

    fn generate_person() {
        println!("{}", xml::from_file("./examples/note.xml").unwrap());

        let person = create_person(10, 2);
        std::fs::write("./examples/person_gen.xml", person.to_string_pretty()).unwrap();
    }
}
