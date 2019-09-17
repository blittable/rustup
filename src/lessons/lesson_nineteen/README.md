# Lesson Nineteen: Web Assembly 

## Objectives 
- Understand Web Assembly Essentials with Rust 

## Overview 

Web Assembly is an intermediate language (il) that is currrently supported in all the major browsers. 

Java and .NET programmers may be familiar with the il for their languages (msil for .NET and bytecode for Java).  Any dotnet or java code can be disassembled:

Here is HelloWorld in .NET 4.0:

```c#
// Metadata version: v4.0.30319  
.assembly extern mscorlib  
{  
  .publickeytoken = (B7 7A 5C 56 19 34 E0 89 )                         // .z\V.4..  
  .ver 4:0:0:0  
}  
.assembly Hello  
{  
  .custom instance void [mscorlib]System.Runtime.CompilerServices.CompilationRelaxationsAttribute::.ctor(int32) = ( 01 00 08 00 00 00 00 00 )   
  .custom instance void [mscorlib]System.Runtime.CompilerServices.RuntimeCompatibilityAttribute::.ctor() = ( 01 00 01 00 54 02 16 57 72 61 70 4E 6F 6E 45 78   // ....T..WrapNonEx  
                                                                                                             63 65 70 74 69 6F 6E 54 68 72 6F 77 73 01 )       // ceptionThrows.  
  .hash algorithm 0x00008004  
  .ver 0:0:0:0  
}  
.module Hello.exe  
// MVID: {7C2770DB-1594-438D-BAE5-98764C39CCCA}  
.imagebase 0x00400000  
.file alignment 0x00000200  
.stackreserve 0x00100000  
.subsystem 0x0003       // WINDOWS_CUI  
.corflags 0x00000001    //  ILONLY  
// Image base: 0x00600000  
```

Because of some limitations with javascript *and* to allow programmers to write web application code in other languages, web assembly (WASM) was approved and implemented. 


Rust has excellent support for web assembly.  In fact, there's a guide specifically for Rust and Web Assembly:
[Rust Web Assembly Book] (https://rustwasm.github.io/docs/book/introduction.html)


## Setup and Configuration

There are two primary ways to work with Web Assembly in Rust: `wasm-pack` and directly with `wasm-bindgen`. `wasm-pack` uses `wasm-bindgen` and provides tooling for npm integration.

We'll be using `wasm-pack` and the setup instructions are here: [wasm-pack](https://rustwasm.github.io/docs/book/game-of-life/setup.html#the-rust-toolchain) 

After setup, we create a new project with: the `cargo generate --git https://github.com/rustwasm/wasm-pack-template`

To build, we run: `wasm-pack build` and our package is output:  ```Your wasm pkg is ready to publish at ./pkg.``

To web enable the app, from the main project directory (not .pkg), we run: `npm init wasm-app www`.

We then cd into the `www` we just created and install our npm dependencies: `npm install`

Running in Firefox:

![wasm running in firefox](artifacts/wasm_inff.png)


## The Bleeding Edge: Wasmtime 

There are some who think that the shared assembly standard should extend beyond the browser.  One such proposal is `wasi` which is a runtime with file system access, although it is sandboxed.

`cargo new --bin demo` 

`rustup target add wasm32-wasi`

`cargo build --target wasm32-wasi`

` wasmtime demo.wasm`







