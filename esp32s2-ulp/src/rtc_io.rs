#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    rtc_gpio_out: RTC_GPIO_OUT,
    rtc_gpio_out_w1ts: RTC_GPIO_OUT_W1TS,
    rtc_gpio_out_w1tc: RTC_GPIO_OUT_W1TC,
    rtc_gpio_enable: RTC_GPIO_ENABLE,
    rtc_gpio_enable_w1ts: RTC_GPIO_ENABLE_W1TS,
    rtc_gpio_enable_w1tc: RTC_GPIO_ENABLE_W1TC,
    rtc_gpio_status: RTC_GPIO_STATUS,
    rtc_gpio_status_w1ts: RTC_GPIO_STATUS_W1TS,
    rtc_gpio_status_w1tc: RTC_GPIO_STATUS_W1TC,
    rtc_gpio_in: RTC_GPIO_IN,
    rtc_gpio_pin: [RTC_GPIO_PIN; 22],
    rtc_debug_sel: RTC_DEBUG_SEL,
    touch_pad: [TOUCH_PAD; 15],
    xtal_32p_pad: XTAL_32P_PAD,
    xtal_32n_pad: XTAL_32N_PAD,
    pad_dac1: PAD_DAC1,
    pad_dac2: PAD_DAC2,
    rtc_pad19: RTC_PAD19,
    rtc_pad20: RTC_PAD20,
    rtc_pad21: RTC_PAD21,
    ext_wakeup0: EXT_WAKEUP0,
    xtl_ext_ctr: XTL_EXT_CTR,
    sar_i2c_io: SAR_I2C_IO,
    rtc_io_touch_ctrl: RTC_IO_TOUCH_CTRL,
    _reserved24: [u8; 0x0110],
    rtc_io_date: RTC_IO_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC GPIO output register"]
    #[inline(always)]
    pub const fn rtc_gpio_out(&self) -> &RTC_GPIO_OUT {
        &self.rtc_gpio_out
    }
    #[doc = "0x04 - RTC GPIO output bit set register"]
    #[inline(always)]
    pub const fn rtc_gpio_out_w1ts(&self) -> &RTC_GPIO_OUT_W1TS {
        &self.rtc_gpio_out_w1ts
    }
    #[doc = "0x08 - RTC GPIO output bit clear register"]
    #[inline(always)]
    pub const fn rtc_gpio_out_w1tc(&self) -> &RTC_GPIO_OUT_W1TC {
        &self.rtc_gpio_out_w1tc
    }
    #[doc = "0x0c - RTC GPIO output enable register"]
    #[inline(always)]
    pub const fn rtc_gpio_enable(&self) -> &RTC_GPIO_ENABLE {
        &self.rtc_gpio_enable
    }
    #[doc = "0x10 - RTC GPIO output enable bit set register"]
    #[inline(always)]
    pub const fn rtc_gpio_enable_w1ts(&self) -> &RTC_GPIO_ENABLE_W1TS {
        &self.rtc_gpio_enable_w1ts
    }
    #[doc = "0x14 - RTC GPIO output enable bit clear register"]
    #[inline(always)]
    pub const fn rtc_gpio_enable_w1tc(&self) -> &RTC_GPIO_ENABLE_W1TC {
        &self.rtc_gpio_enable_w1tc
    }
    #[doc = "0x18 - RTC GPIO interrupt status register"]
    #[inline(always)]
    pub const fn rtc_gpio_status(&self) -> &RTC_GPIO_STATUS {
        &self.rtc_gpio_status
    }
    #[doc = "0x1c - RTC GPIO interrupt status bit set register"]
    #[inline(always)]
    pub const fn rtc_gpio_status_w1ts(&self) -> &RTC_GPIO_STATUS_W1TS {
        &self.rtc_gpio_status_w1ts
    }
    #[doc = "0x20 - RTC GPIO interrupt status bit clear register"]
    #[inline(always)]
    pub const fn rtc_gpio_status_w1tc(&self) -> &RTC_GPIO_STATUS_W1TC {
        &self.rtc_gpio_status_w1tc
    }
    #[doc = "0x24 - RTC GPIO input register"]
    #[inline(always)]
    pub const fn rtc_gpio_in(&self) -> &RTC_GPIO_IN {
        &self.rtc_gpio_in
    }
    #[doc = "0x28..0x80 - RTC configuration for pin %s"]
    #[inline(always)]
    pub const fn rtc_gpio_pin(&self, n: usize) -> &RTC_GPIO_PIN {
        &self.rtc_gpio_pin[n]
    }
    #[doc = "0x80 - RTC debug select register"]
    #[inline(always)]
    pub const fn rtc_debug_sel(&self) -> &RTC_DEBUG_SEL {
        &self.rtc_debug_sel
    }
    #[doc = "0x84..0xc0 - Touch pad %s configuration register"]
    #[inline(always)]
    pub const fn touch_pad(&self, n: usize) -> &TOUCH_PAD {
        &self.touch_pad[n]
    }
    #[doc = "0xc0 - 32KHz crystal P-pad configuration register"]
    #[inline(always)]
    pub const fn xtal_32p_pad(&self) -> &XTAL_32P_PAD {
        &self.xtal_32p_pad
    }
    #[doc = "0xc4 - 32KHz crystal N-pad configuration register"]
    #[inline(always)]
    pub const fn xtal_32n_pad(&self) -> &XTAL_32N_PAD {
        &self.xtal_32n_pad
    }
    #[doc = "0xc8 - DAC1 configuration register"]
    #[inline(always)]
    pub const fn pad_dac1(&self) -> &PAD_DAC1 {
        &self.pad_dac1
    }
    #[doc = "0xcc - DAC2 configuration register"]
    #[inline(always)]
    pub const fn pad_dac2(&self) -> &PAD_DAC2 {
        &self.pad_dac2
    }
    #[doc = "0xd0 - Touch pad 19 configuration register"]
    #[inline(always)]
    pub const fn rtc_pad19(&self) -> &RTC_PAD19 {
        &self.rtc_pad19
    }
    #[doc = "0xd4 - Touch pad 20 configuration register"]
    #[inline(always)]
    pub const fn rtc_pad20(&self) -> &RTC_PAD20 {
        &self.rtc_pad20
    }
    #[doc = "0xd8 - Touch pad 21 configuration register"]
    #[inline(always)]
    pub const fn rtc_pad21(&self) -> &RTC_PAD21 {
        &self.rtc_pad21
    }
    #[doc = "0xdc - External wake up configuration register"]
    #[inline(always)]
    pub const fn ext_wakeup0(&self) -> &EXT_WAKEUP0 {
        &self.ext_wakeup0
    }
    #[doc = "0xe0 - Crystal power down enable GPIO source"]
    #[inline(always)]
    pub const fn xtl_ext_ctr(&self) -> &XTL_EXT_CTR {
        &self.xtl_ext_ctr
    }
    #[doc = "0xe4 - RTC I2C pad selection"]
    #[inline(always)]
    pub const fn sar_i2c_io(&self) -> &SAR_I2C_IO {
        &self.sar_i2c_io
    }
    #[doc = "0xe8 - Touch control register"]
    #[inline(always)]
    pub const fn rtc_io_touch_ctrl(&self) -> &RTC_IO_TOUCH_CTRL {
        &self.rtc_io_touch_ctrl
    }
    #[doc = "0x1fc - Version control register"]
    #[inline(always)]
    pub const fn rtc_io_date(&self) -> &RTC_IO_DATE {
        &self.rtc_io_date
    }
}
#[doc = "RTC_GPIO_OUT (rw) register accessor: RTC GPIO output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_out`] module"]
pub type RTC_GPIO_OUT = crate::Reg<rtc_gpio_out::RTC_GPIO_OUT_SPEC>;
#[doc = "RTC GPIO output register"]
pub mod rtc_gpio_out;
#[doc = "RTC_GPIO_OUT_W1TS (w) register accessor: RTC GPIO output bit set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_out_w1ts`] module"]
pub type RTC_GPIO_OUT_W1TS = crate::Reg<rtc_gpio_out_w1ts::RTC_GPIO_OUT_W1TS_SPEC>;
#[doc = "RTC GPIO output bit set register"]
pub mod rtc_gpio_out_w1ts;
#[doc = "RTC_GPIO_OUT_W1TC (w) register accessor: RTC GPIO output bit clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_out_w1tc`] module"]
pub type RTC_GPIO_OUT_W1TC = crate::Reg<rtc_gpio_out_w1tc::RTC_GPIO_OUT_W1TC_SPEC>;
#[doc = "RTC GPIO output bit clear register"]
pub mod rtc_gpio_out_w1tc;
#[doc = "RTC_GPIO_ENABLE (rw) register accessor: RTC GPIO output enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_enable`] module"]
pub type RTC_GPIO_ENABLE = crate::Reg<rtc_gpio_enable::RTC_GPIO_ENABLE_SPEC>;
#[doc = "RTC GPIO output enable register"]
pub mod rtc_gpio_enable;
#[doc = "RTC_GPIO_ENABLE_W1TS (w) register accessor: RTC GPIO output enable bit set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_enable_w1ts`] module"]
pub type RTC_GPIO_ENABLE_W1TS = crate::Reg<rtc_gpio_enable_w1ts::RTC_GPIO_ENABLE_W1TS_SPEC>;
#[doc = "RTC GPIO output enable bit set register"]
pub mod rtc_gpio_enable_w1ts;
#[doc = "RTC_GPIO_ENABLE_W1TC (w) register accessor: RTC GPIO output enable bit clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_enable_w1tc`] module"]
pub type RTC_GPIO_ENABLE_W1TC = crate::Reg<rtc_gpio_enable_w1tc::RTC_GPIO_ENABLE_W1TC_SPEC>;
#[doc = "RTC GPIO output enable bit clear register"]
pub mod rtc_gpio_enable_w1tc;
#[doc = "RTC_GPIO_STATUS (rw) register accessor: RTC GPIO interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_status`] module"]
pub type RTC_GPIO_STATUS = crate::Reg<rtc_gpio_status::RTC_GPIO_STATUS_SPEC>;
#[doc = "RTC GPIO interrupt status register"]
pub mod rtc_gpio_status;
#[doc = "RTC_GPIO_STATUS_W1TS (w) register accessor: RTC GPIO interrupt status bit set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_status_w1ts`] module"]
pub type RTC_GPIO_STATUS_W1TS = crate::Reg<rtc_gpio_status_w1ts::RTC_GPIO_STATUS_W1TS_SPEC>;
#[doc = "RTC GPIO interrupt status bit set register"]
pub mod rtc_gpio_status_w1ts;
#[doc = "RTC_GPIO_STATUS_W1TC (w) register accessor: RTC GPIO interrupt status bit clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_status_w1tc`] module"]
pub type RTC_GPIO_STATUS_W1TC = crate::Reg<rtc_gpio_status_w1tc::RTC_GPIO_STATUS_W1TC_SPEC>;
#[doc = "RTC GPIO interrupt status bit clear register"]
pub mod rtc_gpio_status_w1tc;
#[doc = "RTC_GPIO_IN (r) register accessor: RTC GPIO input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_in::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_in`] module"]
pub type RTC_GPIO_IN = crate::Reg<rtc_gpio_in::RTC_GPIO_IN_SPEC>;
#[doc = "RTC GPIO input register"]
pub mod rtc_gpio_in;
#[doc = "RTC_GPIO_PIN (rw) register accessor: RTC configuration for pin %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_gpio_pin`] module"]
pub type RTC_GPIO_PIN = crate::Reg<rtc_gpio_pin::RTC_GPIO_PIN_SPEC>;
#[doc = "RTC configuration for pin %s"]
pub mod rtc_gpio_pin;
#[doc = "RTC_DEBUG_SEL (rw) register accessor: RTC debug select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_debug_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_debug_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_debug_sel`] module"]
pub type RTC_DEBUG_SEL = crate::Reg<rtc_debug_sel::RTC_DEBUG_SEL_SPEC>;
#[doc = "RTC debug select register"]
pub mod rtc_debug_sel;
#[doc = "TOUCH_PAD (rw) register accessor: Touch pad %s configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pad`] module"]
pub type TOUCH_PAD = crate::Reg<touch_pad::TOUCH_PAD_SPEC>;
#[doc = "Touch pad %s configuration register"]
pub mod touch_pad;
#[doc = "XTAL_32P_PAD (rw) register accessor: 32KHz crystal P-pad configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal_32p_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_32p_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_32p_pad`] module"]
pub type XTAL_32P_PAD = crate::Reg<xtal_32p_pad::XTAL_32P_PAD_SPEC>;
#[doc = "32KHz crystal P-pad configuration register"]
pub mod xtal_32p_pad;
#[doc = "XTAL_32N_PAD (rw) register accessor: 32KHz crystal N-pad configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal_32n_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_32n_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_32n_pad`] module"]
pub type XTAL_32N_PAD = crate::Reg<xtal_32n_pad::XTAL_32N_PAD_SPEC>;
#[doc = "32KHz crystal N-pad configuration register"]
pub mod xtal_32n_pad;
#[doc = "PAD_DAC1 (rw) register accessor: DAC1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_dac1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_dac1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_dac1`] module"]
pub type PAD_DAC1 = crate::Reg<pad_dac1::PAD_DAC1_SPEC>;
#[doc = "DAC1 configuration register"]
pub mod pad_dac1;
#[doc = "PAD_DAC2 (rw) register accessor: DAC2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_dac2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_dac2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_dac2`] module"]
pub type PAD_DAC2 = crate::Reg<pad_dac2::PAD_DAC2_SPEC>;
#[doc = "DAC2 configuration register"]
pub mod pad_dac2;
#[doc = "RTC_PAD19 (rw) register accessor: Touch pad 19 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_pad19`] module"]
pub type RTC_PAD19 = crate::Reg<rtc_pad19::RTC_PAD19_SPEC>;
#[doc = "Touch pad 19 configuration register"]
pub mod rtc_pad19;
#[doc = "RTC_PAD20 (rw) register accessor: Touch pad 20 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_pad20`] module"]
pub type RTC_PAD20 = crate::Reg<rtc_pad20::RTC_PAD20_SPEC>;
#[doc = "Touch pad 20 configuration register"]
pub mod rtc_pad20;
#[doc = "RTC_PAD21 (rw) register accessor: Touch pad 21 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_pad21`] module"]
pub type RTC_PAD21 = crate::Reg<rtc_pad21::RTC_PAD21_SPEC>;
#[doc = "Touch pad 21 configuration register"]
pub mod rtc_pad21;
#[doc = "EXT_WAKEUP0 (rw) register accessor: External wake up configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup0`] module"]
pub type EXT_WAKEUP0 = crate::Reg<ext_wakeup0::EXT_WAKEUP0_SPEC>;
#[doc = "External wake up configuration register"]
pub mod ext_wakeup0;
#[doc = "XTL_EXT_CTR (rw) register accessor: Crystal power down enable GPIO source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtl_ext_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtl_ext_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtl_ext_ctr`] module"]
pub type XTL_EXT_CTR = crate::Reg<xtl_ext_ctr::XTL_EXT_CTR_SPEC>;
#[doc = "Crystal power down enable GPIO source"]
pub mod xtl_ext_ctr;
#[doc = "SAR_I2C_IO (rw) register accessor: RTC I2C pad selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_i2c_io::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_i2c_io::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_i2c_io`] module"]
pub type SAR_I2C_IO = crate::Reg<sar_i2c_io::SAR_I2C_IO_SPEC>;
#[doc = "RTC I2C pad selection"]
pub mod sar_i2c_io;
#[doc = "RTC_IO_TOUCH_CTRL (rw) register accessor: Touch control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_io_touch_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_io_touch_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_io_touch_ctrl`] module"]
pub type RTC_IO_TOUCH_CTRL = crate::Reg<rtc_io_touch_ctrl::RTC_IO_TOUCH_CTRL_SPEC>;
#[doc = "Touch control register"]
pub mod rtc_io_touch_ctrl;
#[doc = "RTC_IO_DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_io_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_io_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_io_date`] module"]
pub type RTC_IO_DATE = crate::Reg<rtc_io_date::RTC_IO_DATE_SPEC>;
#[doc = "Version control register"]
pub mod rtc_io_date;
