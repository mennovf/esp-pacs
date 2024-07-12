#[doc = "Register `CORE_0_RCD_PDEBUGPC` reader"]
pub type R = crate::R<CORE_0_RCD_PDEBUGPC_SPEC>;
#[doc = "Field `CORE_0_RCD_PDEBUGPC` reader - reg_core_0_rcd_pdebugpc"]
pub type CORE_0_RCD_PDEBUGPC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_0_rcd_pdebugpc"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugpc(&self) -> CORE_0_RCD_PDEBUGPC_R {
        CORE_0_RCD_PDEBUGPC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_RCD_PDEBUGPC")
            .field("core_0_rcd_pdebugpc", &self.core_0_rcd_pdebugpc())
            .finish()
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_rcd_pdebugpc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_RCD_PDEBUGPC_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_PDEBUGPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_rcd_pdebugpc::R`](R) reader structure"]
impl crate::Readable for CORE_0_RCD_PDEBUGPC_SPEC {}
#[doc = "`reset()` method sets CORE_0_RCD_PDEBUGPC to value 0"]
impl crate::Resettable for CORE_0_RCD_PDEBUGPC_SPEC {
    const RESET_VALUE: u32 = 0;
}
