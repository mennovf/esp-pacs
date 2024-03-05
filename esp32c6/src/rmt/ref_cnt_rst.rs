#[doc = "Register `REF_CNT_RST` writer"]
pub type W = crate::W<REF_CNT_RST_SPEC>;
#[doc = "Field `TX_REF_CNT_RST` writer - This register is used to reset the clock divider of CHANNEL0."]
pub type TX_REF_CNT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_REF_CNT_RST_CH1` writer - This register is used to reset the clock divider of CHANNEL1."]
pub type TX_REF_CNT_RST_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_REF_CNT_RST_CH2` writer - This register is used to reset the clock divider of CHANNEL2."]
pub type RX_REF_CNT_RST_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_REF_CNT_RST_CH3` writer - This register is used to reset the clock divider of CHANNEL3."]
pub type RX_REF_CNT_RST_CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REF_CNT_RST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ref_cnt_rst(&mut self) -> TX_REF_CNT_RST_W<REF_CNT_RST_SPEC> {
        TX_REF_CNT_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - This register is used to reset the clock divider of CHANNEL1."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ref_cnt_rst_ch1(&mut self) -> TX_REF_CNT_RST_CH1_W<REF_CNT_RST_SPEC> {
        TX_REF_CNT_RST_CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - This register is used to reset the clock divider of CHANNEL2."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ref_cnt_rst_ch2(&mut self) -> RX_REF_CNT_RST_CH2_W<REF_CNT_RST_SPEC> {
        RX_REF_CNT_RST_CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - This register is used to reset the clock divider of CHANNEL3."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ref_cnt_rst_ch3(&mut self) -> RX_REF_CNT_RST_CH3_W<REF_CNT_RST_SPEC> {
        RX_REF_CNT_RST_CH3_W::new(self, 3)
    }
}
#[doc = "RMT clock divider reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ref_cnt_rst::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_CNT_RST_SPEC;
impl crate::RegisterSpec for REF_CNT_RST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ref_cnt_rst::W`](W) writer structure"]
impl crate::Writable for REF_CNT_RST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REF_CNT_RST to value 0"]
impl crate::Resettable for REF_CNT_RST_SPEC {
    const RESET_VALUE: u32 = 0;
}
