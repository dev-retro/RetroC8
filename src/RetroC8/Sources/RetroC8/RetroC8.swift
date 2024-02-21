// The Swift Programming Language
// https://docs.swift.org/swift-book

import RetroSwift
import Foundation

public struct RetroC8: RetroPlatform {
    public mutating func listInputs() -> [RetroSwift.RetroInput] {
        return [RetroInput]()
    }
    
    public mutating func update(inputs: [RetroSwift.RetroInput]) {
        
    }
    
    public mutating func setup() -> Bool {
        return false
    }
    
    public mutating func start() -> Bool {
        return false
    }
    
    public mutating func pause() -> Bool {
        return false
    }
    
    public mutating func stop() -> Bool {
        return false
    }
    
    public var name = "CHIP-8"
    public var description = "CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker made on his 1802 Microprocessor. "
    public var releaseDate = 1974
    public var noOfPlayers = 2
    public var platformName = "RetroC8"
    public var platformDescription = "Retro platform the for CHIP-8 interpreter"
    
    
    var cpu: CPU
    
    public init() {
        self.cpu = CPU()
    }
    
    public mutating func load(file: [UInt8]) {
        cpu.load(file)
    }
    
    public mutating func tick() {
        cpu.tick()
    }
    
    public mutating func tickTimers(playSound: (Bool) ->()) {
        cpu.tickTimers(playSound)
    }
    
    public func graphics() -> [Bool] {
        cpu.bus.gpu.memory
    }
    
    public func draw() -> Bool {
        cpu.bus.gpu.draw
    }
    
    mutating public func update(draw: Bool) {
        cpu.bus.gpu.update(draw: draw)
    }
    
    mutating public func update(key: UInt, value: Bool) {
        cpu.bus.input.update(key: key, value: value)
    }
}
