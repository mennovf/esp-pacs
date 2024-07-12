#[doc = "Register `RF_PWC` reader"]
pub type R = crate::R<RF_PWC_SPEC>;
#[doc = "Register `RF_PWC` writer"]
pub type W = crate::W<RF_PWC_SPEC>;
#[doc = "Field `MSPI_PHY_XPD` reader - need_des"]
pub type MSPI_PHY_XPD_R = crate::BitReader;
#[doc = "Field `MSPI_PHY_XPD` writer - need_des"]
pub type MSPI_PHY_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_PLL_XPD` reader - need_des"]
pub type SDIO_PLL_XPD_R = crate::BitReader;
#[doc = "Field `SDIO_PLL_XPD` writer - need_des"]
pub type SDIO_PLL_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIF_I2C_RSTB` reader - need_des"]
pub type PERIF_I2C_RSTB_R = crate::BitReader;
#[doc = "Field `PERIF_I2C_RSTB` writer - need_des"]
pub type PERIF_I2C_RSTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_PERIF_I2C` reader - need_des"]
pub type XPD_PERIF_I2C_R = crate::BitReader;
#[doc = "Field `XPD_PERIF_I2C` writer - need_des"]
pub type XPD_PERIF_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_TXRF_I2C` reader - need_des"]
pub type XPD_TXRF_I2C_R = crate::BitReader;
#[doc = "Field `XPD_TXRF_I2C` writer - need_des"]
pub type XPD_TXRF_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_RFRX_PBUS` reader - need_des"]
pub type XPD_RFRX_PBUS_R = crate::BitReader;
#[doc = "Field `XPD_RFRX_PBUS` writer - need_des"]
pub type XPD_RFRX_PBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_CKGEN_I2C` reader - need_des"]
pub type XPD_CKGEN_I2C_R = crate::BitReader;
#[doc = "Field `XPD_CKGEN_I2C` writer - need_des"]
pub type XPD_CKGEN_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn mspi_phy_xpd(&self) -> MSPI_PHY_XPD_R {
        MSPI_PHY_XPD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn sdio_pll_xpd(&self) -> SDIO_PLL_XPD_R {
        SDIO_PLL_XPD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn perif_i2c_rstb(&self) -> PERIF_I2C_RSTB_R {
        PERIF_I2C_RSTB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn xpd_perif_i2c(&self) -> XPD_PERIF_I2C_R {
        XPD_PERIF_I2C_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn xpd_txrf_i2c(&self) -> XPD_TXRF_I2C_R {
        XPD_TXRF_I2C_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn xpd_rfrx_pbus(&self) -> XPD_RFRX_PBUS_R {
        XPD_RFRX_PBUS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn xpd_ckgen_i2c(&self) -> XPD_CKGEN_I2C_R {
        XPD_CKGEN_I2C_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF_PWC")
            .field("mspi_phy_xpd", &self.mspi_phy_xpd())
            .field("sdio_pll_xpd", &self.sdio_pll_xpd())
            .field("perif_i2c_rstb", &self.perif_i2c_rstb())
            .field("xpd_perif_i2c", &self.xpd_perif_i2c())
            .field("xpd_txrf_i2c", &self.xpd_txrf_i2c())
            .field("xpd_rfrx_pbus", &self.xpd_rfrx_pbus())
            .field("xpd_ckgen_i2c", &self.xpd_ckgen_i2c())
            .finish()
    }
}
impl W {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn mspi_phy_xpd(&mut self) -> MSPI_PHY_XPD_W<RF_PWC_SPEC> {
        MSPI_PHY_XPD_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_pll_xpd(&mut self) -> SDIO_PLL_XPD_W<RF_PWC_SPEC> {
        SDIO_PLL_XPD_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn perif_i2c_rstb(&mut self) -> PERIF_I2C_RSTB_W<RF_PWC_SPEC> {
        PERIF_I2C_RSTB_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_perif_i2c(&mut self) -> XPD_PERIF_I2C_W<RF_PWC_SPEC> {
        XPD_PERIF_I2C_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_txrf_i2c(&mut self) -> XPD_TXRF_I2C_W<RF_PWC_SPEC> {
        XPD_TXRF_I2C_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_rfrx_pbus(&mut self) -> XPD_RFRX_PBUS_W<RF_PWC_SPEC> {
        XPD_RFRX_PBUS_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_ckgen_i2c(&mut self) -> XPD_CKGEN_I2C_W<RF_PWC_SPEC> {
        XPD_CKGEN_I2C_W::new(self, 30)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_pwc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_pwc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RF_PWC_SPEC;
impl crate::RegisterSpec for RF_PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_pwc::R`](R) reader structure"]
impl crate::Readable for RF_PWC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rf_pwc::W`](W) writer structure"]
impl crate::Writable for RF_PWC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RF_PWC to value 0x0800_0000"]
impl crate::Resettable for RF_PWC_SPEC {
    const RESET_VALUE: u32 = 0x0800_0000;
}
