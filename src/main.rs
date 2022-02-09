extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use std::error::Error;
use std::rc::Rc;
use html5ever::{parse_document, serialize};
use html5ever::tendril::{TendrilSink};
use rcdom::{RcDom, SerializableHandle};

fn main() -> Result<(), Box<dyn Error>> {
    let input = "
<!DOCTYPE html>
<html>
    <head>
        <title>Template Element Test</title>
        <meta charset=\"utf8\">
    </head>
    <body>
        <template>
            <div>Hello, World!</div>
        </template>
    </body>
</html>
    ".trim();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut input.as_bytes())?;
    
    let mut bytes = vec![];
    let serializable_document: SerializableHandle = Rc::clone(&dom.document).into();
    serialize(&mut bytes, &serializable_document, Default::default())?;
    println!("{}", String::from_utf8(bytes)?);

    Ok(())
}
