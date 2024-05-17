use crate::handlers::location_handler::add::location_database_upsert;
use crate::handlers::location_handler::types::RescanInput;
use db::context::RouterCtx;
use db::prisma::location::Data;
use translation_handler::TranslationHandler;

pub async fn rescan_location(ctx: RouterCtx, input: RescanInput) -> Result<Data, rspc::Error> {
    //TODO: make this work for backend
    let db = &ctx.db;
    let keys = TranslationHandler::get_key_values_from_messages_ts(&input.path).await;
    let key_value = TranslationHandler::get_frontend_translations(&input.path).await;

    let untranslated_keys = key_value
        .into_iter()
        .filter(|entry| {
            entry
                .translations
                .iter()
                .all(|(k, v)| k == "en-GB" || v.trim().is_empty())
        })
        .count();

    let (irrelevant_or_new, fe_or_be) = if input.tag == String::from("FE") {
        (String::from("irrelevant"), input.tag)
    } else {
        (String::from("new"), String::from("BE"))
    };

    let location = location_database_upsert(
        db,
        irrelevant_or_new,
        input.path,
        fe_or_be,
        keys.len() as i32,
        untranslated_keys as i32,
    )
        .await?;

    Ok(location)
}
