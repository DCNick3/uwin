use iced_x86::{
    ConditionCode, Decoder, DecoderOptions, Instruction, InstructionInfoFactory, OpKind, RflagsBits,
};
use iced_x86::OpKind::FarBranch32;

pub(crate) fn main() {
    let mut decoder = Decoder::with_ip(
        EXAMPLE_CODE_BITNESS,
        EXAMPLE_CODE,
        EXAMPLE_CODE_RIP,
        DecoderOptions::NONE,
    );

    // Use a factory to create the instruction info if you need register and
    // memory usage. If it's something else, eg. encoding, flags, etc, there
    // are Instruction methods that can be used instead.
    let mut info_factory = InstructionInfoFactory::new();
    let mut instr = Instruction::default();
    while decoder.can_decode() {
        decoder.decode_out(&mut instr);

        // Gets offsets in the instruction of the displacement and immediates and their sizes.
        // This can be useful if there are relocations in the binary. The encoder has a similar
        // method. This method must be called after decode() and you must pass in the last
        // instruction decode() returned.
        let offsets = decoder.get_constant_offsets(&instr);

        // For quick hacks, it's fine to use the Display trait to format an instruction,
        // but for real code, use a formatter, eg. MasmFormatter. See other examples.
        println!("{:016X} {}", instr.ip(), instr);

        let op_code = instr.op_code();
        let info = info_factory.info(&instr);
        let fpu_info = instr.fpu_stack_increment_info();
        println!("    OpCode: {}", op_code.op_code_string());
        println!("    Instruction: {}", op_code.instruction_string());
        println!("    Encoding: {:?}", instr.encoding());
        println!("    Mnemonic: {:?}", instr.mnemonic());
        println!("    Code: {:?}", instr.code());
        println!(
            "    CpuidFeature: {}",
            instr
                .cpuid_features()
                .iter()
                .map(|&a| format!("{:?}", a))
                .collect::<Vec<String>>()
                .join(" and ")
        );
        println!("    FlowControl: {:?}", instr.flow_control());
        if fpu_info.writes_top() {
            if fpu_info.increment() == 0 {
                println!("    FPU TOP: the instruction overwrites TOP");
            } else {
                println!("    FPU TOP inc: {}", fpu_info.increment());
            }
            println!(
                "    FPU TOP cond write: {}",
                if fpu_info.conditional() { "true" } else { "false" }
            );
        }
        if offsets.has_displacement() {
            println!(
                "    Displacement offset = {}, size = {}",
                offsets.displacement_offset(),
                offsets.displacement_size()
            );
        }
        if offsets.has_immediate() {
            println!(
                "    Immediate offset = {}, size = {}",
                offsets.immediate_offset(),
                offsets.immediate_size()
            );
        }
        if offsets.has_immediate2() {
            println!(
                "    Immediate #2 offset = {}, size = {}",
                offsets.immediate_offset2(),
                offsets.immediate_size2()
            );
        }
        if instr.is_stack_instruction() {
            println!("    SP Increment: {}", instr.stack_pointer_increment());
        }
        if instr.condition_code() != ConditionCode::None {
            println!("    Condition code: {:?}", instr.condition_code());
        }
        if instr.rflags_read() != RflagsBits::NONE {
            println!("    RFLAGS Read: {}", flags(instr.rflags_read()));
        }
        if instr.rflags_written() != RflagsBits::NONE {
            println!("    RFLAGS Written: {}", flags(instr.rflags_written()));
        }
        if instr.rflags_cleared() != RflagsBits::NONE {
            println!("    RFLAGS Cleared: {}", flags(instr.rflags_cleared()));
        }
        if instr.rflags_set() != RflagsBits::NONE {
            println!("    RFLAGS Set: {}", flags(instr.rflags_set()));
        }
        if instr.rflags_undefined() != RflagsBits::NONE {
            println!("    RFLAGS Undefined: {}", flags(instr.rflags_undefined()));
        }
        if instr.rflags_modified() != RflagsBits::NONE {
            println!("    RFLAGS Modified: {}", flags(instr.rflags_modified()));
        }
        if instr.op_kinds().any(|op_kind| op_kind == OpKind::Memory) {
            let size = instr.memory_size().size();
            if size != 0 {
                println!("    Memory size: {}", size);
            }
        }
        for i in 0..instr.op_count() {
            println!("    Op{}Access: {:?}", i, info.op_access(i));
        }
        for i in 0..op_code.op_count() {
            println!("    Op{}: {:?}", i, instr.op_kind(i));
        }
        for reg_info in info.used_registers() {
            println!("    Used reg: {:?}", reg_info);
        }
        for mem_info in info.used_memory() {
            println!("    Used mem: {:?}", mem_info);
        }
    }
}

fn flags(rf: u32) -> String {
    fn append(sb: &mut String, s: &str) {
        if !sb.is_empty() {
            sb.push_str(", ");
        }
        sb.push_str(s);
    }

    let mut sb = String::new();
    if (rf & RflagsBits::OF) != 0 {
        append(&mut sb, "OF");
    }
    if (rf & RflagsBits::SF) != 0 {
        append(&mut sb, "SF");
    }
    if (rf & RflagsBits::ZF) != 0 {
        append(&mut sb, "ZF");
    }
    if (rf & RflagsBits::AF) != 0 {
        append(&mut sb, "AF");
    }
    if (rf & RflagsBits::CF) != 0 {
        append(&mut sb, "CF");
    }
    if (rf & RflagsBits::PF) != 0 {
        append(&mut sb, "PF");
    }
    if (rf & RflagsBits::DF) != 0 {
        append(&mut sb, "DF");
    }
    if (rf & RflagsBits::IF) != 0 {
        append(&mut sb, "IF");
    }
    if (rf & RflagsBits::AC) != 0 {
        append(&mut sb, "AC");
    }
    if (rf & RflagsBits::UIF) != 0 {
        append(&mut sb, "UIF");
    }
    if sb.is_empty() {
        sb.push_str("<empty>");
    }
    sb
}

const EXAMPLE_CODE_BITNESS: u32 = 32;
const EXAMPLE_CODE_RIP: u64 = 0x0000_0000_AAAA_1337;
static EXAMPLE_CODE: &[u8] = &[
    0x9A, 0x37, 0x13, 0x00, 0x00, 0x13, 0x00
];