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

    fn cmd_remove_disable_auth(&mut self) -> &mut Self;

    fn cmd_set_protocol_fee(&mut self) -> &mut Self;

    fn cmd_add_lst(&mut self) -> &mut Self;

    fn cmd_remove_lst(&mut self) -> &mut Self;

    fn cmd_disable_lst_input(&mut self) -> &mut Self;

    fn cmd_enable_lst_input(&mut self) -> &mut Self;

    fn cmd_disable_pool(&mut self) -> &mut Self;

    fn cmd_set_protocol_fee_beneficiary(&mut self) -> &mut Self;

    fn cmd_enable_pool(&mut self) -> &mut Self;

    fn cmd_set_pricing_prog(&mut self) -> &mut Self;

    fn cmd_set_sol_value_calculator_prog(&mut self) -> &mut Self;
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

    fn cmd_add_lst(&mut self) -> &mut Self {
        self.arg("add-lst")
    }

    fn cmd_remove_lst(&mut self) -> &mut Self {
        self.arg("remove-lst")
    }

    fn cmd_disable_lst_input(&mut self) -> &mut Self {
        self.arg("disable-lst-input")
    }

    fn cmd_enable_lst_input(&mut self) -> &mut Self {
        self.arg("enable-lst-input")
    }

    fn cmd_disable_pool(&mut self) -> &mut Self {
        self.arg("disable-pool")
    }

    fn cmd_set_protocol_fee_beneficiary(&mut self) -> &mut Self {
        self.arg("set-protocol-fee-beneficiary")
    }

    fn cmd_enable_pool(&mut self) -> &mut Self {
        self.arg("enable-pool")
    }

    fn cmd_remove_disable_auth(&mut self) -> &mut Self {
        self.arg("remove-disable-auth")
    }

    fn cmd_set_pricing_prog(&mut self) -> &mut Self {
        self.arg("set-pricing-prog")
    }

    fn cmd_set_sol_value_calculator_prog(&mut self) -> &mut Self {
        self.arg("set-sol-value-calculator")
    }
}
