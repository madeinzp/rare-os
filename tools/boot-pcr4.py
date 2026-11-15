#!/usr/bin/env python3
"""PCR4 of a Rare OS image's boot chain (shim + grub), as a SuiPlay measures it.

  tools/boot-pcr4.py <rootfs> [--image REF] [--out FILE] [--check URL]

<rootfs> holds at least the image's /usr/lib/efi (bootupd's layout since the Fedora 45 base:
usr/lib/efi/shim/<evr>/EFI/fedora/shimx64.efi and usr/lib/efi/grub2/<evr>/EFI/fedora/grubx64.efi) or the
older usr/lib/bootupd/updates/EFI/fedora/. tools/boot-pcr4.sh copies those out of an image and runs this.

The SuiPlay firmware's PCR4 (SHA-256 bank) gets, in this order (read from a device's TPM event log, Rare OS
0.1.0, docs/ATTEST.md "Boot values"):
  EV_EFI_ACTION "Calling EFI Application from Boot Option"
  EV_SEPARATOR 00000000
  EV_EFI_BOOT_SERVICES_APPLICATION \\EFI\\fedora\\shimx64.efi   (Authenticode SHA-256, by the firmware)
  EV_EFI_BOOT_SERVICES_APPLICATION \\EFI\\fedora\\grubx64.efi   (Authenticode SHA-256, by shim)
grub measures the kernel, initrd and its config into PCR 8 and 9, not 4, so PCR4 changes only with shim or grub.
The replay over the Rare OS 0.1.1 image's binaries gives the value the device measured (baffe4de...).

Prints one policy-ready line for market attest/policy/boot.unsigned.json ({"id", "name", "value"}).
--out writes it with the binaries' versions and hashes. --check fetches the signed boot values the live
attestor accepts (https://rare.market/v1/device/attest/boot-values) and exits 3 when this PCR4 is not there.
Exit codes: 0 ok, 2 the boot chain could not be found, 3 not accepted by the attestor (or it could not be read).
"""
import argparse, glob, hashlib, json, os, struct, sys, urllib.request

ACTION = b"Calling EFI Application from Boot Option"


def authenticode(data: bytes) -> bytes:
    """PE/COFF Authenticode SHA-256 (what the firmware and shim extend for a loaded image)."""
    pe = struct.unpack_from("<I", data, 0x3C)[0]
    if data[pe:pe + 4] != b"PE\0\0":
        raise ValueError("not a PE image")
    nsec = struct.unpack_from("<H", data, pe + 6)[0]
    optsz = struct.unpack_from("<H", data, pe + 20)[0]
    opt = pe + 24
    magic = struct.unpack_from("<H", data, opt)[0]
    if magic not in (0x10B, 0x20B):
        raise ValueError("unknown optional header")
    csum = opt + 64
    cert_dir = opt + (96 if magic == 0x10B else 112) + 4 * 8
    hdr_size = struct.unpack_from("<I", data, opt + 60)[0]
    cert_off = struct.unpack_from("<I", data, cert_dir)[0]
    h = hashlib.sha256()
    h.update(data[:csum])
    h.update(data[csum + 4:cert_dir])
    h.update(data[cert_dir + 8:hdr_size])
    secs = []
    for i in range(nsec):
        s = opt + optsz + 40 * i
        raw_size, raw_ptr = struct.unpack_from("<II", data, s + 16)
        if raw_size:
            secs.append((raw_ptr, raw_size))
    done = hdr_size
    for ptr, size in sorted(secs):
        h.update(data[ptr:ptr + size])
        done += size
    end = cert_off or len(data)
    if end > done:
        h.update(data[done:end])
    return h.digest()


def read_in_root(root: str, path: str) -> bytes:
    """Reads path inside root, following symlinks inside root (absolute targets are rooted there too)."""
    p = os.path.join(root, path.lstrip("/"))
    for _ in range(16):
        if not os.path.islink(p):
            return open(p, "rb").read()
        t = os.readlink(p)
        p = os.path.join(root, t.lstrip("/")) if os.path.isabs(t) else os.path.normpath(os.path.join(os.path.dirname(p), t))
    raise ValueError(f"symlink loop at {path}")


