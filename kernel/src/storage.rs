pub const BLOCK_SIZE: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError { OutOfRange, ReadOnly }

pub trait BlockDevice {
    fn block_count(&self) -> u64;
    fn read_block(&self, block: u64, buffer: &mut [u8; BLOCK_SIZE]) -> Result<(), StorageError>;
    fn write_block(&mut self, block: u64, buffer: &[u8; BLOCK_SIZE]) -> Result<(), StorageError>;
}

pub struct RamDisk<const BLOCKS: usize> { blocks: [[u8; BLOCK_SIZE]; BLOCKS] }

impl<const BLOCKS: usize> RamDisk<BLOCKS> {
    pub const fn new() -> Self { Self { blocks: [[0; BLOCK_SIZE]; BLOCKS] } }
}

impl<const BLOCKS: usize> BlockDevice for RamDisk<BLOCKS> {
    fn block_count(&self) -> u64 { BLOCKS as u64 }

    fn read_block(&self, block: u64, buffer: &mut [u8; BLOCK_SIZE]) -> Result<(), StorageError> {
        let index = usize::try_from(block).map_err(|_| StorageError::OutOfRange)?;
        let source = self.blocks.get(index).ok_or(StorageError::OutOfRange)?;
        buffer.copy_from_slice(source);
        Ok(())
    }

    fn write_block(&mut self, block: u64, buffer: &[u8; BLOCK_SIZE]) -> Result<(), StorageError> {
        let index = usize::try_from(block).map_err(|_| StorageError::OutOfRange)?;
        let destination = self.blocks.get_mut(index).ok_or(StorageError::OutOfRange)?;
        destination.copy_from_slice(buffer);
        Ok(())
    }
}

pub fn init() {
    let disk = RamDisk::<8>::new();
    crate::serial_println!("storage: block-device ABI ready block_size={} blocks={}", BLOCK_SIZE, disk.block_count());
}
