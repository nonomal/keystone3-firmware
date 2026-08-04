# Harden firmware RNG linkage for issue #2255

## Requirement

Eliminate the insecure linear-congruential PRNG fallback from hardware firmware builds so that losing Keystone's platform `random_buffer()` implementation causes a build failure instead of silently switching SLIP-39 operations to deterministic randomness.

## Classification and branch target

- Classification: security hardening Fix.
- Proposed branch: `clean-dead-code`.
- Merge target: `master`.
- Issue: https://github.com/KeystoneHQ/keystone3-firmware/issues/2255

## Current-state findings

- `src/managers/keystore.c` provides the hardware build's strong `random_buffer()` implementation and combines MCU TRNG, DS28S60 RNG, and ATECC608B RNG output.
- `src/crypto/slip39/trezor-crypto/rand.c` also compiles a weak `random_buffer()` backed by a deterministic linear-congruential generator whenever `COMPILE_SIMULATOR` is not defined.
- SLIP-39 calls `random_buffer()` directly. The current production ELF resolves that call to the strong Keystone implementation.
- The current production ELF does not retain `random32()` after section garbage collection, but the fallback remains available at link time if the strong implementation is removed or excluded.
- The simulator has no hardware RNG implementation and must retain a software-only `random_buffer()` stub.

## TODO

- [x] Delete the trezor-crypto LCG, weak fallback, and unused helper functions.
- [x] Preserve a simulator-only `random_buffer()` stub using the standard library PRNG.
- [x] Confirm the hardware object exports no fallback and a partial link resolves `random_buffer()` to Keystone's strong implementation.
- [x] Confirm the hardware object and partial link contain none of the removed LCG/helper symbols.
- [x] Confirm compiling/linking without a platform `random_buffer()` fails for a hardware configuration.
- [x] Review the final diff and append the completion summary.

## Expected file/module changes

- `src/crypto/slip39/trezor-crypto/rand.c`: remove the insecure test PRNG and retain only a simulator stub.
- `src/crypto/slip39/trezor-crypto/rand.h`: remove declarations for deleted, unused PRNG helpers.
- `docs/plans/fix-2255-prng-fallback.md`: implementation record and completion summary.

No entropy-generation algorithm or hardware RNG mixing logic is expected to change.

## Verification plan

- Run `build.bat production` for the main hardware firmware.
- Inspect `build/mh1903.elf` with `arm-none-eabi-nm` and verify `random_buffer` is present while `random32`, `random_reseed`, `random_uniform`, and `random_permute` are absent.
- Compile `rand.c` with and without `COMPILE_SIMULATOR` and inspect the objects to verify only the simulator configuration defines `random_buffer()`.
- Perform a focused negative link check showing that a hardware consumer of `random_buffer()` fails to link when no platform implementation is supplied.
- Run `git diff --check`.

## Commit naming plan

- `fix: remove firmware PRNG fallback`
- Include the code change and its security rationale in one logical commit; keep any post-implementation workflow-document-only update separate only if needed.

## Risks, assumptions, and open questions

- Assumption: no code calls `random32()`, `random_uniform()`, `random_permute()`, or `random_reseed()` directly. Repository-wide symbol search currently confirms this.
- The simulator still receives non-cryptographic random bytes, now through its standard library PRNG rather than the deleted LCG.
- The change intentionally converts a future missing platform RNG implementation into a link-time failure.
- Full production builds may depend on the locally installed ARM toolchain and prebuilt external libraries.

## Out of scope

- Changing Keystone's existing three-source RNG mixing algorithm.
- Adding boot-time statistical or repeated-value checks for hardware RNG sources.
- General refactoring of the vendored trezor-crypto library.
- Adding unrelated firmware CI infrastructure.

## Completion summary

### Implemented scope

- Removed the deterministic LCG, weak `random_buffer()` fallback, and unused random helper functions from the vendored trezor-crypto implementation.
- Reduced `rand.h` to the sole API used by the firmware: `random_buffer()`.
- Retained a simulator-only `random_buffer()` implementation backed by the standard library PRNG.
- Followed the focused implementation approach demonstrated by Quantus-Network/keystone3-firmware PR #14.

### Behavior and security boundary

- Hardware firmware builds no longer receive any RNG implementation from trezor-crypto.
- Removing or excluding Keystone's hardware `random_buffer()` now produces an undefined-symbol link failure instead of silently activating deterministic randomness.
- Keystone's hardware RNG implementation and its three-source mixing behavior are unchanged.

### Main changed files

- `src/crypto/slip39/trezor-crypto/rand.c`
- `src/crypto/slip39/trezor-crypto/rand.h`

### Verification results

- Hardware compilation of `rand.c` passed; `arm-none-eabi-nm` reported no exported or referenced symbols in the resulting object.
- A partial hardware link with the compiled `keystore.c` object passed and exposed only the strong `random_buffer` symbol.
- A focused hardware link without `keystore.c` failed as expected with `undefined reference to random_buffer`.
- Simulator compilation and link passed; the object defines `random_buffer` and references the host standard library `rand` function.
- Repository search found no remaining source references to the removed APIs.
- `git diff --check` passed.
- `build.bat production` compiled the changed hardware object but the full build stopped in existing unrelated code at `src/ui/gui_analyze/gui_resolve_ur.c:92` because `DeriveContextHashRequest` is undeclared.

### Known limitations and follow-up work

- A new complete production ELF could not be generated because of the unrelated compile error above. The focused object, positive partial-link, and negative-link checks cover the changed RNG linkage behavior.
- No boot-time hardware RNG health test was added; that remains separate follow-up work.

### Branch and commit

- Branch: `clean-dead-code`
- Commit subject: `fix: remove insecure PRNG fallback`
