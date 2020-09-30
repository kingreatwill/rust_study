

// anyhow
// thiserror
// failure - 被日渐废弃
// https://rustcc.cn/article?id=6dcbf032-0483-4980-8bfe-c64a7dfb33c7
/*
Use Result<T, anyhow::Error>, or equivalently anyhow::Result<T>, as the return type of any fallible function.

Within the function, use ? to easily propagate any error that implements the std::error::Error trait.

use anyhow::Result;

fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}
*/
use thiserror::Error;
use std::io::Read;
use std::fs::File;
use std::io;

pub type Result<T> = std::result::Result<T, DataStoreError>;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
    #[error(transparent)]
    Other(#[from] anyhow::Error),  // source and Display delegate to anyhow::Error
}

fn main() {
    println!("{:?}",read_text_from_file("xx.json").err());
    read_text_from_file("xx.json").err();
}

fn read_text_from_file(path: &str) -> Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}