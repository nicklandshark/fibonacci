use anchor_lang::prelude::*;
use bonsol_interface::bonsol_schema::root_as_execution_request_v1;

declare_id!("39v8Th2UHZmynSdzCSfrC1sJWrTFSfGv9A18bmq2SX6V");

pub const FIBONACCI_IMAGE_ID: &str = "a7b2cd51504760f95ea0688cc83677530d7a21b4e09251946bff6ce24c43fcc6";

#[error_code]
pub enum Error {
    #[msg("Execution request data is too short")]
    ExecutionRequestReused,
    #[msg("Invalid execution request")]
    InvalidExecutionRequest,
    #[msg("Invalid callback image id")]
    InvalidCallbackImageId,
}

#[program]
pub mod callback {
    use super::*;

    pub fn callback_handler(ctx: Context<CallbackHandler>) -> Result<()> {
        // The execution request data is stored on the execution account
        let request_data = ctx.accounts.execution_account.try_borrow_data()?;
        if request_data.len() < 2 { 
            return err!(Error::ExecutionRequestReused);
        }

        // Deserialize the execution request which is formatted as a flatbuffer
        let request = root_as_execution_request_v1(&request_data)
            .map_err(|_| Error::InvalidExecutionRequest)?;
        if request.image_id() != Some(FIBONACCI_IMAGE_ID) {
            return err!(Error::InvalidCallbackImageId);
        }

        msg!("Received Callback");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CallbackHandler<'info> {
    pub execution_account: Signer<'info>,
}
