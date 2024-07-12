#[doc = "Register `LEDC_INT_MAP` reader"]
pub type R = crate::R<LEDC_INT_MAP_SPEC>;
#[doc = "Register `LEDC_INT_MAP` writer"]
pub type W = crate::W<LEDC_INT_MAP_SPEC>;
#[doc = "Field `LEDC_INT_MAP` reader - reg_core0_ledc_int_map"]
pub type LEDC_INT_MAP_R = crate::FieldReader;
#[doc = "Field `LEDC_INT_MAP` writer - reg_core0_ledc_int_map"]
pub type LEDC_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_ledc_int_map"]
    #[inline(always)]
    pub fn ledc_int_map(&self) -> LEDC_INT_MAP_R {
        LEDC_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LEDC_INT_MAP")
            .field("ledc_int_map", &self.ledc_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_ledc_int_map"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_int_map(&mut self) -> LEDC_INT_MAP_W<LEDC_INT_MAP_SPEC> {
        LEDC_INT_MAP_W::new(self, 0)
    }
}
#[doc = "ledc intr map register\n\nYou can [`read`](crate::Reg::read) this register and get [`ledc_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LEDC_INT_MAP_SPEC;
impl crate::RegisterSpec for LEDC_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ledc_int_map::R`](R) reader structure"]
impl crate::Readable for LEDC_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ledc_int_map::W`](W) writer structure"]
impl crate::Writable for LEDC_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LEDC_INT_MAP to value 0"]
impl crate::Resettable for LEDC_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
