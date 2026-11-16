# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = Penginstal WinAppMigrator
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = Instal { $app_name }
installer-tagline = Menginstal ulang aplikasi yang diambil dari PC lain
# What an app is called when its package gives it no name.
installer-unnamed-app = Aplikasi

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = Jendela penginstal tidak dapat dibuka: { $error }
installer-window-failed-without-one = Penginstal masih dapat menginstal tanpa jendela: jalankan file ini dari prompt perintah administrator dengan --apply, dan --target-root diikuti sebuah folder untuk memilih lokasinya.

## Checking the installer when it opens

installer-checking = Memeriksa penginstal…
installer-stub-no-package = Ini adalah stub penginstal WinAppMigrator, dan tidak membawa paket aplikasi. Buat penginstal dengan WinAppMigrator, lalu jalankan file .exe yang dihasilkan di sini.
installer-check-stopped = Pemeriksaan penginstal berhenti.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = Tidak dapat menemukan penginstal: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = Tidak dapat membaca penginstal: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = Paket penginstal ini tidak dapat digunakan di PC ini: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = Diambil dari
installer-install-location = Lokasi instalasi
# The title of the folder picker.
installer-choose-location = Pilih lokasi instalasi
installer-folder-has-files = Folder ini sudah berisi file. File dengan nama yang sama akan diganti, dan dikembalikan jika Anda menghapus instalan.
installer-room-needed = Ruang yang diperlukan
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = Ruang di sini mungkin tidak cukup: { $shortfall }. Paket dapat diekstrak di drive lain, tetapi file aplikasi tetap harus muat.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = Instalasi { $app_name } sebelumnya tidak pernah selesai. Instal akan mengurungkan instalasi itu terlebih dahulu.
# Beside "Installer", for whether it proves who made it.
installer-signature = Penginstal
installer-signature-checking = Memeriksa tanda tangannya…
installer-signature-unsigned = Tidak ditandatangani: penerbit tidak dikenal
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = Ditandatangani oleh { $publisher }
installer-signature-unnamed = Ditandatangani oleh penerbit tanpa nama
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = Tanda tangan penginstal ini tidak valid ({ $reason }). Penginstal mungkin telah diubah sejak ditandatangani.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } sudah diinstal di folder ini, jadi salinan ini tidak dapat ditempatkan di sini. Untuk menginstalnya di sini, hapus dulu instalan salinan yang ada di PC ini, di Pengaturan Windows pada Aplikasi, lalu buka penginstal ini lagi. Atau masukkan folder baru yang kosong di atas.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } diinstal di folder ini, jadi { $app_name } tidak dapat ditempatkan di sini. Masukkan folder baru yang kosong di atas. Atau, jika Anda tidak lagi memerlukan { $other_app }, hapus dulu instalannya, di Pengaturan Windows pada Aplikasi, lalu buka penginstal ini lagi.
installer-location-has-files = Folder ini sudah berisi file yang tidak diinstal oleh WinAppMigrator. File tersebut mungkin milik aplikasi lain, jadi tidak ada yang diinstal di atasnya. Masukkan folder baru yang kosong di atas.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = Folder ini tidak dapat digunakan: { $reason }. Masukkan folder lain di atas.

## What the install sets up beyond its files

installer-beyond-files = Yang disiapkan selain file
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = Semua ini dilakukan dengan hak administrator. Jika instalasi gagal, apa yang diubahnya akan dikembalikan, dan Anda akan diberi tahu tentang apa pun yang tidak dapat dikembalikan. Anda juga dapat mengurungkan instalasi ini nanti dari Pengaturan, di Aplikasi yang diinstal. File yang digantinya akan dikembalikan, dan runtime yang diinstalnya tetap ada.
installer-beyond-files-none = Tidak ada: instalasi ini hanya menyalin file, serta pengaturan di kunci registri milik aplikasi itu sendiri.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = Paket ini tidak dapat ditempatkan di lokasi ini: { $problem }
installer-working-out = Menentukan apa yang akan disiapkan di lokasi ini…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = penentuan apa yang akan dilakukannya terhenti
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
       *[other] { $count } lainnya tidak disertakan
    }

## Options

installer-options = Opsi
installer-user-data = Data pengguna
installer-user-data-detail = Aktif: pengaturan, Dokumen, dan Game Tersimpan ditempatkan di folder biasa milik siapa pun yang sedang masuk. Nonaktif: semuanya ditempatkan di dalam folder instalasi, dan tidak ada yang disiapkan untuk berjalan dari sana.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = Ini berisi data milik { $account }.
installer-license = Lisensi
installer-license-detail = File aktivasi dan lisensi, serta nilai registri yang menyimpan lisensi. Beberapa di antaranya mungkin perlu diaktifkan ulang di sini.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = Dependensi
installer-dependencies-detail = Apa yang diperlukan aplikasi ini, dan bagaimana masing-masing disediakan.
installer-dependencies-none = Tidak ada dependensi runtime tambahan yang terdeteksi.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Fitur Windows: { $feature }
installer-dependency-included = Disertakan
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = Diinstal dengan winget: { $id }
installer-dependency-manual = Instal secara manual
installer-dependency-built-in = Bawaan Windows
installer-dependency-ships-with-app = Bawaan aplikasi

