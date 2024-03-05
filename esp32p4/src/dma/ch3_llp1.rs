#[doc = "Register `CH3_LLP1` reader"]
pub type R = crate::R<CH3_LLP1_SPEC>;
#[doc = "Register `CH3_LLP1` writer"]
pub type W = crate::W<CH3_LLP1_SPEC>;
#[doc = "Field `CH3_LOC1` reader - NA"]
pub type CH3_LOC1_R = crate::FieldReader<u32>;
#[doc = "Field `CH3_LOC1` writer - NA"]
pub type CH3_LOC1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch3_loc1(&self) -> CH3_LOC1_R {
        CH3_LOC1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH3_LLP1")
            .field("ch3_loc1", &format_args!("{}", self.ch3_loc1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH3_LLP1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_loc1(&mut self) -> CH3_LOC1_W<CH3_LLP1_SPEC> {
        CH3_LOC1_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_llp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_llp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH3_LLP1_SPEC;
impl crate::RegisterSpec for CH3_LLP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_llp1::R`](R) reader structure"]
impl crate::Readable for CH3_LLP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch3_llp1::W`](W) writer structure"]
impl crate::Writable for CH3_LLP1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH3_LLP1 to value 0"]
impl crate::Resettable for CH3_LLP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
