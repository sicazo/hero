use std::sync::Arc;

use chrono::prelude::*;
use db::prisma::location::{self, Data};
use db::{context::RouterCtx, prisma::PrismaClient};
use prisma_client_rust::QueryError::Deserialize;
use prisma_client_rust::{chrono, QueryError};
use rspc::{Router, RouterBuilder};
use translation_handler::TranslationHandler;

#[derive(serde::Deserialize, serde::Serialize, specta::Type)]
struct ScanResponse {
    keys: i32,
    untranslated_keys: i32,
}

#[derive(serde::Deserialize, serde::Serialize, specta::Type)]
struct ScanInput {
    path: String,
    name: String,
}
#[derive(serde::Deserialize, serde::Serialize, specta::Type)]
struct RescanInput {
    path: String,
    tag: String,
}

pub fn get_location_router() -> RouterBuilder<RouterCtx> {
    Router::<RouterCtx>::new()
        .mutation("add_location", |t| {
            t(|ctx, input: ScanInput| async move {
                let db = &ctx.db;
                let now = Local::now();

                if input.path.contains("messages.ts") {
                    let path = input.path.replace("/messages.ts", "");
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
                    println!("adding new FE location");
                    let mut location_path = input.path.replace("/messages.ts", "");
                    location_path = location_path.replace(r#"\messages.ts"#, "");

                    let location = location_database_upsert(
                        db,
                        input.name,
                        location_path,
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
                        let db = &ctx.db;
                        let now = Local::now();
                        let locations = db
                            .location()
                            .create_many(
                                resources_paths
                                    .iter()
                                    .map(|path| {
                                        location::create_unchecked(
                                            "BE".to_string(),
                                            path.clone(),
                                            path.clone(),
                                            2,
                                            2,
                                            now.to_string(),
                                            vec![],
                                        )
                                    })
                                    .collect(),
                            )
                            .skip_duplicates()
                            .exec()
                            .await?;

                        return Ok(locations);
                    };

                    Err(Vec::new())
                }
            })
        })
        .mutation("rescan_location", |t| {
            t(|ctx, input: RescanInput| async move {
                if input.tag == String::from("FE") {
                    let db = &ctx.db;

                    let keys =
                        TranslationHandler::get_key_values_from_messages_ts(&input.path).await;

                    let key_value = TranslationHandler::get_translations(&input.path).await;

                    let untranslated_keys = key_value
                        .into_iter()
                        .filter(|entry| {
                            entry
                                .translations
                                .iter()
                                .all(|(k, v)| k == "en-GB" || v.trim().is_empty())
                        })
                        .count();

                    let mut location_name = String::from("New");

                    let location = location_database_upsert(
                        db,
                        String::from("irrelevant"),
                        input.path,
                        input.tag,
                        keys.len() as i32,
                        untranslated_keys as i32,
                    )
                    .await?;
                    Ok(location)
                } else {
                    let db = &ctx.db;

                    let keys =
                        TranslationHandler::get_key_values_from_messages_ts(&input.path).await;

                    let key_value = TranslationHandler::get_translations(&input.path).await;

                    let untranslated_keys = key_value
                        .into_iter()
                        .filter(|entry| {
                            entry
                                .translations
                                .iter()
                                .all(|(k, v)| k == "en-GB" || v.trim().is_empty())
                        })
                        .count();
                    let location = location_database_upsert(
                        db,
                        String::from("new"),
                        input.path,
                        String::from("BE"),
                        keys.len() as i32,
                        untranslated_keys as i32,
                    )
                    .await?;

                    Ok(location)
                }
            })
        })
}

async fn location_database_upsert(
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
