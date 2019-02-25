use std::io::Read;
use xml::name::OwnedName;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Clone)]
pub struct Revision {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub title: String,
    pub revisions: Vec<Revision>,
}

impl Page {
    #[allow(dead_code)]
    pub fn new() -> Page {
        Page {
            title: String::from(""),
            revisions: vec![],
        }
    }
}

pub struct Parser<R: Read> {
    xml_parser: EventReader<R>,
    tag_stack: Vec<OwnedName>,
    content: String,
    page: Option<Page>,
    text: Option<String>,
    revisions: Vec<Revision>,
    title: Option<String>,
}

impl<R: Read> Parser<R> {
    #[allow(dead_code)]
    pub fn new(r: R) -> Parser<R> {
        Parser {
            xml_parser: EventReader::new(r),
            tag_stack: vec![],
            content: String::new(),
            page: None,
            text: None,
            revisions: vec![],
            title: None,
        }
    }

    fn reset_page(&mut self) {
        self.page = None;
        self.revisions = vec![];
        self.title = None;
    }

    fn proc_start_elem(&mut self, _lname: &str) {}

    fn proc_end_elem(&mut self, lname: &str) {
        if lname == "page" {
            let title = self.title.clone().expect("Cannon unwrap title");
            self.page = Some(Page {
                title: title,
                revisions: self.revisions.clone(),
            })
        } else if lname == "title" {
            self.title = Some(self.content.clone());
        } else if lname == "text" {
            self.text = Some(self.content.clone());
        } else if lname == "revision" {
            let text = self.text.clone().expect("Cannot unwrap text");
            let revision = Revision { text: text };
            self.revisions.push(revision);
            self.text = None
        }
    }
}

impl<R: Read> Iterator for Parser<R> {
    type Item = Page;

    fn next(&mut self) -> Option<Page> {
        self.content = String::new();
        loop {
            let e = self.xml_parser.next().expect("Cannot get next element");
            match e {
                XmlEvent::EndDocument => {
                    return None;
                }
                XmlEvent::StartElement { name, .. } => {
                    self.proc_start_elem(&name.local_name[..]);
                    self.tag_stack.push(name);
                    self.content.clear();
                }
                XmlEvent::EndElement { ref name, .. } => {
                    self.tag_stack.pop().unwrap();
                    self.proc_end_elem(&name.local_name[..]);
                    if self.page.is_some() {
                        let page = self.page.clone();
                        self.reset_page();
                        return page;
                    }
                }
                XmlEvent::Characters(chars) => {
                    self.content.push_str(&chars[..]);
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use std::io::BufReader;

    static BASIC_XML: &'static str = "<mediawiki>
<page>
<title>TITLE</title>
<revision>
<text>TEXT</text>
</revision>
</page>
</mediawiki>";

    #[test]
    fn basic_title_and_revision_with_text() {
        let reader = BufReader::new(BASIC_XML.as_bytes());
        let mut parser = Parser::new(reader);
        let page = parser.next();
        assert!(page.is_some());
        let page = page.unwrap();
        assert_eq!(page.title, "TITLE");
        assert_eq!(page.revisions.len(), 1);
        assert_eq!(page.revisions[0].text, "TEXT");
    }
}
