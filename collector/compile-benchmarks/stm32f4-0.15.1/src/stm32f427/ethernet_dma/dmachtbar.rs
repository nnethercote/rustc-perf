#[doc = "Register `DMACHTBAR` reader"]
pub struct R(crate::R<DMACHTBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACHTBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACHTBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACHTBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HTBAP` reader - Host transmit buffer address pointer"]
pub type HTBAP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host transmit buffer address pointer"]
    #[inline(always)]
    pub fn htbap(&self) -> HTBAP_R {
        HTBAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host transmit buffer address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmachtbar](index.html) module"]
pub struct DMACHTBAR_SPEC;
impl crate::RegisterSpec for DMACHTBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmachtbar::R](R) reader structure"]
impl crate::Readable for DMACHTBAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACHTBAR to value 0"]
impl crate::Resettable for DMACHTBAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
