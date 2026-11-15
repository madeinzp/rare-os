#!/bin/bash
# PCR4 of an image's boot chain (shim + grub) as a SuiPlay measures it, and whether rare.market's device
# attestor accepts it (tools/boot-pcr4.py has the event order). Run as the user whose podman storage holds the image
# (root in the image workflows):
#   tools/boot-pcr4.sh <image ref> <out.json> [--no-check]
# <image ref> is anything podman create takes: localhost/rare-os:test, oci-archive:/path/rare-os.ociarchive, a registry ref.
# Prints the policy line for market attest/policy/boot.unsigned.json and writes it with the binaries' hashes to <out.json>.
# Exit 0: the attestor accepts this boot chain. 3: it does not yet (sign and deploy the line first, docs/ATTEST.md).
# 2: no boot chain in the image. BOOT_VALUES_URL overrides the attestor's address.
set -euo pipefail
ref="$1"; out="$2"; check=1
[ "${3:-}" = "--no-check" ] && check=0
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
url="${BOOT_VALUES_URL:-https://rare.market/v1/device/attest/boot-values}"
tmp="$(mktemp -d)"
ctr=""; img=""
# An image podman had to import for this (an oci-archive, a registry ref) is removed again; one it already had stays.
# IDs compared without the sha256: prefix (podman images prints it, container inspect does not).
ids() { podman images -q --no-trunc | sed 's/^sha256://' | sort -u; }
before="$(ids)"
cleanup() {
  local rc=$?
  if [ -n "$ctr" ]; then podman rm -f "$ctr" >/dev/null 2>&1 || true; fi
  if [ -n "$img" ] && ! grep -qx "$img" <<<"$before"; then podman rmi "$img" >/dev/null 2>&1 || true; fi
  rm -rf "$tmp"
  exit $rc
}
trap cleanup EXIT
ctr="$(podman create --entrypoint /bin/true "$ref")"
img="$(podman container inspect -f '{{.Image}}' "$ctr" | sed 's/^sha256://')"
mkdir -p "$tmp/root/usr/lib/bootupd"
# Fedora 45 bases keep the EFI payloads in /usr/lib/efi, older ones in /usr/lib/bootupd/updates; copy whichever exist.
podman cp "$ctr:/usr/lib/efi" "$tmp/root/usr/lib/efi" 2>/dev/null || true
podman cp "$ctr:/usr/lib/bootupd/updates" "$tmp/root/usr/lib/bootupd/updates" 2>/dev/null || true
args=("$tmp/root" --image "$ref" --out "$out")
[ $check = 1 ] && args+=(--check "$url")
python3 -I "$here/boot-pcr4.py" "${args[@]}"
