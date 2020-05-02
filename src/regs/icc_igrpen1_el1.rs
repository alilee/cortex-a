// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Interrupt Controller Interrupt Group 1 Enable register
//!
//! Controls whether Group 1 interrupts are enabled for the current Security state.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {
    u32,
    ICC_IGRPEN1_EL1 [
        Enable OFFSET(0) NUMBITS(1) []  // Enables Group 1 interrupts for the current Security state.
    ]
}

pub struct Reg;

impl RegisterReadWrite<u32, ICC_IGRPEN1_EL1::Register> for Reg {
    sys_coproc_read_raw!(u32, "ICC_IGRPEN1_EL1");
    sys_coproc_write_raw!(u32, "ICC_IGRPEN1_EL1");
}

pub static ICC_IGRPEN1_EL1: Reg = Reg {};
