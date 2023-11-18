// The Swift Programming Language
// https://docs.swift.org/swift-book

import RetroSwift

struct RetroC8: CadePlatform {
    func test() -> String {
        return "RetroC8 Running"
    }    
}

@_cdecl("createPlugin")
public func createPlugin() -> UnsafeMutableRawPointer {
    return Unmanaged.passRetained(RetroC8Builder()).toOpaque()
}

final class RetroC8Builder: Builder {
    override func build() -> CadePlatform {
        RetroC8()
    }
}
