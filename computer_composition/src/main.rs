mod cpu;
mod memory;

use cpu::CPU;
use memory::Memory;

// TODO: Define a public Computer struct with two public fields:
// - cpu: CPU
// - memory: Memory

pub struct Computer {
    pub cpu: CPU,
    pub memory: Memory,
}

// TODO: Implement a system_info method on Computer that:
// - Delegates to cpu.specs() and memory.specs()
// - Prints: System: {cpu_specs} | {memory_specs}

impl Computer {
    fn system_info(&self) {
        println!("System: {} | {}", self.cpu.specs(), self.memory.specs())
    }
}
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let cores: u32 = input.trim().parse().expect("Invalid number");
    
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let speed_ghz: f32 = input.trim().parse().expect("Invalid number");
    
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let size_gb: u32 = input.trim().parse().expect("Invalid number");
    
    // TODO: Create CPU and Memory instances using the parsed inputs
    
    let my_cpu = CPU {cores: cores, speed_ghz: speed_ghz};
    let my_memory = Memory {size_gb: size_gb};
    
    // TODO: Create a Computer instance with the CPU and Memory
    let my_computer = Computer {cpu: my_cpu, memory: my_memory};
    
    // TODO: Call system_info() on the computer to print the result
    my_computer.system_info();  
}
