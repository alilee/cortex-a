// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Interrupt Controller System Register Enable register (EL1)
//!
//! Controls whether the System register interface or the memory-mapped interface to the GIC
//! CPU interface is used for EL0 and EL1.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {
    u32,
    ICC_SRE_EL1 [
        DIB OFFSET(2) NUMBITS(1) [],  // Disable IRQ bypass
        DFB OFFSET(1) NUMBITS(1) [],  // Disable FIQ bypass
        SRE OFFSET(0) NUMBITS(1) []   // System Register Enable
    ]
}

pub struct Reg;

impl RegisterReadWrite<u32, ICC_SRE_EL1::Register> for Reg {
    sys_coproc_read_raw!(u32, "ICC_SRE_EL1");
    sys_coproc_write_raw!(u32, "ICC_SRE_EL1");
}

pub static ICC_SRE_EL1: Reg = Reg {};
