#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE_INT_ENA` reader - need_des"]
pub type VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE_INT_ENA` writer - need_des"]
pub type VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA` reader - need_des"]
pub type VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA` writer - need_des"]
pub type VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_UPVOLTAGE_INT_ENA` reader - need_des"]
pub type VDDBAT_UPVOLTAGE_INT_ENA_R = crate::BitReader;
#[doc = "Field `VDDBAT_UPVOLTAGE_INT_ENA` writer - need_des"]
pub type VDDBAT_UPVOLTAGE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_UNDERVOLTAGE_INT_ENA` reader - need_des"]
pub type VDDBAT_UNDERVOLTAGE_INT_ENA_R = crate::BitReader;
#[doc = "Field `VDDBAT_UNDERVOLTAGE_INT_ENA` writer - need_des"]
pub type VDDBAT_UNDERVOLTAGE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD_MODE0_INT_ENA` reader - need_des"]
pub type BOD_MODE0_INT_ENA_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_INT_ENA` writer - need_des"]
pub type BOD_MODE0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_upvoltage_int_ena(&self) -> VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_R {
        VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_undervoltage_int_ena(&self) -> VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_R {
        VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn vddbat_upvoltage_int_ena(&self) -> VDDBAT_UPVOLTAGE_INT_ENA_R {
        VDDBAT_UPVOLTAGE_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn vddbat_undervoltage_int_ena(&self) -> VDDBAT_UNDERVOLTAGE_INT_ENA_R {
        VDDBAT_UNDERVOLTAGE_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn bod_mode0_int_ena(&self) -> BOD_MODE0_INT_ENA_R {
        BOD_MODE0_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "vddbat_charge_upvoltage_int_ena",
                &format_args!("{}", self.vddbat_charge_upvoltage_int_ena().bit()),
            )
            .field(
                "vddbat_charge_undervoltage_int_ena",
                &format_args!("{}", self.vddbat_charge_undervoltage_int_ena().bit()),
            )
            .field(
                "vddbat_upvoltage_int_ena",
                &format_args!("{}", self.vddbat_upvoltage_int_ena().bit()),
            )
            .field(
                "vddbat_undervoltage_int_ena",
                &format_args!("{}", self.vddbat_undervoltage_int_ena().bit()),
            )
            .field(
                "bod_mode0_int_ena",
                &format_args!("{}", self.bod_mode0_int_ena().bit()),
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
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_upvoltage_int_ena(
        &mut self,
    ) -> VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_W<INT_ENA_SPEC> {
        VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_undervoltage_int_ena(
        &mut self,
    ) -> VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_W<INT_ENA_SPEC> {
        VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_upvoltage_int_ena(&mut self) -> VDDBAT_UPVOLTAGE_INT_ENA_W<INT_ENA_SPEC> {
        VDDBAT_UPVOLTAGE_INT_ENA_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_undervoltage_int_ena(&mut self) -> VDDBAT_UNDERVOLTAGE_INT_ENA_W<INT_ENA_SPEC> {
        VDDBAT_UNDERVOLTAGE_INT_ENA_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0_int_ena(&mut self) -> BOD_MODE0_INT_ENA_W<INT_ENA_SPEC> {
        BOD_MODE0_INT_ENA_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
