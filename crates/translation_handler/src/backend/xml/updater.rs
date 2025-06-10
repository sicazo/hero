use quick_xml::name::QName;
use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Reader, Writer,
};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Cursor, Write};

use crate::backend::xml::XmlHandler;

impl XmlHandler {
    pub fn update_value(
        file_path: String,
        key: String,
        value: String,
    ) -> Result<(), quick_xml::Error> {
        let file = File::open(&file_path)?;
        let file_reader = BufReader::new(file);

        let mut reader = Reader::from_reader(file_reader);
        reader.trim_text(false);

        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let mut buf = Vec::new();

        let mut inside_searched_tag = false;
        let mut tag_depth = 0;
        let mut value_replaced = false;

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name() == QName(b"data") => {
                    let mut is_target_tag = false;
                    for attr in e.attributes().filter_map(|a| a.ok()) {
                        if attr.key == QName(b"name") && attr.value.as_ref() == key.as_bytes() {
                            is_target_tag = true;
                            inside_searched_tag = true;
                            tag_depth = 1;
                            break;
                        }
                    }
                    writer.write_event(Event::Start(e))?;
                    if !is_target_tag {
                        tag_depth = 0;
                        inside_searched_tag = false;
                    }
                }
                Event::End(e) if e.name() == QName(b"data") => {
                    tag_depth -= 1;
                    if inside_searched_tag && tag_depth == 0 {
                        if !value_replaced {
                            // Write new <value> tag
                            writer.write_event(Event::Start(BytesStart::new("value")))?;
                            writer.write_event(Event::Text(BytesText::new(&value)))?;
                            writer.write_event(Event::End(BytesEnd::new("value")))?;
                            value_replaced = true;
                        }
                        inside_searched_tag = false;
                    }
                    writer.write_event(Event::End(e))?;
                }
                Event::Start(e) if inside_searched_tag && e.name() == QName(b"value") => {
                    if !value_replaced {
                        // Write new <value> tag instead of the original
                        writer.write_event(Event::Start(BytesStart::new("value")))?;
                        writer.write_event(Event::Text(BytesText::new(&value)))?;
                        writer.write_event(Event::End(BytesEnd::new("value")))?;
                        value_replaced = true;
                    }
                    // Skip the original <value> tag and its content
                    let mut value_depth = 1;
                    while value_depth > 0 {
                        match reader.read_event_into(&mut buf)? {
                            Event::Start(_) => value_depth += 1,
                            Event::End(e) if e.name() == QName(b"value") => value_depth -= 1,
                            _ => {}
                        }
                    }
                }
                Event::Text(_) | Event::CData(_) if inside_searched_tag && !value_replaced => {
                    // Skip stray text or CDATA inside <data> before <value>
                }
                Event::Eof => break,
                Event::Start(e) => {
                    if inside_searched_tag {
                        tag_depth += 1;
                    }
                    writer.write_event(Event::Start(e))?;
                }
                Event::End(e) => {
                    if inside_searched_tag {
                        tag_depth -= 1;
                    }
                    writer.write_event(Event::End(e))?;
                }
                event => {
                    if !inside_searched_tag || (inside_searched_tag && value_replaced) {
                        writer.write_event(event)?;
                    }
                }
            }
            buf.clear();
        }

        let result = writer.into_inner().into_inner();
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?;
        let mut file_writer = BufWriter::new(file);
        file_writer.write_all(&result)?;
        file_writer.flush()?;

        Ok(())
    }
}
