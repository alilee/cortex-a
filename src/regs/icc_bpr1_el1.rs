// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Interrupt Controller Binary Point Register 1
//!
//! Defines the point at which the priority value fields split into two parts, the group
//! priority field and the subpriority field. The group priority field determines Group 1
//! interrupt preemption.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {
    u32,
    ICC_BPR1_EL1 [
        BinaryPoint OFFSET(0) NUMBITS(3) []  // controls how the 8-bit interrupt priority
                                             // field is split into a group priority field,
                                             // that determines interrupt preemption, and a
                                             // subpriority field
    ]
}

pub struct Reg;

impl RegisterReadWrite<u32, ICC_BPR1_EL1::Register> for Reg {
    sys_coproc_read_raw!(u32, "ICC_BPR1_EL1");
    sys_coproc_write_raw!(u32, "ICC_BPR1_EL1");
}

pub static ICC_BPR1_EL1: Reg = Reg {};
