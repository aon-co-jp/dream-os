# DreamOS (dream-os)

**This project is at the concept stage. No code exists yet.**
This `README-English.md` — along with [`README.md`](README.md) (Japanese,
the primary language for this ecosystem), [`CLAUDE.md`](CLAUDE.md), and
[`PORTING.md`](PORTING.md) — documents the purpose, scope, and place of
this project within the ecosystem.

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

Concept stage only — no code, build config, or CI yet. The plan is to
build evidence through open-directx's ongoing completeness work first,
then define DreamOS's concrete technical scope (see the HANDOFF section
of [`CLAUDE.md`](CLAUDE.md) for details).

## Related projects

- [aon-co-jp/open-directx](https://github.com/aon-co-jp/open-directx)
- [aon-co-jp/open-cuda](https://github.com/aon-co-jp/open-cuda)
- [aon-co-jp/aruaru-llm](https://github.com/aon-co-jp/aruaru-llm)
- [aon-co-jp/RUNO](https://github.com/aon-co-jp/RUNO) — the ecosystem-wide
  meta index

## License

Undecided (concept stage).
