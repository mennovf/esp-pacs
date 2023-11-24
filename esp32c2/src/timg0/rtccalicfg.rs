#[doc = "Register `RTCCALICFG` reader"]
pub type R = crate::R<RTCCALICFG_SPEC>;
#[doc = "Register `RTCCALICFG` writer"]
pub type W = crate::W<RTCCALICFG_SPEC>;
#[doc = "Field `RTC_CALI_START_CYCLING` reader - Reserved"]
pub type RTC_CALI_START_CYCLING_R = crate::BitReader;
#[doc = "Field `RTC_CALI_START_CYCLING` writer - Reserved"]
pub type RTC_CALI_START_CYCLING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_CALI_CLK_SEL` reader - 0:rtc slow clock. 1:clk_8m, 2:xtal_32k."]
pub type RTC_CALI_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `RTC_CALI_CLK_SEL` writer - 0:rtc slow clock. 1:clk_8m, 2:xtal_32k."]
pub type RTC_CALI_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTC_CALI_RDY` reader - Reserved"]
pub type RTC_CALI_RDY_R = crate::BitReader;
#[doc = "Field `RTC_CALI_MAX` reader - Reserved"]
pub type RTC_CALI_MAX_R = crate::FieldReader<u16>;
#[doc = "Field `RTC_CALI_MAX` writer - Reserved"]
pub type RTC_CALI_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `RTC_CALI_START` reader - Reserved"]
pub type RTC_CALI_START_R = crate::BitReader;
#[doc = "Field `RTC_CALI_START` writer - Reserved"]
pub type RTC_CALI_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn rtc_cali_start_cycling(&self) -> RTC_CALI_START_CYCLING_R {
        RTC_CALI_START_CYCLING_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 0:rtc slow clock. 1:clk_8m, 2:xtal_32k."]
    #[inline(always)]
    pub fn rtc_cali_clk_sel(&self) -> RTC_CALI_CLK_SEL_R {
        RTC_CALI_CLK_SEL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn rtc_cali_rdy(&self) -> RTC_CALI_RDY_R {
        RTC_CALI_RDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - Reserved"]
    #[inline(always)]
    pub fn rtc_cali_max(&self) -> RTC_CALI_MAX_R {
        RTC_CALI_MAX_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn rtc_cali_start(&self) -> RTC_CALI_START_R {
        RTC_CALI_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCALICFG")
            .field(
                "rtc_cali_start_cycling",
                &format_args!("{}", self.rtc_cali_start_cycling().bit()),
            )
            .field(
                "rtc_cali_clk_sel",
                &format_args!("{}", self.rtc_cali_clk_sel().bits()),
            )
            .field(
                "rtc_cali_rdy",
                &format_args!("{}", self.rtc_cali_rdy().bit()),
            )
            .field(
                "rtc_cali_max",
                &format_args!("{}", self.rtc_cali_max().bits()),
            )
            .field(
                "rtc_cali_start",
                &format_args!("{}", self.rtc_cali_start().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTCCALICFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_cali_start_cycling(&mut self) -> RTC_CALI_START_CYCLING_W<RTCCALICFG_SPEC> {
        RTC_CALI_START_CYCLING_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - 0:rtc slow clock. 1:clk_8m, 2:xtal_32k."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_cali_clk_sel(&mut self) -> RTC_CALI_CLK_SEL_W<RTCCALICFG_SPEC> {
        RTC_CALI_CLK_SEL_W::new(self, 13)
    }
    #[doc = "Bits 16:30 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_cali_max(&mut self) -> RTC_CALI_MAX_W<RTCCALICFG_SPEC> {
        RTC_CALI_MAX_W::new(self, 16)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_cali_start(&mut self) -> RTC_CALI_START_W<RTCCALICFG_SPEC> {
        RTC_CALI_START_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC calibration configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCALICFG_SPEC;
impl crate::RegisterSpec for RTCCALICFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccalicfg::R`](R) reader structure"]
impl crate::Readable for RTCCALICFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtccalicfg::W`](W) writer structure"]
impl crate::Writable for RTCCALICFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCALICFG to value 0x0001_3000"]
impl crate::Resettable for RTCCALICFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_3000;
}
