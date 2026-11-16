#!/usr/bin/env bash
# Tests for os-update (the update state the launcher reads), run as an unprivileged user with stub
# bootc, rpm-ostree, busctl and logger and a fake power supply. Needs jq and flock (Linux).
#   image/tests/os-update.sh
set -uo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
U="$here/../system_files/usr/libexec/rare/os-update"
T="$(mktemp -d)"; trap 'rm -rf "$T"' EXIT
export RARE_UPDATE_STATE_DIR="$T/state" RARE_UPDATE_RUN_DIR="$T/run" RARE_POWER_DIR="$T/psu"
export RARE_USER_RUN="$T/user" RARE_BOOTC="$T/bin/bootc" PATH="$T/bin:$PATH" LOG="$T/log"
# no waiting between tries; nmcli says what $LOG/nm holds (connected when there is none)
export RARE_UPDATE_RETRY_DELAYS="0 0" RARE_NMCLI="$T/bin/nmcli"
mkdir -p "$T/bin" "$T/psu/BAT0" "$T/psu/ACAD" "$T/user/1000" "$LOG"
echo Battery >"$T/psu/BAT0/type"; echo Mains >"$T/psu/ACAD/type"
power() { echo "$1" >"$T/psu/BAT0/capacity"; echo "$2" >"$T/psu/ACAD/online"; }
power 80 1
# rpm-ostree status --json: $LOG/staged decides whether a staged deployment exists
cat >"$T/bin/rpm-ostree" <<'S'
#!/bin/sh
case "$1" in
  status)
    if [ -e "$LOG/staged" ]; then st='{"staged":true,"booted":false,"version":"0.2.0","checksum":"aaaa1111","serial":0,"osname":"default","container-image-reference-digest":"sha256:new"},'; else st=""; fi
    echo "{\"deployments\":[$st{\"booted\":true,\"version\":\"0.1.0\",\"checksum\":\"bbbb2222\",\"container-image-reference\":\"ostree-image-signed:docker://ghcr.io/example/rare-os:stable\"}]}" ;;
  cleanup) rm -f "$LOG/staged" ;;
esac
S
# bootc: $LOG/offer = an update is on offer; upgrade writes progress to the fd and stages
cat >"$T/bin/bootc" <<'S'
#!/bin/bash
echo "$*" >>"$LOG/bootc"
if [[ " $* " == *" --check "* ]]; then
  if [[ -e "$LOG/fail" ]]; then echo "error: Temporary failure in name resolution" >&2; exit 1; fi
  if [[ -e "$LOG/sigfail" ]]; then echo "error: Upgrading: Creating importer: A signature was required, but no signature exists" >&2; exit 1; fi
  if [[ -e "$LOG/failonce" ]]; then rm -f "$LOG/failonce"; echo "error: Get manifest: read tcp: connection reset by peer" >&2; exit 1; fi
  if [[ -e "$LOG/offer" ]]; then printf 'Update available for: x\n  Version: 0.2.0\n  Digest: sha256:new\n'; else echo "No changes in: x"; fi
  exit 0
