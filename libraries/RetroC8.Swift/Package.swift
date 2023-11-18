// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "RetroC8",
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "RetroC8",
            type: .dynamic,
            targets: ["RetroC8"]),
    ],
    dependencies: [
        .package(url: "https://github.com/retro-cade/RetroSwift.git", branch: "main")
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "RetroC8",
            dependencies: ["RetroSwift"]),
        .testTarget(
            name: "RetroC8Tests",
            dependencies: ["RetroC8"]),
    ]
)
