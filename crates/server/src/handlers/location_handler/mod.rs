mod add;
mod delete;
mod rescan;
mod types;
mod update;

use self::types::ScanInput;
use crate::handlers::location_handler::delete::delete_location;
use crate::handlers::location_handler::types::RescanInput;
use add::add_location;
use db::context::RouterCtx;
use rescan::rescan_location;
use rspc::{Router, RouterBuilder};
use local_storage::stores::location_store::Location;

pub fn get_location_router() -> RouterBuilder<RouterCtx> {
    Router::<RouterCtx>::new()
        .mutation("add_location", |t| {
            t(|ctx, input: ScanInput| add_location(ctx, input))
        })
        .mutation("rescan_location", |t| {
            t(|ctx, input: RescanInput| rescan_location(ctx, input))
        })
        .mutation("delete_location", |t| {
            t(|ctx, input: Location| delete_location(ctx, input))
        })
}
