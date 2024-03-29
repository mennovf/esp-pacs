#[doc = "Register `INT_ST_TIMERS` reader"]
pub type R = crate::R<INT_ST_TIMERS_SPEC>;
#[doc = "Field `T0` reader - interrupt when timer0 alarm"]
pub type T0_R = crate::BitReader;
#[doc = "Field `T1` reader - interrupt when timer1 alarm"]
pub type T1_R = crate::BitReader;
#[doc = "Field `WDT` reader - Interrupt when an interrupt stage timeout"]
pub type WDT_R = crate::BitReader;
#[doc = "Field `LACT` reader - "]
pub type LACT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    pub fn t0(&self) -> T0_R {
        T0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    pub fn t1(&self) -> T1_R {
        T1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lact(&self) -> LACT_R {
        LACT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST_TIMERS")
            .field("t0", &format_args!("{}", self.t0().bit()))
            .field("t1", &format_args!("{}", self.t1().bit()))
            .field("wdt", &format_args!("{}", self.wdt().bit()))
            .field("lact", &format_args!("{}", self.lact().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_timers::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_TIMERS_SPEC;
impl crate::RegisterSpec for INT_ST_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_timers::R`](R) reader structure"]
impl crate::Readable for INT_ST_TIMERS_SPEC {}
#[doc = "`reset()` method sets INT_ST_TIMERS to value 0"]
impl crate::Resettable for INT_ST_TIMERS_SPEC {
    const RESET_VALUE: u32 = 0;
}
