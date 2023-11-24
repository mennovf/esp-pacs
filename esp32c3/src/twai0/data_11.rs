#[doc = "Register `DATA_11` reader"]
pub type R = crate::R<DATA_11_SPEC>;
#[doc = "Register `DATA_11` writer"]
pub type W = crate::W<DATA_11_SPEC>;
#[doc = "Field `TX_BYTE_11` reader - In operation mode, it stores the 11th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_11_R = crate::FieldReader;
#[doc = "Field `TX_BYTE_11` writer - In operation mode, it stores the 11th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In operation mode, it stores the 11th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    pub fn tx_byte_11(&self) -> TX_BYTE_11_R {
        TX_BYTE_11_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_11")
            .field("tx_byte_11", &format_args!("{}", self.tx_byte_11().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_11_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In operation mode, it stores the 11th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_11(&mut self) -> TX_BYTE_11_W<DATA_11_SPEC> {
        TX_BYTE_11_W::new(self, 0)
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
#[doc = "Data register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_11_SPEC;
impl crate::RegisterSpec for DATA_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_11::R`](R) reader structure"]
impl crate::Readable for DATA_11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_11::W`](W) writer structure"]
impl crate::Writable for DATA_11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_11 to value 0"]
impl crate::Resettable for DATA_11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
