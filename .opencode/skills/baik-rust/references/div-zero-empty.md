# Cek Pembagi Nol dan Samakan is_empty

Cek pembagi nol sebelum operasi bagi dan modulo.
Samakan semantik `is_empty` antara legacy dan crate baru.
Pilih satu definisi kosong lalu kunci dengan test.

Salah:

```rust
fn divide(a: f64, b: f64) -> f64 {
    // Bad: produces inf without context.
    a / b
}
```

Benar:

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    // Good: zero divisor becomes a clear error.
    if b == 0.0 {
        return Err(String::from("division by zero"));
    }
    Ok(a / b)
}
```

Bandingkan implementasi legacy dan crate baru untuk string kosong, nol, dan koleksi kosong.
Tambah test tabel untuk tiap kasus batas tersebut.

## Sumber

- `src/builtin`
- `crates/baik-std`
