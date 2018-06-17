pub struct ProgramRom(pub usize, pub Vec<u8>);

impl ::std::ops::Index<usize> for ProgramRom {
    type Output = u8;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.1[idx]
    }
}

impl Default for ProgramRom {
    fn default() -> Self {
        ProgramRom (0, Vec::default())
    }
}