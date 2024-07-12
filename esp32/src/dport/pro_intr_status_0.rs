#[doc = "Register `PRO_INTR_STATUS_0` reader"]
pub type R = crate::R<PRO_INTR_STATUS_0_SPEC>;
#[doc = "Field `PRO_INTR_STATUS_0` reader - "]
pub type PRO_INTR_STATUS_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pro_intr_status_0(&self) -> PRO_INTR_STATUS_0_R {
        PRO_INTR_STATUS_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_INTR_STATUS_0")
            .field("pro_intr_status_0", &self.pro_intr_status_0())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_intr_status_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_INTR_STATUS_0_SPEC;
impl crate::RegisterSpec for PRO_INTR_STATUS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_intr_status_0::R`](R) reader structure"]
impl crate::Readable for PRO_INTR_STATUS_0_SPEC {}
#[doc = "`reset()` method sets PRO_INTR_STATUS_0 to value 0"]
impl crate::Resettable for PRO_INTR_STATUS_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
