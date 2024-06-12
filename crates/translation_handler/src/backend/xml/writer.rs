use crate::backend::xml::XmlHandler;
use quick_xml::name::QName;
use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Reader, Writer,
};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Cursor, Write};

impl XmlHandler {
    pub fn write_key_value(
        file_path: String,
        key: String,
        value: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(file_path.clone())?;
        let file_reader = BufReader::new(file);

        let mut reader = Reader::from_reader(file_reader);
        reader.trim_text(false);

        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let mut buf = Vec::new();
        let data_opening_tag = format!("    <data name={} xml:space=\"preserve\">\n", key);
        let value_tag = format!("    <value>{}</value>\n", value);
        let data_closing_tag = "  </data>\n".to_string();

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::End(ref e) if e.name() == QName(b"root") => {
                    writer.write_event(Event::Text(BytesText::from_escaped(
                        data_opening_tag.clone(),
                    )))?;
                    writer.write_event(Event::Text(BytesText::from_escaped(value_tag.clone())))?;
                    writer.write_event(Event::Text(BytesText::from_escaped(
                        data_closing_tag.clone(),
                    )))?;
                    writer.write_event(Event::End(e.to_owned()))?;
                }
                Event::Eof => {
                    break;
                }
                _ => {
                    writer.write_event(&event)?;
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
