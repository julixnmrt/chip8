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

## Apprendre à le faire de A à Z

Voici une page qui explique en détail le fonctionnement et comment créer ton propre émulateur CHIP-8 :

🔗 [https://julixnmrt.github.io/chip8-web/](https://julixnmrt.github.io/chip8-web/)
