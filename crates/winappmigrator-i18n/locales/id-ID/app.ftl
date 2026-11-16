# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = Ubah aplikasi yang diinstal menjadi penginstal yang mengekstrak sendiri
app-theme-to-light = Beralih ke tema terang
app-theme-to-dark = Beralih ke tema gelap
app-reload-list = Muat ulang daftar aplikasi
app-list-loading = Memuat
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
       *[other] { $count } aplikasi
    }
app-list-load-failed = Gagal memuat
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = Jendela tidak dapat dibuka: { $error }

## The list of installed apps

app-list-search = Cari menurut nama atau penerbit  (Ctrl+F)
app-list-filter-all = Semua
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = Per pengguna
app-scope-per-machine = Per mesin
app-scope-unknown = Tidak diketahui
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = Tidak diketahui
app-list-loading-apps = Memuat aplikasi yang diinstal…
# { $error } is why the apps could not be listed.
app-list-could-not-list = Tidak dapat mencantumkan aplikasi: { $error }
app-list-no-matches = Tidak ada aplikasi yang cocok
app-list-no-matches-hint = Sesuaikan teks pencarian atau filter cakupan.
app-list-worker-disconnected = Proses inventaris terputus.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, versi { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = Nama
app-column-name-sort = Urutkan menurut nama
app-column-publisher = Penerbit
app-column-publisher-sort = Urutkan menurut penerbit
app-column-version = Versi
app-column-version-sort = Urutkan menurut versi
app-column-scope = Cakupan
app-column-scope-sort = Urutkan menurut cakupan
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = Arsit.
app-column-arch-sort = Urutkan menurut arsitektur

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = Sumber pemindaian
app-source-this-pc = PC ini
app-source-drive = Drive yang tersambung…
app-source-image = Citra disk…
app-source-network = PC jaringan…
app-source-attaching-image = Melampirkan citra disk, hanya-baca…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } melalui jaringan ({ $address })
app-source-disconnect = Putuskan sambungan
app-source-change = Ubah…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = Pilih citra disk dari PC lain (.vhd atau .vhdx)
app-source-pick-image-filter = Citra disk
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } tidak memiliki volume yang berisi Windows.
app-source-image-not-attached = Citra disk tidak dapat dilampirkan.
app-source-pick-drive-title = Pilih akar drive yang tersambung (mis. E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } tidak tampak seperti drive sistem Windows (tidak ada \Windows dan \Users). Pilih akar drive dari PC lain.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    Sumber ini berisi { $windows }. Penginstal yang dibuat darinya ditujukan untuk Windows 10 dan 11. Dengan Data pengguna, pengaturan yang diam-diam disimpan terpisah oleh Windows untuk aplikasi lama ikut dibawa, dan runtime yang tidak dimiliki Windows 11, seperti DirectX 9, diinstal bersama aplikasi.{ $is_32_bit ->
        [yes] { " " }Aplikasinya 32-bit, jadi ditempatkan di Program Files (x86), tempat Windows 64-bit menyimpannya.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    Sumber ini berisi Windows versi lama. Penginstal yang dibuat darinya ditujukan untuk Windows 10 dan 11. Dengan Data pengguna, pengaturan yang diam-diam disimpan terpisah oleh Windows untuk aplikasi lama ikut dibawa, dan runtime yang tidak dimiliki Windows 11, seperti DirectX 9, diinstal bersama aplikasi.{ $is_32_bit ->
        [yes] { " " }Aplikasinya 32-bit, jadi ditempatkan di Program Files (x86), tempat Windows 64-bit menyimpannya.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = Pasangkan dengan PC lama
app-details-pair-first-hint = Setelah WinAppMigrator Client berjalan di sana dan Anda telah mengetikkan kodenya, aplikasinya akan tercantum di sini, dan Anda dapat membuat penginstal dari salah satunya seperti dari PC ini.
app-details-pick-an-app = Pilih aplikasi
app-details-pick-an-app-hint = Pilih aplikasi dari daftar untuk memilih apa yang disertakan dan membuat penginstalnya.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, versi { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = Penerbit tidak dikenal, versi { $version }
app-details-unknown-publisher = Penerbit tidak dikenal

## What goes in besides the program

app-option-license = Data lisensi dan aktivasi
app-option-license-hint = Mungkin terikat ke PC ini dan perlu diaktifkan ulang di PC baru.
app-option-user-data = Data pengguna
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = Pengaturan dan riwayat, Dokumen, Game Tersimpan, dan ProgramData.
app-option-whose-data = Data milik siapa
app-option-whose-data-hint = Satu penginstal menyimpan data satu akun. Data itu ditujukan untuk siapa pun yang sedang masuk di PC tempat penginstal dijalankan.

## Building the installer

app-output-folder = Folder output
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\Anda\WinAppMigrator
app-output-folder-not-full = Masukkan jalur folder lengkap, misalnya D:\Penginstal.
app-build-create = Buat penginstal
app-build-create-again = Buat penginstal lagi
app-build-cancel = Batal
app-build-cancelling = Membatalkan setelah langkah saat ini…
app-build-cancelled = Pembuatan dibatalkan. Tidak ada penginstal yang ditulis.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = Stub penginstal (winappmigrator-installer.exe) tidak ditemukan di samping aplikasi ini, jadi penginstal yang mengekstrak sendiri tidak dapat dibuat.
app-build-select-an-app-first = Pilih dulu aplikasi yang akan dimigrasikan.
app-build-look-again-first = Aplikasi ini harus dipindai lagi sebelum penginstalnya dapat dibuat.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = Pembuatan berhenti secara tak terduga: { $reason }. Jika ada penginstal yang tertinggal di folder output, jangan gunakan penginstal itu.
app-build-worker-disconnected = Proses migrasi terputus.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = Tidak dapat membuat folder output { $path }: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = Tidak dapat membuat folder penahapan yang terlindungi, jadi tidak ada yang dibuat: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = Penginstal tidak dapat dibuat. { $error }
app-build-no-installer = Pembuatan selesai tanpa menghasilkan penginstal.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = Penginstal telah ditulis, tetapi pemeriksaan hash-nya tidak dapat dijalankan: { $error }. Jangan gunakan penginstal itu.

## How far a build has got

app-progress-preparing = Menentukan apa yang disertakan…
app-progress-checking-room = Memeriksa ketersediaan ruang…
app-progress-bundling = Membundel runtime dan driver…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = Membaca dan mengompresi file: { $done } dari { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = Memeriksa isi penginstal: { $done } dari { $total }…
app-progress-writing = Menulis penginstal…
app-progress-verifying = Memverifikasi hash…

## What a build made

app-result-failed-hash-check = Jangan gunakan penginstal ini: penginstal gagal dalam pemeriksaan hash.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
       *[other] Penginstal dibuat, tetapi { $count } file sudah hilang sejak aplikasi dipindai, jadi tidak disertakan. Lihat Peringatan di bawah.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
       *[other] Penginstal dibuat, tetapi { $count } file tidak dapat diambil, biasanya karena terkunci atau tidak dapat dibaca. Aplikasi mungkin tidak berfungsi tanpa file tersebut. Lihat Peringatan di bawah.
    }
app-result-mentions-who = Penginstal dibuat, dan masih menyebutkan sesuatu tentang asalnya. Baca apa yang disebutkannya di bawah sebelum Anda memberikannya kepada siapa pun.
app-result-ready = Penginstal siap. Paketnya lulus pemeriksaan hash.
app-result-files = File
app-result-installer-size = Ukuran penginstal
app-result-warnings = Peringatan
app-result-log = Log
app-result-copy-error = Salin kesalahan
app-result-installer = Penginstal
app-result-data-file = File data
app-result-open-folder = Buka folder
app-result-copy-path = Salin jalur
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = Tidak dapat membuka folder: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = Aplikasi ini terlalu besar untuk satu file yang dapat dieksekusi, jadi penginstalnya terdiri dari dua file, { $installer } dan { $data_file }. Keduanya harus tetap bersama: salin keduanya ke PC lain, ke folder yang sama, dengan nama-nama ini. Jika Anda menandatangani penginstal, tanda tangan itu juga mencakup file data.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = Jalankan penginstal di PC lain untuk menginstal ulang { $app_name }. Penginstal memeriksa file-nya dengan SHA-256. Windows menampilkan "Penerbit tidak dikenal" karena penginstal yang Anda buat tidak ditandatangani.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = Kesalahan verifikasi ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = Peringatan ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = dan { $count } lainnya.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = Bersiap memindai aplikasi ini…
app-scan-install-record = Membaca cara aplikasi ini diinstal…
app-scan-near-matches = Mencari apa yang mungkin juga bagian dari aplikasi ini…
app-scan-system-items = Mencari layanan, tugas, dan pintasannya…
app-scan-files = Memeriksa file-nya…
app-scan-registry = Membaca apa yang mengaitkannya ke Windows…
app-scan-finishing = Menyelesaikan…
app-scan-look-again = Pindai lagi
app-scan-stop = Hentikan
app-scan-running-hint = Anda dapat memilih aplikasi lain atau mengubah apa yang disertakan selagi proses ini berjalan.
# { $error } is why, in words that follow a colon.
app-scan-failed = Aplikasi ini tidak dapat dipindai: { $error }
app-scan-try-again = Coba lagi
app-scan-stopped = Dihentikan. Penginstal dapat dibuat setelah aplikasi ini dipindai.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = Proses berhenti secara tak terduga: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = File
# Over how big the installer can be: up to that size.
app-room-up-to = Hingga
app-room-free = Kosong
# { $drive } is a drive, such as C:.
app-room-free-on = Kosong di { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = Tidak diketahui
app-room-most-it-could-need = Ruang terbanyak yang mungkin diperlukan, sebelum dikompresi.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } memerlukan ruang kosong hingga { $needed } untuk membuat ini, dan hanya memiliki { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = Ruang tidak cukup. { $drives } Pilih folder output di drive lain, atau kurangi apa yang disertakan.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = Yang disertakan dalam penginstal
app-footprint-working-out = Menentukan apa yang terkait dengannya…
# { $error } is why.
app-footprint-would-not-make-a-package = Yang tersisa tidak dapat menjadi paket: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = Itu juga membuat { $list } tidak disertakan, karena apa yang dijalankan atau disebutkannya tidak akan ada. Semua itu tidak akan disiapkan di PC lain.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = Itu juga membuat { $list }, dan { $count } lainnya, tidak disertakan, karena apa yang dijalankan atau disebutkannya tidak akan ada. Semua itu tidak akan disiapkan di PC lain.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = Data lisensi di bagian yang tidak Anda sertakan tetap ikut, karena itu diatur oleh opsi Data lisensi dan aktivasi: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = Aplikasi ini dipindai { $elapsed } yang lalu. File dibaca saat Anda membuat penginstal, jadi perubahan akan ikut terbaca, dan apa pun yang sudah hilang saat itu akan dicantumkan. Pindai lagi untuk menemukan hal yang baru.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = File program
app-footprint-data-folders = Folder data
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } dengan opsi Data pengguna
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
       *[other] { $included } dari { $count } folder, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
       *[other] { $size } dalam { $count } file data aplikasi ini hanya akan disertakan dengan opsi Data pengguna.
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
       *[other] { $count } file. { $detail }
    }
app-footprint-services = Layanan
app-footprint-startup = Mulai otomatis
app-footprint-tasks = Tugas terjadwal
app-footprint-shortcuts = Pintasan
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } dari { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = Juga tidak disertakan: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = Yang mengaitkannya ke Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
       *[other] { $included } dari { $count } jenis
    }
app-footprint-integration-nothing-left = Tidak ada yang tersisa, karena bagian yang tidak Anda sertakan
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
       *[other] { $count } item disiapkan
    }
