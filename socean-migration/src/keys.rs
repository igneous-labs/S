pub mod scnsol_metadata_update_auth {
    // TODO: Set this to actual metadata update authority pubkey
    #[cfg(not(feature = "testing"))]
    sanctum_macros::declare_program_keys!("TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111", []);

    #[cfg(feature = "testing")]
    sanctum_macros::declare_program_keys!("CK9cEJT7K7oRrMCcEbBQRGqHLGpxKXWnKvW7nHSDMHD1", []);
}

/// Signer of the migrate instruction.
/// Excess rent lamports is refunded to here
pub mod migrate_auth {
    // TODO: Set this to actual migrate auth pubkey
    #[cfg(not(feature = "testing"))]
    sanctum_macros::declare_program_keys!("TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111", []);

    #[cfg(feature = "testing")]
    sanctum_macros::declare_program_keys!("CK9cEJT7K7oRrMCcEbBQRGqHLGpxKXWnKvW7nHSDMHD1", []);
}

pub mod socean_spl_pool {
    sanctum_macros::declare_program_keys!("5oc4nmbNTda9fx8Tw57ShLD132aqDK65vuHH4RU1K4LZ", []);
}

pub mod socean_spl_validator_list {
    sanctum_macros::declare_program_keys!("8pTa29ovYHxjQgX7gjxGi395GAo8DSXCRTKJZvwMc6MR", []);
}

pub mod socean_spl_reserves {
    sanctum_macros::declare_program_keys!("4sDXGroVt7ba45rzXtNto97QjG1rHm8Py3v56Mgg16Nc", []);
}

pub mod socean_laine_vsa {
    sanctum_macros::declare_program_keys!("335H9DZJVjyQHrb8MiEaLhbD5i1sG4YFMWX5t8jLi5bm", []);
}

pub mod scnsol_mint {
    sanctum_macros::declare_program_keys!("5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm", []);
}

pub mod lainesol_mint {
    sanctum_macros::declare_program_keys!("LAinEtNLgpmCP9Rvsf5Hn8W6EhNiKLZQti1xfWMLy6X", []);
}

pub mod socean_program {
    // b"G_\x1d..." is socean stake pool
    // python: base58.b58decode("5oc4nmbNTda9fx8Tw57ShLD132aqDK65vuHH4RU1K4LZ")
    sanctum_macros::declare_program_keys!(
        "5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx",
        [(
            "withdraw_auth",
            b"G_\x1d\x08\xedE\xbcwV-<\xaf\xd1=`\xcc\xdcLo\xa2\x81\x90\xe1Af\xf0\x17C\x01\x19(\n",
            b"withdraw"
        ),]
    );
}

pub mod ata_program {
    // b"\x8d\xd8r\xa3\xb7\x15\xde\xd1\xd4`4?\xf5\xbaJ(\x10.9\x02G%\x89_\xeb\xc7\xa9\xc7\x97!3\xd3" is pool state
    // b"Q=\x96\xf8\xb1\xfc\x8eHK\xf4'h\x84\x04O\xe4i\x8c\x91\xc8\\V\x8d\x8f\x15ti\xf4\x9f\xa7\xee\x11" is protocol fee
    // b"\x06\xdd\xf6\xe1\xd7e\xa1\x93\xd9\xcb\xe1F\xce\xeby\xac\x1c\xb4\x85\xed_[7\x91:\x8c\xf5\x85~\xff\x00\xa9" is token program
    // b"\x04\xe9\x06\xb5\x1e\x90\x97/\xd4\xcdi\x94:\x88a\xda\xc5y?\xa7<\xf7{,\xb3\xd7c#_\x83\x07x" is laineSOL mint
    sanctum_macros::declare_program_keys!(
        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        [
            (
                "lainesol_reserves",
                b"\x8d\xd8r\xa3\xb7\x15\xde\xd1\xd4`4?\xf5\xbaJ(\x10.9\x02G%\x89_\xeb\xc7\xa9\xc7\x97!3\xd3",
                b"\x06\xdd\xf6\xe1\xd7e\xa1\x93\xd9\xcb\xe1F\xce\xeby\xac\x1c\xb4\x85\xed_[7\x91:\x8c\xf5\x85~\xff\x00\xa9",
                b"\x04\xe9\x06\xb5\x1e\x90\x97/\xd4\xcdi\x94:\x88a\xda\xc5y?\xa7<\xf7{,\xb3\xd7c#_\x83\x07x"
            ),
            (
                "lainesol_protocol_fee_accum",
                b"Q=\x96\xf8\xb1\xfc\x8eHK\xf4'h\x84\x04O\xe4i\x8c\x91\xc8\\V\x8d\x8f\x15ti\xf4\x9f\xa7\xee\x11",
                b"\x06\xdd\xf6\xe1\xd7e\xa1\x93\xd9\xcb\xe1F\xce\xeby\xac\x1c\xb4\x85\xed_[7\x91:\x8c\xf5\x85~\xff\x00\xa9",
                b"\x04\xe9\x06\xb5\x1e\x90\x97/\xd4\xcdi\x94:\x88a\xda\xc5y?\xa7<\xf7{,\xb3\xd7c#_\x83\x07x"
            )]
    );
}

