//! Pre-requisites:
//! - All validators except laine removed from SPL pool
//! - All SOL is staked to laine (no SOL except rent-exempt reserves in SPL pool reserves)
//! - Validator List and Pool has enough SOL rent to pay for new accounts' rent (definitely, since our validator list is huge)

use sanctum_misc_utils::log_and_return_wrong_acc_err;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey, stake, system_program, sysvar,
};

solana_program::entrypoint!(process_instruction);

pub mod scnsol_metadata_update_auth {
    // TODO: Set this to actual metadata update authority pubkey
    #[cfg(not(feature = "testing"))]
    sanctum_macros::declare_program_keys!("TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111", []);

    #[cfg(feature = "testing")]
    sanctum_macros::declare_program_keys!("CK9cEJT7K7oRrMCcEbBQRGqHLGpxKXWnKvW7nHSDMHD1", []);
}

pub mod socean_spl_pool {
    sanctum_macros::declare_program_keys!("5oc4nmbNTda9fx8Tw57ShLD132aqDK65vuHH4RU1K4LZ", []);
}

pub mod socean_spl_validator_list {
    sanctum_macros::declare_program_keys!("8pTa29ovYHxjQgX7gjxGi395GAo8DSXCRTKJZvwMc6MR", []);
}

pub mod socean_spl_reserves {
    sanctum_macros::declare_program_keys!("4sDXGroVt7ba45rzXtNto97QjG1rHm8Py3v56Mgg16Nc", []);
}

pub mod scnsol_mint {
    sanctum_macros::declare_program_keys!("5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm", []);
}

pub mod lainesol_mint {
    sanctum_macros::declare_program_keys!("LAinEtNLgpmCP9Rvsf5Hn8W6EhNiKLZQti1xfWMLy6X", []);
}

pub mod socean_program {
    // b"G_\x1d..." is socean stake pool
    // python: base58.b58decode("5oc4nmbNTda9fx8Tw57ShLD132aqDK65vuHH4RU1K4LZ")
    sanctum_macros::declare_program_keys!(
        "5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx",
        [(
            "withdraw_auth",
            b"G_\x1d\x08\xedE\xbcwV-<\xaf\xd1=`\xcc\xdcLo\xa2\x81\x90\xe1Af\xf0\x17C\x01\x19(\n",
            b"withdraw"
        ),]
    );
}

pub mod ata_program {
    // b"\x8d\xd8r\xa3\xb7\x15\xde\xd1\xd4`4?\xf5\xbaJ(\x10.9\x02G%\x89_\xeb\xc7\xa9\xc7\x97!3\xd3" is pool state
    // b"Q=\x96\xf8\xb1\xfc\x8eHK\xf4'h\x84\x04O\xe4i\x8c\x91\xc8\\V\x8d\x8f\x15ti\xf4\x9f\xa7\xee\x11" is protocol fee
    // b"\x06\xdd\xf6\xe1\xd7e\xa1\x93\xd9\xcb\xe1F\xce\xeby\xac\x1c\xb4\x85\xed_[7\x91:\x8c\xf5\x85~\xff\x00\xa9" is token program
    // b"\x04\xe9\x06\xb5\x1e\x90\x97/\xd4\xcdi\x94:\x88a\xda\xc5y?\xa7<\xf7{,\xb3\xd7c#_\x83\x07x" is laineSOL mint
    sanctum_macros::declare_program_keys!(
        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        [
            (
                "lainesol_reserves",
                b"\x8d\xd8r\xa3\xb7\x15\xde\xd1\xd4`4?\xf5\xbaJ(\x10.9\x02G%\x89_\xeb\xc7\xa9\xc7\x97!3\xd3",
                b"\x06\xdd\xf6\xe1\xd7e\xa1\x93\xd9\xcb\xe1F\xce\xeby\xac\x1c\xb4\x85\xed_[7\x91:\x8c\xf5\x85~\xff\x00\xa9",
                b"\x04\xe9\x06\xb5\x1e\x90\x97/\xd4\xcdi\x94:\x88a\xda\xc5y?\xa7<\xf7{,\xb3\xd7c#_\x83\x07x"
            ),
            (
                "lainesol_protocol_fee_accum",
                b"Q=\x96\xf8\xb1\xfc\x8eHK\xf4'h\x84\x04O\xe4i\x8c\x91\xc8\\V\x8d\x8f\x15ti\xf4\x9f\xa7\xee\x11",
                b"\x06\xdd\xf6\xe1\xd7e\xa1\x93\xd9\xcb\xe1F\xce\xeby\xac\x1c\xb4\x85\xed_[7\x91:\x8c\xf5\x85~\xff\x00\xa9",
                b"\x04\xe9\x06\xb5\x1e\x90\x97/\xd4\xcdi\x94:\x88a\xda\xc5y?\xa7<\xf7{,\xb3\xd7c#_\x83\x07x"
            )]
    );
}

