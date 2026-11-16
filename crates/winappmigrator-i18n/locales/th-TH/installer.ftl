# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = ตัวติดตั้ง WinAppMigrator
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = ติดตั้ง { $app_name }
installer-tagline = ติดตั้งแอปพลิเคชันที่เก็บมาจากพีซีเครื่องอื่นอีกครั้ง
# What an app is called when its package gives it no name.
installer-unnamed-app = แอปพลิเคชัน

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = ไม่สามารถเปิดหน้าต่างของตัวติดตั้งได้: { $error }
installer-window-failed-without-one = แต่ยังคงติดตั้งได้โดยไม่ต้องใช้หน้าต่าง: ให้เรียกใช้ไฟล์นี้ จากพร้อมท์คำสั่งที่เรียกใช้ในฐานะผู้ดูแลระบบ พร้อมกับ --apply และใช้ --target-root ตามด้วยโฟลเดอร์เพื่อเลือกตำแหน่ง

## Checking the installer when it opens

installer-checking = กำลังตรวจสอบตัวติดตั้ง…
installer-stub-no-package = นี่คือสตับตัวติดตั้งของ WinAppMigrator ซึ่งไม่มีแพคเกจแอปพลิเคชันอยู่ภายใน โปรดสร้างตัวติดตั้งด้วย WinAppMigrator แล้วเรียกใช้ไฟล์ .exe ที่ได้ที่นี่
installer-check-stopped = การตรวจสอบตัวติดตั้งหยุดลง
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = ไม่สามารถระบุตำแหน่งของตัวติดตั้งได้: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = ไม่สามารถอ่านตัวติดตั้งได้: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = ไม่สามารถใช้แพคเกจของตัวติดตั้งนี้ บนพีซีเครื่องนี้ได้: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = เก็บมาจาก
installer-install-location = ตำแหน่งที่ติดตั้ง
# The title of the folder picker.
installer-choose-location = เลือกตำแหน่งที่ติดตั้ง
installer-folder-has-files = โฟลเดอร์นี้มีไฟล์อยู่แล้ว ไฟล์ที่มีชื่อเดียวกันจะถูกแทนที่ และจะกลับคืนมาหากคุณถอนการติดตั้ง
installer-room-needed = พื้นที่ที่ต้องใช้
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = พื้นที่ที่นี่อาจไม่เพียงพอ: { $shortfall } ตัวติดตั้งสามารถแตกไฟล์บนไดรฟ์อื่นได้ แต่ไฟล์ของแอปยังคงต้องมีพื้นที่เพียงพอ
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = การติดตั้ง { $app_name } ครั้งก่อนไม่เสร็จสมบูรณ์ เมื่อติดตั้ง ระบบจะเลิกทำการติดตั้งครั้งนั้นก่อน
# Beside "Installer", for whether it proves who made it.
installer-signature = ตัวติดตั้ง
installer-signature-checking = กำลังตรวจสอบลายเซ็น…
installer-signature-unsigned = ไม่ได้ลงนาม: ไม่ทราบผู้เผยแพร่
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = ลงนามโดย { $publisher }
installer-signature-unnamed = ลงนามโดยผู้เผยแพร่ที่ไม่มีชื่อ
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = ลายเซ็นของตัวติดตั้งนี้ไม่ถูกต้อง ({ $reason }) ตัวติดตั้งอาจถูกเปลี่ยนแปลงหลังจากลงนาม

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } ติดตั้งอยู่ในโฟลเดอร์นี้แล้ว จึงไม่สามารถติดตั้งสำเนานี้ที่นี่ได้ หากต้องการติดตั้งที่นี่ ให้ถอนการติดตั้งสำเนาบนพีซีเครื่องนี้ก่อน ในการตั้งค่า Windows ภายใต้ “แอป” แล้วเปิดตัวติดตั้งนี้อีกครั้ง หรือใส่โฟลเดอร์ใหม่ที่ว่างเปล่าด้านบน
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } ติดตั้งอยู่ในโฟลเดอร์นี้ จึงไม่สามารถติดตั้ง { $app_name } ที่นี่ได้ โปรดใส่โฟลเดอร์ใหม่ที่ว่างเปล่าด้านบน หรือหากคุณไม่ต้องการ { $other_app } แล้ว ให้ถอนการติดตั้งก่อนในการตั้งค่า Windows ภายใต้ “แอป” แล้วเปิดตัวติดตั้งนี้อีกครั้ง
installer-location-has-files = โฟลเดอร์นี้มีไฟล์ที่ไม่ได้ติดตั้งโดย WinAppMigrator อยู่แล้ว ไฟล์เหล่านั้นอาจเป็นของแอปอื่น จึงไม่มีการติดตั้งทับไฟล์เหล่านั้น โปรดใส่โฟลเดอร์ใหม่ที่ว่างเปล่าด้านบน
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = ไม่สามารถใช้โฟลเดอร์นี้ได้: { $reason } โปรดใส่โฟลเดอร์อื่นด้านบน

