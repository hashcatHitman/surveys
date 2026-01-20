# Survey questions

This is a quick survey about debugging support in Rust. The goal is to
understand how Rustaceans use debuggers when working with Rust, what pain points
they face, and what can be done to improve the experience.

## Cohort Questions

### Do you use Rust?

Type: select one

- Yes, I use Rust [`NEXT`](#what-tools-do-you-use-to-debug-rust-programs-on-which-operating-systems)
- No, I don't currently use Rust, but I have in the past [`NEXT`](#were-issues-with-debugging-support-the-primary-reason-why-you-stopped-using-rust)
- No, I have never used Rust [`NEXT`](#is-there-anything-else-you-would-like-to-tell-us-about-debugging-support-in-rust)

### Were issues with debugging support the primary reason why you stopped using Rust?

Type: select one

- Yes
- No, but they were one of the reasons why I stopped using Rust
- No [`NEXT`](#is-there-anything-else-you-would-like-to-tell-us-about-debugging-support-in-rust)

## Your use of debuggers in Rust

### What tools do you use to debug Rust programs on which operating systems?

To clarify: the "operating system" being asked for is the one on the machine you
write your code using, not the one your code runs on.

Type: matrix (select all that apply)

Your Operating System:

- Linux
- Windows 10/11
- Windows 8.1 or older
- Windows Subsystem for Linux
- macOS
- Other

Tools:

- gdb (CLI)
- gdb (IDE/Extension)
- lldb (CLI)
- lldb (IDE/Extension)
- [BugStalker](https://github.com/godzie44/BugStalker)
- WinDbg
- [Visual Studio](https://visualstudio.microsoft.com/)
- Print Debugging (`println!`, `eprintln!`, `print!`, `eprint!`)
- The `dbg!` macro
- Other CLI tool
- Other IDE/Extension

<!-- TODO: Should we list <https://github.com/metacraft-labs/codetracer>? -->
<!-- TODO: Should we list <https://github.com/kellpossible/stack-debug>? -->
<!-- TODO: Should we list <https://github.com/SeaQL/FireDBG.for.Rust>? -->
<!-- TODO: Should we list debuggers specific to a particular target platform?
(i.e. iOS-specific debugger when developing for iOS) -->
<!-- TODO: Should we switch to a free-form response for "Other" instead? -->

### What are you using debuggers for?

Type: select all that apply

- Getting stack traces from hung/crashed processes
- Step-debugging
- Debugging async code
- Curiosity/learning
- Other (open response)

<!-- TODO: Can we list more uses? Should we? -->

## Difficulties using debuggers in Rust

### When you don't use a dedicated debugger (as opposed to print debugging or none at all), why don't you?

Type: select all that apply

- I don't need to debug because my code works.
- I don't know how to use debuggers.
- It's easier to solve problems through other means.
- It's faster to solve problems through other means.
- The types from external libraries I'm working with have poor debugger support.
- The types from the standard library I'm working with have poor debugger support.
- The language features I'm working with have poor debugger support.
- Other (open response)

### Do you experience any issues when trying to step through code with your debugger?

Type: select one

- Yes
- No [`NEXT`](#what-standard-library-types-are-hard-to-work-with-when-debugging)

### When do you experience issues with trying to step through code with your debugger?

Type: select all that apply

- When iterators are involved
- When closures are involved
- When macros are involved
- When panics are involved
- When trait objects are involved
- When function pointers are involved
- Other (open response)

### What standard library types are hard to work with when debugging?

For example, this could include things like smart pointers or heavily nested
data structures.

Type: free text

> Ideally, encourage people to use fully qualified paths in their answer where
> possible. For example, I might find it really awful trying to use a debugger
> with `std::boxed::Box`. I'd like to avoid ambiguity here, but I also don't
> think anyone would think it's a great idea to make a huge checklist of types
> from the standard library.

### If you are a library author, are you aware of and using the debugger visualizer attribute?

This attribute allows you to provide specialized visualizers for your custom
types. You can find more information about it in
[The Rust Reference: Debugger Attributes](https://doc.rust-lang.org/reference/attributes/debugger.html).

Type: select one

- I am not a library author. [`NEXT`](#if-you-could-use-rustup-to-receive-updates-to-the-debugging-experience-decoupled-from-the-toolchain-would-you-specifically-updates-to-the-visualizer-scripts)
- I was not aware! [`NEXT`](#if-you-could-use-rustup-to-receive-updates-to-the-debugging-experience-decoupled-from-the-toolchain-would-you-specifically-updates-to-the-visualizer-scripts)
- I was aware, and already use it. [`NEXT`](#if-you-could-use-rustup-to-receive-updates-to-the-debugging-experience-decoupled-from-the-toolchain-would-you-specifically-updates-to-the-visualizer-scripts)
- I was aware, and do not use it.

### Why don't you use the debugger visualizer attribute?

Type: select all that apply

- I do not know how to write visualizer scripts.
- My debugger is not supported.
- My libraries' types do not need them.
- Other (open response)

### If you could use `rustup` to receive updates to the debugging experience decoupled from the toolchain, would you? (specifically, updates to the visualizer scripts)

Type: select one

- Yes
- No

## Personal feedback

### Is there anything else you would like to tell us about debugging support in Rust?

Type: free text (optional)
