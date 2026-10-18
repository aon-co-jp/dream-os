#!/bin/bash
# Ubuntuカーネルの設定を見て、インストーラー用initramfsに入れるべきドライバを確認する(開発用)
V=${1:-$(ls /lib/modules | sort -V | tail -1)}
c=/boot/config-$V
echo "kernel $V"
for k in BLK_DEV_NVME SATA_AHCI USB_XHCI_PCI USB_STORAGE USB_UAS VFAT_FS EXFAT_FS NTFS3_FS EXT4_FS \
         FRAMEBUFFER_CONSOLE DRM_SIMPLEDRM SYSFB_SIMPLEFB FB_EFI HID_GENERIC USB_HID VIRTIO_BLK VIRTIO_PCI \
         BLK_DEV_SD ATA_PIIX DEVTMPFS KEYBOARD_ATKBD DRM FB_SIMPLE MODULE_COMPRESS_ZSTD MODULE_COMPRESS_XZ; do
  printf '%-24s %s\n' "$k" "$(grep -E "^CONFIG_$k=" "$c" | cut -d= -f2)"
done
find /lib/modules/$V -name 'simpledrm*' -o -name 'nvme.ko*' -o -name 'xhci-pci.ko*' | head
ls -l /bin/busybox /usr/bin/kmod 2>&1
