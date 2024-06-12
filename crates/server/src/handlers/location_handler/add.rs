use crate::handlers::location_handler::types::ScanInput;
use chrono::Local;
use db::context::RouterCtx;
use db::prisma::location::Data;
use db::prisma::{location, PrismaClient};
use prisma_client_rust::QueryError;
use std::path::Path;
use translation_handler::TranslationHandler;

fn sanitize_path(input_path: &str) -> String {
    input_path
        .replace("/messages.ts", "")
        .replace("\\messages.ts", "")
}

pub async fn add_location(ctx: RouterCtx, input: ScanInput) -> Result<Vec<Data>, rspc::Error> {
    let db = &ctx.db;
    if input.path.contains("messages.ts") {
        let sanitized_path = sanitize_path(&input.path);
        let keys = TranslationHandler::get_key_values_from_messages_ts(&sanitized_path).await;
        let key_value = TranslationHandler::get_frontend_translations(&sanitized_path).await;
        let untranslated_keys = key_value
            .into_iter()
            .filter(|entry| {
                entry
                    .translations
                    .iter()
                    .all(|(k, v)| k == "en-GB" || v.trim().is_empty())
            })
            .count();
        println!("adding new FE location");
        let location = location_database_upsert(
            db,
            input.name,
            sanitized_path,
            "FE".to_string(),
            keys.len() as i32,
            untranslated_keys as i32,
        )
        .await
        .expect("failed to upsert FE location");
        let mut response: Vec<Data> = Vec::new();
        response.push(location);
        return Ok(response);
    } else {
        if let Some(resources_paths) =
            translation_handler::backend::getter::get_resources_from_csproj(
                input.path.clone().as_str(),
            )
        {
            println!("{:?}", resources_paths);
            let now = Local::now();
            let _loc_count = db
                .location()
                .create_many(
                    resources_paths
                        .iter()
                        .map(|path| {
                            let loc_translations = translation_handler::backend::getter::get_translations_from_location(path.clone().as_str());
                            let full_path = Path::new(path);
                            let parent = full_path.parent().unwrap().file_name().unwrap().to_string_lossy();
                            location::create_unchecked(
                                "BE".to_string(),
                                parent.to_string(),
                                path.clone(),
                                loc_translations.len() as i32,
                                2,
                                now.to_string(),
                                vec![],
                            )
                        })
                        .collect(),
                )
                .skip_duplicates()
                .exec()
                .await;
            let locations: Vec<Data> = Vec::new();
            return Ok(locations);
        };
        Err(rspc::Error::new(
            rspc::ErrorCode::InternalServerError,
            "error".to_string(),
        ))
    }
}

pub async fn location_database_upsert(
    db: &PrismaClient,
    name: String,
    path: String,
    tag: String,
    keys: i32,
    untranslated_keys: i32,
) -> Result<Data, QueryError> {
    let now = Local::now();

    db.location()
        .upsert(
            location::path::equals(path.clone()),
            location::create(
                tag,
                name,
                path.clone(),
                keys,
                untranslated_keys,
                now.to_string(),
                vec![],
            ),
            vec![
                location::num_of_keys::set(keys),
                location::num_of_untranslated_keys::set(untranslated_keys),
            ],
        )
        .exec()
        .await
}
