# 🌟 Stellar Micro-Tip Jar (Soroban)

**Stellar Micro-Tip Jar** adalah sebuah *Smart Contract* berbasis Soroban (Stellar) yang dirancang untuk memfasilitasi sistem apresiasi digital (tip) secara terdesentralisasi. Aplikasi ini memungkinkan pengguna untuk mengirimkan XLM atau token lainnya sebagai bentuk dukungan langsung kepada pengembang atau pembuat konten, lengkap dengan pesan dukungan yang tercatat secara permanen di blockchain.

## 🚀 Informasi Deployment

Kontrak ini telah berhasil di-deploy pada jaringan **Stellar Testnet**.

* **Contract ID:** `CBQDIPM6WQC3KB6AWXY3A3CXX3KE2UC3KUO7GQFR5ZKCDUD47H3EXV34`

* **Bahasa Pemrograman:** Rust

* **SDK:** Soroban SDK (v20+)

* **Network:** Testnet

## ✨ Fitur Utama

1. **Sistem Escrow Otomatis:** Menangani perpindahan dana dari dompet pendonor ke kontrak secara aman tanpa perantara.

2. **Pesan Dukungan On-Chain:** Setiap transaksi menyimpan `String` pesan yang terikat dengan ID transaksi, menciptakan interaksi sosial yang transparan.

3. **Audit Saldo Real-time:** Siapa pun dapat memverifikasi total dana yang terkumpul dan melihat riwayat donasi secara terbuka.

4. **Keamanan Berlapis:** Mengintegrasikan `require_auth()` untuk memastikan validitas identitas pengirim melalui **Freighter Wallet**.

## 🛠 Arsitektur Teknis

### Struktur Data

Kontrak menggunakan struktur data `TipEntry` untuk menyimpan informasi setiap donasi:

* `donor`: Alamat publik (Address) pengirim.

* `amount`: Jumlah token dalam satuan *stroops* (i128).

* `message`: Pesan teks pendek (String).

### Mekanisme Penyimpanan (Storage)

Data disimpan menggunakan `Instance Storage` pada Soroban untuk efisiensi biaya gas dan aksesibilitas data yang persisten selama kontrak masih aktif.

## 📋 Prasyarat Sistem

Sebelum berinteraksi dengan kontrak ini, pastikan Anda telah menginstal:

* **Rust & Cargo:** Toolchain bahasa Rust terbaru.

* **Soroban CLI:** Untuk interaksi via terminal.

* **Stellar Laboratory:** Untuk memantau transaksi di Testnet.

* **Freighter Wallet:** Jika ingin menggunakan antarmuka web.

## 💻 Panduan Penggunaan CLI

Gunakan perintah di bawah ini untuk berinteraksi langsung dengan kontrak dari terminal Anda.

### 1. Inisialisasi Tip (Kirim Dana)

Pastikan Anda memiliki saldo XLM Testnet yang cukup.

```bash
soroban contract invoke \
  --id CBQDIPM6WQC3KB6AWXY3A3CXX3KE2UC3KUO7GQFR5ZKCDUD47H3EXV34 \
  --source <IDENTITAS_ANDA> \
  --network testnet \
  -- \
  send_tip \
  --donor <ALAMAT_ANDA> \
  --token_address CDLZ677S3GZ25AEEIWGFAWSWSRH5G6HIOCC7K7O4S7J6R3B6R2L5M5J4 \
  --amount 50000000 \
  --message "Lanjutkan kerja bagusnya, Bang!"
```
(Catatan: token_address di atas adalah alamat standar untuk Native XLM di Testnet)
###2. Cek Total Dana Terkumpul

```bash
soroban contract invoke \
  --id CBQDIPM6WQC3KB6AWXY3A3CXX3KE2UC3KUO7GQFR5ZKCDUD47H3EXV34 \
  --network testnet \
  -- \
  get_total
```

###3. Lihat Riwayat Donatur

```bash
soroban contract invoke \
  --id CBQDIPM6WQC3KB6AWXY3A3CXX3KE2UC3KUO7GQFR5ZKCDUD47H3EXV34 \
  --network testnet \
  -- \
  get_history
```
##🎨 Integrasi Frontend

Untuk menghubungkan dApp ini dengan website Anda, gunakan alur berikut:

    Connect Wallet: Minta izin akses ke Freighter Wallet.

    Fetch Data: Panggil get_total dan get_history saat halaman dimuat untuk menampilkan statistik.

    Sign Transaction: Saat user menekan tombol "Send Tip", buatlah transaksi Soroban, kirim ke Freighter untuk ditandatangani, dan kirim ke jaringan Stellar.

##🔒 Keamanan & Batasan

    Re-entrancy: Kontrak dilindungi dari serangan re-entrancy oleh protokol Soroban.

    Stroops Precision: Selalu ingat bahwa 1 XLM = 10.000.000 Stroops. Pastikan konversi di frontend tepat.

##Screenshot
[Expert](screenshot-expert.png)
[lab](screenshot-lab.png)

Project Info:
Dibuat sebagai bagian dari tugas Workshop dApp Stellar.
© 2024 Developer Community. 🚀