## Buttons, and what is said beside them

installer-install = Instal
installer-preview = Pratinjau perubahan
installer-cancel = Batal
installer-close = Tutup
installer-open-folder = Buka folder instalasi
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = Tidak dapat membuka folder: { $error }
installer-location-blocks = Instal dan Pratinjau perubahan tidak aktif sampai Lokasi instalasi dapat digunakan. Alasannya tertera di atas, di bawah kolom itu.
installer-package-blocks = Instal tidak aktif karena paket ini tidak dapat ditempatkan di lokasi ini. Alasannya tertera di atas.
installer-list-changed = Apa yang akan dilakukan di sini telah berubah, jadi tidak ada yang diinstal. Periksa daftar di bawah, lalu klik Instal lagi jika memang itu yang Anda inginkan.
installer-cancelled-early = Dibatalkan. Tidak ada yang diubah.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } dari { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = Bersiap…
installer-step-cancelling = Membatalkan setelah langkah saat ini…
installer-step-putting-back = Mengembalikan apa yang diubah
installer-step-unpacking = Mengekstrak
installer-step-checking-package = Memeriksa paket
installer-step-recovering = Mengurungkan instalasi sebelumnya yang tidak pernah selesai
# What a preview is doing, whatever step it is on.
installer-step-previewing = Menentukan apa yang akan berubah
installer-step-runtimes = Menginstal runtime
installer-step-files = Menyalin file
installer-step-services = Menyiapkan layanan
installer-step-drivers = Menyiapkan driver
installer-step-registry = Menulis pengaturan registri
installer-step-license = Memulihkan data lisensi
installer-step-tasks = Menyiapkan tugas terjadwal
installer-step-shortcuts = Membuat pintasan
installer-step-environment = Mengatur variabel lingkungan
installer-step-settings-entry = Menambahkan aplikasi ke Pengaturan
installer-step-verifying = Memeriksa apa yang diinstal
installer-step-undoing = Mengurungkan

## When an install cannot go ahead or stops

installer-carries-no-package = Penginstal ini tidak membawa paket.
installer-stopped-unexpectedly = Instalasi berhenti secara tak terduga.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = Instalasi berhenti secara tak terduga: { $reason }. Jika instalasi sudah dimulai, menjalankan penginstal lagi akan mengurungkan dulu apa yang telah dilakukannya.
# { $error } is what went wrong reading it.
installer-package-read-failed = Tidak dapat membaca paket: { $error }
installer-no-programdata = Folder ProgramData di PC ini tidak dapat ditemukan, jadi tidak ada yang diinstal: instalasi menyimpan di sana catatan yang diperlukan untuk mengurungkannya.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = Tidak dapat membuat folder penahapan: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = Tidak dapat mengekstrak paket: { $error }
installer-staging-nowhere = tidak ada tempat untuk mengekstrak paket yang hanya dapat diakses oleh administrator
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = tidak ada tempat untuk mengekstrak paket yang hanya dapat diakses oleh administrator. Yang perlu diperiksa adalah { $failure }

## What came of an install, or of a preview

installer-result = Hasil instalasi
installer-result-preview = Pratinjau: belum ada yang diubah
installer-result-installed = Aplikasi telah diinstal dan lulus pemeriksaan hash.
installer-result-installed-without-left-out = Aplikasi telah diinstal dan lulus pemeriksaan hash, tanpa apa yang tercantum sebagai tidak disertakan di bawah.
installer-result-not-all-put-back = Instalasi gagal, dan tidak semua yang diubahnya dapat dikembalikan. Sisanya tercantum di bawah, dan instalasi berikutnya akan mencoba lagi.
installer-result-cancelled-put-back = Instalasi dibatalkan, dan apa yang diubahnya telah dikembalikan.
installer-result-failed-put-back = Instalasi gagal, dan apa yang diubahnya telah dikembalikan.
installer-result-not-clean = Instalasi tidak selesai dengan bersih. Tinjau masalah di bawah.
installer-result-changes-put-back = Perubahan yang dikembalikan
installer-result-changes-made = Perubahan yang dibuat
installer-result-changes-planned = Perubahan yang direncanakan
installer-result-warnings = Peringatan
installer-result-errors = Kesalahan
installer-result-package-valid = Hash paket valid
installer-result-package-invalid = Hash paket TIDAK VALID
installer-result-files-valid = File yang diinstal valid
installer-result-files-invalid = File yang diinstal TIDAK VALID
installer-result-reboot = Sebuah dependensi perlu memulai ulang PC untuk menyelesaikannya. Mulai ulang Windows, lalu jalankan aplikasi.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = Perubahan ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = dan { $count } lainnya.
# Over the path of the install's log file.
installer-log = Log

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = Stub penginstal WinAppMigrator — tidak membawa paket aplikasi.
installer-cli-stub-how = Buat penginstal dengan aplikasi WinAppMigrator, lalu jalankan file .exe yang dihasilkan di PC target.
installer-cli-usage-title = Penginstal WinAppMigrator yang mengekstrak sendiri.
installer-cli-usage = Jalankan tanpa argumen untuk membuka penginstal grafis, atau:
installer-cli-usage-preview = pratinjau instal ulang: --dry-run [--target-root <DIR>]
installer-cli-usage-apply = instal ulang:           --apply [--target-root <DIR>]
installer-cli-no-package = penginstal tidak berisi paket yang disematkan
installer-cli-no-programdata = folder ProgramData di PC ini tidak dapat ditemukan, jadi tidak ada yang diinstal: instalasi menyimpan di sana catatan yang diperlukan untuk mengurungkannya
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path } tidak dapat digunakan: { $problem }. Berikan --force hanya jika Anda tahu folder itu milik aplikasi ini
installer-cli-no-location = paket ini tidak menyebutkan lokasi aplikasinya semula, jadi berikan --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = aplikasi dalam paket ini semula berada di { $path }, dan lokasi itu tidak dapat digunakan: { $problem }. Berikan --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = Ruang: { $summary }
# { $note } is something to know about the install.
installer-cli-note = Catatan: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = Disiapkan: { $count }
installer-cli-applied = Instal ulang diterapkan.
installer-cli-previewed = Instal ulang dipratinjau.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = Perubahan yang dicatat: { $count }
# { $count } is how many warnings.
installer-cli-warnings = Peringatan: { $count }
# { $count } is how many errors.
installer-cli-errors = Kesalahan: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = Ditolak: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = Hash paket valid: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = Hash instal ulang valid: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = ID instalasi: { $id }
installer-cli-put-back = Instalasi gagal, jadi apa yang diubahnya telah dikembalikan.
# { $path } is the log file.
installer-cli-log = Log: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = log: { $path }
# { $error } is what went wrong.
installer-cli-error = kesalahan: { $error }

