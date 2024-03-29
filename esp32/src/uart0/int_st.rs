#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `RXFIFO_FULL` reader - This is the status bit for rxfifo_full_int_raw when rxfifo_full_int_ena is set to 1."]
pub type RXFIFO_FULL_R = crate::BitReader;
#[doc = "Field `TXFIFO_EMPTY` reader - This is the status bit for txfifo_empty_int_raw when txfifo_empty_int_ena is set to 1."]
pub type TXFIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `PARITY_ERR` reader - This is the status bit for parity_err_int_raw when parity_err_int_ena is set to 1."]
pub type PARITY_ERR_R = crate::BitReader;
#[doc = "Field `FRM_ERR` reader - This is the status bit for frm_err_int_raw when fm_err_int_ena is set to 1."]
pub type FRM_ERR_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` reader - This is the status bit for rxfifo_ovf_int_raw when rxfifo_ovf_int_ena is set to 1."]
pub type RXFIFO_OVF_R = crate::BitReader;
#[doc = "Field `DSR_CHG` reader - This is the status bit for dsr_chg_int_raw when dsr_chg_int_ena is set to 1."]
pub type DSR_CHG_R = crate::BitReader;
#[doc = "Field `CTS_CHG` reader - This is the status bit for cts_chg_int_raw when cts_chg_int_ena is set to 1."]
pub type CTS_CHG_R = crate::BitReader;
#[doc = "Field `BRK_DET` reader - This is the status bit for brk_det_int_raw when brk_det_int_ena is set to 1."]
pub type BRK_DET_R = crate::BitReader;
#[doc = "Field `RXFIFO_TOUT` reader - This is the status bit for rxfifo_tout_int_raw when rxfifo_tout_int_ena is set to 1."]
pub type RXFIFO_TOUT_R = crate::BitReader;
#[doc = "Field `SW_XON` reader - This is the status bit for sw_xon_int_raw when sw_xon_int_ena is set to 1."]
pub type SW_XON_R = crate::BitReader;
#[doc = "Field `SW_XOFF` reader - This is the status bit for sw_xoff_int_raw when sw_xoff_int_ena is set to 1."]
pub type SW_XOFF_R = crate::BitReader;
#[doc = "Field `GLITCH_DET` reader - This is the status bit for glitch_det_int_raw when glitch_det_int_ena is set to 1."]
pub type GLITCH_DET_R = crate::BitReader;
#[doc = "Field `TX_BRK_DONE` reader - This is the status bit for tx_brk_done_int_raw when tx_brk_done_int_ena is set to 1."]
pub type TX_BRK_DONE_R = crate::BitReader;
#[doc = "Field `TX_BRK_IDLE_DONE` reader - This is the stauts bit for tx_brk_idle_done_int_raw when tx_brk_idle_done_int_ena is set to 1."]
pub type TX_BRK_IDLE_DONE_R = crate::BitReader;
#[doc = "Field `TX_DONE` reader - This is the status bit for tx_done_int_raw when tx_done_int_ena is set to 1."]
pub type TX_DONE_R = crate::BitReader;
#[doc = "Field `RS485_PARITY_ERR` reader - This is the status bit for rs485_parity_err_int_raw when rs485_parity_int_ena is set to 1."]
pub type RS485_PARITY_ERR_R = crate::BitReader;
#[doc = "Field `RS485_FRM_ERR` reader - This is the status bit for rs485_fm_err_int_raw when rs485_fm_err_int_ena is set to 1."]
pub type RS485_FRM_ERR_R = crate::BitReader;
#[doc = "Field `RS485_CLASH` reader - This is the status bit for rs485_clash_int_raw when rs485_clash_int_ena is set to 1."]
pub type RS485_CLASH_R = crate::BitReader;
#[doc = "Field `AT_CMD_CHAR_DET` reader - This is the status bit for at_cmd_det_int_raw when at_cmd_char_det_int_ena is set to 1."]
pub type AT_CMD_CHAR_DET_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This is the status bit for rxfifo_full_int_raw when rxfifo_full_int_ena is set to 1."]
    #[inline(always)]
    pub fn rxfifo_full(&self) -> RXFIFO_FULL_R {
        RXFIFO_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the status bit for txfifo_empty_int_raw when txfifo_empty_int_ena is set to 1."]
    #[inline(always)]
    pub fn txfifo_empty(&self) -> TXFIFO_EMPTY_R {
        TXFIFO_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the status bit for parity_err_int_raw when parity_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn parity_err(&self) -> PARITY_ERR_R {
        PARITY_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the status bit for frm_err_int_raw when fm_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn frm_err(&self) -> FRM_ERR_R {
        FRM_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the status bit for rxfifo_ovf_int_raw when rxfifo_ovf_int_ena is set to 1."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RXFIFO_OVF_R {
        RXFIFO_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the status bit for dsr_chg_int_raw when dsr_chg_int_ena is set to 1."]
    #[inline(always)]
    pub fn dsr_chg(&self) -> DSR_CHG_R {
        DSR_CHG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the status bit for cts_chg_int_raw when cts_chg_int_ena is set to 1."]
    #[inline(always)]
    pub fn cts_chg(&self) -> CTS_CHG_R {
        CTS_CHG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the status bit for brk_det_int_raw when brk_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn brk_det(&self) -> BRK_DET_R {
        BRK_DET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the status bit for rxfifo_tout_int_raw when rxfifo_tout_int_ena is set to 1."]
    #[inline(always)]
    pub fn rxfifo_tout(&self) -> RXFIFO_TOUT_R {
        RXFIFO_TOUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the status bit for sw_xon_int_raw when sw_xon_int_ena is set to 1."]
    #[inline(always)]
    pub fn sw_xon(&self) -> SW_XON_R {
        SW_XON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This is the status bit for sw_xoff_int_raw when sw_xoff_int_ena is set to 1."]
    #[inline(always)]
    pub fn sw_xoff(&self) -> SW_XOFF_R {
        SW_XOFF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the status bit for glitch_det_int_raw when glitch_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn glitch_det(&self) -> GLITCH_DET_R {
        GLITCH_DET_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the status bit for tx_brk_done_int_raw when tx_brk_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn tx_brk_done(&self) -> TX_BRK_DONE_R {
        TX_BRK_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the stauts bit for tx_brk_idle_done_int_raw when tx_brk_idle_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn tx_brk_idle_done(&self) -> TX_BRK_IDLE_DONE_R {
        TX_BRK_IDLE_DONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the status bit for tx_done_int_raw when tx_done_int_ena is set to 1."]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the status bit for rs485_parity_err_int_raw when rs485_parity_int_ena is set to 1."]
    #[inline(always)]
    pub fn rs485_parity_err(&self) -> RS485_PARITY_ERR_R {
        RS485_PARITY_ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This is the status bit for rs485_fm_err_int_raw when rs485_fm_err_int_ena is set to 1."]
    #[inline(always)]
    pub fn rs485_frm_err(&self) -> RS485_FRM_ERR_R {
        RS485_FRM_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This is the status bit for rs485_clash_int_raw when rs485_clash_int_ena is set to 1."]
    #[inline(always)]
    pub fn rs485_clash(&self) -> RS485_CLASH_R {
        RS485_CLASH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This is the status bit for at_cmd_det_int_raw when at_cmd_char_det_int_ena is set to 1."]
    #[inline(always)]
    pub fn at_cmd_char_det(&self) -> AT_CMD_CHAR_DET_R {
        AT_CMD_CHAR_DET_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("rxfifo_full", &format_args!("{}", self.rxfifo_full().bit()))
            .field(
                "txfifo_empty",
                &format_args!("{}", self.txfifo_empty().bit()),
            )
            .field("parity_err", &format_args!("{}", self.parity_err().bit()))
            .field("frm_err", &format_args!("{}", self.frm_err().bit()))
            .field("rxfifo_ovf", &format_args!("{}", self.rxfifo_ovf().bit()))
            .field("dsr_chg", &format_args!("{}", self.dsr_chg().bit()))
            .field("cts_chg", &format_args!("{}", self.cts_chg().bit()))
            .field("brk_det", &format_args!("{}", self.brk_det().bit()))
            .field("rxfifo_tout", &format_args!("{}", self.rxfifo_tout().bit()))
            .field("sw_xon", &format_args!("{}", self.sw_xon().bit()))
            .field("sw_xoff", &format_args!("{}", self.sw_xoff().bit()))
            .field("glitch_det", &format_args!("{}", self.glitch_det().bit()))
            .field("tx_brk_done", &format_args!("{}", self.tx_brk_done().bit()))
            .field(
                "tx_brk_idle_done",
                &format_args!("{}", self.tx_brk_idle_done().bit()),
            )
            .field("tx_done", &format_args!("{}", self.tx_done().bit()))
            .field(
                "rs485_parity_err",
                &format_args!("{}", self.rs485_parity_err().bit()),
            )
            .field(
                "rs485_frm_err",
                &format_args!("{}", self.rs485_frm_err().bit()),
            )
            .field("rs485_clash", &format_args!("{}", self.rs485_clash().bit()))
            .field(
                "at_cmd_char_det",
                &format_args!("{}", self.at_cmd_char_det().bit()),
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
