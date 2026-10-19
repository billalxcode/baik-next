# Hindari Clone Boros

Hindari `clone()` di lookup simbol dan render yang sering dipanggil.
Pakai referensi, slice, atau `Cow` untuk data pinjam sementara.
Clone hanya bila kepemilikan baru memang dibutuhkan.

Salah:

```rust
fn lookup(name: String, table: Vec<String>) -> bool {
    // Bad: clones the whole table on every lookup.
    let owned = table.clone();
    owned.contains(&name)
}
```

Benar:

```rust
use std::borrow::Cow;

fn lookup<'a>(name: &str, table: &'a [String]) -> Cow<'a, str> {
    // Good: borrows on hit and allocates only on miss.
    if table.iter().any(|item| item == name) {
        Cow::Borrowed(name)
    } else {
        Cow::Owned(format!("missing: {name}"))
    }
}
```

Profiling sederhana cukup sebagai bukti sebelum optimasi lanjut.
Catat alasan tiap `clone` yang tersisa di komentar English.

## Sumber

- `crates/baik-interp`
- `crates/baik-std`
