#[doc = "Register `RTC_PAD19` reader"]
pub type R = crate::R<RTC_PAD19_SPEC>;
#[doc = "Register `RTC_PAD19` writer"]
pub type W = crate::W<RTC_PAD19_SPEC>;
#[doc = "Field `FUN_IE` reader - input enable in work mode"]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - input enable in work mode"]
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_OE` reader - output enable in sleep mode"]
pub type SLP_OE_R = crate::BitReader;
#[doc = "Field `SLP_OE` writer - output enable in sleep mode"]
pub type SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_IE` reader - input enable in sleep mode"]
pub type SLP_IE_R = crate::BitReader;
#[doc = "Field `SLP_IE` writer - input enable in sleep mode"]
pub type SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_SEL` reader - function sel"]
pub type FUN_SEL_R = crate::FieldReader;
#[doc = "Field `FUN_SEL` writer - function sel"]
pub type FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MUX_SEL` reader - 1: use RTC GPIO,0: use digital GPIO"]
pub type MUX_SEL_R = crate::BitReader;
#[doc = "Field `MUX_SEL` writer - 1: use RTC GPIO,0: use digital GPIO"]
pub type MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUE` reader - RUE"]
pub type RUE_R = crate::BitReader;
#[doc = "Field `RUE` writer - RUE"]
pub type RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDE` reader - RDE"]
pub type RDE_R = crate::BitReader;
#[doc = "Field `RDE` writer - RDE"]
pub type RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRV` reader - DRV"]
pub type DRV_R = crate::FieldReader;
#[doc = "Field `DRV` writer - DRV"]
pub type DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - function sel"]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn mux_sel(&self) -> MUX_SEL_R {
        MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 27 - RUE"]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RDE"]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - DRV"]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_PAD19")
            .field("fun_ie", &format_args!("{}", self.fun_ie().bit()))
            .field("slp_oe", &format_args!("{}", self.slp_oe().bit()))
            .field("slp_ie", &format_args!("{}", self.slp_ie().bit()))
            .field("slp_sel", &format_args!("{}", self.slp_sel().bit()))
            .field("fun_sel", &format_args!("{}", self.fun_sel().bits()))
            .field("mux_sel", &format_args!("{}", self.mux_sel().bit()))
            .field("rue", &format_args!("{}", self.rue().bit()))
            .field("rde", &format_args!("{}", self.rde().bit()))
            .field("drv", &format_args!("{}", self.drv().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_PAD19_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    #[must_use]
    pub fn fun_ie(&mut self) -> FUN_IE_W<RTC_PAD19_SPEC> {
        FUN_IE_W::new(self, 13)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn slp_oe(&mut self) -> SLP_OE_W<RTC_PAD19_SPEC> {
        SLP_OE_W::new(self, 14)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn slp_ie(&mut self) -> SLP_IE_W<RTC_PAD19_SPEC> {
        SLP_IE_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<RTC_PAD19_SPEC> {
        SLP_SEL_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - function sel"]
    #[inline(always)]
    #[must_use]
    pub fn fun_sel(&mut self) -> FUN_SEL_W<RTC_PAD19_SPEC> {
        FUN_SEL_W::new(self, 17)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sel(&mut self) -> MUX_SEL_W<RTC_PAD19_SPEC> {
        MUX_SEL_W::new(self, 19)
    }
    #[doc = "Bit 27 - RUE"]
    #[inline(always)]
    #[must_use]
    pub fn rue(&mut self) -> RUE_W<RTC_PAD19_SPEC> {
        RUE_W::new(self, 27)
    }
    #[doc = "Bit 28 - RDE"]
    #[inline(always)]
    #[must_use]
    pub fn rde(&mut self) -> RDE_W<RTC_PAD19_SPEC> {
        RDE_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - DRV"]
    #[inline(always)]
    #[must_use]
    pub fn drv(&mut self) -> DRV_W<RTC_PAD19_SPEC> {
        DRV_W::new(self, 29)
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
#[doc = "configure RTC PAD19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_PAD19_SPEC;
impl crate::RegisterSpec for RTC_PAD19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_pad19::R`](R) reader structure"]
impl crate::Readable for RTC_PAD19_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_pad19::W`](W) writer structure"]
impl crate::Writable for RTC_PAD19_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_PAD19 to value 0x5000_0000"]
impl crate::Resettable for RTC_PAD19_SPEC {
    const RESET_VALUE: Self::Ux = 0x5000_0000;
}
