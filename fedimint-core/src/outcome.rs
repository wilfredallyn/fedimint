use fedimint_api::serde_module_encoding_wrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum TransactionStatus {
    /// The rejected state is only recorded if the error happens after consensus is achieved on the
    /// transaction. This should happen only rarely, e.g. on double spends since a basic validity
    /// check is performed on transaction submission or on not having enough UTXOs to peg-out.
    Rejected(String),
    /// The transaction was accepted and is now being processed
    Accepted {
        epoch: u64,
        outputs: Vec<SerdeOutputOutcome>,
    },
}

serde_module_encoding_wrapper!(SerdeOutputOutcome, fedimint_api::core::OutputOutcome);

pub mod legacy {
    use fedimint_api::core::{MODULE_KEY_LN, MODULE_KEY_MINT, MODULE_KEY_PROOF, MODULE_KEY_WALLET};
    use fedimint_api::encoding::{Decodable, Encodable};
    use fedimint_api::ServerModulePlugin;
    use fedimint_ln::contracts::incoming::OfferId;
    use fedimint_ln::contracts::{
        AccountContractOutcome, ContractOutcome, DecryptedPreimage, OutgoingContractOutcome,
        Preimage,
    };
    use fedimint_ln::{LightningModule, LightningOutputOutcome};
    use fedimint_mint::{Mint, MintOutputOutcome};
    use fedimint_proof::{Proof, ProofOutputOutcome};
    use fedimint_wallet::{Wallet, WalletOutputOutcome};

