use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Player {
    name: String,
    points: u32,
}

#[derive(Debug, Deserialize)]
struct TransferRequest {
    from: String,
    to: String,
    amount: u32,
}
type Db = Mutex<HashMap<String, Player>>;