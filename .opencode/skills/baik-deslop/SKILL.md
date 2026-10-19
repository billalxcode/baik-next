---
name: baik-deslop
description: Cegah AI slop untuk proyek BAIK. Larang em dash, wajib ASCII keyboard, wajib Bahasa Indonesia efektif tanpa pengulangan. Kode wajib English.
---

# baik-deslop

Tujuan: jaga output tetap ringkas, teknis, dan bebas AI slop.

## Kapan dipakai

Pakai skill ini untuk semua tugas di repo baik-next dan baik-lang-9: tulis kode, tulis docs, jawab pertanyaan, review.

## Aturan keras

1. DILARANG KERAS memakai em dash. Pakai hyphen-minus `-` bila perlu tanda pisah.
2. WAJIB memakai ASCII keyboard saja: kode 32 sampai 126 plus newline dan tab. DILARANG KERAS memakai non-ASCII atau unicode di luar keyboard.
3. SELALU GUNAKAN Bahasa Indonesia.
4. SELALU GUNAKAN kalimat efektif. Satu kalimat untuk satu fakta.
5. TIDAK ADA kalimat berulang. Jangan tulis ulang maksud yang sama dengan kata lain.
6. Hindari AI slop: tanpa pembuka basa-basi, tanpa pujian kosong, tanpa superlatif kosong, tanpa emoji kecuali user minta, tanpa klaim tanpa bukti.
7. Kode WAJIB English. Semua identifier, komentar, pesan error, dan docs di dalam kode memakai English. TIDAK ADA Bahasa Indonesia di dalam kode. Keyword bahasa BAIK (`TULIS`, `KALAU`, `SELAGI`) tetap Indonesia karena itu sintaks bahasa, bukan kode Rust.

## Definisi AI slop yang dilarang

- Pembuka kosong: "Tentu saja", "Sebagai AI", "Semoga membantu", "Mari kita selami".
- Pujian kosong: "luar biasa", "sangat bagus sekali", "revolusioner" tanpa data.
- Paragraf umum yang bisa dipakai untuk topik apa pun.
- Pengulangan instruksi user sebagai kalimat baru.
- Penutup promosi yang tidak diminta.

## Workflow

1. Baca file target langsung sebelum jawab. Jangan tebak isi.
2. Tulis draf jawaban atau kode.
3. Cek draf: hapus kalimat tidak efektif, hapus duplikat makna, ganti non-ASCII ke ASCII, pastikan tidak ada em dash, pastikan kode English tanpa Indonesia.
4. Cek perintah ini bila ragu:
   - `grep -P -n "[^\x00-\x7F]" <file>` harus kosong.
   - `grep -n -- "--" <file>` periksa manual. String `--` di kode CLI tetap boleh. Karakter em dash tunggal tidak boleh muncul.
5. Tulis output akhir dalam Bahasa Indonesia.
6. Untuk kode BAIK, rujuk `.caches/sintaks-dasar-baik.md` dan `.caches/inventaris-ina.md`. Contoh pola: `TULIS`, `KALAU ... MAKA ... STOPKALAU`, `SELAGI ... LAKUKAN ... BALIKLAGI`, `UNTUK ... ULANG ... LAGI`, `fungsi ... balik`, `&Nama()`, `KODE_LUAR`, `untaian`.

## Contoh salah dan benar

Salah: "Tentu saja, solusi luar biasa ini akan merevolusi cara Anda menulis kode BAIK dengan pengalaman yang luar biasa!"

Benar: "Fungsi gagal karena `panjang` menerima `null`. Tambah cek kosong sebelum panggil."

Salah: pakai em dash U+2014 di antara kata. Contoh pola salah: Analisa [EMDASH] hasil [EMDASH] final.

Benar: "Analisa - hasil - final"

Salah: pakai kutip keriting dan panah unicode.

Benar: "\"kutip lurus\", panah -> versi ASCII"

## Format referensi kode

Saat sebut fungsi atau kode, tulis `path:baris`. Contoh: `interpreter.c:1531`, `baik_ident.c:44`, `tree.rs:120`.

## Checklist sebelum kirim

- [ ] Bahasa Indonesia.
- [ ] Tidak ada em dash.
- [ ] Semua karakter ASCII keyboard.
- [ ] Tidak ada kalimat berulang.
- [ ] Kode English tanpa Indonesia.
- [ ] Setiap klaim ada sumber file atau hasil eksekusi.
