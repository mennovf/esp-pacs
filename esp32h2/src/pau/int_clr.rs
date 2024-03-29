#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `DONE` writer - backup done flag"]
pub type DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERROR` writer - error flag"]
pub type ERROR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - backup done flag"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<INT_CLR_SPEC> {
        DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - error flag"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<INT_CLR_SPEC> {
        ERROR_W::new(self, 1)
    }
}
#[doc = "Read only register for error and done\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
