# Philosophie de développement & règles d'environnement (DreamOS / dream-os)

> **Remarque** : Ceci est une traduction condensée de l'état actuel.
> Le journal historique complet des modifications HANDOFF reste
> disponible uniquement en japonais, par souci de concision, dans
> [CLAUDE.md](CLAUDE.md) — s'y référer pour le détail de chaque
> session.

Disque de travail : `F:\runo`. Cette section suit la pratique
consistant à reprendre comme référence la section correspondante du
`CLAUDE.md` de [`open-raid-z`](https://github.com/aon-co-jp/open-raid-z).
Dépôt GitHub : [aon-co-jp/dream-os](https://github.com/aon-co-jp/dream-os).

**Début du concept : 2026-08-06** (le dépôt GitHub vide lui-même avait
déjà été créé le 2026-07-01, même schéma que `open-directx`).

## Rôle de ce projet (phase conceptuelle, seuls quelques PoC sont du code)

**DreamOS** vise un **noyau hybride Linux et TRON OS** destiné à
unifier la plupart des systèmes d'exploitation existant dans le
monde — Windows PowerShell, les applications macOS/iPhone et les
applications Android devraient chacune fonctionner de façon
compatible sur un noyau unique. Les objectifs futurs incluent le
support de Sony PlayStation 5/6 et Nintendo Switch 2.

## Recherche technique (2026-08-06, résumé honnête)

Une recherche Google/GitHub en japonais et en anglais n'a trouvé
**aucune implémentation existante** réalisant "un noyau hybride
Linux×TRON exécutant nativement les applications de toutes les
plateformes" tel qu'envisagé initialement. Constat principal : **il
n'existe en 2026 aucun moyen viable d'exécuter nativement des
applications iOS sur du matériel non-Apple** (Apple bloque lui-même
la compilation JIT et la virtualisation pour les applications tierces
au niveau du système). L'approche `crosvm` de ChromeOS (base VMM
commune pour Linux via Crostini et Android via ARCVM) s'est révélée
être le précédent le plus établi et réaliste. seL4 + Genode/Neptune OS
("microkernel + personnalité de système d'exploitation") constitue
une autre référence. Pour les détails, y compris une analyse propre
des mainframes (IBM z/OS / Hercules / AWS Mainframe Modernization),
voir la section "技術調査結果" dans [CLAUDE.md](CLAUDE.md).

## Plateformes cibles (feuille de route, au 2026-08-17)

**Phase 1 (matériel disponible, démarrage immédiat)** : Windows (PC
avec NVIDIA GT730) et Android (plusieurs smartphones).

**Phase 2+ (reportée, dépendante de licences futures)** : macOS/
iPhone/**iPad** (iPad ajouté le 2026-08-17), diverses distributions
Linux/UNIX, Sony PlayStation 5/6, Nintendo Switch 2/**3** (Switch 3
ajoutée le 2026-08-17), ainsi que **la compatibilité avec les
mainframes IBM (z/OS)**.

**Divulgation honnête** : Le support de macOS/iPhone/iPad, PS5/6 et
Switch 2/3 nécessite des SDK non publics, des NDA et du matériel de
développement des détenteurs de plateformes respectifs — rien de tout
cela n'existe actuellement dans cet écosystème. **La Switch 3 n'a pas
été officiellement annoncée par Nintendo au 2026-08-17** — sa mention
ici n'est qu'un espace réservé pour "si/quand elle sera annoncée", et
ne repose sur aucune information produit réelle. Pour les mainframes
IBM : l'émulateur de jeu d'instructions Hercules est open source et
techniquement intégrable, mais faire tourner un vrai z/OS nécessite
une licence officielle IBM (non disponible). Le « mainframe version
AWS » s'est révélé être un service de migration applicative (AWS
Mainframe Modernization / AWS Transform for mainframe), et non une
couche de compatibilité système.

## PoC implémentés (vérifiés sur matériel réel, Windows GT730 +
Android Adreno 619)

- **`crates/dream-os-kernel`** (réutilise `opencuda-vulkan` d'`open-
  cuda`) : base d'exécution Vulkan commune Windows/Android (kernel
  `vector_add`), contrôle de puissance façon mining-OS
  (`MiningPowerProfile` + kernel double-SHA256 `sha256d_mine`,
  résultats GPU vérifiés contre une référence CPU sur les deux
  plateformes), optimisation combinatoire inspirée de la SBM de
  Toshiba (kernel `sbm_ising`, algorithme SB balistique, GPU/CPU
  convergent vers la même configuration de spins), une application
  concrète du problème du sac à dos basée sur `sbm_ising`
  (sélection de tâches, `task_scheduler.rs`), un pont DXBC→SPIR-V
  d'`open-directx` (`directx_bridge.rs`), ainsi qu'un pont de fused
  flash attention d'`open-cuda` (`flash_attention_bridge.rs`, taille
  jouet 8×16).
- **`crates/dream-os-raid-bridge`** (réutilise `RaidZVdev` +
  `zfs_accel_hlsl` d'`open-raid-z`) : pont d'accélération de parité
  RAID6/Z2, vérification d'une accélération GPU réelle (pas de repli
  CPU), round-trip à 8 stripes et auto-guérison après corruption
  simulée d'un disque, plus une démo exécutable en une seule
  commande.
- **`crates/dream-os-wire`** (réutilise `SecureChannel` d'`open-web-
  server-wire` + `redundant_transmission`/`transaction_log` de
  `RS-SmartTCP`) : transmission de résultats chiffrée AEAD, résistante
  au rejeu et à la falsification, pour le concept World Laboratory
  (informatique distribuée bénévole), plus une persistance
  `aruaru-db` (compatible ACID), vérifiée contre une instance réelle
  d'`aruaru-server`.

## Position dans l'écosystème

Lien avec [`open-directx`](https://github.com/aon-co-jp/open-directx)
(couche de compatibilité DirectX multiplateforme, actuellement
priorité maximale de l'écosystème), [`open-cuda`](https://github.com/aon-co-jp/open-cuda)
(calcul GPU compatible CUDA, backend Vulkan réel) et
[`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm) (inférence
LLM via les kernels Vulkan d'`open-cuda`). La méthode d'intégration
exacte reste ouverte — `dream-os` est délibérément développé par
petites étapes vérifiables indépendamment, pendant que
l'achèvement d'`open-directx` reste prioritaire.

## État actuel

Les PoC ci-dessus sont vérifiés sur matériel réel, mais DreamOS
lui-même (le noyau unifié, le coordinateur World Laboratory) reste en
phase de conception. Le journal historique complet HANDOFF se trouve
dans [CLAUDE.md](CLAUDE.md).

---

Autres langues : [日本語 (Original, historique HANDOFF complet)](CLAUDE.md) ·
[English](CLAUDE-English.md) · [Deutsch](CLAUDE-German.md) ·
[Italiano](CLAUDE-Italian.md) · [Русский](CLAUDE-Russian.md) ·
[Українська](CLAUDE-Ukrainian.md) · [עברית](CLAUDE-Hebrew.md) ·
[فارسی](CLAUDE-Persian.md) · [العربية](CLAUDE-Arabic.md)
