# Rust Try Catch - Reinventing the nightmare!

Welcome to **`rust-try-catch`**, the crate that dares to bring the spirit of Java's infamous `try-catch-finally` block to Rust's peaceful shores.
Why settle for *Result* and *Option* when you can spice up your life with panics, exceptions, and an uncanny resemblance to the debugging hellscape you left behind in your old job?

---

## Features (or "Sins Against Rustaceans")

- **Bring Back `throw` and `catch`**: Just when you thought the only `throw`ing and `catch`ing you do was in your high-schools baseball team, we decided to give it a whole new purpose: nightmares.
- **Panic Recovery**: Because catching panics makes more sense than fixing your code, right?
- **Generic Exception Handling**: Catch `anything`. Who needs type safety when you can have mystery bugs?
- **Finally Blocks**: Clean up after your disasters. Or don’t. We don’t judge.
- **Readability? Nope.** It’s not a bug; it’s a *feature*. Embrace spaghetti syntax and callback hell in a language that fought so hard to avoid it.

---

## Why?

### Because Error Handling in Rust is Too Safe
Rust’s Result and Option types are just too perfect.
Developers are practically forced to think about errors and handle them responsibly. Where's the thrill of runtime crashes?
Rust must embrace the unpredictability of try-catch so we can get back to debugging stack traces at 2 AM, like real programmers.

### We Miss Nested Pyramid Code
Without try-catch, we’re stuck writing linear, readable error handling with .and_then() and the ? operator. UGH Gross!
We want to go back to deeply nested code where `try { try { try { ... } catch { ... } } catch { ... } }` creates a delightful maze for future maintainers to explore.

### The World Needs More `catch exception (e)` Jokes
In the try-catch world, you can slap a generic `catch exception (e)` block everywhere and call it "handling errors."
It’s a rite of passage to spend hours debugging only to realize `catch` swallowed an error you never logged.
Rust users deserve the same rite, don’t you think?

### Your life is already hard enough
The `unwrap()` method is far too stigmatized in Rust.
What if we could just shove all possible errors into a catch block and forget about them?
Productivity will skyrocket when developers no longer worry about pesky things like whether their program works correctly.

### Rust Needs Drama
rust-try-catch would introduce exciting debates about whether to use exceptions or results.
These debates could rival the classic apples vs. oranges.
Rust forums and Reddit threads are way too focused on productive discussions right now. Where’s the chaos?

---

## Getting Started (Warning: Regret Ahead)

### Add to `Cargo.toml`

```toml
[dependencies]
rust-try-catch = "0.1.0" # Or whatever version the world has suffered through.
```

---

# Usage: The Gift That Keeps on Giving


## `try_catch!` Macro
At its core, `try_catch!` is a macro designed to take Rust's Result-based error handling
and replace it with a beautifully convoluted system of try, catch, finally, and questionable life choices.

### Syntax
```text
try_catch! {
    try {
        // Your innocent code goes here.
    } catch (exception => Type) {
        // Caught an exception? Great. Now figure out what to do with it.
    } catch exception (name) {
        // Catch all other exceptions not previously caught.
    } catch panic (panic_info) {
        // For when you're feeling nostalgic about debugging segfaults.
    } finally {
        // Clean up whatever mess you've created.
    }
}
```

#### Explanation of Blocks:
- `try`: Contains the code you're actually trying to run.
- `catch (exception => Type)`: Specifically handles exceptions of type Type. Use this for precise error matching.
- `catch exception (name)`: A catch-all for any exceptions that aren't caught by specific type matches. It's like the last line of defense.
- `catch panic (panic_info)`: Handles panics (because, why not?).
- `finally`: Ensures some cleanup happens, even if all else fails.

### Example (You Asked for It)

```
use rust_try_catch::try_catch;

let result = try_catch! {
    try {
        panic!("Oops!"); // Because Rust's strictness is for cowards.
    } catch panic (info) {
        println!("Panic caught: {:?}", info);
        -42 // Return values that make sense, or don't. We don't care.
    }
};
assert_eq!(result, -42);
```

---

## `tri!` Macro

The little sibling of try_catch!, tri! unwraps a Result and throws an exception if it’s an Err.
It’s like ?, but with extra regret.

### Example

```
use rust_try_catch::{try_catch, tri};

let result = try_catch! {
    try {
        tri!(Err::<i32, &str>("Something went wrong"))
    } catch (e => &str) {
        println!("Handled error: {}", e);
        -1
    }
};
assert_eq!(result, -1);
```

---

## `throw` Function

When you’re tired of meaningful error propagation, just throw an exception instead.
Bonus points if you hide it under 20 layers of function calls.

### Example

```should_panic
use rust_try_catch::throw;

throw("Goodbye, world!"); // It's not just a panic; it's an *exceptional* panic.
```

---

## `throw_guard` and `closure_throw_guard`

When you need to keep your code from completely imploding due to an unchecked throw, `throw_guard` and `closure_throw_guard` are here to wrap functions or closures in a layer of protection—or at least an illusion of it.

The `throw_guard` procedural macro can be applied to functions to ensure that thrown exceptions do not escape their bounds.
Use this when you're feeling particularly responsible.

This works with async functions too

### Example
```should_panic
use rust_try_catch::throw_guard;

#[throw_guard]
fn risky_function() {
    rust_try_catch::throw("Oops, I did it again!");
}

fn main() {
    // Instead of crashing, exceptions from risky_function are turned into panics.
    risky_function();
}
```


The `closure_throw_guard` macro wraps closures instead of functions. Perfect for when you want your lambda to crash gracefully.
### Example
```should_panic
use rust_try_catch::closure_throw_guard;

let guarded_closure = closure_throw_guard!(|| {
    rust_try_catch::throw("Not today!");
});

guarded_closure(); // Exception caught and re-thrown as a panic.
```

---

# Advanced: Layering Bad Decisions

What happens when you combine specific catches,
generic exception handling, panic recovery, and a finally block? Chaos.

```
use rust_try_catch::try_catch;

let result = try_catch! {
    try {
        panic!("Let's see what happens!");
    } catch (e => &'static str) {
        println!("Caught a static str: {}", e);
        0
    } catch exception (ex) {
        println!("Caught a generic exception: {:?}", ex);
        1
    } catch panic (info) {
        println!("Caught a panic: {:?}", info);
        -1
    } finally {
        println!("Because even bad code needs cleanup.");
    }
};

assert_eq!(result, -1);
```

# Why use this crate?
1. You’re feeling nostalgic for Java.
2. You’re bored and like chaos.
3. Rust's error handling is too predictable.
4. Your bug tracker is feeling lonely.
