#[doc = "Register `SLC_RXLINK_DSCR` reader"]
pub type R = crate::R<SLC_RXLINK_DSCR_SPEC>;
#[doc = "Register `SLC_RXLINK_DSCR` writer"]
pub type W = crate::W<SLC_RXLINK_DSCR_SPEC>;
#[doc = "Field `Register` reader - "]
pub type REGISTER_R = crate::FieldReader<u32>;
#[doc = "Field `Register` writer - "]
pub type REGISTER_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn register(&self) -> REGISTER_R {
        REGISTER_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_RXLINK_DSCR")
            .field("register", &format_args!("{}", self.register().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_RXLINK_DSCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn register(&mut self) -> REGISTER_W<SLC_RXLINK_DSCR_SPEC> {
        REGISTER_W::new(self, 0)
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
#[doc = "SLC_RXLINK_DSCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rxlink_dscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rxlink_dscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_RXLINK_DSCR_SPEC;
impl crate::RegisterSpec for SLC_RXLINK_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_rxlink_dscr::R`](R) reader structure"]
impl crate::Readable for SLC_RXLINK_DSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_rxlink_dscr::W`](W) writer structure"]
impl crate::Writable for SLC_RXLINK_DSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_RXLINK_DSCR to value 0"]
impl crate::Resettable for SLC_RXLINK_DSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
