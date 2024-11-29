use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, instruction::Instruction,
    msg, program::invoke, program_error::ProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let bonding_curve_acc = &accounts[3];
    let bonding_curve_data = bonding_curve_acc.try_borrow_data()?;
    let token_balance = u64::from_le_bytes(
        bonding_curve_data[8..16]
            .try_into()
            .map_err(|_| ProgramError::Custom(1))?,
    ) as u128;
    let sol_balance = u64::from_le_bytes(
        bonding_curve_data[16..24]
            .try_into()
            .map_err(|_| ProgramError::Custom(2))?,
    ) as u128;
    drop(bonding_curve_data);
    let sol_amount_in = u64::from_le_bytes(
        instruction_data[0..8]
            .try_into()
            .map_err(|_| ProgramError::Custom(3))?,
    );
    let sol_amount_in_after_fee = (sol_amount_in / 101 * 100) as u128;
    let token_amount_out = token_balance * sol_amount_in_after_fee / (sol_balance + sol_amount_in_after_fee);
    
    let mut data = Vec::with_capacity(24);
    data.extend_from_slice(&[
        0x66u8, 0x06u8, 0x3du8, 0x12u8, 0x01u8, 0xdau8, 0xebu8, 0xeau8,
    ]);
    data.extend_from_slice(&(token_amount_out as u64).to_le_bytes());
    data.extend_from_slice(&sol_amount_in.to_le_bytes());

    let buy_ix = Instruction {
        program_id: *accounts[11].key,
        accounts: accounts
            .iter()
            .map(|account| {
                if account.is_writable {
                    solana_program::instruction::AccountMeta::new(*account.key, account.is_signer)
                } else {
                    solana_program::instruction::AccountMeta::new_readonly(
                        *account.key,
                        account.is_signer,
                    )
                }
            })
            .collect(),
        data,
    };
    invoke(
        &buy_ix,
        accounts
            .iter()
            .map(|account| account.clone())
            .collect::<Vec<AccountInfo>>()
            .as_slice(),
    )?;

    msg!("From UK with love, yours truly, the <a href=\"https://web3engineering.co.uk\">W3E</a> team");
    Ok(())
}
