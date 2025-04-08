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
    
    #[test]
    fn call_subroutine(){
        let mut chip = Chip8::new();
        chip.pc = 0x200;
        let opcode = 0x2ABC;
        
        chip.call_subroutine(opcode);
        assert_eq!(chip.pc, 0xABC);
        assert_eq!(chip.stack.last(), Some(&0x200));
        
        println!("Test passed: pc = {:#X}", chip.pc);
    }
    
    #[test]
    fn return_from_subroutine(){
        let mut chip = Chip8::new();
        chip.pc = 0x200;
        
        chip.call_subroutine(0x2ABC);
        assert_eq!(chip.pc, 0xABC);
        
        chip.return_from_subroutine();
        assert_eq!(chip.pc, 0x200);
        
        assert_eq!(chip.stack.len(), 0);
        
        println!("Test passed: pc = {:#X}", chip.pc);
    }
    
    #[test]
    fn skip_next_register(){
        let mut chip = Chip8::new();
        chip.v[0] = 0xA0;
        
        let opcode = 0x30A0;
        chip.pc = 0x200;
        
        chip.decode_opcode(opcode);
        
        assert_eq!(chip.pc, 0x204);
        println!("Test passed: pc = {:#X}", chip.pc);
    }
    
    #[test]
    fn skip_next_not_same_value(){
        let mut chip = Chip8::new();
        chip.v[0] = 0xA0;
        
        let opcode = 0x40B0;
        chip.pc = 0x200;
        chip.decode_opcode(opcode);
        
        assert_eq!(chip.pc, 0x204);
        println!("Test passed: pc = {:#X}", chip.pc);
    }
    
    #[test]
    fn set_register_random_source(){
        let mut chip = Chip8::new();
        let opcode = 0xC0FF;
        chip.pc = 0x200;
        
        chip.decode_opcode(opcode);
        let v0_value = chip.v[0];
        assert!(v0_value <= 0xFF);
        
        println!("Test passed: v0_value = {:#X}", v0_value);
    }
    
    #[test]
    fn skip_next_if_not_equal(){
        let mut chip = Chip8::new();
        chip.pc = 0x200;
        let opcode = 0x9AB0;
        
        chip.v[10] = 0x01;
        chip.v[11] = 0x02;
        
        chip.decode_opcode(opcode);
        assert_eq!(chip.pc, 0x204);
        
        chip.v[11] = 0x01;
        chip.decode_opcode(opcode);
        assert_eq!(chip.pc, 0x206);
        
        println!("Test passed: pc = {:#X}", chip.pc);
    }
    
    #[test]
    fn draw_sprite_no_collision(){
        let mut chip = Chip8::new();
        
        chip.v[0x1] = 5;
        chip.v[0x2] = 5;
        chip.i = 0x200;

        // Store sprite data (0xFF) in memory starting at address 0x200
        chip.memory[0x200] = 0xFF;
        chip.memory[0x201] = 0x10;
        
        chip.display = [[false; 64]; 32 ];
        
        let opcode = 0xD015;
        
        chip.draw_sprite(opcode);

        assert_eq!(chip.display[1][0], true);
        assert_eq!(chip.display[1][1], true);
        assert_eq!(chip.display[1][2], true);
        assert_eq!(chip.display[1][3], true);
        assert_eq!(chip.display[1][4], true);
        assert_eq!(chip.display[1][5], true);
        assert_eq!(chip.display[1][6], true);
        assert_eq!(chip.display[1][7], true);
        assert_eq!(chip.vf, 0);
    }
    
    #[test]
    fn test_draw_sprite_with_collision() {
        let mut chip = Chip8::new();
        
        chip.v[0x0] = 0;
        chip.v[0x1] = 1;
        chip.i = 0x200;
        
        chip.memory[0x200] = 0xFF;
        chip.memory[0x201] = 0x00;
        chip.display[1][0] = true;
        
        let opcode = 0xD015;
        chip.draw_sprite(opcode);
        
        assert_eq!(chip.display[1][0], false);
        assert_eq!(chip.vf, 1);
    }
    
}
