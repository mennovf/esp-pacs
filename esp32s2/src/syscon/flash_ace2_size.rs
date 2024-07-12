#[doc = "Register `FLASH_ACE2_SIZE` reader"]
pub type R = crate::R<FLASH_ACE2_SIZE_SPEC>;
#[doc = "Register `FLASH_ACE2_SIZE` writer"]
pub type W = crate::W<FLASH_ACE2_SIZE_SPEC>;
#[doc = "Field `FLASH_ACE2_SIZE` reader - "]
pub type FLASH_ACE2_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `FLASH_ACE2_SIZE` writer - "]
pub type FLASH_ACE2_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn flash_ace2_size(&self) -> FLASH_ACE2_SIZE_R {
        FLASH_ACE2_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_ACE2_SIZE")
            .field("flash_ace2_size", &self.flash_ace2_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn flash_ace2_size(&mut self) -> FLASH_ACE2_SIZE_W<FLASH_ACE2_SIZE_SPEC> {
        FLASH_ACE2_SIZE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_ace2_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_ace2_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_ACE2_SIZE_SPEC;
impl crate::RegisterSpec for FLASH_ACE2_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_ace2_size::R`](R) reader structure"]
impl crate::Readable for FLASH_ACE2_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_ace2_size::W`](W) writer structure"]
impl crate::Writable for FLASH_ACE2_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_ACE2_SIZE to value 0x1000"]
impl crate::Resettable for FLASH_ACE2_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
