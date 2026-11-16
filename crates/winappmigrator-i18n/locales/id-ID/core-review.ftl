# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = ini adalah tempat instalasi menyimpan catatannya
core-review-location-has-files = folder ini sudah berisi file, dan tidak ada instalasi yang tercatat di sini, jadi gunakan folder kosong
core-policy-location-not-full-path = harus berupa jalur lengkap
core-policy-location-parent-step = tidak boleh berisi ".."
core-policy-location-not-plain = harus berupa jalur biasa di drive PC ini
core-policy-location-whole-drive = tidak boleh berupa seluruh drive
core-policy-location-windows = tidak boleh berupa folder Windows
core-policy-location-runs-by-itself = ini adalah folder milik Windows, atau folder yang isinya berjalan atau dimuat dengan sendirinya
core-policy-location-shared = folder ini berisi hal lain secara berdampingan, jadi aplikasi memerlukan folder di dalamnya

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = aplikasi ini tidak dapat berjalan di PC ini, jadi tidak ada yang diubah: { $reason }
core-policy-architecture-x86 = x86 32-bit
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = Arm 32-bit
core-policy-architecture-unknown = arsitektur yang tidak dikenal
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = jenis mesin { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names }, dan { $count } lainnya
core-policy-fit-programs-not-said = Penginstal ini tidak menyebutkan apa yang diperlukan programnya, jadi apakah program itu dapat berjalan di PC ini tidak diperiksa.
core-policy-fit-architecture-not-told = Arsitektur PC ini tidak dapat diketahui, jadi apakah program aplikasi ini dapat berjalan di PC ini tidak diperiksa.
core-policy-fit-main-not-said = Penginstal ini tidak menyebutkan apa yang diperlukan program utamanya, jadi keterangan di sini didasarkan pada bagian lain yang dimilikinya.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = driver-nya, { $driver }, dibuat untuk { $built_for }, sedangkan PC ini { $here }. Driver harus sama persis dengan Windows, jadi aplikasi ini tidak dapat berjalan di sini
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } dibuat untuk { $built_for }, sedangkan PC ini { $here }, jadi Windows tidak akan memulainya
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = tidak satu pun programnya dapat dimulai di PC ini, yang merupakan { $here }: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = Beberapa program aplikasi ini dibuat untuk arsitektur lain dan tidak akan dimulai di PC ini, yang merupakan { $here }: { $programs }. Bagian lain aplikasi dapat dimulai, jadi hal ini mungkin tidak menjadi masalah.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = PC ini { $here }, jadi Windows akan menjalankan program berikut melalui emulator: { $programs }. Sebagian besar program dapat berjalan dengan baik seperti itu, dan sebagian tidak.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } menyebutkan bahwa program itu memerlukan Windows { $needs } atau yang lebih baru, sedangkan PC ini menjalankan { $runs }, jadi Windows tidak akan memulainya
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = Beberapa program aplikasi ini menyebutkan bahwa program tersebut memerlukan Windows yang lebih baru dari ini dan tidak akan dimulai di sini: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = Aplikasi ini berasal dari Windows build { $from_build }, sedangkan PC ini menjalankan build { $this_build }. Programnya menyebutkan bahwa program tersebut dapat dimulai di Windows ini, jadi aplikasi mungkin berfungsi, tetapi apa yang diandalkannya di sana mungkin tidak ada di sini.
core-policy-fit-build-not-told = Build Windows ini tidak dapat diketahui, jadi tidak dibandingkan dengan build tempat aplikasi ini berasal.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = mengekstrak paket
core-space-for-install-record = catatan instalasi
core-space-for-app-files = file aplikasi
core-space-for-what-is-replaced = menyimpan apa yang digantinya
core-space-for-build-work = file kerja proses pembuatan
core-space-for-installer = penginstal
core-space-for-package = paket
core-space-for-unpacking-and-installing = mengekstrak paket dan menginstalnya
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " dan " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } memerlukan ruang kosong hingga { $needed } untuk { $purposes }, dan memiliki { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } tidak menyebutkan berapa ruang yang dimilikinya, jadi apakah { $needed } yang diperlukan muat tidak diperiksa
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = hingga { $needed } di { $drive } ({ $free } kosong)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = hingga { $needed } di { $drive } (ruang kosong tidak dapat diketahui)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = ruang tidak cukup untuk ini, jadi tidak ada yang diubah. { $shortfalls }. Kosongkan sebagian ruang di sana, atau pilih lokasi instalasi di drive lain.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = Layanan
core-review-kind-driver = Driver
core-review-kind-scheduled-task = Tugas terjadwal
core-review-kind-autostart = Mulai otomatis
core-review-kind-shortcut = Pintasan
core-review-kind-path = PATH
core-review-kind-environment-variable = Variabel lingkungan
core-review-kind-shell-extension = Ekstensi shell
core-review-kind-menu-command = Perintah menu
core-review-kind-link-type = Jenis tautan
core-review-kind-file-type = Jenis file
core-review-kind-com = COM
core-review-kind-firewall-rule = Aturan firewall
core-review-kind-event-log-source = Sumber log peristiwa
core-review-kind-font = Font
core-review-kind-odbc-driver = Driver ODBC
core-review-kind-odbc-source = Sumber data ODBC
core-review-kind-compatibility-setting = Pengaturan kompatibilitas
core-review-kind-default-apps = Aplikasi default
core-review-kind-program-name = Nama program
core-review-kind-other-settings = Pengaturan lainnya

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] Menjalankan { $file } sebagai { $account }, dimulai sekarang dan setiap kali PC ini dinyalakan
        [delayed] Menjalankan { $file } sebagai { $account }, dimulai sekarang dan tidak lama setelah setiap kali PC ini dinyalakan
        [manual] Menjalankan { $file } sebagai { $account }, dimulai hanya jika ada yang memintanya
        [disabled] Menjalankan { $file } sebagai { $account }, dinonaktifkan
        [other] Menjalankan { $file } sebagai { $account }, { $start }
       *[unknown] Menjalankan { $file } sebagai { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (akun kustom tidak dipulihkan)
# { $file } is a driver's file name.
core-review-driver-detail = Memuat { $file } ke dalam Windows itu sendiri
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = Memuat { $file } ke dalam Windows itu sendiri, { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = dimulai bersama Windows, sebelum apa pun yang lain
core-review-start-system = dimulai bersama Windows, saat Windows sedang dimuat
core-review-start-automatic = dimulai setiap kali PC ini dinyalakan
core-review-start-delayed = dimulai tidak lama setelah setiap kali PC ini dinyalakan
core-review-start-manual = dimulai hanya jika ada yang memintanya
core-review-start-disabled = tidak pernah dimulai, karena dinonaktifkan
core-review-start-unknown = dimulai dengan cara yang tidak disebutkan oleh Windows
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = Berjalan sebagai { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = Berjalan sebagai { $account }, dengan hak tertinggi yang dapat dimilikinya
core-review-task-runs-as-you = Anda, siapa pun yang sedang masuk
core-review-task-runs-as-creator = pengguna yang membuatnya
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    Berjalan { $scope ->
        [everyone] untuk semua orang yang masuk
       *[you] saat Anda masuk
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    Pintasan di folder Startup, { $scope ->
        [everyone] untuk semua orang di PC ini
       *[you] untuk Anda
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    Membuka { $file }, { $scope ->
        [everyone] untuk semua orang di PC ini
       *[you] untuk Anda
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = Menambahkan ke { $name }
core-review-path-install-folder = Folder instalasi
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = Folder instalasi\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = Diatur ke { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
       *[other] { $count } pengaturan di luar kunci milik aplikasi itu sendiri
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = Di { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = Di { $keys }, dan { $count } lainnya
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = domain
core-review-network-private = pribadi
core-review-network-public = publik
core-review-firewall-detail =
    { $way ->
        [allow-in] Mengizinkan sambungan masuk
        [allow-out] Mengizinkan sambungan keluar
        [block-in] Memblokir sambungan masuk
       *[block-out] Memblokir sambungan keluar
    }{ $port_count ->
        [0] { "" }
       *[other] { " " }di port { $ports }
    } untuk { $program }, di { $network_count ->
        [0] semua jaringan
       *[other] jaringan { $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    Memuat { $server } ke File Explorer, { $scope ->
        [everyone] untuk semua orang di PC ini
       *[you] untuk Anda
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    Memuat file yang diinstalnya ke File Explorer, { $scope ->
        [everyone] untuk semua orang di PC ini
       *[you] untuk Anda
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } pada { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    Menambahkan perintah ke menu { $on }, { $scope ->
        [everyone] untuk semua orang di PC ini
       *[you] untuk Anda
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    Mengubah apa yang terjadi saat salah satu dari jenis ini dibuka, { $scope ->
        [everyone] untuk semua orang di PC ini
       *[you] untuk Anda
    }
core-review-client-entry-detail = Dapat dipilih sebagai program untuk ini. Tidak ada yang dialihkan kepadanya
core-review-com-permissions-title = Izin COM
core-review-com-permissions-detail = Menentukan siapa yang boleh memulai kelas COM-nya, atau sebagai apa kelas itu berjalan
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    Tautan jenis ini dibuka dengan aplikasi ini, { $scope ->
        [everyone] untuk semua orang di PC ini
       *[you] untuk Anda
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    File jenis ini dibuka dengan aplikasi ini, { $scope ->
        [everyone] untuk semua orang di PC ini
       *[you] untuk Anda
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = Dapat menulis ke log { $log }, dengan pesan dari file-nya sendiri
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    Dimuat dari folder instalasi saat Anda masuk, { $scope ->
        [everyone] untuk semua orang di PC ini
       *[you] untuk Anda
    }
core-review-odbc-driver-detail = Dimuat ke dalam program yang menggunakan ODBC untuk menjangkau database
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    Sambungan database, { $scope ->
        [everyone] untuk semua orang di PC ini
       *[you] untuk Anda
    }
core-review-default-apps-detail = Dapat dipilih di Aplikasi default. Tidak ada yang dialihkan kepadanya
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    Dialog Jalankan dan program lain dapat menemukannya dengan nama ini, { $scope ->
        [everyone] untuk semua orang di PC ini
       *[you] untuk Anda
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM di { $server }
core-review-com-title-own-file = COM di file yang diinstalnya
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
       *[other] { $count } pengaturan kelas yang dapat dimulai oleh program lain
    }
core-review-com-support-title = Pustaka jenis, antarmuka, dan AppID COM
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = Pengaturan untuk pustaka jenis ({ $libraries }), antarmuka ({ $interfaces }), dan AppID ({ $app_ids }) miliknya, yang menyertai kelasnya
core-review-compatibility-title = Cara programnya dimulai
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
       *[other] { $count } pengaturan kompatibilitas untuk program yang diinstalnya
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = Data pengguna dan pengaturan per pengguna ditujukan untuk { $account }, yang sedang masuk di sini, bukan untuk administrator yang menjalankan ini.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = Tidak ada yang masuk ke sesi ini, jadi data pengguna dan pengaturan per pengguna ditujukan untuk { $account }, akun yang menjalankan ini.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = Tidak dapat mengetahui siapa yang sedang masuk ({ $reason }), jadi pengaturan per pengguna tidak disertakan, dan data pengguna ditempatkan di bawah lokasi instalasi.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = tidak dapat mengetahui siapa yang sedang masuk: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = Tidak ada dalam penginstal ini, karena tampak seperti kredensial
core-review-left-out-settings = Pengaturan dan riwayat yang disimpan akun untuk aplikasi ini, yang hanya disertakan dengan opsi Data pengguna
core-review-left-out-certificates = Aplikasi ini tampaknya mengandalkan sertifikat yang dipercaya di PC-nya, dan sertifikat itu tidak diinstal
core-review-left-out-fonts = Font ini menggunakan nama aplikasi, dan tidak ada yang menyebutkan bahwa font itu miliknya, jadi tidak diinstal
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = Tidak ada dalam penginstal ini, karena tampak seperti kredensial: { $names }. Apa yang ada di PC ini untuk kredensial itu tetap apa adanya, dan Anda mungkin harus memasukkannya lagi.
# { $names } is a list of names of license data.
core-review-note-licenses = Tidak ada dalam penginstal ini, karena opsi Data lisensi dan aktivasi nonaktif saat penginstal dibuat: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = Tidak ada dalam penginstal ini, karena opsi Data pengguna nonaktif saat penginstal dibuat: pengaturan dan riwayat yang disimpan akun untuk aplikasi ini, di { $names }. Aplikasi dimulai di sini seperti untuk orang yang baru pertama kali menggunakannya.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = Aplikasi ini tampaknya mengandalkan sertifikat yang dipercaya di PC-nya, dan sertifikat itu tidak diinstal: { $names }. Anda yang memutuskan apakah akan memercayainya di sini.
# { $names } is a list of font names.
core-review-note-fonts = Font ini menggunakan nama aplikasi, dan tidak ada yang menyebutkan bahwa font itu miliknya, jadi tidak diinstal: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = Tidak disertakan oleh pembuat penginstal ini: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = Ditambahkan oleh pembuat penginstal ini, karena dianggap sebagai bagian dari aplikasi: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } di { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names }, dan { $count } lainnya

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = item ini milik pengaturan satu akun, dan siapa yang sedang masuk tidak dapat diketahui
core-review-name-not-accepted = Windows tidak menerima namanya
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = pengaturannya saat ini tidak dapat dibaca: { $error }
core-review-service-dependency = layanan ini bergantung pada layanan dengan nama yang tidak valid
# { $path } is the program the service runs.
core-review-service-runs-other = layanan ini menjalankan { $path }, yang tidak diinstal oleh paket ini
core-review-service-settings = pengaturannya menyebutkan sesuatu yang tidak diinstal oleh paket ini, atau kunci yang bukan miliknya
core-review-driver-no-hash = driver ini tidak memiliki hash untuk memeriksanya
core-review-license-for-the-machine = lisensi yang disimpan untuk seluruh mesin hanya dilaporkan
core-review-license-starts-something = lisensi tidak punya alasan untuk memulai apa pun
core-review-task-not-captured = definisinya tidak diambil
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = definisinya tidak dapat dibaca: { $error }
core-review-task-no-account = tugas ini berjalan sebagai siapa pun yang menginstalnya, dan siapa yang sedang masuk di sini tidak dapat diketahui
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = pintasan ini membuka { $path }, yang tidak diinstal oleh paket ini
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } sudah ada di PC ini dan menjalankan { $path }, yang bukan berasal dari instalasi sebelumnya di sini. Jika itu adalah salinan lama aplikasi ini, hapus dulu instalannya
core-policy-key-control-characters = nama kuncinya berisi karakter kontrol
core-policy-value-control-characters = nama nilainya berisi karakter kontrol
core-policy-registry-link = ini adalah tautan registri
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = ini mewakili kunci milik PC, dan { $reason }
core-policy-registry-part = tidak ada instalasi aplikasi yang boleh menulis ke bagian registri ini
core-policy-key-refused = tidak ada instalasi aplikasi yang boleh mengubah kunci ini
core-policy-net-debugger = ini akan menetapkan debugger untuk setiap program .NET
core-policy-net-install-root = ini akan mengubah lokasi tempat setiap program .NET memuat .NET
core-policy-class-redirects = ini akan mengubah cara suatu jenis program atau folder dibuka
core-policy-app-path-name = ini menyebut program dengan sesuatu selain nama file
core-policy-app-path-not-installed = ini akan mendaftarkan nama program untuk file yang tidak diinstal oleh paket ini
core-policy-shared-default-verb = ini akan mengubah apa yang terjadi saat setiap file atau folder dari suatu jenis dibuka
core-policy-command-runs-other = ini menambahkan perintah yang menjalankan sesuatu yang tidak diinstal oleh paket ini
core-policy-autostart-not-command = ini dimulai secara otomatis, dan nilainya bukan perintah
core-policy-autostart-runs-other = ini dimulai secara otomatis, dan menjalankan sesuatu yang tidak diinstal oleh paket ini
core-policy-environment-own-rules = variabel lingkungan dipulihkan dengan aturannya sendiri
core-policy-service-not-restored = ini milik layanan yang tidak dipulihkan oleh paket ini
core-policy-service-own-rules = pengaturan layanan itu sendiri hanya dipulihkan melalui aturannya
core-policy-service-dll = ServiceDll-nya bukan file yang diinstal oleh paket ini
core-policy-event-log-source-only = hanya nilai milik sumber log peristiwa itu sendiri yang dapat dipulihkan
core-policy-security-log = tidak ada aplikasi yang menulis ke log Security
core-policy-event-log-messages = pesannya akan berasal dari file yang tidak diinstal oleh paket ini
core-policy-type-library = pustaka jenisnya bukan file yang diinstal oleh paket ini
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = ini akan mengubah cara { $name } dibuka, padahal program itu tidak diinstal oleh paket ini
core-policy-event-log-no-file = sumber log peristiwa harus mengambil pesannya dari file
core-policy-com-runs-other = ini menjalankan sesuatu yang bukan file yang diinstal oleh paket ini
core-policy-com-redirect = ini akan menjadikan kelas ini sesuatu yang tidak diinstal oleh paket ini
core-policy-com-server = ini menjalankan sesuatu yang bukan file yang diinstal oleh paket ini dan juga bukan bagian dari Windows
core-policy-interface = ini akan menentukan cara panggilan ke sebuah antarmuka dilakukan, dengan sesuatu yang tidak diinstal oleh paket ini
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = ini akan mengubah cara { $name } dimulai, padahal program itu tidak diinstal oleh paket ini
core-policy-app-id-surrogate = ini akan di-host oleh program yang tidak diinstal oleh paket ini
core-policy-app-id-service = ini akan berjalan di layanan yang tidak dipulihkan oleh paket ini
core-policy-app-id-run-as = ini akan berjalan sebagai akunnya sendiri
core-policy-app-id-link = ini tidak menyebutkan AppID
core-policy-app-id-remote = ini akan berjalan di tempat selain PC ini
core-policy-odbc-own-driver = itu adalah pengaturan milik ODBC sendiri, bukan driver
core-policy-odbc-driver-not-installed = driver-nya bukan file yang diinstal oleh paket ini
core-policy-loads-other-file = ini akan memuat file yang tidak diinstal oleh paket ini
core-policy-odbc-own-source = itu adalah pengaturan milik ODBC sendiri, bukan sumber data
core-policy-odbc-source-missing = ini mencantumkan sumber data yang tidak dimiliki paket ini
core-policy-odbc-source-driver = driver-nya bukan file yang diinstal oleh paket ini dan juga bukan bagian dari Windows
core-policy-shell-extension = ini akan memuat ekstensi shell yang bukan file yang diinstal oleh paket ini ke File Explorer
core-policy-not-text = isinya bukan teks, padahal akan dibaca sebagai teks
core-policy-network-place = ini menyebutkan tempat di jaringan
core-policy-font = ini akan mendaftarkan font yang tidak diinstal oleh paket ini, atau font di folder Windows
core-policy-compatibility = ini akan mengubah cara berjalannya program yang tidak diinstal oleh paket ini
core-policy-capabilities = ini mencantumkan aplikasi yang detailnya tidak ada dalam paket ini
core-policy-default-program = ini akan mengubah program yang membuka sesuatu secara default
core-policy-url-protocol = ini menyebut dirinya sebagai jenis tautan, dan tidak dibuka dengan apa pun yang diinstal oleh paket ini
core-policy-command-handler = ini menambahkan perintah yang ditangani oleh sesuatu yang tidak diinstal oleh paket ini
core-policy-installer-command = ini akan menjalankan sesuatu yang hanya dapat dibaca oleh Windows Installer
core-policy-creates-class = ini akan membuat kelas yang tidak diinstal oleh paket ini
core-policy-opens-links = ini membuka tautan dengan sesuatu yang tidak diinstal oleh paket ini
core-policy-opens-files = ini membuka file dengan sesuatu yang tidak diinstal oleh paket ini
core-policy-firewall-name = namanya tidak dapat diteruskan oleh program ini
core-policy-firewall-description = deskripsinya tidak dapat diteruskan oleh program ini
core-policy-firewall-every-program = aturan ini berlaku untuk setiap program, sedangkan aturan hanya boleh untuk satu program
# { $path } is the program the rule is for.
core-policy-firewall-program = aturan ini untuk { $path }, yang tidak diinstal oleh paket ini
core-policy-firewall-direction = aturan ini tidak menyebutkan arah yang dapat diatur oleh program ini
core-policy-firewall-action = aturan ini akan mengesampingkan aturan lain, yang tidak boleh dilakukan aturan aplikasi mana pun
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = aturan ini menyebutkan { $setting }, yang tidak dapat diatur seperti semula, atau akan melemahkan fungsi aturan lain
core-policy-firewall-service = aturan ini untuk layanan yang tidak dipulihkan oleh paket ini
core-policy-firewall-parts = sebagian isinya bukan hal yang dapat dinyatakan oleh aturan firewall
core-policy-folder-already-there = folder itu sudah ada, di luar lokasi instalasi, dan tidak ada yang menyebutkan bahwa folder itu milik aplikasi ini, jadi tidak ada program yang ditempatkan di dalamnya
core-policy-variable-name = namanya bukan nama variabel yang valid
core-policy-variable-relied-on = Windows dan program lain mengandalkan variabel ini
core-policy-task-name = namanya tidak diterima oleh Windows
core-policy-task-folder = folder-nya tidak diterima oleh Windows
core-policy-task-windows = tugas ini akan menggantikan salah satu tugas milik Windows sendiri
core-policy-task-other-action = tugas ini melakukan sesuatu selain menjalankan program
core-policy-task-no-program = tugas ini tidak menyebutkan program untuk dijalankan
# { $path } is the program the task runs.
core-policy-task-runs-other = tugas ini menjalankan sesuatu yang tidak diinstal oleh paket ini: { $path }
core-policy-task-exists-unreadable = tugas dengan nama ini sudah ada dan tidak dapat dibaca
core-policy-task-exists-other = tugas dengan nama ini sudah ada dan tidak menjalankan aplikasi ini
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = tugas dengan nama ini sudah ada dan menjalankan { $path }, yang bukan berasal dari instalasi sebelumnya di sini
core-policy-shortcut-not-lnk = ini bukan pintasan .lnk
core-policy-shortcut-place = pintasan ini tidak berada di menu Mulai atau di Desktop
core-policy-short-name = jalurnya menggunakan nama yang tidak dapat diperiksa
core-policy-shortcut-no-folder = PC ini tidak memiliki folder seperti itu untuknya
core-policy-data-startup = ini akan ditempatkan di folder Startup, dan berjalan setiap kali ada yang masuk
core-policy-data-powershell = ini akan ditempatkan di tempat PowerShell menjalankan apa yang ditemukannya saat dimulai
core-policy-data-windows = ini akan ditempatkan di folder milik Windows
core-policy-license-outside = ini berada di luar lokasi instalasi dan setiap folder data yang dikenal
core-rewrite-cdata = ini berisi bagian CDATA
core-rewrite-too-much-to-look-through = paket ini berisi terlalu banyak data dalam pengaturannya sehingga apa yang disebutkannya tidak dapat diperiksa

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] File { $item } tidak disertakan: { $reason }.
        [folder] Folder { $item } tidak disertakan: { $reason }.
        [service] Layanan { $item } tidak disertakan: { $reason }.
        [driver] Driver { $item } tidak disertakan: { $reason }.
        [firewall-rule] Aturan firewall { $item } tidak disertakan: { $reason }.
        [license-data] Data lisensi di { $item } tidak disertakan: { $reason }.
        [license-value] Nilai lisensi { $item } tidak disertakan: { $reason }.
        [scheduled-task] Tugas terjadwal { $item } tidak disertakan: { $reason }.
        [shortcut] Pintasan { $item } tidak disertakan: { $reason }.
        [environment-variable] Variabel lingkungan { $item } tidak disertakan: { $reason }.
       *[value] { $item } tidak disertakan: { $reason }.
    }
core-warning-review-package-path-missing = File yang dapat disalin tidak memiliki jalur relatif paket.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = Paket ini mencantumkan lebih dari { $count } folder kosong, dan hanya sebanyak itu yang dibuat.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
       *[other] { $count } file dari Dokumen ditempatkan di bawah { $target } sebagai gantinya, di folder bernama _external, karena folder Dokumen PC ini berada di { $folder }, dan hanya folder di dalam profil Anda yang ditulisi. Untuk menggunakannya, pindahkan sendiri file tersebut ke folder Dokumen Anda.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
       *[other] { $count } file dari Dokumen ditempatkan di bawah { $target } sebagai gantinya, di folder bernama _external, karena lokasi folder Dokumen PC ini tidak dapat diketahui. Untuk menggunakannya, pindahkan sendiri file tersebut ke folder Dokumen Anda.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
       *[other] { $count } file dari Game Tersimpan ditempatkan di bawah { $target } sebagai gantinya, di folder bernama _external, karena folder Game Tersimpan PC ini berada di { $folder }, dan hanya folder di dalam profil Anda yang ditulisi. Untuk menggunakannya, pindahkan sendiri file tersebut ke folder Game Tersimpan Anda.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
       *[other] { $count } file dari Game Tersimpan ditempatkan di bawah { $target } sebagai gantinya, di folder bernama _external, karena lokasi folder Game Tersimpan PC ini tidak dapat diketahui. Untuk menggunakannya, pindahkan sendiri file tersebut ke folder Game Tersimpan Anda.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } adalah tautan atau file, bukan folder
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } tidak dibuat oleh WinAppMigrator, dan orang lain dapat mengubah isinya. Administrator harus memeriksa dan menghapusnya: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } berisi sesuatu yang tidak dibuat oleh WinAppMigrator, dan orang lain dapat mengubahnya. Administrator harus memeriksa dan menghapusnya
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } menjadi tautan saat sedang digunakan
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } menjadi tautan saat sedang diperiksa
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = tidak dapat mengizinkan pengguna memulai { $path }: { $said }
core-privilege-elevation-unknown = tidak dapat mengetahui apakah proses ini berjalan dengan hak administrator
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = tidak dapat membatasi { $path } hanya untuk administrator: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = { $path } tidak dapat dibatasi hanya untuk administrator, dan drive-nya mungkin tidak menyimpan izin: { $said }
# { $path } is a folder.
core-privilege-written-to-first = proses lain menulis ke { $path } sebelum folder itu dapat dilindungi
core-privilege-no-program-data = Windows tidak menyebutkan lokasi ProgramData

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } tidak berisi paket
# { $path } is a folder.
core-verify-not-unpacked = paket penginstal ini tidak diekstrak ke { $path }
core-verify-schema-supported = Skema manifes paket didukung.
core-verify-packaged-file-missing = File dalam paket tidak ada.
core-verify-no-package-path = File yang dapat disalin tidak memiliki jalur relatif paket.
core-verify-folder-there = Folder kosong ada.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = Item ini tidak dapat dibaca untuk diperiksa: { $error }
core-verify-restored-file-missing = File yang dipulihkan tidak ada.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = Ukuran file tidak cocok. Diharapkan { $expected } byte, ditemukan { $actual } byte.
core-verify-no-hash = Manifes tidak mencatat hash untuk file ini, jadi file ini tidak dapat diperiksa.
core-verify-hash-mismatch = Hash SHA-256 tidak cocok.
core-verify-file-matches = Ukuran dan hash file cocok dengan manifes.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = Verifikasi payload lisensi gagal: { $error }
core-verify-license-payload-missing = Payload lisensi dalam paket tidak ada.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = Verifikasi penyimpanan lisensi gagal: { $error }
core-verify-license-store-file-missing = File penyimpanan lisensi dalam paket tidak ada.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = Jalur penyimpanan lisensi dalam paket tidak valid: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = Verifikasi payload prasyarat gagal: { $error }
core-verify-prerequisite-missing = Payload prasyarat yang dibundel tidak ada.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = Verifikasi citra driver gagal: { $error }
core-verify-bundled-driver-missing = Citra driver yang dibundel tidak ada.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = Jalur driver yang dibundel tidak valid: { $error }
core-verify-prerequisite-present = Prasyarat runtime ada di PC target.
core-verify-prerequisite-not-detected = Prasyarat runtime tidak terdeteksi; mungkin perlu memulai ulang PC atau menyelesaikannya secara manual (lihat log pemulihan untuk hasil yang pasti).
core-verify-prerequisite-not-probed = Keberadaan prasyarat tidak dapat diperiksa melalui registri; instalasi dilakukan sebisanya.
core-verify-license-file-missing = File lisensi yang dipulihkan tidak ada.
core-verify-license-store-restored-missing = File penyimpanan lisensi yang dipulihkan tidak ada.
core-verify-registry-key-there = Kunci registri ada.
core-verify-registry-value-matches = Nilai registri cocok dengan data yang diarsipkan.
core-verify-registry-value-differs = Nilai registri tidak cocok dengan data yang diarsipkan.
core-verify-firewall-rule-there = Windows Firewall memiliki aturan ini.
core-verify-variable-matches = Nilai variabel lingkungan cocok dengan data yang diarsipkan.
core-verify-variable-differs = Nilai variabel lingkungan tidak cocok dengan data yang diarsipkan.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = Variabel lingkungan tidak ada atau tidak dapat dibaca: { $error }
core-verify-service-runs-other = Layanan ada tetapi tidak menjalankan biner yang dipulihkan dengan argumennya.
core-verify-service-other-account = Layanan ada tetapi tidak berjalan sebagai akun yang diharapkan.
core-verify-service-matches = Layanan ada, dan menjalankan biner yang dipulihkan dengan argumen dan akunnya.
core-verify-service-missing = Layanan tidak ada.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = Layanan tidak dapat dibaca: { $error }
core-verify-driver-other-image = Layanan driver tidak mengarah ke citra yang dipulihkan.
core-verify-driver-not-registered = Layanan driver yang dibundel tidak terdaftar setelah pemulihan.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = Layanan driver tidak dapat dibaca: { $error }
core-verify-driver-image-missing = Citra driver tidak ada.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = Citra driver tidak dapat diperiksa: { $error }
core-verify-task-matches = Tugas terjadwal ada dan menjalankan program yang dipulihkan.
core-verify-task-runs-other = Tugas terjadwal ada tetapi tidak menjalankan program yang dipulihkan.
core-verify-task-missing = Tugas terjadwal tidak ada.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = Kueri tugas terjadwal gagal: { $error }
core-verify-shortcut-matches = Pintasan ada dan jalur targetnya cocok dengan target yang dipulihkan.
core-verify-shortcut-opens-other = Pintasan ada tetapi jalur targetnya tidak cocok dengan target yang dipulihkan.
core-verify-shortcut-missing = Pintasan tidak ada.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = Kueri pintasan gagal: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = Skema manifes paket tidak didukung.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = Verifikasi file paket gagal: { $error }
core-warning-verify-folder-missing = Folder kosong tidak ada.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = Kunci registri tidak ada atau tidak dapat dibaca: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = Nilai registri tidak ada atau tidak dapat dibaca: { $error }
core-warning-verify-registry-kind-missing = Jenis nilai registri tidak ada.
core-warning-verify-firewall-rule-missing = Windows Firewall tidak memiliki aturan ini.
core-warning-verify-service-scope = Verifikasi layanan membaca konfigurasi layanan HKLM dan tidak memulai layanan.
core-warning-verify-driver-manual = Satu atau beberapa driver vendor/bawaan Windows tidak didistribusikan ulang oleh WinAppMigrator; instal ulang driver tersebut dengan paket driver vendor yang ditandatangani di PC target.
core-warning-verify-task-scope = Verifikasi tugas terjadwal hanya memeriksa pendaftaran dan referensi perintah.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = Laporan verifikasi WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = Diverifikasi (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = Paket valid: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = Pemulihan valid: { $valid }
core-verify-report-restore-not-run = Pemulihan valid: tidak dijalankan
core-verify-report-items = Item
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } Sumber: { $source }
core-verify-report-warnings = Peringatan
core-verify-report-errors = Kesalahan
core-verify-report-none = Tidak ada.
