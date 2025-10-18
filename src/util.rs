use base64::Engine;
use rand::RngCore;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use zeroize::Zeroize;

pub fn nostr_config_root() -> PathBuf {
    std::env::var_os("GOOSTR_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".config")
                .join("goostr")
        })
}

pub fn nostr_index_path() -> PathBuf {
    nostr_config_root().join("keys.enc")
}

pub fn legacy_keys_json_path() -> PathBuf {
    nostr_config_root().join("keys.json")
}

pub fn ensure_parent_dir(p: &Path) -> std::io::Result<()> {
    if let Some(dir) = p.parent() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

pub fn keystore_secret_path() -> PathBuf {
    nostr_config_root().join("keystore.secret")
}

pub fn ensure_keystore_secret() -> anyhow::Result<Vec<u8>> {
    use anyhow::Context;
    ensure_parent_dir(&keystore_secret_path())?;
    let path = keystore_secret_path();
    if !path.exists() {
        let mut buf = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut buf);
        let b64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(buf);
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .with_context(|| format!("opening {}", path.display()))?;
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o600));
        }
        f.write_all(b64.as_bytes())
            .context("writing keystore secret")?;
        f.flush().ok();
    }
    let mut f = File::open(&path).with_context(|| format!("opening {}", path.display()))?;
    let mut s = String::new();
    f.read_to_string(&mut s)
        .with_context(|| format!("reading {}", path.display()))?;
    let bytes = base64::engine::general_purpose::STANDARD_NO_PAD
        .decode(s.trim())
        .context("decoding keystore secret")?;
    let mut tmp = s;
    tmp.zeroize();
    Ok(bytes)
}
