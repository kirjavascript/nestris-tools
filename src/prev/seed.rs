use super::emu;

use std::collections::HashSet;

pub fn test() {
    use emulator_6502::Interface6502;

    let (mut cpu, mut ram) = emu::load();

    let mut seeds: HashSet<u16> = HashSet::new();

    let seed = 0x8988;

    seeds.insert(seed);

    ram.write(0x17, seed as u8 & 0xFF);
    ram.write(0x18, (seed >> 8) as u8);

    loop {
        cpu.set_program_counter(0x400);
        ram.write(0xEF, 0);

        loop {
            cpu.execute_instruction(&mut ram);

            if ram.read(0xEF) == 0xFF {
                break;
            }
        }

        let seed = ram.read(0x17) as u16 + ((ram.read(0x18) as u16) << 8);

        if seed == 0x8988 {
            break;
        }
        seeds.insert(seed);
    }

    let mut invalid = Vec::new();

    for i in 0..=0xFFFF {
        if !seeds.contains(&i) {
            invalid.push(i);
        }

    }

    // println!("{:#?}", seeds.len());

    let st = vec![0x02,0x07,0x08,0x0A,0x0B,0x0E,0x12    ];
    for d in seeds.iter() {
        if d >> 8 == 0x4c {
            for t in st.iter() {
                println!("${:02x}{:02x}", d & 0xFF, t);
            }
        }

    }


}
