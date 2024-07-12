#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    sar_reader1_ctrl: SAR_READER1_CTRL,
    sar_reader1_status: SAR_READER1_STATUS,
    sar_meas1_ctrl1: SAR_MEAS1_CTRL1,
    sar_meas1_ctrl2: SAR_MEAS1_CTRL2,
    sar_meas1_mux: SAR_MEAS1_MUX,
    sar_atten1: SAR_ATTEN1,
    sar_amp_ctrl1: SAR_AMP_CTRL1,
    sar_amp_ctrl2: SAR_AMP_CTRL2,
    sar_amp_ctrl3: SAR_AMP_CTRL3,
    sar_reader2_ctrl: SAR_READER2_CTRL,
    sar_reader2_status: SAR_READER2_STATUS,
    sar_meas2_ctrl1: SAR_MEAS2_CTRL1,
    sar_meas2_ctrl2: SAR_MEAS2_CTRL2,
    sar_meas2_mux: SAR_MEAS2_MUX,
    sar_atten2: SAR_ATTEN2,
    sar_power_xpd_sar: SAR_POWER_XPD_SAR,
    sar_slave_addr1: SAR_SLAVE_ADDR1,
    sar_slave_addr2: SAR_SLAVE_ADDR2,
    sar_slave_addr3: SAR_SLAVE_ADDR3,
    sar_slave_addr4: SAR_SLAVE_ADDR4,
    sar_tsens_ctrl: SAR_TSENS_CTRL,
    sar_tsens_ctrl2: SAR_TSENS_CTRL2,
    sar_i2c_ctrl: SAR_I2C_CTRL,
    sar_touch_conf: SAR_TOUCH_CONF,
    sar_touch_thres1: SAR_TOUCH_THRES1,
    sar_touch_thres2: SAR_TOUCH_THRES2,
    sar_touch_thres3: SAR_TOUCH_THRES3,
    sar_touch_thres4: SAR_TOUCH_THRES4,
    sar_touch_thres5: SAR_TOUCH_THRES5,
    sar_touch_thres6: SAR_TOUCH_THRES6,
    sar_touch_thres7: SAR_TOUCH_THRES7,
    sar_touch_thres8: SAR_TOUCH_THRES8,
    sar_touch_thres9: SAR_TOUCH_THRES9,
    sar_touch_thres10: SAR_TOUCH_THRES10,
    sar_touch_thres11: SAR_TOUCH_THRES11,
    sar_touch_thres12: SAR_TOUCH_THRES12,
    sar_touch_thres13: SAR_TOUCH_THRES13,
    sar_touch_thres14: SAR_TOUCH_THRES14,
    _reserved38: [u8; 0x3c],
    sar_touch_chn_st: SAR_TOUCH_CHN_ST,
    sar_touch_status0: SAR_TOUCH_STATUS0,
    sar_touch_status1: SAR_TOUCH_STATUS1,
    sar_touch_status2: SAR_TOUCH_STATUS2,
    sar_touch_status3: SAR_TOUCH_STATUS3,
    sar_touch_status4: SAR_TOUCH_STATUS4,
    sar_touch_status5: SAR_TOUCH_STATUS5,
    sar_touch_status6: SAR_TOUCH_STATUS6,
    sar_touch_status7: SAR_TOUCH_STATUS7,
    sar_touch_status8: SAR_TOUCH_STATUS8,
    sar_touch_status9: SAR_TOUCH_STATUS9,
    sar_touch_status10: SAR_TOUCH_STATUS10,
    sar_touch_status11: SAR_TOUCH_STATUS11,
    sar_touch_status12: SAR_TOUCH_STATUS12,
    sar_touch_status13: SAR_TOUCH_STATUS13,
    sar_touch_status14: SAR_TOUCH_STATUS14,
    sar_touch_status15: SAR_TOUCH_STATUS15,
    sar_touch_status16: SAR_TOUCH_STATUS16,
    sar_dac_ctrl1: SAR_DAC_CTRL1,
    sar_dac_ctrl2: SAR_DAC_CTRL2,
    sar_cocpu_state: SAR_COCPU_STATE,
    sar_cocpu_int_raw: SAR_COCPU_INT_RAW,
    sar_cocpu_int_ena: SAR_COCPU_INT_ENA,
    sar_cocpu_int_st: SAR_COCPU_INT_ST,
    sar_cocpu_int_clr: SAR_COCPU_INT_CLR,
    sar_cocpu_debug: SAR_COCPU_DEBUG,
    sar_hall_ctrl: SAR_HALL_CTRL,
    sar_nouse: SAR_NOUSE,
    sar_io_mux_conf: SAR_IO_MUX_CONF,
    sardate: SARDATE,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC ADC1 data and sampling control"]
    #[inline(always)]
    pub const fn sar_reader1_ctrl(&self) -> &SAR_READER1_CTRL {
        &self.sar_reader1_ctrl
    }
    #[doc = "0x04 - saradc1 status for debug"]
    #[inline(always)]
    pub const fn sar_reader1_status(&self) -> &SAR_READER1_STATUS {
        &self.sar_reader1_status
    }
    #[doc = "0x08 - Configure RTC ADC1 controller"]
    #[inline(always)]
    pub const fn sar_meas1_ctrl1(&self) -> &SAR_MEAS1_CTRL1 {
        &self.sar_meas1_ctrl1
    }
    #[doc = "0x0c - Control RTC ADC1 conversion and status"]
    #[inline(always)]
    pub const fn sar_meas1_ctrl2(&self) -> &SAR_MEAS1_CTRL2 {
        &self.sar_meas1_ctrl2
    }
    #[doc = "0x10 - Select the controller for SAR ADC1"]
    #[inline(always)]
    pub const fn sar_meas1_mux(&self) -> &SAR_MEAS1_MUX {
        &self.sar_meas1_mux
    }
    #[doc = "0x14 - Configure SAR ADC1 attenuation"]
    #[inline(always)]
    pub const fn sar_atten1(&self) -> &SAR_ATTEN1 {
        &self.sar_atten1
    }
    #[doc = "0x18 - AMP control"]
    #[inline(always)]
    pub const fn sar_amp_ctrl1(&self) -> &SAR_AMP_CTRL1 {
        &self.sar_amp_ctrl1
    }
    #[doc = "0x1c - AMP control"]
    #[inline(always)]
    pub const fn sar_amp_ctrl2(&self) -> &SAR_AMP_CTRL2 {
        &self.sar_amp_ctrl2
    }
    #[doc = "0x20 - AMP control register"]
    #[inline(always)]
    pub const fn sar_amp_ctrl3(&self) -> &SAR_AMP_CTRL3 {
        &self.sar_amp_ctrl3
    }
    #[doc = "0x24 - RTC ADC2 data and sampling control"]
    #[inline(always)]
    pub const fn sar_reader2_ctrl(&self) -> &SAR_READER2_CTRL {
        &self.sar_reader2_ctrl
    }
    #[doc = "0x28 - saradc2 status for debug"]
    #[inline(always)]
    pub const fn sar_reader2_status(&self) -> &SAR_READER2_STATUS {
        &self.sar_reader2_status
    }
    #[doc = "0x2c - configure rtc saradc2"]
    #[inline(always)]
    pub const fn sar_meas2_ctrl1(&self) -> &SAR_MEAS2_CTRL1 {
        &self.sar_meas2_ctrl1
    }
    #[doc = "0x30 - Control RTC ADC2 conversion and status"]
    #[inline(always)]
    pub const fn sar_meas2_ctrl2(&self) -> &SAR_MEAS2_CTRL2 {
        &self.sar_meas2_ctrl2
    }
    #[doc = "0x34 - Select the controller for SAR ADC2"]
    #[inline(always)]
    pub const fn sar_meas2_mux(&self) -> &SAR_MEAS2_MUX {
        &self.sar_meas2_mux
    }
    #[doc = "0x38 - Configure SAR ADC2 attenuation"]
    #[inline(always)]
    pub const fn sar_atten2(&self) -> &SAR_ATTEN2 {
        &self.sar_atten2
    }
    #[doc = "0x3c - configure saradc’s power by sw"]
    #[inline(always)]
    pub const fn sar_power_xpd_sar(&self) -> &SAR_POWER_XPD_SAR {
        &self.sar_power_xpd_sar
    }
    #[doc = "0x40 - Configure slave addresses 0-1 of RTC I2C"]
    #[inline(always)]
    pub const fn sar_slave_addr1(&self) -> &SAR_SLAVE_ADDR1 {
        &self.sar_slave_addr1
    }
    #[doc = "0x44 - Configure slave addresses 2-3 of RTC I2C"]
    #[inline(always)]
    pub const fn sar_slave_addr2(&self) -> &SAR_SLAVE_ADDR2 {
        &self.sar_slave_addr2
    }
    #[doc = "0x48 - Configure slave addresses 4-5 of RTC I2C"]
    #[inline(always)]
    pub const fn sar_slave_addr3(&self) -> &SAR_SLAVE_ADDR3 {
        &self.sar_slave_addr3
    }
    #[doc = "0x4c - Configure slave addresses 6-7 of RTC I2C"]
    #[inline(always)]
    pub const fn sar_slave_addr4(&self) -> &SAR_SLAVE_ADDR4 {
        &self.sar_slave_addr4
    }
    #[doc = "0x50 - Temperature sensor data control"]
    #[inline(always)]
    pub const fn sar_tsens_ctrl(&self) -> &SAR_TSENS_CTRL {
        &self.sar_tsens_ctrl
    }
    #[doc = "0x54 - Temperature sensor control"]
    #[inline(always)]
    pub const fn sar_tsens_ctrl2(&self) -> &SAR_TSENS_CTRL2 {
        &self.sar_tsens_ctrl2
    }
    #[doc = "0x58 - Configure RTC I2C transmission"]
    #[inline(always)]
    pub const fn sar_i2c_ctrl(&self) -> &SAR_I2C_CTRL {
        &self.sar_i2c_ctrl
    }
    #[doc = "0x5c - Touch sensor configuration register"]
    #[inline(always)]
    pub const fn sar_touch_conf(&self) -> &SAR_TOUCH_CONF {
        &self.sar_touch_conf
    }
    #[doc = "0x60 - Finger threshold for touch pad 1"]
    #[inline(always)]
    pub const fn sar_touch_thres1(&self) -> &SAR_TOUCH_THRES1 {
        &self.sar_touch_thres1
    }
    #[doc = "0x64 - Finger threshold for touch pad 2"]
    #[inline(always)]
    pub const fn sar_touch_thres2(&self) -> &SAR_TOUCH_THRES2 {
        &self.sar_touch_thres2
    }
    #[doc = "0x68 - Finger threshold for touch pad 3"]
    #[inline(always)]
    pub const fn sar_touch_thres3(&self) -> &SAR_TOUCH_THRES3 {
        &self.sar_touch_thres3
    }
    #[doc = "0x6c - Finger threshold for touch pad 4"]
    #[inline(always)]
    pub const fn sar_touch_thres4(&self) -> &SAR_TOUCH_THRES4 {
        &self.sar_touch_thres4
    }
    #[doc = "0x70 - Finger threshold for touch pad 5"]
    #[inline(always)]
    pub const fn sar_touch_thres5(&self) -> &SAR_TOUCH_THRES5 {
        &self.sar_touch_thres5
    }
    #[doc = "0x74 - Finger threshold for touch pad 6"]
    #[inline(always)]
    pub const fn sar_touch_thres6(&self) -> &SAR_TOUCH_THRES6 {
        &self.sar_touch_thres6
    }
    #[doc = "0x78 - Finger threshold for touch pad 7"]
    #[inline(always)]
    pub const fn sar_touch_thres7(&self) -> &SAR_TOUCH_THRES7 {
        &self.sar_touch_thres7
    }
    #[doc = "0x7c - Finger threshold for touch pad 8"]
    #[inline(always)]
    pub const fn sar_touch_thres8(&self) -> &SAR_TOUCH_THRES8 {
        &self.sar_touch_thres8
    }
    #[doc = "0x80 - Finger threshold for touch pad 9"]
    #[inline(always)]
    pub const fn sar_touch_thres9(&self) -> &SAR_TOUCH_THRES9 {
        &self.sar_touch_thres9
    }
    #[doc = "0x84 - Finger threshold for touch pad 10"]
    #[inline(always)]
    pub const fn sar_touch_thres10(&self) -> &SAR_TOUCH_THRES10 {
        &self.sar_touch_thres10
    }
    #[doc = "0x88 - Finger threshold for touch pad 11"]
    #[inline(always)]
    pub const fn sar_touch_thres11(&self) -> &SAR_TOUCH_THRES11 {
        &self.sar_touch_thres11
    }
    #[doc = "0x8c - Finger threshold for touch pad 12"]
    #[inline(always)]
    pub const fn sar_touch_thres12(&self) -> &SAR_TOUCH_THRES12 {
        &self.sar_touch_thres12
    }
    #[doc = "0x90 - Finger threshold for touch pad 13"]
    #[inline(always)]
    pub const fn sar_touch_thres13(&self) -> &SAR_TOUCH_THRES13 {
        &self.sar_touch_thres13
    }
    #[doc = "0x94 - Finger threshold for touch pad 14"]
    #[inline(always)]
    pub const fn sar_touch_thres14(&self) -> &SAR_TOUCH_THRES14 {
        &self.sar_touch_thres14
    }
    #[doc = "0xd4 - Touch channel status register"]
    #[inline(always)]
    pub const fn sar_touch_chn_st(&self) -> &SAR_TOUCH_CHN_ST {
        &self.sar_touch_chn_st
    }
    #[doc = "0xd8 - Status of touch controller"]
    #[inline(always)]
    pub const fn sar_touch_status0(&self) -> &SAR_TOUCH_STATUS0 {
        &self.sar_touch_status0
    }
    #[doc = "0xdc - Touch pad 1 status"]
    #[inline(always)]
    pub const fn sar_touch_status1(&self) -> &SAR_TOUCH_STATUS1 {
        &self.sar_touch_status1
    }
    #[doc = "0xe0 - Touch pad 2 status"]
    #[inline(always)]
    pub const fn sar_touch_status2(&self) -> &SAR_TOUCH_STATUS2 {
        &self.sar_touch_status2
    }
    #[doc = "0xe4 - Touch pad 3 status"]
    #[inline(always)]
    pub const fn sar_touch_status3(&self) -> &SAR_TOUCH_STATUS3 {
        &self.sar_touch_status3
    }
    #[doc = "0xe8 - Touch pad 4 status"]
    #[inline(always)]
    pub const fn sar_touch_status4(&self) -> &SAR_TOUCH_STATUS4 {
        &self.sar_touch_status4
    }
    #[doc = "0xec - Touch pad 5 status"]
    #[inline(always)]
    pub const fn sar_touch_status5(&self) -> &SAR_TOUCH_STATUS5 {
        &self.sar_touch_status5
    }
    #[doc = "0xf0 - Touch pad 6 status"]
    #[inline(always)]
    pub const fn sar_touch_status6(&self) -> &SAR_TOUCH_STATUS6 {
        &self.sar_touch_status6
    }
    #[doc = "0xf4 - Touch pad 7 status"]
    #[inline(always)]
    pub const fn sar_touch_status7(&self) -> &SAR_TOUCH_STATUS7 {
        &self.sar_touch_status7
    }
    #[doc = "0xf8 - Touch pad 8 status"]
    #[inline(always)]
    pub const fn sar_touch_status8(&self) -> &SAR_TOUCH_STATUS8 {
        &self.sar_touch_status8
    }
    #[doc = "0xfc - Touch pad 9 status"]
    #[inline(always)]
    pub const fn sar_touch_status9(&self) -> &SAR_TOUCH_STATUS9 {
        &self.sar_touch_status9
    }
    #[doc = "0x100 - Touch pad 10 status"]
    #[inline(always)]
    pub const fn sar_touch_status10(&self) -> &SAR_TOUCH_STATUS10 {
        &self.sar_touch_status10
    }
    #[doc = "0x104 - Touch pad 11 status"]
    #[inline(always)]
    pub const fn sar_touch_status11(&self) -> &SAR_TOUCH_STATUS11 {
        &self.sar_touch_status11
    }
    #[doc = "0x108 - Touch pad 12 status"]
    #[inline(always)]
    pub const fn sar_touch_status12(&self) -> &SAR_TOUCH_STATUS12 {
        &self.sar_touch_status12
    }
    #[doc = "0x10c - Touch pad 13 status"]
    #[inline(always)]
    pub const fn sar_touch_status13(&self) -> &SAR_TOUCH_STATUS13 {
        &self.sar_touch_status13
    }
    #[doc = "0x110 - Touch pad 14 status"]
    #[inline(always)]
    pub const fn sar_touch_status14(&self) -> &SAR_TOUCH_STATUS14 {
        &self.sar_touch_status14
    }
    #[doc = "0x114 - Touch sleep pad status"]
    #[inline(always)]
    pub const fn sar_touch_status15(&self) -> &SAR_TOUCH_STATUS15 {
        &self.sar_touch_status15
    }
    #[doc = "0x118 - Touch approach count status"]
    #[inline(always)]
    pub const fn sar_touch_status16(&self) -> &SAR_TOUCH_STATUS16 {
        &self.sar_touch_status16
    }
    #[doc = "0x11c - DAC control"]
    #[inline(always)]
    pub const fn sar_dac_ctrl1(&self) -> &SAR_DAC_CTRL1 {
        &self.sar_dac_ctrl1
    }
    #[doc = "0x120 - DAC output control"]
    #[inline(always)]
    pub const fn sar_dac_ctrl2(&self) -> &SAR_DAC_CTRL2 {
        &self.sar_dac_ctrl2
    }
    #[doc = "0x124 - ULP-RISCV status"]
    #[inline(always)]
    pub const fn sar_cocpu_state(&self) -> &SAR_COCPU_STATE {
        &self.sar_cocpu_state
    }
    #[doc = "0x128 - Interrupt raw bit of ULP-RISCV"]
    #[inline(always)]
    pub const fn sar_cocpu_int_raw(&self) -> &SAR_COCPU_INT_RAW {
        &self.sar_cocpu_int_raw
    }
    #[doc = "0x12c - Interrupt enable bit of ULP-RISCV"]
    #[inline(always)]
    pub const fn sar_cocpu_int_ena(&self) -> &SAR_COCPU_INT_ENA {
        &self.sar_cocpu_int_ena
    }
    #[doc = "0x130 - Interrupt status bit of ULP-RISCV"]
    #[inline(always)]
    pub const fn sar_cocpu_int_st(&self) -> &SAR_COCPU_INT_ST {
        &self.sar_cocpu_int_st
    }
    #[doc = "0x134 - Interrupt clear bit of ULP-RISCV"]
    #[inline(always)]
    pub const fn sar_cocpu_int_clr(&self) -> &SAR_COCPU_INT_CLR {
        &self.sar_cocpu_int_clr
    }
    #[doc = "0x138 - ULP-RISCV debug register"]
    #[inline(always)]
    pub const fn sar_cocpu_debug(&self) -> &SAR_COCPU_DEBUG {
        &self.sar_cocpu_debug
    }
    #[doc = "0x13c - hall control"]
    #[inline(always)]
    pub const fn sar_hall_ctrl(&self) -> &SAR_HALL_CTRL {
        &self.sar_hall_ctrl
    }
    #[doc = "0x140 - sar nouse"]
    #[inline(always)]
    pub const fn sar_nouse(&self) -> &SAR_NOUSE {
        &self.sar_nouse
    }
    #[doc = "0x144 - Configure and reset IO MUX"]
    #[inline(always)]
    pub const fn sar_io_mux_conf(&self) -> &SAR_IO_MUX_CONF {
        &self.sar_io_mux_conf
    }
    #[doc = "0x148 - Version Control Register"]
    #[inline(always)]
    pub const fn sardate(&self) -> &SARDATE {
        &self.sardate
    }
}
#[doc = "SAR_READER1_CTRL (rw) register accessor: RTC ADC1 data and sampling control\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_reader1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_reader1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_reader1_ctrl`] module"]
pub type SAR_READER1_CTRL = crate::Reg<sar_reader1_ctrl::SAR_READER1_CTRL_SPEC>;
#[doc = "RTC ADC1 data and sampling control"]
pub mod sar_reader1_ctrl;
#[doc = "SAR_READER1_STATUS (r) register accessor: saradc1 status for debug\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_reader1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_reader1_status`] module"]
pub type SAR_READER1_STATUS = crate::Reg<sar_reader1_status::SAR_READER1_STATUS_SPEC>;
#[doc = "saradc1 status for debug"]
pub mod sar_reader1_status;
#[doc = "SAR_MEAS1_CTRL1 (rw) register accessor: Configure RTC ADC1 controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas1_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas1_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas1_ctrl1`] module"]
pub type SAR_MEAS1_CTRL1 = crate::Reg<sar_meas1_ctrl1::SAR_MEAS1_CTRL1_SPEC>;
#[doc = "Configure RTC ADC1 controller"]
pub mod sar_meas1_ctrl1;
#[doc = "SAR_MEAS1_CTRL2 (rw) register accessor: Control RTC ADC1 conversion and status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas1_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas1_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas1_ctrl2`] module"]
pub type SAR_MEAS1_CTRL2 = crate::Reg<sar_meas1_ctrl2::SAR_MEAS1_CTRL2_SPEC>;
#[doc = "Control RTC ADC1 conversion and status"]
pub mod sar_meas1_ctrl2;
#[doc = "SAR_MEAS1_MUX (rw) register accessor: Select the controller for SAR ADC1\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas1_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas1_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas1_mux`] module"]
pub type SAR_MEAS1_MUX = crate::Reg<sar_meas1_mux::SAR_MEAS1_MUX_SPEC>;
#[doc = "Select the controller for SAR ADC1"]
pub mod sar_meas1_mux;
#[doc = "SAR_ATTEN1 (rw) register accessor: Configure SAR ADC1 attenuation\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_atten1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_atten1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_atten1`] module"]
pub type SAR_ATTEN1 = crate::Reg<sar_atten1::SAR_ATTEN1_SPEC>;
#[doc = "Configure SAR ADC1 attenuation"]
pub mod sar_atten1;
#[doc = "SAR_AMP_CTRL1 (rw) register accessor: AMP control\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_amp_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_amp_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_amp_ctrl1`] module"]
pub type SAR_AMP_CTRL1 = crate::Reg<sar_amp_ctrl1::SAR_AMP_CTRL1_SPEC>;
#[doc = "AMP control"]
pub mod sar_amp_ctrl1;
#[doc = "SAR_AMP_CTRL2 (rw) register accessor: AMP control\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_amp_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_amp_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_amp_ctrl2`] module"]
pub type SAR_AMP_CTRL2 = crate::Reg<sar_amp_ctrl2::SAR_AMP_CTRL2_SPEC>;
#[doc = "AMP control"]
pub mod sar_amp_ctrl2;
#[doc = "SAR_AMP_CTRL3 (rw) register accessor: AMP control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_amp_ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_amp_ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_amp_ctrl3`] module"]
pub type SAR_AMP_CTRL3 = crate::Reg<sar_amp_ctrl3::SAR_AMP_CTRL3_SPEC>;
#[doc = "AMP control register"]
pub mod sar_amp_ctrl3;
#[doc = "SAR_READER2_CTRL (rw) register accessor: RTC ADC2 data and sampling control\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_reader2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_reader2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_reader2_ctrl`] module"]
pub type SAR_READER2_CTRL = crate::Reg<sar_reader2_ctrl::SAR_READER2_CTRL_SPEC>;
#[doc = "RTC ADC2 data and sampling control"]
pub mod sar_reader2_ctrl;
#[doc = "SAR_READER2_STATUS (r) register accessor: saradc2 status for debug\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_reader2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_reader2_status`] module"]
pub type SAR_READER2_STATUS = crate::Reg<sar_reader2_status::SAR_READER2_STATUS_SPEC>;
#[doc = "saradc2 status for debug"]
pub mod sar_reader2_status;
#[doc = "SAR_MEAS2_CTRL1 (rw) register accessor: configure rtc saradc2\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas2_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas2_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas2_ctrl1`] module"]
pub type SAR_MEAS2_CTRL1 = crate::Reg<sar_meas2_ctrl1::SAR_MEAS2_CTRL1_SPEC>;
#[doc = "configure rtc saradc2"]
pub mod sar_meas2_ctrl1;
#[doc = "SAR_MEAS2_CTRL2 (rw) register accessor: Control RTC ADC2 conversion and status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas2_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas2_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas2_ctrl2`] module"]
pub type SAR_MEAS2_CTRL2 = crate::Reg<sar_meas2_ctrl2::SAR_MEAS2_CTRL2_SPEC>;
#[doc = "Control RTC ADC2 conversion and status"]
pub mod sar_meas2_ctrl2;
#[doc = "SAR_MEAS2_MUX (rw) register accessor: Select the controller for SAR ADC2\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_meas2_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_meas2_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_meas2_mux`] module"]
pub type SAR_MEAS2_MUX = crate::Reg<sar_meas2_mux::SAR_MEAS2_MUX_SPEC>;
#[doc = "Select the controller for SAR ADC2"]
pub mod sar_meas2_mux;
#[doc = "SAR_ATTEN2 (rw) register accessor: Configure SAR ADC2 attenuation\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_atten2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_atten2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_atten2`] module"]
pub type SAR_ATTEN2 = crate::Reg<sar_atten2::SAR_ATTEN2_SPEC>;
#[doc = "Configure SAR ADC2 attenuation"]
pub mod sar_atten2;
#[doc = "SAR_POWER_XPD_SAR (rw) register accessor: configure saradc’s power by sw\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_power_xpd_sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_power_xpd_sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_power_xpd_sar`] module"]
pub type SAR_POWER_XPD_SAR = crate::Reg<sar_power_xpd_sar::SAR_POWER_XPD_SAR_SPEC>;
#[doc = "configure saradc’s power by sw"]
pub mod sar_power_xpd_sar;
#[doc = "SAR_SLAVE_ADDR1 (rw) register accessor: Configure slave addresses 0-1 of RTC I2C\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr1`] module"]
pub type SAR_SLAVE_ADDR1 = crate::Reg<sar_slave_addr1::SAR_SLAVE_ADDR1_SPEC>;
#[doc = "Configure slave addresses 0-1 of RTC I2C"]
pub mod sar_slave_addr1;
#[doc = "SAR_SLAVE_ADDR2 (rw) register accessor: Configure slave addresses 2-3 of RTC I2C\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr2`] module"]
pub type SAR_SLAVE_ADDR2 = crate::Reg<sar_slave_addr2::SAR_SLAVE_ADDR2_SPEC>;
#[doc = "Configure slave addresses 2-3 of RTC I2C"]
pub mod sar_slave_addr2;
#[doc = "SAR_SLAVE_ADDR3 (rw) register accessor: Configure slave addresses 4-5 of RTC I2C\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr3`] module"]
pub type SAR_SLAVE_ADDR3 = crate::Reg<sar_slave_addr3::SAR_SLAVE_ADDR3_SPEC>;
#[doc = "Configure slave addresses 4-5 of RTC I2C"]
pub mod sar_slave_addr3;
#[doc = "SAR_SLAVE_ADDR4 (rw) register accessor: Configure slave addresses 6-7 of RTC I2C\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_slave_addr4`] module"]
pub type SAR_SLAVE_ADDR4 = crate::Reg<sar_slave_addr4::SAR_SLAVE_ADDR4_SPEC>;
#[doc = "Configure slave addresses 6-7 of RTC I2C"]
pub mod sar_slave_addr4;
#[doc = "SAR_TSENS_CTRL (rw) register accessor: Temperature sensor data control\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_tsens_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_tsens_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_tsens_ctrl`] module"]
pub type SAR_TSENS_CTRL = crate::Reg<sar_tsens_ctrl::SAR_TSENS_CTRL_SPEC>;
#[doc = "Temperature sensor data control"]
pub mod sar_tsens_ctrl;
#[doc = "SAR_TSENS_CTRL2 (rw) register accessor: Temperature sensor control\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_tsens_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_tsens_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_tsens_ctrl2`] module"]
pub type SAR_TSENS_CTRL2 = crate::Reg<sar_tsens_ctrl2::SAR_TSENS_CTRL2_SPEC>;
#[doc = "Temperature sensor control"]
pub mod sar_tsens_ctrl2;
#[doc = "SAR_I2C_CTRL (rw) register accessor: Configure RTC I2C transmission\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_i2c_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_i2c_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_i2c_ctrl`] module"]
pub type SAR_I2C_CTRL = crate::Reg<sar_i2c_ctrl::SAR_I2C_CTRL_SPEC>;
#[doc = "Configure RTC I2C transmission"]
pub mod sar_i2c_ctrl;
#[doc = "SAR_TOUCH_CONF (rw) register accessor: Touch sensor configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_conf`] module"]
pub type SAR_TOUCH_CONF = crate::Reg<sar_touch_conf::SAR_TOUCH_CONF_SPEC>;
#[doc = "Touch sensor configuration register"]
pub mod sar_touch_conf;
#[doc = "SAR_TOUCH_THRES1 (rw) register accessor: Finger threshold for touch pad 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres1`] module"]
pub type SAR_TOUCH_THRES1 = crate::Reg<sar_touch_thres1::SAR_TOUCH_THRES1_SPEC>;
#[doc = "Finger threshold for touch pad 1"]
pub mod sar_touch_thres1;
#[doc = "SAR_TOUCH_THRES2 (rw) register accessor: Finger threshold for touch pad 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres2`] module"]
pub type SAR_TOUCH_THRES2 = crate::Reg<sar_touch_thres2::SAR_TOUCH_THRES2_SPEC>;
#[doc = "Finger threshold for touch pad 2"]
pub mod sar_touch_thres2;
#[doc = "SAR_TOUCH_THRES3 (rw) register accessor: Finger threshold for touch pad 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres3`] module"]
pub type SAR_TOUCH_THRES3 = crate::Reg<sar_touch_thres3::SAR_TOUCH_THRES3_SPEC>;
#[doc = "Finger threshold for touch pad 3"]
pub mod sar_touch_thres3;
#[doc = "SAR_TOUCH_THRES4 (rw) register accessor: Finger threshold for touch pad 4\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres4`] module"]
pub type SAR_TOUCH_THRES4 = crate::Reg<sar_touch_thres4::SAR_TOUCH_THRES4_SPEC>;
#[doc = "Finger threshold for touch pad 4"]
pub mod sar_touch_thres4;
#[doc = "SAR_TOUCH_THRES5 (rw) register accessor: Finger threshold for touch pad 5\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres5`] module"]
pub type SAR_TOUCH_THRES5 = crate::Reg<sar_touch_thres5::SAR_TOUCH_THRES5_SPEC>;
#[doc = "Finger threshold for touch pad 5"]
pub mod sar_touch_thres5;
#[doc = "SAR_TOUCH_THRES6 (rw) register accessor: Finger threshold for touch pad 6\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres6`] module"]
pub type SAR_TOUCH_THRES6 = crate::Reg<sar_touch_thres6::SAR_TOUCH_THRES6_SPEC>;
#[doc = "Finger threshold for touch pad 6"]
pub mod sar_touch_thres6;
#[doc = "SAR_TOUCH_THRES7 (rw) register accessor: Finger threshold for touch pad 7\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres7`] module"]
pub type SAR_TOUCH_THRES7 = crate::Reg<sar_touch_thres7::SAR_TOUCH_THRES7_SPEC>;
#[doc = "Finger threshold for touch pad 7"]
pub mod sar_touch_thres7;
#[doc = "SAR_TOUCH_THRES8 (rw) register accessor: Finger threshold for touch pad 8\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres8`] module"]
pub type SAR_TOUCH_THRES8 = crate::Reg<sar_touch_thres8::SAR_TOUCH_THRES8_SPEC>;
#[doc = "Finger threshold for touch pad 8"]
pub mod sar_touch_thres8;
#[doc = "SAR_TOUCH_THRES9 (rw) register accessor: Finger threshold for touch pad 9\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres9`] module"]
pub type SAR_TOUCH_THRES9 = crate::Reg<sar_touch_thres9::SAR_TOUCH_THRES9_SPEC>;
#[doc = "Finger threshold for touch pad 9"]
pub mod sar_touch_thres9;
#[doc = "SAR_TOUCH_THRES10 (rw) register accessor: Finger threshold for touch pad 10\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres10`] module"]
pub type SAR_TOUCH_THRES10 = crate::Reg<sar_touch_thres10::SAR_TOUCH_THRES10_SPEC>;
#[doc = "Finger threshold for touch pad 10"]
pub mod sar_touch_thres10;
#[doc = "SAR_TOUCH_THRES11 (rw) register accessor: Finger threshold for touch pad 11\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres11`] module"]
pub type SAR_TOUCH_THRES11 = crate::Reg<sar_touch_thres11::SAR_TOUCH_THRES11_SPEC>;
#[doc = "Finger threshold for touch pad 11"]
pub mod sar_touch_thres11;
#[doc = "SAR_TOUCH_THRES12 (rw) register accessor: Finger threshold for touch pad 12\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres12`] module"]
pub type SAR_TOUCH_THRES12 = crate::Reg<sar_touch_thres12::SAR_TOUCH_THRES12_SPEC>;
#[doc = "Finger threshold for touch pad 12"]
pub mod sar_touch_thres12;
#[doc = "SAR_TOUCH_THRES13 (rw) register accessor: Finger threshold for touch pad 13\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres13`] module"]
pub type SAR_TOUCH_THRES13 = crate::Reg<sar_touch_thres13::SAR_TOUCH_THRES13_SPEC>;
#[doc = "Finger threshold for touch pad 13"]
pub mod sar_touch_thres13;
#[doc = "SAR_TOUCH_THRES14 (rw) register accessor: Finger threshold for touch pad 14\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_thres14`] module"]
pub type SAR_TOUCH_THRES14 = crate::Reg<sar_touch_thres14::SAR_TOUCH_THRES14_SPEC>;
#[doc = "Finger threshold for touch pad 14"]
pub mod sar_touch_thres14;
#[doc = "SAR_TOUCH_CHN_ST (rw) register accessor: Touch channel status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_chn_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_chn_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_chn_st`] module"]
pub type SAR_TOUCH_CHN_ST = crate::Reg<sar_touch_chn_st::SAR_TOUCH_CHN_ST_SPEC>;
#[doc = "Touch channel status register"]
pub mod sar_touch_chn_st;
#[doc = "SAR_TOUCH_STATUS0 (r) register accessor: Status of touch controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status0`] module"]
pub type SAR_TOUCH_STATUS0 = crate::Reg<sar_touch_status0::SAR_TOUCH_STATUS0_SPEC>;
#[doc = "Status of touch controller"]
pub mod sar_touch_status0;
#[doc = "SAR_TOUCH_STATUS1 (r) register accessor: Touch pad 1 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status1`] module"]
pub type SAR_TOUCH_STATUS1 = crate::Reg<sar_touch_status1::SAR_TOUCH_STATUS1_SPEC>;
#[doc = "Touch pad 1 status"]
pub mod sar_touch_status1;
#[doc = "SAR_TOUCH_STATUS2 (r) register accessor: Touch pad 2 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status2`] module"]
pub type SAR_TOUCH_STATUS2 = crate::Reg<sar_touch_status2::SAR_TOUCH_STATUS2_SPEC>;
#[doc = "Touch pad 2 status"]
pub mod sar_touch_status2;
#[doc = "SAR_TOUCH_STATUS3 (r) register accessor: Touch pad 3 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status3`] module"]
pub type SAR_TOUCH_STATUS3 = crate::Reg<sar_touch_status3::SAR_TOUCH_STATUS3_SPEC>;
#[doc = "Touch pad 3 status"]
pub mod sar_touch_status3;
#[doc = "SAR_TOUCH_STATUS4 (r) register accessor: Touch pad 4 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status4`] module"]
pub type SAR_TOUCH_STATUS4 = crate::Reg<sar_touch_status4::SAR_TOUCH_STATUS4_SPEC>;
#[doc = "Touch pad 4 status"]
pub mod sar_touch_status4;
#[doc = "SAR_TOUCH_STATUS5 (r) register accessor: Touch pad 5 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status5`] module"]
pub type SAR_TOUCH_STATUS5 = crate::Reg<sar_touch_status5::SAR_TOUCH_STATUS5_SPEC>;
#[doc = "Touch pad 5 status"]
pub mod sar_touch_status5;
#[doc = "SAR_TOUCH_STATUS6 (r) register accessor: Touch pad 6 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status6`] module"]
pub type SAR_TOUCH_STATUS6 = crate::Reg<sar_touch_status6::SAR_TOUCH_STATUS6_SPEC>;
#[doc = "Touch pad 6 status"]
pub mod sar_touch_status6;
#[doc = "SAR_TOUCH_STATUS7 (r) register accessor: Touch pad 7 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status7`] module"]
pub type SAR_TOUCH_STATUS7 = crate::Reg<sar_touch_status7::SAR_TOUCH_STATUS7_SPEC>;
#[doc = "Touch pad 7 status"]
pub mod sar_touch_status7;
#[doc = "SAR_TOUCH_STATUS8 (r) register accessor: Touch pad 8 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status8`] module"]
pub type SAR_TOUCH_STATUS8 = crate::Reg<sar_touch_status8::SAR_TOUCH_STATUS8_SPEC>;
#[doc = "Touch pad 8 status"]
pub mod sar_touch_status8;
#[doc = "SAR_TOUCH_STATUS9 (r) register accessor: Touch pad 9 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status9`] module"]
pub type SAR_TOUCH_STATUS9 = crate::Reg<sar_touch_status9::SAR_TOUCH_STATUS9_SPEC>;
#[doc = "Touch pad 9 status"]
pub mod sar_touch_status9;
#[doc = "SAR_TOUCH_STATUS10 (r) register accessor: Touch pad 10 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status10`] module"]
pub type SAR_TOUCH_STATUS10 = crate::Reg<sar_touch_status10::SAR_TOUCH_STATUS10_SPEC>;
#[doc = "Touch pad 10 status"]
pub mod sar_touch_status10;
#[doc = "SAR_TOUCH_STATUS11 (r) register accessor: Touch pad 11 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status11`] module"]
pub type SAR_TOUCH_STATUS11 = crate::Reg<sar_touch_status11::SAR_TOUCH_STATUS11_SPEC>;
#[doc = "Touch pad 11 status"]
pub mod sar_touch_status11;
#[doc = "SAR_TOUCH_STATUS12 (r) register accessor: Touch pad 12 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status12`] module"]
pub type SAR_TOUCH_STATUS12 = crate::Reg<sar_touch_status12::SAR_TOUCH_STATUS12_SPEC>;
#[doc = "Touch pad 12 status"]
pub mod sar_touch_status12;
#[doc = "SAR_TOUCH_STATUS13 (r) register accessor: Touch pad 13 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status13`] module"]
pub type SAR_TOUCH_STATUS13 = crate::Reg<sar_touch_status13::SAR_TOUCH_STATUS13_SPEC>;
#[doc = "Touch pad 13 status"]
pub mod sar_touch_status13;
#[doc = "SAR_TOUCH_STATUS14 (r) register accessor: Touch pad 14 status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status14`] module"]
pub type SAR_TOUCH_STATUS14 = crate::Reg<sar_touch_status14::SAR_TOUCH_STATUS14_SPEC>;
#[doc = "Touch pad 14 status"]
pub mod sar_touch_status14;
#[doc = "SAR_TOUCH_STATUS15 (r) register accessor: Touch sleep pad status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status15`] module"]
pub type SAR_TOUCH_STATUS15 = crate::Reg<sar_touch_status15::SAR_TOUCH_STATUS15_SPEC>;
#[doc = "Touch sleep pad status"]
pub mod sar_touch_status15;
#[doc = "SAR_TOUCH_STATUS16 (r) register accessor: Touch approach count status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_touch_status16`] module"]
pub type SAR_TOUCH_STATUS16 = crate::Reg<sar_touch_status16::SAR_TOUCH_STATUS16_SPEC>;
#[doc = "Touch approach count status"]
pub mod sar_touch_status16;
#[doc = "SAR_DAC_CTRL1 (rw) register accessor: DAC control\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_dac_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_dac_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_dac_ctrl1`] module"]
pub type SAR_DAC_CTRL1 = crate::Reg<sar_dac_ctrl1::SAR_DAC_CTRL1_SPEC>;
#[doc = "DAC control"]
pub mod sar_dac_ctrl1;
#[doc = "SAR_DAC_CTRL2 (rw) register accessor: DAC output control\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_dac_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_dac_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_dac_ctrl2`] module"]
pub type SAR_DAC_CTRL2 = crate::Reg<sar_dac_ctrl2::SAR_DAC_CTRL2_SPEC>;
#[doc = "DAC output control"]
pub mod sar_dac_ctrl2;
#[doc = "SAR_COCPU_STATE (rw) register accessor: ULP-RISCV status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_cocpu_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_state`] module"]
pub type SAR_COCPU_STATE = crate::Reg<sar_cocpu_state::SAR_COCPU_STATE_SPEC>;
#[doc = "ULP-RISCV status"]
pub mod sar_cocpu_state;
#[doc = "SAR_COCPU_INT_RAW (r) register accessor: Interrupt raw bit of ULP-RISCV\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_raw`] module"]
pub type SAR_COCPU_INT_RAW = crate::Reg<sar_cocpu_int_raw::SAR_COCPU_INT_RAW_SPEC>;
#[doc = "Interrupt raw bit of ULP-RISCV"]
pub mod sar_cocpu_int_raw;
#[doc = "SAR_COCPU_INT_ENA (rw) register accessor: Interrupt enable bit of ULP-RISCV\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_cocpu_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_ena`] module"]
pub type SAR_COCPU_INT_ENA = crate::Reg<sar_cocpu_int_ena::SAR_COCPU_INT_ENA_SPEC>;
#[doc = "Interrupt enable bit of ULP-RISCV"]
pub mod sar_cocpu_int_ena;
#[doc = "SAR_COCPU_INT_ST (r) register accessor: Interrupt status bit of ULP-RISCV\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_st`] module"]
pub type SAR_COCPU_INT_ST = crate::Reg<sar_cocpu_int_st::SAR_COCPU_INT_ST_SPEC>;
#[doc = "Interrupt status bit of ULP-RISCV"]
pub mod sar_cocpu_int_st;
#[doc = "SAR_COCPU_INT_CLR (w) register accessor: Interrupt clear bit of ULP-RISCV\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_cocpu_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_int_clr`] module"]
pub type SAR_COCPU_INT_CLR = crate::Reg<sar_cocpu_int_clr::SAR_COCPU_INT_CLR_SPEC>;
#[doc = "Interrupt clear bit of ULP-RISCV"]
pub mod sar_cocpu_int_clr;
#[doc = "SAR_COCPU_DEBUG (r) register accessor: ULP-RISCV debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_debug::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_cocpu_debug`] module"]
pub type SAR_COCPU_DEBUG = crate::Reg<sar_cocpu_debug::SAR_COCPU_DEBUG_SPEC>;
#[doc = "ULP-RISCV debug register"]
pub mod sar_cocpu_debug;
#[doc = "SAR_HALL_CTRL (rw) register accessor: hall control\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_hall_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_hall_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_hall_ctrl`] module"]
pub type SAR_HALL_CTRL = crate::Reg<sar_hall_ctrl::SAR_HALL_CTRL_SPEC>;
#[doc = "hall control"]
pub mod sar_hall_ctrl;
#[doc = "SAR_NOUSE (rw) register accessor: sar nouse\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_nouse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_nouse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_nouse`] module"]
pub type SAR_NOUSE = crate::Reg<sar_nouse::SAR_NOUSE_SPEC>;
#[doc = "sar nouse"]
pub mod sar_nouse;
#[doc = "SAR_IO_MUX_CONF (rw) register accessor: Configure and reset IO MUX\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_io_mux_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_io_mux_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_io_mux_conf`] module"]
pub type SAR_IO_MUX_CONF = crate::Reg<sar_io_mux_conf::SAR_IO_MUX_CONF_SPEC>;
#[doc = "Configure and reset IO MUX"]
pub mod sar_io_mux_conf;
#[doc = "SARDATE (rw) register accessor: Version Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sardate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sardate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sardate`] module"]
pub type SARDATE = crate::Reg<sardate::SARDATE_SPEC>;
#[doc = "Version Control Register"]
pub mod sardate;
