use crate::error::Error;
use crate::keys::KeyStore;
use crate::secrets;
use crate::settings::SettingsStore;
use anyhow::{anyhow, Context};
use nostr::prelude::*;
use nostr_sdk::prelude::*;
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell, RwLock};
use tracing::{info, warn};

#[derive(Clone)]
pub struct ActiveClient {
    pub client: Client,
    pub active_label: String,
    pub active_pubkey: PublicKey,
}

static CLIENT_CELL: OnceCell<RwLock<Option<ActiveClient>>> = OnceCell::const_new();
static BUILD_LOCK: OnceCell<Mutex<()>> = OnceCell::const_new();

async fn build_from_keystore(
    ks: &KeyStore,
    settings_store: &SettingsStore,
) -> Result<Option<ActiveClient>, Error> {
    let active = ks.get_active().await;
    let Some(active_entry) = active else {
        return Ok(None);
    };
    let label = active_entry.label.clone();
    let pubkey = PublicKey::from_bech32(&active_entry.public_key)
        .with_context(|| "invalid active public key")?;
    let maybe_nsec = secrets::get(&label)?;
    let client = if let Some(nsec) = maybe_nsec {
        let keys =
            Keys::parse(&nsec).map_err(|e| anyhow!("invalid stored secret for '{label}': {e}"))?;
        Client::builder()
            .signer(keys)
            .opts(ClientOptions::new().automatic_authentication(true))
            .build()
    } else {
        Client::builder()
            .opts(ClientOptions::new().automatic_authentication(true))
            .build()
    };

    let pubkey_hex = pubkey.to_hex();
    if let Some(settings) = settings_store.get_settings(&pubkey_hex).await {
        if !settings.relays.is_empty() {
            info!(
                "auto-connecting to {} relay(s) for key '{}'",
                settings.relays.len(),
                label
            );
            for url in &settings.relays {
                match client.add_relay(url).await {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("failed to add relay {}: {}", url, e);
                    }
                }
            }
            client.connect().await;
        }
    }

    Ok(Some(ActiveClient {
        client,
        active_label: label,
        active_pubkey: pubkey,
    }))
}

pub async fn ensure_client(
    ks: Arc<KeyStore>,
    settings_store: Arc<SettingsStore>,
) -> Result<ActiveClient, Error> {
    let cell = CLIENT_CELL
        .get_or_try_init(|| async {
            Ok::<RwLock<Option<ActiveClient>>, anyhow::Error>(RwLock::new(None))
        })
        .await?;
    {
        let r = cell.read().await;
        if let Some(ac) = r.clone() {
            let active = ks.get_active().await;
            if active.as_ref().map(|e| &e.label) == Some(&ac.active_label) {
                return Ok(ac);
            }
        }
    }
    let _g = BUILD_LOCK
        .get_or_try_init(|| async { Ok::<Mutex<()>, anyhow::Error>(Mutex::new(())) })
        .await?
        .lock()
        .await;
    {
        let r = cell.read().await;
        if let Some(ac) = r.clone() {
            let active = ks.get_active().await;
            if active.as_ref().map(|e| &e.label) == Some(&ac.active_label) {
                return Ok(ac);
            }
        }
    }
    let built = build_from_keystore(&ks, &settings_store).await?;
    if let Some(ac) = built {
        {
            let mut w = cell.write().await;
            *w = Some(ac.clone());
        }
        info!("nostr client initialized for active key");
        Ok(ac)
    } else {
        Err(Error::invalid(
            "no active nostr key; set one with nostr_keys_set_active",
        ))
    }
}

pub async fn reset_cached_client() -> Result<(), Error> {
    if let Some(cell) = CLIENT_CELL.get() {
        let mut w = cell.write().await;
        *w = None;
    }
    Ok(())
}
