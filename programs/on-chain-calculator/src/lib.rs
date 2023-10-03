// The task is to add code everywhere where you find // TODO.or todo!()
// You have to finish the implementation of methods of on-chain Calculator.
// Once you complete the TODOs, make sure that you delete all `todo!()` macros and
// you can try to run the tests using `anchor test` command and start debugging ;-)

use anchor_lang::prelude::*;

declare_id!("8kCVvW4147A2CrhW2Dr4ety2PJ2ds7oUavRNCVtBeew6");

#[program]
pub mod on_chain_calculator {
    use super::*;

    // ------------------------------------------------------------------------------------------------
    // Functions
    //
    /// Initialize the Calculator with the corresponding operands and the corresponding update authority.
    /// Update authority is important to ensure that only privileged persons can modify calculator data.
    pub fn init_calculator(ctx: Context<InitializeCalculator>, x: i64, y: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        let update_authority = ctx.accounts.update_authority.key();

        calculator.x = x;
        calculator.y = y;
        calculator.update_authority = update_authority;

        Ok(())
    }

    /// Update Operand X
    pub fn update_x(ctx: Context<ChangeInternalState>, new_x: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.x = new_x;
        Ok(())
    }
    /// Update Operand Y
    pub fn update_y(ctx: Context<ChangeInternalState>, new_y: i64) -> Result<()> {
        // TODO
        todo!()
    }
    // TODO: Implement the `update_authority` function in the same manner as the update functions above.
    // To pass tests, use the function name as follows: `update_authority`

    /// This function reads data from the Calculator Account and
    /// performs an addition operation. The result, as well as the operands,
    /// are then emitted in the logs. You can subscribe to the logs off-chain
    /// to check the correctness of the operands and the result.
    /// Emitting into the logs is only important for testing purposes;
    /// DO NOT ALTER IT!
    pub fn addition(ctx: Context<Compute>) -> Result<()> {
        let calculator = &ctx.accounts.calculator;

        // TODO
        let operand_x = todo!();
        // TODO
        let operand_y = todo!();

        // TODO
        let result = todo!();

        // The code below will emit operands with the result into logs.
        // We then subscribe to the logs inside tests to verify if the Calculator works correctly.
        // DO NOT MODIFY
        emit!(CalculatorEvent {
            x: operand_x,
            y: operand_y,
            result
        });
        Ok(())
    }
    // TODO: Implement other Calculator functions in the same manner as the addition function above.
    // To pass tests, use function names as follows:
    // - `subtraction`
    // - `multiplication`
    // - `division`

    // ------------------------------------------------------------------------------------------------
}

// ------------------------------------------------------------------------------------------------
// Contexts
//
// In order to specify which accounts (read or write) the program expects, we use Contexts. Each Context is
// a predefined struct that can be used within function declarations. This ensures that, for example,
// the `init_calculator` function expects accounts as defined inside the InitializeCalculator Context.
// Furthermore, the Anchor framework allows us to define additional information about the accounts inside the Context.
// For example, if we want to modify account data, we mark it as (mut). If we want to initialize the account,
// we mark it with `init` (using `init`, the account is automatically mutable), and much more.
#[derive(Accounts)]
pub struct InitializeCalculator<'info> {
    #[account(mut)] // mark account as mutable
    pub update_authority: Signer<'info>,
    #[account(
        init, // initialize account
        payer = update_authority, // who will pay for account initialization
        space = 8 + 8 + 8 + 32 // how much space we need for data
    )]
    pub calculator: Account<'info, Calculator>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeInternalState<'info> {
    pub update_authority: Signer<'info>,
    #[account(mut,
        has_one = update_authority @ CalculatorError::WrongPrivileges
    )]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Compute<'info> {
    pub calculator: Account<'info, Calculator>,
}
// ------------------------------------------------------------------------------------------------
// Stored on-chain Data
//
#[account]
pub struct Calculator {
    pub x: i64,
    pub y: i64,
    pub update_authority: Pubkey,
}

impl Calculator {
    pub fn addition(&self) -> Option<i64> {
        self.x.checked_add(self.y)
    }
    pub fn subtraction(&self) -> Option<i64> {
        self.x.checked_sub(self.y)
    }
    pub fn multiplication(&self) -> Option<i64> {
        self.x.checked_mul(self.y)
    }
    pub fn division(&self) -> Option<i64> {
        self.x.checked_div(self.y)
    }
}
// ------------------------------------------------------------------------------------------------
// Error Codes
//
#[error_code]
pub enum CalculatorError {
    #[msg("You do not have sufficient privileges to updated the Calculator")]
    WrongPrivileges,
}
// ------------------------------------------------------------------------------------------------
// Predefined structure for emitting into logs
//
#[event]
pub struct CalculatorEvent {
    pub x: i64,
    pub y: i64,
    pub result: Option<i64>,
}
// ------------------------------------------------------------------------------------------------
