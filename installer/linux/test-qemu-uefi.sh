#!/bin/bash
# 作成したISOを、仮想UEFI(OVMF)のCDとして起動し、GRUBメニュー → DreamOSインストーラーが起動するかを確認する。
#   sudo ./test-qemu-uefi.sh <dreamos.iso> [待ち秒数]
# 結果: 作業ディレクトリに shot.png(インストーラー画面のスクリーンショット)
set -uo pipefail
ISO=$(readlink -f "${1:?iso}")
WAIT=${2:-200}
W=$(mktemp -d /tmp/dreamos-uefi.XXXXXX)
cd "$W"
cp /usr/share/OVMF/OVMF_VARS_4M.fd vars.fd 2>/dev/null || cp /usr/share/OVMF/OVMF_VARS.fd vars.fd
mkfifo in.fifo
exec 3<>in.fifo
timeout 900 qemu-system-x86_64 -accel tcg -m 2048 -display none -serial stdio \
  -monitor unix:mon.sock,server,nowait -no-reboot \
  -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE_4M.fd \
  -drive if=pflash,format=raw,file=vars.fd \
  -cdrom "$ISO" -boot d \
  <&3 > qemu.log 2>&1 &
QPID=$!
clean() { sed 's/\x1b\[[0-9;?]*[A-Za-z]//g' qemu.log | tr -d '\r'; }
wait_for() {
  local text=$1 limit=${2:-300} i=0
  while [ $i -lt $limit ]; do
    clean | grep -q -F -- "$text" && return 0
    kill -0 $QPID 2>/dev/null || return 1
    sleep 1; i=$((i + 1))
  done
  echo "TIMEOUT waiting for: $text"; return 1
}
if wait_for "DreamOS Installer" 400; then
  sleep 1
  printf '\033[B' >&3; sleep 1   # 下矢印で DreamOS の項目へ
  printf '\n' >&3                # Enter
fi
sleep "$WAIT"
python3 "$(dirname "$(readlink -f "$0")")/shot.py" mon.sock shot.ppm shot.png
kill $QPID 2>/dev/null; wait $QPID 2>/dev/null
echo "work: $W"
