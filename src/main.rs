fn main() {
    println!("XML: {:?}", xml::load_from_file("./examples/person.xml"));
    println!("XML: {:?}", xml::load_from_file("./examples/note.xml"));
}
