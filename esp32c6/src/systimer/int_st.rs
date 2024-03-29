#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `TARGET0` reader - interupt0 status"]
pub type TARGET0_R = crate::BitReader;
#[doc = "Field `TARGET1` reader - interupt1 status"]
pub type TARGET1_R = crate::BitReader;
#[doc = "Field `TARGET2` reader - interupt2 status"]
pub type TARGET2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - interupt0 status"]
    #[inline(always)]
    pub fn target0(&self) -> TARGET0_R {
        TARGET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interupt1 status"]
    #[inline(always)]
    pub fn target1(&self) -> TARGET1_R {
        TARGET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interupt2 status"]
    #[inline(always)]
    pub fn target2(&self) -> TARGET2_R {
        TARGET2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("target0", &format_args!("{}", self.target0().bit()))
            .field("target1", &format_args!("{}", self.target1().bit()))
            .field("target2", &format_args!("{}", self.target2().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "systimer interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
