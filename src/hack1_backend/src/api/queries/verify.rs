use sha2::{Sha256, Digest};

pub fn hash_string_sha256(input: &str) -> String {
    // Create a new Sha256 hasher instance
    let mut hasher = Sha256::new();

    // Feed the input string into the hasher
    hasher.update(input.as_bytes());

    // Retrieve the resulting hash as a byte array
    let result = hasher.finalize();

    // Convert the byte array to a hexadecimal string
    hex::encode(result)
}


// use sha2::{Sha256, Digest};
// use wasm_crypto::ecdsa::{PublicKey, Signature};

// // Function to hash a string using SHA-256
// fn hash_string(data: &str) -> Vec<u8> {
//     let mut hasher = Sha256::new();
//     hasher.update(data.as_bytes());
//     hasher.finalize().to_vec()
// }

// // Function to verify a signature using ECC (wasm-crypto)
// fn verify_signature(public_key: &[u8], data: &str, signature: &[u8]) -> bool {
//     let public_key = PublicKey::from_sec1_bytes(public_key).expect("Invalid public key");
//     let signature = Signature::from_bytes(signature).expect("Invalid signature");

//     // Hash the data using SHA-256
//     let hashed_data = hash_string(data);

//     // Verify the signature
//     public_key.verify(&hashed_data, &signature).is_ok()
// }