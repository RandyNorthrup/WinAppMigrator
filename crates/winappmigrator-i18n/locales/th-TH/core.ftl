# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
       *[other] ทางลัด { $count } รายการบนไดรฟ์นี้ไม่ได้ระบุไฟล์ใด จึงไม่สามารถทราบได้ ว่าทางลัดเหล่านั้นเปิดแอปนี้หรือไม่
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = ไม่สามารถอ่านบันทึกของ { $app_name } บางส่วนได้ สิ่งใดที่บันทึกส่วนนั้นระบุไว้จึงยังคงอยู่ และไฟล์ที่การติดตั้งแทนที่ไว้อาจไม่กลับคืนมา
core-rollback-settings-entry-stays = รายการของแอปในการตั้งค่ายังคงอยู่ เพื่อให้สามารถเรียกใช้สิ่งนี้อีกครั้งจากที่นั่นได้
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } ได้รับการติดตั้งแล้วหรืออาจได้รับการติดตั้งแล้ว และจะยังคงอยู่ เนื่องจากโปรแกรมอื่นอาจใช้งานอยู่
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = สิ่งที่การติดตั้งนี้วางไว้ใน { $folder } อยู่บนไดรฟ์ที่ไม่ได้เชื่อมต่ออยู่ โปรดเชื่อมต่อไดรฟ์นั้น เพื่อให้ดำเนินการนี้จนเสร็จได้
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = สิ่งที่การติดตั้งนี้วางไว้ใน { $folder } ยังคงอยู่ เนื่องจากไม่สามารถอ่านบันทึกส่วนนั้นได้

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = มีบางสิ่งกำลังเปลี่ยนแปลง { $folder } ในระหว่างที่ดำเนินการนี้ จึงไม่มีการดำเนินการใดเพิ่มเติมในโฟลเดอร์นั้น โปรดปิดสิ่งที่กำลังทำเช่นนั้น แล้วเรียกใช้อีกครั้ง
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = ไม่สามารถเปิด { $folder } ได้: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = ไม่มีบันทึกว่า { $folder } เคยอยู่ที่ใด
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = ตอนนี้ { $folder } ชี้ไปยัง { $actual } ไม่ใช่ตำแหน่งที่เคยอยู่ระหว่างการติดตั้ง
# Why a service was not stopped.
core-rollback-reason-did-not-stop = ไม่หยุดภายในเวลาที่กำหนด

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = โฟลเดอร์ { $path } ถูกปล่อยไว้: { $reason }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = ลบโฟลเดอร์ { $path } แล้ว
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = โฟลเดอร์ { $path } ยังคงอยู่ เนื่องจากมีไฟล์ที่การติดตั้งนี้ไม่ได้วางไว้
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = ไม่สามารถลบโฟลเดอร์ { $path } ได้: { $error }
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = ไฟล์ { $path } ถูกปล่อยไว้: { $reason }
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = จะไม่ย้อนกลับ { $path } เนื่องจากโฟลเดอร์ที่ไฟล์เคยอยู่ได้หายไปแล้ว
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = ไม่สามารถลบไฟล์ที่เขียนไม่ครบได้: { $path } ({ $error })
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = ไม่สามารถตรวจสอบ { $path } ได้: { $error }
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = ลบ { $path } แล้ว
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = ไม่สามารถลบ { $path } ได้: { $error }
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = ย้อนกลับ { $path } แล้ว
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = ไม่สามารถย้อนกลับ { $path } ได้: { $error }
# Why a file could not be put back.
core-rollback-backup-gone = ไฟล์สำรองหายไปแล้ว
# { $path } is a file.
core-rollback-no-parent-folder = { $path } ไม่มีโฟลเดอร์หลัก

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } ยังคงอยู่ เนื่องจากสิ่งที่ยังคงอยู่ยังใช้ไฟล์นี้
        [changed] { $path } ยังคงอยู่ เนื่องจากมีการเปลี่ยนแปลงหลังจากที่ติดตั้ง
       *[not-asked] { $path } ยังคงอยู่ เนื่องจากไม่ได้ขอให้ลบออก
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } ยังคงอยู่ เนื่องจากสิ่งที่ยังคงอยู่ยังใช้ไฟล์นี้ และไม่สามารถวางไฟล์ที่ถูกแทนที่ไว้ข้างกันได้: ชื่อสำหรับไฟล์นั้นถูกใช้ไปหมดแล้ว
        [changed] { $path } ยังคงอยู่ เนื่องจากมีการเปลี่ยนแปลงหลังจากที่ติดตั้ง และไม่สามารถวางไฟล์ที่ถูกแทนที่ไว้ข้างกันได้: ชื่อสำหรับไฟล์นั้นถูกใช้ไปหมดแล้ว
       *[not-asked] { $path } ยังคงอยู่ เนื่องจากไม่ได้ขอให้ลบออก และไม่สามารถวางไฟล์ที่ถูกแทนที่ไว้ข้างกันได้: ชื่อสำหรับไฟล์นั้นถูกใช้ไปหมดแล้ว
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } ยังคงอยู่ เนื่องจากสิ่งที่ยังคงอยู่ยังใช้ไฟล์นี้ ไฟล์ที่ถูกแทนที่อยู่ข้างกันในชื่อ { $name }
        [changed] { $path } ยังคงอยู่ เนื่องจากมีการเปลี่ยนแปลงหลังจากที่ติดตั้ง ไฟล์ที่ถูกแทนที่อยู่ข้างกันในชื่อ { $name }
       *[not-asked] { $path } ยังคงอยู่ เนื่องจากไม่ได้ขอให้ลบออก ไฟล์ที่ถูกแทนที่อยู่ข้างกันในชื่อ { $name }
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } ยังคงอยู่ เนื่องจากสิ่งที่ยังคงอยู่ยังใช้ไฟล์นี้ และไม่สามารถวางไฟล์ที่ถูกแทนที่ไว้ข้างกันได้: { $error }
        [changed] { $path } ยังคงอยู่ เนื่องจากมีการเปลี่ยนแปลงหลังจากที่ติดตั้ง และไม่สามารถวางไฟล์ที่ถูกแทนที่ไว้ข้างกันได้: { $error }
       *[not-asked] { $path } ยังคงอยู่ เนื่องจากไม่ได้ขอให้ลบออก และไม่สามารถวางไฟล์ที่ถูกแทนที่ไว้ข้างกันได้: { $error }
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (ค่าเริ่มต้น)
core-rollback-setting-other-account =
    { $kind ->
        [key] คีย์ { $name } ยังคงอยู่ เนื่องจากเป็นของบัญชีที่ติดตั้งแอปนี้ ซึ่งไม่ได้ลงชื่อเข้าใช้อยู่
        [user-variable] ตัวแปรสภาพแวดล้อมของผู้ใช้ { $name } ยังคงอยู่ เนื่องจากเป็นของบัญชีที่ติดตั้งแอปนี้ ซึ่งไม่ได้ลงชื่อเข้าใช้อยู่
        [machine-variable] ตัวแปรสภาพแวดล้อมของระบบ { $name } ยังคงอยู่ เนื่องจากเป็นของบัญชีที่ติดตั้งแอปนี้ ซึ่งไม่ได้ลงชื่อเข้าใช้อยู่
        [user-list] { $name } ของผู้ใช้ยังคงอยู่ เนื่องจากเป็นของบัญชีที่ติดตั้งแอปนี้ ซึ่งไม่ได้ลงชื่อเข้าใช้อยู่
        [machine-list] { $name } ของระบบยังคงอยู่ เนื่องจากเป็นของบัญชีที่ติดตั้งแอปนี้ ซึ่งไม่ได้ลงชื่อเข้าใช้อยู่
       *[value] ค่ารีจิสทรี { $name } ยังคงอยู่ เนื่องจากเป็นของบัญชีที่ติดตั้งแอปนี้ ซึ่งไม่ได้ลงชื่อเข้าใช้อยู่
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] ไม่สามารถเข้าถึงคีย์ { $name } ได้: { $error }
        [user-variable] ไม่สามารถเข้าถึงตัวแปรสภาพแวดล้อมของผู้ใช้ { $name } ได้: { $error }
        [machine-variable] ไม่สามารถเข้าถึงตัวแปรสภาพแวดล้อมของระบบ { $name } ได้: { $error }
        [user-list] ไม่สามารถเข้าถึง { $name } ของผู้ใช้ได้: { $error }
        [machine-list] ไม่สามารถเข้าถึง { $name } ของระบบได้: { $error }
       *[value] ไม่สามารถเข้าถึงค่ารีจิสทรี { $name } ได้: { $error }
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] ไม่สามารถอ่านคีย์ { $name } ได้: { $error }
        [user-variable] ไม่สามารถอ่านตัวแปรสภาพแวดล้อมของผู้ใช้ { $name } ได้: { $error }
        [machine-variable] ไม่สามารถอ่านตัวแปรสภาพแวดล้อมของระบบ { $name } ได้: { $error }
        [user-list] ไม่สามารถอ่าน { $name } ของผู้ใช้ได้: { $error }
        [machine-list] ไม่สามารถอ่าน { $name } ของระบบได้: { $error }
       *[value] ไม่สามารถอ่านค่ารีจิสทรี { $name } ได้: { $error }
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] ตัวแปรสภาพแวดล้อมของผู้ใช้ { $name } ยังคงอยู่ เนื่องจากมีการเปลี่ยนแปลงหลังจากที่ติดตั้ง
        [machine-variable] ตัวแปรสภาพแวดล้อมของระบบ { $name } ยังคงอยู่ เนื่องจากมีการเปลี่ยนแปลงหลังจากที่ติดตั้ง
        [user-list] { $name } ของผู้ใช้ยังคงอยู่ เนื่องจากมีการเปลี่ยนแปลงหลังจากที่ติดตั้ง
        [machine-list] { $name } ของระบบยังคงอยู่ เนื่องจากมีการเปลี่ยนแปลงหลังจากที่ติดตั้ง
       *[value] ค่ารีจิสทรี { $name } ยังคงอยู่ เนื่องจากมีการเปลี่ยนแปลงหลังจากที่ติดตั้ง
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] ย้อนกลับตัวแปรสภาพแวดล้อมของผู้ใช้ { $name } แล้ว
        [machine-variable] ย้อนกลับตัวแปรสภาพแวดล้อมของระบบ { $name } แล้ว
        [user-list] ย้อนกลับ { $name } ของผู้ใช้แล้ว
        [machine-list] ย้อนกลับ { $name } ของระบบแล้ว
       *[value] ย้อนกลับค่ารีจิสทรี { $name } แล้ว
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] ไม่สามารถย้อนกลับตัวแปรสภาพแวดล้อมของผู้ใช้ { $name } ได้: { $error }
        [machine-variable] ไม่สามารถย้อนกลับตัวแปรสภาพแวดล้อมของระบบ { $name } ได้: { $error }
        [user-list] ไม่สามารถย้อนกลับ { $name } ของผู้ใช้ได้: { $error }
        [machine-list] ไม่สามารถย้อนกลับ { $name } ของระบบได้: { $error }
       *[value] ไม่สามารถย้อนกลับค่ารีจิสทรี { $name } ได้: { $error }
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = ค่ารีจิสทรี { $name } ยังคงอยู่ เนื่องจากไม่ได้ขอให้ลบออก
core-rollback-key-holds-others = คีย์ { $name } ยังคงอยู่ เนื่องจากมีสิ่งที่การติดตั้งนี้ไม่ได้วางไว้
core-rollback-key-removed = ลบคีย์ { $name } แล้ว
core-rollback-key-not-removed = ไม่สามารถลบคีย์ { $name } ได้: { $error }
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] นำรายการของการติดตั้งนี้ออกจาก { $name } ของผู้ใช้แล้ว
       *[machine-list] นำรายการของการติดตั้งนี้ออกจาก { $name } ของระบบแล้ว
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] ไม่สามารถนำรายการของการติดตั้งนี้ออกจาก { $name } ของผู้ใช้ได้: { $error }
       *[machine-list] ไม่สามารถนำรายการของการติดตั้งนี้ออกจาก { $name } ของระบบได้: { $error }
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] ไม่สามารถอ่านไดรเวอร์ { $name } ได้: { $error }
       *[service] ไม่สามารถอ่านบริการ { $name } ได้: { $error }
    }