    use crate::CoreError;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Encodable, Decodable)]
    pub enum OutputOutcome {
        Mint(<Mint as ServerModulePlugin>::OutputOutcome),
        Wallet(<Wallet as ServerModulePlugin>::OutputOutcome),
        LN(<LightningModule as ServerModulePlugin>::OutputOutcome),
        Proof(<Proof as ServerModulePlugin>::OutputOutcome),
    }

    impl From<fedimint_api::core::OutputOutcome> for OutputOutcome {
        fn from(oo: fedimint_api::core::OutputOutcome) -> Self {
            match oo.module_key() {
                MODULE_KEY_LN => OutputOutcome::LN(
                    oo.as_any()
                        .downcast_ref::<LightningOutputOutcome>()
                        .expect("Module key matches")
                        .clone(),
                ),
                MODULE_KEY_MINT => OutputOutcome::Mint(
                    oo.as_any()
                        .downcast_ref::<MintOutputOutcome>()
                        .expect("Module key matches")
                        .clone(),
                ),
                MODULE_KEY_WALLET => OutputOutcome::Wallet(
                    oo.as_any()
                        .downcast_ref::<WalletOutputOutcome>()
                        .expect("Module key matches")
                        .clone(),
                ),
                MODULE_KEY_PROOF => OutputOutcome::Proof(
                    oo.as_any()
                        .downcast_ref::<ProofOutputOutcome>()
                        .expect("Module key matches")
                        .clone(),
                ),
                _ => panic!("Unknown Module"),
            }
        }
    }

    pub trait TryIntoOutcome: Sized {
        fn try_into_outcome(common_outcome: OutputOutcome) -> Result<Self, CoreError>;
    }

    impl OutputOutcome {
        pub fn try_into_variant<T: TryIntoOutcome>(self) -> Result<T, CoreError> {
            T::try_into_outcome(self)
        }
    }

    impl TryIntoOutcome for MintOutputOutcome {
        fn try_into_outcome(common_outcome: OutputOutcome) -> Result<Self, CoreError> {
            match common_outcome {
                OutputOutcome::Mint(outcome) => Ok(outcome),
                OutputOutcome::Wallet(_) => Err(CoreError::MismatchingVariant("mint", "wallet")),
                OutputOutcome::LN(_) => Err(CoreError::MismatchingVariant("mint", "ln")),
                OutputOutcome::Proof(_) => Err(CoreError::MismatchingVariant("mint", "proof")),
            }
        }
    }

    impl TryIntoOutcome for WalletOutputOutcome {
        fn try_into_outcome(common_outcome: OutputOutcome) -> Result<Self, CoreError> {
            match common_outcome {
                OutputOutcome::Mint(_) => Err(CoreError::MismatchingVariant("wallet", "mint")),
                OutputOutcome::Wallet(outcome) => Ok(outcome),
                OutputOutcome::LN(_) => Err(CoreError::MismatchingVariant("wallet", "ln")),
                OutputOutcome::Proof(_) => Err(CoreError::MismatchingVariant("wallet", "proof")),
            }
        }
    }

    impl TryIntoOutcome for fedimint_ln::LightningOutputOutcome {
        fn try_into_outcome(common_outcome: OutputOutcome) -> Result<Self, CoreError> {
            match common_outcome {
                OutputOutcome::Mint(_) => Err(CoreError::MismatchingVariant("ln", "mint")),
                OutputOutcome::Wallet(_) => Err(CoreError::MismatchingVariant("ln", "wallet")),
                OutputOutcome::LN(outcome) => Ok(outcome),
                OutputOutcome::Proof(_) => Err(CoreError::MismatchingVariant("ln", "proof")),
            }
        }
    }

    impl TryIntoOutcome for ProofOutputOutcome {
        fn try_into_outcome(common_outcome: OutputOutcome) -> Result<Self, CoreError> {
            match common_outcome {
                OutputOutcome::Mint(_) => Err(CoreError::MismatchingVariant("proof", "mint")),
                OutputOutcome::Wallet(_) => Err(CoreError::MismatchingVariant("proof", "wallet")),
                OutputOutcome::LN(_) => Err(CoreError::MismatchingVariant("proof", "ln")),
                OutputOutcome::Proof(outcome) => Ok(outcome),
            }
        }
    }

    impl TryIntoOutcome for Preimage {
        fn try_into_outcome(common_outcome: OutputOutcome) -> Result<Self, CoreError> {
            if let OutputOutcome::LN(fedimint_ln::LightningOutputOutcome::Contract {
                outcome: ContractOutcome::Incoming(decrypted_preimage),
                ..
            }) = common_outcome
            {
                match decrypted_preimage {
                    DecryptedPreimage::Some(preimage) => Ok(preimage),
                    DecryptedPreimage::Pending => Err(CoreError::PendingPreimage),
                    _ => Err(CoreError::MismatchingVariant("ln::incoming", "other")),
                }
            } else {
                Err(CoreError::MismatchingVariant("ln::incoming", "other"))
            }
        }
    }

    impl TryIntoOutcome for OfferId {
        fn try_into_outcome(common_outcome: OutputOutcome) -> Result<Self, CoreError> {
            if let OutputOutcome::LN(fedimint_ln::LightningOutputOutcome::Offer { id }) =
                common_outcome
            {
                Ok(id)
            } else {
                Err(CoreError::MismatchingVariant("ln::incoming", "other"))
            }
        }
    }

    impl TryIntoOutcome for AccountContractOutcome {
        fn try_into_outcome(common_outcome: OutputOutcome) -> Result<Self, CoreError> {
            if let OutputOutcome::LN(fedimint_ln::LightningOutputOutcome::Contract {
                outcome: ContractOutcome::Account(o),
                ..
            }) = common_outcome
            {
                Ok(o)
            } else {
                Err(CoreError::MismatchingVariant("ln::account", "other"))
            }
        }
    }

    impl TryIntoOutcome for OutgoingContractOutcome {
        fn try_into_outcome(common_outcome: OutputOutcome) -> Result<Self, CoreError> {
            if let OutputOutcome::LN(fedimint_ln::LightningOutputOutcome::Contract {
                outcome: ContractOutcome::Outgoing(o),
                ..
            }) = common_outcome
            {
                Ok(o)
            } else {
                Err(CoreError::MismatchingVariant("ln::outgoing", "other"))
            }
        }
    }
}
