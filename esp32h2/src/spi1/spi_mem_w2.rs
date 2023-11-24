#[doc = "Register `SPI_MEM_W2` reader"]
pub type R = crate::R<SPI_MEM_W2_SPEC>;
#[doc = "Register `SPI_MEM_W2` writer"]
pub type W = crate::W<SPI_MEM_W2_SPEC>;
#[doc = "Field `SPI_MEM_BUF2` reader - data buffer"]
pub type SPI_MEM_BUF2_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_MEM_BUF2` writer - data buffer"]
pub type SPI_MEM_BUF2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_mem_buf2(&self) -> SPI_MEM_BUF2_R {
        SPI_MEM_BUF2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_W2")
            .field(
                "spi_mem_buf2",
                &format_args!("{}", self.spi_mem_buf2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_W2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_buf2(&mut self) -> SPI_MEM_BUF2_W<SPI_MEM_W2_SPEC> {
        SPI_MEM_BUF2_W::new(self, 0)
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
#[doc = "SPI1 memory data buffer2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_w2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_w2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_W2_SPEC;
impl crate::RegisterSpec for SPI_MEM_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_w2::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_W2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_w2::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_W2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_W2 to value 0"]
impl crate::Resettable for SPI_MEM_W2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