core-rollback-service-changed =
    { $kind ->
        [driver] ไดรเวอร์ { $name } ยังคงอยู่ เนื่องจากมีการเปลี่ยนแปลงหลังจากที่ติดตั้ง
       *[service] บริการ { $name } ยังคงอยู่ เนื่องจากมีการเปลี่ยนแปลงหลังจากที่ติดตั้ง
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] ไม่สามารถหยุดไดรเวอร์ { $name } ได้ ({ $reason }) สิ่งที่ไดรเวอร์ใช้อยู่จึงอาจยังคงอยู่ จนกว่าจะเริ่มระบบใหม่
       *[service] ไม่สามารถหยุดบริการ { $name } ได้ ({ $reason }) สิ่งที่บริการใช้อยู่จึงอาจยังคงอยู่ จนกว่าจะเริ่มระบบใหม่
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] ไดรเวอร์ { $name } ถูกหยุดเพื่อดำเนินการนี้ โปรดเริ่มไดรเวอร์อีกครั้งเมื่อคุณต้องการใช้
       *[service] บริการ { $name } ถูกหยุดเพื่อดำเนินการนี้ โปรดเริ่มบริการอีกครั้งเมื่อคุณต้องการใช้
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] ไม่สามารถลบไดรเวอร์ { $name } ได้: { $reason }
       *[service] ไม่สามารถลบบริการ { $name } ได้: { $reason }
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] ไดรเวอร์ { $name } ถูกทำเครื่องหมายให้ลบ และจะถูกลบเมื่อเริ่มระบบใหม่ หรือเมื่อสิ่งที่เปิดไดรเวอร์อยู่ปล่อยไดรเวอร์
       *[service] บริการ { $name } ถูกทำเครื่องหมายให้ลบ และจะถูกลบเมื่อเริ่มระบบใหม่ หรือเมื่อสิ่งที่เปิดบริการอยู่ปล่อยบริการ
    }
