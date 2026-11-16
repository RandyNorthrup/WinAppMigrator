<a id="troubleshooting"></a>

# Pemecahan masalah

Temukan apa yang Anda lihat di bawah, berdasarkan apa yang terjadi atau berdasarkan kata-kata di layar. Kata-kata dalam tanda kutip adalah yang ditampilkan program, terkadang dengan nama atau angka di tengahnya.

Jika tidak ada yang membantu di sini, lihat [Mendapatkan bantuan](#getting-help) di bagian akhir.

<a id="the-window-could-not-open"></a>

## Jendela tidak dapat dibuka

**Yang Anda lihat.** Kotak pesan, bukan jendela:

- dari aplikasi: "Jendela tidak dapat dibuka", dan "Program ini memerlukan Direct3D 12 untuk menggambar jendelanya, yang mungkin tidak dimiliki oleh driver grafis lama atau mesin virtual."
- dari penginstal: hal yang sama, dan "Penginstal masih dapat menginstal tanpa jendela: jalankan file ini dari prompt perintah administrator dengan --apply, dan --target-root diikuti sebuah folder untuk memilih lokasinya."
- dari penghapus instalan: hal yang sama, dan "Penghapus instalan masih dapat menghapus instalan tanpa jendela, dari prompt perintah administrator:" dengan perintah yang tepat.

**Penyebab.** Jendela-jendela ini digambar dengan [Direct3D 12](Glossary#direct3d-12), dan tidak ada cara lain untuk menggambarnya.

**Yang harus dilakukan.**

- Perbarui driver grafis PC.
- Untuk penginstal atau penghapus instalan, gunakan baris perintah. Lihat [Baris perintah](Command-Line).
- Aplikasi tidak memiliki baris perintah. Buat penginstal di PC lain: Anda dapat menyambungkan drive PC lama ke PC tersebut. Lihat [Pindah dari drive yang tersambung](Moving-from-a-Connected-Drive). PC itu juga dapat membaca PC lama melalui jaringan. Lihat [Pindah dari PC melalui jaringan](Moving-over-the-Network).

<a id="starting-the-app"></a>

## Memulai aplikasi

| Yang Anda lihat | Artinya | Yang harus dilakukan |
| --- | --- | --- |
| Permintaan UAC menyebutkan "Penerbit tidak dikenal" untuk aplikasi | Sejak 0.5.0, aplikasi ditandatangani. Aplikasi yang tidak ditandatangani bukan file rilis, atau berasal dari sebelum 0.5.0. | Unduh lagi dari [rilis terbaru](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest), lalu [periksa](Getting-Started#check-the-download). |
| SmartScreen menampilkan "Windows melindungi PC Anda" | SmartScreen jarang melihat file itu | Pilih **Info selengkapnya**, pastikan penerbitnya adalah Randy Northrup, lalu pilih **Tetap jalankan**. |
| Label di bagian atas menampilkan **Gagal memuat**, dan daftar menampilkan "Tidak dapat mencantumkan aplikasi" | Daftar aplikasi tidak dapat dibaca | Klik panah melingkar untuk memuat ulang. Untuk drive yang tersambung, pastikan drive itu masih tersambung. |
| Aplikasi yang Anda inginkan tidak ada di daftar | Aplikasi itu adalah aplikasi Microsoft Store, bagian tersembunyi dari aplikasi lain, atau sama sekali tidak memiliki entri di Pengaturan, seperti aplikasi portabel | Aplikasi Store tidak dapat dipindahkan. Bagian tersembunyi mungkin ditawarkan di **Mungkin juga bagian darinya** saat Anda memilih aplikasi utamanya. Aplikasi tanpa entri di Pengaturan tidak dapat dipilih. |
| "Tidak ada aplikasi yang cocok" | Pencarian atau filter menyembunyikan semuanya | Kosongkan kotak pencarian, lalu pilih **Semua**. |

<a id="building"></a>

## Membuat penginstal

<a id="picking-a-drive-or-a-disk-image"></a>

### Memilih drive atau citra disk

| Yang Anda lihat | Yang harus dilakukan |
| --- | --- |
| Drive "tidak tampak seperti drive sistem Windows (tidak ada \Windows dan \Users). Pilih akar drive dari PC lain." | Pilih bagian teratas drive, seperti `E:\`, bukan folder di dalamnya. Pastikan itu adalah drive tempat Windows diinstal. |
| Sebuah citra "bukan citra disk yang dapat dilampirkan Windows (.vhd atau .vhdx)" | Pilih `.vhd` atau `.vhdx`. Simpan disk VMware atau VirtualBox sebagai salah satunya terlebih dahulu. Untuk checkpoint Hyper-V, `.avhdx`, ekspor disk mesin virtualnya. |
| Sebuah citra "tidak dapat dilampirkan: Windows sudah melampirkannya, dan apa pun yang melampirkannya mungkin sedang menulis ke sana." | Citra itu sedang terbuka di tempat lain, misalnya dipasang di File Explorer. Pilih drive-nya di **Drive yang tersambung…** sebagai gantinya, atau lepaskan citra itu, lalu pilih lagi. |
| Sebuah citra "tidak memiliki volume berisi Windows yang dapat dibaca. Volume yang dienkripsi dengan BitLocker harus dibuka kuncinya terlebih dahulu." | Pastikan itu adalah citra drive tempat Windows diinstal. Jika dikunci dengan [BitLocker](Glossary#bitlocker), pasang citra itu di Windows sendiri, buka kuncinya, lalu pilih drive-nya di **Drive yang tersambung…**. |
| Sebuah citra "adalah disk diferensial, yang hanya berisi perubahan sejak disk induknya" | Gabungkan ke disk induknya, dengan Edit Disk di Hyper-V, lalu pilih disk induk itu. |
| Sebuah citra "tidak dapat dilampirkan:" dan alasan lain | Alasan itu berasal dari Windows sendiri. Baca alasannya, pastikan tidak ada hal lain yang menggunakan citra itu, lalu coba lagi. |

<a id="reading-a-pc-over-the-network"></a>

### Membaca PC melalui jaringan

Daftar lengkapnya, termasuk apa yang ditampilkan klien di PC lama, ada di [Pindah dari PC melalui jaringan](Moving-over-the-Network#if-something-goes-wrong).

| Yang Anda lihat | Yang harus dilakukan |
| --- | --- |
| PC lama tidak tercantum di **2. Pilih PC lama** | Pastikan klien berjalan di sana dan menampilkan kode, dan kedua PC berada di jaringan yang sama. Lalu ketik nama atau alamatnya. Jika melewati router, ketik alamatnya. Firewall selain milik Windows sendiri harus mengizinkan TCP 47732 dan UDP 47731. |
| Sebuah nama "tidak ditemukan di jaringan" | Ketik alamat PC lama sebagai gantinya. |
| "PC lama tidak dapat dijangkau:" dan alasannya | Pastikan klien masih berjalan, dan nama atau alamatnya benar. |
| "itu bukan kode yang ditampilkan PC lama. Periksa lalu coba lagi." | Ketik kode yang ditampilkan klien saat ini. Setelah lima kode salah, klien membuat kode baru. Setelah 20, klien tidak mengizinkan pemasangan apa pun sampai Anda memulainya lagi. |
| Di PC lama, Windows menyebutkan bahwa penerbit klien tidak dikenal | Itu wajar hanya pada Windows 7 tanpa sertifikat akar Microsoft tahun 2020, yang tidak dapat memeriksa tanda tangan klien. Di sana, pilih **Ya** hanya untuk klien yang Anda simpan sendiri dari aplikasi. Di tempat lain, permintaan itu seharusnya menyebutkan Randy Northrup. Jika tidak, jangan jalankan salinan itu. |
| Aplikasi tidak dapat membaca PC lama lagi, setelah jeda yang lama | Klien mengakhiri sambungan yang tidak menerima permintaan apa pun dari aplikasi selama 30 menit. Klik **Putuskan sambungan**, lalu pasangkan lagi. |

<a id="looking-at-the-app"></a>

### Memindai aplikasi

| Yang Anda lihat | Artinya | Yang harus dilakukan |
| --- | --- | --- |
| "Aplikasi ini tidak dapat dipindai:" dan alasannya | Pemindaian gagal. Salah satu alasan umum adalah entri yang sama sekali tidak memiliki program, seperti paket driver. | Klik **Coba lagi**. Jika gagal dengan cara yang sama, entri itu mungkin bukan aplikasi yang dapat dipindahkan WinAppMigrator. |
| "Dihentikan. Penginstal dapat dibuat setelah aplikasi ini dipindai." | Anda mengklik **Hentikan** | Klik **Pindai lagi**. |
| Pemindaian memakan waktu lama | Aplikasi besar, drive yang lambat atau hampir rusak, atau banyak akun | Jika drive mungkin hampir rusak, klik **Hentikan**, buat [citra disk](Glossary#disk-image) dari drive itu, lalu pilih citra itu sebagai gantinya, karena membaca drive yang hampir rusak dapat merusaknya sama sekali. Sejak 0.6.0, pilih citra itu dengan **Citra disk…**. Dengan 0.5.0, pasang citra itu lalu pilih drive-nya. Jika tidak, tunggu. Sementara itu Anda dapat memilih aplikasi lain. |
| Baris yang Anda inginkan berwarna abu-abu dan dicentang | Baris itu berisi program utama, atau apa yang diperlukannya untuk dimulai | Baris itu selalu disertakan. Lihat [baris yang tidak dapat Anda hapus centangnya](Building-an-Installer#the-groups). |
| Sebuah baris menampilkan "Juga tidak disertakan:" | Sesuatu yang Anda hapus centangnya ikut membawanya | Centang lagi hal itu, jika Anda menginginkan baris ini. |
| "Yang tersisa tidak dapat menjadi paket" | Apa yang Anda hapus centangnya tidak menyisakan apa pun yang dapat dibuat | Centang lagi sesuatu. |
| "Data lisensi di bagian yang tidak Anda sertakan tetap ikut" | Lisensi hanya mengikuti opsi **Data lisensi dan aktivasi** | Hapus centang opsi itu jika Anda ingin lisensi tidak disertakan. |

<a id="create-installer-stays-greyed-out"></a>

### Tombol Buat penginstal tetap tidak aktif

Tombol itu menunggu sampai semua hal berikut terpenuhi:

- pemindaian aplikasi telah selesai, untuk opsi seperti saat ini ("Menentukan apa yang terkait dengannya…" berarti aplikasi masih menentukan apa yang ikut terbawa oleh centang terakhir yang Anda hapus);
- **Folder output** berupa jalur lengkap, seperti `D:\Installers` ("Masukkan jalur folder lengkap, misalnya D:\Penginstal.");
- ada cukup ruang di drive itu.

<a id="room"></a>

### Ruang

| Yang Anda lihat | Yang harus dilakukan |
| --- | --- |
| "Ruang tidak cukup." dan berapa ruang yang diperlukan dan dimiliki sebuah drive | Pilih folder output di drive lain, atau kurangi apa yang disertakan. Angka itu adalah jumlah terbesar yang mungkin diperlukan pembuatan penginstal, sebelum kompresi, jadi bisa lebih besar daripada ukuran penginstal nanti. |
| Sebuah drive "tidak menyebutkan berapa ruang yang dimilikinya" | Pembuatan tetap berjalan tanpa memeriksa drive itu. Pastikan drive itu memiliki cukup ruang. |
| Drive FAT32 dan aplikasi besar | FAT32 tidak dapat menyimpan file berukuran 4 GiB. Gunakan drive NTFS atau exFAT. |

<a id="the-done-screen"></a>

### Layar selesai

| Yang Anda lihat | Yang harus dilakukan |
| --- | --- |
| "Penginstal dibuat, tetapi" beberapa file "sudah hilang sejak aplikasi dipindai" | Klik **Pindai lagi**, lalu **Buat penginstal lagi**, jika file tersebut penting. |
| "Penginstal dibuat, tetapi" beberapa file "tidak dapat diambil, biasanya karena terkunci atau tidak dapat dibaca." | Tutup aplikasi, dan apa pun yang menggunakan file-nya, lalu buat lagi. **Peringatan** mencantumkan file mana saja. |
| "Penginstal dibuat, dan masih menyebutkan sesuatu tentang asalnya." | Baca pesan berwarna amber di bawahnya. Lihat [pemeriksaan terakhir](Building-an-Installer#the-last-look). |
| **Peringatan** menampilkan "Tidak dapat mengunduh" sebuah runtime, dan "penginstal akan mencoba winget di PC target." | Runtime itu tidak ada di dalam penginstal, jadi PC baru akan memerlukan internet untuknya. Buat lagi saat online. Lihat [Runtime](Runtimes#at-build-time). |
| "Jangan gunakan penginstal ini: penginstal gagal dalam pemeriksaan hash." | Jangan gunakan. Buat lagi. Jika terjadi lagi, drive mungkin hampir rusak. |
| "Penginstal telah ditulis, tetapi pemeriksaan hash-nya tidak dapat dijalankan" | Jangan gunakan. Buat lagi. |
| "Pembuatan dibatalkan. Tidak ada penginstal yang ditulis." | Anda mengklik **Batal**. |
| "Penginstal tidak dapat dibuat." dan alasannya | Baca alasannya. **Salin kesalahan** menyalinnya, dan **Log** menunjukkan lokasi log. |
| "Tidak dapat membuat folder output" | Pilih folder yang dapat Anda tulisi. |
| "Tidak dapat membuat folder penahapan yang terlindungi, jadi tidak ada yang dibuat" | Proses pembuatan tidak dapat membuat folder kerja privatnya di samping folder output, atau di folder temp Windows. Pilih folder output di drive lain. |
| "Pembuatan berhenti secara tak terduga" | Terjadi kesalahan yang sama sekali tidak terduga. "Jika ada penginstal yang tertinggal di folder output, jangan gunakan penginstal itu." Harap [laporkan](#getting-help). |

<a id="installing"></a>

## Menginstal

<a id="when-the-installer-opens"></a>

### Saat penginstal dibuka

| Yang Anda lihat | Artinya | Yang harus dilakukan |
| --- | --- | --- |
| "Memeriksa penginstal…" dalam waktu lama | Penginstal membaca seluruh paket untuk memeriksa hash-nya | Tunggu. Penginstal yang besar memerlukan waktu. |
| "Tidak dapat membaca penginstal:" dan "penginstal ini rusak atau diubah setelah dibuat, jadi tidak ada yang dibaca dari paketnya. Salin lagi dari tempat penginstal itu dibuat." | File terpotong atau diubah | Salin lagi. |
| "penginstal ini menyimpan paketnya di file bernama" sebuah file `.wamdata`, "di folder yang sama, dan file itu tidak ada" | Penginstal dua file kehilangan file datanya | Salin file `.wamdata` ke folder yang sama. Lihat [Aplikasi besar](Big-Apps). |
| file data "bukan file data penginstal ini, atau file itu rusak atau diubah setelah dibuat" | File data tidak cocok | Salin lagi kedua file, dari tempat keduanya dibuat. |
| "Paket penginstal ini tidak dapat digunakan di PC ini:" dan "aplikasi ini tidak dapat berjalan di PC ini, jadi tidak ada yang diubah" | Program aplikasi dibuat untuk prosesor lain, atau sebuah program menyebutkan bahwa ia memerlukan Windows yang lebih baru | Instal di PC yang seperti PC lama. Tidak ada yang diubah. |
| "Ini adalah stub penginstal WinAppMigrator, dan tidak membawa paket aplikasi." | Ini adalah stub kosong, bukan penginstal yang sudah dibuat | Buat penginstal dengan aplikasi, lalu jalankan penginstal itu. |
| "Tanda tangan penginstal ini tidak valid", dengan alasan dari Windows | Penginstal ditandatangani, tetapi Windows tidak menerima tanda tangannya. Penginstal mungkin telah diubah sejak itu, atau sertifikatnya tidak dipercaya di PC ini. | Jangan instal. Tanyakan kepada pembuatnya. |
| Permintaan UAC menyebutkan "Penerbit tidak dikenal" | Penginstal tidak ditandatangani. Itu normal untuk penginstal yang Anda buat sendiri. | Jalankan hanya jika Anda memercayai asalnya. |

<a id="the-install-location"></a>

### Lokasi instalasi

Ketika **Instal** dan **Pratinjau perubahan** tidak aktif, pemberitahuan merah di bawah **Lokasi instalasi** menjelaskan alasannya. Alasan yang paling umum adalah aplikasi itu sudah diinstal di folder tersebut di PC ini, seperti yang sering terjadi pada peramban web, pemutar media, atau Office: hapus dulu instalan salinan itu, di Pengaturan Windows pada **Aplikasi**, lalu buka penginstal lagi, atau ketik folder baru yang kosong. Alasan-alasannya, dan apa yang harus dilakukan, ada di [Menginstal di PC baru](Installing-on-the-New-PC#when-the-location-cannot-be-used).

<a id="before-or-during-the-install"></a>

### Sebelum atau selama instalasi

| Yang Anda lihat | Artinya | Yang harus dilakukan |
| --- | --- | --- |
| "Ruang di sini mungkin tidak cukup" | Drive tampak kekurangan ruang | Kosongkan ruang, atau pilih lokasi di drive lain. |
| "ruang tidak cukup untuk ini, jadi tidak ada yang diubah" | Drive yang ditulisinya terlalu penuh | Kosongkan ruang di sana, atau pilih lokasi instalasi di drive lain. |
| "Apa yang akan dilakukan di sini telah berubah, jadi tidak ada yang diinstal." | Ada yang berubah di PC sejak daftar ditentukan | Baca daftar lagi, lalu klik **Instal** lagi jika memang itu yang Anda inginkan. |
| "Instalasi" sebuah aplikasi "sebelumnya tidak pernah selesai. Instal akan mengurungkan instalasi itu terlebih dahulu." | Instalasi sebelumnya terputus | Klik **Instal**. Instalasi itu akan diurungkan terlebih dahulu. |
| "instalasi atau penghapusan instalan lain sedang berjalan. Tunggu hingga selesai, lalu coba lagi" | Hanya satu yang dapat berjalan dalam satu waktu | Tunggu, lalu coba lagi. |
| Sebuah file "memiliki nama yang dibuka oleh versi Windows ini sebagai perangkat, bukan sebagai file" | Aplikasi memiliki file bernama seperti `aux.h`, yang tidak dapat disimpan oleh Windows sebelum Windows 11 | Instal di Windows 11. |
| "Tidak dapat membuat folder penahapan" | Tidak ada tempat privat dengan ruang yang cukup untuk mengekstrak paket | Kosongkan ruang di drive lokasi instalasi atau di drive sistem. |
| Folder data seperti `%AppData%` disebut sebagai tautan | Penginstal tidak akan menulis melalui [tautan](Glossary#link) | Hapus centang **Data pengguna** dan **Lisensi**, lalu instal tanpa keduanya. |
| "Folder ProgramData di PC ini tidak dapat ditemukan, jadi tidak ada yang diinstal" | Catatan Windows sendiri tentang lokasi ProgramData tidak dapat dibaca | Penginstal tidak akan menginstal tanpa tempat yang aman untuk menyimpan catatannya. |

<a id="the-result"></a>

### Hasil

| Yang Anda lihat | Yang harus dilakukan |
| --- | --- |
| "Instalasi gagal, dan apa yang diubahnya telah dikembalikan." | Baca pesan merah untuk mengetahui penyebabnya. PC kembali seperti semula, kecuali runtime yang diinstalnya. |
| "Instalasi gagal, dan tidak semua yang diubahnya dapat dikembalikan." | Baca apa yang tercantum. Tutup apa pun yang mungkin menggunakan file tersebut, lalu jalankan penginstal lagi: penginstal akan mencoba lagi terlebih dahulu. |
| "Tidak dapat menginstal" sebuah runtime | Runtime tidak dapat diinstal, jadi instalasi dikembalikan. Untuk DirectX End-User Runtime, PC baru memerlukan internet. Lihat [Runtime](Runtimes#with-no-internet-on-the-new-pc). |
| "Sebuah dependensi perlu memulai ulang PC untuk menyelesaikannya. Mulai ulang Windows, lalu jalankan aplikasi." | Mulai ulang, lalu jalankan aplikasi. |
| "Layanan" itu "berjalan sebagai" sebuah akun "di PC sumber. Kata sandinya tidak ada di penginstal, jadi layanan disiapkan sebagai LocalService." | Buka Layanan, lalu atur akun dan kata sandi layanan itu, jika layanan tidak berfungsi sebagaimana adanya. |
| Beberapa file "ditempatkan di bawah" folder instalasi "sebagai gantinya, di folder bernama _external" | PC ini menyimpan Dokumen atau Game Tersimpan di luar profil Anda. Pindahkan sendiri file tersebut ke folder itu. |
| Sesuatu "tidak disertakan", dengan alasannya | Aturan menolaknya, atau hal itu sudah menjadi milik aplikasi lain di sini. Sisanya tetap diinstal. Jika aplikasi memerlukannya, lihat alasannya. |
| **Hash paket TIDAK VALID** atau **File yang diinstal TIDAK VALID** | Ada yang tidak cocok. Salin lagi penginstal, lalu coba sekali lagi. |

<a id="after-installing"></a>

### Setelah menginstal

| Yang Anda lihat | Yang harus dilakukan |
| --- | --- |
| Aplikasi tidak dapat dimulai | Periksa **Dependensi** untuk runtime yang ditandai **Instal secara manual**, lalu instal runtime itu. Periksa peringatan di hasil. |
| Aplikasi dari Windows 7 tidak dapat dimulai, dan peringatan saat pembuatan menyebutkan sebuah DLL, "dan Windows 10 serta 11 tidak menyertakannya" | DLL itu ditempatkan di folder milik Windows lama oleh sesuatu yang lain, jadi tidak ikut dibawa. Instal apa pun yang menyediakannya, lalu mulai aplikasi lagi. Lihat [Aplikasi dari Windows 7, Vista, 8, atau 8.1](Moving-from-a-Connected-Drive#apps-from-windows-7-vista-8-or-81). |
| Aplikasi dimulai seperti baru | Itu wajar untuk penginstal yang dibuat dengan **Data pengguna** nonaktif. Ini juga terjadi jika centang **Data pengguna** dihapus saat instalasi, karena data kemudian ditempatkan di bawah folder instalasi, dan jika penginstal tidak dapat mengetahui siapa yang sedang masuk, yang akan diberitahukannya. |
| Aplikasi meminta untuk diaktifkan | Lisensi yang terikat ke PC lama perlu diaktifkan lagi. |
| Pembaru milik aplikasi menganggap aplikasi tidak terinstal | Windows Installer tidak memiliki catatan tentang aplikasi itu di sini. Lihat [Keterbatasan](Limitations). |
| Jenis file atau jenis tautan tidak terbuka dengan aplikasi | Aplikasi lain sudah memilikinya di sini, jadi dibiarkan. Pilih aplikasi itu di Pengaturan, di Aplikasi default. |
| Entri `Path` baru tidak terlihat | Buka PowerShell atau prompt perintah baru. Beberapa program baru melihat perubahan setelah Anda keluar lalu masuk lagi. |

<a id="uninstalling"></a>

## Menghapus instalan

| Yang Anda lihat | Yang harus dilakukan |
| --- | --- |
| "Sebagian aplikasi berada di drive yang tidak tersambung." | Sambungkan drive itu, lalu hapus instalan lagi dari Pengaturan. |
| "Tidak semuanya dapat diurungkan. Tutup apa pun yang sedang menggunakannya, lalu hapus instalan lagi dari Pengaturan." | Tutup aplikasi dan apa pun yang menggunakan file-nya. Jika sebuah layanan tidak mau berhenti, mulai ulang PC. Lalu hapus instalan lagi. |
| "ini bukan ID instalasi" | ID yang diberikan ke `--uninstall` salah. Salin dari entri di Pengaturan. Lihat [Baris perintah](Command-Line#finding-the-id). |
| File bernama `.before-install` muncul | Itu adalah file asli dari file yang diganti instalasi, yang telah Anda ubah sejak saat itu. Simpan mana pun yang Anda inginkan. |
| Sebuah runtime masih terinstal | Runtime tetap ada, karena program lain mungkin menggunakannya. Biarkan kecuali Anda yakin tidak ada hal lain yang menggunakannya. |

<a id="scripts"></a>

## Skrip

| Yang Anda lihat | Yang harus dilakukan |
| --- | --- |
| "Operasi yang diminta memerlukan elevasi" | Jalankan PowerShell sebagai administrator. |
| Tidak ada yang dicetak, dan `$LASTEXITCODE` tidak diatur | Salurkan output penginstal, misalnya ke `Out-String`. Lihat [Baris perintah](Command-Line). |
| Kode keluar `2` | Sebuah flag salah, atau `--apply` dan `--dry-run` digunakan bersamaan. |
| "this package does not say where its app was, so give a --target-root" | Tambahkan `--target-root` dengan jalur lengkap. |
| Aplikasi "was in" sebuah folder, "and that cannot be used" | Tambahkan `--target-root` dengan folder lain. |
| Sebuah folder "cannot be used: it already has files" "Give --force only if you know it is this app's" | Pilih folder kosong. Gunakan `--force` hanya untuk folder yang Anda tahu milik aplikasi ini. |

<a id="language-and-text"></a>

## Bahasa dan teks

- **Bahasa yang salah.** Pilih bahasa lain di **Bahasa**, di kanan atas aplikasi atau di layar pertama penginstal. Aplikasi mengingatnya; penginstal menyimpannya untuk sesi itu. Jika belum ada pilihan, masing-masing menggunakan bahasa tampilan Windows pertama yang dimilikinya, atau bahasa Inggris jika tidak ada.
- **Terjemahan terasa keliru.** Semua bahasa selain bahasa Inggris diterjemahkan oleh mesin. Di **Bahasa**, pilih **Laporkan masalah terjemahan**, yang membuka formulir dengan bahasa Anda sudah terisi. Sebutkan apa yang ditampilkan layar dan apa yang seharusnya tertulis.
- **Huruf tampil sebagai kotak kosong.** Jendela menggunakan font bawaan Windows untuk setiap aksara, dari `C:\Windows\Fonts`. Windows yang beberapa fontnya dihapus akan menggambar huruf tersebut sebagai kotak; nama aplikasi lain di daftar juga dapat memerlukan font tersebut.
- **Kata-kata terpotong.** Label yang tidak muat diakhiri dengan `…`; arahkan penunjuk ke label itu, atau pindah ke label itu dengan tombol Tab, untuk melihat seluruhnya. Laporkan dengan menyebutkan bahasa dan layarnya, agar jendela dapat disesuaikan.
- **WinAppMigrator Client mencetak bahasa Inggris di Windows dengan tata letak kanan ke kiri.** Konsol tempatnya berjalan tidak dapat menyambung atau mengurutkan huruf Arab atau Ibrani, jadi klien mencetak bahasa Inggris dan menjelaskan alasannya. `--lang` memilih bahasa apa pun yang dimilikinya.

<a id="getting-help"></a>

## Mendapatkan bantuan

1. Temukan [log](Glossary#log). Jalurnya ditampilkan sebagai **Log** saat terjadi kegagalan, atau dicetak sebagai `Log:` dari baris perintah. Log ada di `%ProgramData%\WinAppMigrator\logs`. Log berisi jalur dan nama kunci, dan tidak pernah berisi isi pengaturan, file lisensi, atau variabel lingkungan. Log tetap dapat berisi nama folder Anda, jadi bacalah sebelum Anda membagikannya.
2. [Buka issue](https://github.com/RandyNorthrup/WinAppMigrator/issues) dengan versi, apa yang Anda lakukan, dan apa yang ditampilkan layar. **Salin kesalahan** di layar selesai aplikasi menyalin pesannya.
3. **Jangan pernah melampirkan penginstal ke issue.** Penginstal dapat berisi data pribadi.
4. Untuk masalah keamanan, laporkan secara pribadi, melalui [saran keamanan GitHub](https://github.com/RandyNorthrup/WinAppMigrator/security/advisories/new).
