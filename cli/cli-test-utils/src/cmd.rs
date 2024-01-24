use std::process::Output;

use assert_cmd::Command;
use async_trait::async_trait;
use sanctum_solana_test_utils::{cli::TempCliConfig, ExtendedBanksClient};
use solana_program_test::{BanksClientError, BanksTransactionResultWithMetadata};

#[async_trait]
pub trait TestCliCmd {
    /// `--send-mode dump-msg`
    fn with_b64_send_mode(&mut self) -> &mut Self;

    /// `--config <TEMP_CLI_PATH>`
    fn with_temp_cli_cfg(&mut self, cfg: &TempCliConfig) -> &mut Self;

    /// Assumes the cli command outputs one or more base64 encoded transactions
    /// to stdout separated by `\n`
    async fn exec_b64_txs<B: ExtendedBanksClient + Send>(
        self,
        bc: B,
    ) -> Vec<Result<BanksTransactionResultWithMetadata, BanksClientError>>;
}

#[async_trait]
impl TestCliCmd for Command {
    fn with_b64_send_mode(&mut self) -> &mut Self {
        self.arg("--send-mode").arg("dump-msg")
    }

    fn with_temp_cli_cfg(&mut self, cfg: &TempCliConfig) -> &mut Self {
        self.arg("--config").arg(cfg.config().path())
    }

    async fn exec_b64_txs<B: ExtendedBanksClient + Send>(
        mut self,
        mut bc: B,
    ) -> Vec<Result<BanksTransactionResultWithMetadata, BanksClientError>> {
        let Output { stdout, .. } = self.output().unwrap();
        let stdout = std::str::from_utf8(&stdout).unwrap();
        let mut res = vec![];
        for b64 in stdout.split('\n') {
            if !b64.is_empty() {
                res.push(bc.exec_b64_tx(b64.as_bytes()).await);
            }
        }
        res
    }
}
