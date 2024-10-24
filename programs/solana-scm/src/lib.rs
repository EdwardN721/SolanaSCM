use anchor_lang::prelude::*;
use std::collections::HashMap;


declare_id!("A5i8uPKdCycDG3nbGCCAUiLzHEc4ddpfeYGQhPEWuaTJ");

#[program]
pub mod registry_project {
    use super::*;

    pub fn create_registry(ctx: Context<CreateRegistry>, name: String) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.name = name;  // Asignar el nombre
        registry.device_count = 0;
        Ok(())
    }

    pub fn add_device(
        ctx: Context<AddDevice>,
        name: String,
        description: String,
    ) -> Result<()> {
        let device = &mut ctx.accounts.device;
        device.name = name;
        device.description = description;
        let registry = &mut ctx.accounts.registry;
        registry.device_count += 1;
        registry.device_ids.push(device.key());
        Ok(())
    }
}

#[derive(Debug)]
#[account]
pub struct Registry {
    pub name: String,       
    pub device_count: u64,
    pub device_ids: Vec<Pubkey>,
}

#[derive(Debug)]
#[account]
pub struct Device {
    pub name: String,
    pub description: String,
}

#[derive(Accounts)]
pub struct CreateRegistry<'info> {
    #[account(init, payer = user, space = 8 + 4 + 32 + 8 + 32 * 100)] // espacio para el nombre
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddDevice<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32)]
    pub device: Account<'info, Device>,
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
