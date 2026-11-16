# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } melewati tautan atau tidak berada di drive yang tersambung, jadi tidak dibaca.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } tidak berada di disk PC ini, atau memiliki nama yang akan membuka hal lain, jadi tidak dibaca.
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } berada di folder akun lain atau bukan milik akun ini, dan catatan aplikasi ini sendiri tidak menyebutkan bahwa file itu semestinya ada di sana, jadi tidak dibaca.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-no-executable = tidak dapat menemukan file yang dapat dieksekusi untuk { $app_name }. Driver, runtime, atau add-in mungkin tidak memiliki program sendiri untuk dikemas.
# { $path } is the file that was picked.
core-scan-not-a-file = target pemindaian bukan file: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = Variabel lingkungan { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = Namanya diawali dengan nama aplikasi ini.
core-near-has-app-name = Namanya mengandung nama aplikasi ini.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = Namanya diawali dengan nama penerbit, { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = Bagian yang tersembunyi dari daftar aplikasi, diinstal di samping aplikasi ini di { $folder }.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = Bagian yang tersembunyi dari daftar aplikasi, dari penerbit yang sama, { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = Ini selalu disertakan bersama program utama
core-footprint-locked-main-program = Berisi program utama, yang selalu disertakan
core-footprint-locked-needed = Berisi apa yang diperlukan program utama untuk dapat dimulai
core-footprint-locked-font = Disertakan bersama font-nya, di bagian Yang mengaitkannya ke Windows
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = Pengaturan di profil Anda (AppData)
core-footprint-place-local = Pengaturan untuk PC ini di profil Anda (AppData)
core-footprint-place-program-data = Digunakan bersama oleh semua orang di PC ini (ProgramData)
core-footprint-place-documents = Di Dokumen Anda
core-footprint-place-saved-games = Di Game Tersimpan Anda
core-footprint-place-public-documents = Di Dokumen Publik
core-footprint-place-app-folder = Di folder aplikasi
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = Di { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = File yang langsung berada di { $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = Tidak ada dalam penginstal ini, karena opsi Data lisensi dan aktivasi nonaktif
core-footprint-font-stays-behind = Item ini tetap tertinggal.
core-footprint-drivers-left-out = Driver milik Windows atau vendor, yang tidak dapat diteruskan, jadi instal ulang driver itu di PC lain
core-footprint-driver-reinstall = Instal ulang dari paket milik vendornya sendiri.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = item ini memerlukan { $service }, yang tidak disertakan
core-footprint-told-to-use-left-out = item ini diatur untuk menggunakan sesuatu di folder yang tidak Anda sertakan
core-footprint-file-in-left-out = file-nya berada di folder yang tidak Anda sertakan
core-footprint-program-in-left-out = program yang menjadi tujuannya berada di folder yang tidak Anda sertakan
core-footprint-names-left-out = item ini menyebutkan folder yang tidak Anda sertakan
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = siapa pun yang membuat penginstal ini tidak menyertakannya
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = pemindaian ini adalah untuk aplikasi lain, bukan aplikasi yang sedang dibuat penginstalnya, jadi tidak ada yang dibuat
core-package-scan-brought-in-more = pemindaian ini membawa data yang tidak diminta oleh pembuatan ini, jadi tidak ada yang dibuat
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } sudah tidak ada, jadi tidak ada yang dibuat. Pindai aplikasi itu lagi.
core-package-needs-stub = arsip yang mengekstrak sendiri memerlukan stub penginstal
# { $reason } is what went wrong.
core-package-staged-file-failed = file di folder penahapan tidak dapat dimasukkan ke paket: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = ruang tidak cukup untuk membuat penginstal ini, jadi tidak ada yang dibaca atau ditulis. { $shortfalls }. Kosongkan sebagian ruang di sana, atau pilih folder output di drive lain.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } melewati tautan atau tidak berada di drive yang tersambung, jadi tidak dibaca
# { $path } is a file's path.
core-package-not-read-not-local = { $path } tidak berada di disk PC ini, jadi tidak dibaca
# { $path } is a file's path.
core-package-not-read-other-account = { $path } mengarah ke folder akun lain, jadi tidak dibaca
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } tidak berada di drive yang tersambung, jadi tidak dibaca
# { $path } is a file's path.
core-package-backup-copy-no-parent = tidak dapat membuat salinan cadangan file tanpa direktori induk: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = tujuan salinan cadangan tidak memiliki direktori induk: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = penyalinan mode cadangan robocopy gagal (kode keluar { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = folder paket sementara
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = penginstal
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = file lain menempati tempat yang sama di paket: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name } berubah di folder terlindungi saat paket sedang dibuat
core-package-worker-died = thread yang mengompresi file berhenti secara tak terduga, jadi tidak ada penginstal yang ditulis
# { $path } is a file's path.
core-package-link-not-read = { $path } adalah tautan, jadi tidak dibaca
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path } tidak dapat dikompresi: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path } tidak dapat dibaca kembali: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = folder tempat menulis harus berupa jalur lengkap: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = folder tempat menulis bukanlah folder: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = jalur keluar dari akar yang dimaksud: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = menolak menulis melalui tautan: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = menolak melewati tautan: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = menolak melewati tautan: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } adalah tautan ke tempat lain, dan orang lain mungkin dapat mengubah tujuannya. Gunakan tempat yang dituju tautan itu
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } tidak memiliki folder induk
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = lokasi instalasi harus berupa jalur lengkap: "{ $path }"
# { $path } is the install location.
core-discovery-install-location-a-link = lokasi instalasi adalah tautan ke tempat lain: { $path }
# { $path } is a file's path.
core-discovery-device-name = jalur menggunakan nama yang dicadangkan untuk perangkat Windows: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } memiliki nama yang dibuka oleh versi Windows ini sebagai perangkat, bukan sebagai file, jadi tidak ada yang ditulis ke sana. File itu dapat diinstal di Windows 11
# { $path } is a file's path.
core-discovery-unsafe-path = jalur tidak aman: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = file tidak memiliki jalur relatif paket: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = paket penginstal
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = stub penginstal yang disematkan
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = entri { $index } dari paket tidak dapat dibaca: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = entri zip memiliki jalur yang tidak aman: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } berisi lebih banyak dari yang disebutkannya
# { $name } is a file's name inside the package.
core-archive-name-too-long = nama ini terlalu panjang untuk zip: { $name }
core-archive-changed-while-copied = paket berubah saat sedang ditempatkan setelah stub, jadi tidak ada penginstal yang ditulis
core-archive-stub-not-a-program = bagian penginstal ini sebelum paketnya bukan program
core-archive-damaged = penginstal ini rusak atau diubah setelah dibuat, jadi tidak ada yang dibaca dari paketnya. Salin lagi dari tempat penginstal itu dibuat.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = penginstal ini menyimpan paketnya di file bernama { $name }, di folder yang sama, dan file itu tidak ada. Kedua file harus tetap bersama: salin { $name } ke sini dari tempat penginstal dibuat, lalu jalankan ini lagi.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = file data penginstal ini, { $name }, tidak dapat dibuka: { $error }. Kedua file harus tetap bersama, di folder yang dapat dibaca PC ini.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } adalah tautan atau folder, sedangkan file data penginstal ini harus berupa file tersendiri, tepat di sampingnya, jadi tidak ada yang dibaca darinya. Salin file yang sebenarnya ke sini.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } bukan file data penginstal ini, atau file itu rusak atau diubah setelah dibuat, jadi tidak ada yang dibaca darinya. Salin lagi kedua file dari tempat keduanya dibuat.
core-archive-stub-signed = stub penginstal ditandatangani; stub harus tetap tidak ditandatangani, jadi tandatangani penginstal yang sudah jadi saja
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = penginstal akan berukuran { $size }, dan Windows tidak dapat menjalankan file yang dapat dieksekusi berukuran 4 GB atau lebih; tidak ada yang ditulis
core-archive-outside-signature = penginstal memiliki data di luar tanda tangannya dan mungkin telah dirusak
core-archive-footer-length = panjang footer arsip yang mengekstrak sendiri tidak valid
core-archive-seek-outside = posisi baca di luar paket
# { $path } is a file's path.
core-archive-unsafe-package-path = jalur paket tidak aman: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = jalur paket kosong: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } bukan citra disk yang dapat dilampirkan Windows (.vhd atau .vhdx)
core-image-needs-administrator = melampirkan citra disk memerlukan hak administrator
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } bukan file
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } adalah disk diferensial, yang hanya berisi perubahan sejak disk induknya, dan Windows juga akan membuka disk induk itu, di mana pun lokasinya menurut disk ini. Gabungkan dulu ke disk induknya, dengan Edit Disk di Hyper-V, lalu pilih disk induk itu
# { $path } is the disk image file.
core-image-unreadable = { $path } bukan .vhd atau .vhdx yang dapat dibaca
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = folder ProgramData di PC ini tidak dapat ditemukan
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows sudah melampirkannya, dan apa pun yang melampirkannya mungkin sedang menulis ke sana. Lepaskan dulu, atau pilih drive-nya sebagai drive yang tersambung
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = { $path } tidak dapat dilampirkan: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } tidak memiliki volume berisi Windows yang dapat dibaca. Volume yang dienkripsi dengan BitLocker harus dibuka kuncinya terlebih dahulu.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows tidak menyebutkan alasannya

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = PC lain terlalu lama menjawab
core-remote-closed = sambungan ke PC lain sudah tertutup
core-remote-failed = sambungan ke PC lain gagal
core-remote-answered-something-else = klien memberikan jawaban yang tidak sesuai
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = { $name } tidak ditemukan di jaringan
core-remote-did-not-say-what-it-is = klien tidak menyebutkan identitasnya
core-remote-ordinary-names-only = hanya jalur di bawah akar PC lain, dengan nama biasa, yang dapat ditanyakan
core-remote-more-but-none = PC lain menyebutkan bahwa sebuah folder berisi lebih banyak, tetapi tidak memberikan isinya sama sekali
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = PC lain mencantumkan lebih dari { $count } item dalam satu folder
core-remote-sent-too-much = PC lain mengirim bagian file yang lebih banyak dari yang diminta
core-remote-name-not-text = nama yang bukan teks tidak dapat ditanyakan
core-remote-below-root-only = hanya jalur di bawah akar PC lain yang dapat ditanyakan
core-remote-file-ended-early = file di PC lain berakhir sebelum waktunya
core-remote-link-not-followed = tautan di PC lain tidak diikuti
core-remote-only-files-opened = hanya file yang dibuka di PC lain
core-remote-listing-said-nothing = daftar isi tidak menyebutkan apa pun tentangnya
core-remote-change-time-unknown = waktu perubahannya tidak diketahui
core-remote-seek-before-start = posisi baca sebelum awal file
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
       *[other] ukurannya { $size } byte, sedangkan yang diharapkan tidak lebih dari { $limit } byte
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = tidak dapat memulai PowerShell: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = tidak dapat membaca tanda tangan: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = file ditandatangani oleh { $signer }, bukan Microsoft
core-signature-not-microsoft-unnamed = file ditandatangani oleh pihak lain, bukan Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = status tanda tangannya adalah { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = Apa yang diperlukan program ini dari Windows tidak dapat dibaca, jadi apakah program ini dapat berjalan di PC lain tidak akan diperiksa.
core-warning-build-external-manifest = Manifes aplikasi eksternal terdeteksi; interpretasi dependensi side-by-side ditunda.
core-warning-build-non-exe = File yang dipilih tidak menggunakan ekstensi .exe; pemindaian PE akan dilanjutkan.
core-warning-build-stand-in-program = Tidak ditemukan file aplikasi biasa yang dapat dieksekusi, jadi file ini digunakan sebagai penggantinya. Hasil arsitektur dan dependensi menggambarkan file ini, dan mungkin tidak menggambarkan aplikasinya.
core-warning-build-other-apps-files-unknown = Tidak semua data Windows Installer tentang aplikasi lain dapat dibaca, jadi folder yang tercatat milik aplikasi ini dapat berisi file aplikasi lain tanpa ada keterangan di sini.
core-warning-build-app-folder-has-another-apps-file = Windows Installer menyebutkan bahwa aplikasi lain memiliki file di folder tempat program aplikasi ini berada, dan file itu ikut dibawa, karena seluruh folder itu diambil.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = Entri App Paths aplikasi ini menyebutkan bahwa aplikasi mencari di folder yang bukan miliknya, jadi folder itu tidak ikut dibawa. Aplikasi mungkin tidak dapat dimulai di PC yang tidak memiliki isi folder itu.
core-warning-build-uninstaller-folder-left-out = Folder tempat penghapus instalan aplikasi ini berada tidak disertakan, karena Windows Installer menyebutkan bahwa aplikasi lain memiliki file di sana.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    Sesuatu yang tercatat untuk aplikasi ini menyebutkan folder yang tidak disertakan, karena { $why ->
        [unreadable] jalurnya keluar dari drive atau melewati tautan, sehingga tidak aman untuk dibaca
        [missing] folder itu tidak ada
        [link] folder itu melewati tautan
        [short-name] folder itu tidak ditulis dengan namanya yang sebenarnya
        [another-account] folder itu berada di folder akun lain
        [one-account] aplikasi ini diinstal untuk satu akun, dan folder itu bukan milik akun tersebut
        [shared] folder itu berisi hal yang digunakan bersama, atau merupakan bagian dari Windows
        [generic-name] namanya tidak menunjukkan siapa pemiliknya
        [publisher] namanya hanya mengacu pada penerbit, dan mungkin berisi produk lain dari penerbit itu
        [holds-the-app] folder itu berisi folder aplikasi ini sendiri, dan mungkin berisi lebih banyak lagi
        [another-apps-name] folder itu menggunakan nama aplikasi lain
        [another-apps-record] catatan instalasinya milik aplikasi lain
        [in-another-apps-folder] folder itu berada di dalam folder aplikasi lain
       *[another-apps] folder itu milik aplikasi lain
    }.
core-warning-build-app-folder-holds-another-app = Aplikasi lain di PC ini tercatat berada di dalam folder aplikasi ini, dan ikut dibawa bersamanya, karena seluruh folder itu diambil.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = Aplikasi ini berasal dari Windows 32-bit. Kunci registrinya ditata seperti cara Windows 64-bit menyimpan kunci aplikasi 32-bit, dan penginstal menawarkan Program Files (x86) untuknya.
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = Sebuah program memerlukan { $dll }, build debug dari runtime Visual C++. Microsoft tidak menyediakan penginstal untuk itu, dan mungkin tidak mengizinkannya diteruskan, jadi program itu tidak akan berjalan di PC tanpa Visual Studio.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } adalah program .NET yang dikemas dalam satu file, yang tidak menyebutkan .NET mana yang diperlukannya, jadi tidak ada yang direncanakan untuknya. Jika program itu memerlukan .NET, program itu tidak akan dimulai di PC yang tidak memilikinya.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
       *[other] { $count } program aplikasi ini tidak diperiksa untuk runtime yang diperlukannya, karena programnya sangat banyak atau sangat besar.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = Aplikasi ini memerlukan { $runtime }, dan tidak ada penginstal untuknya di sini, jadi tidak ada yang dijalankan. Instal sendiri di PC lain.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = Tidak disertakan dalam paket: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = Dependensi tidak dapat ditemukan: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } hanya ditemukan melalui jalur pencarian PC ini, dan bukan bagian dari aplikasi, jadi tidak ikut dibawa. Aplikasi mungkin tidak dapat dimulai di PC yang tidak memilikinya.