core-rollback-service-removed =
    { $kind ->
        [driver] ลบไดรเวอร์ { $name } แล้ว
       *[service] ลบบริการ { $name } แล้ว
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] ไม่สามารถย้อนกลับไดรเวอร์ { $name } ให้เป็นเหมือนเดิมได้: { $reason }
       *[service] ไม่สามารถย้อนกลับบริการ { $name } ให้เป็นเหมือนเดิมได้: { $reason }
    }
core-rollback-service-restored =
    { $kind ->
        [driver] ย้อนกลับการตั้งค่าของไดรเวอร์ { $name } แล้ว
       *[service] ย้อนกลับการตั้งค่าของบริการ { $name } แล้ว
    }
core-rollback-service-not-restarted = บริการ { $name } ทำงานอยู่ก่อนการติดตั้งนี้ และไม่ได้เริ่มทำงานอีกครั้ง: { $reason }
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = บริการ { $name } เคยทำงานในฐานะ { $account } รหัสผ่านของบัญชีนี้ไม่ได้เก็บไว้ โปรดตั้งค่าบัญชีของบริการอีกครั้งใน “บริการ”

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = ไม่สามารถอ่านกฎไฟร์วอลล์ { $name } ได้: { $error }
core-rollback-firewall-says-otherwise = กฎไฟร์วอลล์ { $name } ยังคงอยู่ เนื่องจากกฎที่ใช้ชื่อนี้อยู่ในขณะนี้ ระบุสิ่งที่ต่างออกไป
core-rollback-firewall-changed = กฎไฟร์วอลล์ { $name } ยังคงอยู่ เนื่องจากมีการเปลี่ยนแปลงหลังจากที่ติดตั้ง
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = ลบกฎไฟร์วอลล์ { $name } แล้ว
core-rollback-firewall-not-removed = ไม่สามารถลบกฎไฟร์วอลล์ { $name } ได้: { $error }

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = โฟลเดอร์งาน { $path } ยังคงอยู่ เนื่องจากมีงานที่การติดตั้งนี้ไม่ได้วางไว้
core-rollback-task-folder-not-removed = ไม่สามารถลบโฟลเดอร์งาน { $path } ได้: { $error }
core-rollback-task-not-read = ไม่สามารถอ่านงาน { $name } ได้: { $error }
core-rollback-task-changed = งาน { $name } ยังคงอยู่ เนื่องจากมีการเปลี่ยนแปลงหลังจากที่ติดตั้ง
core-rollback-task-removed = ลบงาน { $name } แล้ว
core-rollback-task-not-removed = ไม่สามารถลบงาน { $name } ได้: { $reason }
core-rollback-task-put-back = ย้อนกลับงาน { $name } แล้ว
core-rollback-task-not-put-back = ไม่สามารถย้อนกลับงาน { $name } ได้: { $reason }

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = ไม่สามารถอ่านบันทึกการติดตั้งนี้ได้ จึงไม่มีการเลิกทำสิ่งใด: { $error }
# { $error } is what went wrong.
core-rollback-nothing-undone = ไม่มีการเลิกทำสิ่งใด: { $error }
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } หากไม่สามารถเลิกทำได้เลย ผู้ดูแลระบบสามารถละทิ้งการติดตั้งนั้นได้โดยลบ { $folder }
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = ไม่สามารถเลิกทำการติดตั้งครั้งก่อน ที่ไม่เสร็จสมบูรณ์ได้ทั้งหมด จึงไม่มีการติดตั้งสิ่งใด โปรดปิดสิ่งที่กำลังใช้รายการเหล่านี้ แล้วลองอีกครั้ง { $details }
core-rollback-install-busy = การติดตั้งนี้กำลังถูกเปลี่ยนแปลงโดยตัวติดตั้ง หรือตัวถอนการติดตั้งอื่น

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = เลิกทำแล้ว: { $count }
core-rollback-count-failed = เลิกทำไม่ได้: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = คงไว้ที่เดิม: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = เหลือค้างไว้: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32 บิต
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64 บิต
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP หรือเก่ากว่า

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = การอ่านไดรฟ์ที่เชื่อมต่อ หรือการตั้งค่าของบัญชีอื่น ต้องใช้สิทธิ์ของผู้ดูแลระบบ
core-system-no-program-data = ไม่พบโฟลเดอร์ ProgramData ของพีซีเครื่องนี้
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = ไม่ใช่ไฟล์ไฮฟ์: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } มีขนาด { $size } ไบต์ ซึ่งใหญ่เกินกว่าจะเป็นไฮฟ์รีจิสทรี หรือใหญ่กว่าขนาดรวมที่ไฮฟ์ของไดรฟ์นี้จะมีได้
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = ไฮฟ์ออฟไลน์ไม่มีการแมปสำหรับ HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } ล้มเหลว: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } ล้มเหลว: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } ไม่ใช่ไดรฟ์ระบบของ Windows (ไม่มีโฟลเดอร์ \Windows)
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } เป็นไดรฟ์ระบบของพีซีเครื่องนี้เอง โปรดเลือก “พีซีเครื่องนี้” เพื่ออ่านพีซีเครื่องนี้
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } ไม่ใช่ไดรฟ์ระบบของ Windows: เส้นทางไปยังรีจิสทรีของไดรฟ์ผ่านลิงก์

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = ไม่มีบันทึกของบัญชี { $account } ที่นี่ จึงไม่ได้รวมข้อมูล หรือการตั้งค่าต่อผู้ใช้ของผู้ใดไว้
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = บัญชีที่ลงชื่อเข้าใช้อยู่ที่นี่ { $account } ไม่มีโปรไฟล์ที่บันทึกไว้ จึงไม่ได้รวมข้อมูล หรือการตั้งค่าต่อผู้ใช้ของผู้ใดไว้
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = ไม่สามารถทราบได้ว่าใครลงชื่อเข้าใช้อยู่ ({ $reason }) จึงไม่ได้รวมข้อมูล หรือการตั้งค่าต่อผู้ใช้ของผู้ใดไว้ โปรดเลือกว่าจะรวมข้อมูลของใคร
core-warning-scan-last-account-unknown = ไดรฟ์นี้ไม่ได้ระบุว่าใครลงชื่อเข้าใช้ล่าสุด และมีบัญชีมากกว่าหนึ่งบัญชี จึงไม่ได้รวมข้อมูล หรือการตั้งค่าต่อผู้ใช้ของผู้ใดไว้ โปรดเลือกว่าจะรวมข้อมูลของใคร
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = ไม่สามารถอ่านการตั้งค่าของ { $account } ได้ จึงไม่ได้รวมไว้: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = ไม่สามารถอ่านคลาส COM และชนิดไฟล์ที่ลงทะเบียนไว้สำหรับ { $account } ได้ จึงไม่ได้รวมไว้: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
       *[other] ไม่สามารถอ่านเส้นทางที่บันทึกไว้บนไดรฟ์นี้ได้ จึงไม่ได้รวมสิ่งที่อยู่ที่นั่นไว้ เส้นทางแต่ละรายการอยู่บนดิสก์อื่น ไม่ใช่เส้นทางแบบเต็ม หรือผ่านลิงก์ซึ่งจะนำออกไปนอกไดรฟ์: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = ไม่สามารถอ่านเส้นทางที่บันทึกไว้บนไดรฟ์นี้ได้ จึงไม่ได้รวมสิ่งที่อยู่ที่นั่นไว้ เส้นทางแต่ละรายการอยู่บนดิสก์อื่น ไม่ใช่เส้นทางแบบเต็ม หรือผ่านลิงก์ซึ่งจะนำออกไปนอกไดรฟ์: { $paths } และอีก { $more } รายการ
