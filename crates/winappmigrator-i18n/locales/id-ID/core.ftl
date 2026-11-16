# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
       *[other] { $count } pintasan di drive ini tidak menyebutkan file, jadi tidak dapat diketahui apakah pintasan tersebut membuka aplikasi ini.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = Sebagian catatan { $app_name } tidak dapat dibaca, jadi apa pun yang dijelaskannya masih ada di tempatnya, dan file yang digantinya mungkin tidak dikembalikan.
core-rollback-settings-entry-stays = Entri aplikasi di Pengaturan tetap ada, agar proses ini dapat dijalankan lagi dari sana.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } telah diinstal, atau mungkin telah diinstal, dan tetap ada, karena program lain mungkin menggunakannya.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = Apa yang ditempatkan instalasi ini di { $folder } berada di drive yang tidak tersambung. Sambungkan drive itu, lalu proses ini dapat diselesaikan.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = Apa yang ditempatkan instalasi ini di { $folder } tetap ada, karena bagian catatan itu tidak dapat dibaca.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = ada sesuatu yang mengubah { $folder } saat proses ini berjalan, jadi tidak ada lagi yang dilakukan di dalamnya. Tutup apa yang melakukannya, lalu jalankan ini lagi
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = { $folder } tidak dapat dibuka: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = tidak ada catatan tentang lokasi { $folder } sebelumnya
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } sekarang mengarah ke { $actual }, bukan ke tempatnya saat instalasi
# Why a service was not stopped.
core-rollback-reason-did-not-stop = layanan tidak berhenti tepat waktu

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = Folder { $path } dibiarkan: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = Menghapus folder { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = Folder { $path } tetap ada, karena berisi file yang tidak ditempatkan oleh instalasi ini.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = Folder { $path } tidak dapat dihapus: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = File { $path } dibiarkan: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } tidak dikembalikan, karena folder tempatnya berada sudah tidak ada.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = File parsial tidak dapat dihapus: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = { $path } tidak dapat diperiksa: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = Menghapus { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = { $path } tidak dapat dihapus: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = Mengembalikan { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = { $path } tidak dapat dikembalikan: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = cadangannya sudah tidak ada
# { $path } is a file.
core-rollback-no-parent-folder = { $path } tidak memiliki folder induk

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } tetap ada, karena masih digunakan oleh sesuatu yang tetap ada.
        [changed] { $path } tetap ada, karena telah berubah sejak diinstal.
       *[not-asked] { $path } tetap ada, karena tidak diminta untuk dihapus.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } tetap ada, karena masih digunakan oleh sesuatu yang tetap ada, dan file yang digantinya tidak dapat ditempatkan di sampingnya: semua nama untuknya sudah dipakai.
        [changed] { $path } tetap ada, karena telah berubah sejak diinstal, dan file yang digantinya tidak dapat ditempatkan di sampingnya: semua nama untuknya sudah dipakai.
       *[not-asked] { $path } tetap ada, karena tidak diminta untuk dihapus, dan file yang digantinya tidak dapat ditempatkan di sampingnya: semua nama untuknya sudah dipakai.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } tetap ada, karena masih digunakan oleh sesuatu yang tetap ada. File yang digantinya ada di sampingnya, sebagai { $name }.
        [changed] { $path } tetap ada, karena telah berubah sejak diinstal. File yang digantinya ada di sampingnya, sebagai { $name }.
       *[not-asked] { $path } tetap ada, karena tidak diminta untuk dihapus. File yang digantinya ada di sampingnya, sebagai { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } tetap ada, karena masih digunakan oleh sesuatu yang tetap ada, dan file yang digantinya tidak dapat ditempatkan di sampingnya: { $error }.
        [changed] { $path } tetap ada, karena telah berubah sejak diinstal, dan file yang digantinya tidak dapat ditempatkan di sampingnya: { $error }.
       *[not-asked] { $path } tetap ada, karena tidak diminta untuk dihapus, dan file yang digantinya tidak dapat ditempatkan di sampingnya: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (Default)
