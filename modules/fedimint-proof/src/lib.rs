use std::collections::{BTreeMap, HashSet};
use std::fmt;

use async_trait::async_trait;
use common::ProofModuleDecoder;
use fedimint_api::cancellable::Cancellable;
use fedimint_api::config::TypedServerModuleConsensusConfig;
use fedimint_api::config::{
    ClientModuleConfig, ConfigGenParams, DkgPeerMsg, ModuleConfigGenParams, ServerModuleConfig,
    TypedServerModuleConfig,
};
use fedimint_api::core::{Decoder, ModuleKey, MODULE_KEY_PROOF};
use fedimint_api::db::{Database, DatabaseTransaction};
use fedimint_api::encoding::{Decodable, Encodable};
use fedimint_api::module::__reexports::serde_json;
use fedimint_api::module::audit::Audit;
use fedimint_api::module::interconnect::ModuleInterconect;
use fedimint_api::module::{
    api_endpoint, ApiEndpoint, InputMeta, ModuleError, ModuleInit, TransactionItemAmount,
};
use fedimint_api::net::peers::MuxPeerConnections;
use fedimint_api::server::ServerModule;
use fedimint_api::task::TaskGroup;
use fedimint_api::{plugin_types_trait_impl, OutPoint, PeerId, ServerModulePlugin};
use fedimint_wallet::db::{UTXOKey, UTXOPrefixKey};
use fedimint_wallet::SpendableUTXO;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;

use crate::config::{ProofConfig, ProofConfigConsensus, ProofConfigPrivate};

pub mod common;
pub mod config;
pub mod db;

#[derive(Debug)]
pub struct Proof {
    pub cfg: ProofConfig,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Encodable, Decodable)]
pub struct ProofOutputConfirmation;

#[derive(Debug, Clone)]
pub struct ProofVerificationCache;

pub struct ProofConfigGenerator;

#[async_trait]
impl ModuleInit for ProofConfigGenerator {
    async fn init(
        &self,
        cfg: ServerModuleConfig,
        _db: Database,
        _task_group: &mut TaskGroup,
    ) -> anyhow::Result<ServerModule> {
        Ok(Proof::new(cfg.to_typed()?).into())
    }

    fn decoder(&self) -> (ModuleKey, Decoder) {
        (MODULE_KEY_PROOF, (&ProofModuleDecoder).into())
    }

    fn trusted_dealer_gen(
        &self,
        peers: &[PeerId],
        params: &ConfigGenParams,
    ) -> BTreeMap<PeerId, ServerModuleConfig> {
        let _params = params
            .get::<ProofConfigGenParams>()
            .expect("Invalid proof params");

        let proof_cfg: BTreeMap<_, ProofConfig> = peers
            .iter()
            .map(|&peer| {
                let config = ProofConfig {
                    private: ProofConfigPrivate {
                        something_private: 3,
                    },
                    consensus: ProofConfigConsensus { something: 1 },
                };
                (peer, config)
            })
            .collect();

        proof_cfg
            .into_iter()
            .map(|(k, v)| (k, v.to_erased()))
            .collect()
    }

    async fn distributed_gen(
        &self,
        _connections: &MuxPeerConnections<ModuleKey, DkgPeerMsg>,
        _our_id: &PeerId,
        _peers: &[PeerId],
        params: &ConfigGenParams,
        _task_group: &mut TaskGroup,
    ) -> anyhow::Result<Cancellable<ServerModuleConfig>> {
        let _params = params
            .get::<ProofConfigGenParams>()
            .expect("Invalid proof params");

        let server = ProofConfig {
            private: ProofConfigPrivate {
                something_private: 3,
            },
            consensus: ProofConfigConsensus { something: 2 },
        };

        Ok(Ok(server.to_erased()))
    }

    fn to_client_config(&self, config: ServerModuleConfig) -> anyhow::Result<ClientModuleConfig> {
        Ok(config
            .to_typed::<ProofConfig>()?
            .consensus
            .to_client_config())
    }

    fn to_client_config_from_consensus_value(
        &self,
        config: serde_json::Value,
    ) -> anyhow::Result<ClientModuleConfig> {
        Ok(serde_json::from_value::<ProofConfigConsensus>(config)?.to_client_config())
    }

    fn validate_config(&self, identity: &PeerId, config: ServerModuleConfig) -> anyhow::Result<()> {
        config.to_typed::<ProofConfig>()?.validate_config(identity)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofConfigGenParams {
    pub important_param: u64,
}

impl ModuleConfigGenParams for ProofConfigGenParams {
    const MODULE_NAME: &'static str = "proof";
}

#[derive(
    Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, Encodable, Decodable, Default,
)]
pub struct ProofInput;

impl fmt::Display for ProofInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProofInput")
    }
}

#[derive(
    Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, Encodable, Decodable, Default,
)]
pub struct ProofOutput;

impl fmt::Display for ProofOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProofOutput")
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, Encodable, Decodable)]
pub struct ProofOutputOutcome;

impl fmt::Display for ProofOutputOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProofOutputOutcome")
    }
}

