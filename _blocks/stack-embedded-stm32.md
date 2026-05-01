# STACK — Embedded Rust STM32 (embassy / cortex-m)

Rust-first by default. STM32H743 is a common reference MCU.

**Crate skeleton:**
```rust
#![no_std]
#![no_main]
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use defmt_rtt as _;
use panic_probe as _;
```

**HAL choice:** embassy (async, preferred for new work) OR cortex-m-rt + HAL crates (if you need full sync control). Pick one per project; no mixing.

**Memory budget (MANDATORY comment in `Cargo.toml`):**
```toml
# STM32H743ZI — flash 2 MiB, RAM 1 MiB. Current: flash 312 KiB / RAM 84 KiB.
```
Update on every commit that moves size by > 4 KiB.

**I/O rules:**
- DMA for any transfer > 32 bytes (UART, SPI, I2C, ADC bursts). Polling in ISRs bricks latency.
- Interrupt priorities EXPLICIT via `NVIC`. Default `0` = highest — two handlers at priority 0 deadlock.
- NO heap allocations in ISRs. `heapless::Vec` / `heapless::String` only, with compile-time capacity.

**Allocator:** default is NO allocator (`#![no_std]` bare). If you add `alloc`, document why — usually avoidable with `heapless`.

**Debug:** `defmt` + `probe-rs` for logging. NEVER `println!` (no stdout).

**Forbidden:** `alloc` without justification; `.unwrap()` outside `#[cfg(debug_assertions)]`; interrupt handlers > 30 LOC (move logic to a task); DMA without `'static` buffers (UB with stack buffers); flashing without `probe-rs erase` when changing memory map.
