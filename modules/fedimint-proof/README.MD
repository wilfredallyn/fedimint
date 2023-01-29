# Proof of Reserves Module

## Overview
We created a module to automatically create a proof-of-reserves transaction as funds are pegged in and out. This transaction proves that the federation is fully reserved.

We chose to work on this since a major obstacle to widespread adoption of Fedimint is trust. Two of the major tradeoffs are [custodial and debasement risk](https://fedimint.org/docs/TradeOffs/Trust-Trade-Offs). The average non-technical person will be skeptical, and increasing transparency will help address their concerns.

## Code Description
The module is based on commit `487efadce7` from the Fedimint master branch. Most of the logic for creating a proof-of-reserves transaction is in the Wallet module. This decision is explained in the [Implementation Details](#implementation-details) section.

### Code changes

[List of all commits and changes](https://github.com/fedimint/fedimint/compare/master...wilfredallyn:fedimint:module)

### Wallet Module
* `consensus_proposal`: [get signature shares for proof tx](https://github.com/wilfredallyn/fedimint/blob/615c86567077a2e94101159c896dae49149e8a5e/modules/fedimint-wallet/src/lib.rs#L458)
* `begin_consensus_epoch`: [calls](https://github.com/wilfredallyn/fedimint/blob/dfe8e266b0ed7951f6f5f34b2c06d58316d65978/modules/fedimint-wallet/src/lib.rs#L481) [`create_proof_tx`](https://github.com/wilfredallyn/fedimint/blob/615c86567077a2e94101159c896dae49149e8a5e/modules/fedimint-wallet/src/lib.rs#L1248) which contains the logic for creating and signing the transaction
    * Check [if there is an existing proof tx](https://github.com/wilfredallyn/fedimint/blob/dfe8e266b0ed7951f6f5f34b2c06d58316d65978/modules/fedimint-wallet/src/lib.rs#L1268) in the db
    * If the current wallet value [has not changed more than 10%](https://github.com/wilfredallyn/fedimint/blob/dfe8e266b0ed7951f6f5f34b2c06d58316d65978/modules/fedimint-wallet/src/lib.rs#L1269-L1282), return without creating new tx. Wallet consensus round will continue as before.
    * If the current wallet value has changed more than 10%, [create a new proof tx](https://github.com/wilfredallyn/fedimint/blob/dfe8e266b0ed7951f6f5f34b2c06d58316d65978/modules/fedimint-wallet/src/lib.rs#L1303-L1309).
        * Creates tx by following logic in `create_peg_out_tx` and `apply_output`: create and sign psbt, take out signature, add UnsignedProof to database, add signature shares (`ProofTxSignatureCI`) to database
        * Save proof signatures to the database
* `end_consensus_epoch`: [calls](https://github.com/wilfredallyn/fedimint/blob/dfe8e266b0ed7951f6f5f34b2c06d58316d65978/modules/fedimint-wallet/src/lib.rs#L702) [`sign_and_finalize_proof_tx`](https://github.com/wilfredallyn/fedimint/blob/dfe8e266b0ed7951f6f5f34b2c06d58316d65978/modules/fedimint-wallet/src/lib.rs#L1373) which follows logic of `end_consensus_epoch` for peg-outs
    * [Sign and finalize](https://github.com/wilfredallyn/fedimint/blob/dfe8e266b0ed7951f6f5f34b2c06d58316d65978/modules/fedimint-wallet/src/lib.rs#L1378-L1416) the unsigned proof tx
    * [Add signed proof tx](https://github.com/wilfredallyn/fedimint/blob/dfe8e266b0ed7951f6f5f34b2c06d58316d65978/modules/fedimint-wallet/src/lib.rs#L1420-L1426) (`SignedProofKey`) to database, removes signature shares (`ProofTxSignatureCI`)

### Proof Module
* [`get_proof_tx_hex`](https://github.com/wilfredallyn/fedimint/blob/f2e4ceb54f129371480e1eb9becb0b4a4ca6c437/modules/fedimint-proof/src/lib.rs#L343-L357): function called by client to get the current proof transaction from the database, returns hex
* [`get_proof_tx_value`](https://github.com/wilfredallyn/fedimint/blob/f2e4ceb54f129371480e1eb9becb0b4a4ca6c437/modules/fedimint-proof/src/lib.rs#L359-L380): function called by client to get the current value of the proof transaction

### Other changes
* Added [`ProofTxHex`](https://github.com/wilfredallyn/fedimint/blob/f2e4ceb54f129371480e1eb9becb0b4a4ca6c437/client/cli/src/main.rs#L114-L116) and [`ProofTxValue`](https://github.com/wilfredallyn/fedimint/blob/f2e4ceb54f129371480e1eb9becb0b4a4ca6c437/client/cli/src/main.rs#L118-L121) commands to cli
* [Updated database docs](https://github.com/wilfredallyn/fedimint/commit/6cccc7b0ef049edc8c14a1abbc7cd21058c2088e)
* [Updated dbdump](https://github.com/wilfredallyn/fedimint/commit/8b4528495010fb7e9b3ac60f49325742acea0f1f#diff-df13abbff07deea9410695e2b8fd054f79b7fada0ac3f56e432ba063f9d0e4a6)
* [Added integration test](https://github.com/wilfredallyn/fedimint/blob/327464ebeacaefbbec1d985dd17ff8584cc2f2be/integrationtests/tests/tests.rs#L1045-L1076)


## Implementation Details
Initially, we tried to put the logic for creating the proof transaction in the Proof module but needed functionality from the Wallet module
* Tried to use the `StatelessWallet` but couldn't get information from the wallet config into the proof config (`cfg.consensus.peg_in_descriptor` and `cfg.private.peg_in_key` from the wallet config)
* Tried to use `ModuleInterconnect` but that was limited to the `validate_input` and `apply_input` methods

The proof-of-reserves transaction typically is a [signed transaction with an invalid input](https://github.com/wilfredallyn/fedimint/blob/dfe8e266b0ed7951f6f5f34b2c06d58316d65978/modules/fedimint-wallet/src/lib.rs#L1320-L1330) to make the transaction unspendable. We tried to add this but couldn't get the decoder to work to process the transaction.

## Example usage (tmuxinator)
```
$ ./scripts/tmuxinator.sh

$ source ./scripts/pegin.sh 50000
# Whenever the wallet value changes > 10% after peg-ins and peg-outs, a new proof transaction is created and signed with all available utxos (minus fees).

$ fedimint-cli proof-tx-hex
# This shows that a proof transaction was created after the peg-in.
{
  "hex": "020000000001023c27878366f4dbf..."
}

$ btc_client decoderawtransaction 020000000001023c27878366f4dbf...
# We can decode the transaction hex to see that we have a valid signed transaction consisting of available utxos minus fees.

{
    "txid": "eccc175338e91e0401b121d5800b2afa3c015d809f4cf9b7c48eec950b0ac8ba",
    "hash": "7a0d380cdfb8f11dedbcb280300c10b72eef7ceff41430e723eb086839368193",
    "version": 2,
    "size": 899,
    ...
    "vout": [
        {
        "value": 0.00028000,
        "n": 0,
        "scriptPubKey": {
            "asm": "0 db56ee981b501faeba043bf29cbf2c37a51382c2f26ff9c67c5d4fef93ffaa65",
            "desc": "addr(bcrt1qmdtwaxqm2q06awsy80efe0evx7j38qkz7fhln3nut487lyll4fjstk8zvx)#f2ce4vmc",
            "hex": "0020db56ee981b501faeba043bf29cbf2c37a51382c2f26ff9c67c5d4fef93ffaa65",
            "address": "bcrt1qmdtwaxqm2q06awsy80efe0evx7j38qkz7fhln3nut487lyll4fjstk8zvx",
            "type": "witness_v0_scripthash"
        }
        },
        {
        "value": 0.00000556,
        "n": 1,
        "scriptPubKey": {
            "asm": "0 db56ee981b501faeba043bf29cbf2c37a51382c2f26ff9c67c5d4fef93ffaa65",
            "desc": "addr(bcrt1qmdtwaxqm2q06awsy80efe0evx7j38qkz7fhln3nut487lyll4fjstk8zvx)#f2ce4vmc",
            "hex": "0020db56ee981b501faeba043bf29cbf2c37a51382c2f26ff9c67c5d4fef93ffaa65",
            "address": "bcrt1qmdtwaxqm2q06awsy80efe0evx7j38qkz7fhln3nut487lyll4fjstk8zvx",
            "type": "witness_v0_scripthash"
        }
    }
]

$ fedimint-cli proof-tx-value
# This is a simple command to show the value of the proof transaction
{
  "value": 28556000  [in msats]
}
```

## Example usage (integration Test)
We also created an integration test ([`proof`](https://github.com/wilfredallyn/fedimint/blob/327464ebeacaefbbec1d985dd17ff8584cc2f2be/integrationtests/tests/tests.rs#L1045-L1076)) that shows how to get the proof transaction hex and value from the client
`user.client.proof_client().get_proof_tx_value()` and `get_proof_tx_hex()`.


## Future work
* [BIP-q127](https://github.com/bitcoin/bips/blob/master/bip-0127.mediawiki#proof-format) proposes a standard format for constructing proof-of-reserves transactions. It would be nice to revise the tx to follow this standard.
* In addition to proof of reserves, it would be interesting implement a testament of liabilities for outstanding notes. Due to the privacy of blind signatures, We don't believe it is possible to prove that the federation has not issued more eCash tokens than there are liabilities. However, we could increase transparency and trust in the fedimint with a merkle sum tree data structure allowing all eCash note holders to verify the presence of their notes, and the absence of any negative sums in the neighboring leaves of the merkle tree. This is not a 'proof' because it isn't independently verifiable like a signed-but-invalid PSBT that spends on-chain funds.
