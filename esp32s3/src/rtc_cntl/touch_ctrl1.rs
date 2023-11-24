#[doc = "Register `TOUCH_CTRL1` reader"]
pub type R = crate::R<TOUCH_CTRL1_SPEC>;
#[doc = "Register `TOUCH_CTRL1` writer"]
pub type W = crate::W<TOUCH_CTRL1_SPEC>;
#[doc = "Field `TOUCH_SLEEP_CYCLES` reader - sleep cycles for timer"]
pub type TOUCH_SLEEP_CYCLES_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_SLEEP_CYCLES` writer - sleep cycles for timer"]
pub type TOUCH_SLEEP_CYCLES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_MEAS_NUM` reader - the meas length (in 8MHz)"]
pub type TOUCH_MEAS_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_MEAS_NUM` writer - the meas length (in 8MHz)"]
pub type TOUCH_MEAS_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sleep cycles for timer"]
    #[inline(always)]
    pub fn touch_sleep_cycles(&self) -> TOUCH_SLEEP_CYCLES_R {
        TOUCH_SLEEP_CYCLES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the meas length (in 8MHz)"]
    #[inline(always)]
    pub fn touch_meas_num(&self) -> TOUCH_MEAS_NUM_R {
        TOUCH_MEAS_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_CTRL1")
            .field(
                "touch_sleep_cycles",
                &format_args!("{}", self.touch_sleep_cycles().bits()),
            )
            .field(
                "touch_meas_num",
                &format_args!("{}", self.touch_meas_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - sleep cycles for timer"]
    #[inline(always)]
    #[must_use]
    pub fn touch_sleep_cycles(&mut self) -> TOUCH_SLEEP_CYCLES_W<TOUCH_CTRL1_SPEC> {
        TOUCH_SLEEP_CYCLES_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the meas length (in 8MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn touch_meas_num(&mut self) -> TOUCH_MEAS_NUM_W<TOUCH_CTRL1_SPEC> {
        TOUCH_MEAS_NUM_W::new(self, 16)
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
#[doc = "configure touch controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_CTRL1_SPEC;
impl crate::RegisterSpec for TOUCH_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_ctrl1::R`](R) reader structure"]
impl crate::Readable for TOUCH_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_ctrl1::W`](W) writer structure"]
impl crate::Writable for TOUCH_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_CTRL1 to value 0x1000_0100"]
impl crate::Resettable for TOUCH_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0100;
}
