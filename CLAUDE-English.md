# Development Philosophy & Environment Rules (DreamOS / dream-os)

> **Note**: This is a condensed translation of the current state. The
> full historical HANDOFF change log remains available only in
> Japanese, for brevity, in [CLAUDE.md](CLAUDE.md) — see there for
> details of individual sessions.

Working drive: `F:\runo`. This section follows the practice of
adopting the corresponding section from
[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)'s
`CLAUDE.md` as the reference. GitHub repo:
[aon-co-jp/dream-os](https://github.com/aon-co-jp/dream-os).

**Concept start: 2026-08-06** (the empty GitHub repo itself was
already created on 2026-07-01, the same pattern as `open-directx`).

## Role of this project (concept stage, only individual PoCs as code)

**DreamOS** aims for a **hybrid kernel combining Linux and TRON OS**
that would unify most of the operating systems that exist in the
world — Windows PowerShell, macOS/iPhone apps, and Android apps are
meant to each run compatibly on a single kernel. Future goals include
support for Sony PlayStation 5/6 and Nintendo Switch 2.

## Technical research (2026-08-06, honest summary)

A Japanese/English Google/GitHub research pass found **no existing
implementation** realizing "a Linux×TRON hybrid kernel natively
running apps from every platform" as originally envisioned. Most
important finding: **as of 2026 there is no practical way to run iOS
apps natively on non-Apple hardware** (Apple blocks JIT compilation
and virtualization for third-party apps system-wide). ChromeOS's
`crosvm` approach (a shared VMM foundation for Linux via Crostini and
Android via ARCVM) proved to be the most established, realistic
precedent. seL4 + Genode/Neptune OS ("microkernel + OS personality")
is another reference point. For details, including an original
analysis of mainframes (IBM z/OS / Hercules / AWS Mainframe
Modernization), see the "技術調査結果" section in [CLAUDE.md](CLAUDE.md).

## Target platforms (roadmap, as of 2026-08-17)

**Phase 1 (hardware available, starting now)**: Windows (a PC with an
NVIDIA GT730) and Android (several phones).

**Phase 2+ (deferred, pending future licensing)**: macOS/iPhone/
**iPad** (iPad added 2026-08-17), various Linux/UNIX distributions,
Sony PlayStation 5/6, Nintendo Switch 2/**3** (Switch 3 added
2026-08-17), and **IBM mainframe (z/OS) compatibility**.

**Honest disclosure**: supporting macOS/iPhone/iPad, PS5/6, and
Switch 2/3 requires non-public SDKs, NDAs, and dev hardware from each
platform holder — none of which this ecosystem currently has.
**Switch 3 has not been officially announced by Nintendo as of
2026-08-17** — its mention here is merely a placeholder for "if/when
announced," not based on real product information. For IBM
mainframes: the instruction-set emulator Hercules is OSS and
technically integrable, but running real z/OS requires an official
IBM license (not held). "AWS mainframe" turned out to be an
application-migration service (AWS Mainframe Modernization / AWS
Transform for mainframe), not an OS compatibility layer.

## Implemented PoCs (real-hardware-verified, Windows GT730 + Android
Adreno 619)

- **`crates/dream-os-kernel`** (reuses `open-cuda`'s
  `opencuda-vulkan`): a shared Windows/Android Vulkan execution
  foundation (the `vector_add` kernel), mining-OS-style power control
  (`MiningPowerProfile` + a `sha256d_mine` double-SHA256 kernel, GPU
  results verified against a CPU reference on both platforms),
  Toshiba SBM-inspired combinatorial optimization (the `sbm_ising`
  kernel, ballistic SB algorithm, GPU/CPU converge to the identical
  spin configuration), a concrete knapsack-problem application of
  `sbm_ising` (task selection, `task_scheduler.rs`), an `open-directx`
  DXBC→SPIR-V bridge (`directx_bridge.rs`), and an `open-cuda` fused
  flash-attention bridge (`flash_attention_bridge.rs`, toy size 8×16).
- **`crates/dream-os-raid-bridge`** (reuses `open-raid-z`'s
  `RaidZVdev` + `zfs_accel_hlsl`): a RAID6/Z2 parity-acceleration
  bridge, verified real GPU acceleration (no CPU fallback), an
  8-stripe roundtrip, and self-healing after simulated single-disk
  corruption, plus a one-command runnable demo.
- **`crates/dream-os-wire`** (reuses `open-web-server-wire`'s
  `SecureChannel` + `RS-SmartTCP`'s `redundant_transmission`/
  `transaction_log`): AEAD-encrypted, replay- and tamper-resistant
  result delivery for the World Laboratory concept (distributed
  volunteer computing), plus `aruaru-db` (ACID-compatible)
  persistence, verified against a real `aruaru-server` instance.

## Place within the ecosystem

Related to [`open-directx`](https://github.com/aon-co-jp/open-directx)
(cross-platform DirectX compatibility layer, currently the
top-priority project in the ecosystem), [`open-cuda`](https://github.com/aon-co-jp/open-cuda)
(CUDA-compatible GPU compute, real Vulkan backend), and
[`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm) (LLM
inference via `open-cuda`'s Vulkan kernels). The exact integration
approach remains undecided — `dream-os` is deliberately grown in
small, independently verifiable steps while completing `open-directx`
stays the priority.

## Current status

The PoCs above are real-hardware verified, but DreamOS itself (the
unified kernel, the World Laboratory coordinator) remains at the
concept/design stage. See [CLAUDE.md](CLAUDE.md) for the full
historical HANDOFF log.

---

Other languages: [日本語 (Original, with full HANDOFF history)](CLAUDE.md) ·
[Deutsch](CLAUDE-German.md) · [Italiano](CLAUDE-Italian.md) ·
[Français](CLAUDE-French.md) · [Русский](CLAUDE-Russian.md) ·
[Українська](CLAUDE-Ukrainian.md) · [עברית](CLAUDE-Hebrew.md) ·
[فارسی](CLAUDE-Persian.md) · [العربية](CLAUDE-Arabic.md)
