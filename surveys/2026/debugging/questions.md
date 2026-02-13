# Survey questions

This is a quick survey about debugging support in Rust. The goal is to
understand how Rustaceans use debuggers when working with Rust, what pain points
they face, and what can be done to improve the experience.

## Cohort Questions

### How would you rate your Rust expertise?

Even if you no longer use Rust, we'd like to know.

Type: select one

- I have never used Rust [`NEXT`](#do-you-use-debuggers-in-other-programming-languages)
- Beginner
- Intermediate
- Advanced

### Do you currently use Rust?

Type: select one

- Yes, I use Rust [`NEXT`](#do-you-use-debuggers-in-rust)
- No, I don't currently use Rust, but I have in the past [`NEXT`](#did-you-use-debuggers-in-rust)

### Do you use debuggers in Rust?

Type: select one

- Yes [`NEXT`](#what-tools-do-you-use-to-debug-rust-programs-on-which-operating-systems)
- No, I don't currently use debuggers in Rust, but I have in the past [`NEXT`](#what-tools-do-you-use-to-debug-rust-programs-on-which-operating-systems)
- No, I have never used debuggers in Rust [`NEXT`](#do-you-use-debuggers-in-other-programming-languages)

### Did you use debuggers in Rust?

Type: select one

- Yes, in the past [`NEXT`](#were-issues-with-debugging-support-the-primary-reason-why-you-stopped-using-rust)
- No, never [`NEXT`](#do-you-use-debuggers-in-other-programming-languages)

### Were issues with debugging support the primary reason why you stopped using Rust?

Type: select one

- Yes [`NEXT`](#what-tools-do-you-use-to-debug-rust-programs-on-which-operating-systems)
- No, but they were one of the reasons why I stopped using Rust [`NEXT`](#what-tools-do-you-use-to-debug-rust-programs-on-which-operating-systems)
- No [`NEXT`](#is-there-anything-else-you-would-like-to-tell-us-about-debugging-support-in-rust)

### Do you use debuggers in other programming languages?

Type: select one

- Yes [`NEXT`](#is-there-anything-else-you-would-like-to-tell-us-about-debugging-support-in-rust)
- No, I don't currently use debuggers in other programming languages, but I have in the past [`NEXT`](#is-there-anything-else-you-would-like-to-tell-us-about-debugging-support-in-rust)
- No, I have never used debuggers in other programming languages [`NEXT`](#is-there-anything-else-you-would-like-to-tell-us-about-debugging-support-in-rust)

## Your use of debuggers in Rust

### What tools do you use to debug Rust programs on which operating systems?

To clarify: the "operating system" being asked for is the one on the machine you
write your code using, not the one your code runs on. If you don't see the
debugger you use listed, you'll have an opportunity to tell us about it in the
next question.

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
- Special embedded debugger
- The `dbg!` macro
- I don't know, I just hit "Debug" in my IDE

### What other debuggers do you use?

Type: free form (optional)

### What are you using debuggers for?

Type: select all that apply

- Getting stack traces from hung/crashed processes
- Line-by-line stepping
- Debugging async code
- Curiosity/learning
- Other (open response)

<!-- TODO: Can we list more uses? Should we? -->

### Do you debug programs that combine Rust with any of the following languages?

Type: select all that apply (optional)

- C
- C#
- C++
- Go
- Java
- JavaScript
- Kotlin
- Python
- Swift
- Zig
- Other (open response)

## Difficulties using debuggers in Rust

### When you don't use a debugger, why don't you?

Type: select all that apply

- I don't need to debug because my code works.
- I don't know how to use debuggers.
- It's easier or faster to solve problems through print debugging or logs.
- It's easier or faster to let an AI model debug my code.
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

Type: free form

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

- I am not a library author. [`NEXT`](#which-of-these-pain-points-have-you-experienced-using-a-debugger-with-rust)
- I was not aware! [`NEXT`](#which-of-these-pain-points-have-you-experienced-using-a-debugger-with-rust)
- I was aware, and already use it. [`NEXT`](#which-of-these-pain-points-have-you-experienced-using-a-debugger-with-rust)
- I was aware, and do not use it.

### Why don't you use the debugger visualizer attribute?

Type: select all that apply

- I do not know how to write visualizer scripts.
- My debugger is not supported.
- My libraries' types do not need them.
- Other (open response)

### Which of these pain points have you experienced using a debugger with Rust?

Type: select all that apply (optional)

- Cannot print variables (i.e. optimized-out code)
- Conditional breakpoints don't trigger
- Current line number jumps around unexpectedly
- Difficult to trace dataflow
- Don't have a source view of the current line being run
- Expression parser doesn't support expressions that you want to write
- Incorrect information displayed (i.e. line numbers, variable values)
- Often step too many times and need to restart from scratch
- Too much detail/irrelevant information (i.e. assembly views)
- Unintuitive interface/errors
- Sub-optimal representations of values (e.g. `Vec<T>` is shown as a pointer, length and capacity rather than the elements)
- Cannot determine the actual type of trait objects

## Personal feedback

### Is there anything else you would like to tell us about debugging support in Rust?

Type: free form (optional)
