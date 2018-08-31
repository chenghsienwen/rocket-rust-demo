#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub name: String,
    pub score: u32,
    pub ts: u64
}
#[derive(Serialize, Deserialize)]
pub struct UserName {
    pub name: String
}
#[derive(Serialize, Deserialize)]
pub struct Score {
    pub score: u32
}
