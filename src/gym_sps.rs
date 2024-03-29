use crate::emu;
use emulator_6502::{MOS6502, Interface6502};
use crate::block::Block;

pub struct GymSPS {
    cpu: MOS6502,
    ram: emu::Ram,
    last_iter: u8,
}

impl GymSPS {
    pub fn new() -> Self {
        let (cpu, ram) = emu::load(include_bytes!("../bin/gym-sps.nes"));

        GymSPS {
            cpu, ram, last_iter: 0,
        }
    }

    pub fn reset(&mut self) {
        self.ram.write(0x19, 0); // spawnID
        self.ram.write(0xBF, 0); // nextPiece
        self.ram.write(0xEF, 0); // iter
        self.cpu.set_program_counter(0x8000);
    }

    pub fn set_input(&mut self, a: u8, b: u8, c: u8) {
        self.ram.write(0x37, a);
        self.ram.write(0x38, b);
        self.ram.write(0x39, c);
    }

    pub fn next(&mut self) -> Block {
        loop {
            self.cpu.execute_instruction(&mut self.ram);

            let iter = self.ram.read(0xef);

            if iter != self.last_iter {
                self.last_iter = iter;
                return self.ram.read(0x19).into()
            }
        }
    }

    pub fn print_start_repeats(&mut self, piece: Block) {
        let mut highest = 0;

        for i in 0..=255 {
            for j in 0..=255 {
                for k in 0..=255 {
                    self.reset();
                    self.set_input(i, j, k);

                    let mut streak = 0;
                    while self.next() == piece {
                        streak += 1;
                    }

                    if streak > highest {
                        println!(
                            "{} {:?} pieces @ {:02x}{:02x}{:02x}",
                            streak,
                            piece,
                            i,
                            j,
                            k,
                        );
                        highest = streak;
                    }
                }
            }
            println!("{}/255", i);
        }
    }

}
