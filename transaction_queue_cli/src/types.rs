# [derive(Debug)]
pub enum priority {
    high,
    medium,
    low,
}
# [derive(Debug)]
pub enum status {
    Pending,
    Executed,
    Dropped,
}
# [derive(Debug)]
pub struct transaction {
    pub id: u64,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub priority: priority,
    pub status: status,
}