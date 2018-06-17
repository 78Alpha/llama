use cpu;
use cpu::instructions_thumb::*;

pub type InstFn = fn(&mut cpu::Cpu, u16) -> cpu::InstrStatus;
mod interpreter {
    use cpu;
    pub use cpu::instructions_thumb::*;
    pub fn undef(cpu: &mut cpu::Cpu, instr: u16) -> cpu::InstrStatus {
        panic!("Unimplemented instruction! {:#X}: {:?}", cpu.regs[15] - cpu.get_pc_offset(), instr)
    }
}

define_insts!(ThumbInstruction: u16, {
    with [ {}.16 ]
    {
        adc: [ {0b0100000101}.10; rm.3; rd.3 ],
        add_1: [ {0b0001110}.7; immed_3.3; rn.3; rd.3 ],
        add_2: [ {0b00110}.5; rd.3; immed_8.8 ],
        add_3: [ {0b0001100}.7; rm.3; rn.3; rd.3 ],
        add_4: [ {0b01000100}.8; h1.1; h2.1; rm.3; rd.3 ],
        add_5: [ {0b10100}.5; rd.3; immed_8.8 ],
        add_6: [ {0b10101}.5; rd.3; immed_8.8 ],
        add_7: [ {0b101100000}.9; immed_7.7 ],
        and: [ {0b0100000000}.10; rm.3; rd.3 ],
        asr_1: [ {0b00010}.5; immed_5.5; rm.3; rd.3 ],
        asr_2: [ {0b0100000100}.10; rs.3; rd.3 ],
        b_1: [ {0b1101}.4; cond.4; signed_imm_8.8 ],
        bic: [ {0b0100001110}.10; rm.3; rd.3 ],
        // bkpt: [ {0b10111110}.8; immed_8.8 ],
        branch: [ {0b111}.3; h_bits.2; offset_11.11 ],
        blx_2: [ {0b010001111}.9; h2.1; rm.3; {0b000}.3 ],
        bx: [ {0b010001110}.9; h2.1; rm.3; {0b000}.3 ],
        //cmn: [ {0b0100001011}.10; rm.3; rn.3 ],
        cmp_1: [ {0b00101}.5; rn.3; immed_8.8 ],
        cmp_2: [ {0b0100001010}.10; rm.3; rn.3 ],
        cmp_3: [ {0b01000101}.8; h1.1; h2.1; rm.3; rn.3 ],
        eor: [ {0b0100000001}.10; rm.3; rd.3 ],
        ldmia: [ {0b11001}.5; rn.3; register_list.8 ],
        ldr_1: [ {0b01101}.5; immed_5.5; rn.3; rd.3 ],
        ldr_2: [ {0b0101100}.7; rm.3; rn.3; rd.3 ],
        ldr_3: [ {0b01001}.5; rd.3; immed_8.8 ],
        ldr_4: [ {0b10011}.5; rd.3; immed_8.8 ],
        ldrb_1: [ {0b01111}.5; immed_5.5; rn.3; rd.3 ],
        ldrb_2: [ {0b0101110}.7; rm.3; rn.3; rd.3 ],
        ldrh_1: [ {0b10001}.5; immed_5.5; rn.3; rd.3 ],
        ldrh_2: [ {0b0101101}.7; rm.3; rn.3; rd.3 ],
        ldrsb: [ {0b0101011}.7; rm.3; rn.3; rd.3 ],
        ldrsh: [ {0b0101111}.7; rm.3; rn.3; rd.3 ],
        lsl_1: [ {0b00000}.5; immed_5.5; rm.3; rd.3 ],
        lsl_2: [ {0b0100000010}.10; rs.3; rd.3 ],
        lsr_1: [ {0b00001}.5; immed_5.5; rm.3; rd.3 ],
        lsr_2: [ {0b0100000011}.10; rs.3; rd.3 ],
        mov_1: [ {0b00100}.5; rd.3; immed_8.8 ],
        mov_2: [ {0b0001110000}.10; rn.3; rd.3 ],
        mov_3: [ {0b01000110}.8; h1.1; h2.1; rm.3; rd.3 ],
        mul: [ {0b0100001101}.10; rm.3; rd.3 ],
        mvn: [ {0b0100001111}.10; rm.3; rd.3 ],
        neg: [ {0b0100001001}.10; rm.3; rd.3 ],
        orr: [ {0b0100001100}.10; rm.3; rd.3 ],
        pop: [ {0b1011110}.7; r_bit.1; register_list.8 ],
        push: [ {0b1011010}.7; r_bit.1; register_list.8 ],
        ror: [ {0b0100000111}.10; rs.3; rd.3 ],
        sbc: [ {0b0100000110}.10; rm.3; rd.3 ],
        stmia: [ {0b11000}.5; rn.3; register_list.8 ],
        str_1: [ {0b01100}.5; immed_5.5; rn.3; rd.3 ],
        str_2: [ {0b0101000}.7; rm.3; rn.3; rd.3 ],
        str_3: [ {0b10010}.5; rd.3; immed_8.8 ],
        strb_1: [ {0b01110}.5; immed_5.5; rn.3; rd.3 ],
        strb_2: [ {0b0101010}.7; rm.3; rn.3; rd.3 ],
        strh_1: [ {0b10000}.5; immed_5.5; rn.3; rd.3 ],
        strh_2: [ {0b0101001}.7; rm.3; rn.3; rd.3 ],
        sub_1: [ {0b0001111}.7; immed_3.3; rn.3; rd.3 ],
        sub_2: [ {0b00111}.5; rd.3; immed_8.8 ],
        sub_3: [ {0b0001101}.7; rm.3; rn.3; rd.3 ],
        sub_4: [ {0b101100001}.9; immed_7.7 ],
        // swi: [ {0b11011111}.8; immed_8.8 ],
        tst: [ {0b0100001000}.10; rm.3; rn.3 ]
    }
});