pub mod token_metadata_program {
    // b"\x0bpe\xb1\xe3\xd1|E8\x9dR\x7fk\x04\xc3\xcdX\xb8ls\x1a\xa0\xfd\xb5I\xb6\xd1\xbc\x03\xf8)F" is metadata program
    // b"GW\x89\x9f\xb8\xbe\xdb\xa2\x87x\xaa\xcdg\xe5h\xe74p\xcc\xe9\x0b\xcdS+l\xb6\x18)v(\x82N" is scnsol mint
    sanctum_macros::declare_program_keys!(
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
        [(
            "metadata_pda",
            b"metadata",
            b"\x0bpe\xb1\xe3\xd1|E8\x9dR\x7fk\x04\xc3\xcdX\xb8ls\x1a\xa0\xfd\xb5I\xb6\xd1\xbc\x03\xf8)F",
            b"GW\x89\x9f\xb8\xbe\xdb\xa2\x87x\xaa\xcdg\xe5h\xe74p\xcc\xe9\x0b\xcdS+l\xb6\x18)v(\x82N"
        )]
    );
}

pub mod lainesol_stake_pool {
    sanctum_macros::declare_program_keys!("2qyEeSAWKfU18AFthrF7JA8z8ZCi1yt76Tqs917vwQTV", []);
}

pub mod lainesol_validator_list {
    sanctum_macros::declare_program_keys!("sHPN95ARJpwN3Yipc22Z3m5118K3czRLBG7WmLDLsMp", []);
}

pub mod lainesol_vsa {
    sanctum_macros::declare_program_keys!("AnBd8duMWDC4ubDfJS1Uv7xDx258FRMcJiBzpok8efjN", []);
}

pub mod lainesol_stake_reserves {
    sanctum_macros::declare_program_keys!("H2HfvQc8JcZxCvAQNdYou9jYHSo2oUU8aadqo2wQ1vK", []);
}

pub mod lainesol_fee_dest {
    sanctum_macros::declare_program_keys!("FQLvrMDsqJ2brYQRqG2Cgp5hvAJ7Z8C7boMtdi75iX7W", []);
}

pub mod spl_stake_pool_program {
    // b"\x1bg&\xb3\xb4\xf9\xa6\x80\x9f\xf9R\\\n\x0f2\xf3\x0f\x9bi\xf6\xd6\xfb\x86\xa5Di\xc9\xac\xa8\xc5d\xbc" is laine stake pool
    sanctum_macros::declare_program_keys!(
        "SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy", [
            (
                "lainesol_withdraw_auth",
                b"\x1bg&\xb3\xb4\xf9\xa6\x80\x9f\xf9R\\\n\x0f2\xf3\x0f\x9bi\xf6\xd6\xfb\x86\xa5Di\xc9\xac\xa8\xc5d\xbc",
                b"withdraw",
            ),
            (
                "lainesol_deposit_auth",
                b"\x1bg&\xb3\xb4\xf9\xa6\x80\x9f\xf9R\\\n\x0f2\xf3\x0f\x9bi\xf6\xd6\xfb\x86\xa5Di\xc9\xac\xa8\xc5d\xbc",
                b"deposit",
            )
        ]
    );
}

// The hard-coded authority to take the ownership of the DOS'd VSAs
pub mod designated_stake_authority {
    // TODO: Set this to actual auth pubkey
    sanctum_macros::declare_program_keys!("TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111", []);
}
