//! 32-bit x86 (ia32) definitions for Phoenix-RTOS.

use crate::prelude::*;

// `phoenix/arch/ia32/types.h`: `typedef __u64 id_t;`
pub type id_t = u64;

// `phoenix/arch/ia32/stdtypes.h`
pub type __u64 = c_ulonglong;
pub type __s64 = c_longlong;

pub type wchar_t = c_int;
