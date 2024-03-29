#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `APB_SARADC_THRES1_LOW` reader - saradc thres1 low interrupt raw"]
pub type APB_SARADC_THRES1_LOW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_LOW` reader - saradc thres0 low interrupt raw"]
pub type APB_SARADC_THRES0_LOW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES1_HIGH` reader - saradc thres1 high interrupt raw"]
pub type APB_SARADC_THRES1_HIGH_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_HIGH` reader - saradc thres0 high interrupt raw"]
pub type APB_SARADC_THRES0_HIGH_R = crate::BitReader;
#[doc = "Field `APB_SARADC2_DONE` reader - saradc2 done interrupt raw"]
pub type APB_SARADC2_DONE_R = crate::BitReader;
#[doc = "Field `APB_SARADC1_DONE` reader - saradc1 done interrupt raw"]
pub type APB_SARADC1_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 26 - saradc thres1 low interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low(&self) -> APB_SARADC_THRES1_LOW_R {
        APB_SARADC_THRES1_LOW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - saradc thres0 low interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low(&self) -> APB_SARADC_THRES0_LOW_R {
        APB_SARADC_THRES0_LOW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - saradc thres1 high interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high(&self) -> APB_SARADC_THRES1_HIGH_R {
        APB_SARADC_THRES1_HIGH_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - saradc thres0 high interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high(&self) -> APB_SARADC_THRES0_HIGH_R {
        APB_SARADC_THRES0_HIGH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - saradc2 done interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc2_done(&self) -> APB_SARADC2_DONE_R {
        APB_SARADC2_DONE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - saradc1 done interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc1_done(&self) -> APB_SARADC1_DONE_R {
        APB_SARADC1_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "apb_saradc_thres1_low",
                &format_args!("{}", self.apb_saradc_thres1_low().bit()),
            )
            .field(
                "apb_saradc_thres0_low",
                &format_args!("{}", self.apb_saradc_thres0_low().bit()),
            )
            .field(
                "apb_saradc_thres1_high",
                &format_args!("{}", self.apb_saradc_thres1_high().bit()),
            )
            .field(
                "apb_saradc_thres0_high",
                &format_args!("{}", self.apb_saradc_thres0_high().bit()),
            )
            .field(
                "apb_saradc2_done",
                &format_args!("{}", self.apb_saradc2_done().bit()),
            )
            .field(
                "apb_saradc1_done",
                &format_args!("{}", self.apb_saradc1_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "digital saradc int register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
