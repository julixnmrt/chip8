# Chip8 Emulator

Un émulateur simple de la console CHIP-8 écrit en Rust utilisant la bibliothèque [minifb](https://docs.rs/minifb/latest/minifb/).


## Fonctionnalités

- Chargement de ROMs `.ch8`
- Affichage 64×32 pixels
- Mappage clavier configurable
- Outils de debug
- CPU fonctionnant à environ 600Hz, timers à 60Hz


## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) installé sur votre machine


## Comment lancer l’émulateur

```bash
git clone https://github.com/julixnmrt/chip8-emulator.git
cd chip8-emulator
cargo run --lib chip8
```

## Apprendre a le faire de 0 

Voici une page qui explique en detaille le fonctionnement et comme le faire de a à Z : 

https://github.com/julixnmrt/chip8-web/settings/pages