impl fmt::Display for ProofOutputConfirmation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProofOutputConfirmation")
    }
}

#[async_trait]
impl ServerModulePlugin for Proof {
    type Decoder = ProofModuleDecoder;
    type Input = ProofInput;
    type Output = ProofOutput;
    type OutputOutcome = ProofOutputOutcome;
    type ConsensusItem = ProofOutputConfirmation;
    type VerificationCache = ProofVerificationCache;

    fn module_key(&self) -> ModuleKey {
        MODULE_KEY_PROOF
    }

    fn decoder(&self) -> &'static Self::Decoder {
        &ProofModuleDecoder
    }

    async fn await_consensus_proposal(&self, dbtx: &mut DatabaseTransaction<'_>) {
        if self.consensus_proposal(dbtx).await.is_empty() {
            std::future::pending().await
        }
    }

    async fn consensus_proposal(
        &self,
        _dbtx: &mut DatabaseTransaction<'_>,
    ) -> Vec<Self::ConsensusItem> {
        vec![]
    }

    async fn begin_consensus_epoch<'a, 'b>(
        &'a self,
        _dbtx: &mut DatabaseTransaction<'b>,
        _consensus_items: Vec<(PeerId, Self::ConsensusItem)>,
    ) {
    }

    fn build_verification_cache<'a>(
        &'a self,
        _inputs: impl Iterator<Item = &'a Self::Input> + Send,
    ) -> Self::VerificationCache {
        ProofVerificationCache
    }

    async fn validate_input<'a, 'b>(
        &self,
        _interconnect: &dyn ModuleInterconect,
        _dbtx: &mut DatabaseTransaction<'b>,
        _verification_cache: &Self::VerificationCache,
        _input: &'a Self::Input,
    ) -> Result<InputMeta, ModuleError> {
        unimplemented!()
    }

    async fn apply_input<'a, 'b, 'c>(
        &'a self,
        _interconnect: &'a dyn ModuleInterconect,
        _dbtx: &mut DatabaseTransaction<'c>,
        _input: &'b Self::Input,
        _cache: &Self::VerificationCache,
    ) -> Result<InputMeta, ModuleError> {
        unimplemented!()
    }

    async fn validate_output(
        &self,
        _dbtx: &mut DatabaseTransaction,
        _output: &Self::Output,
    ) -> Result<TransactionItemAmount, ModuleError> {
        unimplemented!()
    }

    async fn apply_output<'a, 'b>(
        &'a self,
        _dbtx: &mut DatabaseTransaction<'b>,
        _output: &'a Self::Output,
        _out_point: OutPoint,
    ) -> Result<TransactionItemAmount, ModuleError> {
        unimplemented!()
    }

    async fn end_consensus_epoch<'a, 'b>(
        &'a self,
        _consensus_peers: &HashSet<PeerId>,
        dbtx: &mut DatabaseTransaction<'b>,
    ) -> Vec<PeerId> {
        info!("proof: end_consensus_epoch");
        let utxos = self.available_utxos(dbtx).await;
        info!("available_utxos {:?}", &utxos);
        vec![]
    }

    async fn output_status(
        &self,
        _dbtx: &mut DatabaseTransaction<'_>,
        _out_point: OutPoint,
    ) -> Option<Self::OutputOutcome> {
        None
    }

    async fn audit(&self, _dbtx: &mut DatabaseTransaction<'_>, _audit: &mut Audit) {}

    fn api_base_name(&self) -> &'static str {
        "proof"
    }

    fn api_endpoints(&self) -> Vec<ApiEndpoint<Self>> {
        vec![api_endpoint! {
            "/proof_of_reserves",
            async |module: &Proof, dbtx, _params: ()| -> String {
                // TODO x.0.0 seems like smelly code...refactor this obj?
                Ok(module.available_utxos(&mut dbtx).await)
            }
        }]
    }
}

impl Proof {
    /// Create new module instance
    pub fn new(cfg: ProofConfig) -> Proof {
        Proof { cfg }
    }

    async fn available_utxos(&self, dbtx: &mut DatabaseTransaction<'_>) -> String {
        let utxos: Vec<(UTXOKey, SpendableUTXO)> = dbtx
            .find_by_prefix(&UTXOPrefixKey)
            .await
            .collect::<Result<_, _>>()
            .expect("DB error");

        let utxo_addresses: Vec<bitcoin::OutPoint> =
            utxos.into_iter().map(|(utxo_key, _)| utxo_key.0).collect();

        let json = serde_json::to_string(&utxo_addresses).unwrap();
        json
    }
}

// Must be unique.
// TODO: we need to provide guidence for allocating these
pub const MODULE_KEY_DUMMY: u16 = 128;
plugin_types_trait_impl!(
    MODULE_KEY_DUMMY,
    ProofInput,
    ProofOutput,
    ProofOutputOutcome,
    ProofOutputConfirmation,
    ProofVerificationCache
);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Error)]
pub enum ProofError {
    #[error("Something went wrong")]
    SomethingProofWentWrong,
}
