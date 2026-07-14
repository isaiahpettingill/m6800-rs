use crate::cpu::Cpu;
use crate::memory::MemoryBus;
use crate::opcodes::{AddrMode, OpcodeEntry};

pub struct Instructions;

impl Instructions {
    pub fn init<M: MemoryBus>(&self, _cpu: &Cpu<M>) {
    }

    pub fn call<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
        match entry.instruction {
            "nop" => {}
            "tap" => { cpu.status = crate::cpu::StatusFlags::from_byte(cpu.reg.a); }
            "tpa" => { cpu.reg.a = cpu.status.to_byte(); }
            "inx" => ix_op(cpu, |x| x.wrapping_add(1)),
            "dex" => ix_op(cpu, |x| x.wrapping_sub(1)),
            "clv" => { cpu.status.v = false; }
            "sev" => { cpu.status.v = true; }
            "clc" => { cpu.status.c = false; }
            "sec" => { cpu.status.c = true; }
            "cli" => { cpu.status.i = false; }
            "sei" => { cpu.status.i = true; }
            "sba" => sba(cpu),
            "cba" => cba(cpu),
            "nba" => nba(cpu),
            "tab" => tab(cpu),
            "tba" => tba(cpu),
            "daa" => {}
            "aba" => aba(cpu),
            "tsx" => { cpu.reg.ix = cpu.reg.sp.wrapping_add(1); }
            "ins" => { cpu.reg.sp = cpu.reg.sp.wrapping_add(1); }
            "pul" => pul(cpu, entry),
            "des" => { cpu.reg.sp = cpu.reg.sp.wrapping_sub(1); }
            "txs" => { cpu.reg.sp = cpu.reg.ix.wrapping_sub(1); }
            "psh" => psh(cpu, entry),
            "rts" => rts(cpu),
            "rti" => { cpu.restore_from_stack(); }
            "wai" => { cpu.save_to_stack(); cpu.halt = true; }
            "swi" => swi(cpu),
            "neg" => neg(cpu, entry),
            "com" => com(cpu, entry),
            "lsr" => lsr(cpu, entry),
            "ror" => ror_op(cpu, entry),
            "asr" => asr_op(cpu, entry),
            "asl" => asl_op(cpu, entry),
            "rol" => rol_op(cpu, entry),
            "dec" => dec(cpu, entry),
            "inc" => inc(cpu, entry),
            "tst" => tst(cpu, entry),
            "clr" => clr_op(cpu, entry),
            "jmp" => jmp(cpu, entry),
            "sub" => sub(cpu, entry),
            "cmp" => cmp(cpu, entry),
            "sbc" => sbc(cpu, entry),
            "and" => and_op(cpu, entry),
            "bit" => bit_op(cpu, entry),
            "lda" => lda(cpu, entry),
            "sta" => sta(cpu, entry),
            "eor" => eor(cpu, entry),
            "adc" => adc(cpu, entry),
            "ora" => ora(cpu, entry),
            "add" => add(cpu, entry),
            "cpx" => cpx(cpu, entry),
            "lds" => lds(cpu, entry),
            "sts" => sts(cpu, entry),
            "ldx" => ldx(cpu, entry),
            "stx" => stx(cpu, entry),
            "bra" => branch(cpu, entry, true),
            "bhi" => branch(cpu, entry, !cpu.status.c && !cpu.status.z),
            "bls" => branch(cpu, entry, cpu.status.c || cpu.status.z),
            "bcc" => branch(cpu, entry, !cpu.status.c),
            "bcs" => branch(cpu, entry, cpu.status.c),
            "bne" => branch(cpu, entry, !cpu.status.z),
            "beq" => branch(cpu, entry, cpu.status.z),
            "bvc" => branch(cpu, entry, !cpu.status.v),
            "bvs" => branch(cpu, entry, cpu.status.v),
            "bpl" => branch(cpu, entry, !cpu.status.n),
            "bmi" => branch(cpu, entry, cpu.status.n),
            "bge" => branch(cpu, entry, cpu.status.n == cpu.status.v),
            "blt" => branch(cpu, entry, cpu.status.n != cpu.status.v),
            "bgt" => branch(cpu, entry, !cpu.status.z && cpu.status.n == cpu.status.v),
            "ble" => branch(cpu, entry, cpu.status.z || cpu.status.n != cpu.status.v),
            "bsr" => bsr(cpu, entry),
            "jsr" => jsr(cpu, entry),
            "hcf" => { cpu.halt = true; cpu.catch_fire = true; }
            _ => {}
        }
    }
}

