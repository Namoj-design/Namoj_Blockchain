use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;

pub struct Wallet {
    pub keypair: Keypair,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let keypair = Keypair::generate(&mut rng);
        Wallet { keypair }
    }

    pub fn public_key(&self) -> PublicKey {
        self.keypair.public
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }

    pub fn verify(public_key: &PublicKey, message: &[u8], sig: &Signature) -> bool {
        public_key.verify(message, sig).is_ok()
    }
}
