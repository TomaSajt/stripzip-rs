# stripzip-rs
`stripzip-rs` is a tool which removes metadata and unnecessary entries from ZIP archives.

Tested extensions: `.zip`, `.jar`

Install the most recent development version by running
```
cargo install --git https://github.com/TomaSajt/stripzip-rs stripzip-rs
```

Originally created for reproducible java builds on [Nixpkgs](https://github.com/NixOS/nixpkgs)

Inspired by https://github.com/KittyHawkCorp/stripzip

## Features
- remove top-level zip comment
- remove duplicate file entries
  - sometimes bad `.jar` files have those
- filter out entries with a `glob` pattern using the `--blacklist` flag
  - example: `--blacklist "**/*.clj"` (filters out all files with the `.clj` extension)
  - make sure to use parentheses to not allow your shell to expand it for itself
- zero out `last_modified` timestamp of entries
- fixup unix permissions of entries
- remove all extra fields from entries
- remove comment from entries
