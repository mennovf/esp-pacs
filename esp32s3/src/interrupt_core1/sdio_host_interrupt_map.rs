#[doc = "Register `SDIO_HOST_INTERRUPT_MAP` reader"]
pub type R = crate::R<SDIO_HOST_INTERRUPT_MAP_SPEC>;
#[doc = "Register `SDIO_HOST_INTERRUPT_MAP` writer"]
pub type W = crate::W<SDIO_HOST_INTERRUPT_MAP_SPEC>;
#[doc = "Field `SDIO_HOST_INTERRUPT_MAP` reader - this register used to map sdio_host interrupt to one of core1's external interrupt"]
pub type SDIO_HOST_INTERRUPT_MAP_R = crate::FieldReader;
#[doc = "Field `SDIO_HOST_INTERRUPT_MAP` writer - this register used to map sdio_host interrupt to one of core1's external interrupt"]
pub type SDIO_HOST_INTERRUPT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map sdio_host interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn sdio_host_interrupt_map(&self) -> SDIO_HOST_INTERRUPT_MAP_R {
        SDIO_HOST_INTERRUPT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_HOST_INTERRUPT_MAP")
            .field(
                "sdio_host_interrupt_map",
                &format_args!("{}", self.sdio_host_interrupt_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_HOST_INTERRUPT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map sdio_host interrupt to one of core1's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_host_interrupt_map(
        &mut self,
    ) -> SDIO_HOST_INTERRUPT_MAP_W<SDIO_HOST_INTERRUPT_MAP_SPEC> {
        SDIO_HOST_INTERRUPT_MAP_W::new(self, 0)
    }
}
#[doc = "sdio_host interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_host_interrupt_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_host_interrupt_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_HOST_INTERRUPT_MAP_SPEC;
impl crate::RegisterSpec for SDIO_HOST_INTERRUPT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_host_interrupt_map::R`](R) reader structure"]
impl crate::Readable for SDIO_HOST_INTERRUPT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_host_interrupt_map::W`](W) writer structure"]
impl crate::Writable for SDIO_HOST_INTERRUPT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIO_HOST_INTERRUPT_MAP to value 0x10"]
impl crate::Resettable for SDIO_HOST_INTERRUPT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
