#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATE_SPEC>;
#[doc = "Field `DATE` reader - x"]
pub type DATE_R = crate::FieldReader<u32>;
#[doc = "Field `DATE` writer - x"]
pub type DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `CLK_EN` reader - register file clk gating"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - register file clk gating"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27 - x"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - register file clk gating"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("date", &self.date())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - x"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<DATE_SPEC> {
        DATE_W::new(self, 0)
    }
    #[doc = "Bit 31 - register file clk gating"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<DATE_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "x\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATE to value 0x0201_2300"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x0201_2300;
}
