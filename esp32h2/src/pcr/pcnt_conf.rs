#[doc = "Register `PCNT_CONF` reader"]
pub type R = crate::R<PCNT_CONF_SPEC>;
#[doc = "Register `PCNT_CONF` writer"]
pub type W = crate::W<PCNT_CONF_SPEC>;
#[doc = "Field `PCNT_CLK_EN` reader - Set 1 to enable pcnt clock"]
pub type PCNT_CLK_EN_R = crate::BitReader;
#[doc = "Field `PCNT_CLK_EN` writer - Set 1 to enable pcnt clock"]
pub type PCNT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT_RST_EN` reader - Set 0 to reset pcnt module"]
pub type PCNT_RST_EN_R = crate::BitReader;
#[doc = "Field `PCNT_RST_EN` writer - Set 0 to reset pcnt module"]
pub type PCNT_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT_READY` reader - Query this field after reset pcnt module"]
pub type PCNT_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable pcnt clock"]
    #[inline(always)]
    pub fn pcnt_clk_en(&self) -> PCNT_CLK_EN_R {
        PCNT_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset pcnt module"]
    #[inline(always)]
    pub fn pcnt_rst_en(&self) -> PCNT_RST_EN_R {
        PCNT_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset pcnt module"]
    #[inline(always)]
    pub fn pcnt_ready(&self) -> PCNT_READY_R {
        PCNT_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNT_CONF")
            .field("pcnt_clk_en", &format_args!("{}", self.pcnt_clk_en().bit()))
            .field("pcnt_rst_en", &format_args!("{}", self.pcnt_rst_en().bit()))
            .field("pcnt_ready", &format_args!("{}", self.pcnt_ready().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PCNT_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable pcnt clock"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt_clk_en(&mut self) -> PCNT_CLK_EN_W<PCNT_CONF_SPEC> {
        PCNT_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset pcnt module"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt_rst_en(&mut self) -> PCNT_RST_EN_W<PCNT_CONF_SPEC> {
        PCNT_RST_EN_W::new(self, 1)
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
#[doc = "PCNT configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcnt_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcnt_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCNT_CONF_SPEC;
impl crate::RegisterSpec for PCNT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcnt_conf::R`](R) reader structure"]
impl crate::Readable for PCNT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcnt_conf::W`](W) writer structure"]
impl crate::Writable for PCNT_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCNT_CONF to value 0x05"]
impl crate::Resettable for PCNT_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
