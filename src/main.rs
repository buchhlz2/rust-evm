mod evm;
use evm::vm::Vm;
use evm::opcode::Opcode;
use std::env;
use std::error::Error;

// started via tutorial from: https://snoozetime.github.io/2018/11/09/ethereum-vm-1.html

fn debug(vm: &mut Vm) {
    // print out opcodes & relative meaning
    loop {
        match vm.next() {
            // TODO remove END once all opcodes implemented
            Opcode::END => {
                println!("END");
                break;
            },
            x => x.describe(),
        }
        
    }
    
}

fn interpret(vm: &mut Vm) {
    println!("{}{}",vm.pc,vm.code.len());
    while vm.pc < vm.code.len() {
        vm.interpret();
    }
    vm.print_stack();
}

// read data from input of a compiled contract's binary representation file
fn run() -> Result<(), Box<dyn Error>> {
    // get command line inputs where a single compiled contract binary file is of args[1]
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "No file passed as argument for decompilation");
    println!("{:?}",args);
    // set file to decode
    let function: String = String::from(&args[1].clone());
    let binary_filename = &args[2].clone();
    println!("File to decompile: {:?}", binary_filename);

    let mut vm = Vm::new_from_file(&binary_filename).unwrap();
    println!("VM instantiated successfully");

    match &*function {
        "debug" => debug(&mut vm),
        "run" => interpret(&mut vm),
        _ => panic!("Expect either 'debug' or 'run' for first parameter")
    }

    Ok(())
}

fn main() {
    run().unwrap();
}