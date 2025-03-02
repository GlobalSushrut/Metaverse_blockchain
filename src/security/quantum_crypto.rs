use pqcrypto_ntru::*;
use pqcrypto_dilithium::*;
use pqcrypto_traits::sign::{PublicKey, SecretKey, DetachedSignature};
use crate::math::quantum_entropy::QuantumState;

pub struct QuantumCrypto {
    ntru_keypair: Option<(ntruhps2048509::PublicKey, ntruhps2048509::SecretKey)>,
    dilithium_keypair: Option<(dilithium2::PublicKey, dilithium2::SecretKey)>,
}

impl QuantumCrypto {
    pub fn new() -> Self {
        Self {
            ntru_keypair: None,
            dilithium_keypair: None,
        }
    }

    pub fn generate_ntru_keypair(&mut self) -> Result<(), &'static str> {
        match ntruhps2048509::keypair() {
            Ok((pk, sk)) => {
                self.ntru_keypair = Some((pk, sk));
                Ok(())
            }
            Err(_) => Err("Failed to generate NTRU keypair")
        }
    }

    pub fn generate_dilithium_keypair(&mut self) -> Result<(), &'static str> {
        match dilithium2::keypair() {
            Ok((pk, sk)) => {
                self.dilithium_keypair = Some((pk, sk));
                Ok(())
            }
            Err(_) => Err("Failed to generate Dilithium keypair")
        }
    }

    pub fn encrypt_quantum_state(&self, state: &QuantumState) -> Result<Vec<u8>, &'static str> {
        if let Some((pk, _)) = &self.ntru_keypair {
            // Serialize quantum state to bytes (simplified)
            let state_bytes = bincode::serialize(&state.probabilities)
                .map_err(|_| "Failed to serialize quantum state")?;
            
            // Encrypt with NTRU
            ntruhps2048509::encrypt(&state_bytes, pk)
                .map_err(|_| "NTRU encryption failed")
        } else {
            Err("No NTRU keypair available")
        }
    }

    pub fn decrypt_quantum_state(&self, ciphertext: &[u8]) -> Result<Vec<f64>, &'static str> {
        if let Some((_, sk)) = &self.ntru_keypair {
            // Decrypt with NTRU
            let decrypted = ntruhps2048509::decrypt(ciphertext, sk)
                .map_err(|_| "NTRU decryption failed")?;
            
            // Deserialize back to quantum state probabilities
            bincode::deserialize(&decrypted)
                .map_err(|_| "Failed to deserialize quantum state")
        } else {
            Err("No NTRU keypair available")
        }
    }

    pub fn sign_quantum_transaction(&self, transaction_data: &[u8]) -> Result<Vec<u8>, &'static str> {
        if let Some((_, sk)) = &self.dilithium_keypair {
            Ok(dilithium2::detached_sign(transaction_data, sk))
        } else {
            Err("No Dilithium keypair available")
        }
    }

    pub fn verify_quantum_transaction(
        &self,
        transaction_data: &[u8],
        signature: &[u8]
    ) -> Result<bool, &'static str> {
        if let Some((pk, _)) = &self.dilithium_keypair {
            match dilithium2::verify_detached_signature(signature, transaction_data, pk) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false)
            }
        } else {
            Err("No Dilithium keypair available")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_crypto_keypair_generation() {
        let mut crypto = QuantumCrypto::new();
        assert!(crypto.generate_ntru_keypair().is_ok());
        assert!(crypto.generate_dilithium_keypair().is_ok());
    }

    #[test]
    fn test_quantum_transaction_signing() {
        let mut crypto = QuantumCrypto::new();
        crypto.generate_dilithium_keypair().unwrap();
        
        let transaction = b"quantum transaction data";
        let signature = crypto.sign_quantum_transaction(transaction).unwrap();
        assert!(crypto.verify_quantum_transaction(transaction, &signature).unwrap());
    }
}
