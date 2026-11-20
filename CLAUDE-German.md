# Entwicklungsphilosophie & Umgebungsregeln (DreamOS / dream-os)

> **Hinweis**: Dies ist eine kondensierte Übersetzung des aktuellen
> Zustands. Das ausführliche historische HANDOFF-Änderungsprotokoll
> bleibt aus Gründen der Kürze nur auf Japanisch in
> [CLAUDE.md](CLAUDE.md) verfügbar — siehe dort für Details zu
> einzelnen Sitzungen.

Arbeitslaufwerk: `F:\runo`. Dieser Abschnitt folgt der Praxis, den
entsprechenden Abschnitt aus dem `CLAUDE.md` von
[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z) als
Referenz zu übernehmen. GitHub-Repo:
[aon-co-jp/dream-os](https://github.com/aon-co-jp/dream-os).

**Konzeptbeginn: 2026-08-06** (das leere GitHub-Repo selbst wurde
bereits am 2026-07-01 angelegt, gleiches Muster wie bei
`open-directx`).

## Rolle dieses Projekts (Konzeptphase, nur einzelne PoCs als Code)

**DreamOS** strebt einen **Hybrid-Kernel aus Linux und TRON OS** an,
der die meisten in der Welt existierenden Betriebssysteme
vereinheitlichen soll — Windows PowerShell, macOS/iPhone-Apps und
Android-Apps sollen auf einem einzigen Kernel jeweils kompatibel
laufen. Zukünftige Ziele umfassen Unterstützung für Sony
PlayStation 5/6 und Nintendo Switch 2.

## Technische Recherche (2026-08-06, ehrliche Zusammenfassung)

Eine japanisch-englische Google/GitHub-Recherche fand **keine
bestehende Implementierung**, die "einen Linux×TRON-Hybrid-Kernel mit
nativer Ausführung von Apps aller Plattformen" wie ursprünglich
vorgesehen realisiert. Wichtigster Befund: **Es gibt 2026 keine
praktikable Möglichkeit, iOS-Apps nativ auf Nicht-Apple-Hardware
auszuführen** (Apple blockiert JIT-Kompilierung und Virtualisierung
für Drittanbieter-Apps systemweit). ChromeOS' `crosvm`-Ansatz
(gemeinsame VMM-Basis für Linux via Crostini und Android via ARCVM)
erwies sich als das etablierteste, realistischste Vorbild. seL4 +
Genode/Neptune OS ("Mikrokernel + OS-Persönlichkeit") ist ein weiterer
Referenzpunkt. Details inkl. einer eigenen Analyse zu Mainframes
(IBM z/OS / Hercules / AWS Mainframe Modernization) siehe Abschnitt
"技術調査結果" in [CLAUDE.md](CLAUDE.md).

## Zielplattformen (Roadmap, Stand 2026-08-17)

**Phase 1 (Hardware vorhanden, Beginn jetzt)**: Windows (PC mit
NVIDIA GT730) und Android (mehrere Smartphones).

**Phase 2+ (zurückgestellt, abhängig von künftigen Lizenzen)**:
macOS/iPhone/**iPad** (iPad ergänzt 2026-08-17), diverse Linux/
UNIX-Distributionen, Sony PlayStation 5/6, Nintendo Switch 2/**3**
(Switch 3 ergänzt 2026-08-17), sowie **IBM-Mainframe(z/OS)-
Kompatibilität**.

**Ehrliche Offenlegung**: Die Unterstützung von macOS/iPhone/iPad,
PS5/6 und Switch 2/3 erfordert nicht-öffentliche SDKs, NDAs und
Entwicklerhardware der jeweiligen Plattforminhaber — nichts davon ist
in diesem Ökosystem derzeit vorhanden. **Switch 3 wurde von Nintendo
bis 2026-08-17 offiziell nicht angekündigt** — die Erwähnung hier ist
lediglich ein Platzhalter für "falls/wenn angekündigt", nicht auf
realen Produktinformationen basierend. Für IBM-Mainframes: Der
Befehlssatz-Emulator Hercules ist OSS und technisch integrierbar, doch
der Betrieb von echtem z/OS erfordert eine offizielle IBM-Lizenz
(nicht vorhanden). "AWS-Mainframe" erwies sich als
Anwendungsmigrationsdienst (AWS Mainframe Modernization / AWS
Transform for mainframe), nicht als OS-Kompatibilitätsschicht.

## Implementierte PoCs (real-hardware-verifiziert, Windows GT730 +
Android Adreno 619)

- **`crates/dream-os-kernel`** (nutzt `open-cuda`s `opencuda-vulkan`
  wieder): gemeinsame Windows/Android-Vulkan-Ausführungsbasis
  (`vector_add`-Kernel), Mining-OS-artige Leistungssteuerung
  (`MiningPowerProfile` + `sha256d_mine`-Double-SHA256-Kernel, GPU-
  Ergebnisse gegen CPU-Referenz auf beiden Plattformen verifiziert),
  an Toshibas SBM angelehnte kombinatorische Optimierung
  (`sbm_ising`-Kernel, ballistischer SB-Algorithmus, GPU/CPU
  konvergieren zur identischen Spin-Konfiguration), eine konkrete
  Rucksackproblem-Anwendung von `sbm_ising` (Aufgabenauswahl,
  `task_scheduler.rs`), eine `open-directx` DXBC→SPIR-V-Brücke
  (`directx_bridge.rs`) sowie eine `open-cuda` Fused-Flash-Attention-
  Brücke (`flash_attention_bridge.rs`, Spielzeuggröße 8×16).
- **`crates/dream-os-raid-bridge`** (nutzt `open-raid-z`s
  `RaidZVdev` + `zfs_accel_hlsl` wieder): RAID6/Z2-Paritäts-
  beschleunigungsbrücke, verifiziert echte GPU-Beschleunigung (kein
  CPU-Fallback), 8-Stripe-Roundtrip und Selbstheilung nach simulierter
  Beschädigung einer Festplatte, plus eine mit einem Befehl lauffähige
  Demo.
- **`crates/dream-os-wire`** (nutzt `open-web-server-wire`s
  `SecureChannel` + `RS-SmartTCP`s `redundant_transmission`/
  `transaction_log` wieder): AEAD-verschlüsselte, replay- und
  manipulationsresistente Ergebnisübermittlung für das World-
  Laboratory-Konzept (verteiltes Freiwilligen-Computing), plus
  `aruaru-db`(ACID-kompatible)-Persistenz, verifiziert gegen eine
  echte `aruaru-server`-Instanz.

## Position im Ökosystem

Bezug zu [`open-directx`](https://github.com/aon-co-jp/open-directx)
(plattformübergreifende DirectX-Kompatibilitätsschicht, derzeit
höchste Priorität im Ökosystem), [`open-cuda`](https://github.com/aon-co-jp/open-cuda)
(CUDA-kompatible GPU-Berechnung, echter Vulkan-Backend) und
[`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm) (LLM-
Inferenz über `open-cuda`s Vulkan-Kernel). Die genaue
Integrationsweise ist noch offen — `dream-os` wird bewusst in
kleinen, unabhängig verifizierbaren Schritten ausgebaut, während die
Vervollständigung von `open-directx` Priorität hat.

## Aktueller Stand

Die oben genannten PoCs sind auf echter Hardware verifiziert, aber
DreamOS selbst (der vereinheitlichte Kernel, der World-Laboratory-
Koordinator) befindet sich weiterhin in der Konzept-/Entwurfsphase.
Das vollständige historische HANDOFF-Protokoll siehe
[CLAUDE.md](CLAUDE.md).

---

Weitere Sprachen: [日本語 (Original, mit vollständigem HANDOFF-Verlauf)](CLAUDE.md) ·
[English](CLAUDE-English.md) · [Italiano](CLAUDE-Italian.md) ·
[Français](CLAUDE-French.md) · [Русский](CLAUDE-Russian.md) ·
[Українська](CLAUDE-Ukrainian.md) · [עברית](CLAUDE-Hebrew.md) ·
[فارسی](CLAUDE-Persian.md) · [العربية](CLAUDE-Arabic.md)
