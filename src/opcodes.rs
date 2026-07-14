#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddrMode {
    Inh,
    Acc,
    Imm,
    Imm16,
    Dir,
    Dir16,
    Ext,
    Ext16,
    Idx,
    Idx16,
    Immx,
    Rel,
}

#[derive(Clone, Copy, Debug)]
pub struct OpcodeEntry {
    pub instruction: &'static str,
    pub addr_mode: AddrMode,
    pub acc: &'static str,
    pub cycles: u8,
}

const fn op_inh(inst: &'static str, cycles: u8) -> OpcodeEntry {
    OpcodeEntry {
        instruction: inst,
        addr_mode: AddrMode::Inh,
        acc: "",
        cycles,
    }
}

const fn op_rel(inst: &'static str, cycles: u8) -> OpcodeEntry {
    OpcodeEntry {
        instruction: inst,
        addr_mode: AddrMode::Rel,
        acc: "",
        cycles,
    }
}

const fn op_imm(inst: &'static str, acc: &'static str, cycles: u8) -> OpcodeEntry {
    OpcodeEntry {
        instruction: inst,
        addr_mode: AddrMode::Imm,
        acc,
        cycles,
    }
}

const fn op_imm16(inst: &'static str, cycles: u8) -> OpcodeEntry {
    OpcodeEntry {
        instruction: inst,
        addr_mode: AddrMode::Imm16,
        acc: "",
        cycles,
    }
}

const fn op_dir(inst: &'static str, acc: &'static str, cycles: u8) -> OpcodeEntry {
    OpcodeEntry {
        instruction: inst,
        addr_mode: AddrMode::Dir,
        acc,
        cycles,
    }
}

const fn op_dir16(inst: &'static str, cycles: u8) -> OpcodeEntry {
    OpcodeEntry {
        instruction: inst,
        addr_mode: AddrMode::Dir16,
        acc: "",
        cycles,
    }
}

const fn op_ext(inst: &'static str, acc: &'static str, cycles: u8) -> OpcodeEntry {
    OpcodeEntry {
        instruction: inst,
        addr_mode: AddrMode::Ext,
        acc,
        cycles,
    }
}

const fn op_ext16(inst: &'static str, cycles: u8) -> OpcodeEntry {
    OpcodeEntry {
        instruction: inst,
        addr_mode: AddrMode::Ext16,
        acc: "",
        cycles,
    }
}

const fn op_idx(inst: &'static str, acc: &'static str, cycles: u8) -> OpcodeEntry {
    OpcodeEntry {
        instruction: inst,
        addr_mode: AddrMode::Idx,
        acc,
        cycles,
    }
}

const fn op_idx16(inst: &'static str, cycles: u8) -> OpcodeEntry {
    OpcodeEntry {
        instruction: inst,
        addr_mode: AddrMode::Idx16,
        acc: "",
        cycles,
    }
}

const fn op_acc(inst: &'static str, acc: &'static str, cycles: u8) -> OpcodeEntry {
    OpcodeEntry {
        instruction: inst,
        addr_mode: AddrMode::Acc,
        acc,
        cycles,
    }
}

const fn op_immx(inst: &'static str, cycles: u8) -> OpcodeEntry {
    OpcodeEntry {
        instruction: inst,
        addr_mode: AddrMode::Immx,
        acc: "",
        cycles,
    }
}

const EMPTY: OpcodeEntry = OpcodeEntry {
    instruction: "",
    addr_mode: AddrMode::Inh,
    acc: "",
    cycles: 0,
};

