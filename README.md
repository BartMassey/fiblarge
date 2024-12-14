# fiblarge: fibonacci benchmark
Bart Massey 2024

After watching a "Sheafification of G" YouTube video.

Compute the nth Fibonacci number, for even n given on the
command line.

Timing for `1_000_000`:

* Python: 7s
* Rust: 2s
* C: 1.5s

All three give same answer.

Python uses builtin bigints. Rust uses `num::BigUint`. C
uses `GMP`.

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
