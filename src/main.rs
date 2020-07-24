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

fn create_person(num_friends: usize, depth: usize) -> xml::XMLNode {
    let mut person = xml::XMLNode::new("person", String::new());
    person.add_child(xml::XMLNode::new(
        "name",
        NAMES.choose(&mut rand::thread_rng()).unwrap().to_string(),
    ));
    person.add_child(xml::XMLNode::new("address", "Rose Walk 3".to_owned()));
    let mut balance = xml::XMLNode::new("balance", "5".to_owned());
    balance.add_attribute("currency", "pound");
    person.add_child(balance);

    let mut friends = xml::XMLNode::new("friends", String::new());

    if depth > 0 {
        for _ in 0..num_friends {
            friends.add_child(create_person(num_friends, depth - 1));
        }
    }
    person.add_child(friends);
    person
}

fn main() {
    println!("{}", xml::load_from_file("./examples/note.xml").unwrap());

    let person = create_person(4, 8);
    std::fs::write("./examples/person_gen.xml", person.to_string_pretty()).unwrap();
}