core-rollback-setting-other-account =
    { $kind ->
        [key] Kunci { $name } tetap ada, karena milik akun yang menginstal ini, yang saat ini tidak masuk.
        [user-variable] Variabel lingkungan pengguna { $name } tetap ada, karena milik akun yang menginstal ini, yang saat ini tidak masuk.
        [machine-variable] Variabel lingkungan sistem { $name } tetap ada, karena milik akun yang menginstal ini, yang saat ini tidak masuk.
        [user-list] { $name } pengguna tetap ada, karena milik akun yang menginstal ini, yang saat ini tidak masuk.
        [machine-list] { $name } sistem tetap ada, karena milik akun yang menginstal ini, yang saat ini tidak masuk.
       *[value] Nilai registri { $name } tetap ada, karena milik akun yang menginstal ini, yang saat ini tidak masuk.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] Kunci { $name } tidak dapat dijangkau: { $error }.
        [user-variable] Variabel lingkungan pengguna { $name } tidak dapat dijangkau: { $error }.
        [machine-variable] Variabel lingkungan sistem { $name } tidak dapat dijangkau: { $error }.
        [user-list] { $name } pengguna tidak dapat dijangkau: { $error }.
        [machine-list] { $name } sistem tidak dapat dijangkau: { $error }.
       *[value] Nilai registri { $name } tidak dapat dijangkau: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] Kunci { $name } tidak dapat dibaca: { $error }.
        [user-variable] Variabel lingkungan pengguna { $name } tidak dapat dibaca: { $error }.
        [machine-variable] Variabel lingkungan sistem { $name } tidak dapat dibaca: { $error }.
        [user-list] { $name } pengguna tidak dapat dibaca: { $error }.
        [machine-list] { $name } sistem tidak dapat dibaca: { $error }.
       *[value] Nilai registri { $name } tidak dapat dibaca: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] Variabel lingkungan pengguna { $name } tetap ada, karena telah berubah sejak diinstal.
        [machine-variable] Variabel lingkungan sistem { $name } tetap ada, karena telah berubah sejak diinstal.
        [user-list] { $name } pengguna tetap ada, karena telah berubah sejak diinstal.
        [machine-list] { $name } sistem tetap ada, karena telah berubah sejak diinstal.
       *[value] Nilai registri { $name } tetap ada, karena telah berubah sejak diinstal.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] Mengembalikan variabel lingkungan pengguna { $name }
        [machine-variable] Mengembalikan variabel lingkungan sistem { $name }
        [user-list] Mengembalikan { $name } pengguna
        [machine-list] Mengembalikan { $name } sistem
       *[value] Mengembalikan nilai registri { $name }
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] Variabel lingkungan pengguna { $name } tidak dapat dikembalikan: { $error }.
        [machine-variable] Variabel lingkungan sistem { $name } tidak dapat dikembalikan: { $error }.
        [user-list] { $name } pengguna tidak dapat dikembalikan: { $error }.
        [machine-list] { $name } sistem tidak dapat dikembalikan: { $error }.
       *[value] Nilai registri { $name } tidak dapat dikembalikan: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = Nilai registri { $name } tetap ada, karena tidak diminta untuk dihapus.
