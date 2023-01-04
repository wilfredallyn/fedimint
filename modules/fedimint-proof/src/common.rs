use std::io;

use fedimint_api::core::{ConsensusItem, Input, Output, OutputOutcome, PluginDecode};
use fedimint_api::encoding::{Decodable, DecodeError};
use fedimint_api::module::registry::ModuleDecoderRegistry;

use crate::{ProofInput, ProofOutput, ProofOutputConfirmation, ProofOutputOutcome};

#[derive(Debug, Default, Clone)]
pub struct ProofModuleDecoder;

impl PluginDecode for ProofModuleDecoder {
    fn decode_input(mut d: &mut dyn io::Read) -> Result<Input, DecodeError> {
        Ok(Input::from(ProofInput::consensus_decode(
            &mut d,
            &ModuleDecoderRegistry::default(),
        )?))
    }
    fn decode_output(mut d: &mut dyn io::Read) -> Result<Output, DecodeError> {
        Ok(Output::from(ProofOutput::consensus_decode(
            &mut d,
            &ModuleDecoderRegistry::default(),
        )?))
    }

    fn decode_output_outcome(mut d: &mut dyn io::Read) -> Result<OutputOutcome, DecodeError> {
        Ok(OutputOutcome::from(ProofOutputOutcome::consensus_decode(
            &mut d,
            &ModuleDecoderRegistry::default(),
        )?))
    }

    fn decode_consensus_item(
        mut r: &mut dyn io::Read,
    ) -> Result<fedimint_api::core::ConsensusItem, DecodeError> {
        Ok(ConsensusItem::from(
            ProofOutputConfirmation::consensus_decode(&mut r, &ModuleDecoderRegistry::default())?,
        ))
    }
}