fn st<M: MemoryBus>(cpu: &mut Cpu<M>, addr: u16, val: u16) {
    cpu.memory.write(addr, val as u8);
}

fn ld8<M: MemoryBus>(cpu: &Cpu<M>, addr: u16) -> u8 {
    cpu.memory.read(addr)
}

fn ld16<M: MemoryBus>(cpu: &Cpu<M>, addr: u16) -> u16 {
    (cpu.memory.read(addr) as u16) << 8 | cpu.memory.read(addr.wrapping_add(1)) as u16
}

fn st16<M: MemoryBus>(cpu: &mut Cpu<M>, addr: u16, val: u16) {
    cpu.memory.write(addr.wrapping_add(1), val as u8);
    cpu.memory.write(addr, (val >> 8) as u8);
}

fn rel_offset<M: MemoryBus>(cpu: &Cpu<M>) -> u16 {
    let offset = cpu.memory.read(cpu.reg.pc);
    if offset & 0x80 == 0 {
        cpu.reg.pc.wrapping_add(offset as u16)
    } else {
        cpu.reg.pc.wrapping_sub(((!offset).wrapping_add(1)) as u16)
    }
}

fn read_disp<M: MemoryBus>(cpu: &Cpu<M>) -> u16 {
    cpu.memory.read(cpu.reg.pc) as u16
}

fn read_imm<M: MemoryBus>(cpu: &Cpu<M>) -> u16 {
    cpu.memory.read(cpu.reg.pc) as u16
}

fn read_imm16<M: MemoryBus>(cpu: &Cpu<M>) -> u16 {
    ld16(cpu, cpu.reg.pc)
}

fn read_dir_addr<M: MemoryBus>(cpu: &Cpu<M>) -> u16 {
    cpu.memory.read(cpu.reg.pc) as u16
}

fn read_ext_addr<M: MemoryBus>(cpu: &Cpu<M>) -> u16 {
    ld16(cpu, cpu.reg.pc)
}

fn consumed_bytes(entry: &OpcodeEntry) -> u16 {
    match entry.addr_mode {
        AddrMode::Inh | AddrMode::Acc => 0,
        AddrMode::Rel | AddrMode::Imm | AddrMode::Dir | AddrMode::Idx | AddrMode::Immx => 1,
        AddrMode::Imm16 | AddrMode::Ext | AddrMode::Ext16 | AddrMode::Idx16 | AddrMode::Dir16 => 2,
    }
}

fn reg_value<M: MemoryBus>(cpu: &Cpu<M>, acc: &str) -> u8 {
    match acc {
        "a" => cpu.reg.a,
        "b" => cpu.reg.b,
        _ => 0,
    }
}

fn reg_set<M: MemoryBus>(cpu: &mut Cpu<M>, acc: &str, val: u8) {
    match acc {
        "a" => cpu.reg.a = val,
        "b" => cpu.reg.b = val,
        _ => {}
    }
}

