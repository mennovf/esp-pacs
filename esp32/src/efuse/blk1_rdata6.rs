#[doc = "Register `BLK1_RDATA6` reader"]
pub struct R(crate::R<BLK1_RDATA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK1_RDATA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK1_RDATA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK1_RDATA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_BLOCK1_6` reader - "]
pub type RD_BLOCK1_6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rd_block1_6(&self) -> RD_BLOCK1_6_R {
        RD_BLOCK1_6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK1_RDATA6")
            .field(
                "rd_block1_6",
                &format_args!("{}", self.rd_block1_6().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK1_RDATA6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk1_rdata6](index.html) module"]
pub struct BLK1_RDATA6_SPEC;
impl crate::RegisterSpec for BLK1_RDATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk1_rdata6::R](R) reader structure"]
impl crate::Readable for BLK1_RDATA6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK1_RDATA6 to value 0"]
impl crate::Resettable for BLK1_RDATA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