core-warning-scan-network-paths-not-followed =
    { $count ->
       *[other] ไม่สามารถอ่านเส้นทางที่บันทึกไว้ บนพีซีเครื่องนี้ได้ จึงไม่ได้รวมสิ่งที่อยู่ที่นั่นไว้ เส้นทางแต่ละรายการ ไม่ได้อยู่บนดิสก์ของพีซีเครื่องนี้ ซึ่งการตรวจดูจะทำให้เกิดการเชื่อมต่อ ไปยังเส้นทางนั้น หรือมีชื่อที่จะเปิดสิ่งอื่น: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = ไม่สามารถอ่านเส้นทางที่บันทึกไว้ บนพีซีเครื่องนี้ได้ จึงไม่ได้รวมสิ่งที่อยู่ที่นั่นไว้ เส้นทางแต่ละรายการ ไม่ได้อยู่บนดิสก์ของพีซีเครื่องนี้ ซึ่งการตรวจดูจะทำให้เกิดการเชื่อมต่อ ไปยังเส้นทางนั้น หรือมีชื่อที่จะเปิดสิ่งอื่น: { $paths } และอีก { $more } รายการ

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = ทางลัดในโฟลเดอร์ของบัญชีนี้เอง
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
       *[other] ทางลัด { $count } รายการ ซึ่งจะรวมไว้เฉพาะเมื่อเลือกตัวเลือกข้อมูลผู้ใช้
    }
