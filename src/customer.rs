#[derive(Serialize, Deserialize)]
pub struct Customer {
    pub id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub nationality: String
}