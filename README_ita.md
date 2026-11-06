<p align="center">
  <img src="images/EH3D_THERMAL_LAB_BANNER.svg" alt="EH3D-THERMAL-LAB banner" width="100%">
</p>

# 🛠️ EH3D-THERMAL-LAB

<p align="center"><a href="README.md">🇺🇸 English</a> | <a href="README_spa.md">🇪🇸 Español</a> | <a href="README_fra.md">🇫🇷 Français</a> | 🇮🇹 <b>Italiano</b> | <a href="README_deu.md">🇩🇪 Deutsch</a> | <a href="README_zho.md">🇨🇳 中文</a> | <a href="README_jpn.md">🇯🇵 日本語</a> | <a href="README_por.md">🇵🇹 Português</a></p>

### EH3D-THERMAL-LAB: stime termiche della fase 1: il riscaldamento di una pista di rame e reti termiche semplificate, dove ogni risultato dichiara ipotesi, condizioni e limiti. Codice iniziale.

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Markdown%20%2B%20Python-blue.svg" alt="Markdown and Python">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Tests-3-2EA043.svg" alt="Tests">
  <img src="https://img.shields.io/badge/Version-0.0.2-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
</p>

---

**Verifica di onestà - cosa funziona oggi:** una libreria Rust e una riga di comando che calcolano l'aumento di temperatura di una pista di rame percorsa da corrente (effetto Joule, resistenza che cresce con la temperatura, raffreddamento per convezione) e attraverso una rete di resistenze termiche in serie e in parallelo. Un risultato è accettato solo con ipotesi, condizioni al contorno, materiali, incertezza, limitazioni e intervallo di validità; un caso che si scalda troppo o non converge non dà alcun numero. Tutto è stato eseguito solo su casi prodotti dai propri test e confrontati con valori di formula chiusa; nulla è stato confrontato con un risolutore o con una misura. Non esistono: l'uso delle perdite del dominio di simulazione, l'accoppiamento con la geometria meccanica e il confronto con le misure.

---

> **Stato di sicurezza funzionale e informatica:** questo repository è ai primi passi e non è un sistema di sicurezza funzionale. Nulla di quanto scritto qui sostituisce i limiti, gli interblocchi e l'arresto di emergenza della macchina e del suo controllore.

## 📋 In breve

| | |
|---|---|
| Versione | `0.0.2` |
| Maturità | Impalcatura |
| Test | 3 |
| Stack tecnologico | Markdown, Python 3.10+ |
| Su hardware reale | Non ancora eseguito su hardware |

