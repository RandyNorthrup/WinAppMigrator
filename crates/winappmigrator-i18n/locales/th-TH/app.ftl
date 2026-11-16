# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = เปลี่ยนแอปที่ติดตั้งไว้ ให้เป็นตัวติดตั้งแบบแตกไฟล์ได้เอง
app-theme-to-light = สลับเป็นธีมสว่าง
app-theme-to-dark = สลับเป็นธีมมืด
app-reload-list = โหลดรายการแอปพลิเคชันใหม่
app-list-loading = กำลังโหลด
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
       *[other] { $count } แอป
    }
app-list-load-failed = โหลดไม่สำเร็จ
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = ไม่สามารถเปิดหน้าต่างได้: { $error }

## The list of installed apps

app-list-search = ค้นหาตามชื่อหรือผู้เผยแพร่  (Ctrl+F)
app-list-filter-all = ทั้งหมด
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = ต่อผู้ใช้
app-scope-per-machine = ต่อเครื่อง
app-scope-unknown = ไม่ทราบ
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = ไม่ทราบ
app-list-loading-apps = กำลังโหลดแอปพลิเคชันที่ติดตั้ง…
# { $error } is why the apps could not be listed.
app-list-could-not-list = ไม่สามารถแสดงรายการแอปพลิเคชันได้: { $error }
app-list-no-matches = ไม่มีแอปพลิเคชันที่ตรงกัน
app-list-no-matches-hint = ปรับข้อความค้นหาหรือตัวกรองขอบเขต
app-list-worker-disconnected = ส่วนที่รวบรวมรายการแอปขาดการเชื่อมต่อ
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, เวอร์ชัน { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = ชื่อ
app-column-name-sort = เรียงลำดับตามชื่อ
app-column-publisher = ผู้เผยแพร่
app-column-publisher-sort = เรียงลำดับตามผู้เผยแพร่
app-column-version = เวอร์ชัน
app-column-version-sort = เรียงลำดับตามเวอร์ชัน
app-column-scope = ขอบเขต
app-column-scope-sort = เรียงลำดับตามขอบเขต
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = ซีพียู
app-column-arch-sort = เรียงลำดับตามสถาปัตยกรรมซีพียู

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = แหล่งที่มาของการสแกน
app-source-this-pc = พีซีเครื่องนี้
app-source-drive = ไดรฟ์ที่เชื่อมต่อ…
app-source-image = อิมเมจดิสก์…
app-source-network = พีซีบนเครือข่าย…
app-source-attaching-image = กำลังแนบอิมเมจแบบอ่านอย่างเดียว…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } ผ่านเครือข่าย ({ $address })
app-source-disconnect = ยกเลิกการเชื่อมต่อ
app-source-change = เปลี่ยน…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = เลือกอิมเมจดิสก์ของพีซีเครื่องอื่น (.vhd หรือ .vhdx)
app-source-pick-image-filter = อิมเมจดิสก์
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } ไม่มีโวลุ่มที่มี Windows อยู่
app-source-image-not-attached = ไม่สามารถแนบอิมเมจดิสก์ได้
app-source-pick-drive-title = เลือกรากของไดรฟ์ที่เชื่อมต่อ (เช่น E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } ดูไม่เหมือนไดรฟ์ระบบของ Windows (ไม่มี \Windows และ \Users) โปรดเลือกรากของไดรฟ์จากพีซีเครื่องอื่น
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    แหล่งที่มานี้ใช้ { $windows } ตัวติดตั้งที่สร้างจากแหล่งที่มานี้มีไว้สำหรับ Windows 10 และ 11 เมื่อเลือกข้อมูลผู้ใช้ การตั้งค่าที่ Windows แยกเก็บไว้เบื้องหลังสำหรับแอปรุ่นเก่า จะถูกนำมาด้วย และรันไทม์ที่ Windows 11 ไม่มี เช่น DirectX 9 จะถูกติดตั้งพร้อมกับแอป{ $is_32_bit ->
        [yes] { " " }แอปในแหล่งที่มานี้เป็นแบบ 32 บิต จึงจะไปอยู่ใน Program Files (x86) ซึ่งเป็นที่ที่ Windows 64 บิตเก็บแอปแบบนี้
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    แหล่งที่มานี้ใช้ Windows รุ่นเก่า ตัวติดตั้งที่สร้างจากแหล่งที่มานี้มีไว้สำหรับ Windows 10 และ 11 เมื่อเลือกข้อมูลผู้ใช้ การตั้งค่าที่ Windows แยกเก็บไว้เบื้องหลังสำหรับแอปรุ่นเก่า จะถูกนำมาด้วย และรันไทม์ที่ Windows 11 ไม่มี เช่น DirectX 9 จะถูกติดตั้งพร้อมกับแอป{ $is_32_bit ->
        [yes] { " " }แอปในแหล่งที่มานี้เป็นแบบ 32 บิต จึงจะไปอยู่ใน Program Files (x86) ซึ่งเป็นที่ที่ Windows 64 บิตเก็บแอปแบบนี้
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = จับคู่กับพีซีเครื่องเก่า
app-details-pair-first-hint = เมื่อ WinAppMigrator Client ทำงานบนพีซีเครื่องนั้น และคุณพิมพ์รหัสที่ไคลเอ็นต์แสดงแล้ว แอปของพีซีเครื่องนั้นจะแสดงที่นี่ และคุณสามารถสร้างตัวติดตั้งจากแอปใดก็ได้ เหมือนกับจากพีซีเครื่องนี้
app-details-pick-an-app = เลือกแอปพลิเคชัน
app-details-pick-an-app-hint = เลือกแอปจากรายการเพื่อเลือกสิ่งที่จะรวมไว้ และสร้างตัวติดตั้งของแอปนั้น
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, เวอร์ชัน { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = ไม่ทราบผู้เผยแพร่, เวอร์ชัน { $version }
app-details-unknown-publisher = ไม่ทราบผู้เผยแพร่

## What goes in besides the program

app-option-license = ข้อมูลสิทธิ์การใช้งานและการเปิดใช้งาน
app-option-license-hint = อาจผูกอยู่กับพีซีเครื่องนี้ และต้องเปิดใช้งานใหม่บนพีซีเครื่องใหม่
app-option-user-data = ข้อมูลผู้ใช้
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = การตั้งค่าและประวัติ เอกสาร เกมที่บันทึกไว้ และ ProgramData
app-option-whose-data = ข้อมูลของใคร
app-option-whose-data-hint = ตัวติดตั้งหนึ่งตัวเก็บข้อมูลของบัญชีเดียว ข้อมูลจะไปยังผู้ที่ลงชื่อเข้าใช้อยู่ ในเครื่องที่เรียกใช้ตัวติดตั้ง

## Building the installer

app-output-folder = โฟลเดอร์ผลลัพธ์
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\ชื่อผู้ใช้\WinAppMigrator
app-output-folder-not-full = โปรดใส่เส้นทางโฟลเดอร์แบบเต็ม เช่น D:\Installers
app-build-create = สร้างตัวติดตั้ง
app-build-create-again = สร้างตัวติดตั้งอีกครั้ง
app-build-cancel = ยกเลิก
app-build-cancelling = กำลังยกเลิกหลังจากขั้นตอนปัจจุบัน…
app-build-cancelled = ยกเลิกการสร้างแล้ว ไม่มีการเขียนตัวติดตั้ง
# winappmigrator-installer.exe is a file name.
app-build-no-stub = ไม่พบสตับตัวติดตั้ง (winappmigrator-installer.exe) ข้างแอปนี้ จึงไม่สามารถสร้างตัวติดตั้งแบบแตกไฟล์ได้เอง
app-build-select-an-app-first = โปรดเลือกแอปพลิเคชันที่จะย้ายก่อน
app-build-look-again-first = ต้องตรวจดูแอปนี้อีกครั้งก่อนจึงจะสร้างได้
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = การสร้างหยุดลงโดยไม่คาดคิด: { $reason } หากมีตัวติดตั้งเหลืออยู่ในโฟลเดอร์ผลลัพธ์ อย่าใช้ตัวติดตั้งนั้น
app-build-worker-disconnected = ส่วนที่ทำการย้ายขาดการเชื่อมต่อ
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = ไม่สามารถสร้างโฟลเดอร์ผลลัพธ์ { $path } ได้: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = ไม่สามารถสร้างโฟลเดอร์พักไฟล์ที่ได้รับการป้องกัน จึงไม่มีการสร้างสิ่งใด: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = ไม่สามารถสร้างตัวติดตั้งได้ { $error }
app-build-no-installer = การสร้างเสร็จสิ้นแต่ไม่ได้ตัวติดตั้ง
# { $error } is what went wrong.
app-build-hash-check-did-not-run = เขียนตัวติดตั้งแล้ว แต่ไม่สามารถตรวจสอบแฮชได้: { $error } อย่าใช้ตัวติดตั้งนี้

## How far a build has got

app-progress-preparing = กำลังตรวจสอบว่าจะรวมอะไรไว้บ้าง…
app-progress-checking-room = กำลังตรวจสอบพื้นที่ว่าง…
app-progress-bundling = กำลังรวมรันไทม์และไดรเวอร์…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = กำลังอ่านและบีบอัดไฟล์: { $done } จาก { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = กำลังตรวจทานสิ่งที่ตัวติดตั้งระบุ: { $done } จาก { $total }…
app-progress-writing = กำลังเขียนตัวติดตั้ง…
app-progress-verifying = กำลังตรวจสอบแฮช…

## What a build made

app-result-failed-hash-check = อย่าใช้ตัวติดตั้งนี้: ตัวติดตั้งไม่ผ่านการตรวจสอบแฮช
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
       *[other] สร้างตัวติดตั้งแล้ว แต่มีไฟล์ { $count } ไฟล์ที่หายไปหลังจากตรวจดูแอป จึงไม่ได้รวมไว้ ดู “คำเตือน” ด้านล่าง
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
       *[other] สร้างตัวติดตั้งแล้ว แต่ไม่สามารถเก็บไฟล์ { $count } ไฟล์ได้ ซึ่งมักเป็นเพราะไฟล์ถูกล็อกหรืออ่านไม่ได้ แอปอาจทำงานไม่ได้หากไม่มีไฟล์เหล่านี้ ดู “คำเตือน” ด้านล่าง
    }
app-result-mentions-who = สร้างตัวติดตั้งแล้ว แต่ตัวติดตั้งยังมีข้อมูลบางอย่าง ที่บอกว่ามาจากใคร โปรดอ่านสิ่งที่ระบุไว้ด้านล่างก่อนส่งให้ผู้อื่น
app-result-ready = ตัวติดตั้งพร้อมแล้ว แพคเกจผ่านการตรวจสอบแฮช
app-result-files = ไฟล์
app-result-installer-size = ขนาดตัวติดตั้ง
app-result-warnings = คำเตือน
app-result-log = ไฟล์บันทึก
app-result-copy-error = คัดลอกข้อผิดพลาด
app-result-installer = ตัวติดตั้ง
app-result-data-file = ไฟล์ข้อมูล
app-result-open-folder = เปิดโฟลเดอร์
app-result-copy-path = คัดลอกเส้นทาง
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = ไม่สามารถเปิดโฟลเดอร์ได้: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = แอปนี้ใหญ่เกินกว่าจะอยู่ในไฟล์ปฏิบัติการไฟล์เดียว ตัวติดตั้งจึงมีสองไฟล์ คือ { $installer } และ { $data_file } ทั้งสองไฟล์ต้องอยู่ด้วยกัน: คัดลอกทั้งสองไฟล์ไปยังพีซีเครื่องอื่น ไว้ในโฟลเดอร์เดียวกัน โดยใช้ชื่อเหล่านี้ หากคุณลงนามตัวติดตั้ง ลายเซ็นนั้นจะครอบคลุมไฟล์ข้อมูลด้วย
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = เรียกใช้ตัวติดตั้งบนพีซีเครื่องอื่นเพื่อติดตั้ง { $app_name } อีกครั้ง ตัวติดตั้งจะตรวจสอบไฟล์ด้วย SHA-256 ส่วน Windows จะแสดง “ผู้เผยแพร่ที่ไม่รู้จัก” เนื่องจากตัวติดตั้งที่คุณสร้างไม่ได้ลงนาม
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = ข้อผิดพลาดในการตรวจสอบ ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = คำเตือน ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = และอีก { $count } รายการ

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = กำลังเตรียมตรวจดูแอปนี้…
app-scan-install-record = กำลังอ่านว่าแอปติดตั้งไว้อย่างไร…
app-scan-near-matches = กำลังค้นหาสิ่งที่อาจเป็นส่วนหนึ่งของแอปด้วย…
app-scan-system-items = กำลังค้นหาบริการ งาน และทางลัดของแอป…
app-scan-files = กำลังตรวจไฟล์ของแอป…
app-scan-registry = กำลังอ่านสิ่งที่เชื่อมแอปเข้ากับ Windows…
app-scan-finishing = กำลังดำเนินการให้เสร็จ…
app-scan-look-again = ตรวจดูอีกครั้ง
app-scan-stop = หยุด
app-scan-running-hint = คุณสามารถเลือกแอปอื่น หรือเปลี่ยนสิ่งที่รวมไว้ได้ในระหว่างนี้
# { $error } is why, in words that follow a colon.
app-scan-failed = ไม่สามารถตรวจดูแอปนี้ได้: { $error }
app-scan-try-again = ลองอีกครั้ง
app-scan-stopped = หยุดแล้ว คุณจะสร้างตัวติดตั้งได้เมื่อตรวจดูแอปนี้เสร็จแล้ว
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = หยุดลงโดยไม่คาดคิด: { $reason }

## How big an installer can be, and whether there is room for it

app-room-files = ไฟล์
# Over how big the installer can be: up to that size.
app-room-up-to = สูงสุด
app-room-free = ว่าง
# { $drive } is a drive, such as C:.
app-room-free-on = ว่างบน { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = ไม่ทราบ
app-room-most-it-could-need = พื้นที่สูงสุดที่อาจต้องใช้ ก่อนการบีบอัด
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } ต้องมีพื้นที่ว่างสูงสุด { $needed } เพื่อสร้างตัวติดตั้งนี้ แต่มีเพียง { $free }
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = พื้นที่ไม่เพียงพอ { $drives } โปรดเลือกโฟลเดอร์ผลลัพธ์บนไดรฟ์อื่น หรือนำบางรายการออก
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }

## What goes in the installer, part by part

app-footprint-heading = สิ่งที่รวมอยู่ในตัวติดตั้ง
app-footprint-working-out = กำลังตรวจสอบว่ามีอะไรที่ต้องไปด้วยกัน…
# { $error } is why.
app-footprint-would-not-make-a-package = สิ่งที่เหลืออยู่ไม่สามารถสร้างเป็นแพคเกจได้: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = การทำเช่นนี้จะทำให้ไม่รวม { $list } ไว้ด้วย เนื่องจากสิ่งที่รายการเหล่านั้นเรียกใช้หรืออ้างถึง จะไม่อยู่ในตัวติดตั้ง รายการเหล่านั้นจะไม่ถูกตั้งค่าบนพีซีเครื่องอื่น
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = การทำเช่นนี้จะทำให้ไม่รวม { $list } และอีก { $count } รายการไว้ด้วย เนื่องจากสิ่งที่รายการเหล่านั้นเรียกใช้หรืออ้างถึง จะไม่อยู่ในตัวติดตั้ง รายการเหล่านั้นจะไม่ถูกตั้งค่าบนพีซีเครื่องอื่น
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = ข้อมูลสิทธิ์การใช้งานในสิ่งที่คุณไม่รวมไว้ จะยังคงรวมอยู่ เนื่องจากข้อมูลนั้นขึ้นอยู่กับตัวเลือก “ข้อมูลสิทธิ์การใช้งานและการเปิดใช้งาน”: { $list }
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = ตรวจดูแอปนี้เมื่อ { $elapsed } ที่แล้ว ไฟล์จะถูกอ่านตอนที่คุณสร้างตัวติดตั้ง จึงได้รับการเปลี่ยนแปลงล่าสุด และสิ่งที่หายไปในตอนนั้นจะแสดงอยู่ในรายการ คลิก “ตรวจดูอีกครั้ง” เพื่อดูสิ่งที่เพิ่มขึ้นใหม่
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = ไฟล์โปรแกรม
app-footprint-data-folders = โฟลเดอร์ข้อมูล
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } เมื่อเลือกตัวเลือกข้อมูลผู้ใช้
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
       *[other] { $included } จาก { $count } โฟลเดอร์, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
       *[other] ข้อมูลของแอปนี้ขนาด { $size } ใน { $count } ไฟล์จะรวมไว้ เฉพาะเมื่อเลือกตัวเลือกข้อมูลผู้ใช้เท่านั้น
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
       *[other] { $count } ไฟล์ { $detail }
    }
