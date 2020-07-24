fn main() {
    let person = xml::load_from_file("./examples/person.xml").expect("Failed to read xml file");

    println!("Name: '{}'", person.get("name").unwrap().content);

    println!("{}", person.to_string_pretty());
}
