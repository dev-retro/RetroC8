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
    
    mutating func tick() {
        var opCode = fetch()
        execute(opCode: opCode)
        
        //TODO: Add opcode execution
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
        var higherByte = bus.memory[Int(self.pc)]
        var lowerByte = bus.memory[Int(self.pc + 1)]
        
        var opCode: UInt16 = UInt16(higherByte) << 8 | UInt16(lowerByte)
        
        pc += 2
        
        return opCode
    }
    
    mutating func execute(opCode: UInt16) {
        
    }
}
