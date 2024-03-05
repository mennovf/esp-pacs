#[doc = "Register `GEN%s_STMP_CFG` reader"]
pub type R = crate::R<GEN_STMP_CFG_SPEC>;
#[doc = "Register `GEN%s_STMP_CFG` writer"]
pub type W = crate::W<GEN_STMP_CFG_SPEC>;
#[doc = "Field `CMPR_A_UPMETHOD` reader - Configures the update method for PWM generator %s time stamp A's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type CMPR_A_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `CMPR_A_UPMETHOD` writer - Configures the update method for PWM generator %s time stamp A's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type CMPR_A_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CMPR_B_UPMETHOD` reader - Configures the update method for PWM generator %s time stamp B's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type CMPR_B_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `CMPR_B_UPMETHOD` writer - Configures the update method for PWM generator %s time stamp B's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type CMPR_B_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CMPR_A_SHDW_FULL` reader - Represents whether or not generator%s time stamp A's shadow reg is transferred.\\\\0: A's active reg has been updated with shadow register latest value.\\\\1: A's shadow reg is filled and waiting to be transferred to A's active reg"]
pub type CMPR_A_SHDW_FULL_R = crate::BitReader;
#[doc = "Field `CMPR_A_SHDW_FULL` writer - Represents whether or not generator%s time stamp A's shadow reg is transferred.\\\\0: A's active reg has been updated with shadow register latest value.\\\\1: A's shadow reg is filled and waiting to be transferred to A's active reg"]
pub type CMPR_A_SHDW_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR_B_SHDW_FULL` reader - Represents whether or not generator%s time stamp B's shadow reg is transferred.\\\\0: B's active reg has been updated with shadow register latest value.\\\\1: B's shadow reg is filled and waiting to be transferred to B's active reg"]
pub type CMPR_B_SHDW_FULL_R = crate::BitReader;
#[doc = "Field `CMPR_B_SHDW_FULL` writer - Represents whether or not generator%s time stamp B's shadow reg is transferred.\\\\0: B's active reg has been updated with shadow register latest value.\\\\1: B's shadow reg is filled and waiting to be transferred to B's active reg"]
pub type CMPR_B_SHDW_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Configures the update method for PWM generator %s time stamp A's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn cmpr_a_upmethod(&self) -> CMPR_A_UPMETHOD_R {
        CMPR_A_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configures the update method for PWM generator %s time stamp B's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn cmpr_b_upmethod(&self) -> CMPR_B_UPMETHOD_R {
        CMPR_B_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Represents whether or not generator%s time stamp A's shadow reg is transferred.\\\\0: A's active reg has been updated with shadow register latest value.\\\\1: A's shadow reg is filled and waiting to be transferred to A's active reg"]
    #[inline(always)]
    pub fn cmpr_a_shdw_full(&self) -> CMPR_A_SHDW_FULL_R {
        CMPR_A_SHDW_FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents whether or not generator%s time stamp B's shadow reg is transferred.\\\\0: B's active reg has been updated with shadow register latest value.\\\\1: B's shadow reg is filled and waiting to be transferred to B's active reg"]
    #[inline(always)]
    pub fn cmpr_b_shdw_full(&self) -> CMPR_B_SHDW_FULL_R {
        CMPR_B_SHDW_FULL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN_STMP_CFG")
            .field(
                "cmpr_a_upmethod",
                &format_args!("{}", self.cmpr_a_upmethod().bits()),
            )
            .field(
                "cmpr_b_upmethod",
                &format_args!("{}", self.cmpr_b_upmethod().bits()),
            )
            .field(
                "cmpr_a_shdw_full",
                &format_args!("{}", self.cmpr_a_shdw_full().bit()),
            )
            .field(
                "cmpr_b_shdw_full",
                &format_args!("{}", self.cmpr_b_shdw_full().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN_STMP_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures the update method for PWM generator %s time stamp A's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr_a_upmethod(&mut self) -> CMPR_A_UPMETHOD_W<GEN_STMP_CFG_SPEC> {
        CMPR_A_UPMETHOD_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configures the update method for PWM generator %s time stamp B's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr_b_upmethod(&mut self) -> CMPR_B_UPMETHOD_W<GEN_STMP_CFG_SPEC> {
        CMPR_B_UPMETHOD_W::new(self, 4)
    }
    #[doc = "Bit 8 - Represents whether or not generator%s time stamp A's shadow reg is transferred.\\\\0: A's active reg has been updated with shadow register latest value.\\\\1: A's shadow reg is filled and waiting to be transferred to A's active reg"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr_a_shdw_full(&mut self) -> CMPR_A_SHDW_FULL_W<GEN_STMP_CFG_SPEC> {
        CMPR_A_SHDW_FULL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents whether or not generator%s time stamp B's shadow reg is transferred.\\\\0: B's active reg has been updated with shadow register latest value.\\\\1: B's shadow reg is filled and waiting to be transferred to B's active reg"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr_b_shdw_full(&mut self) -> CMPR_B_SHDW_FULL_W<GEN_STMP_CFG_SPEC> {
        CMPR_B_SHDW_FULL_W::new(self, 9)
    }
}
#[doc = "Generator%s time stamp registers A and B transfer status and update method register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_stmp_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_stmp_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN_STMP_CFG_SPEC;
impl crate::RegisterSpec for GEN_STMP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_stmp_cfg::R`](R) reader structure"]
impl crate::Readable for GEN_STMP_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen_stmp_cfg::W`](W) writer structure"]
impl crate::Writable for GEN_STMP_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN%s_STMP_CFG to value 0"]
impl crate::Resettable for GEN_STMP_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
