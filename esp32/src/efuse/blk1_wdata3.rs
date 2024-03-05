#[doc = "Register `BLK1_WDATA3` reader"]
pub type R = crate::R<BLK1_WDATA3_SPEC>;
#[doc = "Register `BLK1_WDATA3` writer"]
pub type W = crate::W<BLK1_WDATA3_SPEC>;
#[doc = "Field `BLK1_DIN3` reader - "]
pub type BLK1_DIN3_R = crate::FieldReader<u32>;
#[doc = "Field `BLK1_DIN3` writer - "]
pub type BLK1_DIN3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn blk1_din3(&self) -> BLK1_DIN3_R {
        BLK1_DIN3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK1_WDATA3")
            .field("blk1_din3", &format_args!("{}", self.blk1_din3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK1_WDATA3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn blk1_din3(&mut self) -> BLK1_DIN3_W<BLK1_WDATA3_SPEC> {
        BLK1_DIN3_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_wdata3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk1_wdata3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK1_WDATA3_SPEC;
impl crate::RegisterSpec for BLK1_WDATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk1_wdata3::R`](R) reader structure"]
impl crate::Readable for BLK1_WDATA3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk1_wdata3::W`](W) writer structure"]
impl crate::Writable for BLK1_WDATA3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK1_WDATA3 to value 0"]
impl crate::Resettable for BLK1_WDATA3_SPEC {
    const RESET_VALUE: u32 = 0;
}
