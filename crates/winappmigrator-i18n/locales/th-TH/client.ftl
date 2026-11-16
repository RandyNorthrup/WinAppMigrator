# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = คอนโซลนี้แสดงภาษา { $language } ไม่ได้ ไคลเอ็นต์จึงใช้ภาษาอังกฤษ
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client ไม่สามารถเริ่มทำงานจากโฟลเดอร์ที่เฉพาะผู้ดูแลระบบ
    เท่านั้นที่เปลี่ยนแปลงได้: { $error }
client-press-enter-to-close = กด Enter เพื่อปิดหน้าต่างนี้
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    โปรแกรมนี้ช่วยให้ WinAppMigrator บนพีซีเครื่องใหม่ของคุณอ่านแอป
    บนพีซีเครื่องนี้ผ่านเครือข่ายของคุณได้ โดยไม่เปลี่ยนแปลงสิ่งใดในเครื่องนี้
    เพียงอ่านจากสแนปช็อตของไดรฟ์ Windows ซึ่งจะถูกลบเมื่อคุณหยุดโปรแกรม
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = ไคลเอ็นต์หยุดทำงาน: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    หากปิดหน้าต่างนี้แทนการหยุดโปรแกรม สแนปช็อตและกฎไฟร์วอลล์จะยังคงอยู่
    จนกว่าจะเรียกใช้ไคลเอ็นต์ที่นี่ครั้งถัดไป ({ $error })
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = กำลังสร้างสแนปช็อตของไดรฟ์ Windows...{ " " }
client-snapshot-taken = เสร็จสิ้น
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    พีซีเครื่องนี้จะไม่ปรากฏบนเครือข่ายโดยอัตโนมัติ ({ $error })
    บนพีซีเครื่องใหม่ ให้พิมพ์ชื่อหรือที่อยู่ของพีซีเครื่องนี้แทน

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }พีซีเครื่องนี้:      { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:       { $windows_name }, { $bits } บิต
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }รหัสการจับคู่:     { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    บนพีซีเครื่องใหม่ ใน WinAppMigrator ให้เลือก “พีซีบนเครือข่าย...” แล้วพิมพ์รหัสนี้
    ปิดแอปที่คุณกำลังย้ายก่อน เพื่อให้ไฟล์ของแอปอยู่ในสภาพเดิมที่แอปทิ้งไว้
    หากต้องการหยุด ให้กด Enter ที่นี่ หรือปิดหน้าต่างนี้

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    จับคู่กับ { $from ->
        [unknown] พีซีเครื่องอื่น
       *[address] { $from }
    } แล้ว กำลังอ่านข้อมูลที่ต้องใช้...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] พีซีเครื่องอื่น
       *[address] { $from }
    } ทำงานเสร็จแล้ว ({ $count ->
       *[other] ตอบคำขอ { $count } รายการ
    })
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    การเชื่อมต่อกับ { $from ->
        [unknown] พีซีเครื่องอื่น
       *[address] { $from }
    } สิ้นสุดลง: { $error }
client-wrong-code =
    { $from ->
        [unknown] พีซีเครื่องอื่น
       *[address] { $from }
    } ใส่รหัสที่ไม่ตรงกับรหัสนี้
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] พีซีเครื่องอื่น
       *[address] { $from }
    } จับคู่ไม่สำเร็จ: { $error }
client-too-many-wrong-codes =
    มีการลองใช้รหัสผิดหลายครั้งเกินไป ตอนนี้จึงไม่มีพีซีเครื่องใดจับคู่กับ
    เครื่องนี้ได้ อาจมีบางอย่างบนเครือข่ายของคุณกำลังเดารหัส หากต้องการจับคู่
    ให้หยุดไคลเอ็นต์แล้วเริ่มใหม่ จากนั้นพิมพ์รหัสใหม่ที่ไคลเอ็นต์แสดง
# { $code } is six digits in two groups, such as 482 913.
client-new-code = รหัสผิดหลายครั้งเกินไป รหัสการจับคู่ใหม่คือ: { $code }

## Stopping

client-stopping = กำลังหยุด: กำลังลบสแนปช็อตและกฎไฟร์วอลล์...
client-stop-done = เสร็จสิ้น

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = ไคลเอ็นต์กำลังทำงานอยู่บนพีซีเครื่องนี้แล้ว หรือกำลังล้างข้อมูลหลังจากไคลเอ็นต์ที่ถูกปิดไป โปรดใช้ไคลเอ็นต์ที่ทำงานอยู่ หรือรอสักครู่แล้วเริ่มใหม่อีกครั้ง
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows ไม่ได้สร้างสแนปช็อตของ { $drive }: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = ไฟร์วอลล์ Windows ไม่อนุญาตให้ไคลเอ็นต์รอรับการเชื่อมต่อ: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client ใช้ตัวเลือกต่อไปนี้:
    
      --lang <ภาษา>  ใช้ภาษานี้ เช่น it-IT หากไคลเอ็นต์มีภาษานั้น
                     แทนภาษาที่ใช้แสดงของ Windows
      --code <รหัส>   เริ่มด้วยรหัสการจับคู่ 6 หลักนี้ สำหรับสคริปต์ที่เริ่ม
                     WinAppMigrator บนพีซีเครื่องใหม่ด้วย
      --help         แสดงข้อความนี้
