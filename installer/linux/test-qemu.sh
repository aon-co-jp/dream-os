#!/bin/bash
# インストーラー環境をQEMUで通し試験する(WSL/Ubuntu)。ダミーのNTFS入りNVMeディスクを縮小して、データが残ることを確認する。
#   sudo ./test-qemu.sh <build-initramfs.shの出力ディレクトリ>
set -uo pipefail
STAGE=${1:?stage dir}
W=$(mktemp -d /tmp/dreamos-qemu.XXXXXX)
cd "$W"
echo "work: $W"

truncate -s 20G disk.img
sgdisk -o disk.img >/dev/null
sgdisk -n1:2048:+100M -t1:ef00 -c1:"EFI System Partition" -n2:0:+18G -t2:0700 -c2:"Basic data partition" disk.img >/dev/null
START=$(sgdisk -i2 disk.img | sed -n 's/^First sector: \([0-9]*\).*/\1/p')
truncate -s 18G ntfs.fs
mkntfs -F -Q -L TESTWIN ntfs.fs >/dev/null 2>&1
head -c 12000000 /dev/urandom > payload.bin
ntfscp -f ntfs.fs payload.bin payload.bin >/dev/null
sha256sum payload.bin | cut -d' ' -f1 > payload.sha
dd if=ntfs.fs of=disk.img bs=1M seek=$((START * 512 / 1048576)) conv=notrunc,sparse status=none
rm ntfs.fs

mkfifo in.fifo
exec 3<>in.fifo
timeout 1500 qemu-system-x86_64 -accel tcg -m 1536 -nographic -no-reboot \
  -kernel "$STAGE/dreamos/vmlinuz" -initrd "$STAGE/dreamos/initrd.img" \
  -append "console=ttyS0 loglevel=3" \
  -drive file=disk.img,format=raw,if=none,id=d0 -device nvme,serial=dreamos0,drive=d0 \
  <&3 > qemu.log 2>&1 &
QPID=$!

clean() { sed 's/\x1b\[[0-9;]*[A-Za-z]//g' qemu.log | tr -d '\r'; }
# 出力に文字列が現れるまで待つ(第3引数: 何個目の出現まで待つか)
wait_for() {
  local text=$1 limit=${2:-300} n=${3:-1} i=0
  while [ $i -lt $limit ]; do
    [ "$(clean | grep -c -F -- "$text")" -ge "$n" ] && return 0
    kill -0 $QPID 2>/dev/null || return 1
    sleep 1; i=$((i + 1))
  done
  echo "TIMEOUT waiting for: $text"; return 1
}
send() { printf '%s\n' "$1" >&3; sleep 1; }

wait_for "Select:" 400 1 && send 2
wait_for "Disk to modify" 60 && send nvme0n1
wait_for "NTFS partition to SHRINK" 60 && send 2
wait_for "GB to take for DreamOS" 120 && send 4
wait_for "shared exFAT area" 30 && send 1
wait_for "to continue" 30 && send "SHRINK nvme0n1 2 4"
wait_for "Done. New layout" 900 || wait_for "Nothing was changed" 5 || true
sleep 2
wait_for "Press Enter" 30 1 && send ""
wait_for "Select:" 60 2 && send 5
sleep 5
kill $QPID 2>/dev/null; wait $QPID 2>/dev/null

echo "---- log (tail) ----"
clean | tail -70
echo "---- verify ----"
sgdisk -p disk.img | tail -6
NPARTS=$(sgdisk -p disk.img | awk '/^ +[0-9]+ /' | wc -l)
S2=$(sgdisk -i2 disk.img | sed -n 's/^First sector: \([0-9]*\).*/\1/p')
E2=$(sgdisk -i2 disk.img | sed -n 's/^Last sector: \([0-9]*\).*/\1/p')
dd if=disk.img of=after.fs bs=512 skip=$S2 count=$((E2 - S2 + 1)) status=none conv=sparse
ntfscat -f after.fs payload.bin 2>/dev/null | sha256sum | cut -d' ' -f1 > after.sha
OK=1
[ "$NPARTS" -eq 4 ] || { echo "FAIL: expected 4 partitions, got $NPARTS"; OK=0; }
diff -q payload.sha after.sha >/dev/null || { echo "FAIL: NTFS data mismatch"; OK=0; }
# 縮小後のNTFSが整合しているか
ntfsresize --info -f after.fs 2>&1 | grep -E "Cluster size|Current volume size|consistent|inconsistent" || true
[ $OK -eq 1 ] && echo "RESULT: PASS (4 partitions, NTFS data intact)" || echo "RESULT: FAIL"
echo "work dir kept: $W"
