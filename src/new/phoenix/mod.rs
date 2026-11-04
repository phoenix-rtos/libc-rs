//! Phoenix-RTOS libphoenix.
//!
//! * Headers: <https://github.com/phoenix-rtos/libphoenix/tree/master/include>
//! * Kernel-shared headers: `phoenix/*.h`

/// Directory: `sys/`
pub(crate) mod sys {
    pub(crate) mod socket;
}

pub(crate) mod pthread;
pub(crate) mod unistd;
