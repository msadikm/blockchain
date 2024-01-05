use std::collections::HashMap;
use std::time::SystemTime;

// For creating and managing blocks - a container that holds data (including transactions)
struct Block {
    block_count: usize,
    transactions: Vec<String>,
    last_hash: String,
    timestamp: f64,
    forger: String,    // Public key of forger
    signature: String, // signature of forger
}

impl Block {
    // Constructor method
    fn new(
        transactions: Vec<String>,
        last_hash: String,
        forger: String,
        block_count: usize,
    ) -> Block {
        Block {
            block_count,
            transactions,
            last_hash,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            forger,
            signature: String::new(),
        }
    }

    // Creates a new block (as a starting point)
    fn genesis() -> Block {
        let transaction: Vec::new(); // There are no transactions
        let last_hash = "genesishash".to_string();
        let forger = "genesis".to_string();
        let block_count = 0;

        let mut genesis_block = Block::new(transaction, last_hash, forger, block_count);
        genesis_block.timestamp = 0.0;

        genesis_block
    }

    // Will help display the block in readable dictionary form
    fn to_json(&self) -> HashMap<String, serde_json::Value> {
        let mut data: HashMap::new();

        data.insert(
            "block_count".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.block_count)),
        );
        data.insert(
            "last_hash".to_string(),
            serde_json::Value::String(self.last_hash.clone()),
        );
        data.insert(
            "signature".to_string(),
            serde_json::Value::String(self.signature.clone()),
        );
        data.insert(
            "forger".to_string(),
            serde_json::Value::String(self.forger.clone()),
        );
        data.insert(
            "timestamp".to_string(),
            serde_json::Value::Number(serde_json::Number::from_64(self.timestamp).unwrap()),
        );

        let json_transations: Vec<serde_json::Value> = self
            .transactions
            .iter()
            .map(|t| serde_json::Value::String(t.clone()))
            .collect();
        data.insert(
            "transactions"to_string(),
            serde_json::Value::Array(json_transations),
        );

        data
    }

    // Generates same dictionary as toJson method but without signature.
    fn payload(&self) -> HashMap<String, serde_json::Value> {
        let mut data = self.to_json();
        data.remove("signature");
        data
    }

    // Adds signature to block
    fn sign(&self, signature: &str) {
        self.signature = signature.to_string();
    }
}
