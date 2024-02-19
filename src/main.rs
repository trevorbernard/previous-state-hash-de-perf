use serde_derive::{Deserialize, Serialize};
use std::{path::{Path, PathBuf}, time::Instant};

use glob::glob;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PreviousStateHashBlock {
    pub protocol_state: ProtocolState,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProtocolState {
    pub previous_state_hash: String,
}

#[derive(PartialEq, Eq)]
pub struct PreviousStateHash(pub String);


fn deserialize_original(path: &Path) -> anyhow::Result<String> {
    let bytes = &std::fs::read(path)?;
    let PreviousStateHashBlock {
        protocol_state: ProtocolState {
            previous_state_hash,
        },
    } = serde_json::from_slice(bytes)?;
    Ok(previous_state_hash)
}

fn main() -> anyhow::Result<()> {
    let paths: Vec<PathBuf> = glob("/Users/tbernard/blocks/1000-blocks/*.json")?
        .filter_map(|x| x.ok())
        .collect();

    println!("Precomputed blocks to deserialize: {}", paths.len());
    let now = Instant::now();
    for path in paths {
        let _ = deserialize_original(&path)?;
    }
    println!("Elapsed time: {:?}", now.elapsed());
    Ok(())
}
