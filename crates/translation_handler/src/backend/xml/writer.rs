use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Cursor, Write};
use crate::backend::xml::XmlReader;
use quick_xml::{Reader, Writer, events::{BytesText, BytesStart, BytesEnd, Event}};
use quick_xml::name::QName;


impl XmlReader {
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
        let first_element = format!("    <data name={} xml:space=\"preserve\">\n",key);
        let second_element = format!("    <value>{}</value>\n", value);
        let third_element = "  </data>\n".to_string();


        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::End(ref e) if e.name() == QName(b"root") => {
                    writer.write_event(Event::Text(BytesText::from_escaped(first_element.clone())))?;
                    writer.write_event(Event::Text(BytesText::from_escaped(second_element.clone())))?;
                    writer.write_event(Event::Text(BytesText::from_escaped(third_element.clone())))?;
                    writer.write_event(Event::End(e.to_owned()))?;
                },
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
