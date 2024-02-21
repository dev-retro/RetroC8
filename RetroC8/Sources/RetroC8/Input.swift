//
//  File.swift
//  
//
//  Created by Glenn Hevey on 20/11/2023.
//

import Foundation

struct Input {
    let keyCount: UInt = 16
    
    var keys: [Bool]
    
    init() {
        keys = [Bool](repeating: false, count: Int(keyCount))
    }
    
    mutating func update(key: UInt, value: Bool) {
        keys[Int(key)] = value
    }
}
