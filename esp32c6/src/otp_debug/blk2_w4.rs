#[doc = "Register `BLK2_W4` reader"]
pub type R = crate::R<BLK2_W4_SPEC>;
#[doc = "Field `BLOCK2_W4` reader - Otp block2 word4 data."]
pub type BLOCK2_W4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block2 word4 data."]
    #[inline(always)]
    pub fn block2_w4(&self) -> BLOCK2_W4_R {
        BLOCK2_W4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK2_W4")
            .field("block2_w4", &format_args!("{}", self.block2_w4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK2_W4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Otp debuger block2 data register4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_w4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK2_W4_SPEC;
impl crate::RegisterSpec for BLK2_W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk2_w4::R`](R) reader structure"]
impl crate::Readable for BLK2_W4_SPEC {}
#[doc = "`reset()` method sets BLK2_W4 to value 0"]
impl crate::Resettable for BLK2_W4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
