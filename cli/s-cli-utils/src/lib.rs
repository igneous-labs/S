pub const CONFIG_HELP: &str =
    "Path to solana CLI config. Defaults to solana cli default if not provided";

pub const TX_SEND_MODE_HELP: &str = "Transaction send mode.
- send-actual: signs and sends the tx to the cluster specified in config and outputs hash to stderr
- sim-only: simulates the tx against the cluster and outputs logs to stderr
- dump-msg: dumps the base64 encoded tx to stdout. For use with inspectors and multisigs
";
