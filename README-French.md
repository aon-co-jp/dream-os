# DreamOS (dream-os)

*Autres langues* : [日本語 (Original)](README.md) ·
[English](README-English.md) · [Deutsch](README-German.md) ·
[Italiano](README-Italian.md) · [Русский](README-Russian.md) ·
[Українська](README-Ukrainian.md) · [עברית](README-Hebrew.md) ·
[فارسی](README-Persian.md) · [العربية](README-Arabic.md)

> 📌 **Dernière mise à jour (2026-08-08)** : `dream-os-raid-bridge`
> (le pont RAID6/Z2 d'open-raid-z) a reçu une démo exécutable
> (`examples/raid6_bridge_benchmark.rs`), vérifiable en une seule
> commande (`cargo run -p dream-os-raid-bridge --example
> raid6_bridge_benchmark --release`) sans avoir à lire le code source
> des tests. Vérifié sur matériel réel NVIDIA GT730 : l'accélérateur
> de parité détecté est bien `Gpu` (pas de repli CPU), un round-trip
> d'écriture/lecture à 8 stripes correspond, et le pont s'auto-répare
> après corruption directe d'un disque (les deux disques de parité de
> RAID6 le restaurent). `dream-os-kernel/src/lib.rs` a également été
> vérifié pour des marqueurs TODO/unimplemented/`allow(dead_code)` —
> aucun trouvé, donc aucune correction n'était nécessaire
> (divulgation honnête, sans prétention d'exhaustivité).

> 📌 **Dernière mise à jour (2026-08-07)** : Ajout d'une nouvelle
> application Android (`android/`) démontrant un PoC d'intégration
> open-cuda (ouverture de périphérique Vulkan) + open-directx
> (traduction DXBC→SPIR-V), vérifiée sur matériel réel (Windows GT730
> + Android Adreno 619). Le plan d'intégration Toshiba SBM/DeepSeek
> reste en suspens — aucune cible d'optimisation concrète par dépôt
> n'a encore été identifiée, donc aucune implémentation spéculative
> n'a été effectuée (voir [CLAUDE.md](CLAUDE.md) pour le compte-rendu
> honnête). Une erreur de documentation antérieure a également été
> corrigée : `sbm_ising` dispose déjà d'un chemin de dispatch GPU via
> `opencuda_core::GpuDevice`, confirmé via `cargo test`.

**Ce projet est en phase conceptuelle, mais depuis le 2026-08-06 il
existe plusieurs codes PoC (preuve de concept) vérifiés sur matériel
réel.** Ce `README-French.md` — avec [`README.md`](README.md)
(japonais, la langue principale de cet écosystème),
[`CLAUDE.md`](CLAUDE.md), [`PORTING.md`](PORTING.md) et
[world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md)
— documente l'objectif, le périmètre, la position dans l'écosystème
et l'état d'implémentation actuel de ce projet.

## Objectif

Un **noyau hybride entre Linux et TRON OS**, visant à unifier la
plupart des systèmes d'exploitation existant dans le monde.
L'objectif final est que PowerShell de Windows, les applications
macOS/iPhone et les applications Android fonctionnent chacune de
façon compatible sur un noyau unique. Les objectifs futurs incluent
également le support de **Sony PlayStation 5 / PlayStation 6 et
Nintendo Switch 2** (orientation fixée par instruction de
l'utilisateur du 2026-08-06).

## Recherche technique (menée le 2026-08-06, résumé honnête)

Une recherche Google/GitHub en japonais et en anglais n'a trouvé
**aucune implémentation existante** réalisant tel quel le concept
« un noyau hybride Linux×TRON exécutant nativement les applications
de toutes les plateformes sur un noyau unique ». Constat
particulièrement important : **il n'existe en 2026 aucun moyen
pratique d'exécuter nativement des applications iOS sur du matériel
non-Apple** (Apple bloquant elle-même le JIT et la virtualisation
pour les applications tierces au niveau système). En revanche,
ChromeOS (méthode consistant à gérer Linux/Android de façon unifiée
sur une base VMM commune `crosvm`) s'est révélé être le précédent le
plus établi et réaliste. Pour les résultats détaillés de la
recherche, la comparaison de maturité par technologie composante et
l'orientation de conception réaliste à venir, voir la section
"技術調査結果" de [`CLAUDE.md`](CLAUDE.md).

## Plateformes cibles (feuille de route, mise à jour le 2026-08-06 sur
la base du matériel réellement disponible)

**Phase 1 (matériel disponible, démarrage immédiat)** : Windows (PC
avec NVIDIA GT730) et Android (plusieurs smartphones).