## What the install sets up beyond its files

installer-beyond-files = สิ่งที่ตัวติดตั้งนี้ตั้งค่านอกเหนือจากไฟล์
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = ทั้งหมดนี้ทำด้วยสิทธิ์ของผู้ดูแลระบบ หากการติดตั้งล้มเหลว สิ่งที่เปลี่ยนแปลงไปจะถูกย้อนกลับ และคุณจะได้รับแจ้งหากมีสิ่งใดที่ย้อนกลับไม่ได้ คุณยังสามารถเลิกทำการติดตั้งในภายหลังได้ จากการตั้งค่า ภายใต้ “แอปที่ติดตั้ง” ไฟล์ที่ถูกแทนที่ไว้จะกลับคืนมา ส่วนรันไทม์ที่ติดตั้งไว้จะยังคงอยู่
installer-beyond-files-none = ไม่มี: ตัวติดตั้งนี้คัดลอกเฉพาะไฟล์ และการตั้งค่าในคีย์รีจิสทรีของแอปเอง
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = ไม่สามารถติดตั้งแพคเกจนี้ไปยังตำแหน่งนี้ได้: { $problem }
installer-working-out = กำลังตรวจสอบว่าจะตั้งค่าอะไรบ้างที่ตำแหน่งนี้…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = การตรวจสอบว่าจะทำอะไรบ้างหยุดลง
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
       *[other] ไม่รวมไว้อีก { $count } รายการ
    }

## Options

installer-options = ตัวเลือก
installer-user-data = ข้อมูลผู้ใช้
installer-user-data-detail = เปิด: การตั้งค่า เอกสาร และเกมที่บันทึกไว้ จะไปยังโฟลเดอร์ปกติของผู้ที่ลงชื่อเข้าใช้อยู่ ปิด: ทั้งหมดจะอยู่ภายใต้โฟลเดอร์ที่ติดตั้ง และจะไม่มีการตั้งค่าให้สิ่งใดทำงาน จากโฟลเดอร์นั้น
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = ข้อมูลนี้เป็นของ { $account }
installer-license = สิทธิ์การใช้งาน
installer-license-detail = ไฟล์การเปิดใช้งานและสิทธิ์การใช้งาน รวมถึงค่ารีจิสทรีที่เก็บสิทธิ์การใช้งาน บางรายการอาจต้องเปิดใช้งานใหม่ที่นี่

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = ส่วนประกอบที่ต้องใช้
installer-dependencies-detail = สิ่งที่แอปนี้ต้องใช้ และวิธีจัดหาแต่ละรายการ
installer-dependencies-none = ไม่พบรันไทม์อื่นที่แอปต้องใช้
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = ฟีเจอร์ของ Windows: { $feature }
installer-dependency-included = รวมไว้แล้ว
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = ติดตั้งด้วย winget: { $id }
installer-dependency-manual = ติดตั้งด้วยตนเอง
installer-dependency-built-in = มีอยู่ใน Windows
installer-dependency-ships-with-app = มาพร้อมกับแอป

## Buttons, and what is said beside them

