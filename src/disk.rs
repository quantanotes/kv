use std::fs::{File, OpenOptions};
use std::io;
use std::path::PathBuf;

use memmap2::MmapMut;

use crate::page::{Page, PAGE_SIZE};

#[derive(thiserror::Error, Debug)]
pub enum DiskError {
    #[error("io error: {0}")]
    IOError(#[from] io::Error),

    #[error("bounds error: cannot read page at index: {0}, max: {1}")]
    BoundsError(usize, usize),
}

pub struct Disk {
    file: File,
    mmap: MmapMut,
    size: usize,
}

impl Disk {
    pub fn new(path: &str) -> Result<Disk, DiskError> {
        let path = PathBuf::from(path);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;

        let mmap = (unsafe { MmapMut::map_mut(&file) })?;

        let size = mmap.len();

        Ok(Disk { file, mmap, size })
    }

    pub fn read(&self, index: usize) -> Result<Page, DiskError> {
        let (start, end) = Disk::page_range(index);

        if end <= self.size {
            Ok(self.mmap[start..end].try_into().unwrap())
        } else {
            Err(DiskError::BoundsError(index, self.size / 4096 - 1))
        }
    }

    pub fn write(&mut self, index: usize, page: &Page) -> Result<(), DiskError> {
        let (start, end) = Disk::page_range(index);

        if end <= self.size {
            self.mmap[start..end].copy_from_slice(page);
            Ok(())
        } else {
            Err(DiskError::BoundsError(index, self.size / 4096 - 1))
        }
    }

    pub fn allocate(&mut self, size: usize) -> Result<(), DiskError> {
        let new_size = self.size + size * PAGE_SIZE;

        self.size = new_size;

        self.file.set_len(new_size.try_into().unwrap())?;

        self.mmap = (unsafe { MmapMut::map_mut(&self.file) })?;

        Ok(())
    }

    pub fn size(&self) -> usize {
        self.mmap.len() / PAGE_SIZE
    }

    fn page_range(index: usize) -> (usize, usize) {
        (index * PAGE_SIZE, (index + 1) * PAGE_SIZE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk() {
        let temp_path = "./tmp.bin";

        let _ = std::fs::remove_file(temp_path);

        let mut disk = Disk::new(temp_path).expect("Failed to create Disk");

        disk.allocate(1).unwrap();

        let write_page = [1u8; PAGE_SIZE];

        disk.write(0, &write_page).unwrap();

        let read_page = disk.read(0).unwrap();

        assert_eq!(write_page, read_page);

        let _ = std::fs::remove_file(temp_path);
    }
}
