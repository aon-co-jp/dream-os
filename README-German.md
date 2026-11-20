# DreamOS (dream-os)

*Andere Sprachen*: [日本語 (Original)](README.md) ·
[English](README-English.md) · [Italiano](README-Italian.md) ·
[Français](README-French.md) · [Русский](README-Russian.md) ·
[Українська](README-Ukrainian.md) · [עברית](README-Hebrew.md) ·
[فارسی](README-Persian.md) · [العربية](README-Arabic.md)

> 📌 **Letzte Aktualisierung (2026-08-08)**: `dream-os-raid-bridge`
> (die open-raid-z RAID6/Z2-Brücke) hat eine lauffähige Demo
> (`examples/raid6_bridge_benchmark.rs`) erhalten, sodass sie mit
> einem einzigen Befehl (`cargo run -p dream-os-raid-bridge --example
> raid6_bridge_benchmark --release`) getestet werden kann, ohne den
> Testquellcode lesen zu müssen. Verifiziert auf echter NVIDIA-
> GT730-Hardware: Der erkannte Paritätsbeschleuniger ist tatsächlich
> `Gpu` (kein CPU-Fallback), ein 8-Stripe-Schreib-/Lese-Roundtrip
> stimmt überein, und die Brücke heilt sich nach direkter
> Beschädigung einer Disk selbst (RAID6s zwei Paritätsdisks stellen
> sie wieder her). Auch `dream-os-kernel/src/lib.rs` wurde auf
> TODO/unimplemented/`allow(dead_code)`-Markierungen geprüft — keine
> gefunden, daher war dort keine Korrektur nötig (ehrliche
> Offenlegung, kein Anspruch auf erschöpfende Prüfung).

> 📌 **Letzte Aktualisierung (2026-08-07)**: Eine neue Android-App
> (`android/`) hinzugefügt, die einen open-cuda(Vulkan-
> Geräteöffnung)+open-directx(DXBC→SPIR-V-Übersetzung)-
> Integrations-PoC demonstriert, verifiziert auf echter Hardware
> (Windows GT730 + Android Adreno 619). Der geplante Toshiba-SBM/
> DeepSeek-Integrationsplan bleibt zurückgestellt — kein konkretes
> Optimierungsziel pro Repo wurde bisher identifiziert, daher wurde
> keine spekulative Implementierung vorgenommen (siehe
> [CLAUDE.md](CLAUDE.md) für die ehrliche Aufarbeitung). Außerdem
> wurde ein früherer Dokumentationsfehler korrigiert: `sbm_ising`
> verfügt bereits über einen GPU-Dispatch-Pfad via
> `opencuda_core::GpuDevice`, bestätigt via `cargo test`.

**Dieses Projekt befindet sich in der Konzeptphase, enthält aber seit
2026-08-06 mehrere auf echter Hardware verifizierte
Proof-of-Concept-Codes.** Diese `README-German.md` — zusammen mit
[`README.md`](README.md) (Japanisch, die Hauptsprache dieses
Ökosystems), [`CLAUDE.md`](CLAUDE.md), [`PORTING.md`](PORTING.md) und
[world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md)
— dokumentiert Zweck, Umfang, Position im Ökosystem und den aktuellen
Implementierungsstand dieses Projekts.

## Zweck

Ein **Hybrid-Kernel aus Linux und TRON OS**, der darauf abzielt, die
meisten in der Welt existierenden Betriebssysteme zu vereinheitlichen.
Endziel ist, dass Windows PowerShell, macOS/iPhone-Apps und
Android-Apps jeweils mit erhaltener Kompatibilität auf einem einzigen
Kernel laufen. Zukünftige Ziele umfassen auch Unterstützung für
**Sony PlayStation 5 / PlayStation 6 und Nintendo Switch 2**
(Richtung durch Nutzeranweisung vom 2026-08-06 festgelegt).

## Technische Recherche (durchgeführt 2026-08-06, ehrliche Zusammenfassung)

Eine Google/GitHub-Recherche auf Japanisch und Englisch fand **keine
bestehende Implementierung**, die "einen Linux×TRON-Hybrid-Kernel mit
nativer Ausführung von Apps aller Plattformen" wie ursprünglich
vorgesehen realisiert. Wichtigster Befund: **Es gibt 2026 keine
praktikable Möglichkeit, iOS-Apps nativ auf Nicht-Apple-Hardware
auszuführen** (Apple selbst blockiert JIT-Kompilierung und
Virtualisierung für Drittanbieter-Apps auf Betriebssystemebene).
Andererseits erwies sich ChromeOS' Ansatz — mehrere Gast-
Betriebssysteme (Linux via Crostini, Android via ARCVM) auf einer
gemeinsamen VMM-Basis (`crosvm`) zu hosten — als das etablierteste,
realistischste Vorbild. Siehe den Abschnitt "技術調査結果"
(Ergebnisse der technischen Recherche) in [`CLAUDE.md`](CLAUDE.md)
für die vollständige Aufschlüsselung nach Komponententechnologie,
Reifegradvergleich und einer vorgeschlagenen realistischen
Architekturrichtung.

## Zielplattformen (Roadmap, aktualisiert 2026-08-06 basierend auf
tatsächlich vorhandener Hardware)

**Phase 1 (Hardware vorhanden, Beginn jetzt)**: Windows (ein PC mit
NVIDIA GT730) und Android (mehrere Smartphones).

