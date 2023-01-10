pub mod cli_ui;
mod core;
//define some basis const here
//const MEMORY_LENGTH :usize = 4096;
/*define RAM size of VM in Bytes */
//const DEBUG_ENABLE :bool = true;
/*  print stuff like below when enable
    PC=00000000,Tpye = LI, rd = 4, group = false, imm = -5
*/
fn main(){
    let mut emu = cli_ui::Emu::init();
    emu.run_args(false, Some(&""));
    let string = String::new();
    let st = match string.is_empty() {
        true => "empty",
        false => "not empty"
    };
    println!("{}",st);
}