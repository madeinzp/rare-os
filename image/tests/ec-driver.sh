#!/usr/bin/env bash
# Tests for the ayaneo_ec / ayaneo_platform install hook (system_files/usr/libexec/rare/ec-driver,
# usr/lib/modprobe.d/rare-suiplay-ec.conf) with a fake DMI and a stub modprobe:
#   image/tests/ec-driver.sh
# On the image's own modules (modprobe -n from a fedora container, the module tree unpacked in
# $1) it also checks what udev's DMI modalias resolves to: image/tests/ec-driver.sh <root>
set -uo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
H="$here/../system_files/usr/libexec/rare/ec-driver"
CONF="$here/../system_files/usr/lib/modprobe.d/rare-suiplay-ec.conf"
T="$(mktemp -d)"; trap 'rm -rf "$T"' EXIT
export RARE_DMI_DIR="$T/dmi" MODPROBE="$T/modprobe" LOG="$T/log"
mkdir -p "$T/dmi"
# The stub logs each call; $T/fail-suiplay makes `modprobe ayaneo_ec_suiplay` fail.
cat >"$MODPROBE" <<'S'
#!/bin/sh
echo "$*" >>"$LOG"
[ "$1" = ayaneo_ec_suiplay ] && [ -e "${LOG%/log}/fail-suiplay" ] && exit 1
exit 0
S
chmod +x "$MODPROBE"
fails=0
dmi() { printf '%s\n' "$1" >"$T/dmi/sys_vendor"; printf '%s\n' "$2" >"$T/dmi/product_name"; }
check() { # name, module and options, expected log
    rm -f "$LOG"; "$H" "${@:2:$#-2}" || { echo "FAIL $1: exit $?"; fails=$((fails + 1)); return; }
    local got; got="$(cat "$LOG" 2>/dev/null)"
    if [[ "$got" == "${!#}" ]]; then echo "ok   $1"; else echo "FAIL $1: got [$got] want [${!#}]"; fails=$((fails + 1)); fi
}

dmi "Mysten Labs, Inc." "SuiPlay0X1"
check "SuiPlay: ayaneo_ec becomes ayaneo_ec_suiplay" ayaneo_ec "ayaneo_ec_suiplay"
check "SuiPlay: ayaneo_platform becomes ayaneo_ec_suiplay" ayaneo_platform "ayaneo_ec_suiplay"
touch "$T/fail-suiplay"
check "SuiPlay without a working ayaneo_ec_suiplay: the fan keeps the kernel's driver" ayaneo_ec foo=1 \
    "$(printf 'ayaneo_ec_suiplay\n--ignore-install ayaneo_ec foo=1')"
rm -f "$T/fail-suiplay"

dmi "AYANEO" "AYANEO 2S"
check "AYANEO 2S: ayaneo_ec as usual" ayaneo_ec "--ignore-install ayaneo_ec"
check "AYANEO 2S: ayaneo_platform as usual, options kept" ayaneo_platform a=1 b=2 "--ignore-install ayaneo_platform a=1 b=2"
dmi "Mysten Labs, Inc." "SuiPlay0X2"
check "another Mysten product: as usual" ayaneo_ec "--ignore-install ayaneo_ec"
rm -f "$T/dmi/sys_vendor" "$T/dmi/product_name"
check "no DMI (a VM, a container): as usual" ayaneo_ec "--ignore-install ayaneo_ec"

# The conf names the hook at its installed path, for both modules, passing the kernel's options.
for m in ayaneo_ec ayaneo_platform; do
    if grep -qx "install $m /usr/libexec/rare/ec-driver $m \$CMDLINE_OPTS" "$CONF"; then echo "ok   conf: $m"; else echo "FAIL conf: $m"; fails=$((fails + 1)); fi
done

# With an unpacked image (usr/lib/modules/<kver>): the SuiPlay0X1's DMI modalias (udev loads every
# module it resolves to) goes to ayaneo_ec_suiplay directly and to the hook for the other two.
if [[ -n "${1:-}" ]]; then
    root="$1"; kver="$(ls "$root/usr/lib/modules" | sort -V | tail -n1)"
    alias='dmi:bvnAmericanMegatrendsInternational,LLC.:bvr1.0:bd01/01/2024:br1.0:svnMystenLabs,Inc.:pnSuiPlay0X1:pvr1.0:rvnMystenLabs,Inc.:rnSuiPlay0X1:rvr1.0:cvnMystenLabs,Inc.:ct10:cvr1.0:sku:'
    conf="$T/modprobe.d"; mkdir -p "$conf"; cp "$CONF" "$conf/"
    out="$(modprobe -n -v -C "$conf" -d "$root/usr" -S "$kver" -a "$alias" 2>&1)"
    echo "$out" | sed 's/^/     /'
    for want in "install /usr/libexec/rare/ec-driver ayaneo_ec" "install /usr/libexec/rare/ec-driver ayaneo_platform" "insmod .*/extra/ayaneo-ec-suiplay.ko"; do
        if grep -Eq "^$want" <<<"$out"; then echo "ok   modalias: $want"; else echo "FAIL modalias: $want"; fails=$((fails + 1)); fi
    done
    if grep -Eq '^insmod .*(kernel/drivers/platform/x86/ayaneo-ec|ayaneo-platform)\.ko' <<<"$out"; then
        echo "FAIL modalias: the kernel's ayaneo_ec or ayaneo_platform is inserted directly"; fails=$((fails + 1))
    else
        echo "ok   modalias: neither ayaneo_ec nor ayaneo_platform is inserted directly"
    fi
fi

echo "$fails failed"
exit $((fails > 0))
