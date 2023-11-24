#[doc = "Register `T1LOADHI` reader"]
pub type R = crate::R<T1LOADHI_SPEC>;
#[doc = "Register `T1LOADHI` writer"]
pub type W = crate::W<T1LOADHI_SPEC>;
#[doc = "Field `LOAD_HI` reader - higher 32 bits of the value that will load into timer 1 time-base counter"]
pub type LOAD_HI_R = crate::FieldReader<u32>;
#[doc = "Field `LOAD_HI` writer - higher 32 bits of the value that will load into timer 1 time-base counter"]
pub type LOAD_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - higher 32 bits of the value that will load into timer 1 time-base counter"]
    #[inline(always)]
    pub fn load_hi(&self) -> LOAD_HI_R {
        LOAD_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1LOADHI")
            .field("load_hi", &format_args!("{}", self.load_hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T1LOADHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - higher 32 bits of the value that will load into timer 1 time-base counter"]
    #[inline(always)]
    #[must_use]
    pub fn load_hi(&mut self) -> LOAD_HI_W<T1LOADHI_SPEC> {
        LOAD_HI_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1loadhi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1loadhi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T1LOADHI_SPEC;
impl crate::RegisterSpec for T1LOADHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t1loadhi::R`](R) reader structure"]
impl crate::Readable for T1LOADHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t1loadhi::W`](W) writer structure"]
impl crate::Writable for T1LOADHI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T1LOADHI to value 0"]
impl crate::Resettable for T1LOADHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
