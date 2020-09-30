use eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Comment {
    pub id: i32,
    pub author: Author,
    pub body: String,
    pub in_reply_to: i32,
}
// cargo run --example http
#[tokio::main]
async fn main() -> Result<()> {
    let c: Comment = reqwest::get("https://xena.greedo.xeserv.us/files/comment.json")
        .await?
        .json()
        .await?;
    println!("comment: {:#?}", c);

    Ok(())
}
