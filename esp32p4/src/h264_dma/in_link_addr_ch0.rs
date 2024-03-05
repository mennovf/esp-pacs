#[doc = "Register `IN_LINK_ADDR_CH0` reader"]
pub type R = crate::R<IN_LINK_ADDR_CH0_SPEC>;
#[doc = "Register `IN_LINK_ADDR_CH0` writer"]
pub type W = crate::W<IN_LINK_ADDR_CH0_SPEC>;
#[doc = "Field `INLINK_ADDR_CH0` reader - This register stores the first inlink descriptor's address."]
pub type INLINK_ADDR_CH0_R = crate::FieldReader<u32>;
#[doc = "Field `INLINK_ADDR_CH0` writer - This register stores the first inlink descriptor's address."]
pub type INLINK_ADDR_CH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr_ch0(&self) -> INLINK_ADDR_CH0_R {
        INLINK_ADDR_CH0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_LINK_ADDR_CH0")
            .field(
                "inlink_addr_ch0",
                &format_args!("{}", self.inlink_addr_ch0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_LINK_ADDR_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the first inlink descriptor's address."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_addr_ch0(&mut self) -> INLINK_ADDR_CH0_W<IN_LINK_ADDR_CH0_SPEC> {
        INLINK_ADDR_CH0_W::new(self, 0)
    }
}
#[doc = "RX CH0 in_link dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_addr_ch0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_addr_ch0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_LINK_ADDR_CH0_SPEC;
impl crate::RegisterSpec for IN_LINK_ADDR_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_link_addr_ch0::R`](R) reader structure"]
impl crate::Readable for IN_LINK_ADDR_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_link_addr_ch0::W`](W) writer structure"]
impl crate::Writable for IN_LINK_ADDR_CH0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_LINK_ADDR_CH0 to value 0"]
impl crate::Resettable for IN_LINK_ADDR_CH0_SPEC {
    const RESET_VALUE: u32 = 0;
}
