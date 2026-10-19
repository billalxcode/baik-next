# Satukan Builtin Duplikat

Satukan definisi builtin agar `src/builtin` dan `baik-std` tidak drift.
Satu sumber benar harus diekspos ulang oleh sisi lain.
Tambah test yang gagal saat daftar builtin berbeda.

Salah:

```rust
// Bad: two separate lists drift over time.
const LEGACY_BUILTINS: &[&str] = &["TULIS", "BACA"];
const STD_BUILTINS: &[&str] = &["TULIS"];
```

Benar:

```rust
// Good: one canonical list shared by both layers.
const CANONICAL_BUILTINS: &[&str] = &["TULIS", "BACA"];

fn is_builtin(name: &str) -> bool {
    CANONICAL_BUILTINS.contains(&name)
}
```

Pilih pemilik kanonis lalu hapus duplikat di sisi lain.
Catat pemetaan legacy di komentar English yang singkat.

## Sumber

- `src/builtin`
- `crates/baik-std`
