# Penanganan Error dengan Result

Pakai `Result` untuk semua error recoverable seperti IO, parse, dan input salah.
Simpan `panic` hanya untuk bug internal yang tidak mungkin pulih.
Tambah konteks di tiap lapisan dengan `map_err` atau tipe error sendiri.

Salah:

```rust
fn load_source(path: &str) -> String {
    // Bad: hides the cause and crashes on missing file.
    std::fs::read_to_string(path).unwrap()
}
```

Benar:

```rust
fn load_source(path: &str) -> Result<String, String> {
    // Good: preserves path context for the caller.
    std::fs::read_to_string(path).map_err(|e| format!("failed to read {path}: {e}"))
}
```

Bedakan error pengguna dan bug internal di review.
Tolak `panic` sebagai jalan pintas validasi input.

## Sumber

- https://doc.rust-lang.org/book/ch09-00-error-handling.html
