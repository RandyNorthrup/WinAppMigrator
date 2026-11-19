# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } ผ่านลิงก์หรือไม่ได้อยู่บนไดรฟ์ที่เชื่อมต่อ จึงไม่ได้อ่าน
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } ไม่ได้อยู่บนดิสก์ของพีซีเครื่องนี้ หรือมีชื่อที่จะเปิดสิ่งอื่น จึงไม่ได้อ่าน
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } อยู่ในโฟลเดอร์ของบัญชีอื่นหรือไม่ใช่ของบัญชีนี้ และบันทึกของแอปนี้เองไม่ได้ระบุ ว่าโปรแกรมควรอยู่ที่นั่น จึงไม่ได้อ่าน
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-nothing-to-package = ไม่พบโปรแกรมของ { $app_name } เอง จึงไม่มีสิ่งใดที่จะใส่ในตัวติดตั้ง ไดรเวอร์ รันไทม์ และส่วนเสริมมักเป็นเช่นนี้ บนพีซีเครื่องใหม่ ให้ติดตั้งไดรเวอร์จากผู้ผลิตหรือ Windows Update และติดตั้งรันไทม์หรือส่วนเสริมพร้อมกับแอปที่ใช้งาน
# { $app_name } is the app's name, as the list of apps calls it. A web app is a website installed as an app, with
# a window and a Start menu entry of its own.
core-scan-web-app = { $app_name } เป็นเว็บแอปที่เบราว์เซอร์ติดตั้งจากเว็บไซต์ ไม่มีโปรแกรมของตัวเอง จึงไม่มีสิ่งใดที่จะใส่ในตัวติดตั้ง บนพีซีเครื่องใหม่ ให้เปิดเว็บไซต์นั้นในเบราว์เซอร์เดียวกันแล้วติดตั้งแอปจากที่นั่น
# { $path } is the program the app's own record names, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-missing = { $path } ไม่อยู่ที่นั่นแล้ว แม้ว่าบันทึกของแอปนี้จะระบุว่าเป็นโปรแกรมของแอป แอปอาจถูกย้ายหรือถูกลบออกบางส่วน การซ่อมแซมหรือติดตั้งแอปใหม่บนพีซีเครื่องนี้น่าจะนำกลับมาได้
# { $path } is the file that was picked.
core-scan-not-a-file = เป้าหมายการสแกนไม่ใช่ไฟล์: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = ตัวแปรสภาพแวดล้อม { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = ชื่อขึ้นต้นด้วยชื่อของแอปนี้
core-near-has-app-name = ชื่อมีชื่อของแอปนี้อยู่ด้วย
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = ชื่อขึ้นต้นด้วยชื่อของผู้เผยแพร่ { $publisher }
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = ส่วนที่ซ่อนไว้จากรายการแอป ซึ่งติดตั้งอยู่ข้างแอปนี้ใน { $folder }
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = ส่วนที่ซ่อนไว้จากรายการแอป จากผู้เผยแพร่รายเดียวกัน { $publisher }

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = รายการเหล่านี้ไปพร้อมกับโปรแกรมหลักเสมอ
core-footprint-locked-main-program = มีโปรแกรมหลัก ซึ่งรวมไว้เสมอ
core-footprint-locked-needed = มีสิ่งที่โปรแกรมหลักขาดไม่ได้ในการเริ่มทำงาน
core-footprint-locked-font = ไปพร้อมกับฟอนต์ ภายใต้ “สิ่งที่เชื่อมแอปเข้ากับ Windows”
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = การตั้งค่าในโปรไฟล์ของคุณ (AppData)
core-footprint-place-local = การตั้งค่าสำหรับพีซีเครื่องนี้ในโปรไฟล์ของคุณ (AppData)
core-footprint-place-program-data = ใช้ร่วมกันโดยทุกคนบนพีซี (ProgramData)
core-footprint-place-documents = ในโฟลเดอร์เอกสารของคุณ
core-footprint-place-saved-games = ในโฟลเดอร์เกมที่บันทึกไว้ของคุณ
core-footprint-place-public-documents = ในโฟลเดอร์เอกสารสาธารณะ
core-footprint-place-app-folder = ในโฟลเดอร์ของแอป
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = ใน { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = ไฟล์ที่อยู่ใน { $folder } โดยตรง
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = ไม่อยู่ในตัวติดตั้งนี้ เนื่องจากตัวเลือก “ข้อมูลสิทธิ์การใช้งานและการเปิดใช้งาน” ปิดอยู่
core-footprint-font-stays-behind = ฟอนต์นี้จะคงอยู่ที่เดิม
core-footprint-drivers-left-out = ไดรเวอร์ที่เป็นของ Windows หรือผู้จำหน่าย ซึ่งส่งต่อไม่ได้ โปรดติดตั้งไดรเวอร์เหล่านี้ใหม่บนพีซีเครื่องอื่น
core-footprint-driver-reinstall = ติดตั้งใหม่จากแพคเกจของผู้จำหน่ายเอง
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = ต้องใช้ { $service } ซึ่งไม่ได้รวมไว้
core-footprint-told-to-use-left-out = ถูกตั้งค่าให้ใช้บางอย่าง ในโฟลเดอร์ที่คุณไม่รวมไว้
core-footprint-file-in-left-out = ไฟล์ของรายการนี้อยู่ในโฟลเดอร์ที่คุณไม่รวมไว้
core-footprint-program-in-left-out = โปรแกรมของรายการนี้ อยู่ในโฟลเดอร์ที่คุณไม่รวมไว้
core-footprint-names-left-out = อ้างถึงโฟลเดอร์ที่คุณไม่รวมไว้
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = ผู้ที่สร้างตัวติดตั้งนี้ไม่ได้รวมไว้
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = การสแกนนี้เป็นของแอปอื่น ที่ไม่ใช่แอปที่กำลังสร้าง จึงไม่มีการสร้างสิ่งใด
core-package-scan-brought-in-more = การสแกนนี้นำข้อมูล ที่การสร้างครั้งนี้ไม่ได้ขอเข้ามาด้วย จึงไม่มีการสร้างสิ่งใด
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = ไม่มี { $path } อยู่แล้ว จึงไม่มีการสร้างสิ่งใด โปรดตรวจดูแอปอีกครั้ง
core-package-needs-stub = ไฟล์เก็บถาวรแบบแตกไฟล์ได้เอง ต้องใช้สตับตัวติดตั้ง
# { $reason } is what went wrong.
core-package-staged-file-failed = ไม่สามารถนำไฟล์ในโฟลเดอร์พักไฟล์ ใส่ลงในแพคเกจได้: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = พื้นที่ไม่เพียงพอสำหรับสร้างตัวติดตั้งนี้ จึงไม่มีการอ่านหรือเขียนสิ่งใด { $shortfalls } โปรดเพิ่มพื้นที่ว่างที่นั่น หรือเลือกโฟลเดอร์ผลลัพธ์บนไดรฟ์อื่น
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } ผ่านลิงก์หรือไม่ได้อยู่บนไดรฟ์ที่เชื่อมต่อ จึงไม่ได้อ่าน
# { $path } is a file's path.
core-package-not-read-not-local = { $path } ไม่ได้อยู่บนดิสก์ของพีซีเครื่องนี้ จึงไม่ได้อ่าน
# { $path } is a file's path.
core-package-not-read-other-account = { $path } นำไปสู่โฟลเดอร์ของบัญชีอื่น จึงไม่ได้อ่าน
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } ไม่ได้อยู่บนไดรฟ์ที่เชื่อมต่อ จึงไม่ได้อ่าน
# { $path } is a file's path.
core-package-backup-copy-no-parent = ไม่สามารถคัดลอกสำรองไฟล์ ที่ไม่มีไดเรกทอรีหลักได้: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = ปลายทางของการคัดลอกสำรอง ไม่มีไดเรกทอรีหลัก: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = การคัดลอกในโหมดสำรองข้อมูลของ robocopy ล้มเหลว (รหัสออก { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = โฟลเดอร์แพคเกจชั่วคราว
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = ตัวติดตั้ง
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = มีไฟล์อื่นอยู่ในตำแหน่งเดียวกันในแพคเกจ: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name } เปลี่ยนไปในโฟลเดอร์ที่ได้รับการป้องกัน ระหว่างที่กำลังสร้างแพคเกจ
core-package-worker-died = เธรดที่บีบอัดไฟล์หยุดลงโดยไม่คาดคิด จึงไม่มีการเขียนตัวติดตั้ง
# { $path } is a file's path.
core-package-link-not-read = { $path } เป็นลิงก์ จึงไม่ได้อ่าน
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = ไม่สามารถบีบอัด { $path } ได้: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = ไม่สามารถอ่าน { $path } กลับมาได้: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = โฟลเดอร์ที่จะเขียนลงไปต้องเป็นเส้นทางแบบเต็ม: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = โฟลเดอร์ที่จะเขียนลงไปไม่ใช่โฟลเดอร์: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = เส้นทางออกไปนอกโฟลเดอร์รากที่กำหนด: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = ปฏิเสธการเขียนผ่านลิงก์: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = ปฏิเสธการผ่านลิงก์: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = ปฏิเสธการผ่านลิงก์: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } เป็นลิงก์ที่ชี้ไปยังที่อื่น และผู้อื่นอาจเปลี่ยนปลายทางของลิงก์ได้ โปรดใช้ตำแหน่งที่ลิงก์ชี้ไปแทน
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } ไม่มีโฟลเดอร์หลัก
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = ตำแหน่งที่ติดตั้งต้องเป็นเส้นทางแบบเต็ม: “{ $path }”
# { $path } is the install location.
core-discovery-install-location-a-link = ตำแหน่งที่ติดตั้งเป็นลิงก์ที่ชี้ไปยังที่อื่น: { $path }
# { $path } is a file's path.
core-discovery-device-name = เส้นทางใช้ชื่อที่สงวนไว้สำหรับอุปกรณ์ของ Windows: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } มีชื่อที่ Windows เวอร์ชันนี้เปิดเป็นอุปกรณ์แทนที่จะเปิดเป็นไฟล์ จึงไม่มีการเขียนสิ่งใดลงไป แต่สามารถติดตั้งบน Windows 11 ได้
# { $path } is a file's path.
core-discovery-unsafe-path = เส้นทางไม่ปลอดภัย: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = ไฟล์ไม่มีเส้นทางสัมพัทธ์ในแพคเกจ: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = แพคเกจของตัวติดตั้ง
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = สตับตัวติดตั้งที่ฝังอยู่
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = ไม่สามารถอ่านรายการที่ { $index } ของแพคเกจได้: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = รายการ zip มีเส้นทางที่ไม่ปลอดภัย: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } มีข้อมูลมากกว่าที่ระบุไว้
# { $name } is a file's name inside the package.
core-archive-name-too-long = ชื่อนี้ยาวเกินไปสำหรับ zip: { $name }
core-archive-changed-while-copied = แพคเกจเปลี่ยนไป ระหว่างที่กำลังนำไปต่อท้ายสตับ จึงไม่มีการเขียนตัวติดตั้ง
core-archive-stub-not-a-program = ส่วนของตัวติดตั้งนี้ที่อยู่ก่อนแพคเกจ ไม่ใช่โปรแกรม
core-archive-damaged = ตัวติดตั้งนี้เสียหายหรือถูกเปลี่ยนแปลง หลังจากสร้าง จึงไม่มีการอ่านสิ่งใดจากแพคเกจ โปรดคัดลอกตัวติดตั้งอีกครั้งจากที่ที่สร้างไว้
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = ตัวติดตั้งนี้เก็บแพคเกจไว้ในไฟล์ชื่อ { $name } ในโฟลเดอร์เดียวกัน แต่ไม่มีไฟล์นั้นอยู่ ทั้งสองไฟล์ต้องอยู่ด้วยกัน: คัดลอก { $name } มาไว้ที่นี่จากที่ที่สร้างตัวติดตั้ง แล้วเรียกใช้อีกครั้ง
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = ไม่สามารถเปิดไฟล์ข้อมูลของตัวติดตั้งนี้ { $name } ได้: { $error } ทั้งสองไฟล์ต้องอยู่ด้วยกัน ในโฟลเดอร์ที่พีซีเครื่องนี้อ่านได้
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } เป็นลิงก์หรือโฟลเดอร์ แต่ไฟล์ข้อมูลของตัวติดตั้งนี้ ต้องเป็นไฟล์จริงที่อยู่ข้างตัวติดตั้ง จึงไม่มีการอ่านสิ่งใดจากไฟล์นั้น โปรดคัดลอกไฟล์จริงมาไว้ที่นี่
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } ไม่ใช่ไฟล์ข้อมูลของตัวติดตั้งนี้ หรือเสียหายหรือถูกเปลี่ยนแปลงหลังจากสร้าง จึงไม่มีการอ่านสิ่งใดจากไฟล์นั้น โปรดคัดลอกทั้งสองไฟล์อีกครั้งจากที่ที่สร้างไว้
core-archive-stub-signed = สตับตัวติดตั้งถูกลงนามไว้ แต่สตับต้องไม่ลงนาม โปรดลงนามตัวติดตั้งที่สร้างเสร็จแล้วแทน
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = ตัวติดตั้งจะมีขนาด { $size } และ Windows ไม่สามารถเรียกใช้ไฟล์ปฏิบัติการที่มีขนาด 4 GB ขึ้นไปได้ จึงไม่มีการเขียนสิ่งใด
core-archive-outside-signature = ตัวติดตั้งมีข้อมูลอยู่นอกลายเซ็น และอาจถูกดัดแปลง
core-archive-footer-length = ความยาวส่วนท้ายของไฟล์เก็บถาวรแบบแตกไฟล์ได้เอง ไม่ถูกต้อง
core-archive-seek-outside = การเลื่อนตำแหน่งออกไปนอกแพคเกจ
# { $path } is a file's path.
core-archive-unsafe-package-path = เส้นทางในแพคเกจไม่ปลอดภัย: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = เส้นทางในแพคเกจว่างเปล่า: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } ไม่ใช่อิมเมจดิสก์ที่ Windows แนบได้ (.vhd หรือ .vhdx)
core-image-needs-administrator = การแนบอิมเมจดิสก์ต้องใช้สิทธิ์ของผู้ดูแลระบบ
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } ไม่ใช่ไฟล์
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } เป็นดิสก์ส่วนต่าง ซึ่งเก็บเฉพาะสิ่งที่เปลี่ยนไปจากดิสก์ต้นแบบ และ Windows จะเปิดดิสก์ต้นแบบนั้นด้วย ไม่ว่าดิสก์ส่วนต่างจะระบุว่าดิสก์ต้นแบบอยู่ที่ใด โปรดผสานดิสก์นี้เข้ากับดิสก์ต้นแบบก่อนด้วย “แก้ไขดิสก์” ของ Hyper-V แล้วเลือกดิสก์ต้นแบบนั้น
# { $path } is the disk image file.
core-image-unreadable = { $path } ไม่ใช่ไฟล์ .vhd หรือ .vhdx ที่อ่านได้
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = ไม่พบโฟลเดอร์ ProgramData ของพีซีเครื่องนี้
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows แนบอิมเมจนี้อยู่แล้ว และสิ่งที่แนบไว้อาจกำลังเขียนข้อมูลลงไป โปรดถอดอิมเมจออกก่อน หรือเลือกไดรฟ์ของอิมเมจเป็นไดรฟ์ที่เชื่อมต่อ
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = ไม่สามารถแนบ { $path } ได้: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } ไม่มีโวลุ่มที่มี Windows ซึ่งอ่านได้ โวลุ่มที่เข้ารหัสด้วย BitLocker ต้องปลดล็อกก่อน
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows ไม่ได้ระบุสาเหตุ

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = พีซีเครื่องอื่นใช้เวลาตอบนานเกินไป
core-remote-closed = การเชื่อมต่อกับพีซีเครื่องอื่นถูกปิดแล้ว
core-remote-failed = การเชื่อมต่อกับพีซีเครื่องอื่นล้มเหลว
core-remote-answered-something-else = ไคลเอ็นต์ตอบกลับเป็นอย่างอื่น
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = ไม่พบ { $name } บนเครือข่าย
core-remote-did-not-say-what-it-is = ไคลเอ็นต์ไม่ได้ระบุว่าตนเองคืออะไร
core-remote-ordinary-names-only = สอบถามได้เฉพาะเส้นทาง ที่อยู่ใต้รากของพีซีเครื่องอื่น และใช้ชื่อปกติเท่านั้น
core-remote-more-but-none = พีซีเครื่องอื่นแจ้งว่าโฟลเดอร์มีรายการเพิ่มเติม แต่ไม่ได้ส่งรายการใดมาเลย
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = พีซีเครื่องอื่นแสดงรายการมากกว่า { $count } รายการในโฟลเดอร์เดียว
core-remote-sent-too-much = พีซีเครื่องอื่นส่งข้อมูลของไฟล์มามากกว่าที่ขอ
core-remote-name-not-text = ไม่สามารถสอบถามชื่อที่ไม่ใช่ข้อความได้
core-remote-below-root-only = สอบถามได้เฉพาะเส้นทาง ที่อยู่ใต้รากของพีซีเครื่องอื่นเท่านั้น
core-remote-file-ended-early = ไฟล์ของพีซีเครื่องอื่นสิ้นสุดก่อนกำหนด
core-remote-link-not-followed = จะไม่ติดตามลิงก์บนพีซีเครื่องอื่น
core-remote-only-files-opened = เปิดได้เฉพาะไฟล์บนพีซีเครื่องอื่นเท่านั้น
core-remote-listing-said-nothing = รายการไม่ได้ระบุถึงสิ่งนี้
core-remote-change-time-unknown = ไม่ทราบเวลาที่เปลี่ยนแปลง
core-remote-seek-before-start = การเลื่อนตำแหน่งไปก่อนจุดเริ่มต้น
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
       *[other] มีขนาด { $size } ไบต์ แต่คาดไว้ไม่เกิน { $limit } ไบต์
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = ไม่สามารถเริ่ม PowerShell ได้: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = ไม่สามารถอ่านลายเซ็นได้: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = ลงนามโดย { $signer } ไม่ใช่ Microsoft
core-signature-not-microsoft-unnamed = ลงนามโดยผู้อื่น ไม่ใช่ Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = สถานะลายเซ็นคือ { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = ไม่สามารถอ่านสิ่งที่โปรแกรมนี้ต้องการจาก Windows ได้ จึงจะไม่มีการตรวจสอบ ว่าโปรแกรมทำงานบนพีซีเครื่องอื่นได้หรือไม่
core-warning-build-external-manifest = ตรวจพบรายการแสดงแอปพลิเคชันภายนอก การตีความส่วนประกอบที่ต้องใช้แบบเคียงข้างกัน จะเลื่อนไปทำภายหลัง
core-warning-build-non-exe = ไฟล์ที่เลือกไม่ได้ใช้นามสกุล .exe การสแกน PE จะดำเนินต่อไป
core-warning-build-stand-in-program = ไม่พบไฟล์ปฏิบัติการของแอปพลิเคชันตามปกติ จึงใช้ไฟล์นี้แทน ผลลัพธ์ด้านสถาปัตยกรรมและส่วนประกอบที่ต้องใช้ อธิบายถึงไฟล์นี้ และอาจไม่ได้อธิบายถึงแอปพลิเคชัน
core-warning-build-other-apps-files-unknown = ไม่สามารถอ่านข้อมูลทั้งหมดที่ Windows Installer มีเกี่ยวกับแอปอื่นได้ ดังนั้นโฟลเดอร์ที่บันทึกไว้ว่าเป็นของแอปนี้ อาจมีไฟล์ของแอปอื่นอยู่ โดยไม่มีสิ่งใดในที่นี้แจ้งให้ทราบ
core-warning-build-app-folder-has-another-apps-file = Windows Installer ระบุว่าแอปอื่นมีไฟล์อยู่ ในโฟลเดอร์เดียวกับโปรแกรมของแอปนี้ และไฟล์นั้นจะถูกนำไปด้วย เนื่องจากโฟลเดอร์นั้นถูกนำไปทั้งหมด
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = รายการ App Paths ของแอปนี้ระบุว่าแอปค้นหาในโฟลเดอร์ ที่ไม่ใช่ของตนเอง จึงไม่ได้นำโฟลเดอร์นั้นไปด้วย แอปอาจเริ่มทำงานไม่ได้ บนพีซีที่ไม่มีสิ่งที่อยู่ในโฟลเดอร์นั้น
core-warning-build-uninstaller-folder-left-out = ไม่ได้รวมโฟลเดอร์ ที่มีตัวถอนการติดตั้งของแอปนี้ไว้ เนื่องจาก Windows Installer ระบุว่าแอปอื่นมีไฟล์อยู่ที่นั่น
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    รายการหนึ่งในบันทึกของแอปนี้ ระบุถึงโฟลเดอร์ที่ไม่ได้รวมไว้ เนื่องจาก{ $why ->
        [unreadable] เส้นทางออกไปนอกไดรฟ์ หรือผ่านลิงก์ จึงไม่สามารถอ่านได้อย่างปลอดภัย
        [missing] ไม่มีอยู่
        [link] ผ่านลิงก์
        [short-name] ไม่ได้เขียนตามชื่อจริง
        [another-account] อยู่ในโฟลเดอร์ของบัญชีอื่น
        [one-account] แอปนี้ติดตั้งไว้สำหรับบัญชีเดียว และรายการนี้ไม่ใช่ของบัญชีนั้น
        [shared] มีสิ่งที่ใช้ร่วมกัน หรือเป็นส่วนหนึ่งของ Windows
        [generic-name] ชื่อไม่ได้บอกว่าเป็นของใคร
        [publisher] ตั้งชื่อตามผู้เผยแพร่เท่านั้น และอาจมีผลิตภัณฑ์อื่นของผู้เผยแพร่อยู่ด้วย
        [holds-the-app] มีโฟลเดอร์ของแอปนี้อยู่ภายใน และอาจมีสิ่งอื่นอีก
        [another-apps-name] ใช้ชื่อของแอปอื่น
        [another-apps-record] บันทึกการติดตั้งเป็นของแอปอื่น
        [in-another-apps-folder] อยู่ในโฟลเดอร์ของแอปอื่น
       *[another-apps] เป็นของแอปอื่น
    }
core-warning-build-app-folder-holds-another-app = แอปอื่นบนพีซีเครื่องนี้ ถูกบันทึกไว้ว่าอยู่ภายในโฟลเดอร์ของแอปนี้ และจะถูกนำไปด้วย เนื่องจากโฟลเดอร์นั้นถูกนำไปทั้งหมด
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = แอปนี้อยู่บน Windows 32 บิต คีย์รีจิสทรีของแอปจึงถูกจัดวางแบบที่ Windows 64 บิตเก็บคีย์ของแอป 32 บิต และตัวติดตั้งจะเสนอ Program Files (x86) ให้แอปนี้
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = โปรแกรมหนึ่งต้องใช้ { $dll } ซึ่งเป็นรันไทม์ Visual C++ รุ่นดีบัก Microsoft ไม่มีตัวติดตั้งสำหรับรันไทม์ดังกล่าว และอาจไม่อนุญาตให้ส่งต่อ โปรแกรมจึงจะทำงานไม่ได้บนพีซีที่ไม่มี Visual Studio
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } เป็นโปรแกรม .NET ที่รวมเป็นไฟล์เดียว ซึ่งไม่ได้ระบุว่าต้องใช้ .NET ใด จึงไม่ได้วางแผนติดตั้ง .NET ให้ หากโปรแกรมต้องใช้ .NET ก็จะเริ่มทำงานไม่ได้บนพีซีที่ไม่มี .NET นั้น
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
       *[other] ไม่ได้ตรวจดูโปรแกรมของแอป { $count } โปรแกรมว่าต้องใช้รันไทม์ใด เนื่องจากแอปมีโปรแกรมจำนวนมาก หรือโปรแกรมมีขนาดใหญ่มาก
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = แอปต้องใช้ { $runtime } แต่ไม่มีตัวติดตั้งสำหรับรันไทม์นี้ที่นี่ จึงไม่มีการเรียกใช้ตัวติดตั้งใด โปรดติดตั้งรันไทม์นี้ด้วยตนเองบนพีซีเครื่องอื่น
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = ไม่ได้รวมไว้ในแพคเกจ: { $why }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = ไม่สามารถหาส่วนประกอบที่ต้องใช้ได้: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = พบ { $name } ผ่านเส้นทางการค้นหาของพีซีเครื่องนี้เท่านั้น และไม่ได้เป็นส่วนหนึ่งของแอป จึงไม่ได้นำไปด้วย แอปอาจเริ่มทำงานไม่ได้บนพีซีที่ไม่มีไฟล์นี้
core-warning-build-driver-detected = ตรวจพบไดรเวอร์หรือส่วนประกอบเคอร์เนล ไดรเวอร์ที่เป็นของแอปจะถูกรวมไว้ และลงทะเบียนใหม่ ส่วนไดรเวอร์ของผู้จำหน่าย หรือไดรเวอร์ที่มาพร้อมกับ Windows จะไม่ถูกแจกจ่ายต่อ และต้องติดตั้งใหม่ จากแพคเกจของผู้จำหน่ายที่ลงนามแล้ว
core-warning-build-near-no-longer-offered = รายการที่ทำเครื่องหมายไว้ ในรายการสิ่งที่อาจเป็นส่วนหนึ่งของแอปนี้ ไม่อยู่ในรายการนั้นแล้ว จึงไม่ได้เพิ่ม โปรดดูรายการอีกครั้ง
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    ส่วนหนึ่งของสิ่งที่ { $app_name } ระบุว่าเป็นของตนไม่ได้รวมไว้: { $why ->
        [unreadable] เส้นทางออกไปนอกไดรฟ์ หรือผ่านลิงก์ จึงไม่สามารถอ่านได้อย่างปลอดภัย
        [missing] ไม่มีอยู่
        [link] ผ่านลิงก์
        [short-name] ไม่ได้เขียนตามชื่อจริง
        [another-account] อยู่ในโฟลเดอร์ของบัญชีอื่น
        [one-account] แอปนี้ติดตั้งไว้สำหรับบัญชีเดียว และรายการนี้ไม่ใช่ของบัญชีนั้น
        [shared] มีสิ่งที่ใช้ร่วมกัน หรือเป็นส่วนหนึ่งของ Windows
        [generic-name] ชื่อไม่ได้บอกว่าเป็นของใคร
        [publisher] ตั้งชื่อตามผู้เผยแพร่เท่านั้น และอาจมีผลิตภัณฑ์อื่นของผู้เผยแพร่อยู่ด้วย
        [holds-the-app] มีโฟลเดอร์ของแอปนี้อยู่ภายใน และอาจมีสิ่งอื่นอีก
        [another-apps-name] ใช้ชื่อของแอปอื่น
        [another-apps-record] บันทึกการติดตั้งเป็นของแอปอื่น
        [in-another-apps-folder] อยู่ในโฟลเดอร์ของแอปอื่น
       *[another-apps] เป็นของแอปอื่น
    }
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = ไม่ได้รวมคีย์รีจิสทรีที่บันทึกการติดตั้งของ { $app_name } ระบุไว้ เนื่องจากบันทึกนั้นระบุถึงสิ่งที่เป็นของแอปอื่นด้วย
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = มีโปรแกรมอยู่ในโฟลเดอร์ที่ Windows เก็บสิ่งที่แอปเขียนไว้สำหรับบัญชีเดียว Windows ไม่เคยวางโปรแกรมไว้ที่นั่น จึงเป็นสิ่งอื่นที่วางไว้ และโปรแกรมนั้นไม่ได้รวมไว้
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = ไม่ได้รวม { $path } ไว้: รายการนี้ผ่านลิงก์บนไดรฟ์ที่เชื่อมต่อ หรือเป็นของบัญชีอื่น ที่ไม่ใช่บัญชีที่กำลังรวมข้อมูล
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = โฟลเดอร์ { $folder } ของ { $user } อยู่ที่ { $path } ซึ่งไม่ได้อยู่บนไดรฟ์นี้ จึงไม่ได้รวมสิ่งที่อยู่ในนั้นไว้
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = โฟลเดอร์ { $folder } ของ { $user } อยู่ที่ { $path } ซึ่งไม่ได้อยู่บนดิสก์ของพีซีเครื่องนี้ จึงไม่ได้รวมสิ่งที่อยู่ในนั้นไว้
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = ไม่สามารถอ่าน { $path } ได้ ({ $error }) จึงไม่ได้รวมข้อมูลแอปพลิเคชันที่เก็บไว้ที่นั่น
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = ไม่สามารถอ่าน { $path } ได้ ({ $error }) ข้อมูลแอปพลิเคชันที่ซ้อนอยู่ในนั้น อาจขาดหายไป
# { $error } is what went wrong.
core-warning-build-entry-unreadable = ไม่สามารถอ่านรายการในระบบไฟล์ได้: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
       *[other] ไม่ได้ติดตามลิงก์หรือชื่อที่จะเปิดสิ่งอื่น { $count } รายการในโฟลเดอร์นี้ สิ่งที่รายการเหล่านั้นชี้ไปจึงไม่อยู่ในตัวติดตั้ง: { $names }
    }{ $more ->
        [0] { "" }
       *[other] { " " }และอีก { $more } รายการ
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
       *[other] โฟลเดอร์ { $count } โฟลเดอร์อยู่ลึกเกิน { $depth } ระดับ สิ่งที่อยู่ในโฟลเดอร์เหล่านั้นจึงไม่อยู่ในตัวติดตั้ง: { $names }
    }{ $more ->
        [0] { "" }
       *[other] { " " }และอีก { $more } รายการ
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = ไม่สามารถอ่านเมตาดาต้าของไฟล์ได้: { $error }
core-warning-build-reserved-name = ชื่อของไฟล์นี้สงวนไว้สำหรับอุปกรณ์ของ Windows จึงไม่ได้คัดลอก
# { $error } is what went wrong.
core-warning-build-hash-failed = ไม่สามารถคำนวณแฮชของไฟล์ได้: { $error }
core-warning-build-cloud-placeholder = ไฟล์บนคลาวด์ที่เป็นแบบออนไลน์เท่านั้น (เช่น ไฟล์ตามความต้องการของ OneDrive) ไม่มีข้อมูลอยู่ในเครื่อง โปรดตั้งค่าให้เก็บไว้ในอุปกรณ์ หรือดาวน์โหลดไฟล์ก่อนสำรองข้อมูล ได้รายงานไว้แต่ไม่ได้คัดลอก
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = ไฟล์ไดรเวอร์/เคอร์เนล “{ $file_name }” จะไม่ถูกคัดลอกเป็นไฟล์ธรรมดา หากไฟล์นี้ใช้กับบริการไดรเวอร์ ที่ลงทะเบียนไว้ของแอปนี้ ตัวติดตั้งจะรวมไฟล์ไว้ และลงทะเบียนไดรเวอร์ใหม่โดยอัตโนมัติ มิฉะนั้นโปรดติดตั้งใหม่ ด้วยแพคเกจไดรเวอร์ที่ลงนามแล้วของผู้จำหน่าย
core-warning-build-file-gone = รายการนี้มีอยู่ตอนที่ตรวจดูแอป แต่ตอนนี้หายไปแล้ว จึงไม่อยู่ในตัวติดตั้ง
# { $error } is what went wrong.
core-warning-build-file-copy-failed = ไม่สามารถคัดลอกไฟล์ลงในแพคเกจได้: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
       *[other] แพคเกจนี้มีไฟล์ ที่ไม่สามารถตรวจได้ว่ามาจากใคร: { $names } เนื้อหาในไฟล์อาจระบุข้อมูลดังกล่าว
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = แอปนี้มีโฟลเดอร์ว่างมากกว่า { $count } โฟลเดอร์ และจะสร้างขึ้นใหม่ได้เพียง { $count } โฟลเดอร์เท่านั้น
core-warning-build-credential-by-now = ไม่ได้รวมไว้ในแพคเกจ: ตอนนี้ดูเหมือนข้อมูลประจำตัว
core-warning-build-no-package-path = ไฟล์ที่คัดลอกได้ไม่มีเส้นทางสัมพัทธ์ในแพคเกจ
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = ไม่สามารถวางส่วนประกอบที่ต้องมีก่อนที่รวมมา { $file_name } ได้: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = ไม่สามารถดาวน์โหลด { $runtime } ได้ ({ $error }) ตัวติดตั้งจะลองใช้ winget บนเครื่องเป้าหมาย
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = { $runtime } ที่ดาวน์โหลดมาไม่ผ่านการตรวจสอบลายเซ็น จึงไม่ได้รวมไว้: { $reason } ตัวติดตั้งจะลองใช้ winget บนเครื่องเป้าหมาย
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = ดาวน์โหลด { $runtime } แล้ว แต่ไม่สามารถคำนวณแฮชได้ ({ $error }) ตัวติดตั้งจะลองใช้ winget บนเครื่องเป้าหมายแทน
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = ไม่สามารถรวมอิมเมจไดรเวอร์สำหรับ “{ $service }” ได้: { $error }
core-warning-build-license-not-copied = ไม่สามารถคัดลอกข้อมูลสิทธิ์การใช้งานได้ (ไม่มีอยู่หรืออ่านไม่ได้) จึงบันทึกไว้เป็นเมตาดาต้าเท่านั้น
core-warning-build-license-file-credential = ไฟล์สิทธิ์การใช้งานดูเหมือนข้อมูลประจำตัว จึงไม่ได้รวมไว้
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = ไม่สามารถแจกแจงบางส่วน ของที่เก็บสิทธิ์การใช้งานได้: { $error }
core-warning-build-license-store-credential = ไฟล์ในที่เก็บสิทธิ์การใช้งาน ดูเหมือนข้อมูลประจำตัว จึงไม่ได้รวมไว้
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = ข้ามไฟล์ในที่เก็บสิทธิ์การใช้งาน: { $error }
