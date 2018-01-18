
use errors::*;

use std::iter::Peekable;
use std::io::Read;
use xml::reader::Events;
use xml::reader::XmlEvent;

/// consume consumes an element as a nothing.
pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<bool> {
    let mut element: Option<String> = None;
    for event in reader {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement { name, .. } => {
                ensure!(element.is_none(), "cannot start element inside bounds");

                element = Some(name.local_name);
            }

            XmlEvent::EndElement { .. } => {
                return Ok(true);
            }

            _ => {}
        }
    }

    return Err("no end tag for bounds".into());
}
