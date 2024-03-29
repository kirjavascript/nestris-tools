fn main() {
    let handle = |out: std::process::Output| {
        if !out.stderr.is_empty() {
            String::from_utf8_lossy(&out.stderr).split('\n')
                .for_each(|line| if !line.is_empty() { println!("cargo:warning={:?}", line)});
        }
        if !out.stdout.is_empty() {
            String::from_utf8_lossy(&out.stdout).split('\n')
                .for_each(|line| if !line.is_empty() { println!("cargo:warning={:?}", line)});
        }
    };

    for name in [
        "lines-level",
        "gym-sps",
        "hard-drop",
        "line-clear",
        "line-clear-crash",
        "line-clear-mod",
    ] {
        let ca65 = std::process::Command::new("ca65").args(&[
            "-l", &format!("bin/{}.lst", name),
            "-g", &format!("asm/{}.asm", name),
            "-o", &format!("bin/{}.o", name),
        ])
            .output().expect("ca65");

        handle(ca65);


        let ld65 = std::process::Command::new("ld65").args(&[
            "-o", &format!("bin/{}.nes", name),
            "-C", "asm/base.nes.cfg",
            &format!("bin/{}.o", name),
        ])
            .output().expect("ld65");

        handle(ld65);
    }

    println!("cargo:rerun-if-changed=./asm");
    println!("cargo:rerun-if-changed=build.rs");
}
