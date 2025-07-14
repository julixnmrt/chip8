mod chip8;
use chip8::Chip8; 
use minifb::{Window, WindowOptions};


const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;

const SCREEN_WIDTH: usize = WIDTH * SCALE;
const SCREEN_HEIGHT: usize = HEIGHT * SCALE;

fn draw_chip8_display(buffer: &mut [u32], display: &[u8; 64 * 32]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index_chip8 = y * WIDTH + x;
            let color = if display[index_chip8] != 0 {
                0xFFFFFFFF
            } else {
                0x00000000
            };

            for dy in 0..SCALE {
                for dx in 0..SCALE {
                    let px = x * SCALE + dx;
                    let py = y * SCALE + dy;
                    let index = py * SCREEN_WIDTH + px;
                    buffer[index] = color;
                }
            }
        }
    }
}


fn main() {
    //Déclarer le chip8 grace au constructeur
    let mut chip8 = Chip8::new();

    //lire la rom IBM.ch8
    let rom = std::fs::read("roms/IBM.ch8").expect("Failed to read ROM");
    chip8.load_rom(&rom);

    //Déclarer la fenetre avec minifb
    let mut window = Window::new(
        "CHIP-8 Emulator",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    //Initialiser le buffer pour la fenêtre
    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    // Loop qui tourne tant que la fenêtre est ouverte
    while window.is_open() {
        chip8.cycle();
        draw_chip8_display(&mut buffer, &chip8.gfx);
        window.update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    }
}
