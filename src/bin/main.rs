use td4_cpu_emulator::cpu::Cpu;
use td4_cpu_emulator::rom::Rom;

fn main() {
    println!("TD4 CPU Emulator");
    let ops = vec![0b000000];
    let rom = Rom::new(ops);

    let cpu = Cpu::new(rom);
    cpu.run();

    // TODO: display
    println!("{:?}", cpu);
}