fi
fd=""; a=("$@"); for ((i=0;i<${#a[@]};i++)); do [[ "${a[$i]}" == --progress-fd ]] && fd="${a[$((i+1))]}"; done
[[ -n "$fd" ]] && echo '{"type":"ProgressBytes","task":"pulling","bytesCached":1000,"bytes":500,"bytesTotal":3000,"stepsCached":1,"steps":1,"stepsTotal":3}' >&"$fd"
if [[ -e "$LOG/slow" ]]; then sleep 30; fi
if [[ -e "$LOG/dlfailonce" ]]; then rm -f "$LOG/dlfailonce"; echo "error: Importing: fetching layer: unexpected EOF" >&2; exit 1; fi
if [[ -e "$LOG/offer" || -e "$LOG/local" ]]; then touch "$LOG/staged"; else echo "No update available."; fi
S
printf '#!/bin/sh\necho "v u 0"\n' >"$T/bin/busctl"
printf '#!/bin/sh\ncat "$LOG/nm" 2>/dev/null || echo connected\n' >"$T/bin/nmcli"
printf '#!/bin/sh\nexit 0\n' >"$T/bin/logger"
chmod +x "$T/bin/"*
S="$T/state/update-state.json"
pass=0; fail=0
ok() { pass=$((pass + 1)); echo "ok   $1"; }
bad() { fail=$((fail + 1)); echo "FAIL $1"; }
eq() { if [[ "$2" == "$3" ]]; then ok "$1"; else bad "$1 (got '$2', want '$3')"; fi; }
j() { jq -r "$1" "$S" 2>/dev/null; }

"$U" >/dev/null 2>&1
eq "up to date" "$(j '.phase + "/" + .result + "/" + (.ok|tostring)')" "idle/current/true"
: >"$LOG/bootc"; "$U" >/dev/null 2>&1
eq "not due again within 20 h" "$(wc -l <"$LOG/bootc" | tr -d ' ')" 0

touch "$LOG/offer"; "$U" --now >/dev/null 2>&1
eq "staged" "$(j '.phase + "/" + .result + "/" + .stagedVersion')" "ready/staged/0.2.0"
eq "progress counts the cached layers" "$(j '"\(.bytes)/\(.bytesTotal)"')" "1500/4000"
[[ -s "$T/state/update-staged" ]] && ok "marker written" || bad "marker written"

rm -f "$LOG/staged" "$T/state/update-staged"; rm -f "$S"
power 20 0; "$U" >/dev/null 2>&1
eq "automatic waits on low battery" "$(j '.phase + "/" + .waitReason')" "waiting/battery"
power 20 0; "$U" --now >/dev/null 2>&1
eq "manual at 20% runs" "$(j '.result')" "staged"
rm -f "$LOG/staged" "$S"; power 10 0; "$U" --now >/dev/null 2>&1
eq "manual under 15% refuses" "$(j '.result')" "failed"
j '.error' | grep -q "Plug in" && ok "says plug in" || bad "says plug in"

rm -f "$S"; power 90 0; touch "$T/user/1000/rare-playing"; "$U" >/dev/null 2>&1
eq "automatic waits for a game" "$(j '.waitReason')" "game"
rm -f "$T/user/1000/rare-playing"

rm -f "$S"; : >"$LOG/bootc"; touch "$LOG/fail"; "$U" >/dev/null 2>&1
eq "offline failure" "$(j '.phase + "/" + .result')" "failed/failed"
j '.error' | grep -q offline && ok "offline in words" || bad "offline in words ($(j .error))"
eq "a network failure is tried three times" "$(grep -c -- --check "$LOG/bootc")" 3
rm -f "$LOG/fail"

# no connection at all: plain words, bootc never asked
rm -f "$S"; : >"$LOG/bootc"; echo disconnected >"$LOG/nm"; "$U" >/dev/null 2>&1
eq "no connection" "$(j '.phase + "/" + .result')" "failed/failed"
j '.error' | grep -q "not connected to the internet" && ok "no connection in words" || bad "no connection in words ($(j .error))"
eq "no check without a connection" "$(wc -l <"$LOG/bootc" | tr -d ' ')" 0
rm -f "$LOG/nm"

# a dropped connection during the check, then it works: no failure shown
rm -f "$S" "$LOG/staged"; : >"$LOG/bootc"; touch "$LOG/offer" "$LOG/failonce"; "$U" --now >/dev/null 2>&1
eq "a check that failed once is tried again" "$(j '.result + "/" + (.ok|tostring)')" "staged/true"
eq "two checks" "$(grep -c -- --check "$LOG/bootc")" 2

# a download that broke off once is tried again
rm -f "$S" "$LOG/staged" "$T/state/update-staged"; touch "$LOG/dlfailonce"; "$U" --now >/dev/null 2>&1
eq "a download that failed once is tried again" "$(j '.result')" "staged"

# a bad signature is not tried again, and says so
rm -f "$S" "$LOG/staged" "$T/state/update-staged"; : >"$LOG/bootc"; touch "$LOG/sigfail"; "$U" --now >/dev/null 2>&1
eq "signature failure" "$(j '.result')" "failed"
eq "a signature failure is not tried again" "$(grep -c -- --check "$LOG/bootc")" 1
j '.error' | grep -q "signature could not be checked" && ok "signature in words" || bad "signature in words ($(j .error))"
rm -f "$LOG/sigfail" "$LOG/staged" "$T/state/update-staged"

rm -f "$S"; echo '{"digest":"sha256:new","version":"0.2.0","reason":"failed"}' >"$T/state/update-skip.json"
"$U" --now >/dev/null 2>&1
eq "a version that did not start is skipped" "$(j '.result')" "skipped"
[[ -e "$LOG/staged" ]] && bad "skipped version staged" || ok "skipped version not downloaded"
echo '{"digest":"sha256:new","version":"0.2.0","reason":"rolled-back"}' >"$T/state/update-skip.json"
rm -f "$S"; "$U" >/dev/null 2>&1
eq "rolled back: automatic skips" "$(j '.result')" "skipped"
"$U" --now >/dev/null 2>&1
eq "rolled back: Update now installs" "$(j '.result')" "staged"
[[ -e "$T/state/update-skip.json" ]] && bad "skip cleared after manual install" || ok "skip cleared after manual install"

# a game starting stops an automatic download
rm -f "$S" "$LOG/staged"; power 90 1; touch "$LOG/slow"
"$U" >/dev/null 2>&1 &
pid=$!; sleep 2; touch "$T/user/1000/rare-playing"; wait $pid
eq "game stops the download" "$(j '.phase + "/" + .waitReason')" "waiting/game"
rm -f "$T/user/1000/rare-playing" "$LOG/slow"

# a staged update lost to a power-off: the check says "No changes", the upgrade stages it again
rm -f "$S" "$LOG/staged" "$LOG/offer"; touch "$LOG/local"; "$U" >/dev/null 2>&1
eq "lost update staged again from local data" "$(j '.result')" "staged"
rm -f "$LOG/local"

echo "$pass passed, $fail failed"
((fail == 0))
