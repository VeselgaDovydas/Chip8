

// Fetch
// Decode
// Execute

/*
- RAM - 4KB - 4096 bytes - 0x200 (512 in decimal)
- Registers - 16/8Bit (V0 to VF) 0 1 2 3 4 5 6 7 8 9 A B C D E F
- Program Counter (PC) - Counter which keeps track of a next instruction
It starts at 0x200 and increases by 2 after each instruction execution.
- Opcode - Every instruction is 2 bytes (16 bits).
For example, 0x00E0 is "Clear screen" instruction
- Delay Timer - 1/60s (60Hz) stops when reaches 0
- Display - 64x32, pixel is 0 or 1.
*/

```
    // println!("Current PC: {:#X}", chip.pc);
    // 
    // chip.stack.push(chip.pc);
    // chip.pc = 0x300;
    // 
    // println!("Jumped to subroutine at: {:#X}", chip.pc);
    // 
    // chip.return_from_subroutine();
    //
    // // Add "Clear screen" opcode
    // chip8.memory[0x200] = 0x00;
    // chip8.memory[0x201] = 0xE0;
    // 
    // let opcode = chip8.fetch_opcode();
    // chip8.decode_opcode(opcode);
    // 
    // println!("{:?}", opcode.to_be_bytes());
    // 
    // // Add "Return from subroutine" opcode
    // chip8.memory[0x202] = 0x00;
    // chip8.memory[0x203] = 0xEE;
    // 
    // let opcode = chip8.fetch_opcode();
    // chip8.decode_opcode(opcode);
    // 
    // println!("{:?}", opcode.to_be_bytes());

```