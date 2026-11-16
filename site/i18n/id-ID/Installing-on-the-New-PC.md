<a id="installing-on-the-new-pc"></a>

# Menginstal di PC baru

Penginstal terdiri dari satu jendela. Penginstal memeriksa dirinya sendiri, menunjukkan kepada Anda di mana aplikasi akan diinstal dan semua yang akan disiapkannya, dan hanya menginstal ketika Anda mengklik **Instal**. Halaman ini membahasnya dari atas ke bawah, menggunakan kata-kata yang ada di layar.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-dark.png">
    <img alt="Penginstal untuk Fabrikam Data Sync, menampilkan bahwa penginstal tidak ditandatangani, lokasi instalasi, ruang yang diperlukan, dan apa yang disiapkannya selain file" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-light.png" width="600">
  </picture>
</p>

<a id="before-you-start"></a>

## Sebelum Anda mulai

- **Salin seluruh penginstal.** Jika proses pembuatan memberi Anda file `.wamdata` selain file `.exe`, salin keduanya ke folder yang sama, dan jangan ubah namanya. Lihat [Aplikasi besar](Big-Apps).
- **Jalankan hanya penginstal yang Anda percayai.** Penginstal berjalan dengan [hak administrator](Glossary#administrator-rights) penuh. Perlakukan penginstal seperti program lain: jalankan hanya jika Anda yang membuatnya, atau Anda mendapatkannya dari orang yang Anda percayai. Lihat [Privasi dan keamanan](Privacy-and-Security).
- **Anda memerlukan hak administrator** di PC ini, dan [Direct3D 12](Glossary#direct3d-12) untuk jendelanya. Tanpa Direct3D 12 Anda tetap dapat menginstal dari prompt perintah. Lihat [Baris perintah](Command-Line).

<a id="open-it"></a>

## Membuka penginstal

Klik dua kali penginstal. Windows menampilkan [permintaan UAC](Glossary#uac-prompt). Penginstal tidak ditandatangani kecuali pembuatnya menandatanganinya, jadi permintaan itu biasanya menyebutkan **Penerbit tidak dikenal**. Itu wajar. Lihat [Privasi dan keamanan](Privacy-and-Security#signing).

Jendela "Penginstal WinAppMigrator", "Menginstal ulang aplikasi yang diambil dari PC lain", langsung terbuka, seberapa pun besarnya penginstal. Jendela menampilkan "Memeriksa penginstal…" selama memeriksa [hash](Glossary#hash) seluruh paketnya. Pemeriksaan ini membaca seluruh paket, jadi penginstal yang besar memerlukan waktu. Sejak saat ini penginstal tetap membuka file-nya sendiri, dan file datanya jika ada, sehingga keduanya tidak dapat diubah atau ditukar selama jendela terbuka.

Pemeriksaan dapat berhenti di sini, dengan pesan berwarna merah. Yang umum adalah:

- Penginstal rusak atau diubah setelah dibuat: salin lagi dari tempat penginstal itu dibuat.
- File datanya tidak ada, atau bukan file yang benar: salin lagi kedua file, ke dalam satu folder.
- "Paket penginstal ini tidak dapat digunakan di PC ini", misalnya karena program aplikasi dibuat untuk jenis prosesor lain, atau memerlukan Windows yang lebih baru. Tidak ada yang diubah.

Semua ini dijelaskan di [Pemecahan masalah](Troubleshooting#installing).

Ketika pemeriksaan lulus, judul jendela berubah menjadi "Instal" dan nama aplikasi.

<a id="its-language"></a>

### Bahasanya

Penginstal menggunakan bahasa PC tempatnya dijalankan, bukan PC tempatnya dibuat: bahasa tampilan Windows jika termasuk dalam 24 bahasa yang dimiliki WinAppMigrator, dan bahasa Inggris jika tidak. **Bahasa**, di kanan atas, mengubahnya untuk sesi ini. Apa yang dicantumkannya tentang aplikasi, termasuk catatan yang ditulis WinAppMigrator saat membuat penginstal itu, juga ditampilkan dalam bahasa tersebut. Penginstal yang dibuat dengan WinAppMigrator 0.6 atau yang lebih lama menampilkan catatan itu dalam bahasa Inggris, sebagaimana catatan itu ditulis. [Instalasi senyap dari baris perintah](Command-Line) selalu menjawab dalam bahasa Inggris, agar dapat dibaca oleh skrip.

<a id="the-app-and-where-it-goes"></a>

## Aplikasi dan lokasinya

Kartu pertama berisi nama aplikasi, dengan penerbit dan versi di bawahnya, misalnya "Fabrikam Inc. · 1.7.3". Lalu:

| Baris | Isinya |
| --- | --- |
| **Diambil dari** | Nama PC asal aplikasi. Baris ini hanya muncul jika penginstal dibuat dengan opsi **Data pengguna** aktif. |
| **Penginstal** | Apakah file penginstal ini [ditandatangani](Glossary#signature): "Memeriksa tanda tangannya…", lalu "Tidak ditandatangani: penerbit tidak dikenal", atau "Ditandatangani oleh" dan nama penanda tangan. Tanda tangan yang tidak valid menampilkan pesan merah sebagai gantinya: "Tanda tangan penginstal ini tidak valid", dengan alasan dari Windows, dan "Penginstal mungkin telah diubah sejak ditandatangani." |
| **Lokasi instalasi** | Folder tempat aplikasi diinstal, dengan **Telusuri** untuk memilihnya. Awalnya berisi folder tempat aplikasi berada di PC lama. Aplikasi dari Windows 32-bit dimulai di `Program Files (x86)`, tempat Windows 64-bit menyimpan aplikasi 32-bit. Jika folder itu tidak dapat digunakan di sini, pemberitahuan merah di bawahnya menjelaskan alasannya. |
| **Ruang yang diperlukan** | Berapa banyak ruang yang diperlukan instalasi di drive itu, dan berapa yang kosong, misalnya "hingga 824,0 MB di C: (356,0 GB kosong)". Angka ini mencakup paket yang sudah diekstrak serta file aplikasi. |

Nama penerbit di baris pertama berasal dari paket, jadi siapa pun dapat menulisnya. Baris **Penginstal** berasal dari Windows, dan itulah yang dapat dipercaya.

<a id="when-the-location-cannot-be-used"></a>

### Jika lokasi tidak dapat digunakan

Jika folder di **Lokasi instalasi** tidak dapat digunakan, pemberitahuan merah di bawahnya menjelaskan alasannya. **Instal** dan **Pratinjau perubahan** tetap tidak aktif sampai Anda memilih folder lain, dan sebuah baris di sampingnya berbunyi "Instal dan Pratinjau perubahan tidak aktif sampai Lokasi instalasi dapat digunakan."

Alasan yang paling umum adalah folder itu sudah berisi file. Folder yang berisi file hanya digunakan jika instalasi WinAppMigrator sebelumnya yang menempatkan file tersebut di sana, sehingga file aplikasi lain tidak pernah diganti:

| Pemberitahuan | Artinya | Yang harus dilakukan |
| --- | --- | --- |
| Nama aplikasi, lalu "sudah diinstal di folder ini, jadi salinan ini tidak dapat ditempatkan di sini." | Aplikasi itu sudah diinstal di PC ini, sering kali oleh penginstalnya sendiri: program utamanya ada di folder itu, atau daftar aplikasi yang diinstal di Windows menyebutkannya di sana. Ini terjadi pada aplikasi yang sudah dimiliki banyak PC, seperti peramban web, pemutar media, atau Office. | Hapus dulu instalan salinan yang ada di PC ini, di Pengaturan Windows pada **Aplikasi**, lalu buka penginstal lagi. Atau ketik folder baru yang kosong di **Lokasi instalasi** untuk menginstal salinan ini di sampingnya. |
| Nama aplikasi lain, lalu "diinstal di folder ini", dan bahwa aplikasi ini "tidak dapat ditempatkan di sini." | Daftar aplikasi yang diinstal di Windows menyebutkan aplikasi lain di folder itu, misalnya edisi Office yang lain. | Ketik folder baru yang kosong di **Lokasi instalasi**. Atau, jika Anda tidak lagi memerlukan aplikasi lain itu, hapus dulu instalannya, di Pengaturan Windows pada **Aplikasi**, lalu buka penginstal lagi. |
| "Folder ini sudah berisi file yang tidak diinstal oleh WinAppMigrator." | Ada sesuatu yang lain di folder itu, dan tidak ada aplikasi terinstal yang tercantum di sana. Mungkin itu milik aplikasi lain. | Ketik folder baru yang kosong di **Lokasi instalasi**. Jika file tersebut adalah sisa dari aplikasi yang sudah Anda hapus instalannya, Anda dapat menghapus folder itu, lalu membuka penginstal lagi. |

Untuk alasan lain, pemberitahuan berbunyi "Folder ini tidak dapat digunakan:", alasannya, dan "Masukkan folder lain di atas." Alasannya adalah:

| Alasan di layar | Yang harus dilakukan |
| --- | --- |
| "harus berupa jalur lengkap" | Ketik jalur yang diawali huruf drive, misalnya `C:\Program Files\Fabrikam Data Sync`. |
| "tidak boleh berupa seluruh drive" | Gunakan folder di drive itu, bukan drive itu sendiri. |
| "tidak boleh berupa folder Windows" | Pilih folder di luar `C:\Windows`. |
| "folder ini berisi hal lain secara berdampingan, jadi aplikasi memerlukan folder di dalamnya" | Tambahkan folder untuk aplikasi, misalnya `C:\Program Files\Fabrikam Data Sync`, bukan `C:\Program Files`. |
| "ini adalah folder milik Windows, atau folder yang isinya berjalan atau dimuat dengan sendirinya" | Pilih folder biasa. Folder Startup, misalnya, ditolak. |
| "harus berupa jalur biasa di drive PC ini" | [Folder bersama di jaringan](Glossary#network-share) ditolak, begitu pula nama yang akan dibaca Windows sebagai sesuatu yang lain. |
| bahwa folder itu "adalah tautan ke tempat lain, dan orang lain mungkin dapat mengubah tujuannya" | Gunakan folder yang dituju [tautan](Glossary#link) itu. |
| "ini adalah tempat instalasi menyimpan catatannya" | Pilih folder di luar `%ProgramData%\WinAppMigrator`. |

Jika folder berisi file dan instalasi sebelumnya dari WinAppMigrator **memang** tercatat di sana, penginstal akan menggunakannya, dan memperingatkan: "Folder ini sudah berisi file. File dengan nama yang sama akan diganti, dan dikembalikan jika Anda menghapus instalan."

Setelah folder diterima, jendela akan mengganti file, nilai registri, layanan, tugas, pintasan, atau variabel dengan nama yang sama, seperti yang dilakukan `--force` di [baris perintah](Command-Line#what---force-does). Apa yang sudah menjadi milik aplikasi lain, seperti layanannya atau jenis filenya, tetap menjadi milik aplikasi itu.

Jika drive tampak kekurangan ruang, catatan berwarna amber berbunyi "Ruang di sini mungkin tidak cukup" dan berapa yang diperlukan. Paket dapat diekstrak di drive lain, tetapi file aplikasi tetap harus muat di drive ini. Instalasi itu sendiri memeriksa ruang lagi sebelum dimulai.

<a id="an-earlier-install-that-never-finished"></a>

### Instalasi sebelumnya yang tidak pernah selesai

Jika instalasi sebelumnya oleh salah satu penginstal ini terputus, karena crash atau listrik padam, catatan berwarna amber berbunyi "Instalasi" aplikasi itu "sebelumnya tidak pernah selesai. Instal akan mengurungkan instalasi itu terlebih dahulu." Hal itu terjadi secara otomatis saat Anda mengklik **Instal**.

<a id="what-this-sets-up-beyond-files"></a>

## Yang disiapkan selain file

Kartu ini adalah daftar yang Anda setujui saat mengklik **Instal**. Di bawah judulnya tertulis:

> Semua ini dilakukan dengan hak administrator. Jika instalasi gagal, apa yang diubahnya akan dikembalikan, dan Anda akan diberi tahu tentang apa pun yang tidak dapat dikembalikan. Anda juga dapat mengurungkan instalasi ini nanti dari Pengaturan, di Aplikasi yang diinstal. File yang digantinya akan dikembalikan, dan runtime yang diinstalnya tetap ada.

Setiap baris memiliki nama, label di sebelah kanan yang menyatakan jenisnya, dan satu baris yang menjelaskan fungsinya. Label berwarna biru seperti biasa menandakan hal yang biasa. Label berwarna **amber** menandai sesuatu yang berjalan dengan hak penuh, seperti layanan yang berjalan sebagai LocalSystem, driver, atau aturan firewall yang mengizinkan sambungan masuk. Periksa baris-baris itu dua kali.

Jika daftar kosong, tertulis "Tidak ada: instalasi ini hanya menyalin file, serta pengaturan di kunci registri milik aplikasi itu sendiri."

Daftar ini ditentukan untuk lokasi yang ada di kotak. Jika Anda mengubah lokasi, sejenak tertulis "Menentukan apa yang akan disiapkan di lokasi ini…", dan **Instal** menunggu.

<a id="what-each-label-means"></a>

### Arti setiap label

| Label | Apa itu | Contoh baris di bawahnya |
| --- | --- | --- |
| **Layanan** | [Layanan](Glossary#service): program yang dijalankan Windows di latar belakang | "Menjalankan FabrikamSync.exe sebagai NT AUTHORITY\LocalService, dimulai sekarang dan setiap kali PC ini dinyalakan". Yang lain berbunyi "dimulai sekarang dan tidak lama setelah setiap kali PC ini dinyalakan", "dimulai hanya jika ada yang memintanya", atau "dinonaktifkan". |
| **Driver** | [Driver](Glossary#driver) milik aplikasi | "Memuat" sebuah file "ke dalam Windows itu sendiri". Selalu amber. Belum ada pengujian proyek yang memuat driver. |
| **Tugas terjadwal** | [Tugas terjadwal](Glossary#scheduled-task) | "Berjalan sebagai SYSTEM", atau sebagai sebuah akun. "dengan hak tertinggi yang dapat dimilikinya" ditandai amber. Tugas yang berjalan sebagai seseorang di PC lama akan berjalan sebagai siapa pun yang menginstalnya. |
| **Mulai otomatis** | [Item startup](Glossary#startup-item) | "Berjalan saat Anda masuk", "Berjalan untuk semua orang yang masuk", atau "Pintasan di folder Startup, untuk Anda" |
| **Pintasan** | [Pintasan](Glossary#shortcut) di Desktop atau di menu Mulai | "Membuka Fabrikam.exe, untuk semua orang di PC ini" atau "untuk Anda" |
| **PATH** | Entri yang ditambahkan ke [variabel Path](Glossary#path-variable) | "Menambahkan ke Path" dan "Folder instalasi\bin". Entri ditambahkan ke isi yang sudah ada, tidak pernah menggantikannya. |
| **Variabel lingkungan** | [Variabel lingkungan](Glossary#environment-variable) | "Diatur ke" nilainya |
| **Ekstensi shell** | Kode yang dimuat ke File Explorer | "Memuat" sebuah file "ke File Explorer, untuk semua orang di PC ini". Amber. |
| **Perintah menu** | Perintah di menu klik kanan | "Menambahkan perintah ke menu" suatu jenis file. "Mengubah apa yang terjadi saat salah satu dari jenis ini dibuka" berwarna amber. |
| **Jenis tautan** | [Protokol tautan](Glossary#link-protocol), seperti `fabrikam:` | "Tautan jenis ini dibuka dengan aplikasi ini" |
| **Jenis file** | [Jenis file](Glossary#file-type), seperti `.fds` | "File jenis ini dibuka dengan aplikasi ini" |
| **COM** | [Kelas COM](Glossary#com-class) yang dapat dimulai oleh program lain | "COM di" sebuah file, dengan jumlah pengaturannya. "Izin COM" berwarna amber. |
| **Aturan firewall** | [Aturan firewall](Glossary#firewall-rule) untuk salah satu programnya | "Mengizinkan sambungan masuk di port 8080 untuk FabrikamSync.exe, di semua jaringan". Mengizinkan sambungan masuk berwarna amber. |
| **Sumber log peristiwa** | Nama yang digunakan aplikasi untuk menulis ke log peristiwa Windows | "Dapat menulis ke log Application, dengan pesan dari file-nya sendiri" |
| **Font** | Font di folder instalasi | "Dimuat dari folder instalasi saat Anda masuk" |
| **Driver ODBC** | Driver database untuk [ODBC](Glossary#odbc) | "Dimuat ke dalam program yang menggunakan ODBC untuk menjangkau database". Amber. |
| **Sumber data ODBC** | Sambungan database yang disimpan | "Sambungan database, untuk semua orang di PC ini" |
| **Pengaturan kompatibilitas** | Cara Windows memulai beberapa programnya | "Cara programnya dimulai" |
| **Aplikasi default** | Entri yang memungkinkan Anda memilih aplikasi itu di Aplikasi default | "Dapat dipilih di Aplikasi default. Tidak ada yang dialihkan kepadanya" |
| **Nama program** | Nama yang dapat digunakan dialog Jalankan dan program lain untuk menemukannya | "Dialog Jalankan dan program lain dapat menemukannya dengan nama ini" |
| **Pengaturan lainnya** | Nilai [registri](Glossary#registry) di luar kunci milik aplikasi itu sendiri dan di luar jenis-jenis di atas | "3 pengaturan di luar kunci milik aplikasi itu sendiri", "Di" kunci-kunci tersebut. Amber jika ada yang berada di kunci milik Microsoft. |

Beberapa hal hanya dapat dinilai ketika instalasi sampai pada hal tersebut, seperti layanan dengan nama yang sama yang sudah menjadi milik aplikasi lain. Hal-hal itu tetap ada di daftar sesuai yang diminta. Ketika instalasi sampai di sana, instalasi membiarkannya, dan memberitahukannya.

<a id="notes-and-what-is-left-out"></a>

### Catatan dan apa yang tidak disertakan

Di bawah baris-baris itu, catatan berwarna amber menyampaikan apa yang perlu Anda ketahui, misalnya:

- bahwa kredensial tidak dimasukkan ke penginstal, jadi Anda mungkin harus memasukkannya lagi;
- bahwa opsi **Data lisensi dan aktivasi** atau **Data pengguna** nonaktif saat penginstal dibuat, jadi data tersebut tidak ada di dalamnya;
- [sertifikat](Glossary#certificate) mana yang tampaknya diandalkan aplikasi, yang tidak pernah diinstal;
- apa yang tidak disertakan oleh pembuat penginstal, dan apa yang ditambahkannya;
- bahwa aplikasi mungkin tidak berfungsi di sini: "Aplikasi ini berasal dari Windows build" yang lebih baru, atau bahwa Windows akan menjalankan beberapa programnya "melalui emulator";
- bahwa file dari Dokumen atau Game Tersimpan akan "ditempatkan di bawah" folder instalasi, "di folder bernama _external", karena folder itu di PC ini berada di luar profil Anda.

Lalu, dengan warna amber, setiap hal yang tidak disertakan oleh aturan keamanan, beserta alasannya. 20 yang pertama ditampilkan lengkap. Sisanya ada di bawah "lainnya tidak disertakan", yang dapat Anda buka. Bagian lain instalasi tetap berjalan tanpa hal-hal tersebut. Apa yang ditolak, dan alasannya, ada di [Isi penginstal](What-Goes-in-an-Installer#what-the-installer-refuses).

Jika paket sama sekali tidak dapat ditempatkan di lokasi yang dipilih, pesan merah berbunyi "Paket ini tidak dapat ditempatkan di lokasi ini" beserta alasannya.

<a id="options"></a>

## Opsi

Kartu ini hanya muncul jika penginstal berisi data pengguna atau data lisensi. Setiap kotak awalnya dicentang.

| Opsi | Isinya |
| --- | --- |
| **Data pengguna** | "Aktif: pengaturan, Dokumen, dan Game Tersimpan ditempatkan di folder biasa milik siapa pun yang sedang masuk. Nonaktif: semuanya ditempatkan di dalam folder instalasi, dan tidak ada yang disiapkan untuk berjalan dari sana." Jika penginstal menyebutkan milik siapa data yang disimpannya, sebuah baris di bawahnya berbunyi "Ini berisi data milik" akun tersebut. |
| **Lisensi** | "File aktivasi dan lisensi, serta nilai registri yang menyimpan lisensi. Beberapa di antaranya mungkin perlu diaktifkan ulang di sini." |

Mengubah sebuah kotak langsung mengubah daftar di atas agar sesuai.

Data pengguna selalu ditujukan untuk **siapa pun yang sedang masuk** ke Windows, meskipun nama administrator lain diketik di permintaan UAC. Jika penginstal tidak dapat mengetahui siapa orangnya, penginstal memberitahukannya, tidak menyertakan pengaturan per pengguna, dan menempatkan data pengguna di bawah folder instalasi alih-alih menebak. Jika tidak ada yang masuk ke sesi itu, data pengguna dan pengaturan per pengguna ditujukan untuk akun yang menjalankan penginstal, dan penginstal juga memberitahukannya.

<a id="dependencies"></a>

## Dependensi

"Apa yang diperlukan aplikasi ini, dan bagaimana masing-masing disediakan." Setiap [runtime](Glossary#runtime) memiliki label:

| Label | Arti |
| --- | --- |
| **Disertakan** | Ada di dalam penginstal. Runtime ini diinstal tanpa internet, kecuali DirectX End-User Runtime, yang penginstalnya mengambil runtime itu dari Microsoft. Dari semua ini, hanya runtime Visual C++ x64 yang benar-benar pernah diinstal dalam pengujian proyek. |
| **Diinstal dengan winget:** dan sebuah ID | Runtime tidak dapat diunduh saat penginstal dibuat, jadi penginstal menggunakan [winget](Glossary#winget), yang memerlukan internet. Cara ini belum dijalankan dalam pengujian proyek. |
| **Fitur Windows: NetFx3** | .NET Framework 3.5, diaktifkan sebagai [fitur Windows](Glossary#windows-feature). Ini dapat memerlukan internet. Cara ini belum dijalankan dalam pengujian proyek. |
| **Bawaan Windows** | .NET Framework 4, yang sudah dimiliki Windows 10 dan 11 |
| **Bawaan aplikasi** | Aplikasi membawa salinannya sendiri |
| **Instal secara manual** | Tidak ada penginstal untuk runtime ini di sini. Instal sendiri. |

Jika tidak ada, tertulis "Tidak ada dependensi runtime tambahan yang terdeteksi." Lihat [Runtime](Runtimes).

<a id="preview-changes"></a>

## Pratinjau perubahan

**Pratinjau perubahan** menentukan dengan tepat apa yang akan dilakukan instalasi, tanpa mengubah apa pun. Fitur ini memeriksa apakah ruang cukup, mengekstrak paket ke folder privat, menentukan perubahannya, lalu menghapus folder itu lagi. Kartu hasilnya berjudul "Pratinjau: belum ada yang diubah", dengan **Perubahan yang direncanakan**, dan daftar perubahan, dengan layanan, tugas, dan pintasan di urutan pertama. Jendela mencantumkan 300 yang pertama, lalu menyebutkan berapa banyak lagi.

Pratinjau tidak memeriksa hash setiap file. Hash seluruh paket sudah diperiksa saat jendela dibuka.

<a id="install"></a>

## Instal

**Instal** tetap tidak aktif sampai lokasi dapat digunakan dan daftar sudah ditentukan untuk lokasi itu. Jika paket sama sekali tidak dapat ditempatkan di lokasi itu, sebuah baris di sampingnya berbunyi "Instal tidak aktif karena paket ini tidak dapat ditempatkan di lokasi ini.", dan alasannya tertera dengan warna merah di **Yang disiapkan selain file**. Saat Anda mengkliknya:

1. Penginstal menentukan daftar sekali lagi. Jika apa yang akan dilakukannya telah berubah sejak Anda melihatnya, penginstal tidak menginstal apa pun, menampilkan daftar baru, dan berbunyi "Apa yang akan dilakukan di sini telah berubah, jadi tidak ada yang diinstal. Periksa daftar di bawah, lalu klik Instal lagi jika memang itu yang Anda inginkan."
2. Penginstal memeriksa apakah ruang cukup di setiap drive yang ditulisinya.
3. Penginstal mengekstrak paket ke folder yang hanya dapat digunakan administrator, sambil memeriksa hash setiap file saat dikeluarkan.
4. Penginstal menginstal, dengan terlebih dahulu mencatat setiap perubahan dalam [jurnal](Glossary#journal), agar dapat diurungkan.
5. Penginstal membaca kembali setiap file yang diinstalnya dan memeriksa hash-nya.
6. Penginstal memulai layanan yang diatur untuk dimulai dengan sendirinya.

Di bagian bawah Anda melihat apa yang sedang dilakukannya, hitungan jika ada, waktu yang telah berlalu, bilah kemajuan, dan **Batal**. Langkah yang mungkin Anda lihat adalah "Bersiap…", "Mengekstrak", "Memeriksa paket", "Mengurungkan instalasi sebelumnya yang tidak pernah selesai", "Menginstal runtime", "Menyalin file", "Menyiapkan layanan", "Menyiapkan driver", "Menulis pengaturan registri", "Memulihkan data lisensi", "Menyiapkan tugas terjadwal", "Membuat pintasan", "Mengatur variabel lingkungan", "Menambahkan aplikasi ke Pengaturan", dan "Memeriksa apa yang diinstal".

**Batal** menampilkan "Membatalkan setelah langkah saat ini…", lalu "Mengembalikan apa yang diubah". Menutup jendela saat penginstal sedang menginstal sama dengan mengklik **Batal**: jendela tetap terbuka sampai semuanya dikembalikan. Jika belum ada yang diubah, tertulis "Dibatalkan. Tidak ada yang diubah."

<a id="the-hash-check"></a>

## Pemeriksaan hash

Penginstal memeriksa [hash](Glossary#hash) tiga kali:

1. **Saat jendela dibuka**: hash seluruh paket. Salinan yang terpotong, atau yang diedit setelah dibuat, ditolak sebelum apa pun di dalamnya dibaca.
2. **Saat mengekstrak**: hash setiap file, sebelum apa pun ditulis ke tempatnya.
3. **Setelah menginstal**: penginstal membaca kembali setiap file yang diinstalnya dan memeriksanya lagi.

Hash mendeteksi kerusakan, dan perubahan oleh siapa pun yang tidak sekaligus memperbarui hash-nya. Hash tidak membuktikan siapa yang membuat penginstal. Hanya [tanda tangan](Glossary#signature) yang dapat membuktikannya.

<a id="the-result"></a>

## Hasil

Kartu hasil muncul di bagian atas, berjudul "Hasil instalasi".

| Pesan | Artinya |
| --- | --- |
| "Aplikasi telah diinstal dan lulus pemeriksaan hash." | Semuanya baik. Jika ada yang tidak disertakan oleh aturan, pesan itu ditambah "tanpa apa yang tercantum sebagai tidak disertakan di bawah." |
| "Instalasi dibatalkan, dan apa yang diubahnya telah dikembalikan." | Anda membatalkannya. |
| "Instalasi gagal, dan apa yang diubahnya telah dikembalikan." | Ada yang gagal. Apa yang diubahnya telah dikembalikan, kecuali runtime yang diinstalnya. Apa pun yang tercantum dengan warna amber di bawahnya dibiarkan, beserta alasannya, misalnya layanan yang baru hilang saat PC dimulai ulang. |
| "Instalasi gagal, dan tidak semua yang diubahnya dapat dikembalikan. Sisanya tercantum di bawah, dan instalasi berikutnya akan mencoba lagi." | Baca daftarnya. Menjalankan penginstal lagi akan mencoba mengembalikannya lagi. |
| "Instalasi tidak selesai dengan bersih. Tinjau masalah di bawah." | Baca pesan di bawahnya. |

Lalu tiga ubin: **Perubahan yang dibuat** (atau **Perubahan yang dikembalikan** setelah kegagalan), **Peringatan**, dan **Kesalahan**. Label menunjukkan **Hash paket valid** dan **File yang diinstal valid**, atau **TIDAK VALID** dengan warna merah. Hingga lima peringatan dan setiap kesalahan ditampilkan sebagai pesan. "Sebuah dependensi perlu memulai ulang PC untuk menyelesaikannya. Mulai ulang Windows, lalu jalankan aplikasi." berarti sebuah runtime meminta PC dimulai ulang.

**Perubahan**, dengan hitungan, membuka daftar apa yang telah dilakukan, hingga 300. Jika instalasi tidak selesai dengan bersih, jalur [log](Glossary#log)-nya ditampilkan sebagai **Log**.

Setelah instalasi yang bersih, tombol di bagian bawah berubah menjadi **Tutup** dan **Buka folder instalasi**.

<a id="where-things-end-up"></a>

## Ke mana semuanya ditempatkan

- **File program** ditempatkan di lokasi instalasi.
- **Data pengguna**, jika kotak itu dicentang, ditempatkan di AppData, Dokumen, dan Game Tersimpan milik akun yang sedang masuk, serta di ProgramData. Folder Dokumen atau Game Tersimpan yang disimpan PC ini di luar profil Anda tidak ditulisi. File tersebut ditempatkan di bawah lokasi instalasi, di `_external`, dan daftar menyebutkannya terlebih dahulu. Jika kotak tidak dicentang, semua data pengguna ditempatkan di bawah lokasi instalasi.
- **Pintasan** ditempatkan di Desktop dan menu Mulai PC ini: milik Anda, atau milik semua orang jika memang di sanalah pintasan itu sebelumnya.
- **Path** hanya ditambahi entri milik aplikasi. Tidak ada hal lain di dalamnya yang berubah.
- **Apa yang sudah dimiliki aplikasi lain di sini tetap menjadi milik aplikasi itu.** Jenis file, jenis tautan, kelas COM, layanan, sumber log peristiwa, font, atau hal serupa yang sudah menjadi milik sesuatu yang lain dibiarkan, dan penginstal memberitahukannya. Aplikasi yang semula membuka tautan `http`, misalnya, harus diatur lagi sebagai default di Pengaturan.
- **Pengaturan** mendapatkan entri untuk aplikasi itu, sehingga Anda dapat menghapus instalannya seperti aplikasi lain. Lihat [Mengurungkan instalasi](Undoing-an-Install).

<a id="common-reasons-it-refuses"></a>

## Alasan umum penginstal menolak

| Yang Anda lihat | Penyebab | Yang harus dilakukan |
| --- | --- | --- |
| "aplikasi ini tidak dapat berjalan di PC ini, jadi tidak ada yang diubah" | Programnya dibuat untuk jenis prosesor lain, atau memerlukan Windows yang lebih baru | Instal di PC yang sesuai dengan PC lama |
| "ruang tidak cukup untuk ini, jadi tidak ada yang diubah" | Ruang kosong tidak cukup di drive yang ditulisinya | Kosongkan ruang, atau pilih lokasi di drive lain |
| "instalasi atau penghapusan instalan lain sedang berjalan" | Hanya satu yang dapat berjalan dalam satu waktu | Tunggu hingga selesai, lalu coba lagi |
| Sebuah file "memiliki nama yang dibuka oleh versi Windows ini sebagai perangkat, bukan sebagai file" | Aplikasi memiliki file bernama seperti `aux.h`, yang tidak dapat disimpan oleh Windows sebelum Windows 11 | Instal di Windows 11 |
| Folder data seperti `%AppData%` berada di bawah tautan | Penginstal tidak akan menulis melalui tautan | Hapus centang **Data pengguna** dan **Lisensi**, lalu instal tanpa keduanya |

Lainnya ada di [Pemecahan masalah](Troubleshooting#installing).