fn read_operand<M: MemoryBus>(cpu: &Cpu<M>, entry: &OpcodeEntry) -> u16 {
    match entry.addr_mode {
        AddrMode::Inh => 0,
        AddrMode::Acc => reg_value(cpu, entry.acc) as u16,
        AddrMode::Imm => read_imm(cpu),
        AddrMode::Imm16 => read_imm16(cpu),
        AddrMode::Dir => ld8(cpu, read_dir_addr(cpu)) as u16,
        AddrMode::Dir16 => ld16(cpu, read_dir_addr(cpu)),
        AddrMode::Ext => ld8(cpu, read_ext_addr(cpu)) as u16,
        AddrMode::Ext16 => ld16(cpu, read_ext_addr(cpu)),
        AddrMode::Idx => {
            let disp = read_disp(cpu);
            ld8(cpu, cpu.reg.ix.wrapping_add(disp)) as u16
        }
        AddrMode::Idx16 => {
            let disp = read_disp(cpu);
            ld16(cpu, cpu.reg.ix.wrapping_add(disp))
        }
        AddrMode::Immx => {
            let disp = read_disp(cpu);
            cpu.reg.ix.wrapping_add(disp)
        }
        AddrMode::Rel => rel_offset(cpu),
    }
}

fn target_addr<M: MemoryBus>(cpu: &Cpu<M>, entry: &OpcodeEntry) -> Option<u16> {
    match entry.addr_mode {
        AddrMode::Dir | AddrMode::Dir16 => Some(read_dir_addr(cpu)),
        AddrMode::Ext | AddrMode::Ext16 => Some(read_ext_addr(cpu)),
        AddrMode::Idx | AddrMode::Idx16 => {
            let disp = read_disp(cpu);
            Some(cpu.reg.ix.wrapping_add(disp))
        }
        AddrMode::Imm => {
            if matches!(entry.instruction, "sta" | "sts" | "stx") {
                Some(read_imm(cpu))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn advance_pc<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    cpu.reg.pc = cpu.reg.pc.wrapping_add(consumed_bytes(entry));
}

fn read_advance<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) -> u16 {
    let val = read_operand(cpu, entry);
    advance_pc(cpu, entry);
    val
}

fn write_advance<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry, val: u16) {
    let addr = target_addr(cpu, entry);
    advance_pc(cpu, entry);
    if let Some(a) = addr {
        match entry.addr_mode {
            AddrMode::Dir | AddrMode::Ext | AddrMode::Idx => st(cpu, a, val),
            AddrMode::Dir16 | AddrMode::Ext16 | AddrMode::Idx16 => st16(cpu, a, val),
            AddrMode::Imm => st(cpu, a, val),
            _ => {}
        }
    }
    if entry.addr_mode == AddrMode::Acc {
        reg_set(cpu, entry.acc, val as u8);
    }
}

fn set_nz<M: MemoryBus>(cpu: &mut Cpu<M>, result: u8) {
    cpu.status.n = (result & 0x80) != 0;
    cpu.status.z = result == 0;
}

fn set_nz16<M: MemoryBus>(cpu: &mut Cpu<M>, result: u16) {
    cpu.status.n = (result & 0x8000) != 0;
    cpu.status.z = result == 0;
}

fn overflow_sub(x: u8, m: u8, result: u8) -> bool {
    ((x & 0x80) != 0 && (m & 0x80) == 0 && (result & 0x80) == 0)
        || ((x & 0x80) == 0 && (m & 0x80) != 0 && (result & 0x80) != 0)
}

fn carry_sub(x: u8, m: u8, result: u8) -> bool {
    ((x & 0x80) == 0 && (m & 0x80) != 0)
        || ((m & 0x80) != 0 && (result & 0x80) != 0)
        || ((result & 0x80) != 0 && (x & 0x80) == 0)
}

fn overflow_sub16(x: u16, m: u16, result: u16) -> bool {
    ((x & 0x8000) != 0 && (m & 0x8000) == 0 && (result & 0x8000) == 0)
        || ((x & 0x8000) == 0 && (m & 0x8000) != 0 && (result & 0x8000) != 0)
}

fn ix_op<M: MemoryBus, F>(cpu: &mut Cpu<M>, op: F)
where
    F: Fn(u16) -> u16,
{
    let result = op(cpu.reg.ix);
    cpu.status.z = (result & 0xFFFF) == 0;
    cpu.reg.ix = result;
}

fn aba<M: MemoryBus>(cpu: &mut Cpu<M>) {
    let result = cpu.reg.a.wrapping_add(cpu.reg.b);
    cpu.status.z = result == 0;
    cpu.status.n = (result & 0x80) != 0;
    cpu.status.c = (cpu.reg.a as u16 + cpu.reg.b as u16) > 0xFF;
    cpu.reg.a = result;
}

fn sba<M: MemoryBus>(cpu: &mut Cpu<M>) {
    let a = cpu.reg.a;
    let b = cpu.reg.b;
    let result = a.wrapping_sub(b);
    set_nz(cpu, result);
    cpu.status.v = overflow_sub(a, b, result);
    cpu.status.c = carry_sub(a, b, result);
    cpu.reg.a = result;
}

fn cba<M: MemoryBus>(cpu: &mut Cpu<M>) {
    let a = cpu.reg.a;
    let b = cpu.reg.b;
    let result = a.wrapping_sub(b);
    set_nz(cpu, result);
    cpu.status.v = overflow_sub(a, b, result);
    cpu.status.c = carry_sub(a, b, result);
}

fn nba<M: MemoryBus>(cpu: &mut Cpu<M>) {
    let result = cpu.reg.a & cpu.reg.b;
    set_nz(cpu, result);
    cpu.status.v = false;
    cpu.reg.a = result;
}

fn tab<M: MemoryBus>(cpu: &mut Cpu<M>) {
    cpu.status.v = false;
    set_nz(cpu, cpu.reg.a);
    cpu.reg.b = cpu.reg.a;
}

fn tba<M: MemoryBus>(cpu: &mut Cpu<M>) {
    cpu.status.v = false;
    set_nz(cpu, cpu.reg.b);
    cpu.reg.a = cpu.reg.b;
}

fn psh<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let val = reg_value(cpu, entry.acc);
    cpu.memory.write(cpu.reg.sp, val);
    cpu.reg.sp = cpu.reg.sp.wrapping_sub(1);
}

