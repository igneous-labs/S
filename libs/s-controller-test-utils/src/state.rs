use async_trait::async_trait;
use s_controller_interface::PoolState;
use s_controller_lib::{
    initial_authority, program::POOL_STATE_ID, try_pool_state_mut, DEFAULT_PRICING_PROGRAM,
    POOL_STATE_SIZE,
};
use sanctum_solana_test_utils::{
    est_rent_exempt_lamports,
    token::{tokenkeg::TokenkegProgramTest, MockMintArgs},
    ExtendedBanksClient, ExtendedProgramTest, IntoAccount,
};
use solana_program::pubkey::Pubkey;
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::account::Account;

#[derive(Clone, Copy, Debug, Default)]
pub struct MockProtocolFeeBps {
    pub trading: u16,
    pub lp: u16,
}

pub const DEFAULT_POOL_STATE: PoolState = PoolState {
    total_sol_value: 0,
    trading_protocol_fee_bps: 0,
    lp_protocol_fee_bps: 0,
    version: 0,
    is_disabled: 0,
    is_rebalancing: 0,
    padding: [0u8; 1],
    admin: initial_authority::ID,
    rebalance_authority: initial_authority::ID,
    protocol_fee_beneficiary: initial_authority::ID,
    pricing_program: DEFAULT_PRICING_PROGRAM,
    lp_token_mint: Pubkey::new_from_array([0u8; 32]),
};

pub struct MockPoolState(pub PoolState);

impl IntoAccount for MockPoolState {
    fn into_account(self) -> Account {
        let mut data = vec![0u8; POOL_STATE_SIZE];
        let dst = try_pool_state_mut(&mut data).unwrap();
        *dst = self.0;
        Account {
            lamports: est_rent_exempt_lamports(POOL_STATE_SIZE),
            data,
            owner: s_controller_lib::program::ID,
            executable: false,
            rent_epoch: u64::MAX,
        }
    }
}

#[async_trait]
pub trait PoolStateBanksClient {
    async fn get_pool_state_acc(&mut self) -> Account;
}

#[async_trait]
impl PoolStateBanksClient for BanksClient {
    async fn get_pool_state_acc(&mut self) -> Account {
        self.get_account_unwrapped(s_controller_lib::program::POOL_STATE_ID)
            .await
    }
}

pub trait PoolStateProgramTest {
    fn add_pool_state(self, pool_state: PoolState) -> Self;
}

impl PoolStateProgramTest for ProgramTest {
    fn add_pool_state(self, pool_state: PoolState) -> Self {
        self.add_account_chained(POOL_STATE_ID, MockPoolState(pool_state).into_account())
    }
}

pub struct MockLpMintToInitArgs {
    pub initial_authority: Pubkey,
    pub addr: Pubkey,
}

pub trait LpTokenProgramTest {
    fn add_mock_lp_mint_to_init(self, args: MockLpMintToInitArgs) -> Self;
    fn add_mock_lp_mint(self, addr: Pubkey, supply: u64) -> Self;
}

impl LpTokenProgramTest for ProgramTest {
    fn add_mock_lp_mint_to_init(
        self,
        MockLpMintToInitArgs {
            initial_authority,
            addr,
        }: MockLpMintToInitArgs,
    ) -> Self {
        self.add_tokenkeg_mint_from_args(
            addr,
            MockMintArgs {
                mint_authority: Some(initial_authority),
                freeze_authority: Some(initial_authority),
                supply: 0,
                decimals: 9,
            },
        )
    }

    fn add_mock_lp_mint(self, addr: Pubkey, supply: u64) -> Self {
        self.add_tokenkeg_mint_from_args(
            addr,
            MockMintArgs {
                mint_authority: Some(POOL_STATE_ID),
                freeze_authority: Some(POOL_STATE_ID),
                supply,
                decimals: 9,
            },
        )
    }
}
