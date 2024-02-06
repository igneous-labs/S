use assert_cmd::Command;
use cli_test_utils::TestCliCmd;
use sanctum_solana_test_utils::cli::TempCliConfig;

pub fn cargo_bin() -> Command {
    Command::cargo_bin("sctr").unwrap()
}

pub fn base_cmd(cfg: &TempCliConfig) -> Command {
    let mut cmd = cargo_bin();
    cmd.with_send_mode_dump_msg().with_cfg_temp_cli(cfg);
    cmd
}

pub trait TestSctrCmd {
    fn cmd_init(&mut self) -> &mut Self;

    fn cmd_set_admin(&mut self) -> &mut Self;

    fn cmd_add_disable_auth(&mut self) -> &mut Self;

    fn cmd_set_protocol_fee(&mut self) -> &mut Self;
}

impl TestSctrCmd for Command {
    fn cmd_init(&mut self) -> &mut Self {
        self.arg("init")
    }

    fn cmd_set_admin(&mut self) -> &mut Self {
        self.arg("set-admin")
    }

    fn cmd_add_disable_auth(&mut self) -> &mut Self {
        self.arg("add-disable-auth")
    }

    fn cmd_set_protocol_fee(&mut self) -> &mut Self {
        self.arg("set-protocol-fee")
    }
}
