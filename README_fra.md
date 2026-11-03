<p align="center">
  <img src="images/EH3D_THERMAL_LAB_BANNER.svg" alt="EH3D-THERMAL-LAB banner" width="100%">
</p>

# 🛠️ EH3D-THERMAL-LAB

<p align="center"><a href="README.md">🇺🇸 English</a> | <a href="README_spa.md">🇪🇸 Español</a> | 🇫🇷 <b>Français</b> | <a href="README_ita.md">🇮🇹 Italiano</a> | <a href="README_deu.md">🇩🇪 Deutsch</a> | <a href="README_zho.md">🇨🇳 中文</a> | <a href="README_jpn.md">🇯🇵 日本語</a> | <a href="README_por.md">🇵🇹 Português</a></p>

### EH3D-THERMAL-LAB : estimations thermiques de l'étape 1 : l'échauffement d'une piste en cuivre et des réseaux thermiques simplifiés, où chaque résultat dit ses hypothèses, ses conditions et ses limites. Code initial.

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Markdown%20%2B%20Python-blue.svg" alt="Markdown and Python">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Tests-3-2EA043.svg" alt="Tests">
  <img src="https://img.shields.io/badge/Version-0.0.1-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
</p>

---

**Vérification d'honnêteté - ce qui tourne aujourd'hui :** une bibliothèque Rust et une ligne de commande qui calculent l'élévation de température d'une piste en cuivre parcourue par un courant (effet Joule, résistance croissant avec la température, refroidissement par convection) et à travers un réseau de résistances thermiques en série et en parallèle. Un résultat n'est accepté qu'avec ses hypothèses, conditions aux limites, matériaux, incertitude, limitations et domaine de validité ; un cas qui chauffe trop ou ne converge pas ne donne aucun nombre. Tout n'a tourné que sur des cas faits par ses propres tests et comparés à des valeurs de formule fermée ; rien n'a été comparé à un solveur ni à une mesure. N'existent pas : l'usage de pertes du domaine de simulation, le couplage avec la géométrie mécanique et la comparaison avec des mesures.

---

> **État de la sûreté et de la sécurité :** ce dépôt en est à ses premiers pas et n'est pas un système de sûreté. Rien de ce qui est écrit ici ne remplace les limites, les verrouillages et l'arrêt d'urgence de la machine et de son contrôleur.

## 📋 En un coup d'œil

| | |
|---|---|
| Version | `0.0.1` |
| Maturité | Échafaudage |
| Tests | 3 |
| Pile technique | Markdown, Python 3.10+ |
| Sur du matériel réel | Pas encore exécuté sur du matériel |

