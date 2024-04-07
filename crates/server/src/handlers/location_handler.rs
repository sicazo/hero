use db::context::RouterCtx;
use prisma_client_rust::QueryError::Deserialize;
use rspc::{Router, RouterBuilder};
use translation_handler::TranslationHandler;

#[derive(serde::Deserialize, serde::Serialize, specta::Type)]
struct ScanResponse {
    keys: i32,
    untranslated_keys: i32,
}

pub fn get_location_router() -> RouterBuilder<RouterCtx> {
    Router::<RouterCtx>::new().mutation("add_location", |t| {
        t(|ctx, path: String| async move {
            let path = path.replace("/messages.ts", "");
            let path = path.replace("\\messages.ts", "");

            let keys = TranslationHandler::get_key_values_from_messages_ts(&path).await;

            let key_value = TranslationHandler::get_translations(&path).await;

            let untranslated_keys = key_value
                .into_iter()
                .filter(|entry| {
                    entry
                        .translations
                        .iter()
                        .all(|(k, v)| k == "en-GB" || v.trim().is_empty())
                })
                .count();
            ScanResponse {
                keys: keys.len() as i32,
                untranslated_keys: untranslated_keys as i32,
            }
        })
    })
}
