#[doc = "Register `BLK9_W6` reader"]
pub type R = crate::R<BLK9_W6_SPEC>;
#[doc = "Field `BLOCK9_W6` reader - Otp block9 word6 data."]
pub type BLOCK9_W6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block9 word6 data."]
    #[inline(always)]
    pub fn block9_w6(&self) -> BLOCK9_W6_R {
        BLOCK9_W6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK9_W6")
            .field("block9_w6", &format_args!("{}", self.block9_w6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK9_W6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Otp debuger block9 data register6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk9_w6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK9_W6_SPEC;
impl crate::RegisterSpec for BLK9_W6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk9_w6::R`](R) reader structure"]
impl crate::Readable for BLK9_W6_SPEC {}
#[doc = "`reset()` method sets BLK9_W6 to value 0"]
impl crate::Resettable for BLK9_W6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
