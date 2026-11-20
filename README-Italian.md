# DreamOS (dream-os)

*Altre lingue*: [日本語 (Originale)](README.md) ·
[English](README-English.md) · [Deutsch](README-German.md) ·
[Français](README-French.md) · [Русский](README-Russian.md) ·
[Українська](README-Ukrainian.md) · [עברית](README-Hebrew.md) ·
[فارسی](README-Persian.md) · [العربية](README-Arabic.md)

> 📌 **Ultimo aggiornamento (2026-08-08)**: `dream-os-raid-bridge`
> (il bridge RAID6/Z2 di open-raid-z) ha ricevuto una demo eseguibile
> (`examples/raid6_bridge_benchmark.rs`), verificabile con un solo
> comando (`cargo run -p dream-os-raid-bridge --example
> raid6_bridge_benchmark --release`) senza dover leggere il codice
> sorgente dei test. Verificato su hardware reale NVIDIA GT730:
> l'acceleratore di parità rilevato è effettivamente `Gpu` (nessun
> fallback su CPU), un round-trip di scrittura/lettura a 8 stripe
> corrisponde, e il bridge si autoripara dopo la corruzione diretta
> di un disco (i due dischi di parità di RAID6 lo ripristinano).
> Anche `dream-os-kernel/src/lib.rs` è stato controllato per
> marcatori TODO/unimplemented/`allow(dead_code)` — nessuno trovato,
> quindi non era necessaria alcuna correzione (dichiarazione onesta,
> nessuna pretesa di verifica esaustiva).

> 📌 **Ultimo aggiornamento (2026-08-07)**: Aggiunta una nuova app
> Android (`android/`) che dimostra un PoC di integrazione
> open-cuda (apertura dispositivo Vulkan) + open-directx
> (traduzione DXBC→SPIR-V), verificata su hardware reale (Windows
> GT730 + Android Adreno 619). Il piano di integrazione Toshiba
> SBM/DeepSeek resta sospeso — non è ancora stato identificato un
> obiettivo di ottimizzazione concreto per repository, quindi non è
> stata effettuata alcuna implementazione speculativa (vedi
> [CLAUDE.md](CLAUDE.md) per la disamina onesta). Inoltre è stato
> corretto un precedente errore di documentazione: `sbm_ising`
> dispone già di un percorso di dispatch GPU tramite
> `opencuda_core::GpuDevice`, confermato tramite `cargo test`.

**Questo progetto è in fase concettuale, ma dal 2026-08-06 esistono
diversi codici PoC (proof-of-concept) verificati su hardware reale.**
Questo `README-Italian.md` — insieme a [`README.md`](README.md)
(giapponese, la lingua principale di questo ecosistema),
[`CLAUDE.md`](CLAUDE.md), [`PORTING.md`](PORTING.md) e
[world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md)
— documenta lo scopo, l'ambito, la posizione nell'ecosistema e lo
stato di implementazione attuale di questo progetto.

## Scopo

