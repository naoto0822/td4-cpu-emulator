use td4_cpu_emulator::cpu::Cpu;

fn main() {
    println!("TD4 CPU Emulator");

    let cpu = Cpu::new();
    cpu.run();
}
