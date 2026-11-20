# PORTING.md — Guida alla portabilità di dream-os (versione condensata)

> **Nota**: Questa è una traduzione condensata. La guida completa
> all'albero dei file e alle dipendenze di percorso resta disponibile
> solo in giapponese (misto a inglese) in [PORTING.md](PORTING.md) —
> consultarla prima di effettuare una portabilità reale.

**Ambito**: `dream-os` è un repository in fase concettuale per uno
strato di servizi applicativi/strato di astrazione GPU destinato a
Windows, macOS, Android e varie distribuzioni Linux (con Sony
PlayStation 5/6 e Nintendo Switch 2 come obiettivi futuri).

Riepilogo dei pattern implementativi riutilizzabili di questo
progetto, nel caso vengano portati in un altro progetto:

1. **Riutilizzo dei backend Vulkan cross-platform esistenti**
   (`opencuda-vulkan` di `open-cuda`) come strato di esecuzione
   condiviso invece di scrivere nuovi kernel da zero — il pattern di
   riuso per dipendenza di percorso usato sistematicamente da
   `dream-os-kernel`.
2. **Controllo software del duty-cycle di potenza**
   (`MiningPowerProfile`) come alternativa neutra rispetto al
   produttore alle API di power-limit stile NVML, quando la GPU
   target non le fornisce (confermato su NVIDIA GT730: tutti i campi
   di power management restituiscono `N/A`).
3. **Embedding QUBO→Ising tramite spin ausiliario** per trasferire il
   termine di campo locale (h) quando un kernel Ising a dimensione
   fissa (`sbm_ising`, 64 spin) implementa solo il termine di
   accoppiamento (J) — e verificare sempre la convenzione di segno
   dell'energia (E vs. -E) contro la dinamica reale del kernel, non
   solo contro la documentazione; un errore di segno ha prodotto un
   risultato apparentemente stabile ma errato, che per caso superava
   i test unitari su CPU.
4. **Prevenzione del TDR (device lost) dei driver GPU mobili Android
   tramite batch-splitting** (suddivisione di grandi dispatch in
   sub-batch più piccoli) — le GPU desktop tollerano grandi dispatch
   singoli, quelle mobili non necessariamente.
5. **Crittografia AEAD + protezione da replay/manomissione**
   (`SecureChannel`) per la trasmissione di risultati worker→
   coordinatore non affidabile in design di calcolo distribuito
   volontario, più un write-ahead log persistente + wrapper di invio
   ridondante multipercorso (`transaction_log` + `redundant_
   transmission` di `RS-SmartTCP`) per "non perdere dati anche se
   tutti i percorsi di invio falliscono".
6. **Calcolo di parità RAID6/Z2 GF(2^8) Reed-Solomon delegato a
   GPU/NPU** (D3D12/DirectML su Windows, Vulkan altrove) tramite
   l'infrastruttura esistente di `open-raid-z` — incluso il
   rilevamento automatico dell'acceleratore con fallback su CPU.
7. **Distribuzione dei binari shader tramite `include_bytes!` invece
   del caricamento file a runtime**, per eseguibili cross-platform
   (evita problemi di percorso `adb push` su Android con file `.spv`
   sciolti).

**Avvertenza importante**: Questo progetto è un repository di
concept/PoC pre-alpha — la visione "kernel ibrido" in sé non è
implementata, esistono solo PoC isolati per GPU/RAID/rete. Va
dichiarato chiaramente ad ogni portabilità.

---

Altre lingue: [日本語 (Originale, dettagli completi)](PORTING.md) ·
[English](PORTING-English.md) · [Deutsch](PORTING-German.md) ·
[Français](PORTING-French.md) · [Русский](PORTING-Russian.md) ·
[Українська](PORTING-Ukrainian.md) · [עברית](PORTING-Hebrew.md) ·
[فارسی](PORTING-Persian.md) · [العربية](PORTING-Arabic.md)
