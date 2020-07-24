use std::collections::HashMap;
use std::fmt;
use std::path::Path;

mod split_unquoted;
use split_unquoted::SplitUnquoted;

#[derive(Debug)]
pub struct XMLNode {
    pub tag: String,
    attributes: HashMap<String, String>,
    children: HashMap<String, Vec<XMLNode>>,
    pub content: String,
}

struct Payload<'a> {
    prolog: &'a str,
    node: Option<XMLNode>,
    remaining: &'a str,
}

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ContentOutsideRoot(usize),
    MissingClosingTag(String, usize),
    MissingClosingDelimiter(usize),
    MissingAttributeValue(String, usize),
    MissingQuotes(String, usize),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

fn validate_root(root: Result<Payload, Error>) -> Result<XMLNode, Error> {
    match root {
        Ok(v) if v.prolog.len() != 0 => Err(Error::ContentOutsideRoot(999)),
        Ok(v) => Ok(v.node.unwrap_or(XMLNode {
            tag: String::new(),
            content: String::new(),
            children: HashMap::new(),
            attributes: HashMap::new(),
        })),
        Err(e) => Err(e),
    }
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<XMLNode, Error> {
    validate_root(load_from_slice(&std::fs::read_to_string(path)?))
}

pub fn load_from_string(string: &str) -> Result<XMLNode, Error> {
    validate_root(load_from_slice(string))
}

/// Loads a xml structure from a slice
/// Ok variant contains a payload with the child node, name prolog, and remaining stringtuple with (prolog, tag_name, tag_data, remaining_from_in)
fn load_from_slice(string: &str) -> Result<Payload, Error> {
    let opening_del = match string.find("<") {
        Some(v) => v,
        None => {
            return Ok(Payload {
                prolog: "",
                node: None,
                remaining: string,
            });
        }
    };

    let closing_del = match string.find(">") {
        Some(v) => v,
        None => return Err(Error::MissingClosingDelimiter(999)),
    };

    let mut tag_parts = SplitUnquoted::split(&string[opening_del + 1..closing_del], ' ');

    let tag_name = tag_parts.next().unwrap().trim();

    // Collect the prolog as everything before opening tag excluding whitespace
    let prolog = string[..opening_del].trim();

    // Is a comment
    // Attempt to read past comment
    if &tag_name[0..1] == "?" {
        return load_from_slice(&string[closing_del + 1..]);
    }

    let mut attributes = HashMap::new();
    for part in tag_parts {
        // Last closing of empty node
        if part == "/" {
            break;
        }

        let equal_sign = match part.find("=") {
            Some(v) => v,
            None => return Err(Error::MissingAttributeValue(part.to_owned(), 999)),
        };

        // Get key and value from attribute
        let (k, v) = part.split_at(equal_sign);

        // Remove quotes from value
        let v = if &v[1..2] == "\"" && &v[v.len() - 1..] == "\"" {
            &v[2..v.len() - 1]
        } else {
            return Err(Error::MissingQuotes(part.to_owned(), 999));
        };
        attributes.insert(k.to_owned(), v.to_owned());
    }

    // Empty but valid node
    if string[opening_del + 1..closing_del].ends_with("/") {
        return Ok(Payload {
            prolog,
            node: Some(XMLNode {
                tag: tag_name.to_owned(),
                children: HashMap::new(),
                attributes: attributes,
                content: String::new(),
            }),
            remaining: &string[closing_del + 1..],
        });
    }

    // Find the closing tag index
    let closing_tag = match string.find(&format!("</{}>", tag_name)) {
        Some(v) => v,
        None => return Err(Error::MissingClosingTag(tag_name.to_owned(), 999)),
    };

    let mut content = String::with_capacity(512);
    let mut children = HashMap::new();

    // Load the inside contents and children
    let mut buf = &string[closing_del + 1..closing_tag];

    while buf.len() != 0 {
        let payload = load_from_slice(buf)?;

        if let Some(child) = payload.node {
            let v = children
                .entry(child.tag.clone())
                .or_insert(Vec::with_capacity(1));
            v.push(child);
        }

        // Nothing was read by child, no more nodes
        if payload.remaining.as_ptr() == buf.as_ptr() {
            break;
        }

        // Put what was before the next tag into the content of the parent tag
        content.push_str(&payload.prolog);
        buf = payload.remaining;
    }

    // Add the remaining inside content to content after no more nodes where found
    content.push_str(buf);

    let remaining = &string[closing_tag + tag_name.len() + 3..];

    Ok(Payload {
        prolog,
        node: Some(XMLNode {
            tag: tag_name.to_owned(),
            attributes,
            children,
            content: content.trim().into(),
        }),
        remaining,
    })
}

impl XMLNode {
    /// Creates a new freestanding node with no attributes and children
    /// Children and attributes can be added later
    /// Content is taken owned as to avoid large copy
    pub fn new(tag: &str, content: String) -> XMLNode {
        XMLNode {
            attributes: HashMap::new(),
            tag: tag.to_owned(),
            content: content,
            children: HashMap::new(),
        }
    }

    /// Returns a list of all child nodes with the specified tag
    pub fn get_children(&self, tag: &str) -> Option<&Vec<XMLNode>> {
        self.children.get(tag)
    }

    /// Adds or updates an attribute
    /// If an attribute with that key already exists it is returned
    pub fn add_attribute(&mut self, key: &str, val: &str) -> Option<String> {
        self.attributes.insert(key.to_owned(), val.to_owned())
    }

    /// Inserts a new child node with the name of the node field
    pub fn add_child(&mut self, node: XMLNode) {
        let v = self
            .children
            .entry(node.tag.clone())
            .or_insert(Vec::with_capacity(1));
        v.push(node);
    }

    // Converts an xml structure to a string with whitespace formatting
    pub fn to_string_pretty(&self) -> String {
        fn internal(node: &XMLNode, depth: usize) -> String {
            if node.tag == "" {
                return "".to_owned();
            }

            match node.children.len() + node.content.len() {
                0 => format!(
                    "{indent}<{}{}/>\n",
                    node.tag,
                    node.attributes
                        .iter()
                        .map(|(k, v)| format!(" {}=\"{}\"", k, v))
                        .collect::<String>(),
                    indent = " ".repeat(depth * 4)
                ),
                _ => format!(
                    "{indent}<{tag}{attr}>{beg}{children}{content}{end}</{tag}>\n",
                    tag = node.tag,
                    attr = node
                        .attributes
                        .iter()
                        .map(|(k, v)| format!(" {}=\"{}\"", k, v))
                        .collect::<String>(),
                    children = node
                        .children
                        .iter()
                        .flat_map(|(_, nodes)| nodes.iter())
                        .map(|node| internal(node, depth + 1))
                        .collect::<String>(),
                    beg = match node.children.len() {
                        0 => "",
                        _ => "\n",
                    },
                    end = match node.children.len() {
                        0 => "".to_owned(),
                        _ => " ".repeat(depth * 4),
                    },
                    content = node.content,
                    indent = " ".repeat(depth * 4),
                ),
            }
        }
        internal(&self, 0)
    }
}

impl std::fmt::Display for XMLNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        if self.tag == "" {
            return write!(f, "");
        }

        match self.children.len() + self.content.len() {
            0 => write!(
                f,
                "<{}{}/>",
                self.tag,
                self.attributes
                    .iter()
                    .map(|(k, v)| format!(" {}=\"{}\"", k, v))
                    .collect::<String>(),
            ),
            _ => write!(
                f,
                "<{tag}{attr}>{children}{content}</{tag}>",
                tag = self.tag,
                attr = self
                    .attributes
                    .iter()
                    .map(|(k, v)| format!(" {}=\"{}\"", k, v))
                    .collect::<String>(),
                children = self
                    .children
                    .iter()
                    .flat_map(|(_, nodes)| nodes.iter())
                    .map(|node| node.to_string())
                    .collect::<String>(),
                content = self.content,
            ),
        }
    }
}
