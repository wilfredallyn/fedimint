use fedimint_api::config::{
    ClientModuleConfig, TypedClientModuleConfig, TypedServerModuleConfig,
    TypedServerModuleConsensusConfig,
};
use fedimint_api::module::__reexports::serde_json;
use fedimint_api::PeerId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProofConfig {
    /// Contains all configuration that will be encrypted such as private key material
    pub private: ProofConfigPrivate,
    /// Contains all configuration that needs to be the same for every federation member
    pub consensus: ProofConfigConsensus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProofConfigConsensus {
    pub something: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProofConfigPrivate {
    pub something_private: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProofClientConfig {
    pub something: u64,
}

impl TypedClientModuleConfig for ProofClientConfig {}

impl TypedServerModuleConsensusConfig for ProofConfigConsensus {
    fn to_client_config(&self) -> ClientModuleConfig {
        serde_json::to_value(&ProofClientConfig {
            something: self.something,
        })
        .expect("Serialization can't fail")
        .into()
    }
}

impl TypedServerModuleConfig for ProofConfig {
    type Local = ();
    type Private = ProofConfigPrivate;
    type Consensus = ProofConfigConsensus;

    fn from_parts(_local: Self::Local, private: Self::Private, consensus: Self::Consensus) -> Self {
        Self { private, consensus }
    }

    fn to_parts(self) -> (Self::Local, Self::Private, Self::Consensus) {
        ((), self.private, self.consensus)
    }

    fn validate_config(&self, _identity: &PeerId) -> anyhow::Result<()> {
        Ok(())
    }
}
