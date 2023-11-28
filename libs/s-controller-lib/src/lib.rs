mod accounts_resolvers;

pub use accounts_resolvers::*;

pub mod program {
    sanctum_macros::declare_program_keys!(
        "scoeWYRwSor53KxfQ8EkNCkka1vasF8td3P3nfHQvsv",
        [
            ("state", b"state"),
            (
                "disable-pool-authority-list",
                b"disable-pool-authority-list"
            ),
            ("rebalance-record", b"rebalance-record"),
            ("protocol-fee", b"protocol-fee"),
        ]
    );
}
