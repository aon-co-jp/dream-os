# DreamOS (dream-os)

**This project is at the concept stage, but since 2026-08-06 it contains
several real-hardware-verified proof-of-concept code.** This
`README-English.md` — along with [`README.md`](README.md) (Japanese, the
primary language for this ecosystem), [`CLAUDE.md`](CLAUDE.md),
[`PORTING.md`](PORTING.md), and
[world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md) —
documents the purpose, scope, place within the ecosystem, and current
implementation status of this project.

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
iPhone, various Linux distributions, Sony PlayStation 5/6, Nintendo
Switch 2, and **IBM mainframe (z/OS family) compatibility** (the
instruction-set emulator Hercules is OSS, but running the actual OS
requires an official IBM license). "AWS mainframe" turned out to be an
application-migration service (AWS Mainframe Modernization etc.), not
an OS compatibility layer — see [`CLAUDE.md`](CLAUDE.md) for details.

**Honest disclosure**: supporting macOS/iPhone, PS5/6, and Switch 2
requires each platform holder's non-public SDK, NDA, and dev hardware,
none of which this ecosystem currently has. Per the user's 2026-08-06
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