Un **kernel ibrido tra Linux e TRON OS**, che punta a unificare la
maggior parte dei sistemi operativi esistenti al mondo. L'obiettivo
finale è che PowerShell di Windows, le app macOS/iPhone e le app
Android funzionino ciascuna mantenendo la compatibilità su un unico
kernel. Gli obiettivi futuri includono anche il supporto per
**Sony PlayStation 5 / PlayStation 6 e Nintendo Switch 2**
(direzione stabilita da istruzione dell'utente del 2026-08-06).

## Ricerca tecnica (condotta il 2026-08-06, riepilogo onesto)

Una ricerca Google/GitHub in giapponese e inglese non ha trovato
**alcuna implementazione esistente** che realizzi come originariamente
concepito "un kernel ibrido Linux×TRON con esecuzione nativa delle
app di tutte le piattaforme". Scoperta principale: **nel 2026 non
esiste un modo praticabile di eseguire app iOS in modo nativo su
hardware non Apple** (Apple stessa blocca la compilazione JIT e la
virtualizzazione per le app di terze parti a livello di sistema
operativo). L'approccio di ChromeOS — ospitare più sistemi operativi
guest (Linux via Crostini, Android via ARCVM) su una base VMM comune
(`crosvm`) — si è rivelato il precedente più consolidato e realistico.
Per la ripartizione completa per tecnologia componente, il confronto
di maturità e una proposta di direzione architetturale realistica,
vedere la sezione "技術調査結果" (risultati della ricerca tecnica) in
[`CLAUDE.md`](CLAUDE.md).

## Piattaforme target (roadmap, aggiornata il 2026-08-06 in base
all'hardware realmente disponibile)

**Fase 1 (hardware disponibile, si inizia ora)**: Windows (un PC con
NVIDIA GT730) e Android (diversi smartphone).

**Fase 2+ (rimandata, dipendente da future licenze/autorizzazioni)**:
macOS/iPhone/**iPad** (aggiunto il 2026-08-17), varie distribuzioni
Linux/UNIX, Sony PlayStation 5/6, Nintendo Switch 2/**3** (aggiunto il
2026-08-17), nonché **compatibilità con mainframe IBM (z/OS)**
(l'emulatore di set di istruzioni Hercules è open source, ma
l'esecuzione del sistema operativo reale richiede una licenza
ufficiale IBM). "Mainframe versione AWS" si è rivelato essere un
servizio di migrazione applicativa (AWS Mainframe Modernization,
ecc.), non uno strato di compatibilità del sistema operativo —
dettagli in [`CLAUDE.md`](CLAUDE.md).

**Dichiarazione onesta**: Il supporto di macOS/iPhone/iPad, PS5/6 e
Switch 2/3 richiede SDK non pubblici, NDA e hardware di sviluppo dei
rispettivi proprietari delle piattaforme, dei quali attualmente
questo ecosistema non dispone. **Switch 3 non è stata ufficialmente
annunciata da Nintendo al 2026-08-17** — la menzione qui è solo un
segnaposto per "se/quando verrà annunciata", non basata su
informazioni reali sul prodotto. Secondo la decisione dell'utente del
2026-08-06, queste piattaforme sono rimandate nell'ipotesi di future
licenze — il lavoro inizia ora con le piattaforme per cui l'hardware
è effettivamente disponibile.

## PoC implementati (2026-08-06, verificati su hardware reale)

`crates/dream-os-kernel` (riutilizza `opencuda-vulkan` di
`open-cuda`):

- **Base di esecuzione Vulkan comune Windows/Android**: il kernel
  `vector_add` è stato effettivamente eseguito e verificato sia su PC
  Windows (NVIDIA GT730) sia su smartphone Android reale (Moto G53Y
  5G, Adreno 619).
- **Funzionalità stile mining-OS con potenza regolabile**:
  `MiningPowerProfile` (throttling duty-cycle lato software) +
  kernel `sha256d_mine` (double-SHA256). I risultati GPU corrispondono
  byte per byte a un'implementazione di riferimento CPU (crate `sha2`)
  su entrambe le piattaforme. È stato implementato un design a batch
  suddivisi come contromisura al timeout (TDR) del driver GPU mobile
  riscontrato su hardware Android reale.
- **Ottimizzazione combinatoria stile quantum-annealing ispirata alla
  Simulated Bifurcation Machine (SBM) di Toshiba**: kernel
  `sbm_ising` (algoritmo SB balistico, minimizzazione di un modello
  di Ising). Verificato su hardware Windows reale che l'esecuzione
  GPU converge esattamente alla stessa configurazione di spin
  dell'implementazione di riferimento CPU.

`crates/dream-os-wire` (riutilizza `SecureChannel` di
`open-web-server-wire`):

- **Protezione della trasmissione dei risultati per il concetto World
  Laboratory** (calcolo distribuito volontario stile BOINC):
  crittografia AEAD + protezione da attacchi replay tramite
  `WorkerChannel`/`CoordinatorChannel`, che rilevano e rifiutano
  risultati worker manomessi o ripetuti. Verificato con tre scenari
  crittografici reali (round-trip normale, rifiuto replay, rifiuto
  manomissione). Vedere
  [world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md)
  per il design completo.

## Posizione nell'ecosistema

Parte dell'ecosistema `aon-co-jp`, in particolare collegato a (modalità
di integrazione esatta ancora da definire — dettagli in
[`CLAUDE.md`](CLAUDE.md)):

- [`open-directx`](https://github.com/aon-co-jp/open-directx) — uno
  strato di compatibilità DirectX cross-platform che fa funzionare
  app/giochi DirectX esistenti solo-Windows su Linux, Android e in
  futuro macOS/PlayStation. **Attualmente il progetto perseguito con
  massima priorità nell'ecosistema in termini di completamento.**
- [`open-cuda`](https://github.com/aon-co-jp/open-cuda) — una base di
  calcolo GPU compatibile CUDA (backend Vulkan reale).
- [`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm) — inferenza
  LLM, che utilizza i kernel Vulkan di `open-cuda`.

## Stato attuale

I PoC sopra elencati sono verificati su hardware reale, ma DreamOS in
sé (il kernel unificato, il coordinatore) è ancora in fase di
concetto/progettazione. Il completamento di open-directx resta la
priorità principale, mentre `dream-os` cresce accanto ad esso in
piccoli passi indipendentemente verificabili (dettagli nella sezione
HANDOFF di [`CLAUDE.md`](CLAUDE.md)).

## Progetti correlati

- [aon-co-jp/open-directx](https://github.com/aon-co-jp/open-directx)
- [aon-co-jp/open-cuda](https://github.com/aon-co-jp/open-cuda)
- [aon-co-jp/aruaru-llm](https://github.com/aon-co-jp/aruaru-llm)
- [aon-co-jp/RUNO](https://github.com/aon-co-jp/RUNO) — il meta-indice
  dell'intero ecosistema

## License

Da decidere (fase concettuale).
