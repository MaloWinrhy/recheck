use std::fs::File;
use std::io::{BufReader, Read};
use blake3;
use normalize_line_endings::normalized;

pub fn hash_file(path: &str) -> Result<String, String> {
    let file = File::open(path).map_err(|e| format!("DoesNotExist : {}", e))?;
    let mut reader = BufReader::new(file);

    let mut raw_content = String::new();
    reader
        .read_to_string(&mut raw_content)
        .map_err(|e| format!("ErrorReadUTF8 : {}", e))?;

    let normalized = normalized(&raw_content);
    
    let hash = blake3::hash(normalized.as_bytes());

    Ok(hash.to_hex().to_string())
}
