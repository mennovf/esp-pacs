#[doc = "Register `HOST_SLCHOST_CONF_W12` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W12_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W12` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W12_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF48` reader - "]
pub type HOST_SLCHOST_CONF48_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF48` writer - "]
pub type HOST_SLCHOST_CONF48_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF49` reader - "]
pub type HOST_SLCHOST_CONF49_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF49` writer - "]
pub type HOST_SLCHOST_CONF49_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF50` reader - "]
pub type HOST_SLCHOST_CONF50_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF50` writer - "]
pub type HOST_SLCHOST_CONF50_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF51` reader - "]
pub type HOST_SLCHOST_CONF51_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF51` writer - "]
pub type HOST_SLCHOST_CONF51_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf48(&self) -> HOST_SLCHOST_CONF48_R {
        HOST_SLCHOST_CONF48_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf49(&self) -> HOST_SLCHOST_CONF49_R {
        HOST_SLCHOST_CONF49_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf50(&self) -> HOST_SLCHOST_CONF50_R {
        HOST_SLCHOST_CONF50_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf51(&self) -> HOST_SLCHOST_CONF51_R {
        HOST_SLCHOST_CONF51_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W12")
            .field(
                "host_slchost_conf48",
                &format_args!("{}", self.host_slchost_conf48().bits()),
            )
            .field(
                "host_slchost_conf49",
                &format_args!("{}", self.host_slchost_conf49().bits()),
            )
            .field(
                "host_slchost_conf50",
                &format_args!("{}", self.host_slchost_conf50().bits()),
            )
            .field(
                "host_slchost_conf51",
                &format_args!("{}", self.host_slchost_conf51().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_CONF_W12_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf48(&mut self) -> HOST_SLCHOST_CONF48_W<HOST_SLCHOST_CONF_W12_SPEC> {
        HOST_SLCHOST_CONF48_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf49(&mut self) -> HOST_SLCHOST_CONF49_W<HOST_SLCHOST_CONF_W12_SPEC> {
        HOST_SLCHOST_CONF49_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf50(&mut self) -> HOST_SLCHOST_CONF50_W<HOST_SLCHOST_CONF_W12_SPEC> {
        HOST_SLCHOST_CONF50_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf51(&mut self) -> HOST_SLCHOST_CONF51_W<HOST_SLCHOST_CONF_W12_SPEC> {
        HOST_SLCHOST_CONF51_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W12_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w12::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w12::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W12_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W12 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W12_SPEC {
    const RESET_VALUE: u32 = 0;
}
