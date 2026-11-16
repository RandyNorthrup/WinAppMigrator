# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = เป็นตำแหน่งที่การติดตั้งเก็บบันทึกไว้
core-review-location-has-files = มีไฟล์อยู่แล้วและไม่มีบันทึกการติดตั้งที่นี่ โปรดใช้โฟลเดอร์ว่าง
core-policy-location-not-full-path = ต้องเป็นเส้นทางแบบเต็ม
core-policy-location-parent-step = ต้องไม่มี “..”
core-policy-location-not-plain = ต้องเป็นเส้นทางธรรมดาบนไดรฟ์ของพีซีเครื่องนี้
core-policy-location-whole-drive = ต้องไม่ใช่ทั้งไดรฟ์
core-policy-location-windows = ต้องไม่ใช่โฟลเดอร์ Windows
core-policy-location-runs-by-itself = เป็นโฟลเดอร์ที่เป็นของ Windows หรือเป็นโฟลเดอร์ ที่สิ่งที่อยู่ภายในจะทำงานหรือโหลดได้เอง
core-policy-location-shared = มีสิ่งอื่นอยู่ร่วมกันในโฟลเดอร์นี้ แอปจึงต้องใช้โฟลเดอร์ย่อยภายในโฟลเดอร์นี้

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = แอปนี้ไม่สามารถทำงานบนพีซีเครื่องนี้ได้ จึงไม่มีการเปลี่ยนแปลงสิ่งใด: { $reason }
core-policy-architecture-x86 = x86 32 บิต
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = Arm 32 บิต
core-policy-architecture-unknown = สถาปัตยกรรมที่ไม่รู้จัก
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = ชนิดเครื่อง { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names } และอีก { $count } รายการ
core-policy-fit-programs-not-said = ตัวติดตั้งนี้ไม่ได้ระบุว่าโปรแกรมต้องการอะไร จึงไม่ได้ตรวจสอบ ว่าโปรแกรมทำงานบนพีซีเครื่องนี้ได้หรือไม่
core-policy-fit-architecture-not-told = ไม่สามารถทราบได้ ว่าพีซีเครื่องนี้ใช้สถาปัตยกรรมใด จึงไม่ได้ตรวจสอบว่าโปรแกรมของแอปนี้ ทำงานบนพีซีเครื่องนี้ได้หรือไม่
core-policy-fit-main-not-said = ตัวติดตั้งนี้ไม่ได้ระบุว่าโปรแกรมหลักต้องการอะไร ข้อมูลที่แสดงที่นี่ จึงอิงจากส่วนที่เหลือของตัวติดตั้ง
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = ไดรเวอร์ { $driver } ของแอปสร้างขึ้นสำหรับ { $built_for } แต่พีซีเครื่องนี้เป็น { $here } ไดรเวอร์ต้องตรงกับ Windows ทุกประการ แอปนี้จึงทำงานที่นี่ไม่ได้
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } สร้างขึ้นสำหรับ { $built_for } แต่พีซีเครื่องนี้เป็น { $here } Windows จึงจะไม่เริ่มโปรแกรมนี้
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = พีซีเครื่องนี้เป็น { $here } และไม่มีโปรแกรมใดของแอปที่เริ่มทำงานได้: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = โปรแกรมบางรายการของแอปนี้ สร้างขึ้นสำหรับสถาปัตยกรรมอื่น และจะไม่เริ่มทำงานบนพีซีเครื่องนี้ซึ่งเป็น { $here }: { $programs } ส่วนที่เหลือของแอปเริ่มทำงานได้ จึงอาจไม่เป็นปัญหา
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = พีซีเครื่องนี้เป็น { $here } Windows จึงจะเรียกใช้โปรแกรมเหล่านี้ผ่านตัวจำลอง: { $programs } โปรแกรมส่วนใหญ่ทำงานได้ดีด้วยวิธีนี้ แต่บางโปรแกรมไม่ได้
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } ระบุว่าต้องใช้ Windows { $needs } ขึ้นไป แต่พีซีเครื่องนี้ใช้ { $runs } Windows จึงจะไม่เริ่มโปรแกรมนี้
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = โปรแกรมบางรายการของแอปนี้ระบุว่าต้องใช้ Windows ที่ใหม่กว่านี้ และจะไม่เริ่มทำงานที่นี่: { $programs }
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = แอปนี้เคยอยู่บน Windows บิลด์ { $from_build } แต่พีซีเครื่องนี้ใช้บิลด์ { $this_build } โปรแกรมของแอประบุว่าเริ่มทำงานบน Windows นี้ได้ จึงอาจใช้งานได้ แต่สิ่งที่แอปเคยพึ่งพาบนเครื่องเดิมอาจไม่มีอยู่ที่นี่
core-policy-fit-build-not-told = ไม่สามารถทราบได้ว่านี่คือ Windows บิลด์ใด จึงไม่ได้เปรียบเทียบกับบิลด์ที่แอปนี้เคยอยู่

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = การแตกไฟล์แพคเกจ
core-space-for-install-record = บันทึกการติดตั้ง
core-space-for-app-files = ไฟล์ของแอป
core-space-for-what-is-replaced = การเก็บสิ่งที่ถูกแทนที่
core-space-for-build-work = ไฟล์ที่ใช้ระหว่างการสร้าง
core-space-for-installer = ตัวติดตั้ง
core-space-for-package = แพคเกจ
core-space-for-unpacking-and-installing = การแตกไฟล์แพคเกจและติดตั้ง
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " และ" }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } ต้องมีพื้นที่ว่างสูงสุด { $needed } สำหรับ{ $purposes } แต่มี { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } ไม่ได้แจ้งว่ามีพื้นที่เท่าใด จึงไม่ได้ตรวจสอบว่าพื้นที่ { $needed } ที่ต้องใช้จะเพียงพอหรือไม่
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = สูงสุด { $needed } บน { $drive } (ว่าง { $free })
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = สูงสุด { $needed } บน { $drive } (ไม่ทราบพื้นที่ว่าง)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { " " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = พื้นที่ไม่เพียงพอ จึงไม่มีการเปลี่ยนแปลงสิ่งใด { $shortfalls } โปรดเพิ่มพื้นที่ว่างที่นั่น หรือเลือกตำแหน่งที่ติดตั้งบนไดรฟ์อื่น

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = บริการ
core-review-kind-driver = ไดรเวอร์
core-review-kind-scheduled-task = งานที่จัดกำหนดการไว้
core-review-kind-autostart = เริ่มทำงานโดยอัตโนมัติ
core-review-kind-shortcut = ทางลัด
core-review-kind-path = PATH
core-review-kind-environment-variable = ตัวแปรสภาพแวดล้อม
core-review-kind-shell-extension = ส่วนขยายเชลล์
core-review-kind-menu-command = คำสั่งเมนู
core-review-kind-link-type = ชนิดลิงก์
core-review-kind-file-type = ชนิดไฟล์
core-review-kind-com = COM
core-review-kind-firewall-rule = กฎไฟร์วอลล์
core-review-kind-event-log-source = แหล่งที่มาของบันทึกเหตุการณ์
core-review-kind-font = ฟอนต์
core-review-kind-odbc-driver = ไดรเวอร์ ODBC
core-review-kind-odbc-source = แหล่งข้อมูล ODBC
core-review-kind-compatibility-setting = การตั้งค่าความเข้ากันได้
core-review-kind-default-apps = แอปเริ่มต้น
core-review-kind-program-name = ชื่อโปรแกรม
core-review-kind-other-settings = การตั้งค่าอื่นๆ

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] เรียกใช้ { $file } ในฐานะ { $account } เริ่มทำงานทันที และทุกครั้งที่พีซีเครื่องนี้เริ่มทำงาน
        [delayed] เรียกใช้ { $file } ในฐานะ { $account } เริ่มทำงานทันที และไม่นานหลังจากพีซีเครื่องนี้เริ่มทำงานในแต่ละครั้ง
        [manual] เรียกใช้ { $file } ในฐานะ { $account } เริ่มทำงานเฉพาะเมื่อมีสิ่งใดร้องขอ
        [disabled] เรียกใช้ { $file } ในฐานะ { $account } ถูกปิดใช้งานอยู่
        [other] เรียกใช้ { $file } ในฐานะ { $account } { $start }
       *[unknown] เรียกใช้ { $file } ในฐานะ { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (ไม่ได้คืนค่าบัญชีที่กำหนดเอง)
# { $file } is a driver's file name.
core-review-driver-detail = โหลด { $file } เข้าไปใน Windows โดยตรง
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = โหลด { $file } เข้าไปใน Windows โดยตรง { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = เริ่มทำงานพร้อมกับ Windows ก่อนสิ่งอื่นใด
core-review-start-system = เริ่มทำงานพร้อมกับ Windows ขณะที่ Windows กำลังเริ่มต้น
core-review-start-automatic = เริ่มทำงาน ทุกครั้งที่พีซีเครื่องนี้เริ่มทำงาน
core-review-start-delayed = เริ่มทำงานไม่นาน หลังจากพีซีเครื่องนี้เริ่มทำงานในแต่ละครั้ง
core-review-start-manual = เริ่มทำงานเฉพาะเมื่อมีสิ่งใดร้องขอ
core-review-start-disabled = ไม่เริ่มทำงานเลย เนื่องจากถูกปิดใช้งาน
core-review-start-unknown = เริ่มทำงานในแบบที่ Windows ไม่ได้ระบุ
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = ทำงานในฐานะ { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = ทำงานในฐานะ { $account } ด้วยสิทธิ์สูงสุดที่มีได้
core-review-task-runs-as-you = คุณ ซึ่งคือผู้ที่ลงชื่อเข้าใช้อยู่
core-review-task-runs-as-creator = ผู้ใช้ที่สร้างงานนี้
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    ทำงาน{ $scope ->
        [everyone] สำหรับทุกคนที่ลงชื่อเข้าใช้
       *[you] เมื่อคุณลงชื่อเข้าใช้
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    ทางลัดในโฟลเดอร์ Startup { $scope ->
        [everyone] สำหรับทุกคนบนพีซีเครื่องนี้
       *[you] สำหรับคุณ
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    เปิด { $file } { $scope ->
        [everyone] สำหรับทุกคนบนพีซีเครื่องนี้
       *[you] สำหรับคุณ
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = เพิ่มลงใน { $name }
core-review-path-install-folder = โฟลเดอร์ที่ติดตั้ง
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = โฟลเดอร์ที่ติดตั้ง\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = ตั้งค่าเป็น { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
       *[other] การตั้งค่า { $count } รายการนอกคีย์ของแอปเอง
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = ใน { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = ใน { $keys } และอีก { $count } รายการ
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = โดเมน
core-review-network-private = ส่วนตัว
core-review-network-public = สาธารณะ
core-review-firewall-detail =
    { $way ->
        [allow-in] อนุญาตการเชื่อมต่อขาเข้า
        [allow-out] อนุญาตการเชื่อมต่อขาออก
        [block-in] บล็อกการเชื่อมต่อขาเข้า
       *[block-out] บล็อกการเชื่อมต่อขาออก
    }{ $port_count ->
        [0] { "" }
       *[other] { " " }บนพอร์ต { $ports }
    } สำหรับ { $program } บน{ $network_count ->
        [0] ทุกเครือข่าย
       *[other] เครือข่าย{ $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    โหลด { $server } เข้าไปในตัวสำรวจไฟล์ { $scope ->
        [everyone] สำหรับทุกคนบนพีซีเครื่องนี้
       *[you] สำหรับคุณ
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    โหลดไฟล์ที่แอปติดตั้งเข้าไปในตัวสำรวจไฟล์ { $scope ->
        [everyone] สำหรับทุกคนบนพีซีเครื่องนี้
       *[you] สำหรับคุณ
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } บน { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    เพิ่มคำสั่งลงในเมนูของ { $on } { $scope ->
        [everyone] สำหรับทุกคนบนพีซีเครื่องนี้
       *[you] สำหรับคุณ
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    เปลี่ยนสิ่งที่เกิดขึ้นเมื่อเปิดรายการประเภทนี้ { $scope ->
        [everyone] สำหรับทุกคนบนพีซีเครื่องนี้
       *[you] สำหรับคุณ
    }
core-review-client-entry-detail = สามารถเลือกเป็นโปรแกรมสำหรับสิ่งนี้ได้ โดยไม่มีการเปลี่ยนแปลงค่าใด
core-review-com-permissions-title = สิทธิ์ของ COM
core-review-com-permissions-detail = ระบุว่าใครสามารถเริ่มคลาส COM ของแอปได้ หรือคลาสเหล่านั้นทำงานในฐานะใคร
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    ลิงก์ชนิดนี้จะเปิดด้วยแอปนี้ { $scope ->
        [everyone] สำหรับทุกคนบนพีซีเครื่องนี้
       *[you] สำหรับคุณ
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    ไฟล์ชนิดนี้จะเปิดด้วยแอปนี้ { $scope ->
        [everyone] สำหรับทุกคนบนพีซีเครื่องนี้
       *[you] สำหรับคุณ
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = สามารถเขียนลงในบันทึก { $log } ได้ โดยใช้ข้อความจากไฟล์ของแอปเอง
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    โหลดจากโฟลเดอร์ที่ติดตั้งเมื่อลงชื่อเข้าใช้ { $scope ->
        [everyone] สำหรับทุกคนบนพีซีเครื่องนี้
       *[you] สำหรับคุณ
    }
core-review-odbc-driver-detail = โหลดเข้าไปในโปรแกรมที่ใช้ ODBC เพื่อเข้าถึงฐานข้อมูล
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    การเชื่อมต่อฐานข้อมูล { $scope ->
        [everyone] สำหรับทุกคนบนพีซีเครื่องนี้
       *[you] สำหรับคุณ
    }
core-review-default-apps-detail = สามารถเลือกได้ใน “แอปเริ่มต้น” โดยไม่มีการเปลี่ยนแปลงค่าใด
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    “เรียกใช้” และโปรแกรมอื่นสามารถค้นหาโปรแกรมนี้ได้ ด้วยชื่อนี้ { $scope ->
        [everyone] สำหรับทุกคนบนพีซีเครื่องนี้
       *[you] สำหรับคุณ
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM ใน { $server }
core-review-com-title-own-file = COM ในไฟล์ที่แอปติดตั้ง
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
       *[other] การตั้งค่า { $count } รายการของคลาสที่โปรแกรมอื่นสามารถเริ่มได้
    }
core-review-com-support-title = ไลบรารีชนิด อินเทอร์เฟซ และ AppID ของ COM
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = การตั้งค่าสำหรับไลบรารีชนิด ({ $libraries }) อินเทอร์เฟซ ({ $interfaces }) และ AppID ({ $app_ids }) ซึ่งไปพร้อมกับคลาสของแอป
core-review-compatibility-title = วิธีที่โปรแกรมของแอปเริ่มทำงาน
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
       *[other] การตั้งค่าความเข้ากันได้ { $count } รายการสำหรับโปรแกรมที่แอปติดตั้ง
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = ข้อมูลผู้ใช้และการตั้งค่าต่อผู้ใช้จะไปยัง { $account } ซึ่งลงชื่อเข้าใช้อยู่ที่นี่ ไม่ใช่ผู้ดูแลระบบที่ใช้เรียกใช้โปรแกรมนี้
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = ไม่มีผู้ใดลงชื่อเข้าใช้เซสชันนี้ ข้อมูลผู้ใช้และการตั้งค่าต่อผู้ใช้จึงจะไปยัง { $account } ซึ่งเป็นบัญชีที่ใช้เรียกใช้โปรแกรมนี้
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = ไม่สามารถทราบได้ว่าใครลงชื่อเข้าใช้อยู่ ({ $reason }) จึงไม่รวมการตั้งค่าต่อผู้ใช้ไว้ และข้อมูลผู้ใช้จะไปอยู่ภายใต้ตำแหน่งที่ติดตั้ง
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = ไม่สามารถทราบได้ว่าใครลงชื่อเข้าใช้อยู่: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = ไม่อยู่ในตัวติดตั้งนี้ เนื่องจากดูเหมือนข้อมูลประจำตัว
core-review-left-out-settings = การตั้งค่าและประวัติที่บัญชีเก็บไว้สำหรับแอป ซึ่งจะรวมไว้เฉพาะเมื่อเลือกตัวเลือกข้อมูลผู้ใช้
core-review-left-out-certificates = แอปดูเหมือนจะอาศัยใบรับรอง ที่ได้รับความเชื่อถือบนพีซีเครื่องเดิม และใบรับรองเหล่านั้นจะไม่ถูกติดตั้ง
core-review-left-out-fonts = ฟอนต์เหล่านี้ใช้ชื่อของแอป แต่ไม่มีสิ่งใดระบุว่าเป็นของแอป จึงไม่ถูกติดตั้ง
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = ไม่อยู่ในตัวติดตั้งนี้ เนื่องจากดูเหมือนข้อมูลประจำตัว: { $names } ข้อมูลเหล่านี้ที่มีอยู่บนพีซีเครื่องนี้จะคงไว้ตามเดิม และคุณอาจต้องป้อนข้อมูลเหล่านี้อีกครั้ง
# { $names } is a list of names of license data.
core-review-note-licenses = ไม่อยู่ในตัวติดตั้งนี้ เนื่องจากตัวเลือก “ข้อมูลสิทธิ์การใช้งานและการเปิดใช้งาน” ปิดอยู่ตอนที่สร้างตัวติดตั้ง: { $names }
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = ไม่อยู่ในตัวติดตั้งนี้ เนื่องจากตัวเลือกข้อมูลผู้ใช้ ปิดอยู่ตอนที่สร้างตัวติดตั้ง: การตั้งค่าและประวัติที่บัญชีเก็บไว้สำหรับแอปใน { $names } แอปจะเริ่มทำงานที่นี่ เหมือนกับสำหรับผู้ที่เพิ่งเริ่มใช้แอป
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = แอปดูเหมือนจะอาศัยใบรับรอง ที่ได้รับความเชื่อถือบนพีซีเครื่องเดิม และใบรับรองเหล่านั้นจะไม่ถูกติดตั้ง: { $names } คุณเป็นผู้ตัดสินใจ ว่าจะเชื่อถือใบรับรองเหล่านี้ที่นี่หรือไม่
# { $names } is a list of font names.
core-review-note-fonts = ฟอนต์เหล่านี้ใช้ชื่อของแอป แต่ไม่มีสิ่งใดระบุว่าเป็นของแอป จึงไม่ถูกติดตั้ง: { $names }
# { $names } is a list of names.
core-review-note-left-out-by-choice = ผู้ที่สร้างตัวติดตั้งนี้ไม่ได้รวมไว้: { $names }
# { $names } is a list of names.
core-review-note-added-by-choice = ผู้ที่สร้างตัวติดตั้งนี้เพิ่มไว้ เนื่องจากเห็นว่าเป็นส่วนหนึ่งของแอป: { $names }
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } ใน { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names } และอีก { $count } รายการ

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = เป็นของการตั้งค่าของบัญชีหนึ่ง และไม่สามารถทราบได้ว่าใครลงชื่อเข้าใช้อยู่
core-review-name-not-accepted = Windows ไม่ยอมรับชื่อนี้
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = ไม่สามารถอ่านการตั้งค่าปัจจุบันได้: { $error }
core-review-service-dependency = ขึ้นอยู่กับบริการที่มีชื่อไม่ถูกต้อง
# { $path } is the program the service runs.
core-review-service-runs-other = เรียกใช้ { $path } ซึ่งแพคเกจนี้ไม่ได้ติดตั้ง
core-review-service-settings = การตั้งค่าระบุถึงสิ่งที่แพคเกจนี้ไม่ได้ติดตั้ง หรือคีย์ที่ไม่ใช่ของบริการเอง
core-review-driver-no-hash = ไม่มีแฮชสำหรับใช้ตรวจสอบ
core-review-license-for-the-machine = สิทธิ์การใช้งานที่เก็บไว้สำหรับทั้งเครื่อง จะได้รับการรายงานเท่านั้น
core-review-license-starts-something = สิทธิ์การใช้งานไม่มีเหตุผลที่จะต้องเริ่มสิ่งใด
core-review-task-not-captured = ไม่ได้เก็บคำจำกัดความของงานไว้
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = ไม่สามารถอ่านคำจำกัดความของงานได้: { $error }
core-review-task-no-account = งานทำงานในฐานะผู้ที่ติดตั้ง และไม่สามารถทราบได้ว่าใครลงชื่อเข้าใช้อยู่ที่นี่
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = เปิด { $path } ซึ่งแพคเกจนี้ไม่ได้ติดตั้ง
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } มีอยู่แล้วบนพีซีเครื่องนี้และเรียกใช้ { $path } ซึ่งไม่ได้มาจากการติดตั้งครั้งก่อนที่นี่ หากเป็นสำเนาเก่าของแอปนี้ ให้ถอนการติดตั้งสำเนานั้นก่อน
core-policy-key-control-characters = ชื่อคีย์มีอักขระควบคุม
core-policy-value-control-characters = ชื่อค่ามีอักขระควบคุม
core-policy-registry-link = เป็นลิงก์รีจิสทรี
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = รายการนี้แทนคีย์ของพีซี และ{ $reason }
core-policy-registry-part = ไม่ควรมีการติดตั้งแอปใด เขียนลงในส่วนนี้ของรีจิสทรี
core-policy-key-refused = ไม่ควรมีการติดตั้งแอปใดเปลี่ยนแปลงคีย์นี้
core-policy-net-debugger = จะกำหนดดีบักเกอร์ให้กับโปรแกรม .NET ทุกโปรแกรม
core-policy-net-install-root = จะเปลี่ยนตำแหน่งที่โปรแกรม .NET ทุกโปรแกรมโหลด .NET
core-policy-class-redirects = จะเปลี่ยนวิธีเปิดโปรแกรม หรือโฟลเดอร์ประเภทหนึ่ง
core-policy-app-path-name = ระบุโปรแกรมด้วยสิ่งที่ไม่ใช่ชื่อไฟล์
core-policy-app-path-not-installed = จะลงทะเบียนชื่อโปรแกรมให้กับไฟล์ ที่แพคเกจนี้ไม่ได้ติดตั้ง
core-policy-shared-default-verb = จะเปลี่ยนสิ่งที่เกิดขึ้น เมื่อเปิดไฟล์หรือโฟลเดอร์ทุกรายการของประเภทหนึ่ง
core-policy-command-runs-other = เพิ่มคำสั่งที่เรียกใช้สิ่งที่แพคเกจนี้ไม่ได้ติดตั้ง
core-policy-autostart-not-command = เริ่มทำงานโดยอัตโนมัติ แต่ค่าไม่ใช่คำสั่ง
core-policy-autostart-runs-other = เริ่มทำงานโดยอัตโนมัติ และเรียกใช้สิ่งที่แพคเกจนี้ไม่ได้ติดตั้ง
core-policy-environment-own-rules = ตัวแปรสภาพแวดล้อมจะถูกคืนค่า ตามกฎเฉพาะของตัวแปรเอง
core-policy-service-not-restored = เป็นของบริการที่แพคเกจนี้ไม่ได้คืนค่า
core-policy-service-own-rules = การตั้งค่าของบริการจะถูกคืนค่า ตามกฎของบริการเท่านั้น
core-policy-service-dll = ServiceDll ไม่ใช่ไฟล์ที่แพคเกจนี้ติดตั้ง
core-policy-event-log-source-only = คืนค่าได้เฉพาะค่า ของแหล่งที่มาของบันทึกเหตุการณ์เองเท่านั้น
core-policy-security-log = ไม่มีแอปใดเขียนลงในบันทึกความปลอดภัย
core-policy-event-log-messages = ข้อความจะมาจากไฟล์ที่แพคเกจนี้ไม่ได้ติดตั้ง
core-policy-type-library = ไลบรารีชนิดไม่ใช่ไฟล์ที่แพคเกจนี้ติดตั้ง
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = จะเปลี่ยนวิธีเปิด { $name } ซึ่งแพคเกจนี้ไม่ได้ติดตั้ง
core-policy-event-log-no-file = แหล่งที่มาของบันทึกเหตุการณ์ ต้องนำข้อความมาจากไฟล์
core-policy-com-runs-other = เรียกใช้สิ่งที่ไม่ใช่ไฟล์ที่แพคเกจนี้ติดตั้ง
core-policy-com-redirect = จะทำให้คลาสนี้กลายเป็นสิ่งที่แพคเกจนี้ไม่ได้ติดตั้ง
core-policy-com-server = เรียกใช้สิ่งที่ไม่ได้เป็นไฟล์ที่แพคเกจนี้ติดตั้ง และไม่ได้เป็นส่วนหนึ่งของ Windows
core-policy-interface = จะกำหนดวิธีเรียกอินเทอร์เฟซ ด้วยสิ่งที่แพคเกจนี้ไม่ได้ติดตั้ง
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = จะเปลี่ยนวิธีเริ่ม { $name } ซึ่งแพคเกจนี้ไม่ได้ติดตั้ง
core-policy-app-id-surrogate = จะถูกโฮสต์โดยโปรแกรมที่แพคเกจนี้ไม่ได้ติดตั้ง
core-policy-app-id-service = จะทำงานในบริการที่แพคเกจนี้ไม่ได้คืนค่า
core-policy-app-id-run-as = จะทำงานในฐานะบัญชีของตนเอง
core-policy-app-id-link = ไม่ได้ระบุ AppID
core-policy-app-id-remote = จะทำงานที่อื่นที่ไม่ใช่พีซีเครื่องนี้
core-policy-odbc-own-driver = นั่นเป็นการตั้งค่าของ ODBC เอง ไม่ใช่ไดรเวอร์
core-policy-odbc-driver-not-installed = ไดรเวอร์ไม่ใช่ไฟล์ที่แพคเกจนี้ติดตั้ง
core-policy-loads-other-file = จะโหลดไฟล์ที่แพคเกจนี้ไม่ได้ติดตั้ง
core-policy-odbc-own-source = นั่นเป็นการตั้งค่าของ ODBC เอง ไม่ใช่แหล่งข้อมูล
core-policy-odbc-source-missing = แสดงแหล่งข้อมูลที่แพคเกจนี้ไม่มี
core-policy-odbc-source-driver = ไดรเวอร์ไม่ได้เป็นไฟล์ที่แพคเกจนี้ติดตั้ง และไม่ได้เป็นส่วนหนึ่งของ Windows
core-policy-shell-extension = จะโหลดส่วนขยายเชลล์ ที่ไม่ใช่ไฟล์ที่แพคเกจนี้ติดตั้ง เข้าไปในตัวสำรวจไฟล์
core-policy-not-text = สิ่งที่เก็บไว้ไม่ใช่ข้อความตามที่จะถูกอ่าน
core-policy-network-place = ระบุตำแหน่งบนเครือข่าย
core-policy-font = จะลงทะเบียนฟอนต์ที่แพคเกจนี้ไม่ได้ติดตั้ง หรือฟอนต์ที่อยู่ในโฟลเดอร์ Windows
core-policy-compatibility = จะเปลี่ยนวิธีการทำงานของโปรแกรม ที่แพคเกจนี้ไม่ได้ติดตั้ง
core-policy-capabilities = แสดงแอปที่ไม่มีรายละเอียดอยู่ในแพคเกจนี้
core-policy-default-program = จะเปลี่ยนโปรแกรมที่ใช้เปิดสิ่งต่างๆ ตามค่าเริ่มต้น
core-policy-url-protocol = ระบุว่าเป็นชนิดลิงก์ แต่ไม่ได้เปิดด้วยสิ่งใดที่แพคเกจนี้ติดตั้ง
core-policy-command-handler = เพิ่มคำสั่งที่จัดการโดยสิ่งที่แพคเกจนี้ไม่ได้ติดตั้ง
core-policy-installer-command = จะเรียกใช้สิ่งที่มีเพียง Windows Installer เท่านั้นที่อ่านได้
core-policy-creates-class = จะสร้างคลาสที่แพคเกจนี้ไม่ได้ติดตั้ง
core-policy-opens-links = เปิดลิงก์ด้วยสิ่งที่แพคเกจนี้ไม่ได้ติดตั้ง
core-policy-opens-files = เปิดไฟล์ด้วยสิ่งที่แพคเกจนี้ไม่ได้ติดตั้ง
core-policy-firewall-name = ชื่อของกฎไม่ใช่ชื่อที่ส่งต่อได้
core-policy-firewall-description = คำอธิบายของกฎไม่ใช่คำอธิบายที่ส่งต่อได้
core-policy-firewall-every-program = ใช้กับทุกโปรแกรม แต่กฎหนึ่งข้อใช้ได้กับโปรแกรมเดียวเท่านั้น
# { $path } is the program the rule is for.
core-policy-firewall-program = ใช้สำหรับ { $path } ซึ่งแพคเกจนี้ไม่ได้ติดตั้ง
core-policy-firewall-direction = ไม่ได้ระบุทิศทางที่สามารถตั้งค่าได้
core-policy-firewall-action = จะลบล้างกฎอื่น ซึ่งกฎของแอปไม่ควรทำ
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = ระบุ { $setting } ซึ่งไม่สามารถตั้งค่าตามเดิมได้ หรือจะทำให้กฎอื่นมีผลน้อยลง
core-policy-firewall-service = ใช้สำหรับบริการที่แพคเกจนี้ไม่ได้คืนค่า
core-policy-firewall-parts = บางส่วนของกฎไม่ใช่สิ่งที่กฎไฟร์วอลล์ระบุได้
core-policy-folder-already-there = โฟลเดอร์นั้นมีอยู่แล้วนอกตำแหน่งที่ติดตั้ง และไม่มีสิ่งใดระบุว่าเป็นของแอปนี้ จึงไม่มีการวางโปรแกรมใดไว้ในนั้น
core-policy-variable-name = ชื่อไม่ใช่ชื่อตัวแปรที่ถูกต้อง
core-policy-variable-relied-on = Windows และโปรแกรมอื่นต้องอาศัยตัวแปรนี้
core-policy-task-name = ชื่อไม่ใช่ชื่อที่ Windows ยอมรับ
core-policy-task-folder = โฟลเดอร์ไม่ใช่โฟลเดอร์ที่ Windows ยอมรับ
core-policy-task-windows = จะแทนที่งานของ Windows เอง
core-policy-task-other-action = ทำสิ่งอื่นนอกเหนือจากการเรียกใช้โปรแกรม
core-policy-task-no-program = ไม่ได้ระบุโปรแกรมที่จะเรียกใช้
# { $path } is the program the task runs.
core-policy-task-runs-other = เรียกใช้สิ่งที่แพคเกจนี้ไม่ได้ติดตั้ง: { $path }
core-policy-task-exists-unreadable = มีงานชื่อนี้อยู่แล้วและไม่สามารถอ่านได้
core-policy-task-exists-other = มีงานชื่อนี้อยู่แล้วและไม่ได้เรียกใช้แอปนี้
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = มีงานชื่อนี้อยู่แล้วและเรียกใช้ { $path } ซึ่งไม่ได้มาจากการติดตั้งครั้งก่อนที่นี่
core-policy-shortcut-not-lnk = ไม่ใช่ทางลัด .lnk
core-policy-shortcut-place = ไม่ได้อยู่ในเมนูเริ่มหรือบนเดสก์ท็อป
core-policy-short-name = เส้นทางใช้ชื่อที่ไม่สามารถตรวจสอบได้
core-policy-shortcut-no-folder = พีซีเครื่องนี้ไม่มีโฟลเดอร์ดังกล่าวสำหรับทางลัดนี้
core-policy-data-startup = จะไปอยู่ในโฟลเดอร์ Startup และทำงานทุกครั้งที่ลงชื่อเข้าใช้
core-policy-data-powershell = จะไปอยู่ในตำแหน่งที่ PowerShell เรียกใช้สิ่งที่พบเมื่อเริ่มทำงาน
core-policy-data-windows = จะไปอยู่ในโฟลเดอร์ที่เป็นของ Windows
core-policy-license-outside = อยู่นอกตำแหน่งที่ติดตั้ง และโฟลเดอร์ข้อมูลที่รู้จักทั้งหมด
core-rewrite-cdata = มีส่วน CDATA
core-rewrite-too-much-to-look-through = แพคเกจนี้มีข้อมูลในการตั้งค่ามาก เกินกว่าจะตรวจสอบ สิ่งที่การตั้งค่าเหล่านั้นอ้างถึงได้

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] ไฟล์ { $item } ไม่ได้รวมไว้: { $reason }
        [folder] โฟลเดอร์ { $item } ไม่ได้รวมไว้: { $reason }
        [service] บริการ { $item } ไม่ได้รวมไว้: { $reason }
        [driver] ไดรเวอร์ { $item } ไม่ได้รวมไว้: { $reason }
        [firewall-rule] กฎไฟร์วอลล์ { $item } ไม่ได้รวมไว้: { $reason }
        [license-data] ข้อมูลสิทธิ์การใช้งานที่ { $item } ไม่ได้รวมไว้: { $reason }
        [license-value] ค่าสิทธิ์การใช้งาน { $item } ไม่ได้รวมไว้: { $reason }
        [scheduled-task] งานที่จัดกำหนดการไว้ { $item } ไม่ได้รวมไว้: { $reason }
        [shortcut] ทางลัด { $item } ไม่ได้รวมไว้: { $reason }
        [environment-variable] ตัวแปรสภาพแวดล้อม { $item } ไม่ได้รวมไว้: { $reason }
       *[value] { $item } ไม่ได้รวมไว้: { $reason }
    }
core-warning-review-package-path-missing = ไฟล์ที่คัดลอกได้ไม่มีเส้นทางสัมพัทธ์ในแพคเกจ
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = แพคเกจนี้มีโฟลเดอร์ว่างมากกว่า { $count } โฟลเดอร์ และจะสร้างได้เพียง { $count } โฟลเดอร์เท่านั้น
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
       *[other] ไฟล์ { $count } ไฟล์จากโฟลเดอร์เอกสารจะไปอยู่ภายใต้ { $target } แทน ในโฟลเดอร์ชื่อ _external เนื่องจากโฟลเดอร์เอกสารของพีซีเครื่องนี้อยู่ที่ { $folder } และจะเขียนข้อมูลลงในโฟลเดอร์ ที่อยู่ภายในโปรไฟล์ของคุณเท่านั้น หากต้องการใช้ไฟล์เหล่านี้ ให้ย้ายไปยังโฟลเดอร์เอกสารของคุณด้วยตนเอง
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
       *[other] ไฟล์ { $count } ไฟล์จากโฟลเดอร์เอกสารจะไปอยู่ภายใต้ { $target } แทน ในโฟลเดอร์ชื่อ _external เนื่องจากไม่สามารถทราบได้ ว่าโฟลเดอร์เอกสารของพีซีเครื่องนี้อยู่ที่ใด หากต้องการใช้ไฟล์เหล่านี้ ให้ย้ายไปยังโฟลเดอร์เอกสารของคุณด้วยตนเอง
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
       *[other] ไฟล์ { $count } ไฟล์จากโฟลเดอร์เกมที่บันทึกไว้จะไปอยู่ภายใต้ { $target } แทน ในโฟลเดอร์ชื่อ _external เนื่องจากโฟลเดอร์เกมที่บันทึกไว้ของพีซีเครื่องนี้ อยู่ที่ { $folder } และจะเขียนข้อมูลลงในโฟลเดอร์ ที่อยู่ภายในโปรไฟล์ของคุณเท่านั้น หากต้องการใช้ไฟล์เหล่านี้ ให้ย้ายไปยังโฟลเดอร์เกมที่บันทึกไว้ของคุณ ด้วยตนเอง
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
       *[other] ไฟล์ { $count } ไฟล์จากโฟลเดอร์เกมที่บันทึกไว้จะไปอยู่ภายใต้ { $target } แทน ในโฟลเดอร์ชื่อ _external เนื่องจากไม่สามารถทราบได้ ว่าโฟลเดอร์เกมที่บันทึกไว้ของพีซีเครื่องนี้ อยู่ที่ใด หากต้องการใช้ไฟล์เหล่านี้ ให้ย้ายไปยังโฟลเดอร์เกมที่บันทึกไว้ของคุณ ด้วยตนเอง
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } เป็นลิงก์หรือไฟล์ ไม่ใช่โฟลเดอร์
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } ไม่ได้สร้างโดย WinAppMigrator และผู้อื่นสามารถเปลี่ยนแปลงสิ่งที่อยู่ภายในได้ ผู้ดูแลระบบควรตรวจสอบและลบโฟลเดอร์นี้: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } มีบางสิ่งอยู่ภายในที่ไม่ได้สร้างโดย WinAppMigrator และผู้อื่นสามารถเปลี่ยนแปลงได้ ผู้ดูแลระบบควรตรวจสอบและลบโฟลเดอร์นี้
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } กลายเป็นลิงก์ในระหว่างที่กำลังใช้งาน
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } กลายเป็นลิงก์ในระหว่างที่กำลังตรวจดู
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = ไม่สามารถอนุญาตให้ผู้ใช้เริ่ม { $path } ได้: { $said }
core-privilege-elevation-unknown = ไม่สามารถทราบได้ว่ากระบวนการนี้ ทำงานด้วยสิทธิ์ที่ยกระดับหรือไม่
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = ไม่สามารถจำกัด { $path } ให้เฉพาะผู้ดูแลระบบได้: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = ไม่สามารถจำกัด { $path } ให้เฉพาะผู้ดูแลระบบได้ และไดรฟ์ของโฟลเดอร์นี้อาจไม่เก็บสิทธิ์: { $said }
# { $path } is a folder.
core-privilege-written-to-first = กระบวนการอื่นเขียนลงใน { $path } ก่อนที่จะป้องกันได้
core-privilege-no-program-data = Windows ไม่ได้ระบุว่า ProgramData อยู่ที่ใด

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } ไม่มีแพคเกจอยู่ภายใน
# { $path } is a folder.
core-verify-not-unpacked = แพคเกจของตัวติดตั้งนี้ไม่ได้ถูกแตกไฟล์ไปยัง { $path }
core-verify-schema-supported = รองรับสคีมาของรายการแสดงแพคเกจ
core-verify-packaged-file-missing = ไฟล์ในแพคเกจหายไป
core-verify-no-package-path = ไฟล์ที่คัดลอกได้ไม่มีเส้นทางสัมพัทธ์ในแพคเกจ
core-verify-folder-there = มีโฟลเดอร์ว่างอยู่
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = ไม่สามารถอ่านเพื่อตรวจสอบได้: { $error }
core-verify-restored-file-missing = ไฟล์ที่คืนค่าหายไป
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = ขนาดไฟล์ไม่ตรงกัน ควรเป็น { $expected } ไบต์ แต่พบ { $actual } ไบต์
core-verify-no-hash = รายการแสดงไม่ได้บันทึกแฮชของไฟล์นี้ไว้ จึงตรวจสอบไม่ได้
core-verify-hash-mismatch = แฮช SHA-256 ไม่ตรงกัน
core-verify-file-matches = ขนาดไฟล์และแฮชตรงกับรายการแสดง
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = การตรวจสอบข้อมูลสิทธิ์การใช้งานล้มเหลว: { $error }
core-verify-license-payload-missing = ข้อมูลสิทธิ์การใช้งานในแพคเกจหายไป
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = การตรวจสอบที่เก็บสิทธิ์การใช้งานล้มเหลว: { $error }
core-verify-license-store-file-missing = ไฟล์ที่เก็บสิทธิ์การใช้งานในแพคเกจหายไป
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = เส้นทางที่เก็บสิทธิ์การใช้งานในแพคเกจ ไม่ถูกต้อง: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = การตรวจสอบข้อมูลส่วนประกอบที่ต้องมีก่อน ล้มเหลว: { $error }
core-verify-prerequisite-missing = ข้อมูลส่วนประกอบที่ต้องมีก่อนที่รวมมาหายไป
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = การตรวจสอบอิมเมจไดรเวอร์ล้มเหลว: { $error }
core-verify-bundled-driver-missing = อิมเมจไดรเวอร์ที่รวมมาหายไป
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = เส้นทางไดรเวอร์ที่รวมมาไม่ถูกต้อง: { $error }
core-verify-prerequisite-present = มีรันไทม์ที่ต้องมีก่อนอยู่บนเครื่องเป้าหมาย
core-verify-prerequisite-not-detected = ไม่พบรันไทม์ที่ต้องมีก่อน อาจต้องเริ่มระบบใหม่ หรือดำเนินการให้เสร็จด้วยตนเอง (ดูผลลัพธ์ที่แน่นอนในไฟล์บันทึกการคืนค่า)
core-verify-prerequisite-not-probed = ไม่สามารถตรวจหาส่วนประกอบที่ต้องมีก่อน จากรีจิสทรีได้ การติดตั้งดำเนินการเท่าที่ทำได้
core-verify-license-file-missing = ไฟล์สิทธิ์การใช้งานที่คืนค่าหายไป
core-verify-license-store-restored-missing = ไฟล์ที่เก็บสิทธิ์การใช้งานที่คืนค่าหายไป
core-verify-registry-key-there = มีคีย์รีจิสทรีอยู่
core-verify-registry-value-matches = ค่ารีจิสทรีตรงกับข้อมูลที่เก็บถาวรไว้
core-verify-registry-value-differs = ค่ารีจิสทรีไม่ตรงกับข้อมูลที่เก็บถาวรไว้
core-verify-firewall-rule-there = ไฟร์วอลล์ Windows มีกฎนี้อยู่
core-verify-variable-matches = ค่าตัวแปรสภาพแวดล้อม ตรงกับข้อมูลที่เก็บถาวรไว้
core-verify-variable-differs = ค่าตัวแปรสภาพแวดล้อม ไม่ตรงกับข้อมูลที่เก็บถาวรไว้
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = ไม่มีตัวแปรสภาพแวดล้อมหรืออ่านไม่ได้: { $error }
core-verify-service-runs-other = มีบริการอยู่ แต่ไม่ได้เรียกใช้ไบนารีที่คืนค่าพร้อมอาร์กิวเมนต์
core-verify-service-other-account = มีบริการอยู่ แต่ไม่ได้ทำงานในฐานะบัญชีที่คาดไว้
core-verify-service-matches = มีบริการอยู่ และเรียกใช้ไบนารีที่คืนค่าพร้อมอาร์กิวเมนต์ และบัญชีที่ถูกต้อง
core-verify-service-missing = ไม่มีบริการ
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = ไม่สามารถอ่านบริการได้: { $error }
core-verify-driver-other-image = บริการไดรเวอร์ไม่ได้ชี้ไปยังอิมเมจที่คืนค่า
core-verify-driver-not-registered = บริการไดรเวอร์ที่รวมมา ไม่ได้ลงทะเบียนหลังการคืนค่า
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = ไม่สามารถอ่านบริการไดรเวอร์ได้: { $error }
core-verify-driver-image-missing = อิมเมจไดรเวอร์หายไป
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = ไม่สามารถตรวจสอบอิมเมจไดรเวอร์ได้: { $error }
core-verify-task-matches = มีงานที่จัดกำหนดการไว้อยู่ และเรียกใช้โปรแกรมที่คืนค่า
core-verify-task-runs-other = มีงานที่จัดกำหนดการไว้อยู่ แต่ไม่ได้เรียกใช้โปรแกรมที่คืนค่า
core-verify-task-missing = ไม่มีงานที่จัดกำหนดการไว้
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = การสอบถามงานที่จัดกำหนดการไว้ล้มเหลว: { $error }
core-verify-shortcut-matches = มีทางลัดอยู่ และเส้นทางเป้าหมายตรงกับเป้าหมายที่คืนค่า
core-verify-shortcut-opens-other = มีทางลัดอยู่ แต่เส้นทางเป้าหมายไม่ตรงกับเป้าหมายที่คืนค่า
core-verify-shortcut-missing = ไม่มีทางลัด
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = การสอบถามทางลัดล้มเหลว: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = ไม่รองรับสคีมาของรายการแสดงแพคเกจ
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = การตรวจสอบไฟล์ในแพคเกจล้มเหลว: { $error }
core-warning-verify-folder-missing = ไม่มีโฟลเดอร์ว่าง
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = ไม่มีคีย์รีจิสทรีหรืออ่านไม่ได้: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = ไม่มีค่ารีจิสทรีหรืออ่านไม่ได้: { $error }
core-warning-verify-registry-kind-missing = ไม่มีชนิดของค่ารีจิสทรี
core-warning-verify-firewall-rule-missing = ไฟร์วอลล์ Windows ไม่มีกฎนี้
core-warning-verify-service-scope = การตรวจสอบบริการ จะอ่านการกำหนดค่าบริการใน HKLM และไม่เริ่มบริการ
core-warning-verify-driver-manual = WinAppMigrator ไม่แจกจ่ายไดรเวอร์ของผู้จำหน่าย หรือไดรเวอร์ที่มาพร้อมกับ Windows อย่างน้อยหนึ่งรายการ โปรดติดตั้งไดรเวอร์เหล่านั้นใหม่บนเครื่องเป้าหมาย ด้วยแพคเกจไดรเวอร์ที่ลงนามแล้วของผู้จำหน่าย
core-warning-verify-task-scope = การตรวจสอบงานที่จัดกำหนดการไว้ จะตรวจสอบเฉพาะการลงทะเบียน และคำสั่งที่อ้างอิงเท่านั้น

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = รายงานการตรวจสอบของ WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = ตรวจสอบเมื่อ (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = แพคเกจถูกต้อง: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = การคืนค่าถูกต้อง: { $valid }
core-verify-report-restore-not-run = การคืนค่าถูกต้อง: ไม่ได้เรียกใช้
core-verify-report-items = รายการ
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } แหล่งที่มา: { $source }
core-verify-report-warnings = คำเตือน
core-verify-report-errors = ข้อผิดพลาด
core-verify-report-none = ไม่มี
