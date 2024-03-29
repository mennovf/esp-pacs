#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `TARGET0` reader - interupt0 raw"]
pub type TARGET0_R = crate::BitReader;
#[doc = "Field `TARGET0` writer - interupt0 raw"]
pub type TARGET0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET1` reader - interupt1 raw"]
pub type TARGET1_R = crate::BitReader;
#[doc = "Field `TARGET1` writer - interupt1 raw"]
pub type TARGET1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET2` reader - interupt2 raw"]
pub type TARGET2_R = crate::BitReader;
#[doc = "Field `TARGET2` writer - interupt2 raw"]
pub type TARGET2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - interupt0 raw"]
    #[inline(always)]
    pub fn target0(&self) -> TARGET0_R {
        TARGET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interupt1 raw"]
    #[inline(always)]
    pub fn target1(&self) -> TARGET1_R {
        TARGET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interupt2 raw"]
    #[inline(always)]
    pub fn target2(&self) -> TARGET2_R {
        TARGET2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("target0", &format_args!("{}", self.target0().bit()))
            .field("target1", &format_args!("{}", self.target1().bit()))
            .field("target2", &format_args!("{}", self.target2().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - interupt0 raw"]
    #[inline(always)]
    #[must_use]
    pub fn target0(&mut self) -> TARGET0_W<INT_RAW_SPEC> {
        TARGET0_W::new(self, 0)
    }
    #[doc = "Bit 1 - interupt1 raw"]
    #[inline(always)]
    #[must_use]
    pub fn target1(&mut self) -> TARGET1_W<INT_RAW_SPEC> {
        TARGET1_W::new(self, 1)
    }
    #[doc = "Bit 2 - interupt2 raw"]
    #[inline(always)]
    #[must_use]
    pub fn target2(&mut self) -> TARGET2_W<INT_RAW_SPEC> {
        TARGET2_W::new(self, 2)
    }
}
#[doc = "systimer interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
