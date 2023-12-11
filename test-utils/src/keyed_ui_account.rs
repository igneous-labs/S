use std::{fs::File, path::Path, str::FromStr};

use serde::{Deserialize, Serialize};
use solana_account_decoder::UiAccount;
use solana_program::pubkey::Pubkey;
use solana_readonly_account::sdk::KeyedAccount;

use crate::test_fixtures_dir;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyedUiAccount {
    pub pubkey: String,
    pub account: UiAccount,
}

impl KeyedUiAccount {
    pub fn from_file<P: AsRef<Path>>(json_file_path: P) -> Self {
        let mut file = File::open(json_file_path).unwrap();
        serde_json::from_reader(&mut file).unwrap()
    }

    /// Load an account from test-fixtures directory.
    /// arg: "account.json" -> "test-fixtures/account.json"
    pub fn from_test_fixtures_file<P: AsRef<Path>>(relative_json_file_path: P) -> Self {
        Self::from_file(test_fixtures_dir().join(relative_json_file_path))
    }

    pub fn to_keyed_account(&self) -> KeyedAccount {
        KeyedAccount {
            pubkey: Pubkey::from_str(&self.pubkey).unwrap(),
            account: self.account.decode().unwrap(),
        }
    }
}