fn pul<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    cpu.reg.sp = cpu.reg.sp.wrapping_add(1);
    let val = cpu.memory.read(cpu.reg.sp);
    reg_set(cpu, entry.acc, val);
}

fn rts<M: MemoryBus>(cpu: &mut Cpu<M>) {
    cpu.reg.pc = (cpu.memory.read(cpu.reg.sp.wrapping_add(1)) as u16) << 8
        | cpu.memory.read(cpu.reg.sp.wrapping_add(2)) as u16;
    cpu.reg.sp = cpu.reg.sp.wrapping_add(2);
}

fn swi<M: MemoryBus>(cpu: &mut Cpu<M>) {
    cpu.save_to_stack();
    cpu.status.i = true;
    let n: u16 = 0xFFFF;
    cpu.reg.pc = (cpu.memory.read(n.wrapping_sub(5)) as u16) << 8
        | cpu.memory.read(n.wrapping_sub(4)) as u16;
}

fn add<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let m = read_advance(cpu, entry) as u8;
    let x = reg_value(cpu, entry.acc);
    let result = x.wrapping_add(m);
    set_nz(cpu, result);
    cpu.status.c = (x as u16 + m as u16) > 0xFF;
    reg_set(cpu, entry.acc, result);
}

fn adc<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let m = read_advance(cpu, entry) as u8;
    let x = reg_value(cpu, entry.acc);
    let carry = if cpu.status.c { 1u16 } else { 0u16 };
    let result = (x as u16).wrapping_add(m as u16).wrapping_add(carry);
    cpu.status.z = (result & 0xFF) == 0;
    cpu.status.n = (result & 0x80) != 0;
    cpu.status.c = result > 0xFF;
    reg_set(cpu, entry.acc, result as u8);
}

fn sub<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let m = read_advance(cpu, entry) as u8;
    let x = reg_value(cpu, entry.acc);
    let result = x.wrapping_sub(m);
    set_nz(cpu, result);
    cpu.status.v = overflow_sub(x, m, result);
    cpu.status.c = carry_sub(x, m, result);
    reg_set(cpu, entry.acc, result);
}

