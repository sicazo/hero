use db::context::RouterCtx;
use db::prisma::location;
use db::prisma::location::Data;

pub async fn delete_location(ctx: RouterCtx, input: Data) -> Result<Data, rspc::Error> {
    let db = &ctx.db;
    let location = db
        .location()
        .delete(location::id::equals(input.id))
        .exec()
        .await?;
    Ok(location)
}
