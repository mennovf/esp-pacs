#[doc = "Register `Core_0_STATUSTABLE11` reader"]
pub type R = crate::R<CORE_0_STATUSTABLE11_SPEC>;
#[doc = "Register `Core_0_STATUSTABLE11` writer"]
pub type W = crate::W<CORE_0_STATUSTABLE11_SPEC>;
#[doc = "Field `CORE_0_FROM_WORLD_11` reader - This bit is used to confirm world before enter entry 11"]
pub type CORE_0_FROM_WORLD_11_R = crate::BitReader;
#[doc = "Field `CORE_0_FROM_WORLD_11` writer - This bit is used to confirm world before enter entry 11"]
pub type CORE_0_FROM_WORLD_11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_FROM_ENTRY_11` reader - This filed is used to confirm in which entry before enter entry 11"]
pub type CORE_0_FROM_ENTRY_11_R = crate::FieldReader;
#[doc = "Field `CORE_0_FROM_ENTRY_11` writer - This filed is used to confirm in which entry before enter entry 11"]
pub type CORE_0_FROM_ENTRY_11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CORE_0_CURRENT_11` reader - This bit is used to confirm whether the current state is in entry 11"]
pub type CORE_0_CURRENT_11_R = crate::BitReader;
#[doc = "Field `CORE_0_CURRENT_11` writer - This bit is used to confirm whether the current state is in entry 11"]
pub type CORE_0_CURRENT_11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 11"]
    #[inline(always)]
    pub fn core_0_from_world_11(&self) -> CORE_0_FROM_WORLD_11_R {
        CORE_0_FROM_WORLD_11_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 11"]
    #[inline(always)]
    pub fn core_0_from_entry_11(&self) -> CORE_0_FROM_ENTRY_11_R {
        CORE_0_FROM_ENTRY_11_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 11"]
    #[inline(always)]
    pub fn core_0_current_11(&self) -> CORE_0_CURRENT_11_R {
        CORE_0_CURRENT_11_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_STATUSTABLE11")
            .field(
                "core_0_from_world_11",
                &format_args!("{}", self.core_0_from_world_11().bit()),
            )
            .field(
                "core_0_from_entry_11",
                &format_args!("{}", self.core_0_from_entry_11().bits()),
            )
            .field(
                "core_0_current_11",
                &format_args!("{}", self.core_0_current_11().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_STATUSTABLE11_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 11"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_from_world_11(&mut self) -> CORE_0_FROM_WORLD_11_W<CORE_0_STATUSTABLE11_SPEC> {
        CORE_0_FROM_WORLD_11_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 11"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_from_entry_11(&mut self) -> CORE_0_FROM_ENTRY_11_W<CORE_0_STATUSTABLE11_SPEC> {
        CORE_0_FROM_ENTRY_11_W::new(self, 1)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 11"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_current_11(&mut self) -> CORE_0_CURRENT_11_W<CORE_0_STATUSTABLE11_SPEC> {
        CORE_0_CURRENT_11_W::new(self, 5)
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
#[doc = "Status register of world switch of entry 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_statustable11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_statustable11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_STATUSTABLE11_SPEC;
impl crate::RegisterSpec for CORE_0_STATUSTABLE11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_statustable11::R`](R) reader structure"]
impl crate::Readable for CORE_0_STATUSTABLE11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_statustable11::W`](W) writer structure"]
impl crate::Writable for CORE_0_STATUSTABLE11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_0_STATUSTABLE11 to value 0"]
impl crate::Resettable for CORE_0_STATUSTABLE11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
