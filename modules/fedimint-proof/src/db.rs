use bitcoin::Txid;
use fedimint_api::db::DatabaseKeyPrefixConst;
use fedimint_api::encoding::{Decodable, Encodable};
use fedimint_wallet::UnsignedTransaction;
use secp256k1::ecdsa::Signature;
use serde::Serialize;
use strum_macros::EnumIter;

#[repr(u8)]
#[derive(Clone, EnumIter, Debug)]
pub enum DbKeyPrefix {
    ProofTxSigCi = 0x50,
    UnsignedProof = 0x51,
}

impl std::fmt::Display for DbKeyPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Encodable, Decodable, Serialize)]
pub struct ProofTxSignatureCI(pub Txid);

impl DatabaseKeyPrefixConst for ProofTxSignatureCI {
    const DB_PREFIX: u8 = DbKeyPrefix::ProofTxSigCi as u8;
    type Key = Self;
    type Value = Vec<Signature>; // TODO: define newtype
}

#[derive(Clone, Debug, Encodable, Decodable)]
pub struct ProofTxSignatureCIPrefix;

impl DatabaseKeyPrefixConst for ProofTxSignatureCIPrefix {
    const DB_PREFIX: u8 = DbKeyPrefix::ProofTxSigCi as u8;
    type Key = ProofTxSignatureCI;
    type Value = Vec<Signature>;
}

#[derive(Clone, Debug, Encodable, Decodable, Serialize)]
pub struct UnsignedProofKey(pub Txid);

impl DatabaseKeyPrefixConst for UnsignedProofKey {
    const DB_PREFIX: u8 = DbKeyPrefix::UnsignedProof as u8;
    type Key = Self;
    type Value = UnsignedTransaction;
}

#[derive(Clone, Debug, Encodable, Decodable)]
pub struct UnsignedProofPrefixKey;

impl DatabaseKeyPrefixConst for UnsignedProofPrefixKey {
    const DB_PREFIX: u8 = DbKeyPrefix::UnsignedProof as u8;
    type Key = UnsignedProofKey;
    type Value = UnsignedTransaction;
}
