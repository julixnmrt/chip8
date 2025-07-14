mod chip8;
use chip8::Chip8; 

fn main() {
    let mut chip8 = Chip8::new();

    // ROM factice
    let rom = vec![0x00, 0xE0, 0xA2, 0xF0]; 
    chip8.load_rom(&rom);

    // Vérifie que la mémoire a bien été remplie
    assert_eq!(chip8.memory[0x200], 0x00);
    assert_eq!(chip8.memory[0x201], 0xE0);
    assert_eq!(chip8.memory[0x202], 0xA2);
    assert_eq!(chip8.memory[0x203], 0xF0);

    println!("ROM chargée correctement !");

    for i in 0..4 {
        println!("--- Cycle {} ---", i + 1);
        chip8.cycle();
    }
}