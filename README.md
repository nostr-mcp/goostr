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

- `nostr_keys_generate`
- `nostr_keys_import`
- `nostr_keys_remove`
- `nostr_keys_list`
- `nostr_keys_set_active`
- `nostr_keys_active`
- `nostr_keys_rename_label`
- `nostr_config_dir`
- `nostr_relays_set`
- `nostr_relays_connect`
- `nostr_relays_disconnect`
- `nostr_relays_status`
- `nostr_events_list`
- `nostr_events_post_text`

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
