use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{ Signature, Verifier, PublicKey}; // Added PublicKey import // Use StdRng instead of ThreadRng

// // Generate keypair and return (private_key_base64, public_key_base64)
// pub fn generate_keypair() -> (String, String) {
//     let mut rng = StdRng::from_entropy(); // Use StdRng, which implements CryptoRng + RngCore

//     let keypair: Keypair = Keypair::generate(&mut rng); // Generate keypair

//     let private_key_b64 = STANDARD.encode(&keypair.to_bytes()); // Encode private key to Base64
//     let public_key_b64 = STANDARD.encode(&keypair.public.to_bytes()); // Encode public key to Base64

//     (private_key_b64, public_key_b64)
// }

// // Sign a message with the private key
// pub fn sign_message(private_key_b64: &Keypair, message: &[u8]) -> String {
//     let signature: Signature = private_key_b64.sign(message); // Sign the message
//     STANDARD.encode(signature.to_bytes()) // Return Base64-encoded signature
// }

// Verify a signature using the public key
pub fn verify_signature(public_key_b64: &str, message: &[u8], signature_b64: &str) -> bool {
    let public_key_bytes = STANDARD.decode(public_key_b64).unwrap(); // Decode Base64 public key
    let signature_bytes = STANDARD.decode(signature_b64).unwrap(); // Decode Base64 signature

    let public_key = PublicKey::from_bytes(&public_key_bytes).unwrap(); // Parse the public key
    let signature = Signature::from_bytes(&signature_bytes).unwrap(); // Parse the signature

    public_key.verify(message, &signature).is_ok() // Return verification result
}

// // Load a keypair from a Base64-encoded private key
// pub fn load_keypair_base64(private_key_b64: &str) -> Keypair {
//     let key_bytes = STANDARD.decode(private_key_b64).unwrap(); // Decode Base64 private key
//     Keypair::from_bytes(&key_bytes).unwrap() // Parse and return the keypair
// }
