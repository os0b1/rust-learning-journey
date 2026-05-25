use crate:: types::{prority,status,transaction};
pub struct mempool {
    pub transactions: Vec<transaction>,

}
impl mempool {
    pub fn new() -> Self {
        mempool {
            transactions: Vec::new(),
        }
    }

    pub fn add_tx (&mut self, tx: transaction) {
        self.transactions.push(tx);
    }
    pub fn show_queue(&self) {
println!("\n=== MEMPOOL ===");        }
    }
    pub fn execute_tx(&mut self) {
    if self.transactions.is_empty() {
        println!("No transactions to execute.");
        return;
    }
    let mut highest_priority_index = 0;
    for (i, tx) in self.transactions.iter().enumerate() {
     match tx.priority {
        prority::High => {
            highest_priority_index = i;
            break;
        } 
        priority::Medium => {
            if matches!(self.transactions[highest_priority_index].priority, prority::Low) {
                highest_priority_index = i;
            }
        }
        prority::Low => {
            if matches!(self.transactions[highest_priority_index].priority, prority::Low) {
                highest_priority_index = i;
            }
        }
        priority::None => {}
        }
    }
    let mut tx = self.transactions.reemove(highest_priority_index);
    tx.status = status::Executed;
    println!("Executed transaction: {:?}", tx);
    println!("{:?}", tx);
}
pub fn drop_tx(&mut self, tx_id: u32) {
   self.transactions.retain(|tx| tx.id != tx_id);
   println!("Dropped transaction with ID: {}", tx_id);
}