fn sbc<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let m = read_advance(cpu, entry) as u8;
    let x = reg_value(cpu, entry.acc);
    let borrow = if cpu.status.c { 1u8 } else { 0u8 };
    let result = x.wrapping_sub(m).wrapping_sub(borrow);
    set_nz(cpu, result);
    cpu.status.v = overflow_sub(x, m, result);
    cpu.status.c = carry_sub(x, m, result);
    reg_set(cpu, entry.acc, result);
}

fn cmp<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let m = read_advance(cpu, entry) as u8;
    let x = reg_value(cpu, entry.acc);
    let result = x.wrapping_sub(m);
    set_nz(cpu, result);
    cpu.status.v = overflow_sub(x, m, result);
    cpu.status.c = carry_sub(x, m, result);
}

fn and_op<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let m = read_advance(cpu, entry) as u8;
    let x = reg_value(cpu, entry.acc);
    let result = x & m;
    set_nz(cpu, result);
    cpu.status.v = false;
    reg_set(cpu, entry.acc, result);
}

fn bit_op<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let m = read_advance(cpu, entry) as u8;
    let x = reg_value(cpu, entry.acc);
    let result = x & m;
    cpu.status.z = result == 0;
    cpu.status.n = false;
    cpu.status.v = false;
}

fn eor<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let m = read_advance(cpu, entry) as u8;
    let x = reg_value(cpu, entry.acc);
    let result = x ^ m;
    set_nz(cpu, result);
    cpu.status.v = false;
    reg_set(cpu, entry.acc, result);
}

fn ora<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let m = read_advance(cpu, entry) as u8;
    let x = reg_value(cpu, entry.acc);
    let result = x | m;
    set_nz(cpu, result);
    reg_set(cpu, entry.acc, result);
}

fn lda<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let val = read_advance(cpu, entry) as u8;
    cpu.status.v = false;
    set_nz(cpu, val);
    reg_set(cpu, entry.acc, val);
}

fn sta<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let val = reg_value(cpu, entry.acc);
    cpu.status.v = false;
    set_nz(cpu, val);
    write_advance(cpu, entry, val as u16);
}

fn lds<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let val = read_advance(cpu, entry);
    cpu.status.v = false;
    cpu.reg.sp = val;
}

fn sts<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let val = cpu.reg.sp;
    cpu.status.n = (val & 0x8000) != 0;
    cpu.status.z = val == 0;
    cpu.status.v = false;
    write_advance(cpu, entry, val);
}

fn ldx<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let val = read_advance(cpu, entry);
    set_nz16(cpu, val);
    cpu.status.v = false;
    cpu.reg.ix = val;
}

fn stx<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let val = cpu.reg.ix;
    cpu.status.n = (val & 0x8000) != 0;
    cpu.status.z = val == 0;
    cpu.status.v = false;
    write_advance(cpu, entry, val);
}

fn cpx<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let m = read_advance(cpu, entry);
    let x = cpu.reg.ix;
    let result = x.wrapping_sub(m);
    set_nz16(cpu, result);
    cpu.status.v = overflow_sub16(x, m, result);
}

fn neg<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    if entry.addr_mode == AddrMode::Acc {
        let val = reg_value(cpu, entry.acc);
        let result = val.wrapping_neg();
        set_nz(cpu, result);
        cpu.status.v = val == 0x80;
        cpu.status.c = val != 0;
        reg_set(cpu, entry.acc, result);
    } else {
        let val = read_advance(cpu, entry) as u8;
        let result = val.wrapping_neg();
        set_nz(cpu, result);
        cpu.status.v = val == 0x80;
        cpu.status.c = val != 0;
        write_advance(cpu, entry, result as u16);
    }
}

fn com<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    if entry.addr_mode == AddrMode::Acc {
        let val = reg_value(cpu, entry.acc);
        let result = !val;
        cpu.status.c = true;
        set_nz(cpu, result);
        cpu.status.v = false;
        reg_set(cpu, entry.acc, result);
    } else {
        let val = read_advance(cpu, entry) as u8;
        let result = !val;
        cpu.status.c = true;
        set_nz(cpu, result);
        cpu.status.v = false;
        write_advance(cpu, entry, result as u16);
    }
}

