use m6800::{Cpu, FlatMemory, MemoryBus, StatusFlags};

fn test_cpu() -> Cpu<FlatMemory> {
    let mem = FlatMemory::with_capacity(0x10000);
    let mut cpu = Cpu::new(mem);
    cpu.reset = true;
    cpu
}

fn setup_reset_vector(cpu: &mut Cpu<FlatMemory>, addr: u16) {
    cpu.memory.write(0xFFFE, (addr >> 8) as u8);
    cpu.memory.write(0xFFFF, addr as u8);
}

#[test]
fn test_nop() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x01); // NOP
    cpu.memory.write(0x0001, 0x01); // NOP
    cpu.memory.write(0x0002, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    assert_eq!(cpu.reg.pc, 1);
    cpu.step();
    assert_eq!(cpu.reg.pc, 2);
}

#[test]
fn test_lda_immediate() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$42
    cpu.memory.write(0x0001, 0x42);
    cpu.memory.write(0x0002, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    assert_eq!(cpu.reg.a, 0x42);
    assert!(!cpu.status.z);
    assert!(!cpu.status.n);
}

#[test]
fn test_lda_then_sta() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$55
    cpu.memory.write(0x0001, 0x55);
    cpu.memory.write(0x0002, 0xB7); // STA $1234
    cpu.memory.write(0x0003, 0x12);
    cpu.memory.write(0x0004, 0x34);
    cpu.memory.write(0x0005, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    assert_eq!(cpu.reg.a, 0x55);
    cpu.step();
    assert_eq!(cpu.memory.read(0x1234), 0x55);
}

#[test]
fn test_add() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$10
    cpu.memory.write(0x0001, 0x10);
    cpu.memory.write(0x0002, 0x8B); // ADD #$20
    cpu.memory.write(0x0003, 0x20);
    cpu.memory.write(0x0004, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    cpu.step();
    assert_eq!(cpu.reg.a, 0x30);
    assert!(!cpu.status.z);
    assert!(!cpu.status.n);
    assert!(!cpu.status.c);
}

#[test]
fn test_add_with_carry() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$FF
    cpu.memory.write(0x0001, 0xFF);
    cpu.memory.write(0x0002, 0x8B); // ADD #$01
    cpu.memory.write(0x0003, 0x01);
    cpu.memory.write(0x0004, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    cpu.step();
    assert_eq!(cpu.reg.a, 0x00);
    assert!(cpu.status.z);
    assert!(cpu.status.c);
    assert!(!cpu.status.n);
}

#[test]
fn test_sub() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$05
    cpu.memory.write(0x0001, 0x05);
    cpu.memory.write(0x0002, 0x80); // SUB #$03
    cpu.memory.write(0x0003, 0x03);
    cpu.memory.write(0x0004, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    cpu.step();
    assert_eq!(cpu.reg.a, 0x02);
    assert!(!cpu.status.z);
    assert!(!cpu.status.n);
    assert!(!cpu.status.c);
}

#[test]
fn test_and() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$FF
    cpu.memory.write(0x0001, 0xFF);
    cpu.memory.write(0x0002, 0x84); // AND #$0F
    cpu.memory.write(0x0003, 0x0F);
    cpu.memory.write(0x0004, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    cpu.step();
    assert_eq!(cpu.reg.a, 0x0F);
    assert!(!cpu.status.z);
}

#[test]
fn test_ora() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$F0
    cpu.memory.write(0x0001, 0xF0);
    cpu.memory.write(0x0002, 0x8A); // ORA #$0F
    cpu.memory.write(0x0003, 0x0F);
    cpu.memory.write(0x0004, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    cpu.step();
    assert_eq!(cpu.reg.a, 0xFF);
}

#[test]
fn test_eor() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$FF
    cpu.memory.write(0x0001, 0xFF);
    cpu.memory.write(0x0002, 0x88); // EOR #$FF
    cpu.memory.write(0x0003, 0xFF);
    cpu.memory.write(0x0004, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    cpu.step();
    assert_eq!(cpu.reg.a, 0x00);
    assert!(cpu.status.z);
}

#[test]
fn test_branch_taken() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$00
    cpu.memory.write(0x0001, 0x00);
    cpu.memory.write(0x0002, 0x27); // BEQ +1 (to 0x0005)
    cpu.memory.write(0x0003, 0x01);
    cpu.memory.write(0x0004, 0x01); // NOP (skipped)
    cpu.memory.write(0x0005, 0x86); // LDA #$01
    cpu.memory.write(0x0006, 0x01);
    cpu.memory.write(0x0007, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    cpu.step(); // BEQ taken
    cpu.step(); // LDA $01
    assert_eq!(cpu.reg.a, 0x01);
}

