# goostr

Goostr is an extension for [goose](https://github.com/block/goose) that bridges the Nostr network to AI agents and tools.

- Protocol: Machine-Client Protocol (MCP)
- Transport: stdio
- Focus: Nostr keys, relays, and events

## Features

- Encrypted key storage with optional OS keyring for secrets
- Nostr key lifecycle: generate, import, rename, set active, list, remove
- Relay management: set, connect, disconnect, status
- Event operations: view, post
- **Active key enforcement**: All signing operations use only the currently active key
- Configurable data directory, JSON or text logs
- Self-install helper to register the extension in Goose config

## Requirements

- Rust 1.74+ (2021 edition)
- goose CLI/application (for integration)
- An OS keyring provider (optional, recommended)

## Install

```bash
git clone https://github.com/nostr-mcp/goostr
cd goostr
cargo build --release
```

The binary is in `target/release/goostr`.

## Quick start

Start the server over stdio:

```bash
goostr
```

Add extension to goose `config.yml`:

```bash
goostr install
```

## Tools

All tool names are stable and lowercase.

### Key Management
- `nostr_keys_generate` - Generate a new Nostr keypair
- `nostr_keys_import` - Import secret key (nsec or npub)
- `nostr_keys_export` - Export a key in various formats (npub/nsec/hex)
  - **Parameters**:
    - `label` (optional): Key label to export (uses active key if omitted)
    - `format` (optional): Export format - `bech32` (default), `hex`, or `both`
    - `include_private` (optional): Include private key in export (default: false)
  - **Warning**: Setting `include_private=true` will expose your private key. Keep it secure!
- `nostr_keys_verify` - Verify a Nostr key format and validity
  - **Parameters**:
    - `key` (required): Key string to verify (npub, nsec, or 64-char hex)
  - **Returns**: Key type, validity status, and derived public key if valid
  - **Use cases**: Validate before import, check key format, verify checksums
- `nostr_keys_get_public_from_private` - Derive public key from a private key
  - **Parameters**:
    - `private_key` (required): Private key in nsec or hex format
  - **Returns**: Public key in both npub and hex formats
  - **Use cases**: Key recovery, verification, migration
- `nostr_keys_remove` - Remove a key by label
- `nostr_keys_list` - List all stored keys (metadata only)
- `nostr_keys_set_active` - Set the active key by label
- `nostr_keys_active` - Get the active key (metadata only)
- `nostr_keys_rename_label` - Rename a key's label

### Configuration
- `nostr_config_dir` - Get or set the directory used to persist the key index

### Relay Management
- `nostr_relays_set` - Set relays and connect
- `nostr_relays_connect` - Connect to relays that were previously added
- `nostr_relays_disconnect` - Disconnect or remove relays
- `nostr_relays_status` - List relay status and flags

### Event Operations
- `nostr_events_list` - Fetch events using presets or custom filters
  - **Presets**: `my_notes`, `mentions_me`, `my_metadata`, `by_author`, `by_kind`
  - **Parameters**:
    - `preset` (required): Preset name
    - `limit` (optional): Maximum number of events to return
    - `timeout_secs` (optional): Query timeout in seconds (default: 10)
    - `since` (optional): Unix timestamp - only return events after this time
    - `until` (optional): Unix timestamp - only return events before this time
    - `kind` (required for `by_kind` preset): Nostr event kind number (e.g., 1 for text notes)
    - `author_npub` (required for `by_author`, optional for `by_kind`): Author's npub
  - **Note**: All presets default to looking back 7 days if `since` is not specified
- `nostr_events_post_text` - Post a new kind=1 text note to configured relays
  - **Parameters**:
    - `content` (required): Text content of the note
    - `pow` (optional): Proof of work difficulty (u8)
    - `to_relays` (optional): Specific relay URLs to publish to
  - **Returns**: Event ID, pubkey that signed it, success/failed relays
- `nostr_events_post_reaction` - Post a kind=7 reaction event (like, emoji) to another event
  - **Parameters**:
    - `event_id` (required): Hex-encoded ID of the event to react to
    - `event_pubkey` (required): Hex-encoded pubkey of the event author
    - `content` (optional): Reaction content - `+` (like, default), `-` (dislike), or emoji
    - `event_kind` (optional): Kind number of the event being reacted to (u16)
    - `relay_hint` (optional): URL hint where the target event can be found
    - `pow` (optional): Proof of work difficulty (u8)
    - `to_relays` (optional): Specific relay URLs to publish to
  - **Returns**: Event ID, pubkey that signed it, success/failed relays
  - **Note**: Follows NIP-25 specification for reactions

### Metadata Operations
- `nostr_metadata_set` - Set kind 0 metadata (profile) for the active key
  - **Parameters**: All optional
    - `name`: Display name
    - `display_name`: Full display name
    - `about`: Bio/description
    - `picture`: Avatar URL
    - `banner`: Banner image URL
    - `nip05`: NIP-05 identifier (username@domain.com)
    - `lud06`: Lightning address (LNURL)
    - `lud16`: Lightning address (user@domain.com)
    - `website`: Website URL
    - `publish` (default: true): Broadcast to relays immediately
  - **Returns**: Event ID, pubkey that signed it, success/failed relays
- `nostr_metadata_get` - Get kind 0 metadata for the active key from local settings
- `nostr_metadata_fetch` - Fetch kind 0 metadata from relays for a key (uses active key if no label specified)

### Common Event Kinds
- `1` - Text note (short text note)
- `3` - Contacts (follow list)
- `7` - Reaction (like, emoji reaction)
- `30023` - Long-form content (article)
- See [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md) for more kinds

## Active Key Enforcement

**All signing operations use only the currently active key.**

When you post a text note or publish metadata, goostr ensures that:

1. The Nostr client is built with the active key's signer
2. All events are signed with the active key
3. The response includes the `pubkey` field for verification

**How it works:**

- Each time an operation requires signing, goostr calls `ensure_client()` which:
  - Retrieves the current active key from the keystore
  - Builds a Nostr client with that key as the signer
  - Caches the client until the active key changes
  
- When you change the active key (via `nostr_keys_set_active`), the cached client is reset
- The next signing operation will rebuild the client with the new active key

**Verification:**

All signing operations (`nostr_events_post_text`, `nostr_events_post_reaction`, `nostr_metadata_set`) return the `pubkey` that signed the event, allowing you to verify that the correct key was used.

**Example workflow:**

```bash
# Generate or import keys
nostr_keys_generate(label="alice")
nostr_keys_generate(label="bob")

# Set Alice as active
nostr_keys_set_active(label="alice")

# This will be signed with Alice's key
nostr_events_post_text(content="Hello from Alice")
# Response includes: {"id": "...", "pubkey": "alice_pubkey_hex", ...}

# Switch to Bob
nostr_keys_set_active(label="bob")

# This will be signed with Bob's key
nostr_events_post_text(content="Hello from Bob")
# Response includes: {"id": "...", "pubkey": "bob_pubkey_hex", ...}
```

## Environment

- `GOOSTR_DIR` overrides `~/.config/goostr`
- `GOOSTR_JSON` enables JSON logs
- `GOOSTR_NO_STDERR` disables stderr logging
- Logs: `~/.config/goostr/logs/goostr.log` (daily rotation)


## Compatibility

- MCP protocol: `2024-11-05`
- Nostr SDK: `nostr`/`nostr-sdk` 0.43

## Contributing

Open an issue to propose changes, then submit focused PRs.
## License

MIT. See `LICENSE`.
