# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = Konsol ini tidak dapat menampilkan { $language }, jadi klien berbahasa Inggris.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client tidak dapat dimulai dari folder yang hanya dapat
    diubah oleh administrator: { $error }
client-press-enter-to-close = Tekan Enter untuk menutup jendela ini.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    Program ini memungkinkan WinAppMigrator di PC baru Anda membaca aplikasi di
    PC ini melalui jaringan Anda. Program ini tidak mengubah apa pun di sini:
    program ini membaca snapshot drive Windows, yang dihapus saat Anda
    menghentikannya.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = Klien berhenti: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    Jika jendela ini ditutup alih-alih dihentikan, snapshot dan aturan firewall
    tetap ada sampai klien dijalankan lagi di sini ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Mengambil snapshot drive Windows...{ " " }
client-snapshot-taken = selesai.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    PC ini tidak akan muncul dengan sendirinya ({ $error }).
    Di PC baru, ketikkan nama atau alamat PC ini.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }PC ini:           { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:          { $windows_name }, { $bits }-bit
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }Kode pemasangan:  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    Di WinAppMigrator pada PC baru Anda, pilih PC jaringan... lalu ketikkan kode
    ini. Tutup dulu aplikasi yang Anda pindahkan, agar file-nya tetap seperti
    saat aplikasi itu ditutup.
    Untuk berhenti, tekan Enter di sini atau tutup jendela ini.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    Dipasangkan dengan { $from ->
        [unknown] PC lain
       *[address] { $from }
    }. PC itu sedang membaca yang diperlukannya...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] PC lain
       *[address] { $from }
    } sudah selesai ({ $count ->
       *[other] { $count } permintaan
    } dijawab).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    Sambungan ke { $from ->
        [unknown] PC lain
       *[address] { $from }
    } berakhir: { $error }
client-wrong-code =
    { $from ->
        [unknown] PC lain
       *[address] { $from }
    } mencoba kode yang bukan kode PC ini.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] PC lain
       *[address] { $from }
    } tidak dapat dipasangkan: { $error }
client-too-many-wrong-codes =
    Terlalu banyak kode salah yang dicoba, jadi tidak ada PC yang dapat
    dipasangkan dengan PC ini sekarang. Mungkin ada sesuatu di jaringan Anda yang
    sedang menebak-nebak. Untuk memasangkan, hentikan klien lalu mulai lagi, dan
    ketikkan kode baru yang ditampilkannya.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = Terlalu banyak kode salah. Kode pemasangan yang baru: { $code }

## Stopping

client-stopping = Menghentikan: menghapus snapshot dan aturan firewall...
client-stop-done = Selesai.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = klien sudah berjalan di PC ini, atau sedang membersihkan sisa klien yang ditutup. Gunakan yang itu, atau tunggu sebentar lalu mulai ini lagi.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows tidak membuat snapshot { $drive }: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Windows Firewall tidak mengizinkan klien mendengarkan: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client menerima opsi berikut:
    
      --lang <bahasa>    Gunakan bahasa ini, misalnya it-IT, jika klien
                         memilikinya, alih-alih bahasa tampilan Windows.
      --code <kode>      Mulai dengan kode pemasangan enam digit ini, untuk skrip
                         yang juga memulai WinAppMigrator di PC baru.
      --help             Tampilkan ini.
