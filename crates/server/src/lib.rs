use db::context::RouterCtx;
use handlers::{storage_handler::get_storage_router, translation_handler::get_translation_router, location_handler::get_location_router};
use rspc::{Config, Router as RspcRouter};
use tokio::time::{sleep, Duration};

mod handlers;

pub fn get_router() -> RspcRouter<RouterCtx> {
    RspcRouter::<RouterCtx>::new()
        .config(Config::new().export_ts_bindings("../../src/lib/procedures.ts"))
        .query("hi", |t| t(|_ctx, _input: ()| "hello world"))
        .merge("stores.", get_storage_router())
        .merge("translations.", get_translation_router())
        .merge("locations.", get_location_router())
        .subscription("test", |t| {
            t(|ctx, input: ()| {
                async_stream::stream! {
                    for i in 0..5 {
                        yield "ping".to_string();
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            })
        })
        .build()
}