**Vai a:** [Panoramica](#-panoramica) · [Architettura](#-architettura) · [Avvio rapido](#-avvio-rapido) · [Test](#-test) · [Struttura del repository](#-struttura-del-repository) · [Progetti correlati](#-progetti-correlati)

## 🚦 Stato del progetto

Ogni riga è una parte di questo repository e il semaforo dice a che punto è oggi. È scritta in base a ciò che il repository può mostrare (test, compilazioni, esecuzioni), non a ciò che è previsto.

🟢 Fatto e verificato · 🟡 Funziona, con limiti noti · 🟠 Fatto a metà o non ancora verificato · 🔴 Non fatto o non funzionante

| Area | Stato | Cosa significa qui |
|---|---|---|
| Controlli del repository | 🟡 | Il controllo del repository, quello delle immagini e i 3 test della base girano nella CI e passano. |
| Test | 🟡 | I 21 test del codice Rust e i 3 della base passano su un computer. Usano solo casi prodotti dai test stessi e valori di formula chiusa. |
| Hardware reale | 🟠 | Non eseguito su nessuna macchina né scheda: nulla qui è stato provato su hardware. |
| Funzione pianificata | 🟠 | Scritto: il riscaldamento Joule di una pista, la rete in serie e in parallelo, il risultato che rifiuta di esistere senza le sue condizioni e il rifiuto dei casi fuori intervallo o senza convergenza. Non scritto: le sorgenti di calore del dominio di simulazione, la geometria dello strumento meccanico e il confronto con le misure. |
| Requisiti | 🟠 | 4 dei 7 requisiti sono a livello di prototipo; le sorgenti di calore, l'accoppiamento con la geometria e il confronto con le misure sono solo pianificati. Nessuno è validato. |

## 🎯 Panoramica

* **Cosa esiste:** una libreria e una riga di comando, senza finestra, che calcolano il riscaldamento di una pista (`trace`), l'aumento attraverso una rete termica (`network`) e verificano che un risultato porti tutte le sue condizioni (`check`). **Pianificato:** sorgenti di calore del dominio di simulazione, l'accoppiamento con la geometria e il confronto con le misure.

## 🧩 Ruolo in EH3D

EH3D-THERMAL-LAB fa parte della famiglia PHOENIX della EH3D Fabrication Suite. Cosa farà, cosa non farà ancora, i suoi requisiti e le sue relazioni con altri repository sono scritti nella cartella `docs/`.

## 🏛️ Architettura

<p align="center">
  <img src="images/architecture.svg" alt="Architettura di EH3D-THERMAL-LAB: chi lo usa, che cosa contiene e da che cosa dipende" width="100%">
</p>

Come si incastrano le parti di questo repository: chi lo usa, che cosa contiene e da che cosa dipende. L'immagine è disegnata da `docs/architecture.json` e verificata rispetto al codice.

## 🚀 Avvio rapido

Serve Python 3.10 o successivo; non c'è nulla da installare.

```bash
python -I tools/ci_validate.py
python -I tools/make_architecture.py --check
python -I tools/make_structure.py --check --readmes
python -I -m unittest discover -s tools -p "test_*.py"
```

## 🧪 Test

La base ha 3 test (`python -I -m unittest discover -s tools -p "test_*.py"`), e passano. La CI esegue il controllo del repository, quello delle immagini e questi test.

## 🏗️ Compilazione e rilascio

La versione sta nel file `VERSION` e in `eh3d.project.json`, e ogni versione ha la sua voce in [CHANGELOG.md](CHANGELOG.md). Non c'è nulla da compilare; le versioni vengono pubblicate su GitHub.

## 📂 Struttura del repository

<p align="center">
  <img src="images/repository-structure.svg" alt="Struttura del repository EH3D-THERMAL-LAB" width="100%">
</p>

<details>
<summary>Versione testuale dell'albero</summary>

```text
EH3D-THERMAL-LAB/
├── docs/
│   ├── architecture.json structure.json         i dati delle due immagini
│   ├── SCOPE.md REQUIREMENTS.md                 ciò che è pianificato e i suoi requisiti
│   ├── DEPENDENCIES.md THREAT_MODEL.md          relazioni con altri repository e modello delle minacce
│   ├── DECISIONS.md STATUS.md                   decisioni applicabili qui e stato onesto
│   ├── NOT_YET.md RULES.md                      idee per dopo e regole per scrivere il codice
│   └── adr/                                     i registri delle decisioni accettate
├── AGENTS.md                                    regole per chi modifica il repository
├── src/                                         la libreria e la riga di comando
├── Cargo.toml Cargo.lock deny.toml              pacchetto Rust e politica delle dipendenze
├── tools/                                       il controllo del repository, i suoi test e gli strumenti delle immagini
├── images/                                      banner e le immagini di Architettura e Struttura del repository
├── .github/                                     workflow di CI e della dashboard, modelli di issue e pull request
├── eh3d.project.json                            manifesto: versione, ruolo, maturità e comando di compilazione
├── VERSION                                      la versione del repository
├── CHANGELOG.md                                 cosa è cambiato in ogni versione
├── SECURITY.md SUPPORT.md                       come segnalare una vulnerabilità e dove chiedere
└── CONTRIBUTING.md CODE_OF_CONDUCT.md LICENSE   come contribuire, codice di condotta e licenza
```

</details>

## 🗺️ Roadmap e limiti noti

**Limiti noti oggi**

* Codice iniziale: ogni risultato è una stima non confrontata con un risolutore né con una misura, e la sua incertezza è un valore provvisorio impostato nelle impostazioni.

**Prossimi passi**

* Attendere che i contratti da cui dipende questo progetto siano accettati (vedi `docs/DEPENDENCIES.md`), poi scrivere la prima versione con test e sostituire le righe di stato con risultati reali.

## 🌍 Lingue

Il README e i testi esistono in inglese, spagnolo, francese, italiano, tedesco, cinese, giapponese e portoghese europeo.

## 🔗 Progetti correlati

* [EH3D-DOCS](https://github.com/JuanenRac/EH3D-DOCS): la mappa della suite, la sua architettura, le convenzioni e i controlli condivisi
* Relazioni pianificate, non ancora costruite (EH3D-PHOENIX-PCB, EH3D-PHOENIX-CAD, EH3D-COMMON, EH3D-LAB-BRIDGE, EH3D-COMPONENT-INTELLIGENCE): vedi [docs/DEPENDENCIES.md](docs/DEPENDENCIES.md)

## 📚 Documentazione

* [Le convenzioni della suite](https://github.com/JuanenRac/EH3D-DOCS/blob/main/docs/CONVENTIONS.md)
* [Registro delle modifiche](CHANGELOG.md)
* [Ambito, requisiti, dipendenze, modello delle minacce e decisioni](docs/SCOPE.md)

## 🤝 Contribuire e comunità

* Prima di una pull request esegui `python -I tools/ci_validate.py` e i test della base: tutto deve passare. Vedi [CONTRIBUTING.md](CONTRIBUTING.md) e il [codice di condotta](CODE_OF_CONDUCT.md).
* Il README esiste in otto lingue: una modifica al testo si fa in tutti e otto i file, e le due immagini sono disegnate da `docs/architecture.json` e `docs/structure.json` con `tools/make_architecture.py` e `tools/make_structure.py`.
* Segnala una vulnerabilità in privato: [SECURITY.md](SECURITY.md). Domande, idee e segnalazioni: electrohobby3d@gmail.com ([SUPPORT.md](SUPPORT.md)).

## 👤 AUTORE

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 LICENZA

Software: GPL-3.0-or-later. Vedi [LICENSE](LICENSE). I progetti hardware della suite usano la CERN OHL-S.
