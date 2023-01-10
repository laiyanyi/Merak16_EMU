use crate::core::Core as Core;
use console::Term as Term;
pub struct Emu{
    vm:Core,
    term:Term
}
impl Emu {
    pub fn init() -> Emu{
        let emu = Emu{
            vm:Core::new(),
            term:Term::stdout()
        };
        match emu.term.write_line("UI and VM initialized!"){
            Err(err)=>{
                panic!("initialize failed due to {}",err);
            },
            Ok(_) =>{}
        }
        return emu;
    }
    pub fn run_args(&mut self,start_run:bool,filename:Option<&str>){
        //load file in vm's memory
        //if start run enable
        
        if start_run {
            self.vm.run();
        }
    }
    pub fn exec(&mut self){
        let mut command;
        loop{
            command = match self.term.read_line(){
                Ok(t) =>t,
                Err(_)=>String::new()
            };
            if !command.is_empty(){
                //to something
            }
        }
    }
}