#[test]
fn test_branch_not_taken() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$01
    cpu.memory.write(0x0001, 0x01);
    cpu.memory.write(0x0002, 0x27); // BEQ +2 (not taken)
    cpu.memory.write(0x0003, 0x02);
    cpu.memory.write(0x0004, 0x86); // LDA #$02
    cpu.memory.write(0x0005, 0x02);
    cpu.memory.write(0x0006, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    cpu.step(); // BEQ not taken
    cpu.step(); // LDA $02
    assert_eq!(cpu.reg.a, 0x02);
}

#[test]
fn test_jsr_rts() {
    let mut cpu = test_cpu();
    cpu.reg.sp = 0xFF;
    cpu.memory.write(0x0000, 0xBD); // JSR $0100
    cpu.memory.write(0x0001, 0x01);
    cpu.memory.write(0x0002, 0x00);
    cpu.memory.write(0x0003, 0x01); // NOP (return here)
    cpu.memory.write(0x0004, 0x3E); // WAI
    cpu.memory.write(0x0100, 0x86); // LDA #$42
    cpu.memory.write(0x0101, 0x42);
    cpu.memory.write(0x0102, 0x39); // RTS
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step(); // JSR -> jump to 0x0100, push return addr (0x0003)
    cpu.step(); // LDA #$42
    cpu.step(); // RTS -> pop return addr (0x0003)
    assert_eq!(cpu.reg.a, 0x42);
    assert_eq!(cpu.reg.pc, 0x0003);
}

#[test]
fn test_psh_pul() {
    let mut cpu = test_cpu();
    cpu.reg.sp = 0xFF;
    cpu.memory.write(0x0000, 0x86); // LDA #$55
    cpu.memory.write(0x0001, 0x55);
    cpu.memory.write(0x0002, 0x36); // PSH A
    cpu.memory.write(0x0003, 0x4F); // CLR A
    cpu.memory.write(0x0004, 0x32); // PUL A
    cpu.memory.write(0x0005, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    cpu.step(); // PSH A
    cpu.step(); // CLR A
    assert_eq!(cpu.reg.a, 0x00);
    cpu.step(); // PUL A
    assert_eq!(cpu.reg.a, 0x55);
}

#[test]
fn test_inx_dex() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0xCE); // LDX #$0100
    cpu.memory.write(0x0001, 0x01);
    cpu.memory.write(0x0002, 0x00);
    cpu.memory.write(0x0003, 0x08); // INX
    cpu.memory.write(0x0004, 0x09); // DEX
    cpu.memory.write(0x0005, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step(); // LDX #$0100
    assert_eq!(cpu.reg.ix, 0x0100);
    cpu.step(); // INX
    assert_eq!(cpu.reg.ix, 0x0101);
    cpu.step(); // DEX
    assert_eq!(cpu.reg.ix, 0x0100);
}

#[test]
fn test_flag_operations() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x0D); // SEC
    cpu.memory.write(0x0001, 0x0C); // CLC
    cpu.memory.write(0x0002, 0x0F); // SEI
    cpu.memory.write(0x0003, 0x0E); // CLI
    cpu.memory.write(0x0004, 0x0B); // SEV
    cpu.memory.write(0x0005, 0x0A); // CLV
    cpu.memory.write(0x0006, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step(); assert!(cpu.status.c);
    cpu.step(); assert!(!cpu.status.c);
    cpu.step(); assert!(cpu.status.i);
    cpu.step(); assert!(!cpu.status.i);
    cpu.step(); assert!(cpu.status.v);
    cpu.step(); assert!(!cpu.status.v);
}

#[test]
fn test_aba() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$12
    cpu.memory.write(0x0001, 0x12);
    cpu.memory.write(0x0002, 0xC6); // LDB #$34
    cpu.memory.write(0x0003, 0x34);
    cpu.memory.write(0x0004, 0x1B); // ABA
    cpu.memory.write(0x0005, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    cpu.step();
    cpu.step();
    assert_eq!(cpu.reg.a, 0x46);
}

