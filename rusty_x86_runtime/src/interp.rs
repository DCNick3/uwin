use crate::{execute_recompiled_code, uwin_find_thunk, ExtendedContext};
use core_mem::ctx::FlatMemoryCtx;
use iced_x86::{Decoder, DecoderOptions};
use rusty_x86::interp::backend::InterpBuilder;
use rusty_x86::types::ControlFlow;
use tracing::debug;

pub unsafe fn run_interp(context: &mut ExtendedContext, memory: FlatMemoryCtx, eip: u32) -> u32 {
    unsafe fn call_thunk(
        context: &mut ExtendedContext,
        memory: FlatMemoryCtx,
        thunk_id: u32,
    ) -> u32 {
        let thunk = uwin_find_thunk(thunk_id);

        match thunk {
            Some(thunk) => (thunk)(context, memory),
            None => {
                panic!("Called unknown thunk (id = {})", thunk_id)
            }
        }
    }

    fn leave_interpreter(context: &mut ExtendedContext, memory: FlatMemoryCtx, eip: u32) -> u32 {
        debug!("Trying to leave interpreter to EIP = {:#010x}", eip);
        execute_recompiled_code(context, memory, eip)
    }

    context.interpreted_blocks.insert(eip);

    let mut builder = {
        InterpBuilder::new(
            std::mem::transmute_copy(&context),
            memory.base_ptr(),
            std::mem::transmute(
                run_interp as unsafe fn(&mut ExtendedContext, FlatMemoryCtx, u32) -> u32,
            ),
            std::mem::transmute(
                call_thunk as unsafe fn(&mut ExtendedContext, FlatMemoryCtx, u32) -> u32,
            ),
        )
    };

    let data = std::slice::from_raw_parts(memory.base_ptr(), 1usize << 32);
    let mut decoder = Decoder::new(32, data, DecoderOptions::NONE);

    decoder.set_ip(eip as u64);
    decoder.set_position(eip as usize).unwrap();

    loop {
        let instr = decoder.decode();

        let flow = rusty_x86::codegen_instr(&mut builder, instr);

        match flow {
            ControlFlow::NextInstruction => {}
            ControlFlow::DirectJump(eip) => {
                return leave_interpreter(context, memory, eip);
            }
            ControlFlow::IndirectJump(eip) => {
                let eip = eip.as_u32().unwrap();
                return leave_interpreter(context, memory, eip);
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
            ControlFlow::Conditional(cond, eip) => {
                if cond {
                    return leave_interpreter(context, memory, eip);
                }
            }
        }
    }
    //
}