fn lsr<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    if entry.addr_mode == AddrMode::Acc {
        let val = reg_value(cpu, entry.acc);
        let result = val >> 1;
        cpu.status.n = false;
        cpu.status.z = result == 0;
        cpu.status.c = (val & 0x01) != 0;
        cpu.status.v = cpu.status.n != cpu.status.c;
        reg_set(cpu, entry.acc, result);
    } else {
        let val = read_advance(cpu, entry) as u8;
        let result = val >> 1;
        cpu.status.n = false;
        cpu.status.z = result == 0;
        cpu.status.c = (val & 0x01) != 0;
        cpu.status.v = cpu.status.n != cpu.status.c;
        write_advance(cpu, entry, result as u16);
    }
}

fn ror_op<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    if entry.addr_mode == AddrMode::Acc {
        let val = reg_value(cpu, entry.acc);
        let result = (val >> 1) | ((cpu.status.c as u8) << 7);
        set_nz(cpu, result);
        cpu.status.c = (val & 0x01) != 0;
        cpu.status.v = cpu.status.n != cpu.status.c;
        reg_set(cpu, entry.acc, result);
    } else {
        let val = read_advance(cpu, entry) as u8;
        let result = (val >> 1) | ((cpu.status.c as u8) << 7);
        set_nz(cpu, result);
        cpu.status.c = (val & 0x01) != 0;
        cpu.status.v = cpu.status.n != cpu.status.c;
        write_advance(cpu, entry, result as u16);
    }
}

fn asr_op<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    if entry.addr_mode == AddrMode::Acc {
        let val = reg_value(cpu, entry.acc);
        let result = (val >> 1) | 0x80;
        set_nz(cpu, result);
        cpu.status.c = (val & 0x01) != 0;
        cpu.status.v = cpu.status.n != cpu.status.c;
        reg_set(cpu, entry.acc, result);
    } else {
        let val = read_advance(cpu, entry) as u8;
        let result = (val >> 1) | 0x80;
        set_nz(cpu, result);
        cpu.status.c = (val & 0x01) != 0;
        cpu.status.v = cpu.status.n != cpu.status.c;
        write_advance(cpu, entry, result as u16);
    }
}

fn asl_op<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    if entry.addr_mode == AddrMode::Acc {
        let val = reg_value(cpu, entry.acc);
        let result = (val as u16) << 1;
        cpu.status.n = (result & 0x80) != 0;
        cpu.status.z = (result & 0xFF) == 0;
        cpu.status.c = result > 0xFF;
        cpu.status.v = cpu.status.c != cpu.status.n;
        reg_set(cpu, entry.acc, result as u8);
    } else {
        let val = read_advance(cpu, entry) as u8;
        let result = (val as u16) << 1;
        cpu.status.n = (result & 0x80) != 0;
        cpu.status.z = (result & 0xFF) == 0;
        cpu.status.c = result > 0xFF;
        cpu.status.v = cpu.status.c != cpu.status.n;
        write_advance(cpu, entry, result as u16);
    }
}

fn rol_op<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    if entry.addr_mode == AddrMode::Acc {
        let val = reg_value(cpu, entry.acc);
        let result = ((val as u16) << 1) | (cpu.status.c as u16);
        cpu.status.n = (result & 0x80) != 0;
        cpu.status.z = (result & 0xFF) == 0;
        cpu.status.c = result > 0xFF;
        cpu.status.v = cpu.status.n != cpu.status.c;
        reg_set(cpu, entry.acc, result as u8);
    } else {
        let val = read_advance(cpu, entry) as u8;
        let result = ((val as u16) << 1) | (cpu.status.c as u16);
        cpu.status.n = (result & 0x80) != 0;
        cpu.status.z = (result & 0xFF) == 0;
        cpu.status.c = result > 0xFF;
        cpu.status.v = cpu.status.n != cpu.status.c;
        write_advance(cpu, entry, result as u16);
    }
}

