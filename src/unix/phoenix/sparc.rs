//! 32-bit SPARC V8 (LEON) definitions for Phoenix-RTOS.

use crate::prelude::*;

// `phoenix/arch/sparcv8leon/types.h`: `typedef __u32 id_t;`
pub type id_t = u32;

// `phoenix/arch/sparcv8leon/stdtypes.h`
pub type __u64 = c_ulonglong;
pub type __s64 = c_longlong;

// `__WCHAR_TYPE__` is `long int` on this toolchain.
pub type wchar_t = c_long;
