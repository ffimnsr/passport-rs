use bcrypt::{hash, DEFAULT_COST};
use std::env;

use crate::errors::ServiceError;

pub fn hash_password(plain: &str) -> Result<String, ServiceError> {
    let hashing_cost: u32 = match env::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };

    hash(plain, hashing_cost).map_err(|_| ServiceError::InternalServerError)
}
