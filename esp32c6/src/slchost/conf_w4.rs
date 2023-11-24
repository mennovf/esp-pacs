#[doc = "Register `CONF_W4` reader"]
pub type R = crate::R<CONF_W4_SPEC>;
#[doc = "Register `CONF_W4` writer"]
pub type W = crate::W<CONF_W4_SPEC>;
#[doc = "Field `SLCHOST_CONF16` reader - *******Description***********"]
pub type SLCHOST_CONF16_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF16` writer - *******Description***********"]
pub type SLCHOST_CONF16_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLCHOST_CONF17` reader - *******Description***********"]
pub type SLCHOST_CONF17_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF17` writer - *******Description***********"]
pub type SLCHOST_CONF17_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLCHOST_CONF18` reader - *******Description***********"]
pub type SLCHOST_CONF18_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF18` writer - *******Description***********"]
pub type SLCHOST_CONF18_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLCHOST_CONF19` reader - *******Description***********"]
pub type SLCHOST_CONF19_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF19` writer - *******Description***********"]
pub type SLCHOST_CONF19_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf16(&self) -> SLCHOST_CONF16_R {
        SLCHOST_CONF16_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf17(&self) -> SLCHOST_CONF17_R {
        SLCHOST_CONF17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf18(&self) -> SLCHOST_CONF18_R {
        SLCHOST_CONF18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf19(&self) -> SLCHOST_CONF19_R {
        SLCHOST_CONF19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W4")
            .field(
                "slchost_conf16",
                &format_args!("{}", self.slchost_conf16().bits()),
            )
            .field(
                "slchost_conf17",
                &format_args!("{}", self.slchost_conf17().bits()),
            )
            .field(
                "slchost_conf18",
                &format_args!("{}", self.slchost_conf18().bits()),
            )
            .field(
                "slchost_conf19",
                &format_args!("{}", self.slchost_conf19().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf16(&mut self) -> SLCHOST_CONF16_W<CONF_W4_SPEC> {
        SLCHOST_CONF16_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf17(&mut self) -> SLCHOST_CONF17_W<CONF_W4_SPEC> {
        SLCHOST_CONF17_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf18(&mut self) -> SLCHOST_CONF18_W<CONF_W4_SPEC> {
        SLCHOST_CONF18_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf19(&mut self) -> SLCHOST_CONF19_W<CONF_W4_SPEC> {
        SLCHOST_CONF19_W::new(self, 24)
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
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W4_SPEC;
impl crate::RegisterSpec for CONF_W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w4::R`](R) reader structure"]
impl crate::Readable for CONF_W4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w4::W`](W) writer structure"]
impl crate::Writable for CONF_W4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W4 to value 0x01ff"]
impl crate::Resettable for CONF_W4_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff;
}
