#![no_std]



use allocator::{AllocError, BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const SIZE: usize> {
    bytestart: usize,
    byteend: usize,
    pageend: usize,
    pagestart:usize
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self { bytestart: 0, byteend: 0, pageend: 0, pagestart: 0 }
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.bytestart  = start;
        self.byteend = start;
        self.pageend = start + size;
        self.pagestart = start + size;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        todo!()
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        let target = self.byteend + layout.size();
        if target > self.pagestart{return Err(AllocError::NoMemory)}
        let ret = core::ptr::NonNull::new(self.byteend as *mut u8);
        let Some(ret) = ret else {
            return Err(AllocError::InvalidParam);
        };
        self.byteend = target;
        return Ok(ret);

    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        // No dealloc
    }

    fn total_bytes(&self) -> usize {
        return self.pageend - self.bytestart
    }

    fn used_bytes(&self) -> usize {
        return (self.byteend - self.bytestart) + (self.pageend - self.pagestart)
    }

    fn available_bytes(&self) -> usize {
        return self.pagestart - self.byteend
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        let target = self.pagestart - num_pages * Self::PAGE_SIZE;
        if target < self.byteend{return Err(AllocError::NoMemory)}
        self.byteend = target;
        return Ok(target);
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        
    }

    fn total_pages(&self) -> usize {
        return (self.pageend - self.bytestart) / Self::PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        return (self.pageend - self.pagestart) / Self::PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        return (self.pagestart - self.byteend) / Self::PAGE_SIZE
    }
}