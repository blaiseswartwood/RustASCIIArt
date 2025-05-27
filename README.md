# Rust ASCII Animation Engine

This repository implements a high-performance, async-driven ASCII animation engine in Rust running code on the GPU using an OpenGL wrapper. It leverages modern Rust features, including the [`tokio`](https://tokio.rs/) async runtime, and is designed around a client-server architecture using shared memory for efficient communication between processes.

## Installation

To use msdf, install the following:

1. Clang Version 21.1.0 [https://github.com/llvm/llvm-project/releases/download/llvmorg-20.1.0/LLVM-20.1.0-win64.exe]() usually this works however if it doesn't try: [https://github.com/llvm/llvm-project/releases - download LLVM-20.1.0-rc3-win64.exe](https://github.com/llvm/llvm-project/releases/download/llvmorg-20.1.0-rc3/LLVM-20.1.0-rc3-win64.exe)
2. CMake 31.1.5: [https://cmake.org/download/](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&ved=2ahUKEwjgtMm4mLKMAxW4kokEHS0UOa8QFnoECBsQAQ&url=https%3A%2F%2Fgithub.com%2FKitware%2FCMake%2Freleases%2Fdownload%2Fv3.31.5%2Fcmake-3.31.5-windows-x86_64.msi&usg=AOvVaw0z0we2J4ABCJhtrKYXMmY4&opi=89978449)

Ensure the usage of MSCV on both rust toolchain and on clang toolchain

    rustup toolchain list

 ![image](https://github.com/user-attachments/assets/b27ad0c4-9c2f-4c59-9cab-d2749b50fc65)

# Rust ASCII Animation Engine

This repository implements a high-performance, async-driven ASCII animation engine in Rust. It leverages modern Rust features, including the [`tokio`](https://tokio.rs/) async runtime, and is designed around a client-server architecture using shared memory for efficient communication between processes.

## Features

- **GPU Based Rendering with OpenGL** Animations are ran on the GPU to ensure maximum performance
- **Async/await with Tokio:** Animations and entity updates are managed using Rust's async/await syntax, powered by the Tokio runtime. This allows for smooth, non-blocking animation updates and efficient use of system resources.
- **Client-Server Architecture:** The engine uses shared memory (`shared_memory` crate) to synchronize state between a server (animation producer) and one or more clients (renderers or consumers). The [`SharedEntityBuffer`](src/entity/shared_entity_buffer.rs) struct manages a lock-protected buffer of entities for concurrent access.
- **Composable Animations:** Animations are built by composing entities and animation traits (e.g., `Translate`, `FadingTrail`, `ColrRand`). Each animation is typically run as an async task, allowing for complex, concurrent visual effects.
- **Test-Driven Development:** The codebase includes extensive unit and integration tests, including visual tests for animation correctness.

## How It Works

### Architecture

- **Server:** Initializes shared memory, loads fonts, and spawns animation tasks. It writes animated entities into the shared buffer.
- **Client:** Reads from the shared buffer and renders the ASCII art to the screen using OpenGL (via Glium).
- **Synchronization:** The shared buffer uses atomic locks and flags to ensure safe concurrent access.
- **Virtualization:** We use an virtual timing system to minimize lag in our system
  
# Building Animations

To create a new animation:

1. **Define the ASCII Art**  
   Place your ASCII art in a string.

2. **Configure Inputs**  
   Set up an `Inputs` struct with parameters like position, spacing, and animation speed.

3. **Spawn Entities**  
   Use a function (e.g., `spawn_entities_from_ascii`) to create and animate entities for each character in your art.

4. **Compose Effects**  
   Chain animation traits (e.g., translation, color, fading) to build complex behaviors.

See `src/example_animations` for many examples, including:

- `ball.rs`
- `elephant.rs`
- `dvd.rs`

# Building and Running

## Prerequisites

- Rust (latest stable)
- [Tokio](https://tokio.rs/) (added as a dependency)
- [Glium](https://github.com/glium/glium) for OpenGL rendering
- Clang and CMake for font rasterization (see `README.md` for details)

## Running

The main entry point is `main.rs`, which sets up the shared memory and launches the animation system.

The server will start when running the system, creating the smem handle file. Any other processes that are started will act as servers. Animation code should be placed in the client loop portion of the code.

# Example Animations

- **Hello World:** `hello_world.rs`
- **Bouncing Ball:** `ball.rs`
- **DVD Logo:** `dvd.rs`
- **Elephant / Buffalo:** `elephant.rs`, `buffalo.rs`
