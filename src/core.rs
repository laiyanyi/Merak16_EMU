use std::{collections::BTreeMap};

#[derive(Debug)]
pub struct Core{
    reg_high:[i16;8],
    reg_low:[i16;8],
    coms:i16,
    pc:u16,
    debug:bool,
    memory: BTreeMap<usize,u8>
}
enum Status {
    Normal,
    HALT,
    Exp
}
#[derive(PartialEq,Debug)]
enum Opcode {
    AND,
    OR,
    XOR,
    ADD,
    SUB,
    SLL,
    SRA,
    SRL,
    NOT,
    COM,
    MVH,
    MVO,
    LH,
    LI,
    SH,
    SLT,
    SOE,
    BOZ,
    BONZ,
    JAL,
    JALR,
    HALT,
    NotAInst
}
struct Inst{
    op:Opcode,
    rs1:u8,
    rs2:u8,
    rd:u8,
    group:bool,
    imm:i16
}
impl Inst {
    fn decode(raw:[u8;2])->Inst{
        let high = Inst::bit_split(raw[1]);
        let low = Inst::bit_split(raw[0]);

        let mut rs1_raw = [false;8];
        rs1_raw[0..3].copy_from_slice(&low[3..6]);
        let mut rs2_raw = [false;8];
        rs2_raw[0..2].copy_from_slice(&low[6..8]);
        rs2_raw[2] = high[0];
        let mut rd_raw = [false;8];
        rd_raw[0..3].copy_from_slice(&low[0..3]);
        let rs1 = Inst::bit_fuse8(rs1_raw);
        let rs2 = Inst::bit_fuse8(rs2_raw);
        let rd = Inst::bit_fuse8(rd_raw);
        let group = high[1];
        let op: Opcode;
        let mut imm = 0i16;
        let mut imm_raw;
        /*
            decode op and imm/off 
        */
        if raw[1] == 255 {
            op = Opcode::HALT;
            imm = 0;
        }else {
            match (high[7],high[6],high[5]) {
                //ALU
                (false,false,false) => {
                    op = match (high[4],high[3],high[2]) {
                        (false,false,false) => Opcode::AND,
                        (false,false,true) => Opcode::OR,
                        (false,true,false) => Opcode::XOR,
                        (false,true,true) => Opcode::ADD,
                        (true,false,false) => Opcode::SUB,
                        (true,false,true) => Opcode::SLL,
                        (true,true,false) => Opcode::SRA,
                        (true,true,true) => Opcode::SRL
                    };
                    imm = 0 ;
                },
                //group
                (false,false,true) => {
                    op = match (high[4],high[3],high[2]) {
                        (false,false,false) => match rs2 {
                            0 =>Opcode::NOT,
                            1 =>Opcode::COM,
                            2 =>Opcode::MVO,
                            3 =>if group {Opcode::MVH}
                            else {Opcode::NotAInst},
                            _ =>Opcode::NotAInst
                        },
                        (false,false,true) => {
                            imm_raw = [high[0];16];
                            imm_raw[0..2].copy_from_slice(&low[6..8]);
                            imm = Inst::bit_fuse16(imm_raw) as i16;
                            Opcode::LH
                        },
                        (false,true,false) => {
                            imm_raw = [low[2];16];
                            imm_raw[0..3].copy_from_slice(&low[0..3]);
                            imm = Inst::bit_fuse16(imm_raw) as i16;
                            Opcode::SH
                        },
                        _ => Opcode::NotAInst,
                    };
                },
                //imm
                (false,true,false) => {
                    op = Opcode::LI;
                    imm_raw = [high[4];16];
                    imm_raw[0..3].copy_from_slice(&low[3..6]);
                    imm_raw[3..5].copy_from_slice(&low[6..8]);
                    imm_raw[5] = high[0];
                    imm_raw[6..9].copy_from_slice(&high[2..5]);
                    imm = Inst::bit_fuse16(imm_raw) as i16;
                },
                //br j
                (true,false,false) =>{
                    imm =0;
                    op = match (high[4],high[3],high[2]) {
                        (false,false,false) => match rd {
                            0 => Opcode::SLT,
                            1 => Opcode::SOE,
                            _ => Opcode::NotAInst,
                        },
                        (false,false,true) => {
                            imm_raw = [high[1];16];
                            imm_raw[0..8].copy_from_slice(&low);
                            imm_raw[8] = high[0];
                            imm = Inst::bit_fuse16(imm_raw) as i16;
                            Opcode::BOZ
                        },
                        (false,true,false) => {
                            imm_raw = [high[1];16];
                            imm_raw[0..8].copy_from_slice(&low);
                            imm_raw[8] = high[0];
                            imm = Inst::bit_fuse16(imm_raw) as i16;
                            Opcode::BONZ
                        },
                        (true,false,false) => {
                            imm_raw = [high[1];16];
                            imm_raw[0..8].copy_from_slice(&low);
                            imm_raw[8] = high[0];
                            imm = Inst::bit_fuse16(imm_raw) as i16;
                            Opcode::JAL
                        },
                        (true,false,true) =>
                        {
                            imm_raw = [high[0];16];
                            imm_raw[0..3].copy_from_slice(&low[0..3]);
                            imm_raw[3..5].copy_from_slice(&low[6..8]);
                            imm = Inst::bit_fuse16(imm_raw) as i16;
                            Opcode::JALR
                        },
                        _ => {
                            imm = 0 ;
                            Opcode::NotAInst
                        },
                    };
                },
                _=>{
                    op = Opcode::NotAInst;
                    imm = 0 ;
                },
            };
        }

        
        Inst { op, rs1, rs2, rd, group: group, imm }
    }
    fn bit_split(raw:u8)->[bool;8]{
        let mut res = [false;8];
        let mut raw_m = raw;
        for i in 0..8 {
            if raw_m%2 == 1{
                res[i] = true;
            }
            raw_m = raw_m/2;
        }
        res
    }
    fn bit_fuse8(raw:[bool;8])->u8{
        let mut res = 0 ;
        let mut base = 1;
        for i in 0..7 {
            if raw[i] {
                res += base;
            }
            base <<= 1;
        }
        if raw[7] {
            res += base;
        }
        res
    }
    fn bit_fuse16(raw:[bool;16])->u16{
        let mut res = 0 ;
        let mut base = 1;
        for i in 0..15 {
            if raw[i] {
                res += base;
            }
            base *= 2;
        }
        if raw[15] {
            res += base;
        }
        res
    }
}
impl Core {
    pub fn new() ->Core{
        Core { 
            reg_high: [0;8], 
            reg_low: [0;8], 
            coms: 0, 
            pc: 0,
            debug:false, 
            memory:BTreeMap::new()
        }
    }
    fn exec(&mut self) ->Status{

        let raw_inst_low = match self.memory.get(&(self.pc as usize)){
            Some(t)=> *t,
            None => {
                self.memory.insert(self.pc as usize, 0u8);
                0u8
            }
        };
        let raw_inst_high = match self.memory.get(&((self.pc+1) as usize)){
            Some(t)=> *t,
            None => {
                self.memory.insert(self.pc as usize, 0u8);
                0u8
            }
        };
        let decoded_inst = Inst::decode([raw_inst_low,raw_inst_high]);
        let rd = decoded_inst.rd as usize;
        let rs1 = decoded_inst.rs1 as usize;
        let rs2 = decoded_inst.rs2 as usize;
        let imm = decoded_inst.imm;
        let group = decoded_inst.group;
        let status;
        let debug = self.debug;
        match decoded_inst.op{
            Opcode::AND => {
                if group {
                    self.reg_high[rd] = self.reg_high[rs1] & self.reg_high[rs2];                     
                }else {
                    self.reg_low[rd] = self.reg_low[rs1] & self.reg_low[rs2];
                }
                if debug {println!("PC={:0>8X},Tpye = AND, rs1 ={}, rs2 ={}, rd = {}, group = {}",self.pc,rs1,rs2,rd,group);}
                self.pc = self.pc+2;
                status = Status::Normal;                
            },
            Opcode::OR => {
                if group {
                    self.reg_high[rd] = self.reg_high[rs1] | self.reg_high[rs2];                     
                }else {
                    self.reg_low[rd] = self.reg_low[rs1] | self.reg_low[rs2];
                }
                if debug {println!("PC={:0>8X},Tpye = OR, rs1 ={}, rs2 ={}, rd = {}, group = {}",self.pc,rs1,rs2,rd,group);}
                self.pc = self.pc+2;
                status = Status::Normal;                
            },
            Opcode::XOR => {
                if group {
                    self.reg_high[rd] = self.reg_high[rs1] ^ self.reg_high[rs2];                     
                }else {
                    self.reg_low[rd] = self.reg_low[rs1] ^ self.reg_low[rs2];
                }
                if debug {println!("PC={:0>8X},Tpye = XOR, rs1 ={}, rs2 ={}, rd = {}, group = {}",self.pc,rs1,rs2,rd,group);}
                self.pc = self.pc+2;
                status = Status::Normal;                
            },
            Opcode::ADD => {
                if group {
                    self.reg_high[rd] = self.reg_high[rs1] + self.reg_high[rs2];                     
                }else {
                    self.reg_low[rd] = self.reg_low[rs1] + self.reg_low[rs2];
                }
                if debug {println!("PC={:0>8X},Tpye = ADD, rs1 ={}, rs2 ={}, rd = {}, group = {}",self.pc,rs1,rs2,rd,group);}
                self.pc = self.pc+2;
                status = Status::Normal;                
            },
            Opcode::SUB => {
                if group {
                    self.reg_high[rd] = self.reg_high[rs1] - self.reg_high[rs2];                     
                }else {
                    self.reg_low[rd] = self.reg_low[rs1] - self.reg_low[rs2];
                }
                if debug {println!("PC={:0>8X},Tpye = SUB, rs1 ={}, rs2 ={}, rd = {}, group = {}",self.pc,rs1,rs2,rd,group);}
                self.pc = self.pc+2;
                status = Status::Normal;                
            },
            Opcode::SLL => {
                if group {
                    self.reg_high[rd] = self.reg_high[rs1] << self.reg_high[rs2];                     
                }else {
                    self.reg_low[rd] = self.reg_low[rs1] << self.reg_low[rs2];
                }
                if debug {println!("PC={:0>8X},Tpye = SLL, rs1 ={}, rs2 ={}, rd = {}, group = {}",self.pc,rs1,rs2,rd,group);}
                self.pc = self.pc+2;
                status = Status::Normal;                
            },
            Opcode::SRA => {
                if group {
                    self.reg_high[rd] = self.reg_high[rs1] >> self.reg_high[rs2];                     
                }else {
                    self.reg_low[rd] = self.reg_low[rs1] >> self.reg_low[rs2];
                }
                if debug {println!("PC={:0>8X},Tpye = SRA, rs1 ={}, rs2 ={}, rd = {}, group = {}",self.pc,rs1,rs2,rd,group);}
                self.pc = self.pc+2;
                status = Status::Normal;                
            },
            Opcode::SRL => {
                if group {
                    self.reg_high[rd] = (self.reg_high[rs1] as u16 >> self.reg_high[rs2] as u16) as i16;                     
                }else {
                    self.reg_low[rd] = (self.reg_low[rs1]as u16 >> self.reg_low[rs2]as u16) as i16;
                }
                if debug {println!("PC={:0>8X},Tpye = SRL, rs1 ={}, rs2 ={}, rd = {}, group = {}",self.pc,rs1,rs2,rd,group);}
                self.pc = self.pc+2;
                status = Status::Normal;                
            },
            Opcode::COM =>{
                if group {
                    self.reg_high[rd] = -self.reg_high[rs1];                     
                }else {
                    self.reg_low[rd] = -self.reg_low[rs1];
                }
                if debug {println!("PC={:0>8X},Tpye = COM, rs1 ={}, rd = {}, group = {}",self.pc,rs1,rd,group);}
                self.pc = self.pc+2;
                status = Status::Normal;
                
            },
            Opcode::NOT =>{
                if group {
                    self.reg_high[rd] = !self.reg_high[rs1];                     
                }else {
                    self.reg_low[rd] = !self.reg_low[rs1];
                }
                self.pc = self.pc+2;
                if debug {println!("PC={:0>8X},Tpye = NOT, rs1 ={}, rd = {}, group = {}",self.pc,rs1,rd,group);}
                status = Status::Normal;
                
            },
            Opcode::MVH =>{
                self.reg_high[rd] = self.reg_high[rs1];
                self.pc = self.pc+2;
                if debug {println!("PC={:0>8X},Tpye = HVD, rs1 ={}, rd = {}, group = {}",self.pc,rs1,rd,group);}
                status = Status::Normal;
                
            },
            Opcode::MVO =>{
                if group {
                    self.reg_high[rd] = self.reg_low[rs1];                     
                }else {
                    self.reg_low[rd] = self.reg_high[rs1];
                }
                if debug {println!("PC={:0>8X},Tpye = MVO, rs1 ={}, rd = {}, group = {}",self.pc,rs1,rd,group);}
                self.pc = self.pc+2;
                status = Status::Normal;
                
            },
            Opcode::LI =>{
                if group {
                    self.reg_high[rd] = imm;                     
                }else {
                    self.reg_low[rd] = imm;
                }
                if debug {println!("PC={:0>8X},Tpye = LI, rd = {}, group = {}, imm = {}",self.pc,rd,group,imm);}
                self.pc = self.pc+2;
                status = Status::Normal;
                
            },
            Opcode::LH =>{
                if group {
                    let addr = (self.reg_high[rs1]+imm)as usize;
                    let data_mem_l = match self.memory.get(&(addr as usize)){
                        Some(t)=> *t,
                        None => {
                            self.memory.insert(self.pc as usize, 0u8);
                            0u8
                        }
                    };
                    let data_mem_h = match self.memory.get(&((addr+1) as usize)){
                        Some(t)=> *t,
                        None => {
                            self.memory.insert(self.pc as usize, 0u8);
                            0u8
                        }
                    };
                    self.reg_high[rd] = data_mem_l as i16 +((data_mem_h as i16) << 8 );                     
                }else {
                    let addr = (self.reg_low[rs1]+imm)as usize;
                    let data_mem_l = match self.memory.get(&(addr as usize)){
                        Some(t)=> *t,
                        None => {
                            self.memory.insert(addr as usize, 0u8);
                            0u8
                        }
                    };
                    let data_mem_h = match self.memory.get(&((addr+1) as usize)){
                        Some(t)=> *t,
                        None => {
                            self.memory.insert((addr+1) as usize, 0u8);
                            0u8
                        }
                    };
                    self.reg_low[rd] = data_mem_l as i16 +((data_mem_h as i16) << 8 );
                }
                if debug {println!("PC={:0>8X},Tpye = LH, rs1 ={}, rd = {}, group = {}, imm = {}",self.pc,rs1,rd,group,imm);}
                self.pc = self.pc+2;
                status = Status::Normal;
                
            },
            Opcode::SH =>{
                let addr = if group {(self.reg_high[rs1]+imm)as usize} else {(self.reg_low[rs1]+imm)as usize};
                let m0_raw = if group {self.reg_high[rs2] as u8} else {self.reg_low[rs2] as u8};
                let m1_raw = if group {((self.reg_high[rs2] as u16) >>8) as u8}else{((self.reg_low[rs2] as u16) >>8) as u8};
                self.memory.insert(addr, m0_raw);
                self.memory.insert(addr+1, m1_raw);

                if debug {println!("PC={:0>8X},Tpye = SH, rs1 ={}, rs2 = {}, group = {}, imm = {}",self.pc,rs1,rs2,group,imm);}
                self.pc = self.pc+2;
                status = Status::Normal;
                
            },
            Opcode::SLT =>{
                self.coms = if group 
                        {if self.reg_high[rs1] < self.reg_high[rs2] {1} else {0}} 
                    else 
                        {if self.reg_low[rs1] < self.reg_low[rs2] {1} else {0} };                     
                if debug {println!("PC={:0>8X},Tpye = SLT, rs1 ={}, rs2 = {}, group = {}",self.pc,rs1,rs2,group);}
                self.pc = self.pc+2;
                status = Status::Normal;                
            }
            Opcode::SOE =>{
                self.coms = if group 
                        {if self.reg_high[rs1] == self.reg_high[rs2] {1} else {0}} 
                    else 
                        {if self.reg_low[rs1] == self.reg_low[rs2] {1} else {0} };   
                if debug {println!("PC={:0>8X},Tpye = SOE, rs1 ={}, rs2 = {}, group = {}",self.pc,rs1,rs2,group);}
                self.pc = self.pc+2;
                status = Status::Normal;                
            }
            Opcode::BOZ =>{
                if debug {println!("PC={:0>8X},Tpye = BOZ, imm ={}",self.pc,imm);}
                self.pc = if self.coms == 0 {(self.pc as i16 + imm) as u16  } else {self.pc +2};
                status = Status::Normal;
                
            }
            Opcode::BONZ =>{
                if debug {println!("PC={:0>8X},Tpye = BONZ, imm ={}",self.pc,imm);}
                self.pc = if self.coms != 0 {(self.pc as i16 + imm )as u16 } else {self.pc +2};
                status = Status::Normal;
            }
            Opcode::JAL =>{
                if debug {println!("PC={:0>8X},Tpye = JAL, imm ={}",self.pc,imm);}
                self.reg_high[7] = (self.pc+2) as i16;
                self.pc = (self.pc as i16 + imm )as u16;
                status = Status::Normal;
            }
            Opcode::JALR =>{
                let addr = if group {self.reg_high[rs1]+imm} else {self.reg_low[rs1]+imm};
                if debug {println!("PC={:0>8X},Tpye = JALR, rs1={}, group = {}, imm ={}",self.pc,rs1,group,imm);}
                self.reg_high[7] = (self.pc+2) as i16;
                self.pc = (self.pc as i16 + addr )as u16;
                status = Status::Normal;
            }
            Opcode::HALT =>{
                if debug {println!("PC={:0>8X},Tpye = HALT",self.pc);}
                self.pc = self.pc;
                status = Status::HALT;
            },
            Opcode::NotAInst => {
                status = Status::Exp;
            }
        }
        //hardwired-zero process
        self.reg_low[0] = 0;
        
        status
    }
    pub fn step_run(&mut self){
        self.exec();
    }
    pub fn run(&mut self){
        loop {
            match self.exec() {
                Status::Exp =>{
                    println!("Expection!!");
                    break;
                }
                Status::HALT =>{
                    println!("Reach Halt Inst! CORE STOP");
                    break;
                }
                Status::Normal =>()
            }
        }
    }
    pub fn get_reg_low(&self) ->[i16;8]{
        self.reg_low
    }
    pub fn get_reg_high(&self) ->[i16;8]{
        self.reg_high
    }
    pub fn get_pc(&self) ->u16{
        self.pc
    }
    pub fn get_coms(&self) ->i16{
        self.coms
    }
    pub fn get_memory(&self) ->BTreeMap<usize, u8> {
        self.memory.clone()
    }
    pub fn enable_debug(&mut self){
        self.debug = true;
    }
    pub fn disable_debug(&mut self){
        self.debug = false;
    }
    pub fn load_memory(&mut self,program:&[u8]){
        for i in 0..program.len(){
            self.memory.insert(i, program[i]);
        }
    }
    pub fn read_memory_cell(&mut self,addr:&usize) -> u8{
        match self.memory.get(addr){
            Some(t)=> *t,
            None => {
                self.memory.insert(self.pc as usize, 0u8);
                0u8
            }
        }
    }
}
#[cfg(test)]
mod decode_tests{
    use crate::core::Opcode;
    #[test]
    fn decode_test_and(){
        let inst=super::Inst::decode([209u8,0u8]);
        assert_eq!(inst.op,Opcode::AND);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
        assert_eq!(inst.rs2,3u8);
    }
    #[test]
    fn decode_test_or(){
        let inst=super::Inst::decode([209u8,4u8]);
        assert_eq!(inst.op,Opcode::OR);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
        assert_eq!(inst.rs2,3u8);
    }
    #[test]
    fn decode_test_xor(){
        let inst=super::Inst::decode([209u8,8u8]);
        assert_eq!(inst.op,Opcode::XOR);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
        assert_eq!(inst.rs2,3u8);
    }
    #[test]
    fn decode_test_add(){
        let inst=super::Inst::decode([209u8,12u8]);
        assert_eq!(inst.op,Opcode::ADD);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
        assert_eq!(inst.rs2,3u8);
    }
    #[test]
    fn decode_test_sub(){
        let inst=super::Inst::decode([209u8,16u8]);
        assert_eq!(inst.op,Opcode::SUB);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
        assert_eq!(inst.rs2,3u8);
    }
    #[test]
    fn decode_test_sll(){
        let inst=super::Inst::decode([209u8,20u8]);
        assert_eq!(inst.op,Opcode::SLL);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
        assert_eq!(inst.rs2,3u8);
    }
    #[test]
    fn decode_test_sra(){
        let inst=super::Inst::decode([209u8,24u8]);
        assert_eq!(inst.op,Opcode::SRA);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
        assert_eq!(inst.rs2,3u8);
    }
    #[test]
    fn decode_test_srl(){
        let inst=super::Inst::decode([209u8,28u8]);
        assert_eq!(inst.op,Opcode::SRL);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
        assert_eq!(inst.rs2,3u8);
    }
    #[test]
    fn decode_test_reg(){
        let inst=super::Inst::decode([255u8,1u8]);
        assert_eq!(inst.op,Opcode::AND);
        assert_eq!(inst.rd,7u8);
        assert_eq!(inst.rs1,7u8);
        assert_eq!(inst.rs2,7u8);
    }
    #[test]
    fn decode_test_jal(){
        let inst=super::Inst::decode([255u8,145u8]);
        assert_eq!(inst.op,Opcode::JAL);
        assert_eq!(inst.imm,511i16);
    }
    #[test]
    fn decode_test_jalr(){
        let inst=super::Inst::decode([255u8,148u8]);
        assert_eq!(inst.op,Opcode::JALR);
        assert_eq!(inst.rs1,7u8);
        assert_eq!(inst.imm,31i16);
    }
    #[test]
    fn decode_test_bonz(){
        let inst=super::Inst::decode([255u8,137u8]);
        assert_eq!(inst.op,Opcode::BONZ);
        assert_eq!(inst.imm,511i16);
    }
    #[test]
    fn decode_test_boz(){
        let inst=super::Inst::decode([0u8,134u8]);
        assert_eq!(inst.op,Opcode::BOZ);
        assert_eq!(inst.imm,-512i16);
    }
    #[test]
    fn decode_test_mvo(){
        let inst=super::Inst::decode([145u8,34u8]);
        assert_eq!(inst.op,Opcode::MVO);
        assert!(inst.group);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
        let inst=super::Inst::decode([145u8,32u8]);
        assert_eq!(inst.op,Opcode::MVO);
        assert!(!inst.group);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
    }
    #[test]
    fn decode_test_mvh(){
        let inst=super::Inst::decode([209u8,34u8]);
        assert_eq!(inst.op,Opcode::MVH);
        assert!(inst.group);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
    }
    #[test]
    fn decode_test_not_com(){
        let inst=super::Inst::decode([17u8,34u8]);
        assert_eq!(inst.op,Opcode::NOT);
        assert!(inst.group);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
        let inst=super::Inst::decode([81u8,32u8]);
        assert_eq!(inst.op,Opcode::COM);
        assert!(!inst.group);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
    }
    #[test]
    fn decode_test_lh(){
        let inst=super::Inst::decode([209u8,38u8]);
        assert_eq!(inst.op,Opcode::LH);
        assert!(inst.group);
        assert_eq!(inst.rd,1u8);
        assert_eq!(inst.rs1,2u8);
        assert_eq!(inst.imm,3i16);
    }
    #[test]
    fn decode_test_sh(){
        let inst=super::Inst::decode([139u8,42u8]);
        assert_eq!(inst.op,Opcode::SH);
        assert!(inst.group);
        assert_eq!(inst.rs1,1u8);
        assert_eq!(inst.rs2,2u8);
        assert_eq!(inst.imm,3i16);
    }
    #[test]
    fn decode_test_li(){
        let inst=super::Inst::decode([249u8,79u8]);
        assert_eq!(inst.op,Opcode::LI);
        assert!(inst.group);
        assert_eq!(inst.imm,255i16);
    }
    #[test]
    fn decode_test_slt(){
        let inst=super::Inst::decode([136u8,130u8]);
        assert_eq!(inst.op,Opcode::SLT);     
        assert!(inst.group);
        assert_eq!(inst.rs1,1u8);
        assert_eq!(inst.rs2,2u8);
    }
    #[test]
    fn decode_test_soe(){
        let inst=super::Inst::decode([137u8,130u8]);
        assert_eq!(inst.op,Opcode::SOE);     
        assert!(inst.group);
        assert_eq!(inst.rs1,1u8);
        assert_eq!(inst.rs2,2u8);
    }

}