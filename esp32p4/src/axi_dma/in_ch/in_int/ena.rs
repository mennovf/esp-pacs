#[doc = "Register `ENA` reader"]
pub type R = crate::R<ENA_SPEC>;
#[doc = "Register `ENA` writer"]
pub type W = crate::W<ENA_SPEC>;
#[doc = "Field `IN_DONE` reader - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_DONE` writer - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF` reader - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` writer - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF` reader - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF` writer - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR` reader - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` writer - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY` reader - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` writer - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L1_OVF` reader - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_L1_OVF_R = crate::BitReader;
#[doc = "Field `INFIFO_L1_OVF` writer - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_L1_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L1_UDF` reader - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_L1_UDF_R = crate::BitReader;
#[doc = "Field `INFIFO_L1_UDF` writer - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_L1_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L2_OVF` reader - The interrupt enable bit for the INFIFO_OVF_L2_CH_INT interrupt."]
pub type INFIFO_L2_OVF_R = crate::BitReader;
#[doc = "Field `INFIFO_L2_OVF` writer - The interrupt enable bit for the INFIFO_OVF_L2_CH_INT interrupt."]
pub type INFIFO_L2_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L2_UDF` reader - The interrupt enable bit for the INFIFO_UDF_L2_CH_INT interrupt."]
pub type INFIFO_L2_UDF_R = crate::BitReader;
#[doc = "Field `INFIFO_L2_UDF` writer - The interrupt enable bit for the INFIFO_UDF_L2_CH_INT interrupt."]
pub type INFIFO_L2_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L3_OVF` reader - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
pub type INFIFO_L3_OVF_R = crate::BitReader;
#[doc = "Field `INFIFO_L3_OVF` writer - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
pub type INFIFO_L3_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L3_UDF` reader - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
pub type INFIFO_L3_UDF_R = crate::BitReader;
#[doc = "Field `INFIFO_L3_UDF` writer - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
pub type INFIFO_L3_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l1_ovf(&self) -> INFIFO_L1_OVF_R {
        INFIFO_L1_OVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l1_udf(&self) -> INFIFO_L1_UDF_R {
        INFIFO_L1_UDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the INFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l2_ovf(&self) -> INFIFO_L2_OVF_R {
        INFIFO_L2_OVF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the INFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l2_udf(&self) -> INFIFO_L2_UDF_R {
        INFIFO_L2_UDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l3_ovf(&self) -> INFIFO_L3_OVF_R {
        INFIFO_L3_OVF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l3_udf(&self) -> INFIFO_L3_UDF_R {
        INFIFO_L3_UDF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENA")
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("in_err_eof", &self.in_err_eof())
            .field("in_dscr_err", &self.in_dscr_err())
            .field("in_dscr_empty", &self.in_dscr_empty())
            .field("infifo_l1_ovf", &self.infifo_l1_ovf())
            .field("infifo_l1_udf", &self.infifo_l1_udf())
            .field("infifo_l2_ovf", &self.infifo_l2_ovf())
            .field("infifo_l2_udf", &self.infifo_l2_udf())
            .field("infifo_l3_ovf", &self.infifo_l3_ovf())
            .field("infifo_l3_udf", &self.infifo_l3_udf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_done(&mut self) -> IN_DONE_W<ENA_SPEC> {
        IN_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<ENA_SPEC> {
        IN_SUC_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<ENA_SPEC> {
        IN_ERR_EOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<ENA_SPEC> {
        IN_DSCR_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<ENA_SPEC> {
        IN_DSCR_EMPTY_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_l1_ovf(&mut self) -> INFIFO_L1_OVF_W<ENA_SPEC> {
        INFIFO_L1_OVF_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_l1_udf(&mut self) -> INFIFO_L1_UDF_W<ENA_SPEC> {
        INFIFO_L1_UDF_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the INFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_l2_ovf(&mut self) -> INFIFO_L2_OVF_W<ENA_SPEC> {
        INFIFO_L2_OVF_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the INFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_l2_udf(&mut self) -> INFIFO_L2_UDF_W<ENA_SPEC> {
        INFIFO_L2_UDF_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_l3_ovf(&mut self) -> INFIFO_L3_OVF_W<ENA_SPEC> {
        INFIFO_L3_OVF_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_l3_udf(&mut self) -> INFIFO_L3_UDF_W<ENA_SPEC> {
        INFIFO_L3_UDF_W::new(self, 10)
    }
}
#[doc = "Interrupt enable bits of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENA_SPEC;
impl crate::RegisterSpec for ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ena::R`](R) reader structure"]
impl crate::Readable for ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ena::W`](W) writer structure"]
impl crate::Writable for ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENA to value 0"]
impl crate::Resettable for ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
