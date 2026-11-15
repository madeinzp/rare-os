#!/bin/bash
# Rechunks a built Rare OS image into at most MAX_LAYERS content-based layers, in place (same tag), with chunkah.
#   sudo tools/rechunk-image.sh <image in containers-storage> [max layers, default 128]
#
# Why: `podman build` leaves the base's layers plus one per Containerfile step (0.1.2: Bazzite's 256 + 16 = 272).
# bootc and rpm-ostree read the layer list through skopeo's image proxy, and on older systems (a SuiPlay that has not
# taken recent GameOS updates) that list comes back in one 32 KiB socket message (GetLayerInfo; newer ones use a pipe,
# GetLayerInfoPiped). 272 layers is about 43 KB of JSON, so those systems failed every switch with
# "failed to invoke method GetLayerInfo: serialization error" before downloading anything. 128 layers is about 20 KB.
# The file contents are not changed (chunkah only regroups files; ostree ignores mtimes), so the boot chain, labels
# and everything a device sees stay the same; only the layer split does. Layers are grouped by RPM (files no RPM owns:
# one layer per big file, the rest together), so a Rare OS update re-downloads the layers whose contents changed.
#
# Runs as root with podman (Ubuntu's 4.9 works: the image is read through an image mount, never by a newer
# containers/storage), needs about the image's size free next to the storage for the intermediate OCI layout
# (WORKDIR, default /var/tmp), and checks the result: at most MAX_LAYERS layers, the GetLayerInfo reply under the
# old 32 KiB limit, and the same config labels as the input.
set -euo pipefail
img="${1:?usage: rechunk-image.sh <image> [max layers]}"
max="${2:-128}"
CHUNKAH="${CHUNKAH:-quay.io/coreos/chunkah:v0.7.1@sha256:71e593421c5de7dcc7c8c5c50e98d3bac572055a76f399f61a491d123c20a523}"
work="$(mktemp -d "${WORKDIR:-/var/tmp}/rare-rechunk.XXXXXX")"
trap 'rm -rf "$work"' EXIT

podman image exists "$img" || { echo "rechunk: $img is not in containers-storage" >&2; exit 1; }
before="$(podman image inspect "$img" --format '{{len .RootFS.Layers}}')"
podman inspect "$img" > "$work/inspect.json"
podman pull -q "$CHUNKAH" >/dev/null

# Gzip level 1 keeps the intermediate layout small and fast; the registry push compresses again (zstd).
podman run --rm --security-opt label=disable \
    --mount "type=image,source=$img,target=/chunkah" \
    -v "$work:/work" \
    "$CHUNKAH" build --config /work/inspect.json --max-layers "$max" --prune /sysroot/ \
        --compressed --compression-level 1 --output oci:/work/oci

# Same name, new layers: the old image's own layers go (the base's stay until the caller prunes).
podman rmi -f "$img" >/dev/null
skopeo copy -q "oci:$work/oci" "containers-storage:$img"

after="$(podman image inspect "$img" --format '{{len .RootFS.Layers}}')"
echo "rechunk: $img $before -> $after layers"
(( after <= max )) || { echo "rechunk: $after layers, more than $max" >&2; exit 1; }
python3 - "$work/inspect.json" <(podman inspect "$img") "$work/oci" <<'EOF'
import json, os, sys
old = json.load(open(sys.argv[1]))[0]["Config"].get("Labels") or {}
new = json.load(open(sys.argv[2]))[0]["Config"].get("Labels") or {}
missing = {k: v for k, v in old.items() if new.get(k) != v}
if missing:
    sys.exit(f"rechunk: labels changed: {missing}")
# What an old image proxy client sends back for GetLayerInfo: one JSON reply, read into a 32 KiB buffer.
oci = sys.argv[3]
idx = json.load(open(os.path.join(oci, "index.json")))
man = json.load(open(os.path.join(oci, "blobs", *idx["manifests"][0]["digest"].split(":"))))
layers = [{"digest": l["digest"], "size": l["size"], "media_type": "application/vnd.oci.image.layer.v1.tar+zstd"}
          for l in man["layers"]]
reply = len(json.dumps({"success": True, "value": layers, "pipeid": 0, "error_message": ""}))
print(f"rechunk: GetLayerInfo reply about {reply} bytes (old clients read at most 32768)")
if reply > 28 * 1024:
    sys.exit("rechunk: the layer list is too close to the old 32 KiB GetLayerInfo limit; lower the layer count")
EOF
