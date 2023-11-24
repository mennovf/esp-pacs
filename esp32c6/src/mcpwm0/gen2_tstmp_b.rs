#[doc = "Register `GEN2_TSTMP_B` reader"]
pub type R = crate::R<GEN2_TSTMP_B_SPEC>;
#[doc = "Register `GEN2_TSTMP_B` writer"]
pub type W = crate::W<GEN2_TSTMP_B_SPEC>;
#[doc = "Field `CMPR2_B` reader - PWM generator 2 time stamp B's shadow register"]
pub type CMPR2_B_R = crate::FieldReader<u16>;
#[doc = "Field `CMPR2_B` writer - PWM generator 2 time stamp B's shadow register"]
pub type CMPR2_B_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PWM generator 2 time stamp B's shadow register"]
    #[inline(always)]
    pub fn cmpr2_b(&self) -> CMPR2_B_R {
        CMPR2_B_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN2_TSTMP_B")
            .field("cmpr2_b", &format_args!("{}", self.cmpr2_b().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN2_TSTMP_B_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 2 time stamp B's shadow register"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr2_b(&mut self) -> CMPR2_B_W<GEN2_TSTMP_B_SPEC> {
        CMPR2_B_W::new(self, 0)
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
#[doc = "Shadow register for register B.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen2_tstmp_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen2_tstmp_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN2_TSTMP_B_SPEC;
impl crate::RegisterSpec for GEN2_TSTMP_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen2_tstmp_b::R`](R) reader structure"]
impl crate::Readable for GEN2_TSTMP_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen2_tstmp_b::W`](W) writer structure"]
impl crate::Writable for GEN2_TSTMP_B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GEN2_TSTMP_B to value 0"]
impl crate::Resettable for GEN2_TSTMP_B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
