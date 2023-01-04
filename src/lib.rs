use neon::prelude::*;
mod direct_io;
pub use direct_io::get_aligned_buffer;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get_aligned_buffer", get_aligned_buffer)?;
    Ok(())
}
