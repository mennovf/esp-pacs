#[doc = "Register `WDTWPROTECT` reader"]
pub type R = crate::R<WDTWPROTECT_SPEC>;
#[doc = "Register `WDTWPROTECT` writer"]
pub type W = crate::W<WDTWPROTECT_SPEC>;
#[doc = "Field `WDT_WKEY` reader - reg_wdt_wkey."]
pub type WDT_WKEY_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_WKEY` writer - reg_wdt_wkey."]
pub type WDT_WKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_wdt_wkey."]
    #[inline(always)]
    pub fn wdt_wkey(&self) -> WDT_WKEY_R {
        WDT_WKEY_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTWPROTECT")
            .field("wdt_wkey", &self.wdt_wkey())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_wdt_wkey."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_wkey(&mut self) -> WDT_WKEY_W<WDTWPROTECT_SPEC> {
        WDT_WKEY_W::new(self, 0)
    }
}
#[doc = "TIMG_WDTWPROTECT_REG.\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtwprotect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtwprotect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTWPROTECT_SPEC;
impl crate::RegisterSpec for WDTWPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtwprotect::R`](R) reader structure"]
impl crate::Readable for WDTWPROTECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtwprotect::W`](W) writer structure"]
impl crate::Writable for WDTWPROTECT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTWPROTECT to value 0x50d8_3aa1"]
impl crate::Resettable for WDTWPROTECT_SPEC {
    const RESET_VALUE: u32 = 0x50d8_3aa1;
}
