# RustVulkan
This is where I learn and experiment with Vulkan in Rust

[Vulkan Tutorial](https://vulkan-tutorial.com)

[Vulkan Tutorial using Ash](https://github.com/adrien-ben/vulkan-tutorial-rs)

## Basics
- First an Instance is created, this is the interface with the Vulkan API
- From the Instance, Physical Devices can be enumerated
- Physical Devices have different queue families for different porpuses
- From a Physical Device, Devices and Command Queues are created

- Memory and Command Buffers are created from Allocators and Builders
- Memory Buffers are used to store data, and have different types that can or not be accessed by the CPU
- Command Buffers are built with as a set of instructions that are then sent to be executed on a Device.
- Signals are returned when a Command Buffer is sent for execution so you can wait and know when it's finished