core-rollback-key-holds-others = Kunci { $name } tetap ada, karena berisi hal yang tidak ditempatkan oleh instalasi ini.
core-rollback-key-removed = Menghapus kunci { $name }
core-rollback-key-not-removed = Kunci { $name } tidak dapat dihapus: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] Mengeluarkan entri instalasi ini dari { $name } pengguna
       *[machine-list] Mengeluarkan entri instalasi ini dari { $name } sistem
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] Entri instalasi ini tidak dapat dikeluarkan dari { $name } pengguna: { $error }.
       *[machine-list] Entri instalasi ini tidak dapat dikeluarkan dari { $name } sistem: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] Driver { $name } tidak dapat dibaca: { $error }.
       *[service] Layanan { $name } tidak dapat dibaca: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] Driver { $name } tetap ada, karena telah berubah sejak diinstal.
       *[service] Layanan { $name } tetap ada, karena telah berubah sejak diinstal.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] Driver { $name } tidak dapat dihentikan ({ $reason }), jadi apa yang sedang digunakannya mungkin tetap ada sampai PC dimulai ulang.
       *[service] Layanan { $name } tidak dapat dihentikan ({ $reason }), jadi apa yang sedang digunakannya mungkin tetap ada sampai PC dimulai ulang.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] Driver { $name } dihentikan untuk melakukan ini. Mulai lagi saat Anda memerlukannya.
       *[service] Layanan { $name } dihentikan untuk melakukan ini. Mulai lagi saat Anda memerlukannya.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] Driver { $name } tidak dapat dihapus: { $reason }.
       *[service] Layanan { $name } tidak dapat dihapus: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] Driver { $name } ditandai untuk dihapus, dan akan hilang saat PC dimulai ulang atau saat apa pun yang membukanya melepaskannya.
       *[service] Layanan { $name } ditandai untuk dihapus, dan akan hilang saat PC dimulai ulang atau saat apa pun yang membukanya melepaskannya.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] Menghapus driver { $name }
       *[service] Menghapus layanan { $name }
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] Driver { $name } tidak dapat dikembalikan seperti semula: { $reason }.
       *[service] Layanan { $name } tidak dapat dikembalikan seperti semula: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] Mengembalikan pengaturan driver { $name }
       *[service] Mengembalikan pengaturan layanan { $name }
    }
core-rollback-service-not-restarted = Layanan { $name } berjalan sebelum instalasi ini, dan tidak dimulai lagi: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = Layanan { $name } berjalan sebagai { $account }. Kata sandinya tidak disimpan, jadi atur lagi akunnya di Layanan.

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = Aturan firewall { $name } tidak dapat dibaca: { $error }.
core-rollback-firewall-says-otherwise = Aturan firewall { $name } tetap ada, karena aturan dengan nama itu yang ada sekarang menyatakan hal lain.
core-rollback-firewall-changed = Aturan firewall { $name } tetap ada, karena telah berubah sejak diinstal.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = Menghapus aturan firewall { $name }
core-rollback-firewall-not-removed = Aturan firewall { $name } tidak dapat dihapus: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = Folder tugas { $path } tetap ada, karena berisi tugas yang tidak ditempatkan oleh instalasi ini.
core-rollback-task-folder-not-removed = Folder tugas { $path } tidak dapat dihapus: { $error }.
core-rollback-task-not-read = Tugas { $name } tidak dapat dibaca: { $error }.
core-rollback-task-changed = Tugas { $name } tetap ada, karena telah berubah sejak diinstal.
core-rollback-task-removed = Menghapus tugas { $name }
core-rollback-task-not-removed = Tugas { $name } tidak dapat dihapus: { $reason }.
core-rollback-task-put-back = Mengembalikan tugas { $name }
core-rollback-task-not-put-back = Tugas { $name } tidak dapat dikembalikan: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = Catatan instalasi ini tidak dapat dibaca, jadi tidak ada yang diurungkan: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = Tidak ada yang diurungkan: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } Jika instalasi itu tidak akan pernah dapat diurungkan, administrator dapat mengabaikannya dengan menghapus { $folder }.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = instalasi sebelumnya yang tidak pernah selesai tidak dapat diurungkan sepenuhnya, jadi tidak ada yang diinstal. Tutup apa pun yang menggunakan item berikut, lalu coba lagi. { $details }
core-rollback-install-busy = instalasi ini sedang diubah oleh penginstal atau penghapus instalan lain

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = Diurungkan: { $count }
core-rollback-count-failed = Tidak dapat diurungkan: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = Dibiarkan di tempatnya: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = Tertinggal: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32-bit
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64-bit
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP atau yang lebih lama

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = membaca drive yang tersambung atau pengaturan akun lain memerlukan hak administrator
core-system-no-program-data = folder ProgramData di PC ini tidak dapat ditemukan
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = bukan file hive: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } berukuran { $size } byte, terlalu besar untuk sebuah hive registri, atau melebihi jumlah total semua hive di drive ini
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = hive offline tidak memiliki pemetaan untuk HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } gagal: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } gagal: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } bukan drive sistem Windows (tidak ada folder \Windows).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } adalah drive sistem milik PC ini sendiri. Pilih "PC ini" untuk membaca PC ini.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } bukan drive sistem Windows: jalan menuju registrinya melewati tautan.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = Akun { $account } tidak tercatat di sini, jadi tidak ada data atau pengaturan per pengguna milik siapa pun yang disertakan.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = Akun yang masuk di sini, { $account }, tidak memiliki profil yang tercatat, jadi tidak ada data atau pengaturan per pengguna milik siapa pun yang disertakan.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = Tidak dapat mengetahui siapa yang sedang masuk ({ $reason }), jadi tidak ada data atau pengaturan per pengguna milik siapa pun yang disertakan. Pilih data milik siapa yang akan disertakan.
core-warning-scan-last-account-unknown = Drive ini tidak menyebutkan siapa yang terakhir masuk, dan memiliki lebih dari satu akun, jadi tidak ada data atau pengaturan per pengguna milik siapa pun yang disertakan. Pilih data milik siapa yang akan disertakan.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = Pengaturan { $account } tidak dapat dibaca, jadi tidak disertakan: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = Kelas COM dan jenis file yang terdaftar untuk { $account } tidak dapat dibaca, jadi tidak disertakan: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
       *[other] Jalur yang tercatat di drive ini tidak dapat dibaca, jadi tidak ada isinya yang disertakan. Setiap jalur berada di disk lain, bukan jalur lengkap, atau melewati tautan yang akan mengarah ke luar drive: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = Beberapa jalur yang tercatat di drive ini tidak dapat dibaca, jadi tidak ada isinya yang disertakan. Setiap jalur berada di disk lain, bukan jalur lengkap, atau melewati tautan yang akan mengarah ke luar drive: { $paths } dan { $more } lainnya
