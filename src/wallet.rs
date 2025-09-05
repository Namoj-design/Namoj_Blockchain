use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use hex;

pub fn generate_keypair_hex() -> (String, String) {
    let mut csprng = OsRng{};
    let kp: Keypair = Keypair::generate(&mut csprng);
    (hex::encode(kp.public.to_bytes()), hex::encode(kp.secret.to_bytes()))
}

pub fn sign_message(secret_hex: &str, msg: &[u8]) -> String {
    let secret_bytes = hex::decode(secret_hex).expect("decode secret");
    let secret = SecretKey::from_bytes(&secret_bytes).expect("secret from bytes");
    // reconstruct Keypair from secret (not recommended for production; keep secret safe)
    let public = PublicKey::from(&secret);
    let kp = Keypair { secret, public };
    let sig: Signature = kp.sign(msg);
    hex::encode(sig.to_bytes())
}

pub fn verify_signature(pub_hex: &str, msg: &[u8], sig_hex: &str) -> bool {
    let pubb = hex::decode(pub_hex).unwrap_or_default();
    let pubk = PublicKey::from_bytes(&pubb).ok();
    let sigb = hex::decode(sig_hex).unwrap_or_default();
    if let (Some(pk), Ok(sig)) = (pubk, ed25519_dalek::Signature::from_bytes(&sigb)) {
        return pk.verify(msg, &sig).is_ok();
    }
    false
}

