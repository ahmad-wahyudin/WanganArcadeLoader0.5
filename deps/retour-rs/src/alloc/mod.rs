use crate::{Error, Result};

/// A handle for allocated proximity memory.
pub struct ExecutableMemory {
  pub(crate) ptr: *mut u8,
  layout: std::alloc::Layout,
}

impl ExecutableMemory {
  pub fn alloc(layout: std::alloc::Layout) -> Result<Self> {
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
      Err(Error::OutOfMemory)
    } else {
      Ok(Self { layout, ptr })
    }
  }
}

impl Drop for ExecutableMemory {
  fn drop(&mut self) {
    // Release the associated memory map (if unique)
    unsafe {
      std::alloc::dealloc(self.ptr, self.layout);
    }
  }
}