core-warning-scan-network-paths-not-followed =
    { $count ->
       *[other] Jalur yang tercatat di PC ini tidak dapat dibaca, jadi tidak ada isinya yang disertakan. Setiap jalur tidak berada di disk PC ini, sehingga memeriksanya akan membuat sambungan ke sana, atau memiliki nama yang akan membuka hal lain: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = Beberapa jalur yang tercatat di PC ini tidak dapat dibaca, jadi tidak ada isinya yang disertakan. Setiap jalur tidak berada di disk PC ini, sehingga memeriksanya akan membuat sambungan ke sana, atau memiliki nama yang akan membuka hal lain: { $paths } dan { $more } lainnya

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = Pintasan di folder milik akun ini
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
       *[other] { $count } pintasan, yang hanya disertakan dengan opsi Data pengguna
    }
core-account-left-out-variables = Variabel lingkungan milik akun ini
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
       *[other] { $count } variabel, yang hanya disertakan dengan opsi Data pengguna
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = Tugas terjadwal { $task }
core-account-task-works-on-account-folder = Tugas ini bekerja pada suatu tempat di folder akun, atau dinamai sesuai tempat itu, dan hanya disertakan dengan opsi Data pengguna
core-account-task-runs-from-account-folder = Apa yang juga dijalankannya dari folder akun hanya disertakan dengan opsi Data pengguna
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = Layanan { $service }
core-account-service-works-on-account-folder = Layanan ini bekerja pada suatu tempat di folder akun, dan hanya disertakan dengan opsi Data pengguna

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = jalur file milik aplikasi itu sendiri
core-account-part-registry = nilai registri
core-account-part-settings-entry = entri aplikasi di Pengaturan
core-account-part-main-program = lokasi program utamanya
core-account-part-shortcuts = pintasan
core-account-part-tasks = tugas terjadwal
core-account-part-services = layanan
core-account-part-variables = variabel lingkungan
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] Paket ini menyebutkan nama akun di { $places }. Paket ini dibuat tanpa data akun mana pun, dan siapa pun yang memiliki penginstalnya dapat membaca hal ini.
        [folder] Paket ini menyebutkan lokasi folder akun di { $places }. Paket ini dibuat tanpa data akun mana pun, dan siapa pun yang memiliki penginstalnya dapat membaca hal ini.
        [computer] Paket ini menyebutkan nama PC di { $places }. Paket ini dibuat tanpa data akun mana pun, dan siapa pun yang memiliki penginstalnya dapat membaca hal ini.
       *[id] Paket ini menyebutkan ID akun di { $places }. Paket ini dibuat tanpa data akun mana pun, dan siapa pun yang memiliki penginstalnya dapat membaca hal ini.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
       *[other] { $count } folder di luar aplikasi tidak dapat dibaca, jadi data aplikasi yang ada di sana mungkin tidak lengkap.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = tampak seperti kredensial
