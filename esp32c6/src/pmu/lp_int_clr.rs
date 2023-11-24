#[doc = "Register `LP_INT_CLR` writer"]
pub type W = crate::W<LP_INT_CLR_SPEC>;
#[doc = "Field `LP_CPU_WAKEUP_INT_CLR` writer - need_des"]
pub type LP_CPU_WAKEUP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_SWITCH_ACTIVE_END_INT_CLR` writer - need_des"]
pub type MODEM_SWITCH_ACTIVE_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_END_INT_CLR` writer - need_des"]
pub type SLEEP_SWITCH_ACTIVE_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_SWITCH_MODEM_END_INT_CLR` writer - need_des"]
pub type SLEEP_SWITCH_MODEM_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_SWITCH_SLEEP_END_INT_CLR` writer - need_des"]
pub type MODEM_SWITCH_SLEEP_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_END_INT_CLR` writer - need_des"]
pub type ACTIVE_SWITCH_SLEEP_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_SWITCH_ACTIVE_START_INT_CLR` writer - need_des"]
pub type MODEM_SWITCH_ACTIVE_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_START_INT_CLR` writer - need_des"]
pub type SLEEP_SWITCH_ACTIVE_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_SWITCH_MODEM_START_INT_CLR` writer - need_des"]
pub type SLEEP_SWITCH_MODEM_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_SWITCH_SLEEP_START_INT_CLR` writer - need_des"]
pub type MODEM_SWITCH_SLEEP_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_START_INT_CLR` writer - need_des"]
pub type ACTIVE_SWITCH_SLEEP_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SW_TRIGGER_INT_CLR` writer - need_des"]
pub type HP_SW_TRIGGER_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_wakeup_int_clr(&mut self) -> LP_CPU_WAKEUP_INT_CLR_W<LP_INT_CLR_SPEC> {
        LP_CPU_WAKEUP_INT_CLR_W::new(self, 20)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_active_end_int_clr(
        &mut self,
    ) -> MODEM_SWITCH_ACTIVE_END_INT_CLR_W<LP_INT_CLR_SPEC> {
        MODEM_SWITCH_ACTIVE_END_INT_CLR_W::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_active_end_int_clr(
        &mut self,
    ) -> SLEEP_SWITCH_ACTIVE_END_INT_CLR_W<LP_INT_CLR_SPEC> {
        SLEEP_SWITCH_ACTIVE_END_INT_CLR_W::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_modem_end_int_clr(
        &mut self,
    ) -> SLEEP_SWITCH_MODEM_END_INT_CLR_W<LP_INT_CLR_SPEC> {
        SLEEP_SWITCH_MODEM_END_INT_CLR_W::new(self, 23)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_sleep_end_int_clr(
        &mut self,
    ) -> MODEM_SWITCH_SLEEP_END_INT_CLR_W<LP_INT_CLR_SPEC> {
        MODEM_SWITCH_SLEEP_END_INT_CLR_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn active_switch_sleep_end_int_clr(
        &mut self,
    ) -> ACTIVE_SWITCH_SLEEP_END_INT_CLR_W<LP_INT_CLR_SPEC> {
        ACTIVE_SWITCH_SLEEP_END_INT_CLR_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_active_start_int_clr(
        &mut self,
    ) -> MODEM_SWITCH_ACTIVE_START_INT_CLR_W<LP_INT_CLR_SPEC> {
        MODEM_SWITCH_ACTIVE_START_INT_CLR_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_active_start_int_clr(
        &mut self,
    ) -> SLEEP_SWITCH_ACTIVE_START_INT_CLR_W<LP_INT_CLR_SPEC> {
        SLEEP_SWITCH_ACTIVE_START_INT_CLR_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_modem_start_int_clr(
        &mut self,
    ) -> SLEEP_SWITCH_MODEM_START_INT_CLR_W<LP_INT_CLR_SPEC> {
        SLEEP_SWITCH_MODEM_START_INT_CLR_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_sleep_start_int_clr(
        &mut self,
    ) -> MODEM_SWITCH_SLEEP_START_INT_CLR_W<LP_INT_CLR_SPEC> {
        MODEM_SWITCH_SLEEP_START_INT_CLR_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn active_switch_sleep_start_int_clr(
        &mut self,
    ) -> ACTIVE_SWITCH_SLEEP_START_INT_CLR_W<LP_INT_CLR_SPEC> {
        ACTIVE_SWITCH_SLEEP_START_INT_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sw_trigger_int_clr(&mut self) -> HP_SW_TRIGGER_INT_CLR_W<LP_INT_CLR_SPEC> {
        HP_SW_TRIGGER_INT_CLR_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_CLR_SPEC;
impl crate::RegisterSpec for LP_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lp_int_clr::W`](W) writer structure"]
impl crate::Writable for LP_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_INT_CLR to value 0"]
impl crate::Resettable for LP_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