pub mod token_metadata_program {
    // b"\x0bpe\xb1\xe3\xd1|E8\x9dR\x7fk\x04\xc3\xcdX\xb8ls\x1a\xa0\xfd\xb5I\xb6\xd1\xbc\x03\xf8)F" is metadata program
    // b"GW\x89\x9f\xb8\xbe\xdb\xa2\x87x\xaa\xcdg\xe5h\xe74p\xcc\xe9\x0b\xcdS+l\xb6\x18)v(\x82N" is scnsol mint
    sanctum_macros::declare_program_keys!(
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
        [(
            "metadata_pda",
            b"metadata",
            b"\x0bpe\xb1\xe3\xd1|E8\x9dR\x7fk\x04\xc3\xcdX\xb8ls\x1a\xa0\xfd\xb5I\xb6\xd1\xbc\x03\xf8)F",
            b"GW\x89\x9f\xb8\xbe\xdb\xa2\x87x\xaa\xcdg\xe5h\xe74p\xcc\xe9\x0b\xcdS+l\xb6\x18)v(\x82N"
        )]
    );
}

pub const MIGRATE_ACCOUNTS_LEN: usize = 18;

pub struct MigrateKeys {
    // SPL
    pub spl_reserves: Pubkey,
    pub spl_pool: Pubkey,
    pub spl_validator_list: Pubkey,
    // Metaplex
    pub metadata_pda: Pubkey,
    pub metadata_update_authority: Pubkey,
    // New S Accounts
    pub s_pool_state: Pubkey,
    pub s_lst_state_list: Pubkey,
    pub s_protocol_fee: Pubkey,
    pub lainesol_reserves: Pubkey,
    pub lainesol_protocol_fee_accum: Pubkey,
    // Common
    pub scnsol_mint: Pubkey,
    pub socean_withdraw_auth: Pubkey, // also serves as scnsol mint auth
    pub lainesol_mint: Pubkey,
    // Programs + Sysvar
    pub token_metadata_program: Pubkey,
    pub stake_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}

pub const MIGRATE_KEYS: MigrateKeys = MigrateKeys {
    spl_reserves: socean_spl_reserves::ID,
    spl_pool: socean_spl_pool::ID,
    spl_validator_list: socean_spl_validator_list::ID,
    metadata_pda: token_metadata_program::METADATA_PDA_ID,
    metadata_update_authority: scnsol_metadata_update_auth::ID,
    s_pool_state: s_controller_lib::program::POOL_STATE_ID,
    s_lst_state_list: s_controller_lib::program::LST_STATE_LIST_ID,
    s_protocol_fee: s_controller_lib::program::PROTOCOL_FEE_ID,
    lainesol_reserves: ata_program::LAINESOL_RESERVES_ID,
    lainesol_protocol_fee_accum: ata_program::LAINESOL_PROTOCOL_FEE_ACCUM_ID,
    scnsol_mint: scnsol_mint::ID,
    socean_withdraw_auth: socean_program::WITHDRAW_AUTH_ID,
    lainesol_mint: lainesol_mint::ID,
    token_metadata_program: token_metadata_program::ID,
    stake_program: stake::program::ID,
    token_program: spl_token::ID,
    system_program: system_program::ID,
    rent: sysvar::rent::ID,
};