pub static OPCODES: [OpcodeEntry; 256] = {
    let mut table: [OpcodeEntry; 256] = [EMPTY; 256];

    table[0x00] = EMPTY;
    table[0x01] = op_inh("nop", 2);
    table[0x02] = EMPTY;
    table[0x03] = EMPTY;
    table[0x04] = EMPTY;
    table[0x05] = EMPTY;
    table[0x06] = op_inh("tap", 2);
    table[0x07] = op_inh("tpa", 2);
    table[0x08] = op_inh("inx", 4);
    table[0x09] = op_inh("dex", 4);
    table[0x0A] = op_inh("clv", 2);
    table[0x0B] = op_inh("sev", 2);
    table[0x0C] = op_inh("clc", 2);
    table[0x0D] = op_inh("sec", 2);
    table[0x0E] = op_inh("cli", 2);
    table[0x0F] = op_inh("sei", 2);
    table[0x10] = op_inh("sba", 2);
    table[0x11] = op_inh("cba", 2);
    table[0x12] = EMPTY;
    table[0x13] = EMPTY;
    table[0x14] = op_inh("nba", 2);
    table[0x15] = EMPTY;
    table[0x16] = op_inh("tab", 2);
    table[0x17] = op_inh("tba", 2);
    table[0x18] = EMPTY;
    table[0x19] = op_inh("daa", 2);
    table[0x1A] = EMPTY;
    table[0x1B] = op_inh("aba", 2);
    table[0x1C] = EMPTY;
    table[0x1D] = EMPTY;
    table[0x1E] = EMPTY;
    table[0x1F] = EMPTY;

    table[0x20] = op_rel("bra", 4);
    table[0x21] = EMPTY;
    table[0x22] = op_rel("bhi", 4);
    table[0x23] = op_rel("bls", 4);
    table[0x24] = op_rel("bcc", 4);
    table[0x25] = op_rel("bcs", 4);
    table[0x26] = op_rel("bne", 4);
    table[0x27] = op_rel("beq", 4);
    table[0x28] = op_rel("bvc", 4);
    table[0x29] = op_rel("bvs", 4);
    table[0x2A] = op_rel("bpl", 4);
    table[0x2B] = op_rel("bmi", 4);
    table[0x2C] = op_rel("bge", 4);
    table[0x2D] = op_rel("blt", 4);
    table[0x2E] = op_rel("bgt", 4);
    table[0x2F] = op_rel("ble", 4);

    table[0x30] = op_inh("tsx", 4);
    table[0x31] = op_inh("ins", 4);
    table[0x32] = op_acc("pul", "a", 4);
    table[0x33] = op_acc("pul", "b", 4);
    table[0x34] = op_inh("des", 4);
    table[0x35] = op_inh("txs", 4);
    table[0x36] = op_acc("psh", "a", 4);
    table[0x37] = op_acc("psh", "b", 4);
    table[0x38] = EMPTY;
    table[0x39] = op_inh("rts", 5);
    table[0x3A] = EMPTY;
    table[0x3B] = op_inh("rti", 10);
    table[0x3C] = EMPTY;
    table[0x3D] = EMPTY;
    table[0x3E] = op_inh("wai", 9);
    table[0x3F] = op_inh("swi", 12);

    table[0x40] = op_acc("neg", "a", 2);
    table[0x41] = EMPTY;
    table[0x42] = EMPTY;
    table[0x43] = op_acc("com", "a", 2);
    table[0x44] = op_acc("lsr", "a", 2);
    table[0x45] = EMPTY;
    table[0x46] = op_acc("ror", "a", 2);
    table[0x47] = op_acc("asr", "a", 2);
    table[0x48] = op_acc("asl", "a", 2);
    table[0x49] = op_acc("rol", "a", 2);
    table[0x4A] = op_acc("dec", "a", 2);
    table[0x4B] = EMPTY;
    table[0x4C] = op_acc("inc", "a", 2);
    table[0x4D] = op_acc("tst", "a", 2);
    table[0x4E] = EMPTY;
    table[0x4F] = op_acc("clr", "a", 2);

    table[0x50] = op_acc("neg", "b", 2);
    table[0x51] = EMPTY;
    table[0x52] = EMPTY;
    table[0x53] = op_acc("com", "b", 2);
    table[0x54] = op_acc("lsr", "b", 2);
    table[0x55] = EMPTY;
    table[0x56] = op_acc("ror", "b", 2);
    table[0x57] = op_acc("asr", "b", 2);
    table[0x58] = op_acc("asl", "b", 2);
    table[0x59] = op_acc("rol", "b", 2);
    table[0x5A] = op_acc("dec", "b", 2);
    table[0x5B] = EMPTY;
    table[0x5C] = op_acc("inc", "b", 2);
    table[0x5D] = op_acc("tst", "b", 2);
    table[0x5E] = EMPTY;
    table[0x5F] = op_acc("clr", "b", 2);

    table[0x60] = op_idx("neg", "", 7);
    table[0x61] = EMPTY;
    table[0x62] = EMPTY;
    table[0x63] = op_idx("com", "", 7);
    table[0x64] = op_idx("lsr", "", 7);
    table[0x65] = EMPTY;
    table[0x66] = op_idx("ror", "", 7);
    table[0x67] = op_idx("asr", "", 7);
    table[0x68] = op_idx("asl", "", 7);
    table[0x69] = op_idx("rol", "", 7);
    table[0x6A] = op_idx("dec", "", 7);
    table[0x6B] = EMPTY;
    table[0x6C] = op_idx("inc", "", 7);
    table[0x6D] = op_idx("tst", "", 7);
    table[0x6E] = op_immx("jmp", 2);
    table[0x6F] = op_idx("clr", "", 7);

    table[0x70] = op_ext("neg", "", 6);
    table[0x71] = EMPTY;
    table[0x72] = EMPTY;
    table[0x73] = op_ext("com", "", 6);
    table[0x74] = op_ext("lsr", "", 6);
    table[0x75] = EMPTY;
    table[0x76] = op_ext("ror", "", 6);
    table[0x77] = op_ext("asr", "", 6);
    table[0x78] = op_ext("asl", "", 6);
    table[0x79] = op_ext("rol", "", 6);
    table[0x7A] = op_ext("dec", "", 6);
    table[0x7B] = EMPTY;
    table[0x7C] = op_ext("inc", "", 6);
    table[0x7D] = op_ext("tst", "", 6);
    table[0x7E] = op_imm16("jmp", 3);
    table[0x7F] = op_ext("clr", "", 6);

    table[0x80] = op_imm("sub", "a", 2);
    table[0x81] = op_imm("cmp", "a", 2);
    table[0x82] = op_imm("sbc", "a", 2);
    table[0x83] = EMPTY;
    table[0x84] = op_imm("and", "a", 2);
    table[0x85] = op_imm("bit", "a", 2);
    table[0x86] = op_imm("lda", "a", 2);
    table[0x87] = op_imm("sta", "a", 2);
    table[0x88] = op_imm("eor", "a", 2);
    table[0x89] = op_imm("adc", "a", 2);
    table[0x8A] = op_imm("ora", "a", 2);
    table[0x8B] = op_imm("add", "a", 2);
    table[0x8C] = op_imm16("cpx", 3);
    table[0x8D] = op_rel("bsr", 8);
    table[0x8E] = op_imm16("lds", 3);
    table[0x8F] = op_imm16("sts", 2);

    table[0x90] = op_dir("sub", "a", 3);
    table[0x91] = op_dir("cmp", "a", 3);
    table[0x92] = op_dir("sbc", "a", 3);
    table[0x93] = EMPTY;
    table[0x94] = op_dir("and", "a", 3);
    table[0x95] = op_dir("bit", "a", 3);
    table[0x96] = op_dir("lda", "a", 3);
    table[0x97] = op_dir("sta", "a", 4);
    table[0x98] = op_dir("eor", "a", 3);
    table[0x99] = op_dir("adc", "a", 3);
    table[0x9A] = op_dir("ora", "a", 3);
    table[0x9B] = op_dir("add", "a", 3);
    table[0x9C] = op_dir16("cpx", 4);
    table[0x9D] = op_inh("hcf", 0);
    table[0x9E] = op_dir16("lds", 4);
    table[0x9F] = op_dir16("sts", 5);

    table[0xA0] = op_idx("sub", "a", 5);
    table[0xA1] = op_idx("cmp", "a", 5);
    table[0xA2] = op_idx("sbc", "a", 5);
    table[0xA3] = EMPTY;
    table[0xA4] = op_idx("and", "a", 5);
    table[0xA5] = op_idx("bit", "a", 5);
    table[0xA6] = op_idx("lda", "a", 5);
    table[0xA7] = op_idx("sta", "a", 6);
    table[0xA8] = op_idx("eor", "a", 5);
    table[0xA9] = op_idx("adc", "a", 5);
    table[0xAA] = op_idx("ora", "a", 5);
    table[0xAB] = op_idx("add", "a", 5);
    table[0xAC] = op_idx16("cpx", 6);
    table[0xAD] = op_immx("jsr", 8);
    table[0xAE] = op_idx16("lds", 6);
    table[0xAF] = op_idx16("sts", 7);

    table[0xB0] = op_ext("sub", "a", 4);
    table[0xB1] = op_ext("cmp", "a", 4);
    table[0xB2] = op_ext("sbc", "a", 4);
    table[0xB3] = EMPTY;
    table[0xB4] = op_ext("and", "a", 4);
    table[0xB5] = op_ext("bit", "a", 4);
    table[0xB6] = op_ext("lda", "a", 4);
    table[0xB7] = op_ext("sta", "a", 5);
    table[0xB8] = op_ext("eor", "a", 4);
    table[0xB9] = op_ext("adc", "a", 4);
    table[0xBA] = op_ext("ora", "a", 4);
    table[0xBB] = op_ext("add", "a", 4);
    table[0xBC] = op_ext16("cpx", 5);
    table[0xBD] = op_imm16("jsr", 9);
    table[0xBE] = op_ext16("lds", 5);
    table[0xBF] = op_ext16("sts", 6);

    table[0xC0] = op_imm("sub", "b", 2);
    table[0xC1] = op_imm("cmp", "b", 2);
    table[0xC2] = op_imm("sbc", "b", 2);
    table[0xC3] = EMPTY;
    table[0xC4] = op_imm("and", "b", 2);
    table[0xC5] = op_imm("bit", "b", 2);
    table[0xC6] = op_imm("lda", "b", 2);
    table[0xC7] = op_imm("sta", "b", 2);
    table[0xC8] = op_imm("eor", "b", 2);
    table[0xC9] = op_imm("adc", "b", 2);
    table[0xCA] = op_imm("ora", "b", 2);
    table[0xCB] = op_imm("add", "b", 2);
    table[0xCC] = EMPTY;
    table[0xCD] = op_inh("hcf", 0);
    table[0xCE] = op_imm16("ldx", 3);
    table[0xCF] = op_imm16("stx", 2);

    table[0xD0] = op_dir("sub", "b", 3);
    table[0xD1] = op_dir("cmp", "b", 3);
    table[0xD2] = op_dir("sbc", "b", 3);
    table[0xD3] = EMPTY;
    table[0xD4] = op_dir("and", "b", 3);
    table[0xD5] = op_dir("bit", "b", 3);
    table[0xD6] = op_dir("lda", "b", 3);
    table[0xD7] = op_dir("sta", "b", 4);
    table[0xD8] = op_dir("eor", "b", 3);
    table[0xD9] = op_dir("adc", "b", 3);
    table[0xDA] = op_dir("ora", "b", 3);
    table[0xDB] = op_dir("add", "b", 3);
    table[0xDC] = EMPTY;
    table[0xDD] = op_inh("hcf", 0);
    table[0xDE] = op_dir16("ldx", 4);
    table[0xDF] = op_dir16("stx", 5);

    table[0xE0] = op_idx("sub", "b", 5);
    table[0xE1] = op_idx("cmp", "b", 5);
    table[0xE2] = op_idx("sbc", "b", 5);
    table[0xE3] = EMPTY;
    table[0xE4] = op_idx("and", "b", 5);
    table[0xE5] = op_idx("bit", "b", 5);
    table[0xE6] = op_idx("lda", "b", 5);
    table[0xE7] = op_idx("sta", "b", 6);
    table[0xE8] = op_idx("eor", "b", 5);
    table[0xE9] = op_idx("adc", "b", 5);
    table[0xEA] = op_idx("ora", "b", 5);
    table[0xEB] = op_idx("add", "b", 5);
    table[0xEC] = EMPTY;
    table[0xED] = op_inh("hcf", 0);
    table[0xEE] = op_idx16("ldx", 6);
    table[0xEF] = op_idx16("stx", 7);

    table[0xF0] = op_ext("sub", "b", 4);
    table[0xF1] = op_ext("cmp", "b", 4);
    table[0xF2] = op_ext("sbc", "b", 4);
    table[0xF3] = EMPTY;
    table[0xF4] = op_ext("and", "b", 4);
    table[0xF5] = op_ext("bit", "b", 4);
    table[0xF6] = op_ext("lda", "b", 4);
    table[0xF7] = op_ext("sta", "b", 5);
    table[0xF8] = op_ext("eor", "b", 4);
    table[0xF9] = op_ext("adc", "b", 4);
    table[0xFA] = op_ext("ora", "b", 4);
    table[0xFB] = op_ext("add", "b", 4);
    table[0xFC] = EMPTY;
    table[0xFD] = op_inh("hcf", 0);
    table[0xFE] = op_ext16("ldx", 5);
    table[0xFF] = op_ext16("stx", 6);

    table
};
