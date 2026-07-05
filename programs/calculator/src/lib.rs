use anchor_lang::prelude::*;

declare_id!("3XG3SjJFeobck2JCr3zgt7RiHJ2dGtqSsFqCxN5f4N4F");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_value: u32) -> Result<()> {
        ctx.accounts.calculator.value = initial_value;
        ctx.accounts.calculator.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn double(ctx: Context<Modify>) -> Result<()> {
        ctx.accounts.calculator.value = ctx
            .accounts
            .calculator
            .value
            .checked_mul(2)
            .ok_or(CalculatorError::Overflow)?;
        Ok(())
    }

    pub fn halve(ctx: Context<Modify>) -> Result<()> {
        ctx.accounts.calculator.value = ctx
            .accounts
            .calculator
            .value
            .checked_div(2)
            .ok_or(CalculatorError::DivideByZero)?;
        Ok(())
    }

    pub fn add(ctx: Context<Modify>, amount: u32) -> Result<()> {
        ctx.accounts.calculator.value = ctx
            .accounts
            .calculator
            .value
            .checked_add(amount)
            .ok_or(CalculatorError::Overflow)?;
        Ok(())
    }

    pub fn sub(ctx: Context<Modify>, amount: u32) -> Result<()> {
        ctx.accounts.calculator.value = ctx
            .accounts
            .calculator
            .value
            .checked_sub(amount)
            .ok_or(CalculatorError::Underflow)?;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct CalculatorState {
    pub value: u32,
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + CalculatorState::INIT_SPACE)]
    pub calculator: Account<'info, CalculatorState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(mut, has_one = authority)]
    pub calculator: Account<'info, CalculatorState>,
    pub authority: Signer<'info>,
}

#[error_code]
pub enum CalculatorError {
    #[msg("The calculator operation overflowed")]
    Overflow,
    #[msg("The calculator operation underflowed")]
    Underflow,
    #[msg("Cannot divide by zero")]
    DivideByZero,
}
