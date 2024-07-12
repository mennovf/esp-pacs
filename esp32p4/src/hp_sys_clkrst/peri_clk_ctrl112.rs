#[doc = "Register `PERI_CLK_CTRL112` reader"]
pub type R = crate::R<PERI_CLK_CTRL112_SPEC>;
#[doc = "Register `PERI_CLK_CTRL112` writer"]
pub type W = crate::W<PERI_CLK_CTRL112_SPEC>;
#[doc = "Field `UART1_SCLK_DIV_NUM` reader - Reserved"]
pub type UART1_SCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `UART1_SCLK_DIV_NUM` writer - Reserved"]
pub type UART1_SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UART1_SCLK_DIV_NUMERATOR` reader - Reserved"]
pub type UART1_SCLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `UART1_SCLK_DIV_NUMERATOR` writer - Reserved"]
pub type UART1_SCLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UART1_SCLK_DIV_DENOMINATOR` reader - Reserved"]
pub type UART1_SCLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `UART1_SCLK_DIV_DENOMINATOR` writer - Reserved"]
pub type UART1_SCLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UART2_CLK_SRC_SEL` reader - Reserved"]
pub type UART2_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `UART2_CLK_SRC_SEL` writer - Reserved"]
pub type UART2_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART2_CLK_EN` reader - Reserved"]
pub type UART2_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART2_CLK_EN` writer - Reserved"]
pub type UART2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn uart1_sclk_div_num(&self) -> UART1_SCLK_DIV_NUM_R {
        UART1_SCLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn uart1_sclk_div_numerator(&self) -> UART1_SCLK_DIV_NUMERATOR_R {
        UART1_SCLK_DIV_NUMERATOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn uart1_sclk_div_denominator(&self) -> UART1_SCLK_DIV_DENOMINATOR_R {
        UART1_SCLK_DIV_DENOMINATOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Reserved"]
    #[inline(always)]
    pub fn uart2_clk_src_sel(&self) -> UART2_CLK_SRC_SEL_R {
        UART2_CLK_SRC_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn uart2_clk_en(&self) -> UART2_CLK_EN_R {
        UART2_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL112")
            .field("uart1_sclk_div_num", &self.uart1_sclk_div_num())
            .field("uart1_sclk_div_numerator", &self.uart1_sclk_div_numerator())
            .field(
                "uart1_sclk_div_denominator",
                &self.uart1_sclk_div_denominator(),
            )
            .field("uart2_clk_src_sel", &self.uart2_clk_src_sel())
            .field("uart2_clk_en", &self.uart2_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_sclk_div_num(&mut self) -> UART1_SCLK_DIV_NUM_W<PERI_CLK_CTRL112_SPEC> {
        UART1_SCLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_sclk_div_numerator(
        &mut self,
    ) -> UART1_SCLK_DIV_NUMERATOR_W<PERI_CLK_CTRL112_SPEC> {
        UART1_SCLK_DIV_NUMERATOR_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_sclk_div_denominator(
        &mut self,
    ) -> UART1_SCLK_DIV_DENOMINATOR_W<PERI_CLK_CTRL112_SPEC> {
        UART1_SCLK_DIV_DENOMINATOR_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn uart2_clk_src_sel(&mut self) -> UART2_CLK_SRC_SEL_W<PERI_CLK_CTRL112_SPEC> {
        UART2_CLK_SRC_SEL_W::new(self, 24)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn uart2_clk_en(&mut self) -> UART2_CLK_EN_W<PERI_CLK_CTRL112_SPEC> {
        UART2_CLK_EN_W::new(self, 26)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl112::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl112::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL112_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL112_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl112::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL112_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl112::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL112_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL112 to value 0x0400_0000"]
impl crate::Resettable for PERI_CLK_CTRL112_SPEC {
    const RESET_VALUE: u32 = 0x0400_0000;
}
