# system_engg

Systems programming from the ground up, in Rust, written by hand.

Each stage is a project you build yourself. Each one produces something the
next stage uses, so by the end you are running your own programs inside your
own shell and watching your own allocator in your own visualiser.

## The chain

| Stage | You build | It teaches | Later stages use it for |
|---|---|---|---|
| 0 | **Warm-ups** (this is where you are): ownership drills, a ring buffer, a bump allocator, `wc`, `cat`, `grep`, a progress tracker | Ownership, borrowing, slices, bits, errors, generics, lifetimes, `Box`, `unsafe`, `read`/`write`, child processes | `find_subslice`, `align_up`, `RingBuffer`, `Words` and `Bump` are all reused |
| 1 | **memscope**, a terminal visualiser: hex view of a byte arena, coloured regions, a scrolling event log | Terminal raw mode, event loops, rendering, trait design | Every later stage reports into it |
| 2 | **Allocator**: free list, splitting, coalescing, then `GlobalAlloc` | Heap internals, fragmentation, alignment, serious `unsafe` | Watched live in memscope; backs later data structures |
| 3 | **CHIP-8 emulator** that runs Pong and Tetris | Fetch/decode/execute, registers, timers, a call stack | memscope becomes its debugger |
| 4 | **Shell** | `fork`/`exec`, pipes, file descriptors, signals, job control | Runs your `wc`, `cat`, `grep` and the emulator |
| 5 | **Key-value store** with a write-ahead log | Durable disk I/O, `fsync`, on-disk formats, crash recovery | The data behind stage 6 |
| 6 | **TCP server**, a thread pool, then a small async executor | Sockets, threads, locks, atomics, what `async` compiles to | Serves the store over the network |
| 7 | **Kernel** (stretch) | Boot, interrupts, paging, bare metal | Your allocator, for real |

Stages 1 onward are added to this repo as you reach them, so each spec can
build on the code you wrote before it.

## Setup

Install Rust from <https://rustup.rs>, then from this folder:

```sh
cargo test -p warmups e01      # run one module's tests (they all fail at first)
```

Start with [stage0_warmups/README.md](stage0_warmups/README.md).

## How to work

1. **Read the picture.** Every module opens with a memory diagram. Redraw it
   on paper from memory before moving on.
2. **Predict.** Answer the module's questions in `ANSWERS.md` before writing
   code. Being wrong on paper first is where most of the learning happens.
3. **Write it yourself.** Type every line. Use the standard library docs
   (`rustup doc --std`) and The Rust Book (`rustup doc --book`) as references.
4. **Read the compiler.** Rust's error messages are the best tutor you have.
   Read the whole message, including the `help:` lines, before changing code.
5. **Commit, push, ask for review.** Review is where you find out what the
   tests cannot tell you.

## Working with Claude on this repo

[CLAUDE.md](CLAUDE.md) tells Claude to act as a tutor here: it reviews, asks
questions and gives hints in steps, and does not write exercise solutions.
Useful things to ask:

- "Review e03." Feedback on correctness, idiom and what to read next.
- "Hint for `reverse` in e08." The smallest nudge first; ask again for more.
- "Check my ANSWERS.md for e01."
- "I'm done with stage 0, set up stage 1."

## Reading alongside

- *The Rust Programming Language* (free, `rustup doc --book`): chapters 4, 8, 9, 10, 15 for stage 0.
- *Operating Systems: Three Easy Pieces* (free at ostep.org): from stage 2 on.
- *Computer Systems: A Programmer's Perspective*: chapters 2, 3 and 9 for stages 2 and 3.
- *Rust Atomics and Locks* by Mara Bos (free online): stage 6.
