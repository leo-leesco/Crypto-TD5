# Ed25519

Please check the latest version on [GitHub](github.com/leo-leesco/Crypto-TD5).

## Build

`cargo build` produces `keygen`, `sign` and `verify` in `target/debug`.

If you want the optimized version, run `cargo build --release`, and the executables can then be found in `target/release`.

## Requirements

`keygen` expects a string.

`sign` expects :
- the path to the datafile
- the path to `prefix.sk`
- the path to `prefix.pk`

`verify` expects :
- the path to the sigfile
- the path to `prefix.pk`
