# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = สิ่งที่การติดตั้งนี้จะทำได้เปลี่ยนไป หลังจากที่ได้รับความยินยอมแล้ว จึงไม่มีการติดตั้งสิ่งใด
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = การดำเนินการ I/O ล้มเหลวสำหรับ { $path }: { $error }
# { $detail } is what went wrong.
core-error-registry = การอ่านรีจิสทรีล้มเหลว: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = การแยกวิเคราะห์ PE ล้มเหลวสำหรับ { $path }: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = ข้อมูลป้อนเข้าไม่ถูกต้อง: { $detail }
# { $detail } is what went wrong.
core-error-serialization = การทำให้รายงานเป็นอนุกรมล้มเหลว: { $detail }
# { $detail } is what went wrong.
core-error-archive = การดำเนินการกับไฟล์เก็บถาวรล้มเหลว: { $detail }
# { $detail } is what went wrong.
core-error-network = การดำเนินการผ่านเครือข่ายล้มเหลว: { $detail }
# { $detail } is what went wrong.
core-error-time-format = การจัดรูปแบบเวลาล้มเหลว: { $detail }
core-error-cancelled = ถูกยกเลิก
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = ไม่ได้ระบุเหตุผล

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = ไม่รู้จักชนิดค่ารีจิสทรี { $kind }
core-journal-not-elevated = ไม่ได้ทำงานด้วยสิทธิ์ที่ยกระดับ
core-journal-needs-admin = การติดตั้งต้องใช้สิทธิ์ของผู้ดูแลระบบ เพื่อเก็บบันทึกการติดตั้งไว้ ในที่ที่ผู้อื่นเปลี่ยนแปลงไม่ได้
# { $path } is a folder.
core-journal-not-an-install-folder = นี่ไม่ใช่โฟลเดอร์ของการติดตั้ง: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = ไม่สามารถเขียนบันทึกการติดตั้งนี้ได้: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = เส้นทางออกไปนอกโฟลเดอร์รากที่กำหนด: { $path }
# { $path } is a folder.
core-journal-nothing-exists = ไม่มีสิ่งใดในนี้อยู่บนพีซีเครื่องนี้: { $path }
core-journal-busy = มีการติดตั้งหรือถอนการติดตั้งอื่นกำลังทำงานอยู่ โปรดรอให้เสร็จ แล้วลองอีกครั้ง
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = ไฟล์ไม่มีโฟลเดอร์
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = ไม่สามารถอ่านสิทธิ์ที่เก็บไว้ได้
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = ไม่สามารถใช้บันทึกการติดตั้งใน { $folder } ได้: บันทึกว่างเปล่า
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = ไม่สามารถใช้บันทึกการติดตั้งใน { $folder } ได้: บันทึกไม่ได้ขึ้นต้นด้วยข้อมูล ว่าเป็นบันทึกของอะไร
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = ไม่สามารถใช้บันทึกการติดตั้งใน { $folder } ได้: บันทึกถูกเขียนโดยโปรแกรมนี้เวอร์ชันอื่น
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = ไม่สามารถใช้บันทึกการติดตั้งใน { $folder } ได้: บันทึกระบุว่าเป็นของการติดตั้งอื่น
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = นี่ไม่ใช่รหัสของการติดตั้ง: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = ไม่สามารถระบุ --apply และ --dry-run พร้อมกันได้
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = ตำแหน่งที่ติดตั้งต้องเป็นเส้นทางแบบเต็ม: “{ $path }”
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = ไม่สามารถใช้ตำแหน่งที่ติดตั้งนั้นได้: { $problem }: “{ $path }”
core-restore-needs-admin-plan = แผนการคืนค่ามีการเปลี่ยนแปลงรีจิสทรี บริการ งานที่จัดกำหนดการไว้ ทางลัด หรือเป้าหมายที่ได้รับการป้องกัน ซึ่งต้องใช้สิทธิ์ของผู้ดูแลระบบ
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = โฟลเดอร์รากเป้าหมายต้องใช้สิทธิ์ที่ยกระดับ: { $path }
core-restore-needs-admin-user-data = การคืนค่าข้อมูลผู้ใช้ลงใน ProgramData หรือตำแหน่งอื่นที่ได้รับการป้องกัน ต้องใช้สิทธิ์ที่ยกระดับ
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error } ไม่สามารถย้อนกลับสิ่งที่ทำไปแล้วได้ทั้งหมด: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = ไม่มีที่ปลอดภัยสำหรับเก็บบันทึก ที่จะใช้เลิกทำการติดตั้งนี้ จึงไม่มีการติดตั้งสิ่งใด: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = มี { $path } อยู่แล้ว จึงไม่มีการติดตั้งสิ่งใด ใช้ --force เพื่อแทนที่
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = แอปนี้มีสิ่งที่ต้องคืนค่าไปยัง { $root } และ { $link } เป็นลิงก์ที่ชี้ไปยังที่อื่นซึ่งผู้อื่นอาจเปลี่ยนแปลงได้ จึงไม่มีการติดตั้งสิ่งใด หากสิ่งที่จะไปที่นั่นเป็นข้อมูลผู้ใช้ หรือข้อมูลสิทธิ์การใช้งาน ให้ติดตั้งโดยไม่รวมข้อมูลผู้ใช้ และข้อมูลสิทธิ์การใช้งาน หากเป็นทางลัดในโฟลเดอร์ Startup ให้ย้ายโฟลเดอร์นั้นกลับไปไว้ที่เดิม
core-restore-hash-check-failed = แพคเกจไม่ผ่านการตรวจสอบแฮช และไม่มีการติดตั้งสิ่งใด
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = แพคเกจไม่ผ่านการตรวจสอบแฮช และไม่มีการติดตั้งสิ่งใด: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = ปฏิเสธการเขียนลงในตำแหน่ง ที่การติดตั้งเก็บบันทึกไว้: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = ปฏิเสธการเขียนนอกตำแหน่งที่ติดตั้ง และโฟลเดอร์ข้อมูลที่รู้จัก: { $path }
# { $path } is a file.
core-restore-no-parent-folder = ปลายทางไม่มีโฟลเดอร์หลัก: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = มีโฟลเดอร์อยู่ในตำแหน่งที่ไฟล์นี้ต้องไปอยู่: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } ไม่ตรงกับที่อยู่ในแพคเกจแล้ว จึงไม่ได้ติดตั้ง
# { $path } is a file in the package.
core-restore-source-is-link = { $path } เป็นลิงก์ และจะไม่ติดตามลิงก์นี้
# { $path } is a file in the package.
core-restore-source-became-link = { $path } กลายเป็นลิงก์ในระหว่างที่กำลังเปิด
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = บริการหยุดเองอีกครั้ง
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = คลาส COM นี้มีอยู่บนพีซีเครื่องนี้แล้ว ในฐานะของโปรแกรมอื่น

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = การตรวจสอบหลังการคืนค่า package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = เริ่มบริการ { $name } แล้ว
core-restore-would-copy = จะคัดลอก { $source } -> { $destination }
core-restore-copied = คัดลอก { $source } -> { $destination } แล้ว
core-restore-would-create-empty-folder = จะสร้างโฟลเดอร์ว่าง { $path }
core-restore-created-empty-folder = สร้างโฟลเดอร์ว่าง { $path } แล้ว
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = จะคืนค่ารีจิสทรี { $key } { $value }
core-restore-would-restore-registry-key = จะคืนค่ารีจิสทรี { $key } (คีย์)
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = จะเพิ่มกฎไฟร์วอลล์ { $name } สำหรับ { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = มีกฎไฟร์วอลล์ { $name } อยู่แล้ว
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = เพิ่มกฎไฟร์วอลล์ { $name } สำหรับ { $program } แล้ว
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = จะคืนค่าบริการ { $name } -> { $command } (ทำงานในฐานะ { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = มีส่วนประกอบที่ต้องมีก่อนอยู่แล้ว: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = จะติดตั้งส่วนประกอบที่ต้องมีก่อน { $name } (ฟีเจอร์ของ Windows)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = จะติดตั้งส่วนประกอบที่ต้องมีก่อน { $name } (ตัวติดตั้งที่รวมมาด้วย)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = จะติดตั้งส่วนประกอบที่ต้องมีก่อน { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = ติดตั้งส่วนประกอบที่ต้องมีก่อน { $name } แล้ว
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = ติดตั้งส่วนประกอบที่ต้องมีก่อน { $name } แล้ว (ต้องเริ่มระบบใหม่)
core-restore-would-restore-license = จะคืนค่าสิทธิ์การใช้งาน { $source } -> { $destination }
core-restore-restored-license = คืนค่าสิทธิ์การใช้งาน { $source } -> { $destination } แล้ว
core-restore-would-restore-license-registry = จะคืนค่ารีจิสทรีของสิทธิ์การใช้งาน { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = ไม่ได้คืนค่าไดรเวอร์โดยอัตโนมัติ: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = จะติดตั้งไดรเวอร์ { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = จะคืนค่างานที่จัดกำหนดการไว้ { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = จะคืนค่าทางลัดเริ่มต้นระบบ { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = จะคืนค่าทางลัด { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] จะคืนค่าตัวแปรสภาพแวดล้อมของผู้ใช้ { $name }
       *[machine] จะคืนค่าตัวแปรสภาพแวดล้อมของระบบ { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] คงตัวแปรสภาพแวดล้อมของผู้ใช้ { $name } ไว้ตามเดิม: ไม่มีส่วนใดเป็นของแอปนี้
       *[machine] คงตัวแปรสภาพแวดล้อมของระบบ { $name } ไว้ตามเดิม: ไม่มีส่วนใดเป็นของแอปนี้
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] คืนค่าตัวแปรสภาพแวดล้อมของผู้ใช้ { $name } แล้ว
       *[machine] คืนค่าตัวแปรสภาพแวดล้อมของระบบ { $name } แล้ว
    }
core-restore-set-uninstaller = ตั้งค่าตัวถอนการติดตั้งสำหรับ { $key } แล้ว
core-restore-placed-driver-image = วางอิมเมจไดรเวอร์ { $source } -> { $destination } แล้ว
# { $name } is a driver's name.
core-restore-configured-driver = กำหนดค่าไดรเวอร์ { $name } แล้ว
# { $name } is a driver's name.
core-restore-created-driver = สร้างไดรเวอร์ { $name } แล้ว
# { $name } is a driver's name.
core-restore-started-driver = เริ่มไดรเวอร์ { $name } แล้ว
core-restore-ensured-registry-key = ยืนยันว่ามีคีย์รีจิสทรี { $key } แล้ว
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = คืนค่ารีจิสทรี { $key } { $value } แล้ว
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = คืนค่ารีจิสทรี { $key } (ค่าเริ่มต้น) แล้ว
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = กำหนดค่าบริการ { $name } แล้ว (ทำงานในฐานะ { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = สร้างบริการ { $name } แล้ว (ทำงานในฐานะ { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = คืนค่างานที่จัดกำหนดการไว้ { $name } แล้ว
core-restore-restored-shortcut = คืนค่าทางลัด { $path } แล้ว

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = บันทึกของการติดตั้งที่ไม่เสร็จสมบูรณ์ ถูกปล่อยไว้ตามเดิม เนื่องจากผู้อื่นสามารถเปลี่ยนแปลงบันทึกนั้นได้ สิ่งที่การติดตั้งนั้นเปลี่ยนแปลงไปยังคงอยู่
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
       *[other] การติดตั้ง { $app_name } ครั้งก่อนไม่เสร็จสมบูรณ์ จึงเลิกทำการติดตั้งนั้นก่อน ย้อนกลับการเปลี่ยนแปลง { $count } รายการแล้ว
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = การติดตั้ง { $app_name } ครั้งก่อนไม่เสร็จสมบูรณ์ เมื่อติดตั้ง ระบบจะเลิกทำการติดตั้งนั้นก่อน
core-warning-restore-user-data-real-locations = ข้อมูลผู้ใช้จะถูกคืนค่าไปยังตำแหน่ง %AppData%/%ProgramData% จริงของเครื่องนี้ แทนที่จะไปยังโฟลเดอร์รากเป้าหมาย
# { $error } is what went wrong.
core-warning-restore-commit-failed = ไม่สามารถทำบันทึกการติดตั้งนี้ ให้เสร็จสมบูรณ์ได้: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = มีการสั่งให้บริการ { $name } เริ่มทำงานแล้ว แต่บริการยังเริ่มไม่เสร็จ บริการอาจเริ่มได้ในภายหลัง และจะเริ่มเองเมื่อเริ่มพีซีเครื่องนี้ครั้งถัดไปด้วย
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = ติดตั้งบริการ { $name } แล้ว แต่บริการไม่เริ่มทำงาน: { $reason } แอปอาจทำงานไม่ได้จนกว่าบริการจะเริ่ม Windows จะลองอีกครั้งเมื่อเริ่มพีซีเครื่องนี้ครั้งถัดไป
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = ไม่สามารถหยุดบริการ { $name } ก่อนได้: { $reason } บริการอาจยังคงเรียกใช้โปรแกรมเดิม จนกว่าจะเริ่มพีซีเครื่องนี้ครั้งถัดไป
core-warning-restore-install-cancelled = การติดตั้งถูกยกเลิก
core-warning-restore-preview-cancelled = การแสดงตัวอย่างถูกยกเลิก
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = ไม่รวม { $item } ไว้: { $reason }
# { $error } is what went wrong.
core-warning-restore-verification-failed = ไม่สามารถตรวจสอบหลังการคืนค่าได้: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] { $path } ไม่ใช่ไฟล์ที่แพคเกจนี้แทนที่ได้ และบริการ { $name } ต้องใช้ไฟล์นี้ จึงไม่มีการติดตั้งสิ่งใด: { $refusal }
        [task] { $path } ไม่ใช่ไฟล์ที่แพคเกจนี้แทนที่ได้ และงานที่จัดกำหนดการไว้ { $name } ต้องใช้ไฟล์นี้ จึงไม่มีการติดตั้งสิ่งใด: { $refusal }
        [firewall] { $path } ไม่ใช่ไฟล์ที่แพคเกจนี้แทนที่ได้ และกฎไฟร์วอลล์ { $name } ต้องใช้ไฟล์นี้ จึงไม่มีการติดตั้งสิ่งใด: { $refusal }
        [shortcut] { $path } ไม่ใช่ไฟล์ที่แพคเกจนี้แทนที่ได้ และทางลัด { $name } ต้องใช้ไฟล์นี้ จึงไม่มีการติดตั้งสิ่งใด: { $refusal }
       *[registry] { $path } ไม่ใช่ไฟล์ที่แพคเกจนี้แทนที่ได้ และค่ารีจิสทรี { $name } ต้องใช้ไฟล์นี้ จึงไม่มีการติดตั้งสิ่งใด: { $refusal }
    }
core-warning-restore-destination-exists = มีปลายทางอยู่แล้ว ใช้ --force เพื่อเขียนทับ
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = ไม่สามารถเขียน { $path } ได้: { $error }
core-warning-restore-not-a-folder = มีสิ่งอื่นที่ไม่ใช่โฟลเดอร์ อยู่ในตำแหน่งของโฟลเดอร์ว่างนี้
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = ไม่สามารถตรวจดู { $path } ได้: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = ไม่สามารถสร้าง { $path } ได้: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = พีซีเครื่องนี้มีรายการนี้อยู่แล้ว โดยตั้งค่าเป็นอย่างอื่น ซึ่งการติดตั้งจะไม่เข้าไปแทนที่
core-warning-restore-registry-review = คืนค่ารีจิสทรีจากข้อมูลแบบมีโครงสร้าง ที่เก็บถาวรไว้ โดยเขียนเส้นทางแอปที่รู้จักใหม่ ให้ชี้ไปยังโฟลเดอร์รากเป้าหมาย
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = ไม่สามารถอ่านกฎของไฟร์วอลล์ Windows ได้: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = มีกฎชื่อนี้สำหรับโปรแกรมนี้อยู่แล้ว โดยตั้งค่าเป็นอย่างอื่น ซึ่งการติดตั้งจะไม่เปลี่ยนแปลง
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: ไม่มีตัวติดตั้งที่รู้จักสำหรับรันไทม์นี้
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = การติดตั้ง { $name } ต้องใช้การยกระดับสิทธิ์ โปรดเรียกใช้ตัวติดตั้งอีกครั้งในฐานะผู้ดูแลระบบ
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = ติดตั้ง { $name } แล้ว แต่ต้องเริ่มระบบใหม่เพื่อให้เสร็จสมบูรณ์
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = ไม่สามารถบันทึกรันไทม์ที่ข้ามไปได้: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = ไม่สามารถติดตั้ง { $name } ได้: { $reason } โปรดดาวน์โหลดจาก { $url }
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = ไม่สามารถบันทึกรันไทม์ที่ติดตั้งแล้วได้: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = เส้นทางของสิทธิ์การใช้งานในแพคเกจไม่ถูกต้อง: { $error }
core-warning-restore-license-needs-admin = การคืนค่าข้อมูลสิทธิ์การใช้งาน ไปยังตำแหน่งที่ได้รับการป้องกัน ต้องใช้การยกระดับสิทธิ์
core-warning-restore-license-exists = มีปลายทางของสิทธิ์การใช้งานอยู่แล้ว ใช้ --force เพื่อเขียนทับ
# { $error } is what went wrong.
core-warning-restore-license-failed = ไม่สามารถคืนค่าข้อมูลสิทธิ์การใช้งานได้: { $error }
core-warning-restore-driver-needs-admin = การลงทะเบียนไดรเวอร์ต้องใช้การยกระดับสิทธิ์ โปรดเรียกใช้ตัวติดตั้งอีกครั้งในฐานะผู้ดูแลระบบ
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = งานที่จัดกำหนดการไว้ { $name } ทำงานในฐานะ SYSTEM ซึ่งมีสิทธิ์เต็มบนพีซีเครื่องนี้
core-warning-restore-shortcut-exists = มีทางลัดอยู่แล้ว ใช้ --force เพื่อเขียนทับ
core-warning-restore-shortcut-all-users = ทางลัดที่คืนค่าอย่างน้อยหนึ่งรายการ ไปอยู่ที่เดสก์ท็อปสาธารณะ หรือตำแหน่งเมนูเริ่มที่ใช้ร่วมกัน
core-warning-restore-environment-refresh = การเปลี่ยนแปลงตัวแปรสภาพแวดล้อม อาจต้องใช้กระบวนการ เชลล์ หรือเซสชันการลงชื่อเข้าใช้ใหม่จึงจะเห็นผล
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = บริการ { $name } ทำงานในฐานะ { $account } บนพีซีต้นทาง รหัสผ่านของบัญชีนี้ไม่ได้อยู่ในตัวติดตั้ง จึงตั้งค่าบริการให้ทำงานในฐานะ LocalService หากบริการทำงานไม่ได้ ให้ตั้งค่าบัญชีและรหัสผ่านใน “บริการ”
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } ยังไม่ได้ลงทะเบียน และน่าจะใช้งานได้หลังจากเริ่มระบบใหม่
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } รายงานว่าสำเร็จแต่ไม่ได้ลงทะเบียน การติดตั้งอาจล้มเหลว
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = เส้นทางไดรเวอร์ที่รวมมาไม่ถูกต้อง: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = ไม่สามารถอ่านการตั้งค่าปัจจุบันของไดรเวอร์ได้: { $error }
core-warning-restore-driver-exists = มีบริการไดรเวอร์อยู่แล้ว ใช้ --force เพื่อกำหนดค่าใหม่
core-warning-restore-driver-image-exists = มีอิมเมจไดรเวอร์อยู่แล้ว ใช้ --force เพื่อเขียนทับ
core-warning-restore-driver-hash-bundled = อิมเมจไดรเวอร์ที่รวมมาไม่ตรงกับแฮชที่บันทึกไว้ จึงไม่ได้ติดตั้ง
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = ไม่สามารถวางอิมเมจไดรเวอร์ได้: { $error }
core-warning-restore-driver-hash-placed = อิมเมจไดรเวอร์เปลี่ยนไปหลังจากวางแล้ว จึงไม่ได้ลงทะเบียน
# { $error } is what went wrong.
core-warning-restore-key-not-created = ไม่สามารถสร้างคีย์รีจิสทรีได้: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = ไม่สามารถอ่านค่าที่จะถูกแทนที่ได้: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = ไม่สามารถเขียนค่ารีจิสทรีได้: { $error }
core-warning-restore-value-exists = มีค่ารีจิสทรีอยู่แล้ว ใช้ --force เพื่อเขียนทับ
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = ไม่สามารถสร้างคีย์รีจิสทรี ของตัวแปรสภาพแวดล้อมได้: { $error }
core-warning-restore-variable-exists = มีตัวแปรสภาพแวดล้อมอยู่แล้ว ใช้ --force เพื่อเขียนทับ
# { $error } is what went wrong.
core-warning-restore-variable-not-written = ไม่สามารถเขียนตัวแปรสภาพแวดล้อมได้: { $error }
core-warning-restore-value-kind-missing = ไม่มีชนิดของค่ารีจิสทรี
# { $error } is what went wrong.
core-warning-restore-service-not-read = ไม่สามารถอ่านการตั้งค่าปัจจุบันของบริการได้: { $error }
core-warning-restore-service-exists = มีบริการอยู่แล้ว ใช้ --force เพื่อกำหนดค่าใหม่
# { $error } is what went wrong.
core-warning-restore-task-not-queried = ไม่สามารถสอบถามงานที่จัดกำหนดการไว้ได้: { $error }
core-warning-restore-task-exists = มีงานที่จัดกำหนดการไว้อยู่แล้ว ใช้ --force เพื่อเขียนทับ
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = ไม่สามารถตรวจดูโฟลเดอร์ของงานได้: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = ไม่สามารถสร้างทางลัดได้ และไม่สามารถย้อนกลับทางลัดเดิมที่ถูกแทนที่ได้: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = ไม่สามารถเริ่มคำสั่งได้: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = คำสั่งล้มเหลว: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = ไม่สามารถเริ่ม PowerShell ได้: { $error }
