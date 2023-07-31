use std::sync::Arc;

use fedimint_core::core::{DynInput, DynOutput, IntoDynInstance, KeyPair, ModuleInstanceId};
use fedimint_core::transaction::Transaction;
use fedimint_core::Amount;
use itertools::multiunzip;
use rand::{CryptoRng, RngCore};
use secp256k1_zkp::Secp256k1;

use crate::module::StateGenerator;
use crate::sm::DynState;
use crate::DynGlobalClientContext;

use tracing::info;

#[derive(Clone)]
pub struct ClientInput<I = DynInput, S = DynState<DynGlobalClientContext>> {
    pub input: I,
    pub keys: Vec<KeyPair>,
    pub state_machines: StateGenerator<S>,
}

impl<I, S> IntoDynInstance for ClientInput<I, S>
where
    I: IntoDynInstance<DynType = DynInput> + 'static,
    S: IntoDynInstance<DynType = DynState<DynGlobalClientContext>> + 'static,
{
    type DynType = ClientInput;

    fn into_dyn(self, module_instance_id: ModuleInstanceId) -> ClientInput {
        ClientInput {
            input: self.input.into_dyn(module_instance_id),
            keys: self.keys,
            state_machines: state_gen_to_dyn(self.state_machines, module_instance_id),
        }
    }
}

#[derive(Clone)]
pub struct ClientOutput<O = DynOutput, S = DynState<DynGlobalClientContext>> {
    pub output: O,
    pub state_machines: StateGenerator<S>,
}

impl<O, S> IntoDynInstance for ClientOutput<O, S>
where
    O: IntoDynInstance<DynType = DynOutput> + 'static,
    S: IntoDynInstance<DynType = DynState<DynGlobalClientContext>> + 'static,
{
    type DynType = ClientOutput;

    fn into_dyn(self, module_instance_id: ModuleInstanceId) -> ClientOutput {
        ClientOutput {
            output: self.output.into_dyn(module_instance_id),
            state_machines: state_gen_to_dyn(self.state_machines, module_instance_id),
        }
    }
}

#[derive(Default, Clone)]
pub struct TransactionBuilder {
    pub(crate) inputs: Vec<ClientInput>,
    pub(crate) outputs: Vec<ClientOutput>,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_input(mut self, input: ClientInput) -> Self {
        self.inputs.push(input);
        self
    }

    pub fn with_output(mut self, output: ClientOutput) -> Self {
        self.outputs.push(output);
        self
    }

    pub fn build<C, R: RngCore + CryptoRng>(
        self,
        secp_ctx: &Secp256k1<C>,
        mut rng: R,
    ) -> (Transaction, Vec<DynState<DynGlobalClientContext>>)
    where
        C: secp256k1_zkp::Signing + secp256k1_zkp::Verification,
    {
        let (inputs, input_keys, input_states): (Vec<_>, Vec<_>, Vec<_>) = multiunzip(
            self.inputs
                .into_iter()
                .map(|input| (input.input, input.keys, input.state_machines)),
        );
        // info!("inputs {:#?}", inputs); // "{:?} {:?}"
        // info!("input_keys {:#?}", input_keys); // "{:?} {:?}"

        // info!("input_states {:#?}", input_states); // "{:?} {:?}"
        //    error[E0277]: `dyn Fn(TransactionId, u64) -> Vec<DynState<DynGlobalClientContext>> + std::marker::Send + Sync` doesn't implement `std::fmt::Debug`
        //    --> fedimint-client/src/transaction/builder.rs:96:37
        //     |
        //  96 |         info!("input_states {:#?}", input_states); // "{:?} {:?}"
        //     |                                     ^^^^^^^^^^^^ `dyn Fn(TransactionId, u64) -> Vec<DynState<DynGlobalClientContext>> + std::marker::Send + Sync` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
        //     |
        //     = help: the trait `std::fmt::Debug` is not implemented for `dyn Fn(TransactionId, u64) -> Vec<DynState<DynGlobalClientContext>> + std::marker::Send + Sync`
        //     = note: this error originates in the macro `format_args` which comes from the expansion of the macro `info` (in Nightly builds, run with -Z macro-backtrace for more info)

        let (outputs, output_states): (Vec<_>, Vec<_>) = self
            .outputs
            .into_iter()
            .map(|output| (output.output, output.state_machines))
            .unzip();
        // info!("outputs {:#?}", outputs); // "{:?} {:?}"
        // info!("output_states {:#?}", output_states); // "{:?} {:?}"

        let txid = Transaction::tx_hash_from_parts(&inputs, &outputs);

        let signature = if !input_keys.is_empty() {
            let keys = input_keys.into_iter().flatten().collect::<Vec<_>>();

            let signature =
                fedimint_core::transaction::agg_sign(&keys, txid.as_hash(), secp_ctx, &mut rng);
            Some(signature)
        } else {
            None
        };

        let transaction = Transaction {
            inputs,
            outputs,
            signature,
        };

        let states = input_states
            .into_iter()
            .enumerate()
            .chain(output_states.into_iter().enumerate())
            .flat_map(|(idx, state_gen)| state_gen(txid, idx as u64))
            .collect::<Vec<_>>();

        (transaction, states)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TransactionBuilderBalance {
    Underfunded(Amount),
    Balanced,
    Overfunded(Amount),
}

fn state_gen_to_dyn<S>(
    state_gen: StateGenerator<S>,
    module_instance: ModuleInstanceId,
) -> StateGenerator<DynState<DynGlobalClientContext>>
where
    S: IntoDynInstance<DynType = DynState<DynGlobalClientContext>> + 'static,
{
    Arc::new(move |txid, index| {
        let states = state_gen(txid, index);
        states
            .into_iter()
            .map(|state| state.into_dyn(module_instance))
            .collect()
    })
}
