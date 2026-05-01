//! Memory detection — total bytes via sysctl + available via vm_stat.

use kei_machine_probe::{detect_memory, MockRunner};

const VM_STAT_FIXTURE: &str = "Mach Virtual Memory Statistics: (page size of 16384 bytes)
Pages free:                              123456.
Pages active:                             65000.
Pages inactive:                           45000.
Pages speculative:                         8000.
Pages throttled:                              0.
Pages wired down:                         55000.
Pages purgeable:                           2000.
\"Translation faults\":                   12345678.
Pages copy-on-write:                     123456.
Pages zero filled:                       654321.
Pages reactivated:                        12345.
Pages purged:                              1000.
File-backed pages:                        24000.
Anonymous pages:                          80000.
Pages stored in compressor:               18000.
Pages occupied by compressor:              7000.
Decompressions:                            3000.
Compressions:                              4500.
Swapins:                                      0.
Swapouts:                                     0.
";

#[test]
fn parses_16gb_total_and_vm_stat() {
    let runner = MockRunner::from_dir(".")
        .with_ok("sysctl_-n_hw.memsize", "17179869184\n")
        .with_ok("vm_stat", VM_STAT_FIXTURE);

    let mem = detect_memory(&runner);

    assert_eq!(mem.total_bytes, 17_179_869_184);
    assert_eq!(mem.total_gb(), 16);

    // free + inactive + speculative + purgeable = 123456 + 45000 + 8000 + 2000
    // = 178456 pages × 16384 bytes = 2_924_183_552
    let expected_avail: u64 = (123_456 + 45_000 + 8_000 + 2_000) * 16_384;
    assert_eq!(mem.available_bytes, expected_avail);

    // pressure = (wired + active + compressed) × 16384 / total
    // = (55000 + 65000 + 7000) × 16384 = 2_080_374_784 bytes
    // / 17_179_869_184 ≈ 12.1% → rounded 12
    assert_eq!(mem.pressure_pct, 12);
}

#[test]
fn handles_missing_vm_stat_gracefully() {
    let runner = MockRunner::from_dir(".")
        .with_ok("sysctl_-n_hw.memsize", "8589934592\n")
        .with_err("vm_stat", "command not found");

    let mem = detect_memory(&runner);
    assert_eq!(mem.total_bytes, 8_589_934_592);
    assert_eq!(mem.total_gb(), 8);
    assert_eq!(mem.available_bytes, 0);
    assert_eq!(mem.pressure_pct, 0);
}
