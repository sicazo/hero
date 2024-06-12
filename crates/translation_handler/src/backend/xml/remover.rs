use crate::backend::xml::XmlHandler;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::{Reader, Writer};
use std::fs::{File, OpenOptions};

use std::io::{BufReader, BufWriter, Cursor, Write};

impl XmlHandler {
    pub fn remove_key_value(
        file_path: String,
        keys: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(file_path.clone())?;
        let file_reader = BufReader::new(file);

        let mut reader = Reader::from_reader(file_reader);
        reader.trim_text(false);

        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let mut buf = Vec::new();

        let mut inside_searched_tag = false;
        let mut tag_depth = 0;

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) if e.name() == QName(b"data") => {
                    let mut is_target_tag = false;
                    for attr in e.attributes().filter_map(|a| a.ok()) {
                        if attr.key == QName(b"name") {
                            if let Ok(attr_value) = std::str::from_utf8(&attr.value) {
                                if keys.contains(&attr_value.to_string()) {
                                    is_target_tag = true;
                                    inside_searched_tag = true;
                                    tag_depth = 1; // Start tracking depth
                                    break;
                                }
                            }
                        }
                    }
                    if !is_target_tag {
                        writer.write_event(&event)?;
                    }
                }
                Event::End(ref e) if e.name() == QName(b"data") => {
                    if inside_searched_tag {
                        tag_depth -= 1;
                        if tag_depth == 0 {
                            inside_searched_tag = false;
                        }
                    } else {
                        writer.write_event(&event)?;
                    }
                }
                Event::Eof => {
                    break;
                }
                _ => {
                    if inside_searched_tag {
                        if let Event::Start(_) = event {
                            tag_depth += 1;
                        } else if let Event::End(_) = event {
                            tag_depth -= 1;
                        }
                    } else {
                        writer.write_event(&event)?;
                    }
                }
            }
            buf.clear();
        }

        let result = writer.into_inner().into_inner();
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path.clone())?;

        let mut file_writer = BufWriter::new(file);
        file_writer.write_all(&*result)?;

        Ok(())
    }
}
