# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = apa yang akan dilakukan instalasi ini telah berubah sejak disetujui, jadi tidak ada yang diinstal
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = I/O gagal untuk { $path }: { $error }
# { $detail } is what went wrong.
core-error-registry = pembacaan registri gagal: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = penguraian PE gagal untuk { $path }: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = input tidak valid: { $detail }
# { $detail } is what went wrong.
core-error-serialization = serialisasi laporan gagal: { $detail }
# { $detail } is what went wrong.
core-error-archive = operasi arsip gagal: { $detail }
# { $detail } is what went wrong.
core-error-network = operasi jaringan gagal: { $detail }
# { $detail } is what went wrong.
core-error-time-format = pemformatan waktu gagal: { $detail }
core-error-cancelled = proses dibatalkan
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = tidak ada alasan yang diberikan

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = jenis nilai registri tidak dikenal { $kind }
core-journal-not-elevated = tidak berjalan dengan hak administrator
core-journal-needs-admin = menginstal memerlukan hak administrator, agar catatannya disimpan di tempat yang tidak dapat diubah orang lain
# { $path } is a folder.
core-journal-not-an-install-folder = ini bukan folder instalasi: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = catatan instalasi ini tidak dapat ditulis: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = jalur keluar dari akar yang dimaksud: { $path }
# { $path } is a folder.
core-journal-nothing-exists = tidak satu pun dari ini ada di PC ini: { $path }
core-journal-busy = instalasi atau penghapusan instalan lain sedang berjalan. Tunggu hingga selesai, lalu coba lagi
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = file tidak memiliki folder
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = apa yang disimpan tidak dapat dibaca
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = catatan instalasi di { $folder } tidak dapat digunakan: catatan itu kosong
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = catatan instalasi di { $folder } tidak dapat digunakan: catatan itu tidak diawali dengan keterangan tentang isinya
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = catatan instalasi di { $folder } tidak dapat digunakan: catatan itu ditulis oleh versi lain dari program ini
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = catatan instalasi di { $folder } tidak dapat digunakan: catatan itu menyebutkan bahwa dirinya milik instalasi lain
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = ini bukan ID instalasi: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = --apply dan --dry-run tidak dapat diberikan bersamaan
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = lokasi instalasi harus berupa jalur lengkap: "{ $path }"
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = lokasi instalasi itu tidak dapat digunakan: { $problem }: "{ $path }"
core-restore-needs-admin-plan = rencana pemulihan mencakup perubahan registri, layanan, tugas terjadwal, pintasan, atau target terlindungi yang memerlukan hak administrator
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = akar target memerlukan hak administrator: { $path }
core-restore-needs-admin-user-data = memulihkan data pengguna ke ProgramData atau lokasi terlindungi lainnya memerlukan hak administrator
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. Tidak semua yang telah dilakukan dapat dikembalikan: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = tidak ada tempat aman untuk menyimpan catatan yang memungkinkan instalasi ini diurungkan, jadi tidak ada yang diinstal: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } sudah ada, jadi tidak ada yang diinstal. Gunakan --force untuk menggantinya
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = aplikasi ini memiliki hal yang perlu dipulihkan ke { $root }, dan { $link } adalah tautan ke tempat lain, yang mungkin dapat diubah orang lain, jadi tidak ada yang diinstal. Jika yang ditempatkan di sana adalah data pengguna atau data lisensi, instal aplikasi tanpa data pengguna dan data lisensinya. Jika itu pintasan di folder Startup, kembalikan folder itu ke tempat semula
core-restore-hash-check-failed = paket gagal dalam pemeriksaan hash dan tidak ada yang diinstal
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = paket gagal dalam pemeriksaan hash dan tidak ada yang diinstal: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = menolak menulis ke tempat instalasi menyimpan catatannya: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = menolak menulis di luar lokasi instalasi dan folder data yang dikenal: { $path }
# { $path } is a file.
core-restore-no-parent-folder = tujuan tidak memiliki folder induk: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = ada folder di tempat file ini harus ditempatkan: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } tidak lagi sama dengan yang disimpan paket, jadi tidak diinstal
# { $path } is a file in the package.
core-restore-source-is-link = { $path } adalah tautan, dan tautan tidak boleh diikuti
# { $path } is a file in the package.
core-restore-source-became-link = { $path } menjadi tautan saat sedang dibuka
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = layanan berhenti lagi dengan sendirinya
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = kelas COM ini sudah ada di PC ini, sebagai milik program lain

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = verifikasi pascapemulihan package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = memulai layanan { $name }
core-restore-would-copy = akan menyalin { $source } -> { $destination }
core-restore-copied = menyalin { $source } -> { $destination }
core-restore-would-create-empty-folder = akan membuat folder kosong { $path }
core-restore-created-empty-folder = membuat folder kosong { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = akan memulihkan registri { $key } { $value }
core-restore-would-restore-registry-key = akan memulihkan registri { $key } (kunci)
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = akan menambahkan aturan firewall { $name } untuk { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = aturan firewall { $name } sudah ada
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = menambahkan aturan firewall { $name } untuk { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = akan memulihkan layanan { $name } -> { $command } (berjalan sebagai { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = prasyarat sudah ada: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = akan menginstal prasyarat { $name } (fitur Windows)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = akan menginstal prasyarat { $name } (penginstal yang dibundel)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = akan menginstal prasyarat { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = menginstal prasyarat { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = menginstal prasyarat { $name } (perlu memulai ulang PC)
core-restore-would-restore-license = akan memulihkan lisensi { $source } -> { $destination }
core-restore-restored-license = memulihkan lisensi { $source } -> { $destination }
core-restore-would-restore-license-registry = akan memulihkan registri lisensi { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = driver tidak dipulihkan secara otomatis: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = akan menginstal driver { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = akan memulihkan tugas terjadwal { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = akan memulihkan pintasan startup { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = akan memulihkan pintasan { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] akan memulihkan variabel lingkungan pengguna { $name }
       *[machine] akan memulihkan variabel lingkungan sistem { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] membiarkan variabel lingkungan pengguna { $name } apa adanya: tidak ada bagiannya yang milik aplikasi ini
       *[machine] membiarkan variabel lingkungan sistem { $name } apa adanya: tidak ada bagiannya yang milik aplikasi ini
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] memulihkan variabel lingkungan pengguna { $name }
       *[machine] memulihkan variabel lingkungan sistem { $name }
    }
core-restore-set-uninstaller = mengatur penghapus instalan untuk { $key }
core-restore-placed-driver-image = menempatkan citra driver { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = mengonfigurasi driver { $name }
# { $name } is a driver's name.
core-restore-created-driver = membuat driver { $name }
# { $name } is a driver's name.
core-restore-started-driver = memulai driver { $name }
core-restore-ensured-registry-key = memastikan kunci registri { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = memulihkan registri { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = memulihkan registri { $key } (Default)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = mengonfigurasi layanan { $name } (berjalan sebagai { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = membuat layanan { $name } (berjalan sebagai { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = memulihkan tugas terjadwal { $name }
core-restore-restored-shortcut = memulihkan pintasan { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = Catatan instalasi yang tidak pernah selesai dibiarkan apa adanya, karena orang lain dapat mengubahnya. Apa pun yang diubah oleh instalasi itu masih ada.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
       *[other] Instalasi { $app_name } sebelumnya tidak pernah selesai, jadi instalasi itu diurungkan terlebih dahulu. { $count } perubahan telah dikembalikan.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = Instalasi { $app_name } sebelumnya tidak pernah selesai. Menginstal akan mengurungkannya terlebih dahulu.
core-warning-restore-user-data-real-locations = Data pengguna dipulihkan ke lokasi %AppData%/%ProgramData% yang sebenarnya di mesin ini, bukan ke akar target.
# { $error } is what went wrong.
core-warning-restore-commit-failed = Catatan instalasi ini tidak dapat diselesaikan: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = Layanan { $name } telah diminta untuk dimulai dan belum selesai dimulai. Layanan itu mungkin masih akan berjalan. Layanan itu juga dimulai dengan sendirinya saat PC ini dinyalakan lagi.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = Layanan { $name } telah diinstal dan tidak dimulai: { $reason }. Aplikasi mungkin tidak berfungsi sampai layanan itu berjalan. Windows akan mencoba lagi saat PC ini dinyalakan lagi.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = Layanan { $name } tidak dapat dihentikan terlebih dahulu: { $reason }. Layanan itu mungkin tetap menjalankan program lama sampai PC ini dinyalakan lagi.
core-warning-restore-install-cancelled = Instalasi dibatalkan.
core-warning-restore-preview-cancelled = Pratinjau dibatalkan.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } tidak disertakan: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = Verifikasi pascapemulihan tidak dapat dijalankan: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] { $path } tidak boleh diganti oleh paket ini, dan layanan { $name } bergantung padanya, jadi tidak ada yang diinstal: { $refusal }
        [task] { $path } tidak boleh diganti oleh paket ini, dan tugas terjadwal { $name } bergantung padanya, jadi tidak ada yang diinstal: { $refusal }
        [firewall] { $path } tidak boleh diganti oleh paket ini, dan aturan firewall { $name } bergantung padanya, jadi tidak ada yang diinstal: { $refusal }
        [shortcut] { $path } tidak boleh diganti oleh paket ini, dan pintasan { $name } bergantung padanya, jadi tidak ada yang diinstal: { $refusal }
       *[registry] { $path } tidak boleh diganti oleh paket ini, dan nilai registri { $name } bergantung padanya, jadi tidak ada yang diinstal: { $refusal }
    }
core-warning-restore-destination-exists = Tujuan sudah ada; gunakan --force untuk menimpanya.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = Tidak dapat menulis { $path }: { $error }
core-warning-restore-not-a-folder = Ada sesuatu selain folder di tempat folder kosong ini seharusnya berada.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = Tidak dapat memeriksa { $path }: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = Tidak dapat membuat { $path }: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = PC ini sudah memilikinya, dengan pengaturan lain, yang tidak diambil alih oleh instalasi
core-warning-restore-registry-review = Nilai registri dipulihkan dari data terstruktur yang diarsipkan, dengan jalur aplikasi yang dikenal ditulis ulang ke akar target.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Aturan Windows Firewall tidak dapat dibaca: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = aturan dengan nama ini untuk program ini sudah ada, dengan pengaturan lain, yang tidak diubah oleh instalasi
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: tidak ada penginstal yang dikenal untuk runtime ini.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = Menginstal { $name } memerlukan hak administrator; jalankan ulang penginstal sebagai administrator.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } telah diinstal, tetapi PC perlu dimulai ulang untuk menyelesaikannya.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = Tidak dapat mencatat runtime yang dilewati: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = Tidak dapat menginstal { $name }: { $reason }. Dapatkan dari { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = Tidak dapat mencatat runtime yang diinstal: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = Jalur lisensi dalam paket tidak valid: { $error }
core-warning-restore-license-needs-admin = Memulihkan data lisensi ke lokasi terlindungi memerlukan hak administrator.
core-warning-restore-license-exists = Tujuan lisensi sudah ada; gunakan --force untuk menimpanya.
# { $error } is what went wrong.
core-warning-restore-license-failed = Tidak dapat memulihkan data lisensi: { $error }
core-warning-restore-driver-needs-admin = Pendaftaran driver memerlukan hak administrator; jalankan ulang penginstal sebagai administrator.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = Tugas terjadwal { $name } berjalan sebagai SYSTEM, dengan hak penuh atas PC ini.
core-warning-restore-shortcut-exists = Pintasan sudah ada; gunakan --force untuk menimpanya.
core-warning-restore-shortcut-all-users = Satu atau beberapa pintasan yang dipulihkan mengarah ke desktop publik atau lokasi menu Mulai bersama.
core-warning-restore-environment-refresh = Perubahan variabel lingkungan mungkin baru terlihat di proses, shell, atau sesi masuk yang baru.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = Layanan { $name } berjalan sebagai { $account } di PC sumber. Kata sandinya tidak ada di penginstal, jadi layanan disiapkan sebagai LocalService. Jika tidak berfungsi, atur akun dan kata sandinya di Layanan.
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } belum terdaftar; seharusnya aktif setelah PC dimulai ulang.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } melaporkan berhasil tetapi tidak terdaftar; instalasinya mungkin gagal.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = Jalur driver yang dibundel tidak valid: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = Tidak dapat membaca pengaturan driver saat ini: { $error }
core-warning-restore-driver-exists = Layanan driver sudah ada; gunakan --force untuk mengonfigurasinya ulang.
core-warning-restore-driver-image-exists = Citra driver sudah ada; gunakan --force untuk menimpanya.
core-warning-restore-driver-hash-bundled = Citra driver yang dibundel tidak cocok dengan hash yang tercatat, jadi tidak diinstal.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = Tidak dapat menempatkan citra driver: { $error }
core-warning-restore-driver-hash-placed = Citra driver berubah setelah ditempatkan, jadi tidak didaftarkan.
# { $error } is what went wrong.
core-warning-restore-key-not-created = Tidak dapat membuat kunci registri: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = Tidak dapat membaca nilai yang akan diganti: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = Tidak dapat menulis nilai registri: { $error }
core-warning-restore-value-exists = Nilai registri sudah ada; gunakan --force untuk menimpanya.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = Tidak dapat membuat kunci registri lingkungan: { $error }
core-warning-restore-variable-exists = Variabel lingkungan sudah ada; gunakan --force untuk menimpanya.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = Tidak dapat menulis variabel lingkungan: { $error }
core-warning-restore-value-kind-missing = Jenis nilai registri tidak ada.
# { $error } is what went wrong.
core-warning-restore-service-not-read = Tidak dapat membaca pengaturan layanan saat ini: { $error }
core-warning-restore-service-exists = Layanan sudah ada; gunakan --force untuk mengonfigurasinya ulang.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = Gagal mengkueri tugas terjadwal: { $error }
core-warning-restore-task-exists = Tugas terjadwal sudah ada; gunakan --force untuk menimpanya.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = Tidak dapat memeriksa folder tugas: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = Pintasan tidak dapat dibuat, dan pintasan yang digantinya tidak dapat dikembalikan: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = Gagal memulai perintah: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = Perintah gagal: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = Gagal memulai PowerShell: { $error }
