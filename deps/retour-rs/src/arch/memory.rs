use crate::{alloc, error::Result, pic};

/// Allocates PIC code at the specified address.
pub fn allocate_pic(emitter: &pic::CodeEmitter) -> Result<alloc::ExecutableMemory> {
  // Allocate memory close to the origin
  let layout = std::alloc::Layout::from_size_align(emitter.len(), 16)?;
  let memory = alloc::ExecutableMemory::alloc(layout)?;
  let code = emitter.emit(memory.ptr as *const _);
  unsafe {
    memory
      .ptr
      .copy_from_nonoverlapping(code.as_ptr(), code.len());
    region::protect(
      memory.ptr,
      code.len(),
      region::Protection::READ_WRITE_EXECUTE,
    )?;
  }
  Ok(memory)
}
