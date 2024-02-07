# stripzip-rs
`stripzip-rs` is a tool which removes metadata from ZIP-like files.

Tested extensions: `.zip`, `.jar`

Install the most recent development version by running
```
cargo install --git https://github.com/TomaSajt/stripzip-rs stripzip-rs
```

Originally created for reproducible java builds on [Nixpkgs](https://github.com/NixOS/nixpkgs)

Inspired by https://github.com/KittyHawkCorp/stripzip

## Features
- remove top-level zip comment
- zero out `last_modified` timestamp of entries
- fix unset unix permissions of entries
- remove all extra fields from entries
- remove comment from entries
