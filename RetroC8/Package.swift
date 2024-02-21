// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "RetroC8",
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "RetroC8",
            targets: ["RetroC8"]),
    ],
    dependencies: [
        .package(url: "https://github.com/dev-retro/RetroSwift.git", from: "1.0.0"),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "RetroC8"),
        .testTarget(
            name: "RetroC8Tests",
            dependencies: ["RetroC8"]),
    ]
)
