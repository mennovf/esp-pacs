#[doc = "Register `CMD7` reader"]
pub type R = crate::R<CMD7_SPEC>;
#[doc = "Register `CMD7` writer"]
pub type W = crate::W<CMD7_SPEC>;
#[doc = "Field `COMMAND7` reader - Content of command 7. For more information, please refer to the register I2C_COMD7_REG in Chapter I²C Controller."]
pub type COMMAND7_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND7` writer - Content of command 7. For more information, please refer to the register I2C_COMD7_REG in Chapter I²C Controller."]
pub type COMMAND7_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND7_DONE` reader - When command 7 is done, this bit changes to 1."]
pub type COMMAND7_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - Content of command 7. For more information, please refer to the register I2C_COMD7_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command7(&self) -> COMMAND7_R {
        COMMAND7_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 7 is done, this bit changes to 1."]
    #[inline(always)]
    pub fn command7_done(&self) -> COMMAND7_DONE_R {
        COMMAND7_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD7")
            .field("command7", &format_args!("{}", self.command7().bits()))
            .field(
                "command7_done",
                &format_args!("{}", self.command7_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Content of command 7. For more information, please refer to the register I2C_COMD7_REG in Chapter I²C Controller."]
    #[inline(always)]
    #[must_use]
    pub fn command7(&mut self) -> COMMAND7_W<CMD7_SPEC> {
        COMMAND7_W::new(self, 0)
    }
}
#[doc = "RTC I2C Command 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD7_SPEC;
impl crate::RegisterSpec for CMD7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd7::R`](R) reader structure"]
impl crate::Readable for CMD7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd7::W`](W) writer structure"]
impl crate::Writable for CMD7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD7 to value 0x0904"]
impl crate::Resettable for CMD7_SPEC {
    const RESET_VALUE: u32 = 0x0904;
}