## The uninstaller's window

uninstaller-window-title = Hapus instalan
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = Hapus instalan { $app_name }
uninstaller-tagline = Mengurungkan apa yang dilakukan penginstalnya
uninstaller-reading = Membaca catatan instalasi ini…
uninstaller-stopped-unexpectedly = Penghapusan instalan berhenti secara tak terduga.
uninstaller-what-will-be-removed = Yang akan dihapus
uninstaller-install-location = Lokasi instalasi
uninstaller-files = File
uninstaller-registry-values = Nilai registri
uninstaller-services = Layanan
uninstaller-drivers = Driver
uninstaller-scheduled-tasks = Tugas terjadwal
uninstaller-environment = Variabel lingkungan
uninstaller-plan-stays = File yang Anda tambahkan atau ubah tetap ada, beserta folder tempatnya berada.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
       *[other] { $count } file yang diganti oleh instalasi ini akan dikembalikan, atau ditempatkan di samping file yang telah berubah sejak itu.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = Runtime yang diinstalnya tetap ada, karena program lain mungkin menggunakannya: { $runtimes }.
uninstaller-options = Opsi
uninstaller-remove-user-data = Hapus juga data pengguna dan data lisensi
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
       *[other] { $count } item pengaturan, data, dan lisensinya. Item tersebut tetap ada kecuali opsi ini aktif. Apa yang digantikannya akan dikembalikan, atau ditempatkan di sampingnya jika telah berubah sejak itu.
    }
uninstaller-uninstall = Hapus instalan
uninstaller-cancel = Batal
uninstaller-close = Tutup
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = Mengurungkan instalasi: { $done } dari { $total }…
uninstaller-step = Mengurungkan instalasi…
uninstaller-done = Instalan aplikasi telah dihapus.
uninstaller-done-except = Instalan aplikasi telah dihapus, kecuali yang tercantum di bawah.
uninstaller-waits-for-drive = Sebagian aplikasi berada di drive yang tidak tersambung. Sambungkan drive itu, lalu hapus instalan lagi dari Pengaturan.
uninstaller-not-all-undone = Tidak semuanya dapat diurungkan. Tutup apa pun yang sedang menggunakannya, lalu hapus instalan lagi dari Pengaturan.
# Over the path of the uninstall's log file.
uninstaller-log = Log
uninstaller-undone = Diurungkan
uninstaller-left-in-place = Dibiarkan di tempatnya
uninstaller-could-not-be-undone = Tidak dapat diurungkan
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = dan { $count } lainnya.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = folder ProgramData di PC ini tidak dapat ditemukan, jadi tidak ada catatan yang dapat dibaca

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = Penghapus instalan WinAppMigrator
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = Jendela penghapus instalan tidak dapat dibuka: { $error }
uninstaller-window-failed-quiet = Penghapus instalan masih dapat menghapus instalan tanpa jendela, dari prompt perintah administrator:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = program ini

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = Menghapus instalan { $app_name }.
# { $path } is the log file.
uninstaller-cli-log = Log: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = kesalahan: { $error }
