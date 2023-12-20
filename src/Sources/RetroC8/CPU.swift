//
//  File.swift
//  
//
//  Created by Glenn Hevey on 19/11/2023.
//

import Foundation

struct CPU {
    let registerSize: Int = 16
    let stackCount: Int = 12
    let programStartAddress: UInt16 = 0x200
    
    var i: UInt16
    var pc: UInt16
    var sp: UInt
    var registers: [UInt8]
    var stack: [UInt16]
    var bus: MemoryBus
    
    init() {
        i = 0
        sp = 0
        pc = programStartAddress
        registers = [UInt8](repeating: 0, count: registerSize)
        stack = [UInt16](repeating: 0, count: stackCount)
        bus = MemoryBus()
    }
    
    mutating func load(_ file: [UInt8]) {
        if file.count < 4096 - 512 {
            bus.memory.replaceSubrange(512...(512 + file.count), with: file)
        }
    }
    
    mutating func tick() {
        let opCode = fetch()
        execute(opCode: opCode)
    }
    
    mutating func tickTimers() {
        if bus.delayTimer > 0 {
            bus.delayTimer -= 1
        }
        
        if bus.soundTimer > 0 {
            if bus.soundTimer == 1 {
                //TODO: BEEP
            }
            bus.soundTimer -= 1
        }
    }
    
    mutating func fetch() -> UInt16 {
        let higherByte = bus.memory[Int(self.pc)]
        let lowerByte = bus.memory[Int(self.pc + 1)]
        
        let opCode: UInt16 = UInt16(higherByte) << 8 | UInt16(lowerByte)
        
        pc += 2
        
        return opCode
    }
    
