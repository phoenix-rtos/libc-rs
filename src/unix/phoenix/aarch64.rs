//! AArch64 definitions for Phoenix-RTOS.

use crate::prelude::*;

// `phoenix/arch/aarch64/types.h`: `typedef __u64 id_t;`
pub type id_t = u64;

// `phoenix/arch/aarch64/stdtypes.h`: 64-bit, so `long` rather than `long long`.
pub type __u64 = c_ulong;
pub type __s64 = c_long;

// The ARM EABI makes `wchar_t` unsigned.
pub type wchar_t = c_uint;