installer-install = ติดตั้ง
installer-preview = แสดงตัวอย่างการเปลี่ยนแปลง
installer-cancel = ยกเลิก
installer-close = ปิด
installer-open-folder = เปิดโฟลเดอร์ที่ติดตั้ง
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = ไม่สามารถเปิดโฟลเดอร์ได้: { $error }
installer-location-blocks = “ติดตั้ง” และ “แสดงตัวอย่างการเปลี่ยนแปลง” จะใช้ไม่ได้ จนกว่าจะสามารถใช้ตำแหน่งที่ติดตั้งได้ เหตุผลแสดงอยู่ด้านบน ใต้ช่องตำแหน่งที่ติดตั้ง
installer-package-blocks = “ติดตั้ง” ใช้ไม่ได้ เนื่องจากไม่สามารถติดตั้งแพคเกจนี้ ไปยังตำแหน่งนี้ได้ เหตุผลแสดงอยู่ด้านบน
installer-list-changed = สิ่งที่ตัวติดตั้งจะทำที่นี่เปลี่ยนไปแล้ว จึงไม่มีการติดตั้งสิ่งใด โปรดตรวจดูรายการด้านล่าง แล้วคลิก “ติดตั้ง” อีกครั้งหากเป็นสิ่งที่คุณต้องการ
installer-cancelled-early = ยกเลิกแล้ว ไม่มีการเปลี่ยนแปลงสิ่งใด

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } จาก { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = กำลังเตรียมพร้อม…
installer-step-cancelling = กำลังยกเลิกหลังจากขั้นตอนปัจจุบัน…
installer-step-putting-back = กำลังย้อนกลับสิ่งที่เปลี่ยนแปลง
installer-step-unpacking = กำลังแตกไฟล์
installer-step-checking-package = กำลังตรวจสอบแพคเกจ
installer-step-recovering = กำลังเลิกทำการติดตั้งครั้งก่อนที่ไม่เสร็จสมบูรณ์
# What a preview is doing, whatever step it is on.
installer-step-previewing = กำลังตรวจสอบสิ่งที่จะเปลี่ยนแปลง
installer-step-runtimes = กำลังติดตั้งรันไทม์
installer-step-files = กำลังคัดลอกไฟล์
installer-step-services = กำลังตั้งค่าบริการ
installer-step-drivers = กำลังตั้งค่าไดรเวอร์
installer-step-registry = กำลังเขียนการตั้งค่ารีจิสทรี
installer-step-license = กำลังคืนค่าข้อมูลสิทธิ์การใช้งาน
installer-step-tasks = กำลังตั้งค่างานที่จัดกำหนดการไว้
installer-step-shortcuts = กำลังสร้างทางลัด
installer-step-environment = กำลังตั้งค่าตัวแปรสภาพแวดล้อม
installer-step-settings-entry = กำลังเพิ่มแอปลงในการตั้งค่า
installer-step-verifying = กำลังตรวจสอบสิ่งที่ติดตั้ง
installer-step-undoing = กำลังเลิกทำ

## When an install cannot go ahead or stops

installer-carries-no-package = ตัวติดตั้งนี้ไม่มีแพคเกจ
installer-stopped-unexpectedly = การติดตั้งหยุดลงโดยไม่คาดคิด
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = การติดตั้งหยุดลงโดยไม่คาดคิด: { $reason } หากการติดตั้งได้เริ่มไปแล้ว เมื่อเรียกใช้ตัวติดตั้งอีกครั้ง ระบบจะเลิกทำสิ่งที่การติดตั้งนั้นทำไว้ก่อน
# { $error } is what went wrong reading it.
installer-package-read-failed = ไม่สามารถอ่านแพคเกจได้: { $error }
installer-no-programdata = ไม่พบโฟลเดอร์ ProgramData ของพีซีเครื่องนี้ จึงไม่ได้ติดตั้งสิ่งใด เนื่องจากการติดตั้งจะเก็บบันทึก ที่ใช้เลิกทำการติดตั้งไว้ที่นั่น
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = ไม่สามารถสร้างโฟลเดอร์พักไฟล์ได้: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = ไม่สามารถแตกไฟล์แพคเกจได้: { $error }
installer-staging-nowhere = ไม่มีตำแหน่งสำหรับแตกไฟล์แพคเกจ ที่เฉพาะผู้ดูแลระบบเท่านั้นเข้าถึงได้
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = ไม่มีตำแหน่งสำหรับแตกไฟล์แพคเกจ ที่เฉพาะผู้ดูแลระบบเท่านั้นเข้าถึงได้ ตำแหน่งที่ควรตรวจสอบคือ { $failure }

## What came of an install, or of a preview

installer-result = ผลการติดตั้ง
installer-result-preview = ตัวอย่าง: ยังไม่มีการเปลี่ยนแปลงสิ่งใด
installer-result-installed = ติดตั้งแอปพลิเคชันแล้ว และผ่านการตรวจสอบแฮช
installer-result-installed-without-left-out = ติดตั้งแอปพลิเคชันแล้ว และผ่านการตรวจสอบแฮช โดยไม่มีรายการที่แสดงว่าไม่รวมไว้ด้านล่าง
installer-result-not-all-put-back = การติดตั้งล้มเหลว และไม่สามารถย้อนกลับสิ่งที่เปลี่ยนแปลงได้ทั้งหมด สิ่งที่เหลืออยู่แสดงอยู่ด้านล่าง และการติดตั้งครั้งถัดไปจะลองอีกครั้ง
installer-result-cancelled-put-back = การติดตั้งถูกยกเลิก และสิ่งที่เปลี่ยนแปลงไปได้รับการย้อนกลับแล้ว
installer-result-failed-put-back = การติดตั้งล้มเหลว และสิ่งที่เปลี่ยนแปลงไปได้รับการย้อนกลับแล้ว
installer-result-not-clean = การติดตั้งเสร็จสิ้นอย่างไม่สมบูรณ์ โปรดตรวจดูปัญหาด้านล่าง
installer-result-changes-put-back = การเปลี่ยนแปลงที่ย้อนกลับแล้ว
installer-result-changes-made = การเปลี่ยนแปลงที่ทำแล้ว
installer-result-changes-planned = การเปลี่ยนแปลงที่วางแผนไว้
installer-result-warnings = คำเตือน
installer-result-errors = ข้อผิดพลาด
installer-result-package-valid = แฮชของแพคเกจถูกต้อง
installer-result-package-invalid = แฮชของแพคเกจไม่ถูกต้อง
installer-result-files-valid = ไฟล์ที่ติดตั้งถูกต้อง
installer-result-files-invalid = ไฟล์ที่ติดตั้งไม่ถูกต้อง
installer-result-reboot = ส่วนประกอบที่ต้องใช้รายการหนึ่ง ต้องเริ่มระบบใหม่ จึงจะเสร็จสมบูรณ์ โปรดเริ่ม Windows ใหม่ แล้วเปิดแอป
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = การเปลี่ยนแปลง ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = และอีก { $count } รายการ
# Over the path of the install's log file.
installer-log = ไฟล์บันทึก

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = สตับตัวติดตั้ง WinAppMigrator — ไม่มีแพคเกจแอปพลิเคชันอยู่ภายใน
installer-cli-stub-how = สร้างตัวติดตั้งด้วยแอป WinAppMigrator แล้วเรียกใช้ไฟล์ .exe ที่ได้บนเครื่องเป้าหมาย
installer-cli-usage-title = ตัวติดตั้งแบบแตกไฟล์ได้เองของ WinAppMigrator
installer-cli-usage = เรียกใช้โดยไม่มีอาร์กิวเมนต์เพื่อเปิดตัวติดตั้งแบบกราฟิก หรือ:
installer-cli-usage-preview = แสดงตัวอย่างการติดตั้งใหม่: --dry-run [--target-root <DIR>]
installer-cli-usage-apply = ติดตั้งใหม่:              --apply [--target-root <DIR>]
installer-cli-no-package = ตัวติดตั้งไม่มีแพคเกจที่ฝังอยู่
installer-cli-no-programdata = ไม่พบโฟลเดอร์ ProgramData ของพีซีเครื่องนี้ จึงไม่ได้ติดตั้งสิ่งใด เนื่องจากการติดตั้งจะเก็บบันทึก ที่ใช้เลิกทำการติดตั้งไว้ที่นั่น
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = ไม่สามารถใช้ { $path } ได้: { $problem } ใช้ --force เฉพาะเมื่อคุณแน่ใจว่าเป็นของแอปนี้
installer-cli-no-location = แพคเกจนี้ไม่ได้ระบุว่าแอปเคยอยู่ที่ใด โปรดระบุ --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = แอปของแพคเกจนี้เคยอยู่ใน { $path } และไม่สามารถใช้ตำแหน่งนั้นได้: { $problem } โปรดระบุ --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = พื้นที่: { $summary }
# { $note } is something to know about the install.
installer-cli-note = หมายเหตุ: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = สิ่งที่ตั้งค่า: { $count }
installer-cli-applied = ติดตั้งใหม่แล้ว
installer-cli-previewed = แสดงตัวอย่างการติดตั้งใหม่แล้ว
# { $count } is how many changes the install logged.
installer-cli-changes-logged = การเปลี่ยนแปลงที่บันทึกไว้: { $count }
# { $count } is how many warnings.
installer-cli-warnings = คำเตือน: { $count }
# { $count } is how many errors.
installer-cli-errors = ข้อผิดพลาด: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = ถูกปฏิเสธ: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = แฮชของแพคเกจถูกต้อง: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = แฮชของการติดตั้งใหม่ถูกต้อง: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = รหัสการติดตั้ง: { $id }
installer-cli-put-back = การติดตั้งล้มเหลว สิ่งที่เปลี่ยนแปลงไปจึงได้รับการย้อนกลับแล้ว
# { $path } is the log file.
installer-cli-log = ไฟล์บันทึก: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = ไฟล์บันทึก: { $path }
# { $error } is what went wrong.
installer-cli-error = ข้อผิดพลาด: { $error }

## The uninstaller's window

uninstaller-window-title = ถอนการติดตั้ง
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = ถอนการติดตั้ง { $app_name }
uninstaller-tagline = เลิกทำสิ่งที่ตัวติดตั้งได้ทำไว้
uninstaller-reading = กำลังอ่านบันทึกการติดตั้งนี้…
uninstaller-stopped-unexpectedly = การถอนการติดตั้งหยุดลงโดยไม่คาดคิด
uninstaller-what-will-be-removed = สิ่งที่จะถูกลบ
uninstaller-install-location = ตำแหน่งที่ติดตั้ง
uninstaller-files = ไฟล์
uninstaller-registry-values = ค่ารีจิสทรี
uninstaller-services = บริการ
uninstaller-drivers = ไดรเวอร์
uninstaller-scheduled-tasks = งานที่จัดกำหนดการไว้
uninstaller-environment = ตัวแปรสภาพแวดล้อม
uninstaller-plan-stays = ไฟล์ที่คุณเพิ่มหรือเปลี่ยนแปลงจะยังคงอยู่ พร้อมกับโฟลเดอร์ที่มีไฟล์เหล่านั้น
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
       *[other] ไฟล์ { $count } ไฟล์ที่การติดตั้งนี้แทนที่ไว้จะกลับคืนมา หรือจะไปอยู่ข้างไฟล์ปัจจุบัน หากไฟล์นั้นมีการเปลี่ยนแปลงหลังการติดตั้ง
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = รันไทม์ที่ติดตั้งไว้จะยังคงอยู่ เนื่องจากโปรแกรมอื่นอาจใช้งานอยู่: { $runtimes }
uninstaller-options = ตัวเลือก
uninstaller-remove-user-data = ลบข้อมูลผู้ใช้และข้อมูลสิทธิ์การใช้งานด้วย
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
       *[other] การตั้งค่า ข้อมูล และสิทธิ์การใช้งานของแอป { $count } รายการ ซึ่งจะยังคงอยู่หากไม่ได้เปิดตัวเลือกนี้ ของเดิมที่รายการเหล่านี้แทนที่ไว้จะกลับคืนมา หรือจะถูกวางไว้ข้างรายการเหล่านี้ หากมีการเปลี่ยนแปลงหลังจากนั้น
    }
