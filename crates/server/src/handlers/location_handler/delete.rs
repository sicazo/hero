use db::context::RouterCtx;
use db::prisma::location;
use db::prisma::location::Data;
use local_storage::stores::location_store::Location;

pub async fn delete_location(ctx: RouterCtx, input: Location) -> Result<Data, rspc::Error> {
    let db = &ctx.db;
    let location = db
        .location()
        .delete(location::path::equals(input.path))
        .exec()
        .await?;
    Ok(location)
}
