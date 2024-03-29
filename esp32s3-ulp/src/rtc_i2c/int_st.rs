#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `SLAVE_TRAN_COMP` reader - slave transit complete interrupt state"]
pub type SLAVE_TRAN_COMP_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST` reader - arbitration lost interrupt state"]
pub type ARBITRATION_LOST_R = crate::BitReader;
#[doc = "Field `MASTER_TRAN_COMP` reader - master transit complete interrupt state"]
pub type MASTER_TRAN_COMP_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE` reader - transit complete interrupt state"]
pub type TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `TIME_OUT` reader - time out interrupt state"]
pub type TIME_OUT_R = crate::BitReader;
#[doc = "Field `ACK_ERR` reader - ack error interrupt state"]
pub type ACK_ERR_R = crate::BitReader;
#[doc = "Field `RX_DATA` reader - receive data interrupt state"]
pub type RX_DATA_R = crate::BitReader;
#[doc = "Field `TX_DATA` reader - transit data interrupt state"]
pub type TX_DATA_R = crate::BitReader;
#[doc = "Field `DETECT_START` reader - detect start interrupt state"]
pub type DETECT_START_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - slave transit complete interrupt state"]
    #[inline(always)]
    pub fn slave_tran_comp(&self) -> SLAVE_TRAN_COMP_R {
        SLAVE_TRAN_COMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - arbitration lost interrupt state"]
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ARBITRATION_LOST_R {
        ARBITRATION_LOST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - master transit complete interrupt state"]
    #[inline(always)]
    pub fn master_tran_comp(&self) -> MASTER_TRAN_COMP_R {
        MASTER_TRAN_COMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transit complete interrupt state"]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - time out interrupt state"]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ack error interrupt state"]
    #[inline(always)]
    pub fn ack_err(&self) -> ACK_ERR_R {
        ACK_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - receive data interrupt state"]
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - transit data interrupt state"]
    #[inline(always)]
    pub fn tx_data(&self) -> TX_DATA_R {
        TX_DATA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - detect start interrupt state"]
    #[inline(always)]
    pub fn detect_start(&self) -> DETECT_START_R {
        DETECT_START_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "slave_tran_comp",
                &format_args!("{}", self.slave_tran_comp().bit()),
            )
            .field(
                "arbitration_lost",
                &format_args!("{}", self.arbitration_lost().bit()),
            )
            .field(
                "master_tran_comp",
                &format_args!("{}", self.master_tran_comp().bit()),
            )
            .field(
                "trans_complete",
                &format_args!("{}", self.trans_complete().bit()),
            )
            .field("time_out", &format_args!("{}", self.time_out().bit()))
            .field("ack_err", &format_args!("{}", self.ack_err().bit()))
            .field("rx_data", &format_args!("{}", self.rx_data().bit()))
            .field("tx_data", &format_args!("{}", self.tx_data().bit()))
            .field(
                "detect_start",
                &format_args!("{}", self.detect_start().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "interrupt state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
