use crate::{execute_recompiled_code, uwin_find_thunk, ExtendedContext};
use core_mem::ctx::FlatMemoryCtx;
use iced_x86::{Decoder, DecoderOptions};
use rusty_x86::interp::backend::InterpBuilder;
use rusty_x86::types::ControlFlow;
use tracing::debug;

fn interp_call_bb<const TRY_LEAVE: bool>(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
    eip: u32,
) -> u32 {
    if TRY_LEAVE {
        debug!("Trying to leave interpreter to EIP = {:#010x}", eip);
        execute_recompiled_code(context, memory, eip)
    } else {
        run_interp::<false>(context, memory, eip)
    }
}

pub fn run_interp<const TRY_LEAVE: bool>(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
    eip: u32,
) -> u32 {
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

    context.interpreted_blocks.insert(eip);

    let mut builder = unsafe {
        InterpBuilder::new(
            std::mem::transmute_copy(&context),
            memory.base_ptr(),
            std::mem::transmute(
                interp_call_bb::<TRY_LEAVE>
                    as unsafe fn(&mut ExtendedContext, FlatMemoryCtx, u32) -> u32,
            ),
            std::mem::transmute(
                call_thunk as unsafe fn(&mut ExtendedContext, FlatMemoryCtx, u32) -> u32,
            ),
        )
    };

    let data = unsafe { std::slice::from_raw_parts(memory.base_ptr(), 1usize << 32) };
    let mut decoder = Decoder::new(32, data, DecoderOptions::NONE);

    decoder.set_ip(eip as u64);
    decoder.set_position(eip as usize).unwrap();

    loop {
        let instr = decoder.decode();

        // TODO: this should be done with hooks
        // eprintln!(
        //     "EXEC      {:#010x}: {:40}  esp={:#010x}",
        //     instr.next_ip32() - instr.len() as u32,
        //     format!("{}", instr),
        //     context.cpu.get_esp()
        // );

        let flow = rusty_x86::codegen_instr(&mut builder, instr);

        match flow {
            ControlFlow::NextInstruction => {}
            ControlFlow::DirectJump(eip) => {
                return interp_call_bb::<TRY_LEAVE>(context, memory, eip);
            }
            ControlFlow::IndirectJump(eip) => {
                let eip = eip.as_u32().unwrap();
                return interp_call_bb::<TRY_LEAVE>(context, memory, eip);
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
                    return interp_call_bb::<TRY_LEAVE>(context, memory, eip);
                }
            }
        }
    }
    //
}