**Aller à  :** [Présentation](#-présentation) · [Architecture](#-architecture) · [Démarrage rapide](#-démarrage-rapide) · [Tests](#-tests) · [Structure du dépôt](#-structure-du-dépôt) · [Projets liés](#-projets-liés)

## 🚦 État du projet

Chaque ligne est une partie de ce dépôt et le feu dit où elle en est aujourd'hui. Elle est écrite d'après ce que le dépôt peut montrer (tests, compilations, exécutions), pas d'après ce qui est prévu.

🟢 Fait et vérifié · 🟡 Fonctionne, avec des limites connues · 🟠 À moitié fait ou pas encore vérifié · 🔴 Pas fait ou ne fonctionne pas

| Domaine | État | Ce que cela veut dire ici |
|---|---|---|
| Contrôles du dépôt | 🟡 | Le contrôle du dépôt, celui des images et les 3 tests de la base tournent dans la CI et réussissent. |
| Tests | 🟡 | Les 21 tests du code Rust et les 3 de la base réussissent sur un ordinateur. Ils n'utilisent que des cas faits par les tests eux-mêmes et des valeurs de formule fermée. |
| Matériel réel | 🟠 | Non exécuté sur une machine ni une carte : rien ici n'a été essayé sur du matériel. |
| Fonction prévue | 🟠 | Écrit : l'échauffement Joule d'une piste, le réseau en série et en parallèle, le résultat qui refuse d'exister sans ses conditions et le refus des cas hors domaine ou sans convergence. Non écrit : les sources de chaleur du domaine de simulation, la géométrie de l'outil mécanique et la comparaison avec des mesures. |
| Exigences | 🟠 | 4 des 7 exigences sont au stade de prototype ; les sources de chaleur, le couplage avec la géométrie et la comparaison avec des mesures sont seulement prévus. Aucune n'est validée. |

## 🎯 Présentation

* **Ce qui existe :** une bibliothèque et une ligne de commande, sans fenêtre, qui calculent l'échauffement d'une piste (`trace`), l'élévation à travers un réseau thermique (`network`) et vérifient qu'un résultat porte toutes ses conditions (`check`). **Prévu :** des sources de chaleur du domaine de simulation, le couplage avec la géométrie et la comparaison avec des mesures.

## 🧩 Rôle dans EH3D

EH3D-THERMAL-LAB fait partie de la famille PHOENIX de l'EH3D Fabrication Suite. Ce qu'il fera, ce qu'il ne fera pas encore, ses exigences et ses relations avec d'autres dépôts sont décrits dans le dossier `docs/`.

## 🏛️ Architecture

<p align="center">
  <img src="images/architecture.svg" alt="Architecture de EH3D-THERMAL-LAB : qui l'utilise, ce qu'il contient et de quoi il dépend" width="100%">
</p>

Comment les parties de ce dépôt s'articulent : qui l'utilise, ce qu'il contient et de quoi il dépend. L'image est dessinée à partir de `docs/architecture.json` et vérifiée par rapport au code.

## 🚀 Démarrage rapide

Il faut Python 3.10 ou plus récent ; il n'y a rien à installer.

```bash
python -I tools/ci_validate.py
python -I tools/make_architecture.py --check
python -I tools/make_structure.py --check --readmes
python -I -m unittest discover -s tools -p "test_*.py"
```

## 🧪 Tests

La base a 3 tests (`python -I -m unittest discover -s tools -p "test_*.py"`), et ils réussissent. La CI exécute le contrôle du dépôt, celui des images et ces tests.

## 🏗️ Compilation et publication

La version se trouve dans le fichier `VERSION` et dans `eh3d.project.json`, et chaque version a son entrée dans [CHANGELOG.md](CHANGELOG.md). Il n'y a rien à compiler ; les versions sont publiées sur GitHub.

## 📂 Structure du dépôt

<p align="center">
  <img src="images/repository-structure.svg" alt="Structure du dépôt EH3D-THERMAL-LAB" width="100%">
</p>

<details>
<summary>Version texte de l'arborescence</summary>

```text
EH3D-THERMAL-LAB/
├── docs/
│   ├── architecture.json structure.json         les données des deux images
│   ├── SCOPE.md REQUIREMENTS.md                 ce qui est prévu et ses exigences
│   ├── DEPENDENCIES.md THREAT_MODEL.md          relations avec d'autres dépôts et modèle de menaces
│   ├── DECISIONS.md STATUS.md                   décisions applicables ici et état honnête
│   ├── NOT_YET.md RULES.md                      idées pour plus tard et règles d'écriture du code
│   └── adr/                                     les décisions acceptées
├── AGENTS.md                                    règles pour quiconque modifie le dépôt
├── src/                                         la bibliothèque et la ligne de commande
├── Cargo.toml Cargo.lock deny.toml              paquet Rust et politique des dépendances
├── tools/                                       le contrôle du dépôt, ses tests et les outils des images
├── images/                                      bannière et images d'Architecture et de Structure du dépôt
├── .github/                                     workflows de CI et du tableau de bord, modèles d'issues et de pull requests
├── eh3d.project.json                            manifeste : version, rôle, maturité et commande de compilation
├── VERSION                                      la version du dépôt
├── CHANGELOG.md                                 ce qui a changé dans chaque version
├── SECURITY.md SUPPORT.md                       signaler une vulnérabilité et poser des questions
└── CONTRIBUTING.md CODE_OF_CONDUCT.md LICENSE   contribuer, code de conduite et licence
```

</details>

## 🗺️ Feuille de route et limites connues

**Limites connues aujourd'hui**

* Code initial : tout résultat est une estimation non comparée à un solveur ni à une mesure, et son incertitude est une valeur provisoire fixée dans les réglages.

**Ensuite**

* Attendre l'acceptation des contrats dont dépend ce projet (voir `docs/DEPENDENCIES.md`), puis écrire la première version avec des tests et remplacer les lignes d'état par des résultats réels.

## 🌍 Langues

Le README et les textes existent en anglais, espagnol, français, italien, allemand, chinois, japonais et portugais européen.

## 🔗 Projets liés

* [EH3D-DOCS](https://github.com/JuanenRac/EH3D-DOCS) : la carte de la suite, son architecture, les conventions et les contrôles partagés
* Relations prévues, pas encore construites (EH3D-PHOENIX-PCB, EH3D-PHOENIX-CAD, EH3D-COMMON, EH3D-LAB-BRIDGE, EH3D-COMPONENT-INTELLIGENCE) : voir [docs/DEPENDENCIES.md](docs/DEPENDENCIES.md)

## 📚 Documentation

* [Les conventions de la suite](https://github.com/JuanenRac/EH3D-DOCS/blob/main/docs/CONVENTIONS.md)
* [Journal des modifications](CHANGELOG.md)
* [Périmètre, exigences, dépendances, modèle de menaces et décisions](docs/SCOPE.md)

## 🤝 Contribuer et communauté

* Avant une pull request, exécutez `python -I tools/ci_validate.py` et les tests de la base : tout doit réussir. Voir [CONTRIBUTING.md](CONTRIBUTING.md) et le [code de conduite](CODE_OF_CONDUCT.md).
* Le README existe en huit langues : une modification du texte se fait dans les huit fichiers, et les deux images sont dessinées à partir de `docs/architecture.json` et `docs/structure.json` par `tools/make_architecture.py` et `tools/make_structure.py`.
* Signalez une vulnérabilité en privé : [SECURITY.md](SECURITY.md). Questions, idées et rapports : electrohobby3d@gmail.com ([SUPPORT.md](SUPPORT.md)).

## 👤 AUTEUR

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 LICENCE

Logiciel : GPL-3.0-or-later. Voir [LICENSE](LICENSE). Les conceptions matérielles de la suite utilisent la CERN OHL-S.
