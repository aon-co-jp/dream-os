# DreamOS (dream-os)

**This project is at the concept stage. No code exists yet.**
This `README-English.md` — along with [`README.md`](README.md) (Japanese,
the primary language for this ecosystem), [`CLAUDE.md`](CLAUDE.md), and
[`PORTING.md`](PORTING.md) — documents the purpose, scope, and place of
this project within the ecosystem.

## Purpose

An application service layer / GPU abstraction layer targeting Windows,
macOS, Android, and various Linux distributions. Future goals include
support for **Sony PlayStation 5 / PlayStation 6 and Nintendo Switch 2**
(direction set by the user on 2026-08-06).

## Target platforms (roadmap, in priority order)

1. Windows
2. macOS
3. Android
4. Various Linux distributions
5. (future) Sony PlayStation 5 / PlayStation 6
6. (future) Nintendo Switch 2

**Honest disclosure**: supporting PS5/6 and Switch 2 requires each
platform holder's non-public SDK, NDA, and dev hardware, none of which
this ecosystem currently has. This is a stated direction, not a
committed timeline or confirmed feasibility.

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
