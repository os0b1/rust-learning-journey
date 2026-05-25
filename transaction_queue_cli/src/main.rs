mod queue;
mod types;
use queue::mempool;
use types::{prority, status, transaction};  
fn main() {
    let mut mempool = mempool::new();
    let tx1 = transaction {
        id: 1,
        sender: "Alice".to_string(),
        receiver: "Bob".to_string(),
        amount: 100,
        priority: prority::High,
        status: status::Pending,
    };
    let tx2 = transaction {
        id: 2,
        sender: "Charlie".to_string(),
        receiver: "Dave".to_string(),
        amount: 50,
        priority: prority::Medium,
        status: status::Pending,
    };
    mempool.add_tx(tx1);
    mempool.add_tx(tx2);
    mempool.show_queue();
    mempool.execute_tx();
    mempool.drop_tx(2);
}
