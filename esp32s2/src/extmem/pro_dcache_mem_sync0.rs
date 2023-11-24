#[doc = "Register `PRO_DCACHE_MEM_SYNC0` reader"]
pub type R = crate::R<PRO_DCACHE_MEM_SYNC0_SPEC>;
#[doc = "Register `PRO_DCACHE_MEM_SYNC0` writer"]
pub type W = crate::W<PRO_DCACHE_MEM_SYNC0_SPEC>;
#[doc = "Field `PRO_DCACHE_MEMSYNC_ADDR` reader - The bits are used to configure the start virtual address for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if the address is validate. The auto operations will be issued if the address is invalidate. It should be combined with PRO_DCACHE_MEM_SYNC1."]
pub type PRO_DCACHE_MEMSYNC_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_DCACHE_MEMSYNC_ADDR` writer - The bits are used to configure the start virtual address for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if the address is validate. The auto operations will be issued if the address is invalidate. It should be combined with PRO_DCACHE_MEM_SYNC1."]
pub type PRO_DCACHE_MEMSYNC_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if the address is validate. The auto operations will be issued if the address is invalidate. It should be combined with PRO_DCACHE_MEM_SYNC1."]
    #[inline(always)]
    pub fn pro_dcache_memsync_addr(&self) -> PRO_DCACHE_MEMSYNC_ADDR_R {
        PRO_DCACHE_MEMSYNC_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_MEM_SYNC0")
            .field(
                "pro_dcache_memsync_addr",
                &format_args!("{}", self.pro_dcache_memsync_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DCACHE_MEM_SYNC0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if the address is validate. The auto operations will be issued if the address is invalidate. It should be combined with PRO_DCACHE_MEM_SYNC1."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_memsync_addr(
        &mut self,
    ) -> PRO_DCACHE_MEMSYNC_ADDR_W<PRO_DCACHE_MEM_SYNC0_SPEC> {
        PRO_DCACHE_MEMSYNC_ADDR_W::new(self, 0)
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
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_mem_sync0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dcache_mem_sync0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_MEM_SYNC0_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_MEM_SYNC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_mem_sync0::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_MEM_SYNC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dcache_mem_sync0::W`](W) writer structure"]
impl crate::Writable for PRO_DCACHE_MEM_SYNC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_DCACHE_MEM_SYNC0 to value 0"]
impl crate::Resettable for PRO_DCACHE_MEM_SYNC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
