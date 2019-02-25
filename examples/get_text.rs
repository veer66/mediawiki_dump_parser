extern crate mediawiki_dump_parser;

use mediawiki_dump_parser::Parser;
use std::io;

fn main() {
    let stdin = io::stdin();
    let parser = Parser::new(stdin);
    for e in parser {
        for revision in e.revisions {
            println!("{}", revision.text);
        }
    }
}
