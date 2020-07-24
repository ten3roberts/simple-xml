use std::collections::HashMap;
use std::path::Path;
use std::{fmt, ops};

mod split_unquoted;
use split_unquoted::SplitUnquoted;

#[derive(Debug)]
pub struct XMLNode {
    tag: String,
    attributes: HashMap<String, String>,
    children: HashMap<String, XMLNode>,
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

    println!("tag_name: {}", tag_name);

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

    // println!("Buf: {}", buf);
    while buf.len() != 0 {
        let payload = load_from_slice(buf)?;

        if let Some(child) = payload.node {
            println!("Inserting: {}", child.tag);
            children.insert(child.tag.to_owned(), child);
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
    pub fn get(&self, tag: &str) -> Option<&XMLNode> {
        self.children.get(tag)
    }

    /// Inserts a new child node with the given tag
    /// The name of the node will be overwritten to that of tag
    /// If a node of that name already is present, it is returned
    pub fn insert(&mut self, tag: &str, mut node: XMLNode) -> Option<XMLNode> {
        node.tag = tag.to_owned();
        self.children.insert(node.tag.to_owned(), node)
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
                    "{indent}<{tag}{attr}>{break}{children}{content}</{tag}>\n",
                    tag = node.tag,
                    attr = node
                        .attributes
                        .iter()
                        .map(|(k, v)| format!(" {}=\"{}\"", k, v))
                        .collect::<String>(),
                    children = node
                        .children
                        .iter()
                        .map(|(_, node)| internal(node, depth + 1))
                        .collect::<String>(),
                    break = if node.children.len() != 0 {
                        format!("\n{}", " ".repeat(depth * 4))
                    } else {
                        "".to_owned()
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
                    .map(|(_, node)| node.to_string())
                    .collect::<String>(),
                content = self.content,
            ),
        }
    }
}
