#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `PER_END_INT_ENA` reader - The enable bit for SPI_MEM_PER_END_INT interrupt."]
pub type PER_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `PER_END_INT_ENA` writer - The enable bit for SPI_MEM_PER_END_INT interrupt."]
pub type PER_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES_END_INT_ENA` reader - The enable bit for SPI_MEM_PES_END_INT interrupt."]
pub type PES_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `PES_END_INT_ENA` writer - The enable bit for SPI_MEM_PES_END_INT interrupt."]
pub type PES_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOTAL_TRANS_END_INT_ENA` reader - The enable bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
pub type TOTAL_TRANS_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `TOTAL_TRANS_END_INT_ENA` writer - The enable bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
pub type TOTAL_TRANS_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT_INT_ENA` reader - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type BROWN_OUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_INT_ENA` writer - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type BROWN_OUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The enable bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn per_end_int_ena(&self) -> PER_END_INT_ENA_R {
        PER_END_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn pes_end_int_ena(&self) -> PES_END_INT_ENA_R {
        PES_END_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The enable bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
    #[inline(always)]
    pub fn total_trans_end_int_ena(&self) -> TOTAL_TRANS_END_INT_ENA_R {
        TOTAL_TRANS_END_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn brown_out_int_ena(&self) -> BROWN_OUT_INT_ENA_R {
        BROWN_OUT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "per_end_int_ena",
                &format_args!("{}", self.per_end_int_ena().bit()),
            )
            .field(
                "pes_end_int_ena",
                &format_args!("{}", self.pes_end_int_ena().bit()),
            )
            .field(
                "total_trans_end_int_ena",
                &format_args!("{}", self.total_trans_end_int_ena().bit()),
            )
            .field(
                "brown_out_int_ena",
                &format_args!("{}", self.brown_out_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn per_end_int_ena(&mut self) -> PER_END_INT_ENA_W<INT_ENA_SPEC> {
        PER_END_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pes_end_int_ena(&mut self) -> PES_END_INT_ENA_W<INT_ENA_SPEC> {
        PES_END_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The enable bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn total_trans_end_int_ena(&mut self) -> TOTAL_TRANS_END_INT_ENA_W<INT_ENA_SPEC> {
        TOTAL_TRANS_END_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_ena(&mut self) -> BROWN_OUT_INT_ENA_W<INT_ENA_SPEC> {
        BROWN_OUT_INT_ENA_W::new(self, 3)
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
#[doc = "SPI1 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
