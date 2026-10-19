# Ganti Magic Numbers Prioritas Operator

Ganti angka prioritas `99`, `100`, `8`, `10`, `6`, `4`, dan `2` dengan konstanta bernama.
Angka mentah menyulitkan review dan mudah salah ubah satu titik.
Pusatkan konstanta di modul operator lalu pakai di parser dan evaluator.

Salah:

```rust
fn precedence(op: &str) -> u8 {
    // Bad: meaning of 10 and 8 is unclear.
    match op {
        "+" => 10,
        "*" => 8,
        _ => 0,
    }
}
```

Benar:

```rust
const PRECEDENCE_ADD: u8 = 10;
const PRECEDENCE_MUL: u8 = 8;

fn precedence(op: &str) -> u8 {
    // Good: names show intent at the call site.
    match op {
        "+" => PRECEDENCE_ADD,
        "*" => PRECEDENCE_MUL,
        _ => 0,
    }
}
```

Acuan utama adalah `src/operator/operator.rs:80-220`.
Samakan nilai di crate lama dan crate baru setelah konstanta dibuat.

## Sumber

- `src/operator/operator.rs:80-220`
