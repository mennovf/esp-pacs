#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    conf0: CONF0,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    dma_out_status: DMA_OUT_STATUS,
    dma_out_push: DMA_OUT_PUSH,
    dma_in_status: DMA_IN_STATUS,
    dma_in_pop: DMA_IN_POP,
    dma_out_link: DMA_OUT_LINK,
    dma_in_link: DMA_IN_LINK,
    conf1: CONF1,
    state0: STATE0,
    state1: STATE1,
    dma_out_eof_des_addr: DMA_OUT_EOF_DES_ADDR,
    dma_in_suc_eof_des_addr: DMA_IN_SUC_EOF_DES_ADDR,
    dma_in_err_eof_des_addr: DMA_IN_ERR_EOF_DES_ADDR,
    dma_out_eof_bfr_des_addr: DMA_OUT_EOF_BFR_DES_ADDR,
    ahb_test: AHB_TEST,
    dma_in_dscr: DMA_IN_DSCR,
    dma_in_dscr_bf0: DMA_IN_DSCR_BF0,
    dma_in_dscr_bf1: DMA_IN_DSCR_BF1,
    dma_out_dscr: DMA_OUT_DSCR,
    dma_out_dscr_bf0: DMA_OUT_DSCR_BF0,
    dma_out_dscr_bf1: DMA_OUT_DSCR_BF1,
    escape_conf: ESCAPE_CONF,
    hung_conf: HUNG_CONF,
    ack_num: ACK_NUM,
    rx_head: RX_HEAD,
    quick_sent: QUICK_SENT,
    q0_word0: Q0_WORD0,
    q0_word1: Q0_WORD1,
    q1_word0: Q1_WORD0,
    q1_word1: Q1_WORD1,
    q2_word0: Q2_WORD0,
    q2_word1: Q2_WORD1,
    q3_word0: Q3_WORD0,
    q3_word1: Q3_WORD1,
    q4_word0: Q4_WORD0,
    q4_word1: Q4_WORD1,
    q5_word0: Q5_WORD0,
    q5_word1: Q5_WORD1,
    q6_word0: Q6_WORD0,
    q6_word1: Q6_WORD1,
    esc_conf0: ESC_CONF0,
    esc_conf1: ESC_CONF1,
    esc_conf2: ESC_CONF2,
    esc_conf3: ESC_CONF3,
    pkt_thres: PKT_THRES,
    _reserved49: [u8; 0x38],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn dma_out_status(&self) -> &DMA_OUT_STATUS {
        &self.dma_out_status
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn dma_out_push(&self) -> &DMA_OUT_PUSH {
        &self.dma_out_push
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn dma_in_status(&self) -> &DMA_IN_STATUS {
        &self.dma_in_status
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn dma_in_pop(&self) -> &DMA_IN_POP {
        &self.dma_in_pop
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn dma_out_link(&self) -> &DMA_OUT_LINK {
        &self.dma_out_link
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn dma_in_link(&self) -> &DMA_IN_LINK {
        &self.dma_in_link
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn state0(&self) -> &STATE0 {
        &self.state0
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn state1(&self) -> &STATE1 {
        &self.state1
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn dma_out_eof_des_addr(&self) -> &DMA_OUT_EOF_DES_ADDR {
        &self.dma_out_eof_des_addr
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn dma_in_suc_eof_des_addr(&self) -> &DMA_IN_SUC_EOF_DES_ADDR {
        &self.dma_in_suc_eof_des_addr
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn dma_in_err_eof_des_addr(&self) -> &DMA_IN_ERR_EOF_DES_ADDR {
        &self.dma_in_err_eof_des_addr
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn dma_out_eof_bfr_des_addr(&self) -> &DMA_OUT_EOF_BFR_DES_ADDR {
        &self.dma_out_eof_bfr_des_addr
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn dma_in_dscr(&self) -> &DMA_IN_DSCR {
        &self.dma_in_dscr
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn dma_in_dscr_bf0(&self) -> &DMA_IN_DSCR_BF0 {
        &self.dma_in_dscr_bf0
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn dma_in_dscr_bf1(&self) -> &DMA_IN_DSCR_BF1 {
        &self.dma_in_dscr_bf1
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn dma_out_dscr(&self) -> &DMA_OUT_DSCR {
        &self.dma_out_dscr
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn dma_out_dscr_bf0(&self) -> &DMA_OUT_DSCR_BF0 {
        &self.dma_out_dscr_bf0
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn dma_out_dscr_bf1(&self) -> &DMA_OUT_DSCR_BF1 {
        &self.dma_out_dscr_bf1
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn escape_conf(&self) -> &ESCAPE_CONF {
        &self.escape_conf
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn hung_conf(&self) -> &HUNG_CONF {
        &self.hung_conf
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn ack_num(&self) -> &ACK_NUM {
        &self.ack_num
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn rx_head(&self) -> &RX_HEAD {
        &self.rx_head
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn quick_sent(&self) -> &QUICK_SENT {
        &self.quick_sent
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn q0_word0(&self) -> &Q0_WORD0 {
        &self.q0_word0
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn q0_word1(&self) -> &Q0_WORD1 {
        &self.q0_word1
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn q1_word0(&self) -> &Q1_WORD0 {
        &self.q1_word0
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn q1_word1(&self) -> &Q1_WORD1 {
        &self.q1_word1
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn q2_word0(&self) -> &Q2_WORD0 {
        &self.q2_word0
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn q2_word1(&self) -> &Q2_WORD1 {
        &self.q2_word1
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn q3_word0(&self) -> &Q3_WORD0 {
        &self.q3_word0
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn q3_word1(&self) -> &Q3_WORD1 {
        &self.q3_word1
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn q4_word0(&self) -> &Q4_WORD0 {
        &self.q4_word0
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn q4_word1(&self) -> &Q4_WORD1 {
        &self.q4_word1
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn q5_word0(&self) -> &Q5_WORD0 {
        &self.q5_word0
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn q5_word1(&self) -> &Q5_WORD1 {
        &self.q5_word1
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn q6_word0(&self) -> &Q6_WORD0 {
        &self.q6_word0
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn q6_word1(&self) -> &Q6_WORD1 {
        &self.q6_word1
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn esc_conf0(&self) -> &ESC_CONF0 {
        &self.esc_conf0
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn esc_conf1(&self) -> &ESC_CONF1 {
        &self.esc_conf1
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn esc_conf2(&self) -> &ESC_CONF2 {
        &self.esc_conf2
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn esc_conf3(&self) -> &ESC_CONF3 {
        &self.esc_conf3
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn pkt_thres(&self) -> &PKT_THRES {
        &self.pkt_thres
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0`] module"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = ""]
pub mod conf0;
#[doc = "INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "DMA_OUT_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_status`] module"]
pub type DMA_OUT_STATUS = crate::Reg<dma_out_status::DMA_OUT_STATUS_SPEC>;
#[doc = ""]
pub mod dma_out_status;
#[doc = "DMA_OUT_PUSH (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_push::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_push::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_push`] module"]
pub type DMA_OUT_PUSH = crate::Reg<dma_out_push::DMA_OUT_PUSH_SPEC>;
#[doc = ""]
pub mod dma_out_push;
#[doc = "DMA_IN_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_status`] module"]
pub type DMA_IN_STATUS = crate::Reg<dma_in_status::DMA_IN_STATUS_SPEC>;
#[doc = ""]
pub mod dma_in_status;
#[doc = "DMA_IN_POP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_pop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_pop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_pop`] module"]
pub type DMA_IN_POP = crate::Reg<dma_in_pop::DMA_IN_POP_SPEC>;
#[doc = ""]
pub mod dma_in_pop;
#[doc = "DMA_OUT_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_link`] module"]
pub type DMA_OUT_LINK = crate::Reg<dma_out_link::DMA_OUT_LINK_SPEC>;
#[doc = ""]
pub mod dma_out_link;
#[doc = "DMA_IN_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_link`] module"]
pub type DMA_IN_LINK = crate::Reg<dma_in_link::DMA_IN_LINK_SPEC>;
#[doc = ""]
pub mod dma_in_link;
#[doc = "CONF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = ""]
pub mod conf1;
#[doc = "STATE0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state0`] module"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = ""]
pub mod state0;
#[doc = "STATE1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state1`] module"]
pub type STATE1 = crate::Reg<state1::STATE1_SPEC>;
#[doc = ""]
pub mod state1;
#[doc = "DMA_OUT_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_eof_des_addr`] module"]
pub type DMA_OUT_EOF_DES_ADDR = crate::Reg<dma_out_eof_des_addr::DMA_OUT_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod dma_out_eof_des_addr;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_suc_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_suc_eof_des_addr`] module"]
pub type DMA_IN_SUC_EOF_DES_ADDR =
    crate::Reg<dma_in_suc_eof_des_addr::DMA_IN_SUC_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod dma_in_suc_eof_des_addr;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_err_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_err_eof_des_addr`] module"]
pub type DMA_IN_ERR_EOF_DES_ADDR =
    crate::Reg<dma_in_err_eof_des_addr::DMA_IN_ERR_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod dma_in_err_eof_des_addr;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_eof_bfr_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_eof_bfr_des_addr`] module"]
pub type DMA_OUT_EOF_BFR_DES_ADDR =
    crate::Reg<dma_out_eof_bfr_des_addr::DMA_OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = ""]
pub mod dma_out_eof_bfr_des_addr;
#[doc = "AHB_TEST (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = ""]
pub mod ahb_test;
#[doc = "DMA_IN_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_dscr`] module"]
pub type DMA_IN_DSCR = crate::Reg<dma_in_dscr::DMA_IN_DSCR_SPEC>;
#[doc = ""]
pub mod dma_in_dscr;
#[doc = "DMA_IN_DSCR_BF0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_dscr_bf0`] module"]
pub type DMA_IN_DSCR_BF0 = crate::Reg<dma_in_dscr_bf0::DMA_IN_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod dma_in_dscr_bf0;
#[doc = "DMA_IN_DSCR_BF1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_dscr_bf1`] module"]
pub type DMA_IN_DSCR_BF1 = crate::Reg<dma_in_dscr_bf1::DMA_IN_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod dma_in_dscr_bf1;
#[doc = "DMA_OUT_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_dscr`] module"]
pub type DMA_OUT_DSCR = crate::Reg<dma_out_dscr::DMA_OUT_DSCR_SPEC>;
#[doc = ""]
pub mod dma_out_dscr;
#[doc = "DMA_OUT_DSCR_BF0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_dscr_bf0`] module"]
pub type DMA_OUT_DSCR_BF0 = crate::Reg<dma_out_dscr_bf0::DMA_OUT_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod dma_out_dscr_bf0;
#[doc = "DMA_OUT_DSCR_BF1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_dscr_bf1`] module"]
pub type DMA_OUT_DSCR_BF1 = crate::Reg<dma_out_dscr_bf1::DMA_OUT_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod dma_out_dscr_bf1;
#[doc = "ESCAPE_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`escape_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`escape_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@escape_conf`] module"]
pub type ESCAPE_CONF = crate::Reg<escape_conf::ESCAPE_CONF_SPEC>;
#[doc = ""]
pub mod escape_conf;
#[doc = "HUNG_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hung_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hung_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hung_conf`] module"]
pub type HUNG_CONF = crate::Reg<hung_conf::HUNG_CONF_SPEC>;
#[doc = ""]
pub mod hung_conf;
#[doc = "ACK_NUM (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ack_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ack_num`] module"]
pub type ACK_NUM = crate::Reg<ack_num::ACK_NUM_SPEC>;
#[doc = ""]
pub mod ack_num;
#[doc = "RX_HEAD (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_head::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_head`] module"]
pub type RX_HEAD = crate::Reg<rx_head::RX_HEAD_SPEC>;
#[doc = ""]
pub mod rx_head;
#[doc = "QUICK_SENT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quick_sent::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quick_sent::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quick_sent`] module"]
pub type QUICK_SENT = crate::Reg<quick_sent::QUICK_SENT_SPEC>;
#[doc = ""]
pub mod quick_sent;
#[doc = "Q0_WORD0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q0_word0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q0_word0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q0_word0`] module"]
pub type Q0_WORD0 = crate::Reg<q0_word0::Q0_WORD0_SPEC>;
#[doc = ""]
pub mod q0_word0;
#[doc = "Q0_WORD1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q0_word1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q0_word1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q0_word1`] module"]
pub type Q0_WORD1 = crate::Reg<q0_word1::Q0_WORD1_SPEC>;
#[doc = ""]
pub mod q0_word1;
#[doc = "Q1_WORD0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q1_word0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q1_word0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q1_word0`] module"]
pub type Q1_WORD0 = crate::Reg<q1_word0::Q1_WORD0_SPEC>;
#[doc = ""]
pub mod q1_word0;
#[doc = "Q1_WORD1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q1_word1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q1_word1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q1_word1`] module"]
pub type Q1_WORD1 = crate::Reg<q1_word1::Q1_WORD1_SPEC>;
#[doc = ""]
pub mod q1_word1;
#[doc = "Q2_WORD0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q2_word0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q2_word0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q2_word0`] module"]
pub type Q2_WORD0 = crate::Reg<q2_word0::Q2_WORD0_SPEC>;
#[doc = ""]
pub mod q2_word0;
#[doc = "Q2_WORD1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q2_word1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q2_word1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q2_word1`] module"]
pub type Q2_WORD1 = crate::Reg<q2_word1::Q2_WORD1_SPEC>;
#[doc = ""]
pub mod q2_word1;
#[doc = "Q3_WORD0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q3_word0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q3_word0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q3_word0`] module"]
pub type Q3_WORD0 = crate::Reg<q3_word0::Q3_WORD0_SPEC>;
#[doc = ""]
pub mod q3_word0;
#[doc = "Q3_WORD1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q3_word1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q3_word1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q3_word1`] module"]
pub type Q3_WORD1 = crate::Reg<q3_word1::Q3_WORD1_SPEC>;
#[doc = ""]
pub mod q3_word1;
#[doc = "Q4_WORD0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q4_word0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q4_word0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q4_word0`] module"]
pub type Q4_WORD0 = crate::Reg<q4_word0::Q4_WORD0_SPEC>;
#[doc = ""]
pub mod q4_word0;
#[doc = "Q4_WORD1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q4_word1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q4_word1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q4_word1`] module"]
pub type Q4_WORD1 = crate::Reg<q4_word1::Q4_WORD1_SPEC>;
#[doc = ""]
pub mod q4_word1;
#[doc = "Q5_WORD0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q5_word0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q5_word0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q5_word0`] module"]
pub type Q5_WORD0 = crate::Reg<q5_word0::Q5_WORD0_SPEC>;
#[doc = ""]
pub mod q5_word0;
#[doc = "Q5_WORD1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q5_word1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q5_word1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q5_word1`] module"]
pub type Q5_WORD1 = crate::Reg<q5_word1::Q5_WORD1_SPEC>;
#[doc = ""]
pub mod q5_word1;
#[doc = "Q6_WORD0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q6_word0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q6_word0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q6_word0`] module"]
pub type Q6_WORD0 = crate::Reg<q6_word0::Q6_WORD0_SPEC>;
#[doc = ""]
pub mod q6_word0;
#[doc = "Q6_WORD1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q6_word1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q6_word1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@q6_word1`] module"]
pub type Q6_WORD1 = crate::Reg<q6_word1::Q6_WORD1_SPEC>;
#[doc = ""]
pub mod q6_word1;
#[doc = "ESC_CONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esc_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_conf0`] module"]
pub type ESC_CONF0 = crate::Reg<esc_conf0::ESC_CONF0_SPEC>;
#[doc = ""]
pub mod esc_conf0;
#[doc = "ESC_CONF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esc_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_conf1`] module"]
pub type ESC_CONF1 = crate::Reg<esc_conf1::ESC_CONF1_SPEC>;
#[doc = ""]
pub mod esc_conf1;
#[doc = "ESC_CONF2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esc_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_conf2`] module"]
pub type ESC_CONF2 = crate::Reg<esc_conf2::ESC_CONF2_SPEC>;
#[doc = ""]
pub mod esc_conf2;
#[doc = "ESC_CONF3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_conf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esc_conf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_conf3`] module"]
pub type ESC_CONF3 = crate::Reg<esc_conf3::ESC_CONF3_SPEC>;
#[doc = ""]
pub mod esc_conf3;
#[doc = "PKT_THRES (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pkt_thres::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pkt_thres::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkt_thres`] module"]
pub type PKT_THRES = crate::Reg<pkt_thres::PKT_THRES_SPEC>;
#[doc = ""]
pub mod pkt_thres;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
