#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATE_SPEC>;
#[doc = "Field `DATE` reader - need des"]
pub type DATE_R = crate::FieldReader<u32>;
#[doc = "Field `DATE` writer - need des"]
pub type DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `I2C_MST_CLK_EN` reader - need des"]
pub type I2C_MST_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2C_MST_CLK_EN` writer - need des"]
pub type I2C_MST_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27 - need des"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28 - need des"]
    #[inline(always)]
    pub fn i2c_mst_clk_en(&self) -> I2C_MST_CLK_EN_R {
        I2C_MST_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("date", &format_args!("{}", self.date().bits()))
            .field(
                "i2c_mst_clk_en",
                &format_args!("{}", self.i2c_mst_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:27 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<DATE_SPEC> {
        DATE_W::new(self, 0)
    }
    #[doc = "Bit 28 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mst_clk_en(&mut self) -> I2C_MST_CLK_EN_W<DATE_SPEC> {
        I2C_MST_CLK_EN_W::new(self, 28)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets DATE to value 0x0220_1300"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x0220_1300;
}