core-account-left-out-variables = ตัวแปรสภาพแวดล้อมของบัญชีนี้
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
       *[other] ตัวแปร { $count } รายการ ซึ่งจะรวมไว้เฉพาะเมื่อเลือกตัวเลือกข้อมูลผู้ใช้
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = งานที่จัดกำหนดการไว้ { $task }
core-account-task-works-on-account-folder = งานนี้ทำงานกับตำแหน่งในโฟลเดอร์ของบัญชี หรือตั้งชื่อตามตำแหน่งนั้น และจะรวมไว้เฉพาะเมื่อเลือกตัวเลือกข้อมูลผู้ใช้
core-account-task-runs-from-account-folder = สิ่งที่งานนี้เรียกใช้จากโฟลเดอร์ของบัญชีด้วย จะรวมไว้เฉพาะเมื่อเลือกตัวเลือกข้อมูลผู้ใช้
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = บริการ { $service }
core-account-service-works-on-account-folder = บริการนี้ทำงานกับตำแหน่งในโฟลเดอร์ของบัญชี และจะรวมไว้เฉพาะเมื่อเลือกตัวเลือกข้อมูลผู้ใช้

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = เส้นทางของไฟล์ของแอปเอง
core-account-part-registry = ค่ารีจิสทรี
core-account-part-settings-entry = รายการของแอปในการตั้งค่า
core-account-part-main-program = ตำแหน่งของโปรแกรมหลัก
core-account-part-shortcuts = ทางลัด
core-account-part-tasks = งานที่จัดกำหนดการไว้
core-account-part-services = บริการ
core-account-part-variables = ตัวแปรสภาพแวดล้อม
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] แพคเกจนี้ระบุชื่อบัญชีไว้ใน { $places } แพคเกจนี้สร้างขึ้นโดยไม่มีข้อมูลของบัญชีใด และทุกคนที่มีตัวติดตั้งสามารถอ่านข้อมูลนี้ได้
        [folder] แพคเกจนี้ระบุตำแหน่งโฟลเดอร์ของบัญชีไว้ใน { $places } แพคเกจนี้สร้างขึ้นโดยไม่มีข้อมูลของบัญชีใด และทุกคนที่มีตัวติดตั้งสามารถอ่านข้อมูลนี้ได้
        [computer] แพคเกจนี้ระบุชื่อพีซีไว้ใน { $places } แพคเกจนี้สร้างขึ้นโดยไม่มีข้อมูลของบัญชีใด และทุกคนที่มีตัวติดตั้งสามารถอ่านข้อมูลนี้ได้
       *[id] แพคเกจนี้ระบุรหัสของบัญชีไว้ใน { $places } แพคเกจนี้สร้างขึ้นโดยไม่มีข้อมูลของบัญชีใด และทุกคนที่มีตัวติดตั้งสามารถอ่านข้อมูลนี้ได้
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
       *[other] ไม่สามารถอ่านโฟลเดอร์ { $count } โฟลเดอร์ที่อยู่นอกแอปได้ ข้อมูลของแอปที่อยู่ในนั้นจึงอาจขาดหายไป
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = ดูเหมือนข้อมูลประจำตัว
core-secret-why-license-option-off = ตัวเลือก “ข้อมูลสิทธิ์การใช้งานและการเปิดใช้งาน” ปิดอยู่
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
       *[other] ค่า { $count } รายการ ซึ่งจะรวมไว้เฉพาะเมื่อเลือกตัวเลือกข้อมูลผู้ใช้
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (ค่าที่ชื่อมีข้อมูลประจำตัว)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = อาร์กิวเมนต์ที่มีข้อมูลประจำตัว
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } ในคำสั่งที่ใช้เรียกใช้ { $name }
        [task] { $option } ในคำสั่งที่ใช้เรียกใช้งาน { $name }
        [shortcut] { $option } ในทางลัด { $name }
        [opener] { $option } ในสิ่งที่ใช้เปิด { $name }
        [opener-unnamed] { $option } ในสิ่งที่ใช้เปิดชนิดไฟล์หนึ่ง
       *[uninstall] { $option } ในคำสั่งที่ใช้ถอนการติดตั้ง { $name }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = คำจำกัดความของงาน { $task } ซึ่งไม่สามารถอ่านเพื่อตรวจสอบได้
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = ตัวแปรสภาพแวดล้อม { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = สิ่งที่คลาส { $class } ระบุว่าเรียกใช้
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = คำอธิบายของกฎไฟร์วอลล์ { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = ตรวจพบไฟล์สิทธิ์การใช้งาน: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = มีที่เก็บสิทธิ์การใช้งานของ { $framework }
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = ตัวแปรสภาพแวดล้อมของสิทธิ์การใช้งาน { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = ค่ารีจิสทรีบ่งชี้ว่าเป็นข้อมูลสิทธิ์การใช้งาน: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = ค่ารีจิสทรีบ่งชี้ว่าเป็นข้อมูลสิทธิ์การใช้งาน
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = คีย์รีจิสทรีบ่งชี้ว่าเป็นข้อมูลสิทธิ์การใช้งาน: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = ตรวจพบข้อมูลสิทธิ์การใช้งาน/การเปิดใช้งาน เมื่อเปิดตัวเลือก “ข้อมูลสิทธิ์การใช้งานและการเปิดใช้งาน” ตัวติดตั้งจะคืนค่าสิ่งที่ย้ายได้ ส่วนค่าที่ใช้ทั้งเครื่อง และที่เก็บที่ผูกกับฮาร์ดแวร์ จะได้รับการรายงานเท่านั้น WinAppMigrator ไม่ถอดรหัส ไม่ข้ามการเปิดใช้งาน และไม่จำลองดองเกิล
core-warning-scan-license-machine-bound = สิทธิ์การใช้งานบางรายการผูกกับเครื่อง (ดองเกิลฮาร์ดแวร์ การเปิดใช้งานที่ล็อกกับเครื่อง หรือเข้ารหัสไว้) ตัวติดตั้งจะคืนค่าเท่าที่ทำได้ แต่สิทธิ์การใช้งานเหล่านี้ อาจยังต้องเปิดใช้งานใหม่บนพีซีเครื่องใหม่

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = ไม่สามารถอ่านคีย์ย่อยของ App Paths ได้: { $error }
core-warning-scan-registry-limit = การเก็บรีจิสทรีถึงขีดจำกัดความปลอดภัย ที่กำหนดไว้แล้ว
core-warning-scan-registry-subtree-limit = การเก็บทรีย่อยของรีจิสทรี ถึงขีดจำกัดความปลอดภัยที่กำหนดไว้แล้ว
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = ไม่สามารถอ่านคีย์รีจิสทรีได้: { $error }
core-warning-scan-registry-value-too-large = ค่ารีจิสทรีเกินขีดจำกัดขนาดการเก็บที่กำหนดไว้

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = โฟลเดอร์ของงานที่จัดกำหนดการไว้บนไดรฟ์นี้ ผ่านลิงก์ จึงไม่ได้อ่านงานในโฟลเดอร์นั้น
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = งานที่จัดกำหนดการไว้ { $task } ไม่ได้รวมไว้: งานนี้ทำงานในฐานะบัญชีอื่น ที่ไม่ใช่บัญชีที่กำลังรวมการตั้งค่า ผู้อื่นที่ไม่ใช่ผู้ดูแลระบบ สามารถเปลี่ยนแปลงไฟล์ของงานได้ หรือแอปมีงานมากเกินกว่าจะเก็บไว้ได้
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = ไม่สามารถอ่านงานที่จัดกำหนดการไว้บางส่วนได้: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = ไม่สามารถอ่านงานที่จัดกำหนดการไว้ { $path } ได้: { $reason } หากงานนี้เรียกใช้แอป งานนี้จะไม่ได้รวมไว้
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = มีขนาด { $size } ไบต์ ซึ่งใหญ่เกินกว่าจะเป็นงาน
# Why a task's file could not be read.
core-task-ends-mid-character = สิ้นสุดกลางอักขระ
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = ไม่ใช่ UTF-16 ที่ถูกต้อง: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = ไม่ใช่ UTF-8 หรือ UTF-16 ที่ถูกต้อง: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = การคืนค่าใช้ Register-ScheduledTask กับคำจำกัดความ XML ที่เก็บถาวรไว้

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = ไม่สามารถอ่านได้: { $reason }
core-pe-headers-misaligned = ส่วนหัวไม่ได้อยู่ในขอบเขตที่โปรแกรมต้องมี
# { $size } is the file's size in bytes.
core-pe-too-large = มีขนาด { $size } ไบต์ ซึ่งใหญ่เกินกว่าจะเป็นโปรแกรม
# PE is the name of the format Windows programs are in.
core-pe-no-headers = ไม่มีส่วนหัว PE

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = ไม่สามารถอ่านตารางการนำเข้าได้: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = ไม่สามารถอ่านชื่อในตัวอธิบายการนำเข้าได้: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
       *[other] ข้ามตัวอธิบายการโหลดแบบหน่วงเวลารุ่นเก่า (v1) { $count } รายการ จะแยกวิเคราะห์เฉพาะการนำเข้าแบบหน่วงเวลา รุ่นใหม่ที่ใช้ RVA เท่านั้น
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = ไฟล์ปฏิบัติการไม่มีไดเรกทอรีหลัก: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL อยู่ข้างไฟล์ปฏิบัติการที่เลือก
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = ไม่สามารถสแกนส่วนประกอบที่ต้องใช้ แบบเรียกซ้ำได้: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } อยู่ในโฟลเดอร์ Windows ของพีซีที่แอปนี้เคยอยู่ แต่ Windows 10 และ 11 ไม่มีไฟล์นี้มาให้ มีบางสิ่งติดตั้งไฟล์นี้ไว้ที่นั่น ไฟล์นี้จึงไม่ได้เป็นส่วนหนึ่งของไฟล์ของแอปเอง และจะไม่ถูกนำไปด้วย แอปอาจเริ่มทำงานไม่ได้ จนกว่าจะติดตั้งสิ่งที่จัดหาไฟล์นี้
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = ไม่สามารถคำนวณแฮช ของส่วนประกอบที่ต้องใช้ได้: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = ไม่มีตัวแปรสภาพแวดล้อม SystemRoot/windir การค้นหา DLL ของระบบจึงมีข้อจำกัด
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = ไม่สามารถอ่านสถาปัตยกรรม ของส่วนประกอบที่ต้องใช้ได้: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = ไม่สามารถอ่านคีย์รีจิสทรีของบริการได้: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = ไม่สามารถอ่านคีย์รีจิสทรีของบริการ เพื่อค้นหาไดรเวอร์ได้: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = ไม่สามารถสอบถามทางลัดได้: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = ไม่สามารถแยกวิเคราะห์ ผลลัพธ์การสอบถามทางลัดได้: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = ไม่สามารถอ่านคีย์รีจิสทรี ของตัวแปรสภาพแวดล้อมของเครื่องได้: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] ไดรเวอร์ { $kind } “{ $name }” ที่เป็นของแอปพลิเคชัน ({ $path }, ลงนามโดย { $signer }) ตัวติดตั้งจะรวมไฟล์ .sys ไว้และลงทะเบียนบริการไดรเวอร์ใหม่โดยอัตโนมัติ (sc create ต้องใช้การยกระดับสิทธิ์) จากนั้นจะยืนยันว่าบริการลงทะเบียนแล้ว
       *[no] ไดรเวอร์ { $kind } “{ $name }” ที่เป็นของแอปพลิเคชัน ({ $path }, ไม่ทราบลายเซ็น) ตัวติดตั้งจะรวมไฟล์ .sys ไว้และลงทะเบียนบริการไดรเวอร์ใหม่โดยอัตโนมัติ (sc create ต้องใช้การยกระดับสิทธิ์) จากนั้นจะยืนยันว่าบริการลงทะเบียนแล้ว
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] ไดรเวอร์ { $kind } “{ $name }” ของผู้จำหน่าย/ที่มาพร้อมกับ Windows ({ $path }, ลงนามโดย { $signer }) WinAppMigrator ไม่แจกจ่ายไดรเวอร์ของบุคคลภายนอก โปรดติดตั้งไดรเวอร์นี้ใหม่บนเครื่องเป้าหมาย ด้วยแพคเกจไดรเวอร์ที่ลงนามแล้วของผู้จำหน่าย ก่อนเปิดแอป
       *[no] ไดรเวอร์ { $kind } “{ $name }” ของผู้จำหน่าย/ที่มาพร้อมกับ Windows ({ $path }, ไม่ทราบลายเซ็น) WinAppMigrator ไม่แจกจ่ายไดรเวอร์ของบุคคลภายนอก โปรดติดตั้งไดรเวอร์นี้ใหม่บนเครื่องเป้าหมาย ด้วยแพคเกจไดรเวอร์ที่ลงนามแล้วของผู้จำหน่าย ก่อนเปิดแอป
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = มีอยู่ใน Windows
core-runtime-handling-ships-with-app = มาพร้อมกับแอป
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = ฟีเจอร์ของ Windows: { $feature }
core-runtime-handling-included = รวมไว้แล้ว
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = ติดตั้งด้วย winget: { $id }
core-runtime-handling-manual = ติดตั้งด้วยตนเอง

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = ปฏิเสธการดาวน์โหลด { $url }: ไม่ใช่ HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = การเริ่มต้น TLS ล้มเหลว: { $error }
core-runtime-download-failed = การดาวน์โหลด { $url } ล้มเหลว: { $error }
core-runtime-download-unreadable = การอ่าน { $url } ล้มเหลว: { $error }
core-runtime-download-empty = การดาวน์โหลด { $url } ไม่ได้ส่งข้อมูลกลับมา
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = การดาวน์โหลด { $url } ถูกตัดกลางคัน: ได้รับ { $came } จาก { $expected } ไบต์

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = ไม่มีตัวติดตั้งที่รู้จักสำหรับรันไทม์นี้
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = แพคเกจระบุ { $path } เป็นตัวติดตั้งของรันไทม์นี้ ซึ่งไม่ได้อยู่ในตำแหน่งที่ควรอยู่ จึงไม่ได้เรียกใช้
# { $path } is a file.
core-runtime-payload-missing = ไม่มีตัวติดตั้งที่รวมมา: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = เส้นทางของตัวติดตั้งที่รวมมาไม่ถูกต้อง: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = ไม่มีตัวติดตั้งที่รวมมาหรือรหัส winget โปรดดาวน์โหลดจาก { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = ไม่สามารถสร้างโฟลเดอร์สำหรับแตกไฟล์ได้: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = ตัวติดตั้งที่รวมมาไม่ผ่านการตรวจสอบลายเซ็น และไม่ได้เรียกใช้: { $reason }
core-runtime-payload-timed-out = ตัวติดตั้งที่รวมมาทำงานไม่เสร็จภายในเวลาที่กำหนด และถูกยุติ
# { $error } is what went wrong.
core-runtime-payload-not-started = ไม่สามารถเริ่มตัวติดตั้งที่รวมมาได้: { $error }
core-runtime-installer-killed = ตัวติดตั้งถูกยุติโดยไม่มีรหัสออก
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = ตัวติดตั้งออกด้วยรหัส { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM ไม่สามารถเปิดใช้งานฟีเจอร์ของ Windows { $feature } ได้ (รหัสออก { $code }) อาจดาวน์โหลดข้อมูลของฟีเจอร์ไม่ได้ โปรดเปิดใช้งานผ่าน “ฟีเจอร์ของ Windows” หรือเรียกใช้ `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM ไม่สามารถเปิดใช้งานฟีเจอร์ของ Windows { $feature } ได้ (ไม่ทราบรหัสออก) อาจดาวน์โหลดข้อมูลของฟีเจอร์ไม่ได้ โปรดเปิดใช้งานผ่าน “ฟีเจอร์ของ Windows” หรือเรียกใช้ `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = การเปิดใช้งานฟีเจอร์ของ Windows { $feature } ไม่เสร็จภายในเวลาที่กำหนดและถูกยุติ
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = ไม่มี DISM สำหรับเปิดใช้งานฟีเจอร์ของ Windows { $feature }: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = ไม่สามารถค้นหา winget ได้: { $error }
core-runtime-winget-not-installed = ไม่ได้ติดตั้ง winget
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } ไม่ได้อยู่ในตำแหน่งที่ Windows เก็บแพคเกจของตนเอง จึงไม่ได้เรียกใช้
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } ไม่ผ่านการตรวจสอบลายเซ็นและไม่ได้เรียกใช้: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = ไม่สามารถใช้ winget เพื่อติดตั้ง { $id } ได้: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } ออกด้วยรหัส { $code }
core-runtime-winget-failed-no-code = winget install { $id } ออกด้วยรหัสที่ไม่ทราบ
core-runtime-winget-timed-out = winget install { $id } ทำงานไม่เสร็จภายในเวลาที่กำหนดและถูกยุติ
# { $error } is what went wrong.
core-runtime-winget-not-started = ไม่มี winget สำหรับติดตั้ง { $id }: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = ไม่ได้ใช้ { $path }: ผู้อื่นที่ไม่ใช่ผู้ดูแลระบบ สามารถเปลี่ยนแปลงไฟล์นี้ได้
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
       *[other] บันเดิล Burn ที่มีแพคเกจ Windows Installer { $count } แพคเกจ
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
       *[other] ฐานข้อมูล Windows Installer: เส้นทางคีย์ของคอมโพเนนต์ { $count } รายการ
    }
