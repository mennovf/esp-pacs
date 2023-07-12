#[doc = "Register `BLK2_RDATA4` reader"]
pub struct R(crate::R<BLK2_RDATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK2_RDATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK2_RDATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK2_RDATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_BLOCK2_4` reader - "]
pub type RD_BLOCK2_4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rd_block2_4(&self) -> RD_BLOCK2_4_R {
        RD_BLOCK2_4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK2_RDATA4")
            .field(
                "rd_block2_4",
                &format_args!("{}", self.rd_block2_4().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK2_RDATA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk2_rdata4](index.html) module"]
pub struct BLK2_RDATA4_SPEC;
impl crate::RegisterSpec for BLK2_RDATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk2_rdata4::R](R) reader structure"]
impl crate::Readable for BLK2_RDATA4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK2_RDATA4 to value 0"]
impl crate::Resettable for BLK2_RDATA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
