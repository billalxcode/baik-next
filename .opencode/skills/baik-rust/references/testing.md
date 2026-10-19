# Unit Test dan Integration Test

Tulis unit test untuk tiap fungsi baru di modul yang sama.
Tambah integration test untuk perilaku publik lewat crate API.
Uji jalur sukses, jalur error, dan nilai batas.

Salah:

```rust
pub fn add(a: i64, b: i64) -> i64 {
    // Bad: no test locks the behavior.
    a + b
}
```

Benar:

```rust
pub fn add(a: i64, b: i64) -> i64 {
    // Good: covered by the unit test below.
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_values() {
        assert_eq!(add(2, 3), 5);
    }
}
```

Jalankan `cargo test --workspace` sebelum minta review.
Simpan fixture integrasi di `tests/` dengan nama English yang jelas.

## Sumber

- https://doc.rust-lang.org/book/ch11-00-testing.html
