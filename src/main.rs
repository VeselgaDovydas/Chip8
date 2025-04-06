use rand::prelude::*;
mod tests;

struct Chip8{
    pc: u16, // 16-bit integer
    v: [u8; 16], // 8-bit 16 times
    memory: [u8; 4096], // 8-bit 4096 times
    timer: [u8; 60],
    display: [[bool; 64]; 32 ], //Display - 64x32, pixel is 0 or 1.
    stack: Vec<u16>,
    i: u16
}

impl Chip8{
    fn new() -> Chip8{
        Chip8{
            pc: 0x200,
            v: [0; 16],
            memory: [0; 4096],
            timer: [0; 60],
            display: [[false; 64]; 32 ],
            stack: Vec::<u16>::new(),
            i: 0
        }
    }
    
    fn step_pc_counter(&mut self) {
        self.pc = self.pc + 2;
    }

    fn retrieve_opcode_register_data(&mut self, opcode: u16) -> (u8, u8){
        (((opcode & 0x0F00) >> 8) as u8, (opcode & 0x00FF) as u8)
    }
    
    fn fetch_opcode(&mut self) -> u16 {
        let high_byte = self.memory[self.pc as usize] as u16;
        let low_byte = self.memory[(self.pc + 1) as usize] as u16;
        self.step_pc_counter();
        (high_byte << 8) | low_byte // shifts to left and combines with Bitwise OR
    }
    
    fn decode_opcode(&mut self, opcode: u16) {
        match opcode {
            0x00E0 => self.clear_screen(),
            0x00EE => self.return_from_subroutine(),
            0x1000..=0x1FFF => self.jump_to_address(opcode),
            0x2000..=0x2FFF => self.call_subroutine(opcode),
            0x3000..=0x3FFF => self.skip_next_same_value(opcode),
            0x4000..=0x4FFF => self.skip_next_not_same_value(opcode),
            0x6000..=0x6FFF => self.set_register(opcode),
            0x7000..=0x7FFF => self.add_to_register(opcode),
            0x9000..=0x9FFF => self.skip_next_if_not_equal(opcode),
            0xA000..=0xAFFF => self.index_register(opcode),
            0xC000..=0xCFFF => self.set_register_random_source(opcode),
            0xD000..=0xDFFF => self.draw_sprite(opcode),
            _ => println!("opcode not found {:#X}", opcode)
        }
    }
    
    ///0x00E0
    fn clear_screen(&mut self){
        println!("Executing 'clear screen");
        self.display = [[false; 64]; 32 ];
    }
    
    ///0x00EE
    fn return_from_subroutine(&mut self) {
        println!("Executing 'return from subroutine");
        if let Some(address) = self.stack.pop() {
            self.pc = address;
            println!("Returned to address {:#X}", self.pc);
        } else {
            println!("No address for subroutine");
        }
    }

    ///0x1000..=0x1FFF
    fn jump_to_address(&mut self, opcode: u16) {
        println!("Executing 'jump_to_address'");
        self.pc = opcode & 0x0FFF;
    }
    
    ///0x2000..=0x2FFF
    fn call_subroutine(&mut self, opcode: u16) {
        println!("Executing 'call_subroutine' with opcode {:#X}", opcode);
        let address = opcode & 0x0FFF;
        self.stack.push(self.pc);
        self.pc = address;
    }
    
    ///0x3000..=0x3FFF
    fn skip_next_same_value(&mut self, opcode: u16) {
        println!("Executing 'skip_next_register'");
        let (register_index, register_value) = self.retrieve_opcode_register_data(opcode);
        
        if self.v[register_index as usize] == register_value {
            self.pc += 4;
        }
    }
    
    ///0x4000..=0x4FFF
    fn skip_next_not_same_value(&mut self, opcode: u16) {
        println!("Executing 'skip_next_register_not_same_value'");
        let (register_index, register_value) = self.retrieve_opcode_register_data(opcode);
        if self.v[register_index as usize] != register_value {
            self.pc += 4;
        }
    }
    
    ///0x6000..=0x6FFF
    fn set_register(&mut self, opcode: u16) {
        println!("Executing 'set register'");
        
        let (register_index, register_value) = self.retrieve_opcode_register_data(opcode);
        self.v[register_index as usize] = register_value;

        println!("Opcode {:#X} Register {:#X} = {:#X}", opcode, register_index, register_value);
        /*
            0x6ABF 
            0110 1010 1011 1111
            |
            0000 1111 0000 0000
            =
            0000 1010 0000 0000
            >> 8
            0000 0000 0000 1010
        */
    }

    ///0x7000..=0x7FFF
    fn add_to_register(&mut self, opcode: u16) {
        println!("Executing 'add to register");
        
        let (register_index, register_value) = self.retrieve_opcode_register_data(opcode);
        self.v[register_index as usize] = self.v[register_index as usize].wrapping_add(register_value);
        
        println!("Opcode {:#X} Register {:#X} before {:#X}, after {:#X}", 
                 opcode, register_index, register_value, self.v[register_index as usize]);
    }

    ///0x9000..=0x9FFF
    fn skip_next_if_not_equal(&mut self, opcode: u16) {
        let register_index_x = (opcode & 0x0F00) >> 8;
        let register_index_y = (opcode & 0x00F0) >> 4;

        if self.v[register_index_x as usize] != self.v[register_index_y as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    ///0xA000..=0xAFFF
    fn index_register(&mut self, opcode: u16) {
        println!("Executing 'index_register");
        let index_value = opcode & 0x0FFF;
        self.i = index_value;
        
        println!("Opcode {:#X} Index set as {:#X}", opcode, index_value);
    }
    
    ///0xC000..=0xCFFF
    fn set_register_random_source(&mut self, opcode: u16) {
        println!("Executing 'set_random_source'");
        let mut rng = rand::rng();
        let rng_value = rng.random_range(0..=255) as u8;
        let (register_index, register_value) = self.retrieve_opcode_register_data(opcode);
        self.v[register_index as usize] = rng_value & register_value;
    }
    
    ///0xD000..=0xDFFF
    fn draw_sprite(&mut self, opcode: u16) {
        //Dxyn - DRW Vx, Vy, nibble
        println!("Executing 'draw_sprite'");
        let vy = ((opcode & 0x00F0) >> 4) as u8;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let length = (opcode & 0x000F) as u8;
        
        // todo: draw logic
    }
}

fn main() {
    println!("Running the Chip8 emulator...");
}