    mutating func execute(opCode: UInt16) {
        let op1 = (opCode & 0xF000) >> 12
        let op2 = (opCode & 0x0F00) >> 8
        let op3 = (opCode & 0x00F0) >> 4
        let op4 = (opCode & 0x000F)
        
        switch (op1, op2, op3, op4) {
        case (0x0, 0x0, 0x0, 0x0):
            return
        case (0x0, 0x0, 0xE, 0x0):
            bus.gpu.memory = [Bool](repeating: false, count: bus.gpu.memory.count)
        case (0x0, 0x0, 0xE, 0xE):
            if sp >= 2 {
                sp -= 2
            }
            pc = stack[Int(sp)]
        case (0x1, _, _, _):
            pc = opCode & 0x0FFF
        case (0x2, _, _, _):
            stack[Int(sp)] = pc
            sp += 2
            pc = opCode & 0x0FFF
        case (0x3, _, _, _):
            if registers[Int(op2)] == opCode & 0x0FF {
                pc += 2
            }
        case (0x4, _, _, _):
            if registers[Int(op2)] != opCode & 0x0FF {
                pc += 2
            }
        case (0x5, _, _, _):
            if registers[Int(op2)] == registers[Int(op3)] {
                pc += 2
            }
        case (0x6, _, _, _):
            registers[Int(op2)] = UInt8(opCode & 0x00FF)
        case (0x7, _, _, _):
            let value = registers[Int(op2)] &+ UInt8(opCode & 0x00FF)
            registers[Int(op2)] = value
        case (0x8, _, _, 0x0):
            registers[Int(op2)] = registers[Int(op3)]
        case (0x8, _, _, 0x1):
            registers[Int(op2)] = registers[Int(op2)] | registers[Int(op3)]
            registers[0xF] = 0x0;
        case (0x8, _, _, 0x2):
            registers[Int(op2)] &= registers[Int(op3)]
            registers[0xF] = 0x0
        case (0x8, _, _, 0x3):
            registers[Int(op2)] ^= self.registers[Int(op3)]
            registers[0xF] = 0x0
        case (0x8, _, _, 0x4):
            let (value, carry) = registers[Int(op2)].addingReportingOverflow(registers[Int(op3)])
            registers[Int(op2)] = value
            registers[0xF] = carry == false ? 0x0 : 0xF
        case (0x8, _, _, 0x5):
            let (value, carry) = registers[Int(op2)].subtractingReportingOverflow(registers[Int(op3)])
            registers[Int(op2)] = value
            registers[0xF] = carry == true ? 0x0 : 0xF
        case (0x8, _, _, 0x6):
            let bit = registers[Int(op3)] & 0x1
            registers[Int(op2)] = registers[Int(op3)] >> 1
            registers[0xF] = bit
        case (0x8, _, _, 0x7):
            let (value, carry) = registers[Int(op3)].subtractingReportingOverflow(registers[Int(op2)])
            registers[Int(op2)] = value
            registers[0xF] = carry == true ? 0x0 : 0xF
        case (0x8, _, _, 0xE):
            let bit = (registers[Int(op3)] & 0x8) >> 3
            registers[Int(op2)] = registers[Int(op3)] << 1
            registers[0xF] = bit
        case (0x9, _, _, 0x0):
            if registers[Int(op2)] != registers[Int(op3)] {
                pc += 2
            }
        case (0xA, _, _, _):
            i = opCode & 0x0FFF
        case (0xB, _, _, _):
            pc = (opCode & 0x0FFF) + UInt16(registers[0x0])
        case (0xC, _, _, _):
            let random = UInt16.random(in: 0...255)
            registers[Int(op2)] = UInt8(random & (opCode & 0x00FF))
        case (0xD, _, _, _):
            let x = UInt16(registers[Int(op2)] % 64)
            let y = UInt16(registers[Int(op3)] % 32)
            let height = UInt16(op4)
            
            registers[0xF] = 0

            for y_line in 0..<height {
                let pixel = bus.memory[Int(i + y_line)]
                
                for x_line in 0..<UInt16(8) {
                    if x + x_line < 64 && y + y_line < 32 {
                        if pixel & (0x80 >> x_line) != 0 {
                            let location = x + x_line + ((y + y_line) * 64)  //UInt8(UInt(x + x_line) + (UInt((y + y_line)) * 64))
                            if bus.gpu.memory[Int(location)] {
                                registers[0xF] = 1
                            }
                            
                            let pixelValue = !bus.gpu.memory[Int(location)]
                            
                            bus.gpu.memory[Int(location)] = pixelValue
                        }
                    }
                }
                bus.gpu.draw = true
            }
        case (0xE, _, 0x9, 0xE):
            if bus.input.keys[Int(registers[Int(op2)])] {
                pc += 2
            }
        case (0xE, _, 0xA, 0x1):
            if !bus.input.keys[Int(registers[Int(op2)])] {
                pc += 2
            }
        case (0xF, _, 0x0, 0x7):
            registers[Int(op2)] = bus.delayTimer
        case (0xF, _, 0x0, 0xA):
            var keyPressed = false
            
            for key in bus.input.keys {
                if key {
                    keyPressed = true
                }
                
                if !keyPressed {
                    pc -= 2
                }
            }
        case (0xF, _, 0x1, 0x5):
            bus.delayTimer = registers[Int(op2)]
        case (0xF, _, 0x1, 0x8):
            bus.soundTimer = registers[Int(op2)]
        case (0xF, _, 0x1, 0xE):
            i += UInt16(registers[Int(op2)])
        case (0xF, _, 0x2, 0x9):
            i = UInt16(registers[Int(op2)]) * 0x5
        case (0xF, _, 0x3, 0x3):
            bus.memory[Int(i)] = registers[Int(op2)] / 100
            bus.memory[Int(i + 1)] = (registers[Int(op2)] / 10) % 10
            bus.memory[Int(i + 2)] = (registers[Int(op2)] % 100) % 10
        case (0xF, _, 0x5, 0x5):
            var i_temp: UInt16 = 0
            for j in 0...op2 {
                bus.memory[Int(i + j)] = registers[Int(j)]
                i_temp = j
            }
            self.i = i_temp + UInt16(op2) + 1
        case (0xF, _, 0x6, 0x5):
            var i_temp: UInt16 = 0
            for j in 0...op2 {
                registers[Int(j)] = bus.memory[Int(i + j)]
                i_temp = j
            }
            self.i = i_temp + UInt16(op2) + 1;
        default:
            fatalError("Unimplemented OpCode: \(opCode)")
        }
    }
    

}


