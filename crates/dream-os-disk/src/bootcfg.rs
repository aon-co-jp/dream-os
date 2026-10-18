//! ブートメニュー(GRUB)と、複数OSの同時起動(仮想マシン)用コマンドの生成。

use crate::layout::Os;

/// ESPの`/EFI/BOOT/grub.cfg`に置く設定。Windowsはブートマネージャへチェイン、
/// LinuxとDreamOSは各パーティション内の設定ファイルへ委譲する(ファイルで探すのでディスク番号に依存しない)。
pub fn grub_cfg(oses: &[Os], default: Os, timeout_s: u32) -> String {
    let mut s = String::new();
    s.push_str("# DreamOS が生成したブートメニュー (dream-os-disk)\n");
    s.push_str(&format!("set timeout={timeout_s}\n"));
    let idx = oses.iter().position(|o| *o == default).unwrap_or(0);
    s.push_str(&format!("set default={idx}\n\n"));
    for o in oses {
        match o {
            Os::Windows => s.push_str(concat!(
                "menuentry \"Windows\" {\n",
                "  insmod part_gpt\n  insmod chain\n  insmod fat\n",
                "  search --no-floppy --file --set=root /EFI/Microsoft/Boot/bootmgfw.efi\n",
                "  chainloader /EFI/Microsoft/Boot/bootmgfw.efi\n}\n\n"
            )),
            Os::Linux => s.push_str(concat!(
                "menuentry \"Linux\" {\n",
                "  insmod part_gpt\n  insmod ext2\n",
                "  search --no-floppy --file --set=root /boot/grub/grub.cfg\n",
                "  configfile /boot/grub/grub.cfg\n}\n\n"
            )),
            Os::DreamOs => s.push_str(concat!(
                "menuentry \"DreamOS Installer (partition / format disks)\" {\n",
                "  insmod part_gpt\n  insmod iso9660\n  insmod gzio\n",
                "  search --no-floppy --file --set=root /dreamos/vmlinuz\n",
                "  linux /dreamos/vmlinuz console=tty0 loglevel=3\n",
                "  initrd /dreamos/initrd.img\n}\n\n"
            )),
        }
    }
    s.push_str("menuentry \"UEFI firmware settings\" {\n  fwsetup\n}\n");
    s
}

/// 同時起動(ホストOS上で別OSを仮想マシンとして動かす)用のQEMUコマンド。
/// `host`はホストOS、`guest_image`はゲストのディスクイメージ(実パーティションは渡さない=ホストの破損防止)。
pub fn qemu_command(host: Os, guest: Os, guest_image: &str, mem_mib: u32, cpus: u32) -> String {
    let accel = match host {
        Os::Windows => "whpx",
        Os::Linux | Os::DreamOs => "kvm",
    };
    let exe = if host == Os::Windows { "qemu-system-x86_64.exe" } else { "qemu-system-x86_64" };
    let fw = match guest {
        Os::Windows => " -bios OVMF.fd",
        _ => " -bios OVMF.fd",
    };
    format!(
        "{exe} -accel {accel} -m {mem_mib} -smp {cpus} -drive file={guest_image},format=raw,if=virtio{fw} -nic user,model=virtio-net-pci -display gtk"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grub_cfg_has_each_entry_and_default_index() {
        let c = grub_cfg(&[Os::Windows, Os::Linux, Os::DreamOs], Os::Linux, 5);
        assert!(c.contains("set default=1"));
        assert!(c.contains("bootmgfw.efi") && c.contains("/boot/grub/grub.cfg") && c.contains("/dreamos/vmlinuz") && c.contains("/dreamos/initrd.img"));
    }

    #[test]
    fn qemu_uses_platform_accelerator() {
        assert!(qemu_command(Os::Windows, Os::Linux, "l.img", 4096, 2).contains("whpx"));
        assert!(qemu_command(Os::Linux, Os::Windows, "w.img", 4096, 2).contains("kvm"));
    }
}