**Phase 2+ (zurückgestellt, abhängig von künftigen Lizenzen/
Genehmigungen)**: macOS/iPhone/**iPad** (ergänzt 2026-08-17), diverse
Linux/UNIX-Distributionen, Sony PlayStation 5/6, Nintendo
Switch 2/**3** (ergänzt 2026-08-17) sowie **IBM-Mainframe(z/OS)-
Kompatibilität** (der Befehlssatz-Emulator Hercules ist OSS, doch der
Betrieb des tatsächlichen Betriebssystems erfordert eine offizielle
IBM-Lizenz). "AWS-Mainframe" erwies sich als
Anwendungsmigrationsdienst (AWS Mainframe Modernization etc.), nicht
als OS-Kompatibilitätsschicht — Details siehe [`CLAUDE.md`](CLAUDE.md).

**Ehrliche Offenlegung**: Die Unterstützung von macOS/iPhone/iPad,
PS5/6 und Switch 2/3 erfordert nicht-öffentliche SDKs, NDAs und
Entwicklerhardware der jeweiligen Plattforminhaber, die in diesem
Ökosystem derzeit alle fehlen. **Switch 3 wurde von Nintendo bis
2026-08-17 offiziell nicht angekündigt — die Erwähnung hier ist
lediglich ein Platzhalter für "falls/wenn angekündigt und
veröffentlicht", nicht auf realen Produktinformationen basierend**
(klar formuliert, um keine übertriebenen Ansprüche zu erheben). Gemäß
der Nutzerentscheidung vom 2026-08-06 werden diese unter der Prämisse
zukünftiger Lizenzierung zurückgestellt — die Arbeit beginnt jetzt mit
den Plattformen, für die tatsächlich Hardware vorhanden ist (Details
im Abschnitt "スコープの絞り込み" in [`CLAUDE.md`](CLAUDE.md)).

## Implementierte PoCs (2026-08-06, real-hardware-verifiziert)

`crates/dream-os-kernel` (nutzt `open-cuda`s `opencuda-vulkan`
wieder):

- **Gemeinsame Windows/Android-Vulkan-Ausführungsbasis**: Der
  `vector_add`-Kernel wurde sowohl auf einem Windows-PC (NVIDIA
  GT730) als auch auf einem echten Android-Smartphone (Moto G53Y 5G,
  Adreno 619) tatsächlich ausgeführt und verifiziert.
- **Mining-OS-artige Leistungssteuerung**: `MiningPowerProfile`
  (softwareseitige Duty-Cycle-Drosselung) + ein `sha256d_mine`-Kernel
  (Double-SHA256). GPU-Ergebnisse wurden byteweise gegen eine
  CPU-Referenz (das `sha2`-Crate) auf beiden Plattformen verifiziert.
  Ein Batch-Splitting-Design wurde ergänzt, um einen auf dem echten
  Android-Gerät entdeckten Mobile-GPU-Treiber-Timeout (TDR) zu
  umgehen.
- **An Toshibas Simulated Bifurcation Machine (SBM) angelehnte
  quantenannealing-artige kombinatorische Optimierung**: Ein
  `sbm_ising`-Kernel (der ballistische SB-Algorithmus, Minimierung
  eines Ising-Modells). Auf echter Windows-Hardware verifiziert,
  dass der GPU-Lauf zur exakt gleichen Spin-Konfiguration wie eine
  CPU-Referenzimplementierung konvergiert.

`crates/dream-os-wire` (nutzt `open-web-server-wire`s
`SecureChannel` wieder):

- **Schutz der Ergebnisübermittlung für das World-Laboratory-Konzept**
  (BOINC-artiges verteiltes Freiwilligen-Computing): AEAD-
  Verschlüsselung + Replay-Angriffsschutz via `WorkerChannel`/
  `CoordinatorChannel`, die manipulierte oder wiederholte Worker-
  Ergebnisse erkennen und ablehnen. Mit drei echten kryptografischen
  Testszenarien verifiziert (normaler Roundtrip, Replay-Ablehnung,
  Manipulations-Ablehnung). Siehe
  [world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md)
  für das vollständige Design.

## Position im Ökosystem

Teil des `aon-co-jp`-Ökosystems, insbesondere verbunden mit (genaue
Integrationsweise noch offen — Details siehe [`CLAUDE.md`](CLAUDE.md)):

- [`open-directx`](https://github.com/aon-co-jp/open-directx) — eine
  plattformübergreifende DirectX-Kompatibilitätsschicht, die
  bestehende Windows-only-DirectX-Apps/-Spiele auf Linux, Android und
  künftig macOS/PlayStation laufen lässt. **Derzeit das im Ökosystem
  mit höchster Priorität verfolgte Projekt hinsichtlich
  Vervollständigung.**
- [`open-cuda`](https://github.com/aon-co-jp/open-cuda) — eine
  CUDA-kompatible GPU-Berechnungsbasis (echter Vulkan-Backend).
- [`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm) — LLM-
  Inferenz, unter Nutzung von `open-cuda`s Vulkan-Kernel.

## Aktueller Stand

Die oben genannten PoCs sind auf echter Hardware verifiziert, aber
DreamOS selbst (der vereinheitlichte Kernel, der Koordinator) befindet
sich weiterhin in der Konzept-/Entwurfsphase. Die Vervollständigung
von open-directx bleibt oberste Priorität, während `dream-os` daneben
in kleinen, unabhängig verifizierbaren Schritten wächst (Details siehe
den HANDOFF-Abschnitt in [`CLAUDE.md`](CLAUDE.md)).

## Verwandte Projekte

- [aon-co-jp/open-directx](https://github.com/aon-co-jp/open-directx)
- [aon-co-jp/open-cuda](https://github.com/aon-co-jp/open-cuda)
- [aon-co-jp/aruaru-llm](https://github.com/aon-co-jp/aruaru-llm)
- [aon-co-jp/RUNO](https://github.com/aon-co-jp/RUNO) — der
  Meta-Index des gesamten Ökosystems

## License

Unentschieden (Konzeptphase).
