#[doc = "Register `BLK0_RDATA5` reader"]
pub struct R(crate::R<BLK0_RDATA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_RDATA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_RDATA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_RDATA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_RDATA5` writer"]
pub struct W(crate::W<BLK0_RDATA5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_RDATA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BLK0_RDATA5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_RDATA5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD_SPI_PAD_CONFIG_CLK` reader - "]
pub type RD_SPI_PAD_CONFIG_CLK_R = crate::FieldReader;
#[doc = "Field `RD_SPI_PAD_CONFIG_Q` reader - "]
pub type RD_SPI_PAD_CONFIG_Q_R = crate::FieldReader;
#[doc = "Field `RD_SPI_PAD_CONFIG_D` reader - "]
pub type RD_SPI_PAD_CONFIG_D_R = crate::FieldReader;
#[doc = "Field `RD_SPI_PAD_CONFIG_CS0` reader - "]
pub type RD_SPI_PAD_CONFIG_CS0_R = crate::FieldReader;
#[doc = "Field `RD_CHIP_VER_REV2` reader - "]
pub type RD_CHIP_VER_REV2_R = crate::BitReader;
#[doc = "Field `RD_RESERVE_0_181` reader - "]
pub type RD_RESERVE_0_181_R = crate::BitReader;
#[doc = "Field `RD_RESERVE_0_181` writer - "]
pub type RD_RESERVE_0_181_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_RDATA5_SPEC, O>;
#[doc = "Field `RD_VOL_LEVEL_HP_INV` reader - "]
pub type RD_VOL_LEVEL_HP_INV_R = crate::FieldReader;
#[doc = "Field `RD_WAFER_VERSION_MINOR` reader - "]
pub type RD_WAFER_VERSION_MINOR_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_0_186` reader - "]
pub type RD_RESERVE_0_186_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_0_186` writer - "]
pub type RD_RESERVE_0_186_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_RDATA5_SPEC, 2, O>;
#[doc = "Field `RD_FLASH_CRYPT_CONFIG` reader - "]
pub type RD_FLASH_CRYPT_CONFIG_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rd_spi_pad_config_clk(&self) -> RD_SPI_PAD_CONFIG_CLK_R {
        RD_SPI_PAD_CONFIG_CLK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn rd_spi_pad_config_q(&self) -> RD_SPI_PAD_CONFIG_Q_R {
        RD_SPI_PAD_CONFIG_Q_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn rd_spi_pad_config_d(&self) -> RD_SPI_PAD_CONFIG_D_R {
        RD_SPI_PAD_CONFIG_D_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn rd_spi_pad_config_cs0(&self) -> RD_SPI_PAD_CONFIG_CS0_R {
        RD_SPI_PAD_CONFIG_CS0_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rd_chip_ver_rev2(&self) -> RD_CHIP_VER_REV2_R {
        RD_CHIP_VER_REV2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rd_reserve_0_181(&self) -> RD_RESERVE_0_181_R {
        RD_RESERVE_0_181_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn rd_vol_level_hp_inv(&self) -> RD_VOL_LEVEL_HP_INV_R {
        RD_VOL_LEVEL_HP_INV_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rd_wafer_version_minor(&self) -> RD_WAFER_VERSION_MINOR_R {
        RD_WAFER_VERSION_MINOR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn rd_reserve_0_186(&self) -> RD_RESERVE_0_186_R {
        RD_RESERVE_0_186_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rd_flash_crypt_config(&self) -> RD_FLASH_CRYPT_CONFIG_R {
        RD_FLASH_CRYPT_CONFIG_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_RDATA5")
            .field(
                "rd_spi_pad_config_clk",
                &format_args!("{}", self.rd_spi_pad_config_clk().bits()),
            )
            .field(
                "rd_spi_pad_config_q",
                &format_args!("{}", self.rd_spi_pad_config_q().bits()),
            )
            .field(
                "rd_spi_pad_config_d",
                &format_args!("{}", self.rd_spi_pad_config_d().bits()),
            )
            .field(
                "rd_spi_pad_config_cs0",
                &format_args!("{}", self.rd_spi_pad_config_cs0().bits()),
            )
            .field(
                "rd_chip_ver_rev2",
                &format_args!("{}", self.rd_chip_ver_rev2().bit()),
            )
            .field(
                "rd_reserve_0_181",
                &format_args!("{}", self.rd_reserve_0_181().bit()),
            )
            .field(
                "rd_vol_level_hp_inv",
                &format_args!("{}", self.rd_vol_level_hp_inv().bits()),
            )
            .field(
                "rd_wafer_version_minor",
                &format_args!("{}", self.rd_wafer_version_minor().bits()),
            )
            .field(
                "rd_reserve_0_186",
                &format_args!("{}", self.rd_reserve_0_186().bits()),
            )
            .field(
                "rd_flash_crypt_config",
                &format_args!("{}", self.rd_flash_crypt_config().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_RDATA5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn rd_reserve_0_181(&mut self) -> RD_RESERVE_0_181_W<21> {
        RD_RESERVE_0_181_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn rd_reserve_0_186(&mut self) -> RD_RESERVE_0_186_W<26> {
        RD_RESERVE_0_186_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_rdata5](index.html) module"]
pub struct BLK0_RDATA5_SPEC;
impl crate::RegisterSpec for BLK0_RDATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_rdata5::R](R) reader structure"]
impl crate::Readable for BLK0_RDATA5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_rdata5::W](W) writer structure"]
impl crate::Writable for BLK0_RDATA5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK0_RDATA5 to value 0"]
impl crate::Resettable for BLK0_RDATA5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
