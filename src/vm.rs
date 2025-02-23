


defvm!(
    0x01 => "a ~> (a + 1)",
    0x02 => "a b ~> b a",
    0x03 => "[2] ~> $0",
);

pub struct Stack {
    dat: [0x100;u8],
    ptr: u8,
}

pub struct VM {
    ram: [0x100;u8],
    dev: [0x100;u8],
    wst: Stack,
    rst: Stack,
    pc: u16,
}

impl VM {
    pub fn step(&mut self) {
        let instr = self.ram[self.pc];
        let mut s = if instr & 0x40 == 1 { &self.rst } else { &self.wst }

        match instr {
            0x01 => /* INC */ uxn_instr!("a ~> (a + 1)"),
            0x02 => /* POP */ uxn_instr!("a ~>"),
            0x03 => /* NIP */ uxn_instr!("b a ~> a"),
            0x04 => /* SWP */ uxn_instr!("b a ~> a b"),
            0x05 => /* ROT */ uxn_instr!("c b a ~> a c b"),
            0x06 => /* DUP */ uxn_instr!("a ~> a a"),
            0x07 => /* OVR */ uxn_instr!("a b ~> a b a"),
            0x08 => /* EQU */ uxn_instr!("a b ~> (a == b)"),
            0x09 => /* NEQ */ uxn_instr!("a b ~> (a != b)"),
            0x0A => /* GTH */ uxn_instr!("a b ~> (a > b)"),
            0x0B => /* LTH */ uxn_instr!("a b ~> (a < b)"),
            0x0C => /* JMP */ {
                self.pc += t;
            },
            0x0D => /* JCN */ {

            }

        }
    }
}
