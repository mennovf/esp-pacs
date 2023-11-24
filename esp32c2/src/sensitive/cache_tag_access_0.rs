#[doc = "Register `CACHE_TAG_ACCESS_0` reader"]
pub type R = crate::R<CACHE_TAG_ACCESS_0_SPEC>;
#[doc = "Register `CACHE_TAG_ACCESS_0` writer"]
pub type W = crate::W<CACHE_TAG_ACCESS_0_SPEC>;
#[doc = "Field `CACHE_TAG_ACCESS_LOCK` reader - Need add description"]
pub type CACHE_TAG_ACCESS_LOCK_R = crate::BitReader;
#[doc = "Field `CACHE_TAG_ACCESS_LOCK` writer - Need add description"]
pub type CACHE_TAG_ACCESS_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    pub fn cache_tag_access_lock(&self) -> CACHE_TAG_ACCESS_LOCK_R {
        CACHE_TAG_ACCESS_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_TAG_ACCESS_0")
            .field(
                "cache_tag_access_lock",
                &format_args!("{}", self.cache_tag_access_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_TAG_ACCESS_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn cache_tag_access_lock(&mut self) -> CACHE_TAG_ACCESS_LOCK_W<CACHE_TAG_ACCESS_0_SPEC> {
        CACHE_TAG_ACCESS_LOCK_W::new(self, 0)
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
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_tag_access_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_tag_access_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_TAG_ACCESS_0_SPEC;
impl crate::RegisterSpec for CACHE_TAG_ACCESS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_tag_access_0::R`](R) reader structure"]
impl crate::Readable for CACHE_TAG_ACCESS_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_tag_access_0::W`](W) writer structure"]
impl crate::Writable for CACHE_TAG_ACCESS_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_TAG_ACCESS_0 to value 0"]
impl crate::Resettable for CACHE_TAG_ACCESS_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
