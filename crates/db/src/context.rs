use crate::prisma::PrismaClient;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct RouterCtx {
    pub db: Arc<PrismaClient>,
}