uninstaller-uninstall = ถอนการติดตั้ง
uninstaller-cancel = ยกเลิก
uninstaller-close = ปิด
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = กำลังเลิกทำการติดตั้ง: { $done } จาก { $total }…
uninstaller-step = กำลังเลิกทำการติดตั้ง…
uninstaller-done = ถอนการติดตั้งแอปแล้ว
uninstaller-done-except = ถอนการติดตั้งแอปแล้ว ยกเว้นรายการที่แสดงด้านล่าง
uninstaller-waits-for-drive = บางส่วนอยู่บนไดรฟ์ที่ไม่ได้เชื่อมต่ออยู่ โปรดเชื่อมต่อไดรฟ์นั้น แล้วถอนการติดตั้งอีกครั้งจากการตั้งค่า
uninstaller-not-all-undone = ไม่สามารถเลิกทำได้ทั้งหมด โปรดปิดสิ่งที่กำลังใช้งานอยู่ แล้วถอนการติดตั้งอีกครั้งจากการตั้งค่า
# Over the path of the uninstall's log file.
uninstaller-log = ไฟล์บันทึก
uninstaller-undone = เลิกทำแล้ว
uninstaller-left-in-place = คงไว้ที่เดิม
uninstaller-could-not-be-undone = เลิกทำไม่ได้
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = และอีก { $count } รายการ
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = ไม่พบโฟลเดอร์ ProgramData ของพีซีเครื่องนี้ จึงไม่มีบันทึกการติดตั้งให้อ่าน

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = ตัวถอนการติดตั้ง WinAppMigrator
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = ไม่สามารถเปิดหน้าต่างของตัวถอนการติดตั้งได้: { $error }
uninstaller-window-failed-quiet = แต่ยังคงถอนการติดตั้งได้โดยไม่ต้องใช้หน้าต่าง จากพร้อมท์คำสั่งที่เรียกใช้ในฐานะผู้ดูแลระบบ:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = โปรแกรมนี้

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = กำลังถอนการติดตั้ง { $app_name }
# { $path } is the log file.
uninstaller-cli-log = ไฟล์บันทึก: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = ข้อผิดพลาด: { $error }
