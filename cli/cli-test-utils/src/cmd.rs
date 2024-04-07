use assert_cmd::Command;
use async_trait::async_trait;
use sanctum_solana_test_utils::cli::TempCliConfig;

#[async_trait]
pub trait TestCliCmd {
    /// `--send-mode dump-msg`
    fn with_send_mode_dump_msg(&mut self) -> &mut Self;

    /// `--config <TEMP_CLI_CFG_PATH>`
    fn with_cfg_temp_cli(&mut self, cfg: &TempCliConfig) -> &mut Self;
}

#[async_trait]
impl TestCliCmd for Command {
    fn with_send_mode_dump_msg(&mut self) -> &mut Self {
        self.arg("--send-mode").arg("dump-msg")
    }

    fn with_cfg_temp_cli(&mut self, cfg: &TempCliConfig) -> &mut Self {
        self.arg("--config").arg(cfg.config().path())
    }
}
