# PORTING.md — Guide de portabilité de dream-os (version condensée)

> **Remarque** : Ceci est une traduction condensée. Le guide complet
> de l'arborescence des fichiers et des dépendances de chemin reste
> disponible uniquement en japonais (mêlé d'anglais) dans
> [PORTING.md](PORTING.md) — s'y référer avant toute portabilité
> réelle.

**Périmètre** : `dream-os` est un dépôt en phase conceptuelle pour une
couche de services applicatifs / couche d'abstraction GPU destinée à
Windows, macOS, Android et diverses distributions Linux (avec Sony
PlayStation 5/6 et Nintendo Switch 2 comme objectifs futurs).

Résumé des schémas d'implémentation réutilisables de ce projet, au cas
où ils seraient portés vers un autre projet :

1. **Réutilisation des backends Vulkan multiplateformes existants**
   (`opencuda-vulkan` d'`open-cuda`) comme couche d'exécution partagée
   plutôt que d'écrire de nouveaux kernels à partir de zéro — le
   schéma de réutilisation par dépendance de chemin employé de façon
   systématique par `dream-os-kernel`.
2. **Contrôle logiciel du duty-cycle de puissance**
   (`MiningPowerProfile`) comme alternative neutre vis-à-vis du
   fabricant aux API de limitation de puissance type NVML, lorsque le
   GPU cible ne les fournit pas (confirmé sur NVIDIA GT730 : tous les
   champs de gestion de puissance renvoient `N/A`).
3. **Encodage QUBO→Ising via un spin auxiliaire** pour transférer le
   terme de champ local (h) lorsqu'un kernel Ising à dimension fixe
   (`sbm_ising`, 64 spins) n'implémente que le terme de couplage
   (J) — et toujours vérifier la convention de signe de l'énergie
   (E vs. -E) par rapport à la dynamique réelle du kernel, pas
   seulement à la documentation ; une erreur de signe a produit un
   résultat apparemment stable mais faux, qui passait par hasard les
   tests unitaires sur CPU.
4. **Prévention du TDR (device lost) des pilotes GPU mobiles Android
   par découpage en lots** (fractionner les grands dispatchs en
   sous-lots plus petits) — les GPU de bureau tolèrent de gros
   dispatchs uniques, les GPU mobiles pas nécessairement.
5. **Chiffrement AEAD + protection anti-rejeu/anti-falsification**
   (`SecureChannel`) pour la transmission de résultats worker→
   coordinateur non fiable dans des conceptions d'informatique
   distribuée bénévole, plus un journal d'écriture anticipée (WAL)
   persistant + un wrapper d'envoi redondant multi-chemin
   (`transaction_log` + `redundant_transmission` de `RS-SmartTCP`)
   pour « ne pas perdre de données même si toutes les voies d'envoi
   échouent ».
6. **Calcul de parité RAID6/Z2 Reed-Solomon GF(2^8) déchargé sur
   GPU/NPU** (D3D12/DirectML sous Windows, Vulkan ailleurs) via
   l'infrastructure existante d'`open-raid-z` — y compris la
   détection automatique d'accélérateur avec repli CPU.
7. **Distribution des binaires de shaders via `include_bytes!` plutôt
   que par chargement de fichiers à l'exécution**, pour les
   exécutables multiplateformes (évite les problèmes de chemin
   `adb push` sous Android avec des fichiers `.spv` séparés).

**Avertissement important** : Ce projet est un dépôt de concept/PoC
pré-alpha — la vision de « noyau hybride » elle-même n'est pas
implémentée, seuls existent des PoC isolés GPU/RAID/réseau. À
divulguer clairement lors de toute portabilité.

---

Autres langues : [日本語 (Original, détails complets)](PORTING.md) ·
[English](PORTING-English.md) · [Deutsch](PORTING-German.md) ·
[Italiano](PORTING-Italian.md) · [Русский](PORTING-Russian.md) ·
[Українська](PORTING-Ukrainian.md) · [עברית](PORTING-Hebrew.md) ·
[فارسی](PORTING-Persian.md) · [العربية](PORTING-Arabic.md)
