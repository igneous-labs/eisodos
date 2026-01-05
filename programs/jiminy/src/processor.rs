use crate::{Cpi, ProgramResult};
use jiminy_cpi::{
    account::{Abr, AccountHandle},
    program_error::{INVALID_ARGUMENT, NOT_ENOUGH_ACCOUNT_KEYS},
};
use jiminy_log::sol_log;
use sanctum_system_jiminy::sanctum_system_core::instructions::{
    create_account::{
        CreateAccountIxAccs, CreateAccountIxAccsDestr, CreateAccountIxArgs, CreateAccountIxData,
    },
    transfer::{TransferIxAccs, TransferIxAccsDestr, TransferIxData},
};

#[inline(always)]
pub fn process_ping() -> ProgramResult {
    Ok(())
}

#[inline(always)]
pub fn process_log() -> ProgramResult {
    const MSG: &str = "Instruction: Log";
    sol_log(MSG);
    Ok(())
}

#[inline(always)]
pub fn process_account(accounts: &[AccountHandle], expected: u64) -> ProgramResult {
    if accounts.len() == expected as usize {
        Ok(())
    } else {
        Err(INVALID_ARGUMENT.into())
    }
}

#[inline(always)]
pub fn process_create_account(abr: &mut Abr, accounts: &[AccountHandle]) -> ProgramResult {
    let [funding, new, _remaining @ ..] = accounts else {
        return Err(NOT_ENOUGH_ACCOUNT_KEYS.into());
    };
    Cpi::new().invoke_fwd(
        abr,
        &sanctum_system_jiminy::sanctum_system_core::ID,
        CreateAccountIxData::new(&CreateAccountIxArgs {
            lamports: 500_000_000,
            space: 10,
            owner: &crate::ID,
        })
        .as_buf(),
        CreateAccountIxAccs::from_destr(CreateAccountIxAccsDestr {
            funding: *funding,
            new: *new,
        })
        .0,
    )
}

#[inline(always)]
pub fn process_transfer(abr: &mut Abr, accounts: &[AccountHandle]) -> ProgramResult {
    let [from, to, _remaining @ ..] = accounts else {
        return Err(NOT_ENOUGH_ACCOUNT_KEYS.into());
    };
    Cpi::new().invoke_fwd(
        abr,
        &sanctum_system_jiminy::sanctum_system_core::ID,
        TransferIxData::new(1_000_000_000).as_buf(),
        TransferIxAccs::from_destr(TransferIxAccsDestr {
            from: *from,
            to: *to,
        })
        .0,
    )
}