core-secret-why-license-option-off = opsi Data lisensi dan aktivasi nonaktif
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
       *[other] { $count } nilai, yang hanya disertakan dengan opsi Data pengguna
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (nilai yang namanya berisi kredensial)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = argumen yang berisi kredensial
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } dalam cara { $name } dijalankan
        [task] { $option } dalam cara tugas { $name } dijalankan
        [shortcut] { $option } dalam pintasan { $name }
        [opener] { $option } dalam apa yang membuka { $name }
        [opener-unnamed] { $option } dalam apa yang membuka suatu jenis file
       *[uninstall] { $option } dalam cara instalan { $name } dihapus
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = Definisi tugas { $task }, yang tidak dapat dibaca untuk diperiksa
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = Variabel lingkungan { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = Apa yang menurut kelas { $class } dijalankannya
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = Deskripsi aturan firewall { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = File lisensi terdeteksi: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = Penyimpanan lisensi { $framework } ada
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = Variabel lingkungan lisensi { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = Nilai registri menunjukkan data lisensi: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = Nilai registri menunjukkan data lisensi
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = Kunci registri menunjukkan data lisensi: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = Data lisensi/aktivasi terdeteksi. Jika opsi Data lisensi dan aktivasi aktif, penginstal memulihkan apa yang dapat dipindahkan; nilai untuk seluruh mesin dan penyimpanan yang terikat perangkat keras hanya dilaporkan. WinAppMigrator tidak pernah mendekripsi, melewati aktivasi, atau mengemulasi dongle.
core-warning-scan-license-machine-bound = Beberapa lisensi terikat ke mesin (dongle perangkat keras, aktivasi yang terkunci ke mesin, atau dienkripsi). Penginstal memulihkan apa yang dapat dipulihkan; lisensi ini mungkin tetap perlu diaktifkan ulang di PC baru.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = Tidak dapat membaca subkunci App Paths: { $error }
core-warning-scan-registry-limit = Pengambilan registri mencapai batas keamanan yang dikonfigurasi.
core-warning-scan-registry-subtree-limit = Pengambilan subpohon registri mencapai batas keamanan yang dikonfigurasi.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = Tidak dapat membaca kunci registri: { $error }
core-warning-scan-registry-value-too-large = Nilai registri melebihi batas ukuran pengambilan yang dikonfigurasi.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = Folder tugas terjadwal di drive ini melewati tautan, jadi tugasnya tidak dibaca.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = Tugas terjadwal { $task } tidak disertakan: tugas ini berjalan sebagai akun selain akun yang pengaturannya sedang disertakan, file-nya dapat diubah oleh selain administrator, atau aplikasi memiliki terlalu banyak tugas untuk disimpan.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = Tidak dapat membaca sebagian tugas terjadwal: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = Tidak dapat membaca tugas terjadwal { $path }: { $reason }. Jika tugas itu menjalankan aplikasi, tugas itu tidak disertakan.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = ukurannya { $size } byte, terlalu besar untuk sebuah tugas
# Why a task's file could not be read.
core-task-ends-mid-character = file berakhir di tengah karakter
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = file bukan UTF-16 yang valid: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = file bukan UTF-8 atau UTF-16 yang valid: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = Pemulihan menggunakan Register-ScheduledTask dengan definisi XML yang diarsipkan.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = file tidak dapat dibaca: { $reason }
core-pe-headers-misaligned = header-nya tidak berada pada batas yang seharusnya untuk sebuah program
# { $size } is the file's size in bytes.
core-pe-too-large = ukurannya { $size } byte, terlalu besar untuk sebuah program
# PE is the name of the format Windows programs are in.
core-pe-no-headers = file tidak memiliki header PE

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = Tabel impor tidak dapat dibaca: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = Nama deskriptor impor tidak dapat dibaca: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
       *[other] { $count } deskriptor delay-load lama (v1) dilewati; hanya impor delay modern berbasis RVA yang diurai.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = file yang dapat dieksekusi tidak memiliki direktori induk: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL berada di samping file yang dapat dieksekusi yang dipilih
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = Tidak dapat memindai dependensi rekursif: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } berada di folder Windows di PC asal aplikasi ini, dan Windows 10 serta 11 tidak menyertakannya. Sesuatu telah menginstalnya di sana, jadi file itu bukan bagian dari file milik aplikasi itu sendiri dan tidak ikut dibawa. Aplikasi mungkin tidak dapat dimulai sampai apa pun yang menyediakannya diinstal.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = Tidak dapat menghitung hash dependensi: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = Variabel lingkungan SystemRoot/windir tidak tersedia; pencarian DLL sistem terbatas.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = Tidak dapat membaca arsitektur dependensi: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = Tidak dapat membaca kunci registri layanan: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = Tidak dapat membaca kunci registri layanan untuk penemuan driver: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = Tidak dapat mengkueri pintasan: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = Tidak dapat mengurai output kueri pintasan: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = Tidak dapat membaca kunci registri lingkungan mesin: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] Driver { $kind } milik aplikasi '{ $name }' ({ $path }, ditandatangani oleh { $signer }). Penginstal membundel file .sys-nya dan mendaftarkan ulang layanan driver secara otomatis (sc create, memerlukan hak administrator), lalu memastikan layanan itu terdaftar.
       *[no] Driver { $kind } milik aplikasi '{ $name }' ({ $path }, tanda tangan tidak diketahui). Penginstal membundel file .sys-nya dan mendaftarkan ulang layanan driver secara otomatis (sc create, memerlukan hak administrator), lalu memastikan layanan itu terdaftar.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] Driver { $kind } vendor/bawaan Windows '{ $name }' ({ $path }, ditandatangani oleh { $signer }). WinAppMigrator tidak mendistribusikan ulang driver pihak ketiga; instal ulang driver itu dengan paket driver vendor yang ditandatangani di PC target sebelum menjalankan aplikasi.
       *[no] Driver { $kind } vendor/bawaan Windows '{ $name }' ({ $path }, tanda tangan tidak diketahui). WinAppMigrator tidak mendistribusikan ulang driver pihak ketiga; instal ulang driver itu dengan paket driver vendor yang ditandatangani di PC target sebelum menjalankan aplikasi.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Bawaan Windows
