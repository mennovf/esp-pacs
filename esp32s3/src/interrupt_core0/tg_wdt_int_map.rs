#[doc = "Register `TG_WDT_INT_MAP` reader"]
pub type R = crate::R<TG_WDT_INT_MAP_SPEC>;
#[doc = "Register `TG_WDT_INT_MAP` writer"]
pub type W = crate::W<TG_WDT_INT_MAP_SPEC>;
#[doc = "Field `TG_WDT_INT_MAP` reader - this register used to map rg_wdt interrupt to one of core0's external interrupt"]
pub type TG_WDT_INT_MAP_R = crate::FieldReader;
#[doc = "Field `TG_WDT_INT_MAP` writer - this register used to map rg_wdt interrupt to one of core0's external interrupt"]
pub type TG_WDT_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map rg_wdt interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn tg_wdt_int_map(&self) -> TG_WDT_INT_MAP_R {
        TG_WDT_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TG_WDT_INT_MAP")
            .field("tg_wdt_int_map", &self.tg_wdt_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map rg_wdt interrupt to one of core0's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tg_wdt_int_map(&mut self) -> TG_WDT_INT_MAP_W<TG_WDT_INT_MAP_SPEC> {
        TG_WDT_INT_MAP_W::new(self, 0)
    }
}
#[doc = "tg_wdt interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg_wdt_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg_wdt_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TG_WDT_INT_MAP_SPEC;
impl crate::RegisterSpec for TG_WDT_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tg_wdt_int_map::R`](R) reader structure"]
impl crate::Readable for TG_WDT_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tg_wdt_int_map::W`](W) writer structure"]
impl crate::Writable for TG_WDT_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TG_WDT_INT_MAP to value 0x10"]
impl crate::Resettable for TG_WDT_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
