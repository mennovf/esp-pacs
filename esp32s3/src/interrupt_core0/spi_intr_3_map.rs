#[doc = "Register `SPI_INTR_3_MAP` reader"]
pub type R = crate::R<SPI_INTR_3_MAP_SPEC>;
#[doc = "Register `SPI_INTR_3_MAP` writer"]
pub type W = crate::W<SPI_INTR_3_MAP_SPEC>;
#[doc = "Field `SPI_INTR_3_MAP` reader - this register used to map spi_intr_3 interrupt to one of core0's external interrupt"]
pub type SPI_INTR_3_MAP_R = crate::FieldReader;
#[doc = "Field `SPI_INTR_3_MAP` writer - this register used to map spi_intr_3 interrupt to one of core0's external interrupt"]
pub type SPI_INTR_3_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map spi_intr_3 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn spi_intr_3_map(&self) -> SPI_INTR_3_MAP_R {
        SPI_INTR_3_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_INTR_3_MAP")
            .field("spi_intr_3_map", &self.spi_intr_3_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map spi_intr_3 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn spi_intr_3_map(&mut self) -> SPI_INTR_3_MAP_W<SPI_INTR_3_MAP_SPEC> {
        SPI_INTR_3_MAP_W::new(self, 0)
    }
}
#[doc = "spi_intr_3 interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_intr_3_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_intr_3_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_INTR_3_MAP_SPEC;
impl crate::RegisterSpec for SPI_INTR_3_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_intr_3_map::R`](R) reader structure"]
impl crate::Readable for SPI_INTR_3_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_intr_3_map::W`](W) writer structure"]
impl crate::Writable for SPI_INTR_3_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_INTR_3_MAP to value 0x10"]
impl crate::Resettable for SPI_INTR_3_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