def one(root: str, pattern: str, what: str):
    found = sorted(glob.glob(os.path.join(root, pattern)))
    if len(found) > 1:
        sys.exit(f"boot-pcr4: more than one {what} in the image: {[os.path.relpath(f, root) for f in found]}")
    return os.path.relpath(found[0], root) if found else None


def locate(root: str):
    shim = one(root, "usr/lib/efi/shim/*/EFI/fedora/shimx64.efi", "shim")
    grub = one(root, "usr/lib/efi/grub2/*/EFI/fedora/grubx64.efi", "grub2")
    if shim and grub:
        evr = lambda p: p.split("/")[4].split(":")[-1]
        return shim, grub, evr(shim), evr(grub)
    old = "usr/lib/bootupd/updates/EFI/fedora/"
    if os.path.lexists(os.path.join(root, old + "shimx64.efi")) and os.path.lexists(os.path.join(root, old + "grubx64.efi")):
        versions = {}
        try:
            meta = json.load(open(os.path.join(root, "usr/lib/bootupd/updates/EFI.json")))
            versions = {v["name"]: v["rpm_evr"].split(":")[-1] for v in meta.get("versions", [])}
        except (OSError, ValueError, KeyError):
            pass
        return old + "shimx64.efi", old + "grubx64.efi", versions.get("shim", "unknown"), versions.get("grub2", "unknown")
    print("boot-pcr4: no shimx64.efi + grubx64.efi under usr/lib/efi or usr/lib/bootupd/updates", file=sys.stderr)
    sys.exit(2)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("root")
    ap.add_argument("--image", default="")
    ap.add_argument("--out")
    ap.add_argument("--check")
    a = ap.parse_args()

    shim_p, grub_p, shim_v, grub_v = locate(a.root)
    shim, grub = read_in_root(a.root, shim_p), read_in_root(a.root, grub_p)
    digests = [hashlib.sha256(ACTION).digest(), hashlib.sha256(b"\0\0\0\0").digest(), authenticode(shim), authenticode(grub)]
    pcr = bytes(32)
    for d in digests:
        pcr = hashlib.sha256(pcr + d).digest()
    value = pcr.hex()
    entry = {
        "id": f"rare-os-shim-{shim_v}-grub-{grub_v}",
        "name": f"Rare OS boot chain: shim {shim_v} + grub {grub_v} (\\EFI\\fedora)" + (f", first in {a.image}" if a.image else ""),
        "value": value,
    }
    print(json.dumps(entry))
    if a.out:
        info = lambda p, b, v, d: {"path": "/" + p, "evr": v, "sha256": hashlib.sha256(b).hexdigest(), "authenticode": d.hex()}
        with open(a.out, "w") as f:
            json.dump({"image": a.image, "pcr4": entry, "shim": info(shim_p, shim, shim_v, digests[2]),
                       "grub": info(grub_p, grub, grub_v, digests[3])}, f, indent=2)
            f.write("\n")
    if a.check:
        try:
            # Cloudflare in front of rare.market turns away Python's default User-Agent.
            req = urllib.request.Request(a.check, headers={"User-Agent": "rare-os-image-build/1", "Accept": "application/json"})
            with urllib.request.urlopen(req, timeout=20) as r:
                live = json.load(r)
            known = {e["value"]: e["id"] for e in live["pcr4"]}
        except Exception as e:  # noqa: BLE001 - any failure to read the live policy fails the check
            print(f"boot-pcr4: could not read the attestor's boot values from {a.check}: {e}", file=sys.stderr)
            sys.exit(3)
        if value in known:
            print(f"boot-pcr4: accepted by the attestor as {known[value]}", file=sys.stderr)
        else:
            print(f"boot-pcr4: PCR4 {value} (shim {shim_v}, grub {grub_v}) is NOT in the attestor's signed boot values.\n"
                  "Devices on this image would attest at level firmware and could not claim the Rare Pass. Add the line above\n"
                  "to market attest/policy/boot.unsigned.json, sign it (scripts/sign-boot.ts) and deploy the attestor first\n"
                  "(docs/ATTEST.md, Boot values for a new base).", file=sys.stderr)
            sys.exit(3)


if __name__ == "__main__":
    main()
