#[doc = "Register `PRO_BT_BB_INT_MAP` reader"]
pub type R = crate::R<PRO_BT_BB_INT_MAP_SPEC>;
#[doc = "Register `PRO_BT_BB_INT_MAP` writer"]
pub type W = crate::W<PRO_BT_BB_INT_MAP_SPEC>;
#[doc = "Field `PRO_BT_BB_INT_MAP` reader - This register is used to map BT_BB_INT interrupt signal to one of the CPU interrupts."]
pub type PRO_BT_BB_INT_MAP_R = crate::FieldReader;
#[doc = "Field `PRO_BT_BB_INT_MAP` writer - This register is used to map BT_BB_INT interrupt signal to one of the CPU interrupts."]
pub type PRO_BT_BB_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - This register is used to map BT_BB_INT interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    pub fn pro_bt_bb_int_map(&self) -> PRO_BT_BB_INT_MAP_R {
        PRO_BT_BB_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_BT_BB_INT_MAP")
            .field(
                "pro_bt_bb_int_map",
                &format_args!("{}", self.pro_bt_bb_int_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_BT_BB_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register is used to map BT_BB_INT interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn pro_bt_bb_int_map(&mut self) -> PRO_BT_BB_INT_MAP_W<PRO_BT_BB_INT_MAP_SPEC> {
        PRO_BT_BB_INT_MAP_W::new(self, 0)
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
#[doc = "BT_BB_INT interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_bt_bb_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_bt_bb_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_BT_BB_INT_MAP_SPEC;
impl crate::RegisterSpec for PRO_BT_BB_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_bt_bb_int_map::R`](R) reader structure"]
impl crate::Readable for PRO_BT_BB_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_bt_bb_int_map::W`](W) writer structure"]
impl crate::Writable for PRO_BT_BB_INT_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_BT_BB_INT_MAP to value 0x10"]
impl crate::Resettable for PRO_BT_BB_INT_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
