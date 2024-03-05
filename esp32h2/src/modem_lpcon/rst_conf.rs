#[doc = "Register `RST_CONF` writer"]
pub type W = crate::W<RST_CONF_SPEC>;
#[doc = "Field `RST_COEX` writer - "]
pub type RST_COEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_I2C_MST` writer - "]
pub type RST_I2C_MST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RST_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rst_coex(&mut self) -> RST_COEX_W<RST_CONF_SPEC> {
        RST_COEX_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rst_i2c_mst(&mut self) -> RST_I2C_MST_W<RST_CONF_SPEC> {
        RST_I2C_MST_W::new(self, 2)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst_conf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST_CONF_SPEC;
impl crate::RegisterSpec for RST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rst_conf::W`](W) writer structure"]
impl crate::Writable for RST_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST_CONF to value 0"]
impl crate::Resettable for RST_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
