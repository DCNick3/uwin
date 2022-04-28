use crate::types::CpuContext;
use crate::ControlFlow;
use iced_x86::{Decoder, DecoderOptions};

pub mod backend;

/// # Safety
///
/// Memory points to 2 GiB of reserved address space
///
/// Will segfault if an attempt to execute non-mapped code is done or the target code touches non-mapped memory
pub unsafe fn interpret_simple(context: &mut CpuContext, memory: *mut u8, eip: u32) -> u32 {
    fn call_thunk(_context: &mut CpuContext, _memory: *mut u8, thunk_id: u32) -> u32 {
        unimplemented!("Calling thunk #{}", thunk_id)
    }

    let mut builder = backend::InterpBuilder::new(context, memory, interpret_simple, call_thunk);

    let data = std::slice::from_raw_parts(memory, 1usize << 32);
    let mut decoder = Decoder::new(32, data, DecoderOptions::NONE);

    decoder.set_ip(eip as u64);
    decoder.set_position(eip as usize).unwrap();

    loop {
        let instr = decoder.decode();

        let flow = crate::codegen_instr(&mut builder, instr);

        match flow {
            ControlFlow::NextInstruction => {}
            ControlFlow::DirectJump(eip) => {
                decoder.set_ip(eip as u64);
                decoder.set_position(eip as usize).unwrap();
            }
            ControlFlow::IndirectJump(eip) => {
                let eip = eip.as_u32().unwrap();
                decoder.set_ip(eip as u64);
                decoder.set_position(eip as usize).unwrap();
            }
            ControlFlow::CallCheck(eip) => {
                let eip = eip.as_u32().unwrap();
                if eip != decoder.ip() as u32 {
                    return eip;
                }
            }
            ControlFlow::Return(eip) => {
                let eip = eip.as_u32().unwrap();
                return eip;
            }
            ControlFlow::Conditional(cond, target) => {
                if cond {
                    decoder.set_ip(target as u64);
                    decoder.set_position(target as usize).unwrap();
                }
            }
        }
    }
}
