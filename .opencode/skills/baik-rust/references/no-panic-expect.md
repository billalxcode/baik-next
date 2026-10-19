# Larang Panic dan Expect di CLI dan Render

Aturan ini menolak `panic!` dan `expect` di CLI serta render `baik-interp`.
Kembalikan `Result` atau `String` error agar proses exit terkendali.
Pesan error harus sebut konteks, nilai, dan aksi perbaikan.

Salah:

```rust
fn render_value(index: usize, values: &[String]) -> &str {
    // Bad: crashes the CLI on bad index.
    &values[index]
}
```

Benar:

```rust
fn render_value(index: usize, values: &[String]) -> Result<&str, String> {
    // Good: returns a clear error for the CLI layer.
    values.get(index).ok_or_else(|| format!("render index out of range: {index}"))
}
```

Ganti semua `panic!` di render dengan error yang bisa ditampilkan ke pengguna.
Uji jalur gagal dengan input kosong dan indeks di luar rentang.

## Sumber

- `crates/baik-interp`
- `src/main.rs`