pub struct MigrateAccounts<'me, 'info> {
    pub spl_reserves: &'me AccountInfo<'info>,
    pub spl_pool: &'me AccountInfo<'info>,
    pub spl_validator_list: &'me AccountInfo<'info>,
    pub metadata_pda: &'me AccountInfo<'info>,
    pub metadata_update_authority: &'me AccountInfo<'info>,
    pub s_pool_state: &'me AccountInfo<'info>,
    pub s_lst_state_list: &'me AccountInfo<'info>,
    pub s_protocol_fee: &'me AccountInfo<'info>,
    pub lainesol_reserves: &'me AccountInfo<'info>,
    pub lainesol_protocol_fee_accum: &'me AccountInfo<'info>,
    pub scnsol_mint: &'me AccountInfo<'info>,
    pub socean_withdraw_auth: &'me AccountInfo<'info>,
    pub lainesol_mint: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}

impl MigrateAccounts<'_, '_> {
    pub fn verify_account_keys(&self) -> Result<(), ProgramError> {
        let Self {
            spl_reserves,
            spl_pool,
            spl_validator_list,
            metadata_pda,
            metadata_update_authority,
            s_pool_state,
            s_lst_state_list,
            s_protocol_fee,
            lainesol_reserves,
            lainesol_protocol_fee_accum,
            scnsol_mint,
            socean_withdraw_auth,
            lainesol_mint,
            token_metadata_program,
            stake_program,
            token_program,
            system_program,
            rent,
        } = self;
        for (actual, expected) in &[
            (*spl_reserves.key, MIGRATE_KEYS.spl_reserves),
            (*spl_pool.key, MIGRATE_KEYS.spl_pool),
            (*spl_validator_list.key, MIGRATE_KEYS.spl_validator_list),
            (*metadata_pda.key, MIGRATE_KEYS.metadata_pda),
            (
                *metadata_update_authority.key,
                MIGRATE_KEYS.metadata_update_authority,
            ),
            (*s_pool_state.key, MIGRATE_KEYS.s_pool_state),
            (*s_lst_state_list.key, MIGRATE_KEYS.s_lst_state_list),
            (*s_protocol_fee.key, MIGRATE_KEYS.s_protocol_fee),
            (*lainesol_reserves.key, MIGRATE_KEYS.lainesol_reserves),
            (
                *lainesol_protocol_fee_accum.key,
                MIGRATE_KEYS.lainesol_protocol_fee_accum,
            ),
            (*scnsol_mint.key, MIGRATE_KEYS.scnsol_mint),
            (*socean_withdraw_auth.key, MIGRATE_KEYS.socean_withdraw_auth),
            (*lainesol_mint.key, MIGRATE_KEYS.lainesol_mint),
            (
                *token_metadata_program.key,
                MIGRATE_KEYS.token_metadata_program,
            ),
            (*stake_program.key, MIGRATE_KEYS.stake_program),
            (*token_program.key, MIGRATE_KEYS.token_program),
            (*system_program.key, MIGRATE_KEYS.system_program),
            (*rent.key, MIGRATE_KEYS.rent),
        ] {
            if actual != expected {
                return Err(log_and_return_wrong_acc_err((*actual, *expected)));
            }
        }
        Ok(())
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_ACCOUNTS_LEN]>
    for MigrateAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_ACCOUNTS_LEN]) -> Self {
        Self {
            spl_reserves: &arr[0],
            spl_pool: &arr[1],
            spl_validator_list: &arr[2],
            metadata_pda: &arr[3],
            metadata_update_authority: &arr[4],
            s_pool_state: &arr[5],
            s_lst_state_list: &arr[6],
            s_protocol_fee: &arr[7],
            lainesol_reserves: &arr[8],
            lainesol_protocol_fee_accum: &arr[9],
            scnsol_mint: &arr[10],
            socean_withdraw_auth: &arr[11],
            lainesol_mint: &arr[12],
            token_metadata_program: &arr[13],
            stake_program: &arr[14],
            token_program: &arr[15],
            system_program: &arr[16],
            rent: &arr[17],
        }
    }
}

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    todo!()
}
