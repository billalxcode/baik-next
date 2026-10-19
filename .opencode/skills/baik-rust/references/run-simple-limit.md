# Batas run_simple sebagai Stage 1

Tandai `run_simple` hanya untuk stage 1 dengan dukungan `TULIS` dan `TAMAT`.
Tolak tambah statement baru tanpa ubah dokumen stage dan test.
Kembalikan error jelas untuk statement di luar stage 1.

Salah:

```rust
fn run_simple(op: &str) -> Result<(), String> {
    // Bad: silently ignores unsupported ops.
    Ok(())
}
```

Benar:

```rust
fn run_simple(op: &str) -> Result<(), String> {
    // Good: boundary is explicit and testable.
    match op {
        "TULIS" | "TAMAT" => Ok(()),
        _ => Err(format!("unsupported in stage 1: {op}")),
    }
}
```

Dokumentasikan batas ini di komentar fungsi dan di test integrasi.
Perluas dukungan hanya lewat rencana stage berikutnya.

## Sumber

- `crates/baik-interp`