**Phase 2+ (reportée, dépendante de licences/autorisations futures)** :
macOS/iPhone, diverses distributions Linux, Sony PlayStation 5/6,
Nintendo Switch 2, **mainframes IBM (z/OS, l'émulateur de jeu
d'instructions Hercules lui-même est open source, mais le système
d'exploitation réel nécessite une licence officielle IBM)**. Le
« mainframe version AWS » s'est révélé être un service de migration
applicative et non une couche de compatibilité système, voir
[`CLAUDE.md`](CLAUDE.md) pour les détails.

**Divulgation honnête** : Le support de macOS/iPhone, PS5/6, Switch 2
et des mainframes IBM nécessite des SDK non publics, des NDA, du
matériel/de développement et des licences officielles de chaque
fabricant — cet écosystème n'en dispose actuellement d'aucun. Ces
plateformes sont reportées dans l'hypothèse d'une future obtention de
licences, le travail commençant d'abord avec Windows et Android, pour
lesquels du matériel est disponible (détails dans la section
« スコープの絞り込み » de [`CLAUDE.md`](CLAUDE.md)).

## PoC implémentés (2026-08-06, vérifiés sur matériel réel)

`crates/dream-os-kernel` (réutilise `opencuda-vulkan` d'`open-cuda`) :

- **Base d'exécution Vulkan commune Windows/Android** : le kernel
  `vector_add` a été effectivement dispatché et vérifié à la fois sur
  du matériel Windows réel (NVIDIA GT730) et sur un smartphone
  Android réel (Moto G53Y 5G, Adreno 619).
- **Fonctionnalité mining-OS avec puissance ajustable** :
  `MiningPowerProfile` (contrôle du duty-cycle côté logiciel) +
  kernel `sha256d_mine` (double-SHA256). Les résultats GPU
  correspondent exactement à une implémentation de référence CPU
  (crate `sha2`) sur les deux plateformes. Un design de découpage en
  lots a été implémenté comme parade au timeout (TDR) du pilote GPU
  mobile observé sur matériel Android réel.
- **Optimisation combinatoire de type recuit quantique inspirée de la
  Simulated Bifurcation Machine (SBM) de Toshiba** : kernel
  `sbm_ising` (algorithme SB balistique, minimisation d'un modèle
  d'Ising). Vérifié sur matériel Windows réel que la version GPU
  converge vers exactement la même configuration de spins que
  l'implémentation de référence CPU.

`crates/dream-os-wire` (réutilise `SecureChannel` d'`open-web-server-
wire`) :

- **Protection de la transmission des résultats pour le concept World
  Laboratory** (informatique distribuée bénévole type BOINC) :
  chiffrement AEAD + protection anti-rejeu via `WorkerChannel`/
  `CoordinatorChannel`, qui détectent et rejettent les résultats
  worker falsifiés ou rejoués. Vérifié avec trois scénarios
  cryptographiques réels (round-trip normal, rejet de rejeu, rejet de
  falsification). Voir
  [world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md)
  pour la conception complète.

## Position dans l'écosystème

Ce projet fait partie de l'écosystème `aon-co-jp`, en particulier lié
à (méthode d'intégration concrète non encore déterminée, détails dans
[`CLAUDE.md`](CLAUDE.md)) :

- [`open-directx`](https://github.com/aon-co-jp/open-directx) — une
  couche de compatibilité multiplateforme faisant fonctionner des
  applications/jeux DirectX Windows uniquement sur Linux/Android/et
  à terme macOS/PlayStation. **Actuellement le projet dont
  l'achèvement est poursuivi en priorité absolue dans l'écosystème.**
- [`open-cuda`](https://github.com/aon-co-jp/open-cuda) — une base de
  calcul GPU compatible CUDA (backend Vulkan réel).
- [`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm) — inférence
  LLM (utilisant les kernels Vulkan d'open-cuda).

## État actuel

Les codes PoC ci-dessus sont vérifiés sur matériel réel, mais DreamOS
lui-même (le noyau unifié, le coordinateur) reste en phase de concept
et de conception. Tout en donnant la priorité à l'amélioration
d'open-directx, dream-os grandit par petites unités vérifiables
(détails dans le HANDOFF de [`CLAUDE.md`](CLAUDE.md)).

## Projets liés

- [aon-co-jp/open-directx](https://github.com/aon-co-jp/open-directx)
- [aon-co-jp/open-cuda](https://github.com/aon-co-jp/open-cuda)
- [aon-co-jp/aruaru-llm](https://github.com/aon-co-jp/aruaru-llm)
- [aon-co-jp/RUNO](https://github.com/aon-co-jp/RUNO) — le méta-index
  de l'ensemble de l'écosystème

## License

Non décidée (phase conceptuelle).
