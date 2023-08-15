(function() {var implementors = {
"fedimint_atomic_broadcast":[["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_atomic_broadcast/db/struct.SignedBlockKey.html\" title=\"struct fedimint_atomic_broadcast::db::SignedBlockKey\">SignedBlockKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_atomic_broadcast/db/struct.UnitsKey.html\" title=\"struct fedimint_atomic_broadcast::db::UnitsKey\">UnitsKey</a>"]],
"fedimint_client":[["impl&lt;S&gt; <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client/db/struct.ClientSecretKey.html\" title=\"struct fedimint_client::db::ClientSecretKey\">ClientSecretKey</a>&lt;S&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"fedimint_client/secret/trait.RootSecretStrategy.html\" title=\"trait fedimint_client::secret::RootSecretStrategy\">RootSecretStrategy</a>,</span>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client/db/struct.OperationLogKey.html\" title=\"struct fedimint_client::db::OperationLogKey\">OperationLogKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client/db/struct.ChronologicalOperationLogKey.html\" title=\"struct fedimint_client::db::ChronologicalOperationLogKey\">ChronologicalOperationLogKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client/db/struct.CachedApiVersionSetKey.html\" title=\"struct fedimint_client::db::CachedApiVersionSetKey\">CachedApiVersionSetKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client/db/struct.ClientConfigKey.html\" title=\"struct fedimint_client::db::ClientConfigKey\">ClientConfigKey</a>"],["impl&lt;GC&gt; <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client/sm/executor/struct.ActiveStateKey.html\" title=\"struct fedimint_client::sm::executor::ActiveStateKey\">ActiveStateKey</a>&lt;GC&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;GC: <a class=\"trait\" href=\"fedimint_client/sm/trait.GlobalContext.html\" title=\"trait fedimint_client::sm::GlobalContext\">GlobalContext</a>,</span>"],["impl&lt;GC&gt; <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client/sm/executor/struct.InactiveStateKey.html\" title=\"struct fedimint_client::sm::executor::InactiveStateKey\">InactiveStateKey</a>&lt;GC&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;GC: <a class=\"trait\" href=\"fedimint_client/sm/trait.GlobalContext.html\" title=\"trait fedimint_client::sm::GlobalContext\">GlobalContext</a>,</span>"]],
"fedimint_client_legacy":[["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client_legacy/db/struct.ClientSecretKey.html\" title=\"struct fedimint_client_legacy::db::ClientSecretKey\">ClientSecretKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client_legacy/ln/db/struct.OutgoingPaymentKey.html\" title=\"struct fedimint_client_legacy::ln::db::OutgoingPaymentKey\">OutgoingPaymentKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client_legacy/ln/db/struct.OutgoingPaymentClaimKey.html\" title=\"struct fedimint_client_legacy::ln::db::OutgoingPaymentClaimKey\">OutgoingPaymentClaimKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client_legacy/ln/db/struct.OutgoingContractAccountKey.html\" title=\"struct fedimint_client_legacy::ln::db::OutgoingContractAccountKey\">OutgoingContractAccountKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client_legacy/ln/db/struct.ConfirmedInvoiceKey.html\" title=\"struct fedimint_client_legacy::ln::db::ConfirmedInvoiceKey\">ConfirmedInvoiceKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client_legacy/ln/db/struct.LightningGatewayKey.html\" title=\"struct fedimint_client_legacy::ln::db::LightningGatewayKey\">LightningGatewayKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client_legacy/mint/db/struct.NoteKey.html\" title=\"struct fedimint_client_legacy::mint::db::NoteKey\">NoteKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client_legacy/mint/db/struct.PendingNotesKey.html\" title=\"struct fedimint_client_legacy::mint::db::PendingNotesKey\">PendingNotesKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client_legacy/mint/db/struct.OutputFinalizationKey.html\" title=\"struct fedimint_client_legacy::mint::db::OutputFinalizationKey\">OutputFinalizationKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client_legacy/mint/db/struct.NextECashNoteIndexKey.html\" title=\"struct fedimint_client_legacy::mint::db::NextECashNoteIndexKey\">NextECashNoteIndexKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client_legacy/mint/db/struct.NotesPerDenominationKey.html\" title=\"struct fedimint_client_legacy::mint::db::NotesPerDenominationKey\">NotesPerDenominationKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_client_legacy/wallet/db/struct.PegInKey.html\" title=\"struct fedimint_client_legacy::wallet::db::PegInKey\">PegInKey</a>"]],
"fedimint_core":[],
"fedimint_dummy_client":[["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_dummy_client/db/struct.DummyClientFundsKeyV0.html\" title=\"struct fedimint_dummy_client::db::DummyClientFundsKeyV0\">DummyClientFundsKeyV0</a>"]],
"fedimint_dummy_server":[["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_dummy_server/db/struct.DummyFundsKeyV0.html\" title=\"struct fedimint_dummy_server::db::DummyFundsKeyV0\">DummyFundsKeyV0</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_dummy_server/db/struct.DummyFundsKeyV1.html\" title=\"struct fedimint_dummy_server::db::DummyFundsKeyV1\">DummyFundsKeyV1</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_dummy_server/db/struct.DummyOutcomeKey.html\" title=\"struct fedimint_dummy_server::db::DummyOutcomeKey\">DummyOutcomeKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_dummy_server/db/struct.DummySignatureShareKey.html\" title=\"struct fedimint_dummy_server::db::DummySignatureShareKey\">DummySignatureShareKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_dummy_server/db/struct.DummySignatureKey.html\" title=\"struct fedimint_dummy_server::db::DummySignatureKey\">DummySignatureKey</a>"]],
"fedimint_ln_client":[["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_ln_client/db/struct.LightningGatewayKey.html\" title=\"struct fedimint_ln_client::db::LightningGatewayKey\">LightningGatewayKey</a>"]],
"fedimint_ln_common":[["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.ContractKey.html\" title=\"struct fedimint_ln_common::db::ContractKey\">ContractKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.ContractUpdateKey.html\" title=\"struct fedimint_ln_common::db::ContractUpdateKey\">ContractUpdateKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.OfferKey.html\" title=\"struct fedimint_ln_common::db::OfferKey\">OfferKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.ProposeDecryptionShareKey.html\" title=\"struct fedimint_ln_common::db::ProposeDecryptionShareKey\">ProposeDecryptionShareKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.AgreedDecryptionShareKey.html\" title=\"struct fedimint_ln_common::db::AgreedDecryptionShareKey\">AgreedDecryptionShareKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.LightningGatewayKey.html\" title=\"struct fedimint_ln_common::db::LightningGatewayKey\">LightningGatewayKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_ln_common/db/struct.BlockCountVoteKey.html\" title=\"struct fedimint_ln_common::db::BlockCountVoteKey\">BlockCountVoteKey</a>"]],
"fedimint_mint_client":[["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_mint_client/db/struct.NoteKey.html\" title=\"struct fedimint_mint_client::db::NoteKey\">NoteKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_mint_client/db/struct.NextECashNoteIndexKey.html\" title=\"struct fedimint_mint_client::db::NextECashNoteIndexKey\">NextECashNoteIndexKey</a>"]],
"fedimint_mint_common":[["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_mint_common/db/struct.NonceKey.html\" title=\"struct fedimint_mint_common::db::NonceKey\">NonceKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_mint_common/db/struct.ProposedPartialSignatureKey.html\" title=\"struct fedimint_mint_common::db::ProposedPartialSignatureKey\">ProposedPartialSignatureKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_mint_common/db/struct.ReceivedPartialSignatureKey.html\" title=\"struct fedimint_mint_common::db::ReceivedPartialSignatureKey\">ReceivedPartialSignatureKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_mint_common/db/struct.OutputOutcomeKey.html\" title=\"struct fedimint_mint_common::db::OutputOutcomeKey\">OutputOutcomeKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"enum\" href=\"fedimint_mint_common/db/enum.MintAuditItemKey.html\" title=\"enum fedimint_mint_common::db::MintAuditItemKey\">MintAuditItemKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_mint_common/db/struct.EcashBackupKey.html\" title=\"struct fedimint_mint_common::db::EcashBackupKey\">EcashBackupKey</a>"]],
"fedimint_server":[["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_server/db/struct.AcceptedTransactionKey.html\" title=\"struct fedimint_server::db::AcceptedTransactionKey\">AcceptedTransactionKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_server/db/struct.EpochHistoryKey.html\" title=\"struct fedimint_server::db::EpochHistoryKey\">EpochHistoryKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_server/db/struct.LastEpochKey.html\" title=\"struct fedimint_server::db::LastEpochKey\">LastEpochKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_server/db/struct.ClientConfigSignatureKey.html\" title=\"struct fedimint_server::db::ClientConfigSignatureKey\">ClientConfigSignatureKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_server/db/struct.ClientConfigSignatureShareKey.html\" title=\"struct fedimint_server::db::ClientConfigSignatureShareKey\">ClientConfigSignatureShareKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_server/db/struct.ConsensusUpgradeKey.html\" title=\"struct fedimint_server::db::ConsensusUpgradeKey\">ConsensusUpgradeKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_server/db/struct.ClientConfigDownloadKey.html\" title=\"struct fedimint_server::db::ClientConfigDownloadKey\">ClientConfigDownloadKey</a>"]],
"fedimint_wallet_client":[["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_wallet_client/db/struct.NextPegInTweakIndexKey.html\" title=\"struct fedimint_wallet_client::db::NextPegInTweakIndexKey\">NextPegInTweakIndexKey</a>"]],
"fedimint_wallet_common":[["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.BlockHashKey.html\" title=\"struct fedimint_wallet_common::db::BlockHashKey\">BlockHashKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.UTXOKey.html\" title=\"struct fedimint_wallet_common::db::UTXOKey\">UTXOKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.UnsignedTransactionKey.html\" title=\"struct fedimint_wallet_common::db::UnsignedTransactionKey\">UnsignedTransactionKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.PendingTransactionKey.html\" title=\"struct fedimint_wallet_common::db::PendingTransactionKey\">PendingTransactionKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.PegOutTxSignatureCI.html\" title=\"struct fedimint_wallet_common::db::PegOutTxSignatureCI\">PegOutTxSignatureCI</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.PegOutBitcoinTransaction.html\" title=\"struct fedimint_wallet_common::db::PegOutBitcoinTransaction\">PegOutBitcoinTransaction</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.BlockCountVoteKey.html\" title=\"struct fedimint_wallet_common::db::BlockCountVoteKey\">BlockCountVoteKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.FeeRateVoteKey.html\" title=\"struct fedimint_wallet_common::db::FeeRateVoteKey\">FeeRateVoteKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"fedimint_wallet_common/db/struct.PegOutNonceKey.html\" title=\"struct fedimint_wallet_common::db::PegOutNonceKey\">PegOutNonceKey</a>"]],
"ln_gateway":[["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"ln_gateway/db/struct.FederationIdKey.html\" title=\"struct ln_gateway::db::FederationIdKey\">FederationIdKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"ln_gateway/db/struct.FederationRegistrationKey.html\" title=\"struct ln_gateway::db::FederationRegistrationKey\">FederationRegistrationKey</a>"],["impl <a class=\"trait\" href=\"fedimint_core/db/trait.DatabaseRecord.html\" title=\"trait fedimint_core::db::DatabaseRecord\">DatabaseRecord</a> for <a class=\"struct\" href=\"ln_gateway/db/struct.GatewayPublicKey.html\" title=\"struct ln_gateway::db::GatewayPublicKey\">GatewayPublicKey</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()