//! 32-bit ARM definitions for Phoenix-RTOS.

use crate::prelude::*;

// `phoenix/arch/armv7a/types.h` has `typedef __u64 id_t;` but
// `phoenix/arch/armv7r/types.h` and `armv8r/types.h` have `typedef __u32 id_t;`.
// `target_arch` is "arm" for all three, so the two groups are told apart by
// `target_env`: the MMU-less Cortex-R targets use `libphoenix_nommu`.
#[cfg(not(target_env = "libphoenix_nommu"))]
pub type id_t = u64;
#[cfg(target_env = "libphoenix_nommu")]
pub type id_t = u32;

// `phoenix/arch/armv7a/stdtypes.h`
pub type __u64 = c_ulonglong;
pub type __s64 = c_longlong;

// The ARM EABI makes `wchar_t` unsigned.
pub type wchar_t = c_uint;
