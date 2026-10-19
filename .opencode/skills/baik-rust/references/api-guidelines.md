# Ikuti Rust API Guidelines

Ikuti checklist Rust API Guidelines untuk tiap API publik baru.
Pakai nama English yang konsisten dengan std dan ekosistem.
Hindari singkatan tak umum dan hindari tipe tak perlu di nama.

Salah:

```rust
pub fn get_str_vec() -> Vec<String> {
    // Bad: name repeats types and uses vague verb.
    vec![]
}
```

Benar:

```rust
pub fn messages() -> Vec<String> {
    // Good: noun name matches the returned concept.
    vec![]
}
```

Cek kembali visibilitas, ownership, dan bentuk error sebelum stabilkan API.
Rujuk bagian naming dan checklist di dokumen resmi.

## Sumber

- https://rust-lang.github.io/api-guidelines/
