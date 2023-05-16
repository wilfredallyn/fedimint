window.SIDEBAR_ITEMS = {"constant":[["DEFAULT_MAX_NOTES_PER_DENOMINATION","By default, the maximum notes per denomination when change-making for users"],["KIND",""],["LOG_TARGET",""],["MINT_BACKUP_RESTORE_OPERATION_ID",""],["MINT_E_CASH_TYPE_CHILD_ID",""]],"enum":[["CombineError",""],["MintClientStateMachines",""],["MintError",""],["MintMetaVariants",""],["PeerErrorType",""],["ReissueExternalNotesState","The high-level state of a reissue operation started with [`MintClientExt::reissue_external_notes`]."],["SpendOOBState","The high-level state of a raw e-cash spend operation started with [`MintClientExt::spend_notes`]."]],"fn":[["mint_operation",""],["parse_ecash",""],["select_notes_from_stream",""]],"mod":[["backup",""],["config",""],["db","Database keys used throughout the mint client module"],["input","State machines for mint inputs"],["oob","State machines for out-of-band transmitted e-cash notes"],["output","State machines for mint outputs"]],"struct":[["BackupRequest",""],["BlindNonce","[`Nonce`] but blinded by the user key"],["InsufficientBalanceError",""],["MintClientContext",""],["MintClientGen",""],["MintClientModule",""],["MintCommonGen",""],["MintConsensusItem","Data structures taking into account different amount tiers A consenus item from one of the federation members contributing partials signatures to blind nonces submitted in it"],["MintInput",""],["MintMeta",""],["MintModuleTypes",""],["MintOutput",""],["MintOutputBlindSignatures","Result of Federation members confirming [`MintOutput`] by contributing partial signatures via [`MintConsensusItem`]"],["MintOutputOutcome",""],["MintOutputSignatureShare","Blind signature share from one Federation peer for a single [`MintOutput`]"],["MintShareErrors","Represents an array of mint indexes that delivered faulty shares"],["Nonce","Unique ID of a mint note."],["Note","An verifiable one time use IOU from the mint."],["NoteIndex","An index used to deterministically derive [`Note`]s"],["SignedBackupRequest",""],["SpendOOBRefund",""],["SpendableNote","A [`Note`] with associated secret key that allows to proof ownership (spend it)"]],"trait":[["MintClientExt",""]]};