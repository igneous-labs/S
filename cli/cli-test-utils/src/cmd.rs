use std::process::Output;

use assert_cmd::Command;
use async_trait::async_trait;
use sanctum_solana_test_utils::{cli::TempCliConfig, ExtendedBanksClient};
use solana_program_test::{BanksClientError, BanksTransactionResultWithMetadata};

#[async_trait]
pub trait TestCliCmd {
    /// `--send-mode dump-msg`
    fn with_send_mode_dump_msg(&mut self) -> &mut Self;

    /// `--config <TEMP_CLI_CFG_PATH>`
    fn with_cfg_temp_cli(&mut self, cfg: &TempCliConfig) -> &mut Self;

    /// Assumes the cli command outputs one or more base64 encoded transactions
    /// to stdout separated by `\n`
    async fn exec_b64_txs<B: ExtendedBanksClient + Send>(
        self,
        bc: B,
    ) -> Vec<Result<BanksTransactionResultWithMetadata, BanksClientError>>;
}

#[async_trait]
impl TestCliCmd for Command {
    fn with_send_mode_dump_msg(&mut self) -> &mut Self {
        self.arg("--send-mode").arg("dump-msg")
    }

    fn with_cfg_temp_cli(&mut self, cfg: &TempCliConfig) -> &mut Self {
        self.arg("--config").arg(cfg.config().path())
    }

    async fn exec_b64_txs<B: ExtendedBanksClient + Send>(
        mut self,
        mut bc: B,
    ) -> Vec<Result<BanksTransactionResultWithMetadata, BanksClientError>> {
        let Output {
            stdout,
            status,
            stderr,
        } = self.output().unwrap();
        assert!(
            status.success(),
            "{}",
            std::str::from_utf8(&stderr).unwrap()
        );
        let stdout = std::str::from_utf8(&stdout).unwrap();
        // run txs in sequence, waiting on result of the prev before exec-ing next
        let mut res = vec![];
        for b64 in stdout.split('\n') {
            if !b64.is_empty() {
                res.push(bc.exec_b64_tx(b64.as_bytes()).await);
            }
        }
        res
    }
}