core-source-inno-log = บันทึกการถอนการติดตั้งของ Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
       *[other] บันทึกการถอนการติดตั้งของ Inno Setup: เส้นทางที่บันทึกไว้ { $count } รายการ
    }
core-source-nsis = ลายเซ็นตัวถอนการติดตั้งของ NSIS (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
       *[other] NSIS AdvUninstLog uninstall.log: เส้นทางที่บันทึกไว้ { $count } รายการ
    }
core-source-squirrel = โครงสร้างต่อผู้ใช้ของ Squirrel (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = เวอร์ชันปัจจุบันของ Squirrel: { $version }
core-source-chocolatey = เมตาดาต้าของแพคเกจ Chocolatey
core-source-msix = แพคเกจ MSIX / Appx
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = ข้อมูลประจำตัวใน AppxManifest: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
       *[other] ข้อมูลแพคเกจ MSIX ต่อผู้ใช้: { $count } ตำแหน่ง
    }
# App Paths is a registry key's name.
core-source-app-paths = การลงทะเบียน App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } นำเข้า { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = ฟอนต์ที่บันทึกการติดตั้งของแอประบุไว้
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = ตั้งชื่อตามแอปหรือผู้เผยแพร่แอป
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = บันทึกการติดตั้งของแอประบุถึงใบรับรองนี้

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows ไม่ได้ระบุว่าเดสก์ท็อปนี้เป็นของใคร: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = ไม่สามารถสอบถาม Windows ได้ว่าเดสก์ท็อปนี้เป็นของใคร: { $error }
core-profile-signed-in-account-no-profile = บัญชีที่ลงชื่อเข้าใช้อยู่ที่นี่ { $account } ไม่มีโปรไฟล์ที่บันทึกไว้
core-profile-desktop-not-a-person = เดสก์ท็อปที่นี่เป็นของ { $account } ซึ่งไม่ใช่บัญชีของบุคคล
core-profile-nobody-signed-in-running-account-no-profile = ไม่มีผู้ใดลงชื่อเข้าใช้ที่นี่ และบัญชีที่เรียกใช้โปรแกรมนี้ { $account } ไม่มีโปรไฟล์ที่บันทึกไว้
core-profile-nobody-signed-in-running-not-a-person = ไม่มีผู้ใดลงชื่อเข้าใช้ที่นี่ และ { $account } ซึ่งเรียกใช้โปรแกรมนี้ไม่ใช่บัญชีของบุคคล
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = ไม่มีผู้ใดลงชื่อเข้าใช้ที่นี่ และบัญชีนี้ไม่มี SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = มีบัญชีมากกว่าหนึ่งบัญชี ที่มีเดสก์ท็อปในเซสชันนี้: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = ไม่สามารถทราบได้ว่าใครลงชื่อเข้าใช้อยู่ ({ $reason })
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = ไม่สามารถทราบได้ว่าใครลงชื่อเข้าใช้อยู่ ({ $reason }) โปรดเลือกว่าจะรวมข้อมูลของใคร