core-runtime-handling-ships-with-app = Bawaan aplikasi
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Fitur Windows: { $feature }
core-runtime-handling-included = Disertakan
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = Diinstal dengan winget: { $id }
core-runtime-handling-manual = Instal secara manual

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = menolak mengunduh { $url }: bukan HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = inisialisasi TLS gagal: { $error }
core-runtime-download-failed = unduhan { $url } gagal: { $error }
core-runtime-download-unreadable = pembacaan { $url } gagal: { $error }
core-runtime-download-empty = unduhan { $url } tidak mengembalikan data
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = unduhan { $url } terputus: hanya { $came } dari { $expected } byte yang diterima

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = tidak ada penginstal yang dikenal untuk runtime ini
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = paket menyebutkan { $path } sebagai penginstal runtime ini, padahal itu bukan tempat seharusnya, jadi tidak dijalankan
# { $path } is a file.
core-runtime-payload-missing = penginstal yang dibundel tidak ada: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = jalur penginstal yang dibundel tidak valid: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = tidak ada penginstal yang dibundel atau ID winget; dapatkan dari { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = tidak dapat membuat folder untuk tempat mengekstraknya: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = penginstal yang dibundel gagal dalam pemeriksaan tanda tangan dan tidak dijalankan: { $reason }
core-runtime-payload-timed-out = penginstal yang dibundel tidak selesai dalam batas waktu dan dihentikan paksa
# { $error } is what went wrong.
core-runtime-payload-not-started = tidak dapat memulai penginstal yang dibundel: { $error }
core-runtime-installer-killed = penginstal dihentikan paksa tanpa kode keluar
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = penginstal keluar dengan kode { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM tidak dapat mengaktifkan fitur Windows { $feature } (kode keluar { $code }); payload kemungkinan tidak dapat diunduh. Aktifkan melalui Fitur Windows, atau jalankan `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM tidak dapat mengaktifkan fitur Windows { $feature } (kode keluar tidak diketahui); payload kemungkinan tidak dapat diunduh. Aktifkan melalui Fitur Windows, atau jalankan `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = pengaktifan fitur Windows { $feature } tidak selesai dalam batas waktu dan dihentikan paksa
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = DISM tidak tersedia untuk mengaktifkan fitur Windows { $feature }: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = tidak dapat mencari winget: { $error }
core-runtime-winget-not-installed = winget tidak diinstal
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } bukan tempat Windows menyimpan paketnya sendiri, jadi tidak dijalankan
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } gagal dalam pemeriksaan tanda tangan dan tidak dijalankan: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = tidak dapat menggunakan winget untuk menginstal { $id }: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } keluar dengan kode { $code }
core-runtime-winget-failed-no-code = winget install { $id } keluar dengan kode yang tidak diketahui
core-runtime-winget-timed-out = winget install { $id } tidak selesai dalam batas waktu dan dihentikan paksa
# { $error } is what went wrong.
core-runtime-winget-not-started = winget tidak tersedia untuk menginstal { $id }: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } tidak digunakan: pengguna selain administrator dapat mengubahnya
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
       *[other] Bundel Burn berisi { $count } paket Windows Installer
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
       *[other] Database Windows Installer: { $count } jalur kunci komponen
    }
