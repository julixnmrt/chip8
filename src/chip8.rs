pub struct Chip8 {
    pub memory: [u8; 4096],          // 4K de mémoire
    pub v: [u8; 16],                 // 16 registres V0 à VF
    pub i: u16,                      // Registre d'index
    pub pc: u16,                     // Compteur de programme

    pub gfx: [u8; 64 * 32],          // Mémoire vidéo

    pub delay_timer: u8,             // Timer de délai
    pub sound_timer: u8,             // Timer de son

    pub stack: [u16; 16],            // Pile
    pub sp: u8,                      // Pointeur de pile

    pub keypad: [bool; 16],          // État des touches du clavier (0-F)
}


impl Chip8 {

    pub fn new() -> Self {
        Self {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200, 
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keypad: [false; 16],
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        let start = 0x200;
        let end = start + rom.len();
        if end > self.memory.len() {
            panic!("ROM is too large to fit in memory.");
        }
        self.memory[start..end].copy_from_slice(rom);
    }

    pub fn cycle(&mut self) {
        // 1. Lire les deux octets en mémoire
        let high = self.memory[self.pc as usize] as u16;
        let low  = self.memory[(self.pc + 1) as usize] as u16;
        // 2. Combiner pour obtenir l'opcode
        let opcode = (high << 8) | low;

        // 3. Avancer le compteur de programme
        self.pc += 2;

        // 4. Exécuter l'opcode
        self.exec(opcode);
    }

pub fn exec(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x0000 => {
                match opcode {
                    0x00E0 => {
                        self.gfx = [0; 64 * 32];
                    }
                    0x00EE => {
                    }
                    _ => {
                        // TODO: ignore 0x0NNN (appel système)
                    }
                }
            }

            0x1000 => {
                    let nnn = opcode & 0x0FFF;
                    self.pc = nnn
            }
            0x6000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize; 
                let nn = (opcode & 0x00FF) as u8;          
                self.v[x] = nn;
            }

            0x7000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize; 
                let nn = (opcode & 0x00FF) as u8;          
                  self.v[x] = self.v[x].wrapping_add(nn); 
            }
            0xA000 => {
                let nnn = opcode & 0x0FFF;
                self.i = nnn;
            }
            0xD000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                let n = (opcode & 0x000F) as usize;

                let vx = self.v[x] as usize;
                let vy = self.v[y] as usize;

                self.v[0xF] = 0; // VF = 0 (pas de collision)

                for row in 0..n {
                    let sprite_byte = self.memory[(self.i + row as u16) as usize];

                    for col in 0..8 {
                        // Extraire le bit le plus à gauche (pixel)
                        let sprite_pixel = (sprite_byte >> (7 - col)) & 1;

                        if sprite_pixel == 1 {
                            let x_coord = (vx + col) % 64;  // écran largeur 64
                            let y_coord = (vy + row) % 32;  // écran hauteur 32
                            let idx = x_coord + y_coord * 64;

                            // Collision si pixel allumé devient éteint
                            if self.gfx[idx] == 1 {
                                self.v[0xF] = 1;
                            }

                            // XOR le pixel (0->1 ou 1->0)
                            self.gfx[idx] ^= 1;
                        }
                    }
                }
            }
            _ => {
                println!("Opcode non implémenté: {opcode:04X}");
            }
        }
    }
}