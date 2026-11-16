<a id="getting-started"></a>

# Memulai

Halaman ini memandu Anda dari unduhan hingga layar pertama aplikasi.

<a id="what-you-need"></a>

## Yang Anda perlukan

- **Windows 10 atau Windows 11, 64-bit.** Aplikasi ini adalah program x64. Windows 11 on Arm menjalankannya melalui [emulasi](Glossary#emulation). Windows 10 on Arm tidak dapat menjalankan program x64, dan Windows 32-bit sama sekali tidak dapat menjalankannya. Windows 10 dan 11 desktop pada x64 belum dijalankan dalam pengujian proyek, yang menggunakan Windows Server 2022 dan 2025 serta Windows 11 on Arm. Lihat [Persyaratan dan pengujian](Requirements-and-Testing).
- **Hak administrator** di PC. Aplikasi selalu memintanya saat dimulai. Lihat [Hak administrator](Glossary#administrator-rights).
- **Direct3D 12**, yang digunakan Windows untuk menggambar jendela aplikasi. Windows 10 dan 11 biasanya memilikinya. Driver grafis lama atau mesin virtual mungkin tidak. Lihat [Direct3D 12](Glossary#direct3d-12).
- **Ruang di drive** untuk penginstal yang Anda buat. Aplikasi memberi tahu Anda berapa banyak ruang yang diperlukan sebelum Anda mulai.
- **Internet di PC lama bersifat opsional.** Jika tersedia saat Anda membuat penginstal, aplikasi mengunduh [runtime](Glossary#runtime) Microsoft yang dapat disertakannya dan memasukkannya ke dalam penginstal. Lihat [Runtime](Runtimes).

Detail lebih lanjut, serta apa yang sudah dan belum diuji, ada di [Persyaratan dan pengujian](Requirements-and-Testing).

<a id="download"></a>

## Unduh

Buka [rilis terbaru](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest). Rilis itu memiliki tiga file:

| File | Apa itu | Ambil jika |
| --- | --- | --- |
| `WinAppMigrator-v0.7.1.exe` | Aplikasi itu sendiri. Satu file, tanpa ada yang perlu diinstal. | Anda hanya ingin menggunakannya. Inilah file yang diinginkan kebanyakan orang. |
| `WinAppMigrator-v0.7.1-windows-x64.zip` | Aplikasi yang sama, sebagai `winappmigrator.exe`, dengan README, log perubahan, lisensi, dan lisensi pihak ketiga | Anda juga menginginkan dokumen-dokumennya |
| `SHA256SUMS.txt` | Checksum dari dua file di atas | Anda ingin memeriksa unduhan Anda |

Bagian penginstal yang masuk ke dalam setiap penginstal, yaitu [stub penginstal](Glossary#installer-stub), tidak pernah diterbitkan tersendiri. Bagian itu sudah ada di dalam aplikasi.

<a id="check-the-download"></a>

## Periksa unduhan

Ini opsional, tetapi layak dilakukan. Setiap pemeriksaan di bawah berdiri sendiri. Jalankan di PowerShell, di folder tempat Anda menyimpan file.

**1. Checksum.** [Hash](Glossary#hash) adalah sidik jari sebuah file. Jika satu byte saja berubah, hash-nya ikut berubah.

```powershell
Get-FileHash .\WinAppMigrator-v0.7.1.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

Angka panjang dari `Get-FileHash` harus sama dengan angka di baris untuk file tersebut. `Get-FileHash` mencetak huruf kapital, sedangkan file itu menggunakan huruf kecil. Itu tidak masalah: bandingkan huruf dan angkanya.

**2. Tanda tangan.** [Tanda tangan](Glossary#signature) menyatakan siapa yang membuat file, dan bahwa file itu tidak berubah sejak saat itu.

```powershell
$signature = Get-AuthenticodeSignature .\WinAppMigrator-v0.7.1.exe
$signature.Status                     # should say Valid
$signature.SignerCertificate.Subject  # should include CN=Randy Northrup
```

Anda juga dapat mengklik kanan file, memilih **Properti**, lalu melihat tab **Tanda Tangan Digital**. Versi 0.5.0 adalah rilis pertama yang aplikasinya ditandatangani. Unduhan dari rilis sebelumnya tidak ditandatangani.

**3. Tempat pembuatannya.** Setiap rilis sejak 0.5.0 mencatat bahwa alur kerja rilis milik GitHub sendiri yang membuatnya, dari repositori ini. Catatan ini disebut [atestasi](Glossary#attestation). Jika Anda memiliki [GitHub CLI](https://cli.github.com/), jalankan:

```powershell
gh attestation verify .\WinAppMigrator-v0.7.1.exe --repo RandyNorthrup/WinAppMigrator
```

Hal yang sama berlaku untuk file zip.

<a id="start-it"></a>

## Jalankan

1. **Klik dua kali** `WinAppMigrator-v0.7.1.exe`. Tidak ada yang perlu diinstal. Aplikasi berjalan dari mana pun Anda menyimpannya.
2. **SmartScreen.** [SmartScreen](Glossary#smartscreen) Windows mungkin menampilkan layar "Windows melindungi PC Anda" untuk file yang jarang dilihatnya. Jika demikian, pilih **Info selengkapnya** dan pastikan penerbitnya adalah Randy Northrup sebelum Anda memilih **Tetap jalankan**.
3. **Permintaan administrator.** Windows menanyakan apakah aplikasi boleh membuat perubahan. Ini adalah [permintaan UAC](Glossary#uac-prompt). Aplikasi selalu meminta, karena membaca drive PC lain, dan file yang hanya boleh dibaca administrator, memerlukan hak administrator. Untuk unduhan yang ditandatangani, permintaan itu menyebutkan **Randy Northrup** sebagai penerbit terverifikasi. Jika tertulis "Penerbit tidak dikenal" untuk file ini, file itu bukan file rilis yang ditandatangani, jadi jangan jalankan.
4. **Jendela terbuka.** Jika Anda malah melihat pesan bahwa jendela tidak dapat dibuka dan memerlukan Direct3D 12, lihat [Pemecahan masalah](Troubleshooting#the-window-could-not-open). Aplikasi tidak dapat berjalan tanpa jendelanya.

<a id="first-run"></a>

## Saat pertama kali dijalankan

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-dark.png">
    <img alt="Aplikasi dengan Fabrikam Data Sync dipilih" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-light.png" width="860">
  </picture>
</p>

- Daftar aplikasi yang diinstal ada di sebelah kiri. Saat daftar dimuat, label kecil di kanan atas menampilkan **Memuat**, lalu berapa banyak aplikasi yang ditemukan, misalnya **5 aplikasi**.
- Jendela dimulai dalam mode terang atau gelap, sesuai mode yang digunakan Windows. Tombol matahari atau bulan di kanan atas mengubahnya, dan aplikasi mengingat pilihan Anda. Lihat [File yang disimpannya di PC](Files-It-Keeps-on-a-PC). Dalam tema kontras Windows, aplikasi menggunakan warna tema itu dan tidak memiliki tombol tersebut.
- Aplikasi menggunakan bahasa tampilan Windows Anda jika bahasa itu termasuk dalam 24 bahasanya, dan bahasa Inggris jika tidak. **Bahasa**, di kanan atas, mencantumkan bahasa-bahasa itu dengan namanya sendiri dan langsung mengubah jendela; aplikasi mengingat pilihan Anda. Bahasa Arab, Persia, Urdu, dan Ibrani ditata dari kanan ke kiri. Lihat [Bahasa dan aksesibilitas](Languages-and-Accessibility).
- Klik sebuah aplikasi. Sisi kanan menampilkan apa yang akan masuk ke penginstalnya, dan langsung mulai menentukannya. Tidak ada yang dibuat sampai Anda mengklik **Buat penginstal**.

Berikutnya: [Membuat penginstal](Building-an-Installer).
