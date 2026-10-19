# Wajibkan Clippy Lints

Jalankan clippy untuk grup correctness, style, complexity, dan perf.
Anggap temuan correctness sebagai bug dan perbaiki sebelum merge.
Dokumentasikan pengecualian dengan alasan English yang singkat.

Salah:

```rust
fn check_len(items: &[String]) -> bool {
    // Bad: verbose pattern triggers clippy style lint.
    if items.len() == 0 {
        true
    } else {
        false
    }
}
```

Benar:

```rust
fn check_len(items: &[String]) -> bool {
    // Good: idiomatic form passes clippy cleanly.
    items.is_empty()
}
```

Perintah verifikasi:

```sh
cargo clippy --workspace --all-targets -- -D clippy::correctness -D clippy::style -D clippy::complexity -D clippy::perf
```

Simpan output clippy sebagai bukti review bila diminta QA.

## Sumber

- https://doc.rust-lang.org/clippy/
