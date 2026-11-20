# PORTING.md — Leitfaden zur Portierung von dream-os (Kurzfassung)

> **Hinweis**: Dies ist eine kondensierte Übersetzung. Die
> vollständige Dateibaum-/Pfadabhängigkeits-Anleitung bleibt nur auf
> Japanisch (und Englisch gemischt) in [PORTING.md](PORTING.md)
> verfügbar — dort nachschlagen, bevor tatsächlich portiert wird.

**Umfang**: `dream-os` ist ein Konzeptphasen-Repository für eine
Anwendungsdienstschicht/GPU-Abstraktionsschicht für Windows, macOS,
Android und diverse Linux-Distributionen (mit Sony PlayStation 5/6
und Nintendo Switch 2 als Zukunftszielen).

Zusammenfassung der wiederverwendbaren Implementierungsmuster aus
diesem Projekt, falls sie in ein anderes Projekt portiert werden:

1. **Wiederverwendung bestehender plattformübergreifender Vulkan-
   Backends** (`open-cuda`s `opencuda-vulkan`) als gemeinsame
   Ausführungsschicht statt neuer Kernel von Grund auf — das
   Pfadabhängigkeits-Wiederverwendungsmuster, das `dream-os-kernel`
   durchgängig nutzt.
2. **Software-seitige Duty-Cycle-Leistungssteuerung**
   (`MiningPowerProfile`) als herstellerneutrale Alternative zu
   NVML-artigen Power-Limit-APIs, wenn die Ziel-GPU diese nicht
   bereitstellt (auf NVIDIA GT730 bestätigt: alle
   Power-Management-Felder liefern `N/A` zurück).
3. **QUBO→Ising-Einbettung via Hilfsspin** zur Übertragung des
   lokalen Feldterms (h), wenn ein fest dimensionierter
   Ising-Kernel (`sbm_ising`, 64 Spins) nur den Kopplungsterm (J)
   implementiert — und die Energie-Vorzeichenkonvention (E vs. -E)
   immer gegen die tatsächliche Kernel-Dynamik prüfen, nicht nur
   gegen die Dokumentation; ein Vorzeichenfehler erzeugte ein
   scheinbar stabiles, aber falsches Ergebnis, das zufällig die
   CPU-Unit-Tests bestand.
4. **Android-Mobile-GPU-Treiber-TDR (Device-Lost) Vermeidung durch
   Batch-Splitting** (große Dispatches in kleinere Sub-Batches
   aufteilen) — Desktop-GPUs vertragen große Einzel-Dispatches,
   mobile GPUs nicht zwingend.
5. **AEAD-Verschlüsselung + Replay-/Manipulationsschutz**
   (`SecureChannel`) für nicht vertrauenswürdige
   Worker→Koordinator-Ergebnisübermittlung in verteilten
   Freiwilligen-Computing-Designs, plus ein dauerhaftes
   Write-Ahead-Log + redundanter Mehrwege-Sende-Wrapper
   (`RS-SmartTCP`s `transaction_log` + `redundant_transmission`)
   für "Daten nicht verlieren, selbst wenn alle Sendewege
   fehlschlagen".
6. **GF(2^8)-Reed-Solomon-RAID6/Z2-Paritätsberechnung auf GPU/NPU
   ausgelagert** (D3D12/DirectML unter Windows, Vulkan sonst) über
   die bestehende `open-raid-z`-Infrastruktur — inklusive
   automatischer Beschleuniger-Erkennung mit CPU-Fallback.
7. **Shader-Binärdateien via `include_bytes!` statt
   Laufzeit-Dateiladen ausliefern**, bei plattformübergreifenden
   Executables (vermeidet Android-`adb push`-Pfadprobleme mit
   losen `.spv`-Dateien).

**Wichtiger Vorbehalt**: Dieses Projekt ist ein
Prä-Alpha-Konzept-/PoC-Repository — die "Hybrid-Kernel"-Vision selbst
ist nicht implementiert, es existieren nur isolierte GPU-/RAID-/
Netzwerk-PoCs. Dies bei jeder Portierung klar offenlegen.

---

Weitere Sprachen: [日本語 (Original, vollständige Details)](PORTING.md) ·
[English](PORTING-English.md) · [Italiano](PORTING-Italian.md) ·
[Français](PORTING-French.md) · [Русский](PORTING-Russian.md) ·
[Українська](PORTING-Ukrainian.md) · [עברית](PORTING-Hebrew.md) ·
[فارسی](PORTING-Persian.md) · [العربية](PORTING-Arabic.md)
