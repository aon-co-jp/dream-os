#!/bin/bash
# USB/CD から起動する署名なしの GRUB(BOOTX64.EFI)を作る。WSL/Ubuntu で実行する。
#   sudo ./build-grub-efi.sh <出力先BOOTX64.EFI>
# 必要: grub-efi-amd64-bin grub-common
# 注意: 署名が無いので、PCのセキュアブートをオフにしないと起動しない。
set -euo pipefail
OUT=${1:?出力先のパスを指定}
T=$(mktemp -d)
trap 'rm -rf "$T"' EXIT
cat > "$T/embed.cfg" <<'EOC'
search --no-floppy --file --set=root /EFI/BOOT/grub.cfg
configfile /EFI/BOOT/grub.cfg
EOC
grub-mkstandalone -O x86_64-efi -o "$OUT" \
  --modules="part_gpt part_msdos fat ext2 iso9660 gzio chain search search_fs_file configfile normal echo ls loadenv efifwsetup efi_gop all_video linux" \
  "boot/grub/grub.cfg=$T/embed.cfg"
ls -l "$OUT"
