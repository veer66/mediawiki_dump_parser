# mediawiki_dump_parser
A library for parsing XML dumped from Mediawiki, e.g., Wikipedia

## Working In Process!

## Examples

````
URL=http://download.wikimedia.org/enwiki/20190220/enwiki-20190220-pages-articles.xml.bz2
wget $URL -O - | bzcat | cargo run --release --example get_title
````

## Features
* Extracting pages with title and revisions with text
