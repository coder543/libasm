# libasm

This is a library to provide inline assembly support for stable Rust.

All that is required to add inline assembly to a project is to create a build script similar to this:

```rust
extern crate libasm;

fn main() {
    libasm::parse();
}
```

Ths is an example `main.rs`:

```rust
#[macro_use]
extern crate libasm;

lasm! {
    "x86_64-unknown-linux-gnu"
    fn add2 -> %rax {
        mov %rax, %rdi
        add %rax, %rsi
    }

    "x86_64-pc-windows-msvc"
    fn add2 -> rax {
        mov rax, rcx
        add rax, rdx
    }
}

extern "C" {
    fn add2(a: u64, b: u64) -> u64;
}

fn main() {
    let x = unsafe { add2(3, 4) };
    println!("Hello, world! 3 + 4 = {}", x);
}
```

A `lasm!` declaration provides a list of target-triple specific assembly functions. It is required to declare your own prototype for the function, as shown here after the `lasm!` block. If the target-triple being compiled for does not have a matching declaration, you will encounter a linker error unless the implementation comes from somewhere else.