use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

register_bitfields! {
    u32,

    GPFSEL1 [
        FSEL14 OFFSET(12) NUMBITS(3) [
            Alt5 = 0b010,
        ],
        FSEL15 OFFSET(15) NUMBITS(3) [
            Alt5 = 0b010,
        ],
    ],
    GPPUPPDN0 [
        PUPPDN14 OFFSET(28) NUMBITS(2) [
            None = 0b00,
        ],
        PUPPDN15 OFFSET(30) NUMBITS(2) [
            None = 0b00,
        ],
    ],

    AUX_ENABLES [
        MINI_UART OFFSET(0) NUMBITS(1) [
            Enable = 1,
        ],
    ],
    AUX_MU_IER_REG [
        INT OFFSET(0) NUMBITS(1) [
            Disable = 0,
        ],
    ],
    AUX_MU_IIR_REG [
        FIFO OFFSET(1) NUMBITS(2) [
            ClearRxAndTx = 0b11,
        ],
    ],
    AUX_MU_LCR_REG [
        DATA_SIZE OFFSET(0) NUMBITS(2) [
            EightBits = 0b11,
        ],
    ],
    AUX_MU_MCR_REG [
        RTS OFFSET(1) NUMBITS(1) [
            High = 0,
        ]
    ],
    AUX_MU_LSR_REG [
        RX_READY OFFSET(0) NUMBITS(1) [],
        TX_READY OFFSET(5) NUMBITS(1) [],
    ],
    AUX_MU_CNTL_REG [
        RX OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],
        TX OFFSET(1) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],
    ],
}

register_structs! {
    #[allow(non_snake_case)]
    GPIO_Registers {
        (0x00 => _reserved1),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => _reserved2),
        (0xE4 => GPPUPPDN0: ReadWrite<u32, GPPUPPDN0::Register>),
        (0xE8 => @END),
    },

    #[allow(non_snake_case)]
    AUX_Registers {
        (0x00 => _reserved1),
        (0x04 => AUX_ENABLES: ReadWrite<u32, AUX_ENABLES::Register>),
        (0x08 => _reserved2),
        (0x40 => AUX_MU_IO_REG: ReadWrite<u32>),
        (0x44 => AUX_MU_IER_REG: ReadWrite<u32, AUX_MU_IER_REG::Register>),
        (0x48 => AUX_MU_IIR_REG: ReadWrite<u32, AUX_MU_IIR_REG::Register>),
        (0x4C => AUX_MU_LCR_REG: ReadWrite<u32, AUX_MU_LCR_REG::Register>),
        (0x50 => AUX_MU_MCR_REG: ReadWrite<u32, AUX_MU_MCR_REG::Register>),
        (0x54 => AUX_MU_LSR_REG: ReadWrite<u32, AUX_MU_LSR_REG::Register>),
        (0x58 => _reserved3),
        (0x60 => AUX_MU_CNTL_REG: ReadWrite<u32, AUX_MU_CNTL_REG::Register>),
        (0x64 => _reserved4),
        (0x68 => AUX_MU_BAUD_REG: ReadWrite<u32>),
        (0x6C => @END),
    }
}

const UART_BAUDRATE: usize = 115200;

const GPIO_REGS: *mut GPIO_Registers = 0xFE20_0000 as *mut _;
const AUX_REGS: *mut AUX_Registers = 0xFE21_5000 as *mut _;

#[rustfmt::skip]
pub unsafe fn init() {
    (*AUX_REGS).AUX_ENABLES.write(AUX_ENABLES::MINI_UART::Enable);
    (*AUX_REGS).AUX_MU_IER_REG.write(AUX_MU_IER_REG::INT::Disable);
    (*AUX_REGS).AUX_MU_CNTL_REG.write(AUX_MU_CNTL_REG::RX::Disable + AUX_MU_CNTL_REG::TX::Disable);
    (*AUX_REGS).AUX_MU_LCR_REG.write(AUX_MU_LCR_REG::DATA_SIZE::EightBits);
    (*AUX_REGS).AUX_MU_MCR_REG.write(AUX_MU_MCR_REG::RTS::High);
    (*AUX_REGS).AUX_MU_IIR_REG.write(AUX_MU_IIR_REG::FIFO::ClearRxAndTx);

    let divisor = 500000000 / (8 * UART_BAUDRATE) - 1;
    (*AUX_REGS).AUX_MU_BAUD_REG.set(divisor as u32);

    (*GPIO_REGS).GPFSEL1.write(GPFSEL1::FSEL14::Alt5 + GPFSEL1::FSEL15::Alt5);
    (*GPIO_REGS).GPPUPPDN0.write(GPPUPPDN0::PUPPDN14::None + GPPUPPDN0::PUPPDN15::None);

    (*AUX_REGS).AUX_MU_CNTL_REG.write(AUX_MU_CNTL_REG::RX::Enable + AUX_MU_CNTL_REG::TX::Enable);
}

pub unsafe fn read_byte() -> u8 {
    while (*AUX_REGS).AUX_MU_LSR_REG.read(AUX_MU_LSR_REG::RX_READY) == 0 {
        core::hint::spin_loop();
    }
    (*AUX_REGS).AUX_MU_IO_REG.get() as u8
}

pub unsafe fn write_byte(c: u8) {
    while (*AUX_REGS).AUX_MU_LSR_REG.read(AUX_MU_LSR_REG::TX_READY) == 0 {
        core::hint::spin_loop();
    }
    (*AUX_REGS).AUX_MU_IO_REG.set(c as u32);
}
