use crate::instructions::Instructions;
use crate::memory::MemoryBus;
use crate::opcodes::{OpcodeEntry, OPCODES};

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub ix: u16,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            ix: 0,
            sp: 0,
            pc: 0,
        }
    }
}

pub struct StatusFlags {
    pub c: bool,
    pub v: bool,
    pub z: bool,
    pub n: bool,
    pub i: bool,
    pub h: bool,
}

impl StatusFlags {
    pub fn new() -> Self {
        Self {
            c: false,
            v: false,
            z: false,
            n: false,
            i: true,
            h: false,
        }
    }

    pub fn from_byte(val: u8) -> Self {
        Self {
            c: (val & 0x01) != 0,
            v: (val & 0x02) != 0,
            z: (val & 0x04) != 0,
            n: (val & 0x08) != 0,
            i: (val & 0x10) != 0,
            h: (val & 0x20) != 0,
        }
    }

    pub fn to_byte(&self) -> u8 {
        let mut status = 0xC0;
        if self.c { status |= 0x01; }
        if self.v { status |= 0x02; }
        if self.z { status |= 0x04; }
        if self.n { status |= 0x08; }
        if self.i { status |= 0x10; }
        if self.h { status |= 0x20; }
        status
    }
}

pub struct Cpu<M: MemoryBus> {
    pub reg: Registers,
    pub status: StatusFlags,
    pub memory: M,
    pub halt: bool,
    pub catch_fire: bool,
    pub nmi: bool,
    pub irq: bool,
    pub reset: bool,
    total_cycles: u64,
}

impl<M: MemoryBus> Cpu<M> {
    pub fn new(memory: M) -> Self {
        Self {
            reg: Registers::new(),
            status: StatusFlags::new(),
            memory,
            halt: false,
            catch_fire: false,
            nmi: false,
            irq: false,
            reset: true,
            total_cycles: 0,
        }
    }

    pub(crate) fn save_to_stack(&mut self) {
        let sp = self.reg.sp;
        self.memory.write(sp, self.reg.pc as u8);
        self.memory.write(sp.wrapping_sub(1), (self.reg.pc >> 8) as u8);
        self.memory.write(sp.wrapping_sub(2), self.reg.ix as u8);
        self.memory.write(sp.wrapping_sub(3), (self.reg.ix >> 8) as u8);
        self.memory.write(sp.wrapping_sub(4), self.reg.a);
        self.memory.write(sp.wrapping_sub(5), self.reg.b);
        self.memory.write(sp.wrapping_sub(6), self.status.to_byte());
        self.reg.sp = sp.wrapping_sub(7);
    }

    pub(crate) fn restore_from_stack(&mut self) {
        let sp = self.reg.sp;
        self.status = StatusFlags::from_byte(self.memory.read(sp.wrapping_add(1)));
        self.reg.b = self.memory.read(sp.wrapping_add(2));
        self.reg.a = self.memory.read(sp.wrapping_add(3));
        self.reg.ix = (self.memory.read(sp.wrapping_add(4)) as u16) << 8
            | self.memory.read(sp.wrapping_add(5)) as u16;
        self.reg.pc = (self.memory.read(sp.wrapping_add(6)) as u16) << 8
            | self.memory.read(sp.wrapping_add(7)) as u16;
        self.reg.sp = sp.wrapping_add(7);
    }

    fn fetch(&mut self) -> u8 {
        let opcode = self.memory.read(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
        opcode
    }

    fn decode(&self, opcode: u8) -> &'static OpcodeEntry {
        if self.catch_fire {
            return &OPCODES[0x00];
        }
        &OPCODES[opcode as usize]
    }

    fn execute(&mut self, entry: &OpcodeEntry) -> u64 {
        if entry.instruction.is_empty() {
            self.halt = true;
            let nop = &OPCODES[0x01];
            self.execute_without_halt(nop)
        } else {
            self.execute_without_halt(entry)
        }
    }

    fn execute_without_halt(&mut self, entry: &OpcodeEntry) -> u64 {
        Instructions::call(self, entry);
        entry.cycles as u64
    }

    pub fn step(&mut self) -> u64 {
        if self.halt {
            if self.catch_fire {
                self.fetch();
            } else if self.nmi {
                self.nmi = false;
                self.status.i = true;
                let n: u16 = 0xFFFF;
                self.reg.pc = (self.memory.read(n.wrapping_sub(3)) as u16) << 8
                    | self.memory.read(n.wrapping_sub(2)) as u16;
            } else if self.irq && !self.status.i {
                self.irq = false;
                self.status.i = true;
                let n: u16 = 0xFFFF;
                self.reg.pc = (self.memory.read(n.wrapping_sub(7)) as u16) << 8
                    | self.memory.read(n.wrapping_sub(6)) as u16;
            }
            return 1;
        }

        if self.reset {
            self.reset = false;
            let n: u16 = 0xFFFF;
            self.reg.pc = (self.memory.read(n.wrapping_sub(1)) as u16) << 8
                | self.memory.read(n) as u16;
        } else if self.nmi {
            self.nmi = false;
            self.save_to_stack();
            let n: u16 = 0xFFFF;
            self.reg.pc = (self.memory.read(n.wrapping_sub(3)) as u16) << 8
                | self.memory.read(n.wrapping_sub(2)) as u16;
        } else if self.irq && !self.status.i {
            self.irq = false;
            self.save_to_stack();
            self.status.i = true;
            let n: u16 = 0xFFFF;
            self.reg.pc = (self.memory.read(n.wrapping_sub(7)) as u16) << 8
                | self.memory.read(n.wrapping_sub(6)) as u16;
        }

        let opcode = self.fetch();
        let entry = self.decode(opcode);
        let cycles = self.execute(entry);
        self.total_cycles += cycles;
        cycles
    }

    pub fn total_cycles(&self) -> u64 {
        self.total_cycles
    }

    pub fn run(&mut self) {
        self.halt = false;
    }
}
