//! Utils for loading accounts into ProgramTest

use std::{fs::read_dir, path::Path, str::FromStr};

use solana_program_test::ProgramTest;
use solana_sdk::{account::Account, pubkey::Pubkey};

use crate::{test_fixtures_dir, KeyedUiAccount};

/// For nice method call-chaining
/// `let program_test = program_test.add_account_from_file(...).add_account_from_file(...);`
pub trait AddAccount: Sized {
    fn add_account_inner(self, address: Pubkey, account: Account) -> Self;

    fn add_keyed_ui_account(self, KeyedUiAccount { pubkey, account }: KeyedUiAccount) -> Self {
        self.add_account_inner(
            Pubkey::from_str(&pubkey).unwrap(),
            account.decode().unwrap(),
        )
    }

    /// Panics if unable to load json file
    fn add_account_from_file<P: AsRef<Path>>(self, json_file_path: P) -> Self {
        self.add_keyed_ui_account(KeyedUiAccount::from_file(json_file_path))
    }

    /// Add an account from test-fixtures directory.
    /// arg: "account.json" -> "test-fixtures/account.json"
    fn add_test_fixtures_account<P: AsRef<Path>>(self, relative_json_file_path: P) -> Self {
        self.add_keyed_ui_account(KeyedUiAccount::from_test_fixtures_file(
            relative_json_file_path,
        ))
    }

    /// Adds all .json accounts in test-fixtures/ directory
    fn add_all_test_fixtures_accounts(mut self) -> Self {
        let read_dir_iter = read_dir(test_fixtures_dir()).unwrap();
        for entry_res in read_dir_iter {
            let entry = entry_res.unwrap();
            let path = entry.path();
            let ext = match path.extension() {
                Some(e) => e,
                None => continue,
            };
            if ext != "json" {
                continue;
            }
            self = self.add_account_from_file(path);
        }
        self
    }
}

impl AddAccount for ProgramTest {
    fn add_account_inner(mut self, address: Pubkey, account: Account) -> Self {
        self.add_account(address, account);
        self
    }
}