app-footprint-services = บริการ
app-footprint-startup = เริ่มทำงานโดยอัตโนมัติ
app-footprint-tasks = งานที่จัดกำหนดการไว้
app-footprint-shortcuts = ทางลัด
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } จาก { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = ไม่รวมไว้ด้วย: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }: { $detail }
app-footprint-integrations = สิ่งที่เชื่อมแอปเข้ากับ Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
       *[other] { $included } จาก { $count } ประเภท
    }
app-footprint-integration-nothing-left = ไม่เหลือสิ่งใดเลย เนื่องจากสิ่งที่คุณไม่รวมไว้
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
       *[other] ตั้งค่า { $count } รายการ
    }
app-footprint-prerequisites = สิ่งที่แอปต้องใช้ในการทำงาน
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
       *[other] รันไทม์ { $count } รายการ
    }
app-footprint-near-matches = อาจเป็นส่วนหนึ่งของแอปด้วย
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = เพิ่มแล้ว { $ticked } จาก { $count }
app-footprint-near-matches-hint = รายการเหล่านี้จะไม่อยู่ในตัวติดตั้ง หากคุณไม่ได้ทำเครื่องหมาย การทำเครื่องหมายรายการใด จะตรวจดูแอปอีกครั้ง ด้วยการตรวจสอบแบบเดียวกับส่วนที่เหลือ สิ่งที่ดูเหมือนรหัสผ่านจึงยังคงไม่ถูกรวมไว้
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = มาพร้อมกับตัวเลือกข้อมูลผู้ใช้ { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = มากกว่า { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = โฟลเดอร์: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = ส่วนที่ซ่อนอยู่ของแอป: { $place }
app-footprint-nothing-left-out = ไม่มีรายการใดที่ไม่รวมไว้เพราะดูเหมือนรหัสผ่าน สิทธิ์การใช้งาน หรือใบรับรอง
app-footprint-left-out = สิ่งที่ไม่รวมไว้และเหตุผล
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
       *[other] { $count } รายการ
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = ไม่มีตัวติดตั้งใดได้รับอนุญาต ให้ตั้งค่ารายการเหล่านี้:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = ข้อควรทราบ ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = ไม่พบ: { $list }
app-none-found-services = บริการ
app-none-found-startup = รายการเริ่มต้นระบบ
app-none-found-tasks = งานที่จัดกำหนดการไว้
app-none-found-shortcuts = ทางลัด
app-none-found-integrations = สิ่งที่เชื่อมแอปเข้ากับ Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. วางไคลเอ็นต์ไว้บนพีซีเครื่องเก่า
app-network-step-client-hint = WinAppMigrator Client ทำงานบนพีซีที่แอปอยู่ในขณะนี้ ซึ่งใช้ Windows 7 หรือใหม่กว่า ไคลเอ็นต์อนุญาตให้เฉพาะพีซีเครื่องนี้ อ่านข้อมูลแบบเข้ารหัส โดยใช้รหัสที่ไคลเอ็นต์แสดง
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = บันทึกลงในไดรฟ์ USB { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = บันทึกลงในไดรฟ์ USB { $drive } ({ $name })
app-network-save-to-folder = บันทึกลงในโฟลเดอร์…
app-network-look-for-usb-again = ค้นหาไดรฟ์ USB อีกครั้ง
app-network-no-usb = ไม่ได้เสียบไดรฟ์ USB อยู่ โปรดเสียบไดรฟ์ หรือบันทึกลงในโฟลเดอร์ ที่คุณเข้าถึงได้จากพีซีเครื่องเก่า เช่น โฟลเดอร์ที่แชร์บนเครือข่ายของคุณ
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = บันทึกเป็น { $path } แล้ว บนพีซีเครื่องเก่า ให้ปิดแอปที่คุณกำลังย้าย จากนั้นดับเบิลคลิกไฟล์นี้ และอนุญาตให้ทำการเปลี่ยนแปลง ไคลเอ็นต์จะแสดงรหัส
app-network-step-pick = 2. เลือกพีซีเครื่องเก่า
app-network-looking = กำลังค้นหาพีซีที่เรียกใช้ไคลเอ็นต์ บนเครือข่ายของคุณ…
app-network-type-name = หรือพิมพ์ชื่อหรือที่อยู่ของพีซี
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC หรือ 192.168.1.20
app-network-step-code = 3. พิมพ์รหัสที่แสดง
app-network-code = รหัสการจับคู่
app-network-connect = เชื่อมต่อ
app-network-pairing = กำลังจับคู่…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = การจับคู่หยุดลงก่อนเสร็จสมบูรณ์
app-network-wrong-code = รหัสนี้ไม่ใช่รหัสที่พีซีเครื่องเก่าแสดง โปรดตรวจสอบแล้วลองอีกครั้ง
# { $error } is what went wrong.
app-network-not-reached = ไม่สามารถติดต่อพีซีเครื่องเก่าได้: { $error }
app-network-pick-folder-title = เลือกโฟลเดอร์สำหรับ WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = ไม่สามารถบันทึกไคลเอ็นต์ลงใน { $path } ได้: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = WinAppMigrator รุ่นนี้ไม่มีไคลเอ็นต์มาด้วย โปรดสร้างไคลเอ็นต์ด้วย scripts\build-client.ps1 แล้วสร้างแอปอีกครั้ง
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = ไม่สามารถบันทึกไฟล์หมายเหตุข้างไคลเอ็นต์ได้: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    เรียกใช้โปรแกรมนี้ บนพีซีที่คุณกำลังย้ายแอปออก: Windows 7, 8, 10 หรือ 11
    
    1. ปิดแอปที่คุณกำลังย้าย
    2. ดับเบิลคลิก “WinAppMigrator Client.exe” และอนุญาตให้ทำการเปลี่ยนแปลง
    3. โปรแกรมจะแสดงรหัสการจับคู่ 6 หลัก ใน WinAppMigrator บนพีซีเครื่องใหม่
       ให้เลือก “พีซีบนเครือข่าย...” เลือกพีซีเครื่องนี้ แล้วพิมพ์รหัส
    4. เมื่อพีซีเครื่องใหม่ทำงานเสร็จ ให้กด Enter ในหน้าต่างของไคลเอ็นต์
       หรือปิดหน้าต่างนั้น
    
    โปรแกรมนี้อ่านข้อมูลเท่านั้น ระหว่างที่ทำงาน โปรแกรมจะเก็บสแนปช็อตของ
    ไดรฟ์ Windows และอนุญาตพอร์ตสองพอร์ตของตนเอง ผ่านไฟร์วอลล์ Windows
    จากเครือข่ายภายในของคุณ และจะลบทั้งสองอย่างเมื่อหยุดทำงาน ข้อมูลที่ส่ง
    ผ่านเครือข่ายจะถูกเข้ารหัส และมีเพียงพีซีที่มีรหัสเท่านั้นที่อ่านข้อมูลได้
