---
name: baik-rust
description: Aturan Rust untuk interpreter BAIK agar bebas panic dan konsisten.
---

# baik-rust

## Tujuan

Skill ini menstandarkan kode Rust di repo baik-next.
Fokus utama adalah hapus panic di jalur input dan samakan semantik legacy.
Semua contoh kode memakai English.

## Kapan Pakai

Pakai skill ini saat ubah kode di `crates/` atau `src/`.
Pakai skill ini saat review parser, interpreter, operator, atau builtin.
Pakai skill ini sebelum kirim perubahan Rust ke quality-assurance.

## References

Baca file yang relevan sebelum ubah kode.
- `references/no-unwrap-input.md`: Tolak `unwrap` di jalur input dan pakai `Result` dengan `?`.
- `references/no-panic-expect.md`: Tolak `panic` dan `expect` di CLI dan render lalu kembalikan error jelas.
- `references/error-handling.md`: Pakai `Result` untuk error recoverable dan simpan `panic` hanya untuk bug.
- `references/magic-numbers.md`: Ganti angka prioritas operator dengan konstanta bernama.
- `references/builtin-dedup.md`: Satukan definisi builtin agar `src/builtin` dan `baik-std` tidak drift.
- `references/run-simple-limit.md`: Tandai `run_simple` sebagai stage 1 yang hanya dukung `TULIS` dan `TAMAT`.
- `references/div-zero-empty.md`: Cek pembagi nol dan samakan semantik `is_empty` antar implementasi.
- `references/clippy-lints.md`: Jalankan grup lint clippy untuk correctness, style, complexity, dan perf.
- `references/api-guidelines.md`: Ikuti checklist dan aturan nama dari Rust API Guidelines.
- `references/testing.md`: Tulis unit test per fungsi plus integration test untuk perilaku publik.
- `references/naming-docs.md`: Pakai nama English dan tulis docs untuk tiap item publik.
- `references/clone-efficiency.md`: Hindari `clone` boros dan pakai referensi atau `Cow`.

## Checklist Review

- [ ] Tidak ada `unwrap`, `expect`, atau `panic` baru di jalur input.
- [ ] Semua angka prioritas operator memakai konstanta bernama.
- [ ] Error recoverable memakai `Result` dengan pesan jelas.
- [ ] `cargo clippy` bersih untuk grup yang diwajibkan.
- [ ] Test baru menutup fungsi dan jalur yang diubah.
- [ ] Semua karakter file adalah ASCII dan tanpa em dash.
