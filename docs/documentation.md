---
title: "Individual Final Project BLJ"
subtitle: "Creating a Notes App using Rust"
author: "by Matteo Luciano Siro Cipriani"
date: "\\today"
geometry: margin=2.5cm
mainfont: "Segoe UI"
sansfont: "Segoe UI"
monofont: "Consolas"
colorlinks: true
linkcolor: black
urlcolor: blue
citecolor: blue
header-includes: |
  \usepackage{fancyhdr}
  \usepackage{titling}
  \usepackage{tocbibind}
  \usepackage{hyperref}
  \usepackage{pgfgantt}
  \usepackage{xcolor}
  \hypersetup{
    pdfborder={0 0 1},
    urlbordercolor={0 0 1},
    linkbordercolor={0 0 0}
  }
  \setcounter{secnumdepth}{3}
  \setcounter{tocdepth}{3}
  \pagestyle{fancy}
  \fancyhf{}
  \fancyhead[L]{Individual Final Project BLJ}
  \fancyhead[R]{Rust Notes App}
  \fancyfoot[L]{\today}
  \fancyfoot[C]{Version 1.0 | Rust Notes App | Matteo L. S. Cipriani}
  \fancyfoot[R]{Page \thepage}
  \renewcommand{\headrulewidth}{0.4pt}
  \renewcommand{\footrulewidth}{0.4pt}
  \renewcommand{\maketitle}{%
    \thispagestyle{empty}
    \vspace*{\fill}
    \begin{center}
      {\Huge\bfseries Individual Final Project BLJ}\\[2em]
      {\LARGE\bfseries Creating a Notes App using Rust}\\[3em]
      {\large\bfseries Name:} by Matteo Luciano Siro Cipriani\\[1em]
      {\large\bfseries Abgabedatum:} \today\\[1em]
      {\large\bfseries Lehrfirma:} Soreco AG\\[3em]
    \end{center}
    \vspace*{\fill}
    \newpage
  }
output: pdf_document
---

\tableofcontents

\newpage

# Versioning

| Version | Date       | Time  | Updates                 | Author          |
| ------- | ---------- | ----- | ----------------------- | --------------- |
| 1.0.0   | 03.06.2025 | 13:06 | Started Documentation   | Matteo Cipriani |
| 1.1.0   | 03.06.2025 | 14:27 | Started Introduction    | Matteo Cipriani |
| 1.2.0   | 03.06.2025 | 15:08 | Started Listing Sources | Matteo Cipriani |
| 1.3.0   | 03.06.2025 | 17:11 | Added Pllaned Schedule  | Matteo Cipriani |

\newpage

# Introduction

## Task Definition

At the end of the year, each apprentice who worked in the ZLI is required to do a project of their own choosing. They have to plan, execute and document an appropriate project over the span of 4 weeks, while working Monday - Wednesday (or Wednesday - Friday, depending on their school days). With this project, the apprentices can demonstrate what they have learned from the coaches during the last year, as all competences required to fulfill the project have been topics during this past year, some have been used very frequently, while others have only been discussed during 1 week.

## Project Description

I chose to create a Notes App using Rust. I initially wanted to make a To-Do App, but as I have already done a To-Do App using Dart & Flutter as my Sportferienprojekt, I chose to go with something different. I want to try to write this project purely in Rust, to see how much of the language I have learned during the last year, and I can definetely learn new things from this project too. Because Rust is quite famous for being a really safe programming language, I want to try and implement one or two ways to encrypt and store the data safely.

## Known Risks

I know that creating an application purely in Rust might be difficult, especially because Rust isn't really made to design, but to work. To implement a GUI, you have to use crates, which are known to sometimes be even more difficult than the standard Rust syntax itself. And Rust itself has a pretty steep learning curve too. Managing lifetimes, references, and borrowing can be complex, especially with dynamically changing data like note content. On top of that, Rust's error system (e.g., `Result` and `Option`) is safe but verbose, requiring you to explicitly handle many cases.

\newpage

# Planning

## Schedule

![Schedule (Planned)](./assets/images/gantt_planned.png)

## Entscheidungsmatrix

Übersicht über mögliche Wege und Entscheidungskriterien

\newpage

# Hauptteil

## Vorgehen und Zwischenschritte

Beschreibung des Vorgehens

## Ergebnis der Arbeit

Konstruktionszeichnungen, Zusammenstellungen etc.

## Funktionsbeschreibung

Mit Querverweis zum Anhang

## Arbeitsjournal

Dokumentation des Arbeitsprozesses

## Testplan

Beschreibung der durchgeführten Tests

\newpage

# Dailies

## Day 1: 02.06.2025

## Day 2: 03.06.2025

\newpage

# Appendix

## Sources

- [docs.rs](https://www.docs.rs)
  - Basic Documentation for all of the crates used
  - Further linking to official websites / GitHub repositories with official examples / code snippets
  - Structs, Enums, Functions, Models & Type Aliases for each crate (if available)
- [GitHub](https://www.github.com)
  - Extensive Documentation about crates
  - Function snippets
  - Implementation Methods
- [Rust's official website](https://www.rust-lang.org/)
  - Basic Questions about Rust's functionality
  - Further linking to community boards
- [THE Rust Book](https://doc.rust-lang.org/book/)
  - Basic Introduction to Rust
  - Easy explanations for some more complicated topics of Rust
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
  - Examples for some crucial features
    - Examples are editable & executable
  - Good "playground"
- [The Cargo Book](https://doc.rust-lang.org/cargo/guide/)
  - Guide through Rust's package manager
  - Easy point to access Features, Commands and general infos about cargo

## Begriffserklärungen/Glossar

Definitionen wichtiger Begriffe

## Programm-Code

Code-Beispiele und Scripts

## Foto-Dokumentation

Relevante Bilder und Screenshots

## KI-Chat-Auszüge

Dokumentation von KI-Unterstützung