core-source-inno-log = Log penghapusan instalan Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
       *[other] Log penghapusan instalan Inno Setup: { $count } jalur tercatat
    }
core-source-nsis = Tanda tangan penghapus instalan NSIS (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
       *[other] NSIS AdvUninstLog uninstall.log: { $count } jalur tercatat
    }
core-source-squirrel = Tata letak per pengguna Squirrel (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Versi Squirrel saat ini: { $version }
core-source-chocolatey = Metadata paket Chocolatey
core-source-msix = Paket MSIX / Appx
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = Identitas AppxManifest: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
       *[other] Data paket MSIX per pengguna: { $count } lokasi
    }
# App Paths is a registry key's name.
core-source-app-paths = Pendaftaran App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } mengimpor { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = Font yang disebutkan dalam catatan instalasi aplikasi
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = sertifikat ini dinamai sesuai aplikasi atau penerbitnya
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = catatan instalasi aplikasi menyebutkannya

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows tidak menyebutkan pemilik desktop ini: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = Windows tidak dapat ditanya siapa pemilik desktop ini: { $error }
core-profile-signed-in-account-no-profile = akun yang masuk di sini, { $account }, tidak memiliki profil yang tercatat
core-profile-desktop-not-a-person = desktop di sini milik { $account }, yang bukan akun milik seseorang
core-profile-nobody-signed-in-running-account-no-profile = tidak ada yang masuk di sini, dan akun yang menjalankan ini, { $account }, tidak memiliki profil yang tercatat
core-profile-nobody-signed-in-running-not-a-person = tidak ada yang masuk di sini, dan { $account }, yang menjalankan ini, bukan akun milik seseorang
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = tidak ada yang masuk di sini, dan akun ini tidak memiliki SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = lebih dari satu akun memiliki desktop dalam sesi ini: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = Tidak dapat mengetahui siapa yang sedang masuk ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = Tidak dapat mengetahui siapa yang sedang masuk ({ $reason }). Pilih data milik siapa yang akan disertakan.
