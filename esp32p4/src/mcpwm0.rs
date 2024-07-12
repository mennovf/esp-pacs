#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_cfg: CLK_CFG,
    timer: [TIMER; 3],
    timer_synci_cfg: TIMER_SYNCI_CFG,
    operator_timersel: OPERATOR_TIMERSEL,
    ch: [CH; 3],
    fault_detect: FAULT_DETECT,
    cap_timer_cfg: CAP_TIMER_CFG,
    cap_timer_phase: CAP_TIMER_PHASE,
    cap_ch_cfg: [CAP_CH_CFG; 3],
    cap_ch: [CAP_CH; 3],
    cap_status: CAP_STATUS,
    update_cfg: UPDATE_CFG,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_clr: INT_CLR,
    evt_en: EVT_EN,
    task_en: TASK_EN,
    evt_en2: EVT_EN2,
    op_tstmp_e1: (),
    _reserved20: [u8; 0x04],
    op_tstmp_e2: (),
    _reserved21: [u8; 0x14],
    clk: CLK,
    version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - PWM clock prescaler register."]
    #[inline(always)]
    pub const fn clk_cfg(&self) -> &CLK_CFG {
        &self.clk_cfg
    }
    #[doc = "0x04..0x34 - Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS"]
    #[inline(always)]
    pub const fn timer(&self, n: usize) -> &TIMER {
        &self.timer[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x34 - Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS"]
    #[inline(always)]
    pub fn timer_iter(&self) -> impl Iterator<Item = &TIMER> {
        self.timer.iter()
    }
    #[doc = "0x34 - Synchronization input selection register for PWM timers."]
    #[inline(always)]
    pub const fn timer_synci_cfg(&self) -> &TIMER_SYNCI_CFG {
        &self.timer_synci_cfg
    }
    #[doc = "0x38 - PWM operator's timer select register"]
    #[inline(always)]
    pub const fn operator_timersel(&self) -> &OPERATOR_TIMERSEL {
        &self.operator_timersel
    }
    #[doc = "0x3c..0xe4 - Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0xe4 - Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0xe4 - Fault detection configuration and status register"]
    #[inline(always)]
    pub const fn fault_detect(&self) -> &FAULT_DETECT {
        &self.fault_detect
    }
    #[doc = "0xe8 - Capture timer configuration register"]
    #[inline(always)]
    pub const fn cap_timer_cfg(&self) -> &CAP_TIMER_CFG {
        &self.cap_timer_cfg
    }
    #[doc = "0xec - Capture timer sync phase register"]
    #[inline(always)]
    pub const fn cap_timer_phase(&self) -> &CAP_TIMER_PHASE {
        &self.cap_timer_phase
    }
    #[doc = "0xf0..0xfc - Capture channel %s configuration register"]
    #[inline(always)]
    pub const fn cap_ch_cfg(&self, n: usize) -> &CAP_CH_CFG {
        &self.cap_ch_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf0..0xfc - Capture channel %s configuration register"]
    #[inline(always)]
    pub fn cap_ch_cfg_iter(&self) -> impl Iterator<Item = &CAP_CH_CFG> {
        self.cap_ch_cfg.iter()
    }
    #[doc = "0xf0 - Capture channel 0 configuration register"]
    #[inline(always)]
    pub const fn cap_ch0_cfg(&self) -> &CAP_CH_CFG {
        self.cap_ch_cfg(0)
    }
    #[doc = "0xf4 - Capture channel 1 configuration register"]
    #[inline(always)]
    pub const fn cap_ch1_cfg(&self) -> &CAP_CH_CFG {
        self.cap_ch_cfg(1)
    }
    #[doc = "0xf8 - Capture channel 2 configuration register"]
    #[inline(always)]
    pub const fn cap_ch2_cfg(&self) -> &CAP_CH_CFG {
        self.cap_ch_cfg(2)
    }
    #[doc = "0xfc..0x108 - CAP%s capture value register"]
    #[inline(always)]
    pub const fn cap_ch(&self, n: usize) -> &CAP_CH {
        &self.cap_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xfc..0x108 - CAP%s capture value register"]
    #[inline(always)]
    pub fn cap_ch_iter(&self) -> impl Iterator<Item = &CAP_CH> {
        self.cap_ch.iter()
    }
    #[doc = "0x108 - Last capture trigger edge information register"]
    #[inline(always)]
    pub const fn cap_status(&self) -> &CAP_STATUS {
        &self.cap_status
    }
    #[doc = "0x10c - Generator Update configuration register"]
    #[inline(always)]
    pub const fn update_cfg(&self) -> &UPDATE_CFG {
        &self.update_cfg
    }
    #[doc = "0x110 - Interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x114 - Interrupt raw status register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x118 - Interrupt masked status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x11c - Interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x120 - Event enable register"]
    #[inline(always)]
    pub const fn evt_en(&self) -> &EVT_EN {
        &self.evt_en
    }
    #[doc = "0x124 - Task enable register"]
    #[inline(always)]
    pub const fn task_en(&self) -> &TASK_EN {
        &self.task_en
    }
    #[doc = "0x128 - Event enable register2"]
    #[inline(always)]
    pub const fn evt_en2(&self) -> &EVT_EN2 {
        &self.evt_en2
    }
    #[doc = "0x12c..0x138 - Generator%s timer stamp E1 value register"]
    #[inline(always)]
    pub const fn op_tstmp_e1(&self, n: usize) -> &OP_TSTMP_E1 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(300)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12c..0x138 - Generator%s timer stamp E1 value register"]
    #[inline(always)]
    pub fn op_tstmp_e1_iter(&self) -> impl Iterator<Item = &OP_TSTMP_E1> {
        (0..3).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(300)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x12c - Generator0 timer stamp E1 value register"]
    #[inline(always)]
    pub const fn op0_tstmp_e1(&self) -> &OP_TSTMP_E1 {
        self.op_tstmp_e1(0)
    }
    #[doc = "0x134 - Generator1 timer stamp E1 value register"]
    #[inline(always)]
    pub const fn op1_tstmp_e1(&self) -> &OP_TSTMP_E1 {
        self.op_tstmp_e1(1)
    }
    #[doc = "0x13c - Generator2 timer stamp E1 value register"]
    #[inline(always)]
    pub const fn op2_tstmp_e1(&self) -> &OP_TSTMP_E1 {
        self.op_tstmp_e1(2)
    }
    #[doc = "0x130..0x13c - Generator%s timer stamp E2 value register"]
    #[inline(always)]
    pub const fn op_tstmp_e2(&self, n: usize) -> &OP_TSTMP_E2 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(304)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x13c - Generator%s timer stamp E2 value register"]
    #[inline(always)]
    pub fn op_tstmp_e2_iter(&self) -> impl Iterator<Item = &OP_TSTMP_E2> {
        (0..3).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(304)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x130 - Generator0 timer stamp E2 value register"]
    #[inline(always)]
    pub const fn op0_tstmp_e2(&self) -> &OP_TSTMP_E2 {
        self.op_tstmp_e2(0)
    }
    #[doc = "0x138 - Generator1 timer stamp E2 value register"]
    #[inline(always)]
    pub const fn op1_tstmp_e2(&self) -> &OP_TSTMP_E2 {
        self.op_tstmp_e2(1)
    }
    #[doc = "0x140 - Generator2 timer stamp E2 value register"]
    #[inline(always)]
    pub const fn op2_tstmp_e2(&self) -> &OP_TSTMP_E2 {
        self.op_tstmp_e2(2)
    }
    #[doc = "0x144 - Global configuration register"]
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    #[doc = "0x148 - Version register."]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
#[doc = "CLK_CFG (rw) register accessor: PWM clock prescaler register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cfg`] module"]
pub type CLK_CFG = crate::Reg<clk_cfg::CLK_CFG_SPEC>;
#[doc = "PWM clock prescaler register."]
pub mod clk_cfg;
#[doc = "Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS"]
pub use self::timer::TIMER;
#[doc = r"Cluster"]
#[doc = "Cluster TIMER%s, containing TIMER?_CFG0, TIMER?_CFG1, TIMER?_SYNC, TIMER?_STATUS"]
pub mod timer;
#[doc = "TIMER_SYNCI_CFG (rw) register accessor: Synchronization input selection register for PWM timers.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_synci_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_synci_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_synci_cfg`] module"]
pub type TIMER_SYNCI_CFG = crate::Reg<timer_synci_cfg::TIMER_SYNCI_CFG_SPEC>;
#[doc = "Synchronization input selection register for PWM timers."]
pub mod timer_synci_cfg;
#[doc = "OPERATOR_TIMERSEL (rw) register accessor: PWM operator's timer select register\n\nYou can [`read`](crate::Reg::read) this register and get [`operator_timersel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`operator_timersel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@operator_timersel`] module"]
pub type OPERATOR_TIMERSEL = crate::Reg<operator_timersel::OPERATOR_TIMERSEL_SPEC>;
#[doc = "PWM operator's timer select register"]
pub mod operator_timersel;
#[doc = "Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS"]
pub mod ch;
#[doc = "FAULT_DETECT (rw) register accessor: Fault detection configuration and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_detect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_detect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_detect`] module"]
pub type FAULT_DETECT = crate::Reg<fault_detect::FAULT_DETECT_SPEC>;
#[doc = "Fault detection configuration and status register"]
pub mod fault_detect;
#[doc = "CAP_TIMER_CFG (rw) register accessor: Capture timer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_timer_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_timer_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_timer_cfg`] module"]
pub type CAP_TIMER_CFG = crate::Reg<cap_timer_cfg::CAP_TIMER_CFG_SPEC>;
#[doc = "Capture timer configuration register"]
pub mod cap_timer_cfg;
#[doc = "CAP_TIMER_PHASE (rw) register accessor: Capture timer sync phase register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_timer_phase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_timer_phase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_timer_phase`] module"]
pub type CAP_TIMER_PHASE = crate::Reg<cap_timer_phase::CAP_TIMER_PHASE_SPEC>;
#[doc = "Capture timer sync phase register"]
pub mod cap_timer_phase;
#[doc = "CAP_CH_CFG (rw) register accessor: Capture channel %s configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_ch_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_ch_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_ch_cfg`] module"]
pub type CAP_CH_CFG = crate::Reg<cap_ch_cfg::CAP_CH_CFG_SPEC>;
#[doc = "Capture channel %s configuration register"]
pub mod cap_ch_cfg;
#[doc = "CAP_CH (r) register accessor: CAP%s capture value register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_ch`] module"]
pub type CAP_CH = crate::Reg<cap_ch::CAP_CH_SPEC>;
#[doc = "CAP%s capture value register"]
pub mod cap_ch;
#[doc = "CAP_STATUS (r) register accessor: Last capture trigger edge information register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_status`] module"]
pub type CAP_STATUS = crate::Reg<cap_status::CAP_STATUS_SPEC>;
#[doc = "Last capture trigger edge information register"]
pub mod cap_status;
#[doc = "UPDATE_CFG (rw) register accessor: Generator Update configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`update_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`update_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@update_cfg`] module"]
pub type UPDATE_CFG = crate::Reg<update_cfg::UPDATE_CFG_SPEC>;
#[doc = "Generator Update configuration register"]
pub mod update_cfg;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable register"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: Interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Interrupt raw status register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Interrupt masked status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Interrupt masked status register"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod int_clr;
#[doc = "EVT_EN (rw) register accessor: Event enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_en`] module"]
pub type EVT_EN = crate::Reg<evt_en::EVT_EN_SPEC>;
#[doc = "Event enable register"]
pub mod evt_en;
#[doc = "TASK_EN (rw) register accessor: Task enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_en`] module"]
pub type TASK_EN = crate::Reg<task_en::TASK_EN_SPEC>;
#[doc = "Task enable register"]
pub mod task_en;
#[doc = "EVT_EN2 (rw) register accessor: Event enable register2\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_en2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_en2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_en2`] module"]
pub type EVT_EN2 = crate::Reg<evt_en2::EVT_EN2_SPEC>;
#[doc = "Event enable register2"]
pub mod evt_en2;
#[doc = "OP_TSTMP_E1 (rw) register accessor: Generator%s timer stamp E1 value register\n\nYou can [`read`](crate::Reg::read) this register and get [`op_tstmp_e1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op_tstmp_e1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op_tstmp_e1`] module"]
pub type OP_TSTMP_E1 = crate::Reg<op_tstmp_e1::OP_TSTMP_E1_SPEC>;
#[doc = "Generator%s timer stamp E1 value register"]
pub mod op_tstmp_e1;
#[doc = "OP_TSTMP_E2 (rw) register accessor: Generator%s timer stamp E2 value register\n\nYou can [`read`](crate::Reg::read) this register and get [`op_tstmp_e2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op_tstmp_e2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op_tstmp_e2`] module"]
pub type OP_TSTMP_E2 = crate::Reg<op_tstmp_e2::OP_TSTMP_E2_SPEC>;
#[doc = "Generator%s timer stamp E2 value register"]
pub mod op_tstmp_e2;
#[doc = "CLK (rw) register accessor: Global configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "Global configuration register"]
pub mod clk;
#[doc = "VERSION (rw) register accessor: Version register.\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version register."]
pub mod version;
