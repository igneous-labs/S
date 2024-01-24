use solana_sdk::signer::keypair::Keypair;
use tempfile::NamedTempFile;

pub fn temp_keyfile(kp: &Keypair) -> NamedTempFile {
    let kp_bytes = kp.to_bytes();
    let f = NamedTempFile::new().unwrap();
    serde_json::to_writer(f.as_file(), kp_bytes.as_ref()).unwrap();
    f
}
