pub struct Memory {
    pub bytes: [u8; 0x800],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            bytes: [0u8; 0x800],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_correctly() {
        Memory::new();
    }
}