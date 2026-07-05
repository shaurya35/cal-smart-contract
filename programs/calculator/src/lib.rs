use anchor_lang::prelude::*;

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

declare_id!("6de6tHtmgwWUTJw3ghTeDbgYvMMbvUtdfYyCrF3v8bLq");

#[program]
pub mod anchor_calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.new_account.data = 1;

        let rect = Rect {
            width: 10,
            height: 20,
        };
        msg!("Rect area: {}", rect.area());

        Ok(())
    }

    pub fn double(ctx: Context<Double>) -> Result<()> {
        ctx.accounts.account.data = ctx
            .accounts
            .account
            .data
            .checked_mul(2)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        Ok(())
    }

    pub fn halve(ctx: Context<Halve>) -> Result<()> {
        ctx.accounts.account.data /= 2;
        Ok(())
    }

    pub fn add(ctx: Context<Add>, amount: u32) -> Result<()> {
        ctx.accounts.account.data = ctx
            .accounts
            .account
            .data
            .checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        Ok(())
    }
    
    pub fn sub(ctx: Context<Sub>, amount: u32) -> Result<()> {
        ctx.accounts.account.data = ctx
            .accounts
            .account
            .data
            .checked_sub(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct NewAccount {
    pub data: u32,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + NewAccount::INIT_SPACE)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Double<'info> {
    #[account(mut)]
    pub account: Account<'info, NewAccount>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Halve<'info> {
    #[account(mut)]
    pub account: Account<'info, NewAccount>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub account: Account<'info, NewAccount>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Sub<'info> {
    #[account(mut)]
    pub account: Account<'info, NewAccount>,
    pub signer: Signer<'info>,
}
