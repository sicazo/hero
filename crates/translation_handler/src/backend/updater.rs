use crate::backend::xml::XmlHandler;
use crate::frontend::updater::UpdatedKeyValues;
use crate::TranslationHandler;
use db::prisma::settings;
use std::error::Error;

impl TranslationHandler {
    pub async fn update_backend_key(
        path: String,
        values: UpdatedKeyValues,
        _settings: settings::Data,
    ) -> Result<(), Box<dyn Error>> {
        XmlHandler::update_value(path, values.ts_key, values.json_key)?;
        Ok(())
    }
}