app-footprint-prerequisites = Yang diperlukan untuk berjalan
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
       *[other] { $count } runtime
    }
app-footprint-near-matches = Mungkin juga bagian darinya
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $ticked } dari { $count } ditambahkan
app-footprint-near-matches-hint = Tidak disertakan dalam penginstal kecuali Anda mencentangnya. Mencentang salah satunya akan memindai aplikasi lagi, dengan pemeriksaan yang sama seperti yang lain, jadi apa pun yang tampak seperti kata sandi tetap tidak disertakan.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = Disertakan dengan opsi Data pengguna. { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = lebih dari { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = Folder: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = Bagian tersembunyi dari aplikasi: { $place }
app-footprint-nothing-left-out = Tidak ada yang dikecualikan karena tampak seperti kata sandi, lisensi, atau sertifikat.
app-footprint-left-out = Tidak disertakan, dan alasannya
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
       *[other] { $count } item
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = Tidak ada penginstal yang akan diizinkan menyiapkan hal berikut:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = Perlu diketahui ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = Tidak ditemukan: { $list }.
app-none-found-services = layanan
app-none-found-startup = item startup
app-none-found-tasks = tugas terjadwal
app-none-found-shortcuts = pintasan
app-none-found-integrations = apa pun yang mengaitkannya ke Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. Tempatkan klien di PC lama
app-network-step-client-hint = WinAppMigrator Client berjalan di PC tempat aplikasi berada sekarang, Windows 7 atau yang lebih baru. Klien hanya mengizinkan PC ini membacanya, secara terenkripsi, dengan kode yang ditampilkannya.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = Simpan ke drive USB { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = Simpan ke drive USB { $drive } ({ $name })
app-network-save-to-folder = Simpan ke folder…
app-network-look-for-usb-again = Cari drive USB lagi
app-network-no-usb = Tidak ada drive USB yang dicolokkan. Colokkan drive USB, atau simpan ke folder: folder yang dapat Anda jangkau dari PC lama, seperti folder bersama di jaringan Anda.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = Disimpan sebagai { $path }. Di PC lama, tutup aplikasi yang Anda pindahkan, lalu klik dua kali file tersebut dan izinkan membuat perubahan. Klien akan menampilkan kode.
app-network-step-pick = 2. Pilih PC lama
app-network-looking = Mencari PC yang menjalankan klien di jaringan Anda…
app-network-type-name = Atau ketikkan nama atau alamatnya
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = PC-LAMA atau 192.168.1.20
app-network-step-code = 3. Ketikkan kode yang ditampilkannya
app-network-code = Kode pemasangan
app-network-connect = Sambungkan
app-network-pairing = Memasangkan…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = pemasangan berhenti sebelum selesai
app-network-wrong-code = itu bukan kode yang ditampilkan PC lama. Periksa lalu coba lagi.
# { $error } is what went wrong.
app-network-not-reached = PC lama tidak dapat dijangkau: { $error }
app-network-pick-folder-title = Pilih folder untuk WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = klien tidak dapat disimpan di { $path }: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = build WinAppMigrator ini tidak membawa klien. Buat klien dengan scripts\build-client.ps1, lalu buat aplikasi lagi.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = catatan di sampingnya tidak dapat disimpan: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    Jalankan ini di PC asal aplikasi yang Anda pindahkan: Windows 7, 8, 10,
    atau 11.
    
    1. Tutup aplikasi yang Anda pindahkan.
    2. Klik dua kali "WinAppMigrator Client.exe" dan izinkan membuat perubahan.
    3. Klien menampilkan kode pemasangan 6 digit. Di WinAppMigrator pada PC baru
       Anda, pilih PC jaringan..., pilih PC ini, lalu ketikkan kodenya.
    4. Setelah PC baru selesai, tekan Enter di jendela klien, atau tutup
       jendelanya.
    
    Klien hanya membaca. Selama berjalan, klien menyimpan snapshot drive Windows
    dan mengizinkan dua port miliknya melewati Windows Firewall dari jaringan
    lokal Anda, lalu menghapus keduanya saat berhenti. Data yang melintasi
    jaringan dienkripsi, dan hanya PC yang memiliki kode yang dapat membaca
    apa pun.
