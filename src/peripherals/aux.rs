pub const AUX_BASE: usize = 0x3F215000;

pub const IRQ_STATUS_OFFSET: usize = 0x0;
pub const AUX_ENABLES_OFFSET: usize = 0x04;
pub const AUX_MU_IO_REG_OFFSET: usize = 0x40;
pub const AUX_MU_IER_REG_OFFSET: usize = 0x44;
pub const AUX_MU_IIR_REG_OFFSET: usize = 0x48;
pub const AUX_MU_LCR_REG_OFFSET: usize = 0x4C;
pub const AUX_MU_MCR_REG_OFFSET: usize = 0x50;
pub const AUX_MU_LSR_REG_OFFSET: usize = 0x54;
pub const AUX_MU_MSR_REG_OFFSET: usize = 0x58;
pub const AUX_MU_SCRATCH_OFFSET: usize = 0x5C;
pub const AUX_MU_CNTL_REG_OFFSET: usize = 0x60;
pub const AUX_MU_STAT_REG_OFFSET: usize = 0x64;
pub const AUX_MU_BAUD_REG_OFFSET: usize = 0x68;
