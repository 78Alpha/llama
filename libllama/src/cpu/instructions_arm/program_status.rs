use cpu;
use cpu::{Cpu, Version, v5, v6};
use cpu::interpreter_arm as arm;

pub fn mrs<V: Version>(cpu: &mut Cpu<V>, data: arm::Mrs::Bf) -> cpu::InstrStatus {
    if !cpu::cond_passed(data.cond.get(), &cpu.cpsr) {
        return cpu::InstrStatus::InBlock;
    }

    let rd = data.rd.get();
    let r_bit = data.r_bit.get();

    if r_bit == 1 {
        cpu.regs[rd as usize] = cpu.get_current_spsr().val;
    } else {
        cpu.regs[rd as usize] = cpu.cpsr.val;
    }

    cpu::InstrStatus::InBlock
}

pub fn instr_msr<V: Version>(cpu: &mut Cpu<V>, data: arm::Msr1::Bf, immediate: bool) -> cpu::InstrStatus {
    if !cpu::cond_passed(data.cond.get(), &cpu.cpsr) {
        return cpu::InstrStatus::InBlock;
    }

    let field_mask = data.field_mask.get();
    let shifter_operand = data.shifter_operand.get();

    let val = if immediate {
        let immed_8 = bits!(shifter_operand, 0:7);
        let rotate_imm = bits!(shifter_operand, 8:11);
        immed_8.rotate_right(rotate_imm * 2)
    } else {
        cpu.regs[bits!(shifter_operand, 0:3) as usize]
    };

    let unalloc_mask = if V::is::<v5>() { 0x07FFFF00u32 } else { 0x06F0FC00 };
    let user_mask    = if V::is::<v5>() { 0xF8000000u32 } else { 0xF80F0200 };
    let priv_mask    = if V::is::<v5>() { 0x0000000Fu32 } else { 0x000001DF };
    let state_mask   = if V::is::<v5>() { 0x00000020u32 } else { 0x01000020 };

    if val & unalloc_mask != 0 {
        error!("Attempted to set reserved PSR bits through MSR instruction!");
    }

    let mut byte_mask = 0u32;
    byte_mask |= if bit!(field_mask, 0) == 1 { 0x000000FF } else { 0 };
    byte_mask |= if bit!(field_mask, 1) == 1 { 0x0000FF00 } else { 0 };
    byte_mask |= if bit!(field_mask, 2) == 1 { 0x00FF0000 } else { 0 };
    byte_mask |= if bit!(field_mask, 3) == 1 { 0xFF000000 } else { 0 };

    if data.r_bit.get() == 0 {
        // CPSR
        // TODO: Check privileges
        let cleared_cpsr = cpu.cpsr.val & !byte_mask;
        cpu.cpsr.val = cleared_cpsr | (val & byte_mask);

        if bit!(field_mask, 0) == 1 {
            // CPU mode may have been changed
            cpu.regs.swap(cpu::Mode::from_num(cpu.cpsr.mode.get()));
        }
    } else {
        // SPSR
        let spsr = cpu.get_current_spsr();
        byte_mask &= user_mask | priv_mask | state_mask;

        let cleared_spsr = spsr.val & !byte_mask;
        spsr.val = cleared_spsr | (val & byte_mask);
    }

    cpu::InstrStatus::InBlock
}

pub fn cps<V: Version>(cpu: &mut Cpu<V>, data: arm::Cps::Bf) -> cpu::InstrStatus {
    assert!(V::is::<v6>());
    
    if let cpu::Mode::Usr = cpu::Mode::from_num(cpu.cpsr.mode.get()) {
        return cpu::InstrStatus::InBlock
    }

    if data.imod.get() & 2 != 0 {
        let new = data.imod.get() & 1;
        if data.a_bit.get() != 0 { cpu.cpsr.disable_imp_abt.set(new) }
        if data.i_bit.get() != 0 { cpu.cpsr.disable_irq_bit.set(new) }
        if data.f_bit.get() != 0 { cpu.cpsr.disable_fiq_bit.set(new) }
    }
    if data.mmod.get() != 0 {
        cpu.cpsr.mode.set(data.mode.get());
        cpu.regs.swap(cpu::Mode::from_num(data.mode.get()));
    }

    cpu::InstrStatus::InBlock
}

pub fn msr_1<V: Version>(cpu: &mut Cpu<V>, data: arm::Msr1::Bf) -> cpu::InstrStatus {
    instr_msr(cpu, data, true)
}

pub fn msr_2<V: Version>(cpu: &mut Cpu<V>, data: arm::Msr2::Bf) -> cpu::InstrStatus {
    instr_msr(cpu, arm::Msr1::new(data.val), false)
}

