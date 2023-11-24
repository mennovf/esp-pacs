#[doc = "Register `SRAM_ACE2_ATTR` reader"]
pub type R = crate::R<SRAM_ACE2_ATTR_SPEC>;
#[doc = "Register `SRAM_ACE2_ATTR` writer"]
pub type W = crate::W<SRAM_ACE2_ATTR_SPEC>;
#[doc = "Field `SRAM_ACE2_ATTR` reader - "]
pub type SRAM_ACE2_ATTR_R = crate::FieldReader;
#[doc = "Field `SRAM_ACE2_ATTR` writer - "]
pub type SRAM_ACE2_ATTR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sram_ace2_attr(&self) -> SRAM_ACE2_ATTR_R {
        SRAM_ACE2_ATTR_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_ACE2_ATTR")
            .field(
                "sram_ace2_attr",
                &format_args!("{}", self.sram_ace2_attr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_ACE2_ATTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn sram_ace2_attr(&mut self) -> SRAM_ACE2_ATTR_W<SRAM_ACE2_ATTR_SPEC> {
        SRAM_ACE2_ATTR_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace2_attr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace2_attr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_ACE2_ATTR_SPEC;
impl crate::RegisterSpec for SRAM_ACE2_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ace2_attr::R`](R) reader structure"]
impl crate::Readable for SRAM_ACE2_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_ace2_attr::W`](W) writer structure"]
impl crate::Writable for SRAM_ACE2_ATTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_ACE2_ATTR to value 0x07"]
impl crate::Resettable for SRAM_ACE2_ATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
