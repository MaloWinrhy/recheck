pub mod hasher;
pub mod normalizer;
pub mod metadata;

use crate::hasher::hash_file;

pub fn compute_recheck_hash(path: &str) -> Result<String, String> {
    hash_file(path)
}
