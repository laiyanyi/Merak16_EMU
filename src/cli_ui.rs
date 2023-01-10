use crate::core::Core as Core;
use console::Term as Term;
const RED:&str = "\u{001b}[31;1m";
const BLUE:&str = "\u{001b}[34;1m";
const YELLOW:&str = "\u{001b}[33;1m";
const RESET_COLOR:&str = "\u{001b}[0m";
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
    pub fn run_args(&mut self,start_run:bool,_filename:Option<&str>){
        //load file in vm's memory
        //if start run enable

        if start_run {
            self.vm.run();
        }
    }
    pub fn exec(&mut self){
        let mut command:String;
        let mut buf:String;
        loop{
            command = match self.term.read_line(){
                Ok(t) =>t,
                Err(_)=>String::new()
            };
            if !command.is_empty(){
                match command.as_str() {
                    "run" => {
                        self.vm.run();
                    }
                    ,
                    "step" => {
                        self.vm.step_run();
                    }
                    ,
                    "show" => {
                        match self.term.write_line("Enter regi, regx or memory"){
                            Ok(_) =>{},
                            Err(_) => panic!("write_fail")
                        };
                        buf = match self.term.read_line(){
                            Ok(t) =>t,
                            Err(_)=>String::new()
                        };
                        match buf.as_str() {
                            "regi" =>{
                                buf = String::new();
                                for i in 0..8{
                                    buf.push_str(format!("reg[{:2}]= {:3} ",i,self.vm.get_reg_low()[i]).as_str());
                                }
                                match self.term.write_line(buf.as_str()){
                                    Ok(_) =>{},
                                    Err(_) => panic!("write_fail")
                                };
                                buf = String::new();
                                for i in 0..8{
                                    buf.push_str(format!("reg[{:2}]= {:3} ",i+8,self.vm.get_reg_high()[i]).as_str());
                                }
                                match self.term.write_line(buf.as_str()){
                                    Ok(_) =>{},
                                    Err(_) => panic!("write_fail")
                                };
                            },
                            "regh" =>{
                                buf = String::new();
                                for i in 0..8{
                                    buf.push_str(format!("reg[{:2}]= {:04X} ",i,self.vm.get_reg_low()[i]).as_str());
                                }
                                match self.term.write_line(buf.as_str()){
                                    Ok(_) =>{},
                                    Err(_) => panic!("write_fail")
                                };
                                buf = String::new();
                                for i in 0..8{
                                    buf.push_str(format!("reg[{:2}]= {:04X} ",i+8,self.vm.get_reg_high()[i]).as_str());
                                }
                                match self.term.write_line(buf.as_str()){
                                    Ok(_) =>{},
                                    Err(_) => panic!("write_fail")
                                };
                            },
                            "memory" =>{
                                match self.term.write_line("Enter Start Addr in Hex(Without 0x)?"){
                                    Ok(_) =>{},
                                    Err(_) => panic!("write_fail")
                                };
                                buf = match self.term.read_line(){
                                    Ok(t) =>t,
                                    Err(_)=>String::new()
                                };
                                let mut s_a;
                                match i16::from_str_radix(&buf,16){
                                    Ok(t)=>{
                                        s_a =t;
                                    },
                                    Err(_)=>{
                                        let mut temp = String::new();
                                        temp.push_str(RED);
                                        temp.push_str("syntax error");
                                        temp.push_str(RESET_COLOR);
                                        match self.term.write_line(temp.as_str()){
                                            Ok(_) =>{continue;},
                                            Err(_) => panic!("write_fail")
                                        };
                                    }
                                }
                                match self.term.write_line("Enter End Addr in Hex(Without 0x)?"){
                                    Ok(_) =>{},
                                    Err(_) => panic!("write_fail")
                                };
                                buf = match self.term.read_line(){
                                    Ok(t) =>t,
                                    Err(_)=>String::new()
                                };
                                let mut e_a;
                                match i16::from_str_radix(&buf,16){
                                    Ok(t)=>{
                                        e_a =t;
                                    },
                                    Err(_)=>{
                                        let mut temp = String::new();
                                        temp.push_str(RED);
                                        temp.push_str("syntax error");
                                        temp.push_str(RESET_COLOR);
                                        match self.term.write_line(temp.as_str()){
                                            Ok(_) =>{continue;},
                                            Err(_) => panic!("write_fail")
                                        };
                                    }
                                }
                                if(e_a<s_a){
                                    let mut temp = String::new();
                                        temp.push_str(RED);
                                        temp.push_str("error end addr smaller than start!");
                                        temp.push_str(RESET_COLOR);
                                        match self.term.write_line(temp.as_str()){
                                            Ok(_) =>{continue;},
                                            Err(_) => panic!("write_fail")
                                        };
                                }
                                buf = String::new();
                                buf.push_str(YELLOW);
                                buf.push_str(format!("MEMORY|").as_str());
                                for i in 0..32{
                                    buf.push_str(format!(" {:02X}",(i as usize)).as_str());
                                }
                                buf.push_str(RESET_COLOR);
                                match self.term.write_line(buf.as_str()){
                                    Ok(_) =>{},
                                    Err(_) => panic!("write_fail")
                                };
                                let mut n_a = (s_a>>5)<<5;
                                while n_a <= e_a{
                                    buf = String::new();
                                    buf.push_str(format!("0X{:04X}|",n_a).as_str());
                                    for i in 0..32{
                                        if n_a+i >= s_a && n_a+i <= e_a{
                                            buf.push_str(BLUE);
                                        }
                                        buf.push_str(format!(" {:02X}",self.vm.read_memory_cell(&((n_a+i) as usize))).as_str());
                                        buf.push_str(RESET_COLOR);
                                    }
                                    match self.term.write_line(buf.as_str()){
                                        Ok(_) =>{},
                                        Err(_) => panic!("write_fail")
                                    };
                                    n_a +=32;
                                }
                            },
                            _ =>{
                                let mut temp = String::new();
                                temp.push_str(RED);
                                temp.push_str("syntax error");
                                temp.push_str(RESET_COLOR);
                                match self.term.write_line(temp.as_str()){
                                    Ok(_) =>{continue;},
                                    Err(_) => panic!("write_fail")
                                };
                            }
                        }
                    }
                    ,
                    "quit" =>{
                        break;
                    },
                    "help" =>{
                        match self.term.write_line("HELP TEST"){//TODO
                            Ok(_) =>{},
                            Err(_) => panic!("write_fail")
                        };
                    },
                    _ => {
                        let mut temp = String::new();
                        temp.push_str(RED);
                        temp.push_str("syntax error");
                        temp.push_str(RESET_COLOR);
                        match self.term.write_line(temp.as_str()){
                            Ok(_) =>{continue;},
                            Err(_) => panic!("write_fail")
                        };
                    }
                }
            }
        }
    }
}