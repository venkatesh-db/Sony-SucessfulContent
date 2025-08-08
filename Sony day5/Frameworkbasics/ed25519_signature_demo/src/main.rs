
use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use rand::rngs::OsRng;

fn main() {
    // Generate a keypair
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    // The message to be signed
    let message: &[u8] = b"Hello from Venkatesh Blockchain!";

    // Sign the message
    let signature: Signature = keypair.sign(message);

    // Verify the signature
    let verified = keypair.public.verify(message, &signature).is_ok();

    println!("🔐 Message: {:?}", std::str::from_utf8(message).unwrap());
    println!("🖊️ Signature: {:?}", signature.to_bytes());
    println!("✅ Verification: {}", verified);
}
