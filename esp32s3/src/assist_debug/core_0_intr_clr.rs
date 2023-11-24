#[doc = "Register `CORE_0_INTR_CLR` reader"]
pub type R = crate::R<CORE_0_INTR_CLR_SPEC>;
#[doc = "Register `CORE_0_INTR_CLR` writer"]
pub type W = crate::W<CORE_0_INTR_CLR_SPEC>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_CLR` reader - Core0 dram0 area0 read monitor interrupt clr"]
pub type CORE_0_AREA_DRAM0_0_RD_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_CLR` writer - Core0 dram0 area0 read monitor interrupt clr"]
pub type CORE_0_AREA_DRAM0_0_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_CLR` reader - Core0 dram0 area0 write monitor interrupt clr"]
pub type CORE_0_AREA_DRAM0_0_WR_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_CLR` writer - Core0 dram0 area0 write monitor interrupt clr"]
pub type CORE_0_AREA_DRAM0_0_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_CLR` reader - Core0 dram0 area1 read monitor interrupt clr"]
pub type CORE_0_AREA_DRAM0_1_RD_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_CLR` writer - Core0 dram0 area1 read monitor interrupt clr"]
pub type CORE_0_AREA_DRAM0_1_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_CLR` reader - Core0 dram0 area1 write monitor interrupt clr"]
pub type CORE_0_AREA_DRAM0_1_WR_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_CLR` writer - Core0 dram0 area1 write monitor interrupt clr"]
pub type CORE_0_AREA_DRAM0_1_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_0_RD_CLR` reader - Core0 PIF area0 read monitor interrupt clr"]
pub type CORE_0_AREA_PIF_0_RD_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_0_RD_CLR` writer - Core0 PIF area0 read monitor interrupt clr"]
pub type CORE_0_AREA_PIF_0_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_0_WR_CLR` reader - Core0 PIF area0 write monitor interrupt clr"]
pub type CORE_0_AREA_PIF_0_WR_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_0_WR_CLR` writer - Core0 PIF area0 write monitor interrupt clr"]
pub type CORE_0_AREA_PIF_0_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_1_RD_CLR` reader - Core0 PIF area1 read monitor interrupt clr"]
pub type CORE_0_AREA_PIF_1_RD_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_1_RD_CLR` writer - Core0 PIF area1 read monitor interrupt clr"]
pub type CORE_0_AREA_PIF_1_RD_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_1_WR_CLR` reader - Core0 PIF area1 write monitor interrupt clr"]
pub type CORE_0_AREA_PIF_1_WR_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_1_WR_CLR` writer - Core0 PIF area1 write monitor interrupt clr"]
pub type CORE_0_AREA_PIF_1_WR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_SP_SPILL_MIN_CLR` reader - Core0 stackpoint overflow monitor interrupt clr"]
pub type CORE_0_SP_SPILL_MIN_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MIN_CLR` writer - Core0 stackpoint overflow monitor interrupt clr"]
pub type CORE_0_SP_SPILL_MIN_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_SP_SPILL_MAX_CLR` reader - Core0 stackpoint underflow monitor interrupt clr"]
pub type CORE_0_SP_SPILL_MAX_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MAX_CLR` writer - Core0 stackpoint underflow monitor interrupt clr"]
pub type CORE_0_SP_SPILL_MAX_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_CLR` reader - IBUS busy monitor interrupt clr"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_CLR` writer - IBUS busy monitor interrupt clr"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_CLR` reader - DBUS busy monitor interrupt clr"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_CLR` writer - DBUS busy monitor interrupt clr"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_clr(&self) -> CORE_0_AREA_DRAM0_0_RD_CLR_R {
        CORE_0_AREA_DRAM0_0_RD_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_clr(&self) -> CORE_0_AREA_DRAM0_0_WR_CLR_R {
        CORE_0_AREA_DRAM0_0_WR_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_clr(&self) -> CORE_0_AREA_DRAM0_1_RD_CLR_R {
        CORE_0_AREA_DRAM0_1_RD_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_clr(&self) -> CORE_0_AREA_DRAM0_1_WR_CLR_R {
        CORE_0_AREA_DRAM0_1_WR_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_clr(&self) -> CORE_0_AREA_PIF_0_RD_CLR_R {
        CORE_0_AREA_PIF_0_RD_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_clr(&self) -> CORE_0_AREA_PIF_0_WR_CLR_R {
        CORE_0_AREA_PIF_0_WR_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_clr(&self) -> CORE_0_AREA_PIF_1_RD_CLR_R {
        CORE_0_AREA_PIF_1_RD_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_clr(&self) -> CORE_0_AREA_PIF_1_WR_CLR_R {
        CORE_0_AREA_PIF_1_WR_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Core0 stackpoint overflow monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_clr(&self) -> CORE_0_SP_SPILL_MIN_CLR_R {
        CORE_0_SP_SPILL_MIN_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Core0 stackpoint underflow monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_clr(&self) -> CORE_0_SP_SPILL_MAX_CLR_R {
        CORE_0_SP_SPILL_MAX_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_clr(&self) -> CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_R {
        CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_clr(&self) -> CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_R {
        CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_INTR_CLR")
            .field(
                "core_0_area_dram0_0_rd_clr",
                &format_args!("{}", self.core_0_area_dram0_0_rd_clr().bit()),
            )
            .field(
                "core_0_area_dram0_0_wr_clr",
                &format_args!("{}", self.core_0_area_dram0_0_wr_clr().bit()),
            )
            .field(
                "core_0_area_dram0_1_rd_clr",
                &format_args!("{}", self.core_0_area_dram0_1_rd_clr().bit()),
            )
            .field(
                "core_0_area_dram0_1_wr_clr",
                &format_args!("{}", self.core_0_area_dram0_1_wr_clr().bit()),
            )
            .field(
                "core_0_area_pif_0_rd_clr",
                &format_args!("{}", self.core_0_area_pif_0_rd_clr().bit()),
            )
            .field(
                "core_0_area_pif_0_wr_clr",
                &format_args!("{}", self.core_0_area_pif_0_wr_clr().bit()),
            )
            .field(
                "core_0_area_pif_1_rd_clr",
                &format_args!("{}", self.core_0_area_pif_1_rd_clr().bit()),
            )
            .field(
                "core_0_area_pif_1_wr_clr",
                &format_args!("{}", self.core_0_area_pif_1_wr_clr().bit()),
            )
            .field(
                "core_0_sp_spill_min_clr",
                &format_args!("{}", self.core_0_sp_spill_min_clr().bit()),
            )
            .field(
                "core_0_sp_spill_max_clr",
                &format_args!("{}", self.core_0_sp_spill_max_clr().bit()),
            )
            .field(
                "core_0_iram0_exception_monitor_clr",
                &format_args!("{}", self.core_0_iram0_exception_monitor_clr().bit()),
            )
            .field(
                "core_0_dram0_exception_monitor_clr",
                &format_args!("{}", self.core_0_dram0_exception_monitor_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_INTR_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_0_rd_clr(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_0_RD_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_DRAM0_0_RD_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_0_wr_clr(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_0_WR_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_DRAM0_0_WR_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_1_rd_clr(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_1_RD_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_DRAM0_1_RD_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_1_wr_clr(
        &mut self,
    ) -> CORE_0_AREA_DRAM0_1_WR_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_DRAM0_1_WR_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_0_rd_clr(&mut self) -> CORE_0_AREA_PIF_0_RD_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_PIF_0_RD_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_0_wr_clr(&mut self) -> CORE_0_AREA_PIF_0_WR_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_PIF_0_WR_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_1_rd_clr(&mut self) -> CORE_0_AREA_PIF_1_RD_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_PIF_1_RD_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_1_wr_clr(&mut self) -> CORE_0_AREA_PIF_1_WR_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_AREA_PIF_1_WR_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Core0 stackpoint overflow monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_sp_spill_min_clr(&mut self) -> CORE_0_SP_SPILL_MIN_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_SP_SPILL_MIN_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Core0 stackpoint underflow monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_sp_spill_max_clr(&mut self) -> CORE_0_SP_SPILL_MAX_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_SP_SPILL_MAX_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_iram0_exception_monitor_clr(
        &mut self,
    ) -> CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_dram0_exception_monitor_clr(
        &mut self,
    ) -> CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W::new(self, 11)
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
#[doc = "core0 monitor interrupt clr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_intr_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_intr_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_INTR_CLR_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_intr_clr::R`](R) reader structure"]
impl crate::Readable for CORE_0_INTR_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_intr_clr::W`](W) writer structure"]
impl crate::Writable for CORE_0_INTR_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_INTR_CLR to value 0"]
impl crate::Resettable for CORE_0_INTR_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
