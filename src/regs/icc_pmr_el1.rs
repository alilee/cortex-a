// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Interrupt Controller Interrupt Priority Mask Register
//!
//! Provides an interrupt priority filter. Only interrupts with higher priority than the
//! value in this register are signaled to the PE.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {
    u32,
    ICC_PMR_EL1 [
        Priority OFFSET(0) NUMBITS(8) []  // Priority mask level for the CPU interface
    ]
}

pub struct Reg;

impl RegisterReadWrite<u32, ICC_PMR_EL1::Register> for Reg {
    sys_coproc_read_raw!(u32, "ICC_PMR_EL1");
    sys_coproc_write_raw!(u32, "ICC_PMR_EL1");
}

pub static ICC_PMR_EL1: Reg = Reg {};
