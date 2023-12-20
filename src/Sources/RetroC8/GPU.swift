//
//  File.swift
//  
//
//  Created by Glenn Hevey on 20/11/2023.
//

import Foundation

struct GPU {
    let graphicsSize: UInt = 64 * 32
    
    var memory: [Bool]
    var draw: Bool
    
    init() {
        memory = [Bool](repeating: false, count: Int(graphicsSize))
        draw = true //TODO: Confirm whether this can be false once working.
    }
    
    mutating func update(draw: Bool) {
        self.draw = draw
    }
    
}
