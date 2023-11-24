#[doc = "Register `DMA_IN_DSCR_BF0` reader"]
pub type R = crate::R<DMA_IN_DSCR_BF0_SPEC>;
#[doc = "Field `INLINK_DSCR_BF0` reader - The content of current in link descriptor's first dword"]
pub type INLINK_DSCR_BF0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The content of current in link descriptor's first dword"]
    #[inline(always)]
    pub fn inlink_dscr_bf0(&self) -> INLINK_DSCR_BF0_R {
        INLINK_DSCR_BF0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_IN_DSCR_BF0")
            .field(
                "inlink_dscr_bf0",
                &format_args!("{}", self.inlink_dscr_bf0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_IN_DSCR_BF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_dscr_bf0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_IN_DSCR_BF0_SPEC;
impl crate::RegisterSpec for DMA_IN_DSCR_BF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_in_dscr_bf0::R`](R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_BF0_SPEC {}
#[doc = "`reset()` method sets DMA_IN_DSCR_BF0 to value 0"]
impl crate::Resettable for DMA_IN_DSCR_BF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
