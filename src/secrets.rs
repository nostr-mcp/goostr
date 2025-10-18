use anyhow::{Context, Result};
use keyring::Entry;

const SERVICE: &str = "goostr";

fn entry_for(label: &str) -> Result<Entry> {
    Entry::new(SERVICE, label).context("creating keyring entry")
}

pub fn set(label: &str, secret: &str) -> Result<()> {
    let e = entry_for(label)?;
    e.set_password(secret).context("storing secret in keyring")
}

pub fn get(label: &str) -> Result<Option<String>> {
    let e = entry_for(label)?;
    match e.get_password() {
        Ok(s) => Ok(Some(s)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e).context("retrieving secret from keyring"),
    }
}

pub fn delete(label: &str) -> Result<()> {
    let e = entry_for(label)?;
    match e.delete_password() {
        Ok(_) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e).context("deleting secret from keyring"),
    }
}
