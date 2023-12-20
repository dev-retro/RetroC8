// The Swift Programming Language
// https://docs.swift.org/swift-book

public struct RetroC8 {
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
    
    public mutating func tickTimers() {
        cpu.tickTimers()
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
