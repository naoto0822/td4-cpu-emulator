use td4_cpu_emulator::cpu::Cpu;
use td4_cpu_emulator::rom::Rom;

fn main() {
    println!("TD4 CPU Emulator");
    let ops = vec![
        0b00110001,
        0b00000001,
    ];
    let rom = Rom::new(ops);

    let mut cpu = Cpu::new(rom);
    cpu.run();

    cpu.print();
}
