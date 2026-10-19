# Larang Unwrap di Jalur Input

Aturan ini menolak `unwrap`, `expect`, dan `panic` di semua jalur input pengguna.
Pakai `Result` dan operator `?` agar error mengalir ke pemanggil.
Contoh jalur input adalah parsing tree, node, expr, entry lib, dan entry CLI.

Salah:

```rust
fn parse_count(input: &str) -> usize {
    // Bad: crashes on invalid input.
    input.trim().parse().unwrap()
}
```

Benar:

```rust
fn parse_count(input: &str) -> Result<usize, String> {
    // Good: caller decides how to report the error.
    input.trim().parse().map_err(|e| format!("invalid count: {e}"))
}
```

Periksa pola `unwrap()` di `tree.rs`, `node.rs`, dan `expr.rs`.
Periksa juga `lib.rs:31` dan `main.rs:17` karena keduanya berada di jalur input.

## Sumber

- `tree.rs`
- `node.rs`
- `expr.rs`
- `lib.rs:31`
- `main.rs:17`
