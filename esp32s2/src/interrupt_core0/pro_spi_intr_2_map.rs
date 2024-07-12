#[doc = "Register `PRO_SPI_INTR_2_MAP` reader"]
pub type R = crate::R<PRO_SPI_INTR_2_MAP_SPEC>;
#[doc = "Register `PRO_SPI_INTR_2_MAP` writer"]
pub type W = crate::W<PRO_SPI_INTR_2_MAP_SPEC>;
#[doc = "Field `PRO_SPI_INTR_2_MAP` reader - This register is used to map SPI_INTR_2 interrupt signal to one of the CPU interrupts."]
pub type PRO_SPI_INTR_2_MAP_R = crate::FieldReader;
#[doc = "Field `PRO_SPI_INTR_2_MAP` writer - This register is used to map SPI_INTR_2 interrupt signal to one of the CPU interrupts."]
pub type PRO_SPI_INTR_2_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - This register is used to map SPI_INTR_2 interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    pub fn pro_spi_intr_2_map(&self) -> PRO_SPI_INTR_2_MAP_R {
        PRO_SPI_INTR_2_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_SPI_INTR_2_MAP")
            .field("pro_spi_intr_2_map", &self.pro_spi_intr_2_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - This register is used to map SPI_INTR_2 interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn pro_spi_intr_2_map(&mut self) -> PRO_SPI_INTR_2_MAP_W<PRO_SPI_INTR_2_MAP_SPEC> {
        PRO_SPI_INTR_2_MAP_W::new(self, 0)
    }
}
#[doc = "SPI_INTR_2 interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_spi_intr_2_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_spi_intr_2_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_SPI_INTR_2_MAP_SPEC;
impl crate::RegisterSpec for PRO_SPI_INTR_2_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_spi_intr_2_map::R`](R) reader structure"]
impl crate::Readable for PRO_SPI_INTR_2_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_spi_intr_2_map::W`](W) writer structure"]
impl crate::Writable for PRO_SPI_INTR_2_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_SPI_INTR_2_MAP to value 0x10"]
impl crate::Resettable for PRO_SPI_INTR_2_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
