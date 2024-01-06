extern crate base64;
extern crate serde;
extern crate serde_json;
extern crate serde_pickle;
extern crate sha2;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_pickle::to_vec;
use sha2::Sha256;

pub struct BlockchainUtils {}

impl BlockchainUtils {
    // Creates a hash of data using the SHA256 hash algorithm
    pub fn hash(data: &Value) -> String {
        // Serialize data to JSON formatted string
        let data_str = serde_json::to_string(data).expect("Failed to serialize data to json");

        // Hash the serialized data using SHA-256
        let mut hasher = Sha256::new();
        hasher.update(data_str.as_bytes());
        let hash_result = hasher.finalize();

        // Convert hash result to hexadecimal string
        let hash_hex = hash_result
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();

        hash_hex
    }

    // Encode message object into a format that is allowed to send accross network
    pub fn encode<T: Serialize>(object_to_encode: T) -> String {
        let json_string =
            serde_json::to_string(object_to_encode).expect("Failed to serialize to json");
        let json_bytes = json_string.to_bytes();
        let base64_encoded = base64::encode(json_bytes);

        base64_encoded
    }

    // Recreate Object
    pub fn decode(encoded_object: &str) -> Vec<u8> {
        // Decode base64 encoded string
        base64::decode(encoded_object).expect("Failed to decode base64")
    }
}
