#[derive(Serialize, Deserialize)]
pub struct Game {
    pub sessionId: Option<String>,
    pub status: String,
    pub ts: u64
}
#[derive(Serialize, Deserialize)]
pub struct Status {
    pub status: String
}
