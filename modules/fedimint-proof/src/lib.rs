use std::collections::{BTreeMap, HashSet};
use std::fmt;

use async_trait::async_trait;
use bitcoin::Txid;
use common::ProofModuleDecoder;
use fedimint_api::cancellable::Cancellable;
use fedimint_api::config::TypedServerModuleConsensusConfig;
use fedimint_api::config::{
    ClientModuleConfig, ConfigGenParams, DkgPeerMsg, ModuleConfigGenParams, ServerModuleConfig,
    TypedServerModuleConfig,
};
use fedimint_api::core::{Decoder, ModuleKey, MODULE_KEY_PROOF};
use fedimint_api::db::{Database, DatabaseTransaction};
use fedimint_api::encoding::{Decodable, Encodable, UnzipConsensus};
use fedimint_api::module::__reexports::serde_json;
use fedimint_api::module::audit::Audit;
use fedimint_api::module::interconnect::ModuleInterconect;
use fedimint_api::module::{
    api_endpoint, ApiEndpoint, InputMeta, ModuleError, ModuleInit, TransactionItemAmount,
};
use fedimint_api::net::peers::MuxPeerConnections;
use fedimint_api::server::ServerModule;
use fedimint_api::task::TaskGroup;
use fedimint_api::Amount;
use fedimint_api::{plugin_types_trait_impl, OutPoint, PeerId, ServerModulePlugin};
use fedimint_wallet::db::{UTXOKey, UTXOPrefixKey};
use fedimint_wallet::{PegOutSignatureItem, SpendableUTXO, UnsignedTransaction};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{info, warn};

use crate::config::{ProofConfig, ProofConfigConsensus, ProofConfigPrivate};
use crate::db::{ProofTxSignatureCIPrefix, UnsignedProofKey, UnsignedProofPrefixKey};

pub mod common;
pub mod config;
pub mod db;

#[derive(Debug)]
pub struct Proof {
    pub cfg: ProofConfig,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Encodable, Decodable)]
pub struct ProofOutputConfirmation;

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, UnzipConsensus, Encodable, Decodable,
)]
pub enum ProofConsensusItem {
    ProofSignature(PegOutSignatureItem),
}

impl std::fmt::Display for ProofConsensusItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProofConsensusItem::ProofSignature(sig) => {
                write!(f, "Proof signature for Bitcoin TxId {}", sig.txid)
            }
        }
    }
}

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
    type ConsensusItem = ProofConsensusItem;
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
        dbtx: &mut DatabaseTransaction<'_>,
    ) -> Vec<Self::ConsensusItem> {
        info!("proof consensus proposal");
        dbtx.find_by_prefix(&ProofTxSignatureCIPrefix)
            .await
            .map(|res| {
                let (key, val) = res.expect("FB error");
                ProofConsensusItem::ProofSignature(PegOutSignatureItem {
                    txid: key.0,
                    signature: val,
                })
            })
            .collect()
    }

    async fn begin_consensus_epoch<'a, 'b>(
        &'a self,
        dbtx: &mut DatabaseTransaction<'b>,
        consensus_items: Vec<(PeerId, Self::ConsensusItem)>,
    ) {
        info!("proof begin_consensus_epoch");
        let UnzipProofConsensusItem {
            proof_signature: proof_signatures,
        } = consensus_items.into_iter().unzip_proof_consensus_item();
        // UnzipWalletConsensusItem
        // Save signatures to the database
        self.save_proof_signatures(dbtx, proof_signatures).await;

        // dbtx.insert_entry(&RoundConsensusKey, &round_consensus)
    }

    fn build_verification_cache<'a>(
        &'a self,
        _inputs: impl Iterator<Item = &'a Self::Input> + Send,
    ) -> Self::VerificationCache {
        ProofVerificationCache
    }

    async fn validate_input<'a, 'b>(
        &self,
        interconnect: &dyn ModuleInterconect,
        dbtx: &mut DatabaseTransaction<'b>,
        verification_cache: &Self::VerificationCache,
        input: &'a Self::Input,
    ) -> Result<InputMeta, ModuleError> {
        Ok(InputMeta {
            amount: TransactionItemAmount {
                amount: Amount { msats: 0 },
                fee: Amount { msats: 0 },
            },
            puk_keys: vec![],
        })
    }

    async fn apply_input<'a, 'b, 'c>(
        &'a self,
        interconnect: &'a dyn ModuleInterconect,
        dbtx: &mut DatabaseTransaction<'c>,
        input: &'b Self::Input,
        cache: &Self::VerificationCache,
    ) -> Result<InputMeta, ModuleError> {
        let meta = self
            .validate_input(interconnect, dbtx, cache, input)
            .await?;
        info!("proof apply input");
        proof_tx(interconnect).await;
        // let (proof_tx, sigs) = proof_tx(interconnect).await;
        // info!("apply_input proof_tx: {:?}", proof_tx);
        // use api endpoint with interconect
        // call wallet.offline_wallet functions in interconect (need wallet.cfg)
        // create_tx
        // sign_psbt
        // derive_script
        Ok(meta)
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

        // Need to remove UnsignedProof from db
        // dbtx.remove_entry(&ProofTxSignatureCI(key.0))
        //     .await
        //     .expect("DB Error");
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

    async fn save_proof_signatures<'a>(
        &self,
        dbtx: &mut DatabaseTransaction<'a>,
        signatures: Vec<(PeerId, PegOutSignatureItem)>,
    ) {
        let mut cache: BTreeMap<Txid, UnsignedTransaction> = dbtx
            .find_by_prefix(&UnsignedProofPrefixKey)
            .await
            .map(|res| {
                let (key, val) = res.expect("DB error");
                (key.0, val)
            })
            .collect();

        for (peer, sig) in signatures.into_iter() {
            match cache.get_mut(&sig.txid) {
                Some(unsigned) => unsigned.signatures.push((peer, sig)),
                None => warn!(
                    "{} sent proof signature for unknown PSBT {}",
                    peer, sig.txid
                ),
            }
        }

        for (txid, unsigned) in cache.into_iter() {
            dbtx.insert_entry(&UnsignedProofKey(txid), &unsigned)
                .await
                .expect("DB Error");
        }
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
    fedimint_api::core::MODULE_KEY_PROOF,
    ProofInput,
    ProofOutput,
    ProofOutputOutcome,
    ProofConsensusItem,
    ProofVerificationCache
);

async fn proof_tx(interconnect: &dyn ModuleInterconect) {
    // -> (UnsignedTransaction, Vec<secp256k1::ecdsa::Signature>)
    // This is a future because we are normally reading from a network socket. But for internal
    // calls the data is available instantly in one go, so we can just block on it.
    let body = interconnect
        .call("wallet", "/proof_tx".to_owned(), Default::default())
        .await
        .expect("Wallet module not present or malfunctioning!");

    info!("interconnect body {:?}", &body);
    // serde_json::from_value(body).expect("Malformed block height response from wallet module!")
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Error)]
pub enum ProofError {
    #[error("Something went wrong")]
    SomethingProofWentWrong,
}
