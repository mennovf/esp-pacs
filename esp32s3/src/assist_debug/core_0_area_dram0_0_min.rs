#[doc = "Register `CORE_0_AREA_DRAM0_0_MIN` reader"]
pub type R = crate::R<CORE_0_AREA_DRAM0_0_MIN_SPEC>;
#[doc = "Register `CORE_0_AREA_DRAM0_0_MIN` writer"]
pub type W = crate::W<CORE_0_AREA_DRAM0_0_MIN_SPEC>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_MIN` reader - Core0 dram0 region0 start addr"]
pub type CORE_0_AREA_DRAM0_0_MIN_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_MIN` writer - Core0 dram0 region0 start addr"]
pub type CORE_0_AREA_DRAM0_0_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Core0 dram0 region0 start addr"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_min(&self) -> CORE_0_AREA_DRAM0_0_MIN_R {
        CORE_0_AREA_DRAM0_0_MIN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_AREA_DRAM0_0_MIN")
            .field("core_0_area_dram0_0_min", &self.core_0_area_dram0_0_min())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Core0 dram0 region0 start addr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_0_min(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_0_MIN_W<CORE_0_AREA_DRAM0_0_MIN_SPEC> {
        CORE_0_AREA_DRAM0_0_MIN_W::new(self, 0)
    }
}
#[doc = "core0 dram0 region0 addr configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_dram0_0_min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_area_dram0_0_min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_AREA_DRAM0_0_MIN_SPEC;
impl crate::RegisterSpec for CORE_0_AREA_DRAM0_0_MIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_area_dram0_0_min::R`](R) reader structure"]
impl crate::Readable for CORE_0_AREA_DRAM0_0_MIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_area_dram0_0_min::W`](W) writer structure"]
impl crate::Writable for CORE_0_AREA_DRAM0_0_MIN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_0_AREA_DRAM0_0_MIN to value 0xffff_ffff"]
impl crate::Resettable for CORE_0_AREA_DRAM0_0_MIN_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
