// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Memory Attribute Indirection Register - EL1
//!
//! Provides the memory attribute encodings corresponding to the possible AttrIndx values in a
//! Long-descriptor format translation table entry for stage 1 translations at EL1.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    MAIR_EL1 [
        /// Attribute 7
        Attr7_Device OFFSET(56) NUMBITS(8) [
            nGnRnE = 0b00000000,
            nGnRE = 0b00000100,
            nGRE = 0b00001000,
            GRE = 0b00001100
        ],
        Attr7_Outer OFFSET(60) NUMBITS(4) [
            NonCacheable = 0b0100,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],
        Attr7_Inner OFFSET(56) NUMBITS(4) [
            WriteThrough_Transient_AllocRW = 0b0011,
            WriteThrough_Transient_AllocR = 0b0010,
            WriteThrough_Transient_AllocW = 0b0001,
            NonCacheable = 0b0100,
            WriteBack_Transient_AllocRW = 0b0111,
            WriteBack_Transient_AllocR = 0b0110,
            WriteBack_Transient_AllocW = 0b0101,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],

        /// Attribute 6
        Attr6_Device OFFSET(48) NUMBITS(8) [
            nGnRnE = 0b00000000,
            nGnRE = 0b00000100,
            nGRE = 0b00001000,
            GRE = 0b00001100
        ],
        Attr6_Outer OFFSET(52) NUMBITS(4) [
            NonCacheable = 0b0100,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],
        Attr6_Inner OFFSET(48) NUMBITS(4) [
            WriteThrough_Transient_AllocRW = 0b0011,
            WriteThrough_Transient_AllocR = 0b0010,
            WriteThrough_Transient_AllocW = 0b0001,
            NonCacheable = 0b0100,
            WriteBack_Transient_AllocRW = 0b0111,
            WriteBack_Transient_AllocR = 0b0110,
            WriteBack_Transient_AllocW = 0b0101,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],

        /// Attribute 5
        Attr5_Device OFFSET(40) NUMBITS(8) [
            nGnRnE = 0b00000000,
            nGnRE = 0b00000100,
            nGRE = 0b00001000,
            GRE = 0b00001100
        ],
        Attr5_Outer OFFSET(44) NUMBITS(4) [
            NonCacheable = 0b0100,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],
        Attr5_Inner OFFSET(40) NUMBITS(4) [
            WriteThrough_Transient_AllocRW = 0b0011,
            WriteThrough_Transient_AllocR = 0b0010,
            WriteThrough_Transient_AllocW = 0b0001,
            NonCacheable = 0b0100,
            WriteBack_Transient_AllocRW = 0b0111,
            WriteBack_Transient_AllocR = 0b0110,
            WriteBack_Transient_AllocW = 0b0101,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],

        /// Attribute 4
        Attr4_Device OFFSET(32) NUMBITS(8) [
            nGnRnE = 0b00000000,
            nGnRE = 0b00000100,
            nGRE = 0b00001000,
            GRE = 0b00001100
        ],
        Attr4_Outer OFFSET(36) NUMBITS(4) [
            NonCacheable = 0b0100,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],
        Attr4_Inner OFFSET(32) NUMBITS(4) [
            WriteThrough_Transient_AllocRW = 0b0011,
            WriteThrough_Transient_AllocR = 0b0010,
            WriteThrough_Transient_AllocW = 0b0001,
            NonCacheable = 0b0100,
            WriteBack_Transient_AllocRW = 0b0111,
            WriteBack_Transient_AllocR = 0b0110,
            WriteBack_Transient_AllocW = 0b0101,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],

        /// Attribute 3
        Attr3_Device OFFSET(24) NUMBITS(8) [
            nGnRnE = 0b00000000,
            nGnRE = 0b00000100,
            nGRE = 0b00001000,
            GRE = 0b00001100
        ],
        Attr3_Outer OFFSET(28) NUMBITS(4) [
            NonCacheable = 0b0100,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],
        Attr3_Inner OFFSET(24) NUMBITS(4) [
            WriteThrough_Transient_AllocRW = 0b0011,
            WriteThrough_Transient_AllocR = 0b0010,
            WriteThrough_Transient_AllocW = 0b0001,
            NonCacheable = 0b0100,
            WriteBack_Transient_AllocRW = 0b0111,
            WriteBack_Transient_AllocR = 0b0110,
            WriteBack_Transient_AllocW = 0b0101,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],

        /// Attribute 2
        Attr2_Device OFFSET(16) NUMBITS(8) [
            nGnRnE = 0b00000000,
            nGnRE = 0b00000100,
            nGRE = 0b00001000,
            GRE = 0b00001100
        ],
        Attr2_Outer OFFSET(20) NUMBITS(4) [
            NonCacheable = 0b0100,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],
        Attr2_Inner OFFSET(16) NUMBITS(4) [
            WriteThrough_Transient_AllocRW = 0b0011,
            WriteThrough_Transient_AllocR = 0b0010,
            WriteThrough_Transient_AllocW = 0b0001,
            NonCacheable = 0b0100,
            WriteBack_Transient_AllocRW = 0b0111,
            WriteBack_Transient_AllocR = 0b0110,
            WriteBack_Transient_AllocW = 0b0101,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],


        /// Attribute 1
        Attr1_Device OFFSET(8) NUMBITS(8) [
            nGnRnE = 0b00000000,
            nGnRE = 0b00000100,
            nGRE = 0b00001000,
            GRE = 0b00001100
        ],
        Attr1_Outer OFFSET(12) NUMBITS(4) [
            NonCacheable = 0b0100,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],
        Attr1_Inner OFFSET(8) NUMBITS(4) [
            WriteThrough_Transient_AllocRW = 0b0011,
            WriteThrough_Transient_AllocR = 0b0010,
            WriteThrough_Transient_AllocW = 0b0001,
            NonCacheable = 0b0100,
            WriteBack_Transient_AllocRW = 0b0111,
            WriteBack_Transient_AllocR = 0b0110,
            WriteBack_Transient_AllocW = 0b0101,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],

        /// Attribute 0
        Attr0_Device OFFSET(0) NUMBITS(8) [
            nGnRnE = 0b00000000,
            nGnRE = 0b00000100,
            nGRE = 0b00001000,
            GRE = 0b00001100
        ],
        Attr0_Outer OFFSET(4) NUMBITS(4) [
            NonCacheable = 0b0100,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ],
        Attr0_Inner OFFSET(0) NUMBITS(4) [
            WriteThrough_Transient_AllocRW = 0b0011,
            WriteThrough_Transient_AllocR = 0b0010,
            WriteThrough_Transient_AllocW = 0b0001,
            NonCacheable = 0b0100,
            WriteBack_Transient_AllocRW = 0b0111,
            WriteBack_Transient_AllocR = 0b0110,
            WriteBack_Transient_AllocW = 0b0101,
            WriteThrough_NonTransient_AllocRW = 0b1011,
            WriteThrough_NonTransient_AllocR = 0b1010,
            WriteThrough_NonTransient_AllocW = 0b1001,
            WriteBack_NonTransient_AllocRW = 0b1111,
            WriteBack_NonTransient_AllocR = 0b1110,
            WriteBack_NonTransient_AllocW = 0b1101
        ]
    ]
}

pub struct Reg;

impl RegisterReadWrite<u64, MAIR_EL1::Register> for Reg {
    sys_coproc_read_raw!(u64, "MAIR_EL1");
    sys_coproc_write_raw!(u64, "MAIR_EL1");
}

pub static MAIR_EL1: Reg = Reg {};