core-warning-build-driver-detected = Driver atau komponen kernel terdeteksi. Driver milik aplikasi akan dibundel dan didaftarkan ulang; driver vendor atau bawaan Windows tidak didistribusikan ulang dan harus diinstal ulang dari paket vendornya yang ditandatangani.
core-warning-build-near-no-longer-offered = Sesuatu yang dicentang dalam daftar hal yang mungkin juga milik aplikasi ini sudah tidak ada di daftar itu, jadi tidak ditambahkan. Periksa daftar itu lagi.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    Sebagian dari apa yang disebut { $app_name } sebagai miliknya tidak disertakan: { $why ->
        [unreadable] jalurnya keluar dari drive atau melewati tautan, sehingga tidak aman untuk dibaca
        [missing] bagian itu tidak ada
        [link] bagian itu melewati tautan
        [short-name] bagian itu tidak ditulis dengan namanya yang sebenarnya
        [another-account] bagian itu berada di folder akun lain
        [one-account] aplikasi ini diinstal untuk satu akun, dan bagian itu bukan milik akun tersebut
        [shared] bagian itu berisi hal yang digunakan bersama, atau merupakan bagian dari Windows
        [generic-name] namanya tidak menunjukkan siapa pemiliknya
        [publisher] namanya hanya mengacu pada penerbit, dan mungkin berisi produk lain dari penerbit itu
        [holds-the-app] bagian itu berisi folder aplikasi ini sendiri, dan mungkin berisi lebih banyak lagi
        [another-apps-name] bagian itu menggunakan nama aplikasi lain
        [another-apps-record] catatan instalasinya milik aplikasi lain
        [in-another-apps-folder] bagian itu berada di dalam folder aplikasi lain
       *[another-apps] bagian itu milik aplikasi lain
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = Kunci registri yang tercantum dalam catatan instalasi { $app_name } tidak disertakan, karena catatan itu juga menyebutkan hal yang merupakan milik aplikasi lain.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = Sebuah program berada di folder tempat Windows menyimpan apa yang ditulis aplikasi untuk satu akun. Windows tidak pernah menaruh program di sana, jadi sesuatu yang lain melakukannya, dan program itu tidak disertakan.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } tidak disertakan: item ini melewati tautan di drive yang tersambung, atau milik akun selain akun yang datanya sedang disertakan.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = Folder { $folder } milik { $user } berada di { $path }, yang tidak ada di drive ini, jadi isinya tidak disertakan.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = Folder { $folder } milik { $user } berada di { $path }, yang tidak ada di disk PC ini, jadi isinya tidak disertakan.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = Tidak dapat membaca { $path } ({ $error }); data aplikasi yang disimpan di sana tidak disertakan.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = Tidak dapat membaca { $path } ({ $error }); data aplikasi di subfolder di sana mungkin tidak lengkap.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = Tidak dapat membaca entri sistem file: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
       *[other] { $count } tautan, atau nama yang akan membuka hal lain, tidak diikuti di folder ini, jadi apa yang ditujunya tidak ada di penginstal: { $names }
    }{ $more ->
        [0] .
       *[other] , dan { $more } lainnya.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
       *[other] { $count } folder berada lebih dari { $depth } tingkat di bawah, jadi isinya tidak ada di penginstal: { $names }
    }{ $more ->
        [0] .
       *[other] , dan { $more } lainnya.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = Tidak dapat membaca metadata file: { $error }
core-warning-build-reserved-name = Nama file ini dicadangkan untuk perangkat Windows, jadi file ini tidak disalin.
# { $error } is what went wrong.
core-warning-build-hash-failed = Tidak dapat menghitung hash file: { $error }
core-warning-build-cloud-placeholder = File cloud yang hanya tersedia online (mis. File Sesuai Permintaan OneDrive) tidak memiliki data lokal; sematkan atau unduh file itu sebelum pencadangan. Dilaporkan tetapi tidak disalin.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = File driver/kernel '{ $file_name }' tidak disalin sebagai file biasa. Jika file itu mendukung layanan driver terdaftar untuk aplikasi ini, penginstal akan membundelnya dan mendaftarkan ulang driver secara otomatis; jika tidak, instal ulang dengan paket driver vendor yang ditandatangani.
core-warning-build-file-gone = Item ini ada saat aplikasi dipindai dan sekarang sudah hilang, jadi tidak ada di penginstal.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = Tidak dapat menyalin file ke dalam paket: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
       *[other] Paket ini memiliki file yang isinya tidak dapat diperiksa untuk mencari keterangan tentang asalnya: { $names }. Isinya mungkin menyebutkan hal itu.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = Aplikasi ini memiliki lebih dari { $count } folder kosong, dan hanya sebanyak itu yang akan dibuat ulang.
core-warning-build-credential-by-now = Tidak disertakan dalam paket: item ini sekarang tampak seperti kredensial.
core-warning-build-no-package-path = File yang dapat disalin tidak memiliki jalur relatif paket.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = Tidak dapat menempatkan prasyarat yang dibundel { $file_name }: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = Tidak dapat mengunduh { $runtime } ({ $error }); penginstal akan mencoba winget di PC target.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = Unduhan { $runtime } gagal dalam pemeriksaan tanda tangan, jadi tidak dibundel: { $reason }. Penginstal akan mencoba winget di PC target.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } telah diunduh tetapi hash-nya tidak dapat dihitung ({ $error }); penginstal akan mencoba winget di PC target sebagai gantinya.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = Tidak dapat membundel citra driver untuk '{ $service }': { $error }
core-warning-build-license-not-copied = Data lisensi tidak dapat disalin (hilang atau tidak dapat dibaca); hanya dicatat sebagai metadata.
core-warning-build-license-file-credential = File lisensi tampak seperti kredensial, dan tidak disertakan.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = Tidak dapat menelusuri sebagian penyimpanan lisensi: { $error }
core-warning-build-license-store-credential = Sebuah file di penyimpanan lisensi tampak seperti kredensial, dan tidak disertakan.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = Melewati file penyimpanan lisensi: { $error }
