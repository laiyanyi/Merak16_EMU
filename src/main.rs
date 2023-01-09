pub mod core;
pub mod cli_ui;
//define some basis const here
const MEMORY_LENGTH :usize = 4096;
/*define RAM size of VM in Bytes */
const DEBUG_ENABLE :bool = true;
/*  print stuff like below when enable
    PC=00000000,Tpye = LI, rd = 4, group = false, imm = -5
*/
fn main(){
    //load program in raw Binary in an array of unsigned Byte
    let mut program = vec![255u8;MEMORY_LENGTH];
    program[0] = 220;//
    program[1] = 93;//LI a4,-5
    
    //create a Core struct with the program you load
    let mut core = core::Core::new_from_vec(&program,DEBUG_ENABLE);
    
    //run step by step
    
    core.step_run();
    
    
    
    //or run it until exec HALT
    core.run();

    //output result by variable debug output{:?} or just output it
    println!("memory = {:?}",core.get_memory());//reg high group
    println!("reg high group = {:?}",core.get_reg_high());//reg high group
    println!("reg low group = {:?}",core.get_reg_low());//reg low group
    println!("reg a0 = {}",core.get_reg_low()[0]);//reg low group
    println!("PC = {:08X}",core.get_pc());//pc
}