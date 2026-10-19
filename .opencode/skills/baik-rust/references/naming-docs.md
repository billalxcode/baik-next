# Nama English dan Docs Publik

Pakai nama English untuk semua fungsi, tipe, modul, dan variabel.
Tulis docs `///` untuk tiap item publik beserta contoh singkat.
Ikuti bab documentation dari Rust API Guidelines.

Salah:

```rust
// Bad: missing docs and unclear name.
pub fn proc(d: Vec<String>) -> Vec<String> {
    d
}
```

Benar:

```rust
/// Filters blank messages from the input list.
///
/// # Example
///
/// ```
/// let kept = filter_blank_messages(vec![String::from("hi")]);
/// assert_eq!(kept.len(), 1);
/// ```
pub fn filter_blank_messages(messages: Vec<String>) -> Vec<String> {
    messages.into_iter().filter(|m| !m.is_empty()).collect()
}
```

Pastikan `cargo doc` tanpa warning untuk item publik baru.
Tolak komentar dan pesan berbahasa campuran di dalam kode.

## Sumber

- https://rust-lang.github.io/api-guidelines/documentation.html
