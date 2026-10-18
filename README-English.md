# DreamOS (dream-os)

*日本語*: [README.md](README.md) ·
*Other languages*: [Deutsch](README-German.md) · [Italiano](README-Italian.md) ·
[Français](README-French.md) · [Русский](README-Russian.md) ·
[Українська](README-Ukrainian.md) · [עברית](README-Hebrew.md) ·
[فارسی](README-Persian.md) · [العربية](README-Arabic.md)

> 📌 **Recent update (2026-08-08)**: Added a runnable demo
> (`examples/raid6_bridge_benchmark.rs`) to `dream-os-raid-bridge` (the
> open-raid-z RAID6/Z2 bridge), so it can be exercised with a single
> command (`cargo run -p dream-os-raid-bridge --example
> raid6_bridge_benchmark --release`) instead of having to read test
> source. Verified on real NVIDIA GT730 hardware: the detected parity
> accelerator is actually `Gpu` (not a CPU fallback), an 8-stripe
> write/read roundtrip matches, and the bridge self-heals after directly
> corrupting one disk (RAID6's two parity disks recover it). Also
> checked `dream-os-kernel/src/lib.rs` for TODO/unimplemented/
> `allow(dead_code)` markers — found none, so no fix was needed there
> (honest disclosure, not a claim of exhaustive audit).

> 📌 **Recent update (2026-08-07)**: added a new Android app (`android/`)
> demonstrating an open-cuda (Vulkan device open) + open-directx
> (DXBC→SPIR-V translation) integration PoC, verified on real hardware
> (Windows GT730 + Android Adreno 619). The Toshiba SBM/DeepSeek
> integration plan remains pending — no concrete per-repo optimization
> target has been identified yet, so no speculative implementation was
> made (see [CLAUDE.md](CLAUDE.md) for the honest write-up). Also
> corrected an earlier documentation error: `sbm_ising` already has a
> GPU dispatch path via `opencuda_core::GpuDevice`, confirmed via
> `cargo test`.

**This project is at the concept stage, but since 2026-08-06 it contains
several real-hardware-verified proof-of-concept code.** This
`README-English.md` — along with [`README.md`](README.md) (Japanese, the
primary language for this ecosystem), [`CLAUDE.md`](CLAUDE.md),
[`PORTING.md`](PORTING.md), and
[world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md) —
documents the purpose, scope, place within the ecosystem, and current
implementation status of this project.

> 📌 **Hand-off (2026-10-18)**: the real-USB write test of `dream-os-disk` is unfinished. Resume steps, unverified items and cautions are at the end of [`PORTING.md`](PORTING.md) ("Hand-off", Japanese and English).

## Purpose

A **hybrid kernel combining Linux and TRON OS**, aiming to unify most of
the operating systems that exist in the world. The end goal is for
Windows PowerShell, macOS/iPhone apps, and Android apps to all run with
their respective compatibility preserved on a single kernel. Future goals
also include support for **Sony PlayStation 5 / PlayStation 6 and
Nintendo Switch 2** (direction set by the user on 2026-08-06).

## Technical research (conducted 2026-08-06, honest summary)

A Google/GitHub research pass in Japanese and English found **no existing
implementation that achieves "a Linux×TRON hybrid kernel natively running
apps from every platform" as originally envisioned**. The most important
finding: **there is currently no practical way, as of 2026, to run iOS
apps natively on non-Apple hardware** (Apple itself blocks JIT
compilation and virtualization for third-party apps at the OS level).
On the other hand, ChromeOS's approach — hosting multiple guest OSes
(Linux via Crostini, Android via ARCVM) on a shared VMM foundation
(`crosvm`) — turned out to be the most proven, realistic precedent. See
the "技術調査結果" (technical research findings) section of
[`CLAUDE.md`](CLAUDE.md) for the full breakdown by component technology,
maturity comparison, and a proposed realistic architecture direction.

## Target platforms (roadmap, updated 2026-08-06 based on actual hardware on hand)

**Phase 1 (hardware available, starting now)**: Windows (a PC with an
NVIDIA GT730) and Android (several phones).

**Phase 2+ (deferred, pending future licensing/permission)**: macOS/
iPhone/**iPad** (added 2026-08-17), various Linux/UNIX distributions,
Sony PlayStation 5/6, Nintendo Switch 2/**3** (added 2026-08-17), and
**IBM mainframe (z/OS family) compatibility** (the instruction-set
emulator Hercules is OSS, but running the actual OS requires an
official IBM license). "AWS mainframe" turned out to be an
application-migration service (AWS Mainframe Modernization etc.), not
an OS compatibility layer — see [`CLAUDE.md`](CLAUDE.md) for details.

**Honest disclosure**: supporting macOS/iPhone/iPad, PS5/6, and
Switch 2/3 requires each platform holder's non-public SDK, NDA, and dev
hardware, none of which this ecosystem currently has. **Switch 3 in
particular has not been officially announced by Nintendo as of
2026-08-17 — its inclusion here is only a placeholder for "if/when it
is announced and released," not based on any real product information**
(stated plainly to avoid overclaiming). Per the user's 2026-08-06
decision, these are deferred on the premise of obtaining licensing later
— work starts now with the platforms we actually have hardware for
(see the "スコープの絞り込み" section of [`CLAUDE.md`](CLAUDE.md) for
details).

## Implemented PoCs (2026-08-06, real-hardware verified)

`crates/dream-os-kernel` (reuses `open-cuda`'s `opencuda-vulkan`):

- **Shared Windows/Android Vulkan execution backend**: the `vector_add`
  kernel was actually dispatched and verified on both a Windows PC
  (NVIDIA GT730) and a real Android phone (Moto G53Y 5G, Adreno 619).
- **Mining-OS-style power output control**: `MiningPowerProfile`
  (software-side duty-cycle throttling) + a `sha256d_mine` kernel
  (double-SHA256). GPU results were verified byte-for-byte against a
  CPU reference (the `sha2` crate) on both Windows and Android hardware.
  A batch-splitting design was added to work around a mobile GPU
  driver timeout (TDR) discovered on the real Android device.
- **Toshiba Simulated Bifurcation Machine (SBM)-inspired quantum-
  annealing-style combinatorial optimization**: an `sbm_ising` kernel
  (the ballistic SB algorithm, minimizing an Ising model). Verified on
  real Windows hardware that the GPU run converges to the exact same
  spin configuration as a CPU reference implementation.

`crates/dream-os-wire` (reuses `open-web-server-wire`'s `SecureChannel`):

- **Result-submission protection for the World Laboratory concept**
  (BOINC-style volunteer distributed computing): AEAD encryption +
  replay-attack protection via `WorkerChannel`/`CoordinatorChannel`,
  detecting and rejecting tampered or replayed worker results. Verified
  with three real cryptographic test scenarios (normal round-trip,
  replay rejection, tamper rejection). See
  [world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md)
  for the full design.

## Disk formatting, multi-boot and installer media (2026-10-18, `dream-os-disk`)

`crates/dream-os-disk` (zero dependencies) adds: **HDD formatting** (GPT + protective MBR, FAT32 for the EFI System Partition and a shared partition; NTFS/ext4/swap partitions are created but left for the Windows/Linux installers to format); **Windows + Linux + DreamOS side by side** (layout planner, GRUB boot menu that chain-loads the Windows boot manager; "simultaneous running" = generated QEMU commands, `whpx` on Windows hosts, `kvm` on Linux); **USB/CD installer media** (`make-iso` builds a UEFI-bootable ISO9660 + Joliet + El Torito image that can be burned to CD/DVD *and* copied to a USB stick; `installer/windows/make-usb.ps1` selects only USB-bus, non-system disks, dry-run by default, and `-BurnCd` opens Windows' disc burner).

Safety: `format --device` needs an exact `--confirm "ERASE <path> <bytes>"` and refuses `PhysicalDrive0` and mounted devices. Status: 15 unit tests pass; GPT checked with `sfdisk --verify`, FAT32 with `fsck.vfat` (clean) and `mtools`, ISO by mounting it on Windows (Japanese long names read correctly). **Not verified**: writing to a real USB stick, `format --device` on a real disk, burning a disc, UEFI boot on hardware. DreamOS itself is not a bootable OS yet, so the UEFI bootloader (e.g. GRUB) must be supplied with `--bootloader`; Windows/Linux themselves are installed by their own official installers (user-supplied ISOs).
Other-language READMEs have not been updated yet.

> 📌 **DreamOS installer environment (2026-10-18)**: choosing "DreamOS Installer" in GRUB on the USB/CD boots a text installer that shrinks a disk's NTFS partition (data kept) and creates DreamOS partitions. See the end of [`PORTING.md`](PORTING.md). There is still no DreamOS system itself.

## Place within the ecosystem

Part of the `aon-co-jp` ecosystem, related in particular to (exact
integration approach undecided — see [`CLAUDE.md`](CLAUDE.md) for
details):

- [`open-directx`](https://github.com/aon-co-jp/open-directx) — a
  cross-platform DirectX compatibility layer that runs existing
  Windows-only DirectX apps/games on Linux, Android, and eventually
  macOS/PlayStation. **Currently the top-priority project in this
  ecosystem for improving completeness.**
- [`open-cuda`](https://github.com/aon-co-jp/open-cuda) — a
  CUDA-compatible GPU compute foundation (real Vulkan backend).
- [`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm) — LLM
  inference, using open-cuda's Vulkan kernels.

## Current status

The PoCs above are real-hardware verified, but DreamOS itself (the
unified kernel, the coordinator) is still at the concept/design stage.
open-directx's completeness remains the top priority, with dream-os
grown in small, independently verifiable increments alongside it (see
the HANDOFF section of [`CLAUDE.md`](CLAUDE.md) for details).

## Related projects

- [aon-co-jp/open-directx](https://github.com/aon-co-jp/open-directx)
- [aon-co-jp/open-cuda](https://github.com/aon-co-jp/open-cuda)
- [aon-co-jp/aruaru-llm](https://github.com/aon-co-jp/aruaru-llm)
- [aon-co-jp/RUNO](https://github.com/aon-co-jp/RUNO) — the ecosystem-wide
  meta index

## License

Undecided (concept stage).
