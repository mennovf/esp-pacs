#[doc = "Register `WR_TIM_CONF1` reader"]
pub type R = crate::R<WR_TIM_CONF1_SPEC>;
#[doc = "Register `WR_TIM_CONF1` writer"]
pub type W = crate::W<WR_TIM_CONF1_SPEC>;
#[doc = "Field `TSUP_A` reader - Configures the programming setup time."]
pub type TSUP_A_R = crate::FieldReader;
#[doc = "Field `TSUP_A` writer - Configures the programming setup time."]
pub type TSUP_A_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWR_ON_NUM` reader - Configures the power up time for VDDQ."]
pub type PWR_ON_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `PWR_ON_NUM` writer - Configures the power up time for VDDQ."]
pub type PWR_ON_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `THP_A` reader - Configures the programming hold time."]
pub type THP_A_R = crate::FieldReader;
#[doc = "Field `THP_A` writer - Configures the programming hold time."]
pub type THP_A_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures the programming setup time."]
    #[inline(always)]
    pub fn tsup_a(&self) -> TSUP_A_R {
        TSUP_A_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - Configures the power up time for VDDQ."]
    #[inline(always)]
    pub fn pwr_on_num(&self) -> PWR_ON_NUM_R {
        PWR_ON_NUM_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - Configures the programming hold time."]
    #[inline(always)]
    pub fn thp_a(&self) -> THP_A_R {
        THP_A_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WR_TIM_CONF1")
            .field("tsup_a", &format_args!("{}", self.tsup_a().bits()))
            .field("pwr_on_num", &format_args!("{}", self.pwr_on_num().bits()))
            .field("thp_a", &format_args!("{}", self.thp_a().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WR_TIM_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the programming setup time."]
    #[inline(always)]
    #[must_use]
    pub fn tsup_a(&mut self) -> TSUP_A_W<WR_TIM_CONF1_SPEC> {
        TSUP_A_W::new(self, 0)
    }
    #[doc = "Bits 8:23 - Configures the power up time for VDDQ."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_on_num(&mut self) -> PWR_ON_NUM_W<WR_TIM_CONF1_SPEC> {
        PWR_ON_NUM_W::new(self, 8)
    }
    #[doc = "Bits 24:31 - Configures the programming hold time."]
    #[inline(always)]
    #[must_use]
    pub fn thp_a(&mut self) -> THP_A_W<WR_TIM_CONF1_SPEC> {
        THP_A_W::new(self, 24)
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
#[doc = "Configurarion register 1 of eFuse programming timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_TIM_CONF1_SPEC;
impl crate::RegisterSpec for WR_TIM_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_tim_conf1::R`](R) reader structure"]
impl crate::Readable for WR_TIM_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wr_tim_conf1::W`](W) writer structure"]
impl crate::Writable for WR_TIM_CONF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_TIM_CONF1 to value 0x0126_6701"]
impl crate::Resettable for WR_TIM_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0126_6701;
}
