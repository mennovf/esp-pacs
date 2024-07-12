#[doc = "Register `RD_REPEAT_DATA1` reader"]
pub type R = crate::R<RD_REPEAT_DATA1_SPEC>;
#[doc = "Field `VDD_SPI_DREFM` reader - SPI regulator medium voltage reference."]
pub type VDD_SPI_DREFM_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_DREFL` reader - SPI regulator low voltage reference."]
pub type VDD_SPI_DREFL_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_XPD` reader - SPI regulator power up signal."]
pub type VDD_SPI_XPD_R = crate::BitReader;
#[doc = "Field `VDD_SPI_TIEH` reader - SPI regulator output is short connected to VDD3P3_RTC_IO."]
pub type VDD_SPI_TIEH_R = crate::BitReader;
#[doc = "Field `VDD_SPI_FORCE` reader - Set this bit and force to use the configuration of eFuse to configure VDD_SPI."]
pub type VDD_SPI_FORCE_R = crate::BitReader;
#[doc = "Field `VDD_SPI_EN_INIT` reader - Set SPI regulator to 0 to configure init\\[1:0\\]=0."]
pub type VDD_SPI_EN_INIT_R = crate::BitReader;
#[doc = "Field `VDD_SPI_ENCURLIM` reader - Set SPI regulator to 1 to enable output current limit."]
pub type VDD_SPI_ENCURLIM_R = crate::BitReader;
#[doc = "Field `VDD_SPI_DCURLIM` reader - Tunes the current limit threshold of SPI regulator when tieh=0, about 800 mA/(8+d)."]
pub type VDD_SPI_DCURLIM_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_INIT` reader - Adds resistor from LDO output to ground. 0: no resistance 1: 6 K 2: 4 K 3: 2 K."]
pub type VDD_SPI_INIT_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_DCAP` reader - Prevents SPI regulator from overshoot."]
pub type VDD_SPI_DCAP_R = crate::FieldReader;
#[doc = "Field `WDT_DELAY_SEL` reader - Selects RTC watchdog timeout threshold, in unit of slow clock cycle. 0: 40000. 1: 80000. 2: 160000. 3:320000."]
pub type WDT_DELAY_SEL_R = crate::FieldReader;
#[doc = "Field `SPI_BOOT_CRYPT_CNT` reader - Set this bit to enable SPI boot encrypt/decrypt. Odd number of 1: enable. even number of 1: disable."]
pub type SPI_BOOT_CRYPT_CNT_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE0` reader - Set this bit to enable revoking first secure boot key."]
pub type SECURE_BOOT_KEY_REVOKE0_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE1` reader - Set this bit to enable revoking second secure boot key."]
pub type SECURE_BOOT_KEY_REVOKE1_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE2` reader - Set this bit to enable revoking third secure boot key."]
pub type SECURE_BOOT_KEY_REVOKE2_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_0` reader - Purpose of Key0."]
pub type KEY_PURPOSE_0_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_1` reader - Purpose of Key1."]
pub type KEY_PURPOSE_1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - SPI regulator medium voltage reference."]
    #[inline(always)]
    pub fn vdd_spi_drefm(&self) -> VDD_SPI_DREFM_R {
        VDD_SPI_DREFM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPI regulator low voltage reference."]
    #[inline(always)]
    pub fn vdd_spi_drefl(&self) -> VDD_SPI_DREFL_R {
        VDD_SPI_DREFL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SPI regulator power up signal."]
    #[inline(always)]
    pub fn vdd_spi_xpd(&self) -> VDD_SPI_XPD_R {
        VDD_SPI_XPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI regulator output is short connected to VDD3P3_RTC_IO."]
    #[inline(always)]
    pub fn vdd_spi_tieh(&self) -> VDD_SPI_TIEH_R {
        VDD_SPI_TIEH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit and force to use the configuration of eFuse to configure VDD_SPI."]
    #[inline(always)]
    pub fn vdd_spi_force(&self) -> VDD_SPI_FORCE_R {
        VDD_SPI_FORCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set SPI regulator to 0 to configure init\\[1:0\\]=0."]
    #[inline(always)]
    pub fn vdd_spi_en_init(&self) -> VDD_SPI_EN_INIT_R {
        VDD_SPI_EN_INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set SPI regulator to 1 to enable output current limit."]
    #[inline(always)]
    pub fn vdd_spi_encurlim(&self) -> VDD_SPI_ENCURLIM_R {
        VDD_SPI_ENCURLIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Tunes the current limit threshold of SPI regulator when tieh=0, about 800 mA/(8+d)."]
    #[inline(always)]
    pub fn vdd_spi_dcurlim(&self) -> VDD_SPI_DCURLIM_R {
        VDD_SPI_DCURLIM_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Adds resistor from LDO output to ground. 0: no resistance 1: 6 K 2: 4 K 3: 2 K."]
    #[inline(always)]
    pub fn vdd_spi_init(&self) -> VDD_SPI_INIT_R {
        VDD_SPI_INIT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Prevents SPI regulator from overshoot."]
    #[inline(always)]
    pub fn vdd_spi_dcap(&self) -> VDD_SPI_DCAP_R {
        VDD_SPI_DCAP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Selects RTC watchdog timeout threshold, in unit of slow clock cycle. 0: 40000. 1: 80000. 2: 160000. 3:320000."]
    #[inline(always)]
    pub fn wdt_delay_sel(&self) -> WDT_DELAY_SEL_R {
        WDT_DELAY_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Set this bit to enable SPI boot encrypt/decrypt. Odd number of 1: enable. even number of 1: disable."]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt(&self) -> SPI_BOOT_CRYPT_CNT_R {
        SPI_BOOT_CRYPT_CNT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - Set this bit to enable revoking first secure boot key."]
    #[inline(always)]
    pub fn secure_boot_key_revoke0(&self) -> SECURE_BOOT_KEY_REVOKE0_R {
        SECURE_BOOT_KEY_REVOKE0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to enable revoking second secure boot key."]
    #[inline(always)]
    pub fn secure_boot_key_revoke1(&self) -> SECURE_BOOT_KEY_REVOKE1_R {
        SECURE_BOOT_KEY_REVOKE1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to enable revoking third secure boot key."]
    #[inline(always)]
    pub fn secure_boot_key_revoke2(&self) -> SECURE_BOOT_KEY_REVOKE2_R {
        SECURE_BOOT_KEY_REVOKE2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Purpose of Key0."]
    #[inline(always)]
    pub fn key_purpose_0(&self) -> KEY_PURPOSE_0_R {
        KEY_PURPOSE_0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Purpose of Key1."]
    #[inline(always)]
    pub fn key_purpose_1(&self) -> KEY_PURPOSE_1_R {
        KEY_PURPOSE_1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA1")
            .field("vdd_spi_drefm", &self.vdd_spi_drefm())
            .field("vdd_spi_drefl", &self.vdd_spi_drefl())
            .field("vdd_spi_xpd", &self.vdd_spi_xpd())
            .field("vdd_spi_tieh", &self.vdd_spi_tieh())
            .field("vdd_spi_force", &self.vdd_spi_force())
            .field("vdd_spi_en_init", &self.vdd_spi_en_init())
            .field("vdd_spi_encurlim", &self.vdd_spi_encurlim())
            .field("vdd_spi_dcurlim", &self.vdd_spi_dcurlim())
            .field("vdd_spi_init", &self.vdd_spi_init())
            .field("vdd_spi_dcap", &self.vdd_spi_dcap())
            .field("wdt_delay_sel", &self.wdt_delay_sel())
            .field("spi_boot_crypt_cnt", &self.spi_boot_crypt_cnt())
            .field("secure_boot_key_revoke0", &self.secure_boot_key_revoke0())
            .field("secure_boot_key_revoke1", &self.secure_boot_key_revoke1())
            .field("secure_boot_key_revoke2", &self.secure_boot_key_revoke2())
            .field("key_purpose_0", &self.key_purpose_0())
            .field("key_purpose_1", &self.key_purpose_1())
            .finish()
    }
}
#[doc = "BLOCK0 data register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA1_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data1::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA1_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA1 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA1_SPEC {
    const RESET_VALUE: u32 = 0;
}
