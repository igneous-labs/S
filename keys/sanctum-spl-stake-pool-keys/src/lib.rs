pub mod sanctum_spl_stake_pool_program {
    sanctum_macros::declare_program_keys!("SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY", []);
}

pub mod sanctum_spl_stake_pool_program_progdata {
    sanctum_macros::declare_program_keys!("Cn5fegqLh8Fmvffisr4Wk3LmuaUgMMzTFfEuidpZFsvV", []);
}

// TODO: spin this off into its own lib crate once sanctum-spl diverges from spl
pub mod sanctum_spl_sol_val_calc_program {
    sanctum_macros::declare_program_keys!(
        "sspUE1vrh7xRoXxGsg7vR1zde2WdGtJRbyK9uRumBDy",
        [("sanctum_spl_calculator_state", b"state")]
    );
}
