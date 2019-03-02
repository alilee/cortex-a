/*
 * Copyright (c) 2018 by the author(s)
 *
 * =============================================================================
 *
 * Licensed under either of
 *   - Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
 *   - MIT License (http://opensource.org/licenses/MIT)
 * at your option.
 *
 * =============================================================================
 *
 * Author(s):
 *   - Andre Richter <andre.o.richter@gmail.com>
 */

//! EL0 Read-Only Software Thread ID
//!
//! Provides a location where software executing at EL1 or higher can store
//! thread identifying information that is visible to software executing at
//! EL0, for OS management purposes.

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "TPIDRRO_EL0");
    sys_coproc_write_raw!(u64, "TPIDRRO_EL0");
}

pub static TPIDRRO_EL0: Reg = Reg {};
