use quick_xml::{events::Event, name::QName, reader::Reader};

pub fn test() {
    let xml = r#"
    <data name="Label_Building_Location" xml:space="preserve">
    <value>Building Location</value>
  </data>
   <data name="Label_BackgroundColor" xml:space="preserve">
    <value>Background Color</value>
  </data>
  <data name="Label_Building_Area" xml:space="preserve">
    <value>Heatbale Living Area</value>
  </data>
  <data name="Label_Building_Floor" xml:space="preserve">
    <value>Floor</value>
  </data>
  <data name="Label_Building_Height" xml:space="preserve">
    <value>Building Height</value>
  </data>
  <data name="Label_Building_Location" xml:space="preserve">
    <value>Building Location</value>
  </data>
  <data name="Label_Building_PostalCode" xml:space="preserve">
    <value>Postal Code</value>
  </data>
  <data name="Label_Building_PostalCode_Help" xml:space="preserve">
    <value>Indicate where the building is located for which you want to receive an offer.</value>
  </data>
  <data name="Label_Building_Refurbishments" xml:space="preserve">
    <value>Additional Refurbishments</value>
  </data>
  <data name="Label_Building_Residents" xml:space="preserve">
    <value>Number of Residents</value>
  </data>
    "#;
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut inside_data = false;
    let mut inside_name = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == QName(b"data") => {
                for attr in e.attributes().filter_map(|a| a.ok()) {
                    if attr.key == QName(b"name") {
                        let name_value = attr.decode_and_unescape_value(&reader).unwrap();
                        println!("Name attribute: {}", name_value);
                        inside_data = true;
                    }
                }
            }
            Ok(Event::Start(ref e)) if e.name() == QName(b"value") && inside_data => {
                inside_name = true;
            }
            Ok(Event::End(ref e)) if e.name() == QName(b"value") && inside_data => {
                inside_name = false;
            }
            Ok(Event::End(ref e)) if e.name() == QName(b"data") => {
                inside_data = false;
            }
            Ok(Event::Text(e)) if inside_name => {
                let value_content = e.unescape().unwrap();
                println!("Value tag content: {}", value_content);
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}