fn dec<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    if entry.addr_mode == AddrMode::Acc {
        let val = reg_value(cpu, entry.acc);
        let result = val.wrapping_sub(1);
        set_nz(cpu, result);
        cpu.status.v = val == 0x80;
        reg_set(cpu, entry.acc, result);
    } else {
        let val = read_advance(cpu, entry) as u8;
        let result = val.wrapping_sub(1);
        set_nz(cpu, result);
        cpu.status.v = val == 0x80;
        write_advance(cpu, entry, result as u16);
    }
}

fn inc<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    if entry.addr_mode == AddrMode::Acc {
        let val = reg_value(cpu, entry.acc);
        let result = val.wrapping_add(1);
        set_nz(cpu, result);
        cpu.status.v = val == 0x7F;
        cpu.status.c = cpu.status.z;
        reg_set(cpu, entry.acc, result);
    } else {
        let val = read_advance(cpu, entry) as u8;
        let result = val.wrapping_add(1);
        set_nz(cpu, result);
        cpu.status.v = val == 0x7F;
        cpu.status.c = cpu.status.z;
        write_advance(cpu, entry, result as u16);
    }
}

fn tst<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    if entry.addr_mode == AddrMode::Acc {
        let val = reg_value(cpu, entry.acc);
        set_nz(cpu, val);
        cpu.status.v = false;
        cpu.status.c = false;
    } else {
        let val = read_advance(cpu, entry) as u8;
        set_nz(cpu, val);
        cpu.status.v = false;
        cpu.status.c = false;
    }
}

fn clr_op<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    if entry.addr_mode == AddrMode::Acc {
        reg_set(cpu, entry.acc, 0);
    } else {
        write_advance(cpu, entry, 0);
    }
    cpu.status.n = false;
    cpu.status.z = true;
    cpu.status.v = false;
    cpu.status.c = false;
}

fn jmp<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let target = read_advance(cpu, entry);
    cpu.reg.pc = target;
}

fn branch<M: MemoryBus>(cpu: &mut Cpu<M>, _entry: &OpcodeEntry, taken: bool) {
    let offset = cpu.memory.read(cpu.reg.pc);
    cpu.reg.pc = cpu.reg.pc.wrapping_add(1);
    if taken {
        if offset & 0x80 == 0 {
            cpu.reg.pc = cpu.reg.pc.wrapping_add(offset as u16);
        } else {
            cpu.reg.pc = cpu.reg.pc.wrapping_sub(((!offset).wrapping_add(1)) as u16);
        }
    }
}

fn bsr<M: MemoryBus>(cpu: &mut Cpu<M>, _entry: &OpcodeEntry) {
    let offset = cpu.memory.read(cpu.reg.pc);
    cpu.reg.pc = cpu.reg.pc.wrapping_add(1);
    let target = if offset & 0x80 == 0 {
        cpu.reg.pc.wrapping_add(offset as u16)
    } else {
        cpu.reg.pc.wrapping_sub(((!offset).wrapping_add(1)) as u16)
    };
    cpu.memory.write(cpu.reg.sp, cpu.reg.pc as u8);
    cpu.memory.write(cpu.reg.sp.wrapping_sub(1), (cpu.reg.pc >> 8) as u8);
    cpu.reg.sp = cpu.reg.sp.wrapping_sub(2);
    cpu.reg.pc = target;
}

fn jsr<M: MemoryBus>(cpu: &mut Cpu<M>, entry: &OpcodeEntry) {
    let target = read_advance(cpu, entry);
    cpu.memory.write(cpu.reg.sp, cpu.reg.pc as u8);
    cpu.memory.write(cpu.reg.sp.wrapping_sub(1), (cpu.reg.pc >> 8) as u8);
    cpu.reg.sp = cpu.reg.sp.wrapping_sub(2);
    cpu.reg.pc = target;
}
