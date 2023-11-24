#[doc = "Register `SCL_STOP_PERIOD` reader"]
pub type R = crate::R<SCL_STOP_PERIOD_SPEC>;
#[doc = "Register `SCL_STOP_PERIOD` writer"]
pub type W = crate::W<SCL_STOP_PERIOD_SPEC>;
#[doc = "Field `SCL_STOP_PERIOD` reader - time period for SCL to stop after I2C end is triggered"]
pub type SCL_STOP_PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `SCL_STOP_PERIOD` writer - time period for SCL to stop after I2C end is triggered"]
pub type SCL_STOP_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - time period for SCL to stop after I2C end is triggered"]
    #[inline(always)]
    pub fn scl_stop_period(&self) -> SCL_STOP_PERIOD_R {
        SCL_STOP_PERIOD_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_STOP_PERIOD")
            .field(
                "scl_stop_period",
                &format_args!("{}", self.scl_stop_period().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_STOP_PERIOD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - time period for SCL to stop after I2C end is triggered"]
    #[inline(always)]
    #[must_use]
    pub fn scl_stop_period(&mut self) -> SCL_STOP_PERIOD_W<SCL_STOP_PERIOD_SPEC> {
        SCL_STOP_PERIOD_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "configure scl stop period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_period::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_period::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_STOP_PERIOD_SPEC;
impl crate::RegisterSpec for SCL_STOP_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_stop_period::R`](R) reader structure"]
impl crate::Readable for SCL_STOP_PERIOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_stop_period::W`](W) writer structure"]
impl crate::Writable for SCL_STOP_PERIOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCL_STOP_PERIOD to value 0x08"]
impl crate::Resettable for SCL_STOP_PERIOD_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
