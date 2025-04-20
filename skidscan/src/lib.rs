#![allow(clippy::missing_safety_doc)]

pub use skidscan_macros::*;

#[cfg(feature = "obfuscate")]
pub use obfstr::obfstr;

mod signatures;
pub use signatures::*;

mod modulescan;
pub use modulescan::ModuleSigScanError;

pub trait SigscanPtr: Copy + Ord {
    unsafe fn next(self) -> Self;
    unsafe fn byte(self) -> u8;
    unsafe fn rewind(self, bytes: usize) -> Self;
}
impl SigscanPtr for *const u8 {
    #[inline(always)]
    unsafe fn next(self) -> Self {
        self.add(1)
    }
    #[inline(always)]
    unsafe fn byte(self) -> u8 {
        *self
    }
    #[inline(always)]
    unsafe fn rewind(self, bytes: usize) -> Self {
        self.sub(bytes)
    }
}
impl SigscanPtr for *mut u8 {
    #[inline(always)]
    unsafe fn next(self) -> Self {
        self.add(1)
    }
    #[inline(always)]
    unsafe fn byte(self) -> u8 {
        *self
    }
    #[inline(always)]
    unsafe fn rewind(self, bytes: usize) -> Self {
        self.sub(bytes)
    }
}

trait SigScan {
    /// Scans this slice of bytes for a given signature
    ///
    /// Returns the index of the first occurrence of the signature in the slice, or None if not found
    fn sigscan(&self, signature: &Signature) -> Option<usize>;
}
impl<B: AsRef<[u8]>> SigScan for B {
    #[inline(always)]
    fn sigscan(&self, signature: &Signature) -> Option<usize> {
        signature.scan(self.as_ref())
    }
}

#[cfg(test)]
mod test;