#[test]
fn test_instruction_count() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x01); // NOP (2 cycles)
    cpu.memory.write(0x0001, 0x01); // NOP (2 cycles)
    cpu.memory.write(0x0002, 0x01); // NOP (2 cycles)
    cpu.memory.write(0x0003, 0x3E); // WAI (9 cycles)
    setup_reset_vector(&mut cpu, 0x0000);

    let cycles = cpu.step(); // NOP
    assert_eq!(cycles, 2);
    let cycles = cpu.step(); // NOP
    assert_eq!(cycles, 2);
    let cycles = cpu.step(); // NOP
    assert_eq!(cycles, 2);
    let cycles = cpu.step(); // WAI
    assert_eq!(cycles, 9);
}

#[test]
fn test_direct_addressing() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x00A0, 0x42);
    cpu.memory.write(0x0000, 0x96); // LDA $A0 (direct)
    cpu.memory.write(0x0001, 0xA0);
    cpu.memory.write(0x0002, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    assert_eq!(cpu.reg.a, 0x42);
}

#[test]
fn test_indexed_addressing() {
    let mut cpu = test_cpu();
    cpu.reg.ix = 0x0100;
    cpu.memory.write(0x0110, 0x55);
    cpu.memory.write(0x0000, 0xA6); // LDA $10,X
    cpu.memory.write(0x0001, 0x10);
    cpu.memory.write(0x0002, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    assert_eq!(cpu.reg.a, 0x55);
}

#[test]
fn test_invalid_opcode_treated_as_nop_and_halt() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x12); // Invalid opcode
    cpu.memory.write(0x0001, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    let _cyc = cpu.step();
    assert!(cpu.halt);
}

#[test]
fn test_flag_from_byte_roundtrip() {
    let flags = StatusFlags::from_byte(0b00100001);
    assert!(flags.c);
    assert!(!flags.v);
    assert!(!flags.z);
    assert!(!flags.n);
    assert!(!flags.i);
    assert!(flags.h);

    let byte = flags.to_byte();
    assert!(byte & 0x01 != 0);
    assert!(byte & 0x20 != 0);
    assert!(byte & 0xC0 != 0);
}

#[test]
fn test_reset_vector() {
    let mut cpu = test_cpu();
    cpu.memory.write(0xFFFE, 0x12);
    cpu.memory.write(0xFFFF, 0x34);
    setup_reset_vector(&mut cpu, 0x1234);
    cpu.step();
    assert_eq!(cpu.reg.pc, 0x1235);
}

#[test]
fn test_inc_dec() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$05
    cpu.memory.write(0x0001, 0x05);
    cpu.memory.write(0x0002, 0x4C); // INC A
    cpu.memory.write(0x0003, 0x4A); // DEC A
    cpu.memory.write(0x0004, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    assert_eq!(cpu.reg.a, 0x05);
    cpu.step(); // INC A
    assert_eq!(cpu.reg.a, 0x06);
    cpu.step(); // DEC A
    assert_eq!(cpu.reg.a, 0x05);
}

#[test]
fn test_asl_lsr() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$80
    cpu.memory.write(0x0001, 0x80);
    cpu.memory.write(0x0002, 0x48); // ASL A
    cpu.memory.write(0x0003, 0x44); // LSR A
    cpu.memory.write(0x0004, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    assert_eq!(cpu.reg.a, 0x80);
    cpu.step(); // ASL A - 0x80 << 1 = 0x00, carry = 1
    assert_eq!(cpu.reg.a, 0x00);
    assert!(cpu.status.c);
    assert!(cpu.status.z);
    cpu.step(); // LSR A - 0x00 >> 1 = 0x00
    assert_eq!(cpu.reg.a, 0x00);
}

#[test]
fn test_rol_ror() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0x86); // LDA #$81
    cpu.memory.write(0x0001, 0x81);
    cpu.memory.write(0x0002, 0x49); // ROL A
    cpu.memory.write(0x0003, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    // ROL A: 0x81 << 1 | carry(0) = 0x02, carry = 1
    cpu.step();
    assert_eq!(cpu.reg.a, 0x02);
    assert!(cpu.status.c);
}

#[test]
fn test_ldx_immediate() {
    let mut cpu = test_cpu();
    cpu.memory.write(0x0000, 0xCE); // LDX #$1234
    cpu.memory.write(0x0001, 0x12);
    cpu.memory.write(0x0002, 0x34);
    cpu.memory.write(0x0003, 0x3E); // WAI
    setup_reset_vector(&mut cpu, 0x0000);
    cpu.step();
    assert_eq!(cpu.reg.ix, 0x1234);
    assert!(!cpu.status.z);
}
