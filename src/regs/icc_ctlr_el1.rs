// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Interrupt Controller Control Register (EL1)
//!
//! Controls aspects of the behavior of the GIC CPU interface and provides information about
//! the features implemented.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {
    u32,
    ICC_CTLR_EL1 [
        A3V OFFSET(15) NUMBITS(1) [],     // Affinity 3 Valid (RO)
        SEIS OFFSET(14) NUMBITS(1) [],    // SEI Support (RO)
        IDbits OFFSET(11) NUMBITS(3) [],  // Identifier bits (RO)
        PRIbits OFFSET(8) NUMBITS(3) [],  // Priority bits (RO)
        PMHE OFFSET(6) NUMBITS(1) [],     // Priority Mask Hint Enable
        EOImode OFFSET(1) NUMBITS(1) [],  // End of Interrupt mode
        CBPR OFFSET(0) NUMBITS(1) []      // Common Binary Point Register
    ]
}

pub struct Reg;

impl RegisterReadWrite<u32, ICC_CTLR_EL1::Register> for Reg {
    sys_coproc_read_raw!(u32, "ICC_CTLR_EL1");
    sys_coproc_write_raw!(u32, "ICC_CTLR_EL1");
}

pub static ICC_CTLR_EL1: Reg = Reg {};
