#!/usr/bin/env bash
# AC-A11Y-AXE red drill: Axe.Windows on the real windows, with the code that gives each panel its bounds broken, then put
# back exactly. Builds in the drill worktree (at HEAD), scans on testpc1 in the signed-in console session.
set -u
SP=/c/Users/Randy/AppData/Local/Temp/claude/9b6bfcf6d6737e8e/28316fa3-07c3-42a6-9952-23884d58bf11/scratchpad
WT=$SP/drills-wt
export CARGO_TARGET_DIR=$(cygpath -w $SP/drill-target)
export PATH=$HOME/.cargo/bin:$PATH
FILE=crates/winappmigrator-ui/src/widgets.rs
FIND='        .find(|frame| frame.kind().is_some_and(|kind| kind.is_panel()))'
REPLACE='        .find(|_frame| false)'
EXPECT='An on-screen element must not have a null BoundingRectangle property.'
LOG=$SP/axe-drill.log
: > $LOG
say() { echo "$*" | tee -a $LOG; }

build() {
  (cd $WT && cargo build -p winappmigrator-installer -p winappmigrator-gui 2>&1 | tail -2) | tee -a $LOG
  return ${PIPESTATUS[0]}
}

scan() {
  local stage=$1
  ssh testpc1 "New-Item -ItemType Directory -Force C:\WamRepro\dl\axe-drill\$stage | Out-Null" 2>/dev/null
  scp -q "$SP/drill-target/debug/winappmigrator.exe" "testpc1:C:/WamRepro/dl/axe-drill/$stage/WinAppMigrator-drill.exe" 2>/dev/null || return 1
  scp -q "$SP/drill-target/debug/winappmigrator-installer.exe" "testpc1:C:/WamRepro/dl/axe-drill/$stage/winappmigrator-installer-drill.exe" 2>/dev/null || return 1
  ssh testpc1 "powershell -NoProfile -ExecutionPolicy Bypass -File C:\WamRepro\scripts\run-console.ps1 -Script C:\WamRepro\scripts\axe-drill-stage.ps1 -WaitSeconds 300 -Arguments '-Stage $stage'" 2>&1 \
    | grep -v -e 'post-quantum' -e 'store now' -e 'server may need' | tr -d '\r' | grep -v '^\s*$' | tee $SP/axe-drill-$stage.txt | tee -a $LOG
}

# An Axe.Windows run passes when every window it scanned says "exit 0"; it is red when one says "exit 1".
verdict() {
  local file=$SP/axe-drill-$1.txt
  local scans=$(grep -c '^  exit ' $file)
  local bad=$(grep '^  exit ' $file | grep -vc '^  exit 0$')
  if [ "$scans" -lt 3 ]; then echo 99; elif [ "$bad" -eq 0 ]; then echo 0; else grep '^  exit ' $file | grep -v '^  exit 0$' | head -1 | awk '{print $2}'; fi
}

cd $WT
before=$(sha256sum $FILE | cut -d' ' -f1)
say "drill a11y-axe"
say "file $FILE"
say "mutation: '$FIND' -> '$REPLACE'"
say "expect: $EXPECT"
say "===== baseline (sha256 $before)"
build || { say "baseline build failed"; exit 1; }
scan baseline
b=$(verdict baseline); say "baseline exit $b"

python - "$FILE" "$FIND" "$REPLACE" <<'PY'
import pathlib, sys
p = pathlib.Path(sys.argv[1]); s = p.read_bytes().decode('utf-8')
assert s.count(sys.argv[2]) == 1, 'mutation target not found exactly once'
p.write_bytes(s.replace(sys.argv[2], sys.argv[3]).encode('utf-8'))
PY
mutated=$(sha256sum $FILE | cut -d' ' -f1)
say "===== mutated (sha256 $mutated)"
build || say "mutated build failed"
scan mutated
m=$(verdict mutated); say "mutated exit $m"

git -C $WT checkout -- $FILE
after=$(sha256sum $FILE | cut -d' ' -f1)
say "===== restored (sha256 $after)"
build || { say "restored build failed"; exit 1; }
scan restored
r=$(verdict restored); say "restored exit $r"

observed=$(grep -m1 -F "$EXPECT" $SP/axe-drill-mutated.txt | sed 's/^ *//')
passed=false
if [ "$b" = 0 ] && [ "$r" = 0 ] && [ "$m" != 0 ] && [ "$m" != 99 ] && [ -n "$observed" ] && [ "$before" = "$after" ] && [ "$before" != "$mutated" ]; then passed=true; fi
say "drill a11y-axe passed=$passed: baseline $b, mutated $m with '$observed', restored $r"
python - "$SP/axe-drill.json" <<PY
import json, sys
json.dump({
  "name": "a11y-axe", "file": "$FILE", "mutation": """$FIND -> $REPLACE""",
  "command": "cargo build -p winappmigrator-installer -p winappmigrator-gui; on testpc1, as the signed-in user: axe-lab.ps1 (AxeWindowsCLI 2.4.2 --processid <pid> --outputdirectory <dir> --scanid <name> --alwayssavetestfile) on the app, the uninstaller with no record, and the installer with no package",
  "expected_diagnostic": "$EXPECT", "before": "$before", "mutated": "$mutated", "after": "$after",
  "baseline_exit": int("$b"), "mutated_exit": int("$m"), "restored_exit": int("$r"),
  "observed_diagnostic": """$observed""", "failure": None, "passed": "$passed" == "true",
}, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=2)
PY
