use crate::{
    instruction::Instruction,
    processor::{
        process_account, process_create_account, process_log, process_ping, process_transfer,
    },
    ProgramResult, MAX_ACCOUNTS,
};
use jiminy_cpi::account::AccountHandle;
use jiminy_entrypoint::{account::Abr, entrypoint};

entrypoint!(process_instruction, MAX_ACCOUNTS);

#[inline(always)]
pub fn process_instruction(
    abr: &mut Abr,
    accounts: &[AccountHandle],
    instruction_data: &[u8],
    _program_id: &[u8; 32],
) -> ProgramResult {
    let instruction = Instruction::unpack(instruction_data)?;

    match instruction {
        Instruction::Ping => process_ping(),
        Instruction::Log => process_log(),
        Instruction::Account { expected } => process_account(accounts, expected),
        Instruction::CreateAccount => process_create_account(abr, accounts),
        Instruction::Transfer => process_transfer(abr, accounts),
    }
}
