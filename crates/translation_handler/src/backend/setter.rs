use db::prisma::settings;
use local_storage::stores::translation_store::TranslationEntry;
use crate::TranslationHandler;

impl TranslationHandler {
    pub async fn add_new_backend_key(path: String, key: String, value: String, settings: settings::Data) -> Result<Vec<TranslationEntry>, std::io::Error>{
        Ok(vec![])
    }
}