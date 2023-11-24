#[doc = "Register `TARGET2_CONF` reader"]
pub type R = crate::R<TARGET2_CONF_SPEC>;
#[doc = "Register `TARGET2_CONF` writer"]
pub type W = crate::W<TARGET2_CONF_SPEC>;
#[doc = "Field `TARGET2_PERIOD` reader - Set alarm period for system timer target 2, only valid in periodic alarms mode."]
pub type TARGET2_PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `TARGET2_PERIOD` writer - Set alarm period for system timer target 2, only valid in periodic alarms mode."]
pub type TARGET2_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `TARGET2_PERIOD_MODE` reader - Set work mode for system timer target 2. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
pub type TARGET2_PERIOD_MODE_R = crate::BitReader;
#[doc = "Field `TARGET2_PERIOD_MODE` writer - Set work mode for system timer target 2. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
pub type TARGET2_PERIOD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET2_WORK_EN` reader - System timer target 2 work enable."]
pub type TARGET2_WORK_EN_R = crate::BitReader;
#[doc = "Field `TARGET2_WORK_EN` writer - System timer target 2 work enable."]
pub type TARGET2_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:29 - Set alarm period for system timer target 2, only valid in periodic alarms mode."]
    #[inline(always)]
    pub fn target2_period(&self) -> TARGET2_PERIOD_R {
        TARGET2_PERIOD_R::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bit 30 - Set work mode for system timer target 2. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
    #[inline(always)]
    pub fn target2_period_mode(&self) -> TARGET2_PERIOD_MODE_R {
        TARGET2_PERIOD_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - System timer target 2 work enable."]
    #[inline(always)]
    pub fn target2_work_en(&self) -> TARGET2_WORK_EN_R {
        TARGET2_WORK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET2_CONF")
            .field(
                "target2_period",
                &format_args!("{}", self.target2_period().bits()),
            )
            .field(
                "target2_period_mode",
                &format_args!("{}", self.target2_period_mode().bit()),
            )
            .field(
                "target2_work_en",
                &format_args!("{}", self.target2_work_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TARGET2_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:29 - Set alarm period for system timer target 2, only valid in periodic alarms mode."]
    #[inline(always)]
    #[must_use]
    pub fn target2_period(&mut self) -> TARGET2_PERIOD_W<TARGET2_CONF_SPEC> {
        TARGET2_PERIOD_W::new(self, 0)
    }
    #[doc = "Bit 30 - Set work mode for system timer target 2. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
    #[inline(always)]
    #[must_use]
    pub fn target2_period_mode(&mut self) -> TARGET2_PERIOD_MODE_W<TARGET2_CONF_SPEC> {
        TARGET2_PERIOD_MODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - System timer target 2 work enable."]
    #[inline(always)]
    #[must_use]
    pub fn target2_work_en(&mut self) -> TARGET2_WORK_EN_W<TARGET2_CONF_SPEC> {
        TARGET2_WORK_EN_W::new(self, 31)
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
#[doc = "Configure work mode for system timer target 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target2_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target2_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET2_CONF_SPEC;
impl crate::RegisterSpec for TARGET2_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target2_conf::R`](R) reader structure"]
impl crate::Readable for TARGET2_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target2_conf::W`](W) writer structure"]
impl crate::Writable for TARGET2_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGET2_CONF to value 0"]
impl crate::Resettable for TARGET2_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
