# Filosofia di sviluppo & regole d'ambiente (DreamOS / dream-os)

> **Nota**: Questa è una traduzione condensata dello stato attuale. Il
> registro storico completo delle modifiche HANDOFF resta disponibile
> solo in giapponese, per brevità, in [CLAUDE.md](CLAUDE.md) — vedere
> lì per i dettagli delle singole sessioni.

Unità di lavoro: `F:\runo`. Questa sezione segue la pratica di
adottare come riferimento la sezione corrispondente del `CLAUDE.md` di
[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z). Repository
GitHub: [aon-co-jp/dream-os](https://github.com/aon-co-jp/dream-os).

**Inizio del concetto: 2026-08-06** (il repository GitHub vuoto in sé
era già stato creato il 2026-07-01, stesso pattern di `open-directx`).

## Ruolo di questo progetto (fase concettuale, solo alcuni PoC come codice)

**DreamOS** punta a un **kernel ibrido tra Linux e TRON OS** che mira
a unificare la maggior parte dei sistemi operativi esistenti al
mondo — Windows PowerShell, app macOS/iPhone e app Android dovrebbero
funzionare ciascuna con compatibilità su un unico kernel. Gli
obiettivi futuri includono il supporto per Sony PlayStation 5/6 e
Nintendo Switch 2.

## Ricerca tecnica (2026-08-06, riepilogo onesto)

Una ricerca Google/GitHub in giapponese e inglese non ha trovato
**alcuna implementazione esistente** che realizzi "un kernel ibrido
Linux×TRON con esecuzione nativa delle app di tutte le piattaforme"
come originariamente concepito. Scoperta principale: **nel 2026 non
esiste un modo praticabile di eseguire app iOS in modo nativo su
hardware non Apple** (Apple blocca la compilazione JIT e la
virtualizzazione per le app di terze parti a livello di sistema).
L'approccio `crosvm` di ChromeOS (base VMM comune per Linux via
Crostini e Android via ARCVM) si è rivelato il precedente più
consolidato e realistico. seL4 + Genode/Neptune OS ("microkernel +
personalità del sistema operativo") è un altro punto di riferimento.
Per i dettagli, inclusa un'analisi propria sui mainframe (IBM z/OS /
Hercules / AWS Mainframe Modernization), vedere la sezione
"技術調査結果" in [CLAUDE.md](CLAUDE.md).

## Piattaforme target (roadmap, aggiornata al 2026-08-17)

**Fase 1 (hardware disponibile, si inizia ora)**: Windows (PC con
NVIDIA GT730) e Android (diversi smartphone).

**Fase 2+ (rimandata, dipendente da future licenze)**: macOS/iPhone/
**iPad** (iPad aggiunto il 2026-08-17), varie distribuzioni Linux/
UNIX, Sony PlayStation 5/6, Nintendo Switch 2/**3** (Switch 3
aggiunta il 2026-08-17), nonché **compatibilità con mainframe IBM
(z/OS)**.

**Dichiarazione onesta**: Il supporto di macOS/iPhone/iPad, PS5/6 e
Switch 2/3 richiede SDK non pubblici, NDA e hardware di sviluppo dei
rispettivi proprietari delle piattaforme — nulla di tutto ciò è
attualmente presente in questo ecosistema. **Switch 3 non è stata
ufficialmente annunciata da Nintendo al 2026-08-17** — la menzione qui
è solo un segnaposto per "se/quando verrà annunciata", non basata su
informazioni reali sul prodotto. Per i mainframe IBM: l'emulatore di
set di istruzioni Hercules è open source e tecnicamente integrabile,
ma l'esecuzione di un vero z/OS richiede una licenza ufficiale IBM
(non disponibile). "Mainframe versione AWS" si è rivelato essere un
servizio di migrazione applicativa (AWS Mainframe Modernization / AWS
Transform for mainframe), non uno strato di compatibilità del sistema
operativo.

## PoC implementati (verificati su hardware reale, Windows GT730 +
Android Adreno 619)

- **`crates/dream-os-kernel`** (riutilizza `opencuda-vulkan` di
  `open-cuda`): base di esecuzione Vulkan comune Windows/Android
  (kernel `vector_add`), controllo di potenza stile mining-OS
  (`MiningPowerProfile` + kernel double-SHA256 `sha256d_mine`,
  risultati GPU verificati contro riferimento CPU su entrambe le
  piattaforme), ottimizzazione combinatoria ispirata alla SBM di
  Toshiba (kernel `sbm_ising`, algoritmo SB balistico, GPU/CPU
  convergono alla stessa configurazione di spin), un'applicazione
  concreta del problema dello zaino basata su `sbm_ising` (selezione
  task, `task_scheduler.rs`), un bridge DXBC→SPIR-V di `open-directx`
  (`directx_bridge.rs`) e un bridge di fused flash attention di
  `open-cuda` (`flash_attention_bridge.rs`, dimensione giocattolo
  8×16).
- **`crates/dream-os-raid-bridge`** (riutilizza `RaidZVdev` +
  `zfs_accel_hlsl` di `open-raid-z`): bridge di accelerazione della
  parità RAID6/Z2, verifica di accelerazione GPU reale (nessun
  fallback su CPU), round-trip a 8 stripe e autoguarigione dopo
  danneggiamento simulato di un disco, più una demo eseguibile con un
  solo comando.
- **`crates/dream-os-wire`** (riutilizza `SecureChannel` di
  `open-web-server-wire` + `redundant_transmission`/`transaction_log`
  di `RS-SmartTCP`): trasmissione di risultati crittografata AEAD,
  resistente a replay e manomissioni, per il concetto World
  Laboratory (calcolo distribuito volontario), più persistenza
  `aruaru-db` (compatibile ACID), verificata contro un'istanza reale
  di `aruaru-server`.

## Posizione nell'ecosistema

Collegamento a [`open-directx`](https://github.com/aon-co-jp/open-directx)
(strato di compatibilità DirectX cross-platform, attualmente massima
priorità nell'ecosistema), [`open-cuda`](https://github.com/aon-co-jp/open-cuda)
(calcolo GPU compatibile CUDA, backend Vulkan reale) e
[`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm) (inferenza
LLM tramite i kernel Vulkan di `open-cuda`). Il modo esatto di
integrazione è ancora aperto — `dream-os` viene deliberatamente
sviluppato in piccoli passi indipendentemente verificabili, mentre il
completamento di `open-directx` resta prioritario.

## Stato attuale

I PoC sopra elencati sono verificati su hardware reale, ma DreamOS in
sé (il kernel unificato, il coordinatore World Laboratory) è ancora
in fase di concetto/progettazione. Il registro storico completo
HANDOFF si trova in [CLAUDE.md](CLAUDE.md).

---

Altre lingue: [日本語 (Originale, con cronologia HANDOFF completa)](CLAUDE.md) ·
[English](CLAUDE-English.md) · [Deutsch](CLAUDE-German.md) ·
[Français](CLAUDE-French.md) · [Русский](CLAUDE-Russian.md) ·
[Українська](CLAUDE-Ukrainian.md) · [עברית](CLAUDE-Hebrew.md) ·
[فارسی](CLAUDE-Persian.md) · [العربية](CLAUDE-Arabic.md)
