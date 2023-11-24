#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `TIMER0_STOP_INT_RAW` reader - "]
pub type TIMER0_STOP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER1_STOP_INT_RAW` reader - "]
pub type TIMER1_STOP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER2_STOP_INT_RAW` reader - "]
pub type TIMER2_STOP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER0_TEZ_INT_RAW` reader - "]
pub type TIMER0_TEZ_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER1_TEZ_INT_RAW` reader - "]
pub type TIMER1_TEZ_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER2_TEZ_INT_RAW` reader - "]
pub type TIMER2_TEZ_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER0_TEP_INT_RAW` reader - "]
pub type TIMER0_TEP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER1_TEP_INT_RAW` reader - "]
pub type TIMER1_TEP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIMER2_TEP_INT_RAW` reader - "]
pub type TIMER2_TEP_INT_RAW_R = crate::BitReader;
#[doc = "Field `FAULT0_INT_RAW` reader - "]
pub type FAULT0_INT_RAW_R = crate::BitReader;
#[doc = "Field `FAULT1_INT_RAW` reader - "]
pub type FAULT1_INT_RAW_R = crate::BitReader;
#[doc = "Field `FAULT2_INT_RAW` reader - "]
pub type FAULT2_INT_RAW_R = crate::BitReader;
#[doc = "Field `FAULT0_CLR_INT_RAW` reader - "]
pub type FAULT0_CLR_INT_RAW_R = crate::BitReader;
#[doc = "Field `FAULT1_CLR_INT_RAW` reader - "]
pub type FAULT1_CLR_INT_RAW_R = crate::BitReader;
#[doc = "Field `FAULT2_CLR_INT_RAW` reader - "]
pub type FAULT2_CLR_INT_RAW_R = crate::BitReader;
#[doc = "Field `OP0_TEA_INT_RAW` reader - "]
pub type OP0_TEA_INT_RAW_R = crate::BitReader;
#[doc = "Field `OP1_TEA_INT_RAW` reader - "]
pub type OP1_TEA_INT_RAW_R = crate::BitReader;
#[doc = "Field `OP2_TEA_INT_RAW` reader - "]
pub type OP2_TEA_INT_RAW_R = crate::BitReader;
#[doc = "Field `OP0_TEB_INT_RAW` reader - "]
pub type OP0_TEB_INT_RAW_R = crate::BitReader;
#[doc = "Field `OP1_TEB_INT_RAW` reader - "]
pub type OP1_TEB_INT_RAW_R = crate::BitReader;
#[doc = "Field `OP2_TEB_INT_RAW` reader - "]
pub type OP2_TEB_INT_RAW_R = crate::BitReader;
#[doc = "Field `FH0_CBC_INT_RAW` reader - "]
pub type FH0_CBC_INT_RAW_R = crate::BitReader;
#[doc = "Field `FH1_CBC_INT_RAW` reader - "]
pub type FH1_CBC_INT_RAW_R = crate::BitReader;
#[doc = "Field `FH2_CBC_INT_RAW` reader - "]
pub type FH2_CBC_INT_RAW_R = crate::BitReader;
#[doc = "Field `FH0_OST_INT_RAW` reader - "]
pub type FH0_OST_INT_RAW_R = crate::BitReader;
#[doc = "Field `FH1_OST_INT_RAW` reader - "]
pub type FH1_OST_INT_RAW_R = crate::BitReader;
#[doc = "Field `FH2_OST_INT_RAW` reader - "]
pub type FH2_OST_INT_RAW_R = crate::BitReader;
#[doc = "Field `CAP0_INT_RAW` reader - "]
pub type CAP0_INT_RAW_R = crate::BitReader;
#[doc = "Field `CAP1_INT_RAW` reader - "]
pub type CAP1_INT_RAW_R = crate::BitReader;
#[doc = "Field `CAP2_INT_RAW` reader - "]
pub type CAP2_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer0_stop_int_raw(&self) -> TIMER0_STOP_INT_RAW_R {
        TIMER0_STOP_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer1_stop_int_raw(&self) -> TIMER1_STOP_INT_RAW_R {
        TIMER1_STOP_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer2_stop_int_raw(&self) -> TIMER2_STOP_INT_RAW_R {
        TIMER2_STOP_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timer0_tez_int_raw(&self) -> TIMER0_TEZ_INT_RAW_R {
        TIMER0_TEZ_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timer1_tez_int_raw(&self) -> TIMER1_TEZ_INT_RAW_R {
        TIMER1_TEZ_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn timer2_tez_int_raw(&self) -> TIMER2_TEZ_INT_RAW_R {
        TIMER2_TEZ_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn timer0_tep_int_raw(&self) -> TIMER0_TEP_INT_RAW_R {
        TIMER0_TEP_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn timer1_tep_int_raw(&self) -> TIMER1_TEP_INT_RAW_R {
        TIMER1_TEP_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn timer2_tep_int_raw(&self) -> TIMER2_TEP_INT_RAW_R {
        TIMER2_TEP_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fault0_int_raw(&self) -> FAULT0_INT_RAW_R {
        FAULT0_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fault1_int_raw(&self) -> FAULT1_INT_RAW_R {
        FAULT1_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn fault2_int_raw(&self) -> FAULT2_INT_RAW_R {
        FAULT2_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fault0_clr_int_raw(&self) -> FAULT0_CLR_INT_RAW_R {
        FAULT0_CLR_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fault1_clr_int_raw(&self) -> FAULT1_CLR_INT_RAW_R {
        FAULT1_CLR_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fault2_clr_int_raw(&self) -> FAULT2_CLR_INT_RAW_R {
        FAULT2_CLR_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn op0_tea_int_raw(&self) -> OP0_TEA_INT_RAW_R {
        OP0_TEA_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn op1_tea_int_raw(&self) -> OP1_TEA_INT_RAW_R {
        OP1_TEA_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn op2_tea_int_raw(&self) -> OP2_TEA_INT_RAW_R {
        OP2_TEA_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn op0_teb_int_raw(&self) -> OP0_TEB_INT_RAW_R {
        OP0_TEB_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn op1_teb_int_raw(&self) -> OP1_TEB_INT_RAW_R {
        OP1_TEB_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn op2_teb_int_raw(&self) -> OP2_TEB_INT_RAW_R {
        OP2_TEB_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn fh0_cbc_int_raw(&self) -> FH0_CBC_INT_RAW_R {
        FH0_CBC_INT_RAW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn fh1_cbc_int_raw(&self) -> FH1_CBC_INT_RAW_R {
        FH1_CBC_INT_RAW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn fh2_cbc_int_raw(&self) -> FH2_CBC_INT_RAW_R {
        FH2_CBC_INT_RAW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fh0_ost_int_raw(&self) -> FH0_OST_INT_RAW_R {
        FH0_OST_INT_RAW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn fh1_ost_int_raw(&self) -> FH1_OST_INT_RAW_R {
        FH1_OST_INT_RAW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn fh2_ost_int_raw(&self) -> FH2_OST_INT_RAW_R {
        FH2_OST_INT_RAW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cap0_int_raw(&self) -> CAP0_INT_RAW_R {
        CAP0_INT_RAW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cap1_int_raw(&self) -> CAP1_INT_RAW_R {
        CAP1_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cap2_int_raw(&self) -> CAP2_INT_RAW_R {
        CAP2_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "timer0_stop_int_raw",
                &format_args!("{}", self.timer0_stop_int_raw().bit()),
            )
            .field(
                "timer1_stop_int_raw",
                &format_args!("{}", self.timer1_stop_int_raw().bit()),
            )
            .field(
                "timer2_stop_int_raw",
                &format_args!("{}", self.timer2_stop_int_raw().bit()),
            )
            .field(
                "timer0_tez_int_raw",
                &format_args!("{}", self.timer0_tez_int_raw().bit()),
            )
            .field(
                "timer1_tez_int_raw",
                &format_args!("{}", self.timer1_tez_int_raw().bit()),
            )
            .field(
                "timer2_tez_int_raw",
                &format_args!("{}", self.timer2_tez_int_raw().bit()),
            )
            .field(
                "timer0_tep_int_raw",
                &format_args!("{}", self.timer0_tep_int_raw().bit()),
            )
            .field(
                "timer1_tep_int_raw",
                &format_args!("{}", self.timer1_tep_int_raw().bit()),
            )
            .field(
                "timer2_tep_int_raw",
                &format_args!("{}", self.timer2_tep_int_raw().bit()),
            )
            .field(
                "fault0_int_raw",
                &format_args!("{}", self.fault0_int_raw().bit()),
            )
            .field(
                "fault1_int_raw",
                &format_args!("{}", self.fault1_int_raw().bit()),
            )
            .field(
                "fault2_int_raw",
                &format_args!("{}", self.fault2_int_raw().bit()),
            )
            .field(
                "fault0_clr_int_raw",
                &format_args!("{}", self.fault0_clr_int_raw().bit()),
            )
            .field(
                "fault1_clr_int_raw",
                &format_args!("{}", self.fault1_clr_int_raw().bit()),
            )
            .field(
                "fault2_clr_int_raw",
                &format_args!("{}", self.fault2_clr_int_raw().bit()),
            )
            .field(
                "op0_tea_int_raw",
                &format_args!("{}", self.op0_tea_int_raw().bit()),
            )
            .field(
                "op1_tea_int_raw",
                &format_args!("{}", self.op1_tea_int_raw().bit()),
            )
            .field(
                "op2_tea_int_raw",
                &format_args!("{}", self.op2_tea_int_raw().bit()),
            )
            .field(
                "op0_teb_int_raw",
                &format_args!("{}", self.op0_teb_int_raw().bit()),
            )
            .field(
                "op1_teb_int_raw",
                &format_args!("{}", self.op1_teb_int_raw().bit()),
            )
            .field(
                "op2_teb_int_raw",
                &format_args!("{}", self.op2_teb_int_raw().bit()),
            )
            .field(
                "fh0_cbc_int_raw",
                &format_args!("{}", self.fh0_cbc_int_raw().bit()),
            )
            .field(
                "fh1_cbc_int_raw",
                &format_args!("{}", self.fh1_cbc_int_raw().bit()),
            )
            .field(
                "fh2_cbc_int_raw",
                &format_args!("{}", self.fh2_cbc_int_raw().bit()),
            )
            .field(
                "fh0_ost_int_raw",
                &format_args!("{}", self.fh0_ost_int_raw().bit()),
            )
            .field(
                "fh1_ost_int_raw",
                &format_args!("{}", self.fh1_ost_int_raw().bit()),
            )
            .field(
                "fh2_ost_int_raw",
                &format_args!("{}", self.fh2_ost_int_raw().bit()),
            )
            .field(
                "cap0_int_raw",
                &format_args!("{}", self.cap0_int_raw().bit()),
            )
            .field(
                "cap1_int_raw",
                &format_args!("{}", self.cap1_int_raw().bit()),
            )
            .field(
                "cap2_int_raw",
                &format_args!("{}", self.cap2_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
