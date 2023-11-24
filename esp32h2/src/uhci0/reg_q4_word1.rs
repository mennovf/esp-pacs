#[doc = "Register `REG_Q4_WORD1` reader"]
pub type R = crate::R<REG_Q4_WORD1_SPEC>;
#[doc = "Register `REG_Q4_WORD1` writer"]
pub type W = crate::W<REG_Q4_WORD1_SPEC>;
#[doc = "Field `SEND_Q4_WORD1` reader - a"]
pub type SEND_Q4_WORD1_R = crate::FieldReader<u32>;
#[doc = "Field `SEND_Q4_WORD1` writer - a"]
pub type SEND_Q4_WORD1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - a"]
    #[inline(always)]
    pub fn send_q4_word1(&self) -> SEND_Q4_WORD1_R {
        SEND_Q4_WORD1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_Q4_WORD1")
            .field(
                "send_q4_word1",
                &format_args!("{}", self.send_q4_word1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REG_Q4_WORD1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - a"]
    #[inline(always)]
    #[must_use]
    pub fn send_q4_word1(&mut self) -> SEND_Q4_WORD1_W<REG_Q4_WORD1_SPEC> {
        SEND_Q4_WORD1_W::new(self, 0)
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
#[doc = "a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_q4_word1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_q4_word1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_Q4_WORD1_SPEC;
impl crate::RegisterSpec for REG_Q4_WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_q4_word1::R`](R) reader structure"]
impl crate::Readable for REG_Q4_WORD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_q4_word1::W`](W) writer structure"]
impl crate::Writable for REG_Q4_WORD1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_Q4_WORD1 to value 0"]
impl crate::Resettable for REG_Q4_WORD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
