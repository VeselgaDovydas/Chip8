use crate::Chip8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_to_address() {
        let mut chip = Chip8::new();
        // Simulate loading the opcode 0x1ABC
        let opcode = 0x1ABC;

        // Execute the jump to address
        chip.jump_to_address(opcode);

        // Assert the pc has been set to 0xABC
        assert_eq!(chip.pc, 0xABC);
        println!("Test passed: pc = {:#X}", chip.pc);
    }
}
