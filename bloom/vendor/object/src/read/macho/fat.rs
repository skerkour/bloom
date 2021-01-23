use crate::read::{Architecture, Error, ReadError, Result};
use crate::{macho, BigEndian, Bytes, Pod};

pub use macho::{FatArch32, FatArch64, FatHeader};

impl FatHeader {
    /// Attempt to parse a 32-bit fat header.
    pub fn parse_arch32<'data>(file: &'data [u8]) -> Result<&'data [FatArch32]> {
        let mut file = Bytes(file);
        let header = file
            .read::<FatHeader>()
            .read_error("Invalid fat header size or alignment")?;
        if header.magic.get(BigEndian) != macho::FAT_MAGIC {
            return Err(Error("Invalid 32-bit fat magic"));
        }
        file.read_slice::<FatArch32>(header.nfat_arch.get(BigEndian) as usize)
            .read_error("Invalid nfat_arch")
    }

    /// Attempt to parse a 64-bit fat header.
    pub fn parse_arch64<'data>(file: &'data [u8]) -> Result<&'data [FatArch64]> {
        let mut file = Bytes(file);
        let header = file
            .read::<FatHeader>()
            .read_error("Invalid fat header size or alignment")?;
        if header.magic.get(BigEndian) != macho::FAT_MAGIC_64 {
            return Err(Error("Invalid 64-bit fat magic"));
        }
        file.read_slice::<FatArch64>(header.nfat_arch.get(BigEndian) as usize)
            .read_error("Invalid nfat_arch")
    }
}

/// A trait for generic access to `FatArch32` and `FatArch64`.
#[allow(missing_docs)]
pub trait FatArch: Pod {
    type Word: Into<u64>;

    fn cputype(&self) -> u32;
    fn cpusubtype(&self) -> u32;
    fn offset(&self) -> Self::Word;
    fn size(&self) -> Self::Word;
    fn align(&self) -> u32;

    fn architecture(&self) -> Architecture {
        match self.cputype() {
            macho::CPU_TYPE_ARM => Architecture::Arm,
            macho::CPU_TYPE_ARM64 => Architecture::Aarch64,
            macho::CPU_TYPE_X86 => Architecture::I386,
            macho::CPU_TYPE_X86_64 => Architecture::X86_64,
            macho::CPU_TYPE_MIPS => Architecture::Mips,
            _ => Architecture::Unknown,
        }
    }

    fn data<'data>(&self, file: &'data [u8]) -> Result<&'data [u8]> {
        let offset = self.offset().into();
        let size = self.size().into();
        let data = Bytes(file)
            .read_bytes_at(offset as usize, size as usize)
            .read_error("Invalid fat arch offset or size")?;
        Ok(data.0)
    }
}

impl FatArch for FatArch32 {
    type Word = u32;

    fn cputype(&self) -> u32 {
        self.cputype.get(BigEndian)
    }

    fn cpusubtype(&self) -> u32 {
        self.cpusubtype.get(BigEndian)
    }

    fn offset(&self) -> Self::Word {
        self.offset.get(BigEndian)
    }

    fn size(&self) -> Self::Word {
        self.size.get(BigEndian)
    }

    fn align(&self) -> u32 {
        self.align.get(BigEndian)
    }
}

impl FatArch for FatArch64 {
    type Word = u64;

    fn cputype(&self) -> u32 {
        self.cputype.get(BigEndian)
    }

    fn cpusubtype(&self) -> u32 {
        self.cpusubtype.get(BigEndian)
    }

    fn offset(&self) -> Self::Word {
        self.offset.get(BigEndian)
    }

    fn size(&self) -> Self::Word {
        self.size.get(BigEndian)
    }

    fn align(&self) -> u32 {
        self.align.get(BigEndian)
    }
}
