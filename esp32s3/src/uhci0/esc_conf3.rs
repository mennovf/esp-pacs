#[doc = "Register `ESC_CONF3` reader"]
pub type R = crate::R<ESC_CONF3_SPEC>;
#[doc = "Register `ESC_CONF3` writer"]
pub type W = crate::W<ESC_CONF3_SPEC>;
#[doc = "Field `ESC_SEQ2` reader - This register is used to define a char that need to be decoded, default is 0x13 that used as flow control char."]
pub type ESC_SEQ2_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ2` writer - This register is used to define a char that need to be decoded, default is 0x13 that used as flow control char."]
pub type ESC_SEQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ESC_SEQ2_CHAR0` reader - This register is used to define the first char of slip escape sequence when encoding the UHCI_ESC_SEQ2, default is 0xdb."]
pub type ESC_SEQ2_CHAR0_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ2_CHAR0` writer - This register is used to define the first char of slip escape sequence when encoding the UHCI_ESC_SEQ2, default is 0xdb."]
pub type ESC_SEQ2_CHAR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ESC_SEQ2_CHAR1` reader - This register is used to define the second char of slip escape sequence when encoding the UHCI_ESC_SEQ2, default is 0xdf."]
pub type ESC_SEQ2_CHAR1_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ2_CHAR1` writer - This register is used to define the second char of slip escape sequence when encoding the UHCI_ESC_SEQ2, default is 0xdf."]
pub type ESC_SEQ2_CHAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register is used to define a char that need to be decoded, default is 0x13 that used as flow control char."]
    #[inline(always)]
    pub fn esc_seq2(&self) -> ESC_SEQ2_R {
        ESC_SEQ2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register is used to define the first char of slip escape sequence when encoding the UHCI_ESC_SEQ2, default is 0xdb."]
    #[inline(always)]
    pub fn esc_seq2_char0(&self) -> ESC_SEQ2_CHAR0_R {
        ESC_SEQ2_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register is used to define the second char of slip escape sequence when encoding the UHCI_ESC_SEQ2, default is 0xdf."]
    #[inline(always)]
    pub fn esc_seq2_char1(&self) -> ESC_SEQ2_CHAR1_R {
        ESC_SEQ2_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESC_CONF3")
            .field("esc_seq2", &format_args!("{}", self.esc_seq2().bits()))
            .field(
                "esc_seq2_char0",
                &format_args!("{}", self.esc_seq2_char0().bits()),
            )
            .field(
                "esc_seq2_char1",
                &format_args!("{}", self.esc_seq2_char1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ESC_CONF3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to define a char that need to be decoded, default is 0x13 that used as flow control char."]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq2(&mut self) -> ESC_SEQ2_W<ESC_CONF3_SPEC> {
        ESC_SEQ2_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - This register is used to define the first char of slip escape sequence when encoding the UHCI_ESC_SEQ2, default is 0xdb."]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq2_char0(&mut self) -> ESC_SEQ2_CHAR0_W<ESC_CONF3_SPEC> {
        ESC_SEQ2_CHAR0_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - This register is used to define the second char of slip escape sequence when encoding the UHCI_ESC_SEQ2, default is 0xdf."]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq2_char1(&mut self) -> ESC_SEQ2_CHAR1_W<ESC_CONF3_SPEC> {
        ESC_SEQ2_CHAR1_W::new(self, 16)
    }
}
#[doc = "Escape sequence configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_conf3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esc_conf3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESC_CONF3_SPEC;
impl crate::RegisterSpec for ESC_CONF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esc_conf3::R`](R) reader structure"]
impl crate::Readable for ESC_CONF3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`esc_conf3::W`](W) writer structure"]
impl crate::Writable for ESC_CONF3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESC_CONF3 to value 0x00df_db13"]
impl crate::Resettable for ESC_CONF3_SPEC {
    const RESET_VALUE: u32 = 0x00df_db13;
}
