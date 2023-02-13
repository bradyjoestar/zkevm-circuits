use eth_types::Field;
use halo2_proofs::plonk::Error;

use crate::{
    evm_circuit::{
        param::{N_BYTES_GAS, N_BYTES_MEMORY_ADDRESS, N_BYTES_MEMORY_WORD_SIZE},
        step::ExecutionState,
        util::{
            common_gadget::TransferGadget,
            constraint_builder::{ConstraintBuilder, ReversionInfo},
            math_gadget::ConstantDivisionGadget,
            memory_gadget::{MemoryAddressGadget, MemoryExpansionGadget},
            CachedRegion, Cell, Word,
        },
    },
    witness::{Block, Call, ExecStep, Transaction},
};

use super::ExecutionGadget;

#[derive(Clone, Debug)]
pub(crate) struct CreateGadget<F, const IS_CREATE2: bool, const S: ExecutionState> {
    opcode: Cell<F>,
    value: Word<F>,
    tx_id: Cell<F>,
    reversion_info: ReversionInfo<F>,
    was_warm: Cell<F>,
    depth: Cell<F>,
    callee_reversion_info: ReversionInfo<F>,
    callee_is_success: Cell<F>,
    transfer: TransferGadget<F>,
    initialization_code: MemoryAddressGadget<F>,
    initialization_code_word_size: ConstantDivisionGadget<F, N_BYTES_MEMORY_ADDRESS>,
    memory_expansion: MemoryExpansionGadget<F, 1, N_BYTES_MEMORY_WORD_SIZE>,
    gas_left: ConstantDivisionGadget<F, N_BYTES_GAS>,
    keccak_output: Word<F>,
}

impl<F: Field, const IS_CREATE2: bool, const S: ExecutionState> ExecutionGadget<F>
    for CreateGadget<F, IS_CREATE2, S>
{
    const NAME: &'static str = "CREATE";

    const EXECUTION_STATE: ExecutionState = S;

    fn configure(cb: &mut ConstraintBuilder<F>) -> Self {
        unimplemented!()
    }

    fn assign_exec_step(
        &self,
        region: &mut CachedRegion<'_, '_, F>,
        offset: usize,
        block: &Block<F>,
        transaction: &Transaction,
        call: &Call,
        step: &ExecStep,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}
