use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod writeup {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn add_note(ctx:Context<Approved>, ipfs_url: string) -> Result<()> {
        ctx.accounts.note.ipfs_url = ipfs_url;
        ctx.accounts.note.owner = ctx.accounts.owner.key();
        Ok(())
    }

    pub fn get_note(ctx:Context<Approved>) -> Result<()> {
        todo!("Implement this")
    }
}

#[account]
pub struct Note {
    ipfs_url: string,
    owner: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Approved<'info> {
    pub owner: Signer<'info>;
    pub note: Account<'info, Note>
}
