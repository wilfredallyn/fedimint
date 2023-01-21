use std::sync::Arc;

use fedimint_core::modules::proof::config::ProofClientConfig;
use thiserror::Error;

use crate::utils::ClientContext;
use crate::ApiError;

/// Federation module client for the Proof of reserves module. It can create a signed PSBT spending all
/// wallet outputs that is invalid to broadcast but proves that the funds are spendable.
#[derive(Debug)]
pub struct ProofClient {
    pub config: ProofClientConfig,
    pub context: Arc<ClientContext>,
}

impl ProofClient {
    pub async fn get_proof_of_reserves(&self) -> Result<String> {
        let proof: String = self.context.api.proof_of_reserves().await?;
        Ok(proof)
    }
}

type Result<T> = std::result::Result<T, ProofClientError>;

#[derive(Error, Debug)]
pub enum ProofClientError {
    #[error("Could not produce proof of reserves")]
    ProofError,
    #[error("Proof API error: {0}")]
    ApiError(#[from] ApiError),
}
