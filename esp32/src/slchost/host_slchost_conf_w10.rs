#[doc = "Register `HOST_SLCHOST_CONF_W10` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W10_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W10` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W10_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF40` reader - "]
pub type HOST_SLCHOST_CONF40_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF40` writer - "]
pub type HOST_SLCHOST_CONF40_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF41` reader - "]
pub type HOST_SLCHOST_CONF41_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF41` writer - "]
pub type HOST_SLCHOST_CONF41_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF42` reader - "]
pub type HOST_SLCHOST_CONF42_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF42` writer - "]
pub type HOST_SLCHOST_CONF42_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF43` reader - "]
pub type HOST_SLCHOST_CONF43_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF43` writer - "]
pub type HOST_SLCHOST_CONF43_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf40(&self) -> HOST_SLCHOST_CONF40_R {
        HOST_SLCHOST_CONF40_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf41(&self) -> HOST_SLCHOST_CONF41_R {
        HOST_SLCHOST_CONF41_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf42(&self) -> HOST_SLCHOST_CONF42_R {
        HOST_SLCHOST_CONF42_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf43(&self) -> HOST_SLCHOST_CONF43_R {
        HOST_SLCHOST_CONF43_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W10")
            .field(
                "host_slchost_conf40",
                &format_args!("{}", self.host_slchost_conf40().bits()),
            )
            .field(
                "host_slchost_conf41",
                &format_args!("{}", self.host_slchost_conf41().bits()),
            )
            .field(
                "host_slchost_conf42",
                &format_args!("{}", self.host_slchost_conf42().bits()),
            )
            .field(
                "host_slchost_conf43",
                &format_args!("{}", self.host_slchost_conf43().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_CONF_W10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf40(&mut self) -> HOST_SLCHOST_CONF40_W<HOST_SLCHOST_CONF_W10_SPEC> {
        HOST_SLCHOST_CONF40_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf41(&mut self) -> HOST_SLCHOST_CONF41_W<HOST_SLCHOST_CONF_W10_SPEC> {
        HOST_SLCHOST_CONF41_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf42(&mut self) -> HOST_SLCHOST_CONF42_W<HOST_SLCHOST_CONF_W10_SPEC> {
        HOST_SLCHOST_CONF42_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf43(&mut self) -> HOST_SLCHOST_CONF43_W<HOST_SLCHOST_CONF_W10_SPEC> {
        HOST_SLCHOST_CONF43_W::new(self, 24)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W10_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w10::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w10::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W10 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
