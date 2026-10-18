#!/bin/bash
# DreamOS インストーラー環境(カーネル + initramfs)を作る。WSL/Ubuntuで実行する。
#   sudo ./build-initramfs.sh <出力ディレクトリ>
# 出力: <出力>/dreamos/vmlinuz, <出力>/dreamos/initrd.img
# 必要: linux-image-generic busybox-static kmod cpio ntfs-3g gdisk e2fsprogs dosfstools exfatprogs util-linux
set -euo pipefail
OUT=${1:?出力ディレクトリを指定}
HERE=$(cd "$(dirname "$0")" && pwd)
V=${KVER:-$(ls /lib/modules | sort -V | tail -1)}
ROOT=$(mktemp -d)
trap 'rm -rf "$ROOT"' EXIT

mkdir -p "$ROOT"/usr/{bin,sbin,lib,lib64} "$ROOT"/{proc,sys,dev,tmp,run,mnt,etc}
ln -s usr/bin "$ROOT/bin"; ln -s usr/sbin "$ROOT/sbin"; ln -s usr/lib "$ROOT/lib"; ln -s usr/lib64 "$ROOT/lib64"

copybin() {
  local src; src=$(command -v "$1") || { echo "見つかりません: $1" >&2; exit 1; }
  src=$(readlink -f "$src")
  local dst="$ROOT/usr/bin/$1"
  case "$1" in mkfs.*|ntfsresize|sgdisk) dst="$ROOT/usr/sbin/$1" ;; esac
  cp -L "$src" "$dst"
  ldd "$src" | awk '/=>/ {print $3} /ld-linux/ {print $1}' | while read -r l; do
    [ -f "$l" ] || continue
    local real; real=$(readlink -f "$l")
    mkdir -p "$ROOT$(dirname "$l")"
    cp -L "$real" "$ROOT$l" 2>/dev/null || true
  done
}
for t in ntfsresize sgdisk mkfs.ext4 mkfs.exfat mkfs.vfat blkid lsblk kmod; do copybin "$t"; done
ln -s kmod "$ROOT/usr/bin/modprobe"; ln -s kmod "$ROOT/usr/bin/insmod"; ln -s kmod "$ROOT/usr/bin/depmod"
[ -f /etc/mke2fs.conf ] && cp /etc/mke2fs.conf "$ROOT/etc/"

# busybox: 上でコピーした本物のコマンドが無いものだけをシンボリックリンクで補う
cp -L /bin/busybox "$ROOT/usr/bin/busybox"
for a in $("$ROOT/usr/bin/busybox" --list); do
  [ -e "$ROOT/usr/bin/$a" ] || [ -e "$ROOT/usr/sbin/$a" ] || ln -s busybox "$ROOT/usr/bin/$a"
done

# 必要なカーネルモジュール(依存込み)
MODSRC=/lib/modules/$V
mkdir -p "$ROOT/usr/lib/modules/$V"
for m in nvme ahci usb_storage uas hid_generic usbhid exfat ntfs3 ext4 vfat; do
  modprobe -S "$V" --show-depends "$m" 2>/dev/null | awk '/^insmod/ {print $2}' | while read -r f; do
    [ -f "$f" ] || continue
    rel=${f#"$MODSRC"/}
    mkdir -p "$ROOT/usr/lib/modules/$V/$(dirname "$rel")"
    cp "$f" "$ROOT/usr/lib/modules/$V/$rel"
  done
done
for f in modules.builtin modules.builtin.modinfo modules.order; do
  [ -f "$MODSRC/$f" ] && cp "$MODSRC/$f" "$ROOT/usr/lib/modules/$V/"
done
depmod -b "$ROOT" "$V"

install -m 0755 "$HERE/init" "$ROOT/init"
install -m 0755 "$HERE/dreamos-install" "$ROOT/usr/bin/dreamos-install"

mkdir -p "$OUT/dreamos"
cp "/boot/vmlinuz-$V" "$OUT/dreamos/vmlinuz"
(cd "$ROOT" && find . | cpio -o -H newc --quiet | gzip -9) > "$OUT/dreamos/initrd.img"
chmod 644 "$OUT/dreamos/vmlinuz" "$OUT/dreamos/initrd.img"
ls -l "$OUT/dreamos"
echo "kernel $V, modules: $(find "$ROOT/usr/lib/modules/$V" -name '*.ko*' | wc -l)"
