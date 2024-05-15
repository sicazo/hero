use crate::backend::xml::XmlHandler;
use crate::TranslationHandler;
use std::error::Error;

impl TranslationHandler {
    pub async fn remove_backend_key(path: String, keys: Vec<String>) -> Result<(), Box<dyn Error>> {
        XmlHandler::remove_key_value(path, keys)
    }
}
