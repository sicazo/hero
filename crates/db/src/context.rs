use crate::prisma::PrismaClient;
use std::sync::{Arc, Mutex};
use crate::prisma::location::Data;

#[derive(Clone)]
pub struct RouterCtx {
    pub db: Arc<PrismaClient>,
    pub location: Option<Data>
}
