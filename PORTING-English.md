# PORTING.md — dream-os porting guide (condensed)

> **Note**: This is a condensed translation. The full file-tree/path-
> dependency guide remains available only in Japanese (mixed with
> English) in [PORTING.md](PORTING.md) — consult that before actually
> porting.

**Scope**: `dream-os` is a concept-stage repository for an application
service layer / GPU abstraction layer for Windows, macOS, Android, and
various Linux distributions (with Sony PlayStation 5/6 and Nintendo
Switch 2 as future goals).

Summary of reusable implementation patterns from this project, in
case they get ported to another project:

1. **Reusing an existing cross-platform Vulkan backend**
   (`open-cuda`'s `opencuda-vulkan`) as a shared execution layer
   instead of writing a new kernel from scratch — the path-dependency
   reuse pattern that `dream-os-kernel` uses throughout.
2. **Software-side duty-cycle power control** (`MiningPowerProfile`)
   as a vendor-neutral alternative to NVML-style power-limit APIs
   when the target GPU doesn't expose them (confirmed on an NVIDIA
   GT730: all power-management fields return `N/A`).
3. **QUBO→Ising embedding via an ancilla spin** to carry the local
   field term (h) when a fixed-size Ising kernel (`sbm_ising`, 64
   spins) implements only the coupling term (J) — and always check
   the energy sign convention (E vs. -E) against the actual kernel
   dynamics, not just the documentation; a sign mistake produced a
   seemingly stable but wrong result that happened to pass the
   CPU unit tests.
4. **Avoiding Android mobile-GPU-driver TDR (device-lost) via batch
   splitting** (breaking large dispatches into smaller sub-batches) —
   desktop GPUs tolerate large single dispatches, mobile GPUs don't
   necessarily.
5. **AEAD encryption + replay/tamper protection** (`SecureChannel`)
   for untrusted worker→coordinator result submission in distributed
   volunteer-computing designs, plus a durable write-ahead log +
   redundant multi-path send wrapper (`RS-SmartTCP`'s
   `transaction_log` + `redundant_transmission`) for "never lose data
   even if every send path fails."
6. **Offloading GF(2^8) Reed-Solomon RAID6/Z2 parity computation to
   GPU/NPU** (D3D12/DirectML on Windows, Vulkan elsewhere) via the
   existing `open-raid-z` infrastructure — including automatic
   accelerator detection with a CPU fallback.
7. **Shipping shader binaries via `include_bytes!` instead of runtime
   file loading**, for cross-platform executables (avoids Android
   `adb push` path issues with loose `.spv` files).

**Important caveat**: this project is a pre-alpha concept/PoC
repository — the "hybrid kernel" vision itself is not implemented,
only isolated GPU/RAID/networking PoCs exist. Disclose this clearly
whenever porting.

---

Other languages: [日本語 (Original, full details)](PORTING.md) ·
[Deutsch](PORTING-German.md) · [Italiano](PORTING-Italian.md) ·
[Français](PORTING-French.md) · [Русский](PORTING-Russian.md) ·
[Українська](PORTING-Ukrainian.md) · [עברית](PORTING-Hebrew.md) ·
[فارسی](PORTING-Persian.md) · [العربية](PORTING-Arabic.md)
