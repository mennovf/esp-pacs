#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `APB_SARADC_TSENS` reader - saradc tsens interrupt state"]
pub type APB_SARADC_TSENS_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES1_LOW` reader - saradc thres1 low interrupt state"]
pub type APB_SARADC_THRES1_LOW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_LOW` reader - saradc thres0 low interrupt state"]
pub type APB_SARADC_THRES0_LOW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES1_HIGH` reader - saradc thres1 high interrupt state"]
pub type APB_SARADC_THRES1_HIGH_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_HIGH` reader - saradc thres0 high interrupt state"]
pub type APB_SARADC_THRES0_HIGH_R = crate::BitReader;
#[doc = "Field `APB_SARADC2_DONE` reader - saradc2 done interrupt state"]
pub type APB_SARADC2_DONE_R = crate::BitReader;
#[doc = "Field `APB_SARADC1_DONE` reader - saradc1 done interrupt state"]
pub type APB_SARADC1_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 25 - saradc tsens interrupt state"]
    #[inline(always)]
    pub fn apb_saradc_tsens(&self) -> APB_SARADC_TSENS_R {
        APB_SARADC_TSENS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - saradc thres1 low interrupt state"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low(&self) -> APB_SARADC_THRES1_LOW_R {
        APB_SARADC_THRES1_LOW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - saradc thres0 low interrupt state"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low(&self) -> APB_SARADC_THRES0_LOW_R {
        APB_SARADC_THRES0_LOW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - saradc thres1 high interrupt state"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high(&self) -> APB_SARADC_THRES1_HIGH_R {
        APB_SARADC_THRES1_HIGH_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - saradc thres0 high interrupt state"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high(&self) -> APB_SARADC_THRES0_HIGH_R {
        APB_SARADC_THRES0_HIGH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - saradc2 done interrupt state"]
    #[inline(always)]
    pub fn apb_saradc2_done(&self) -> APB_SARADC2_DONE_R {
        APB_SARADC2_DONE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - saradc1 done interrupt state"]
    #[inline(always)]
    pub fn apb_saradc1_done(&self) -> APB_SARADC1_DONE_R {
        APB_SARADC1_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "apb_saradc_tsens",
                &format_args!("{}", self.apb_saradc_tsens().bit()),
            )
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
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "digital saradc int register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
