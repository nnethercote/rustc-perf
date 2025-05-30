#[doc = "Register `MACA1LR` reader"]
pub struct R(crate::R<MACA1LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA1LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA1LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA1LR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACA1LR` writer"]
pub struct W(crate::W<MACA1LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA1LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MACA1LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA1LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACA1L` reader - MACA1LR"]
pub type MACA1L_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MACA1L` writer - MACA1LR"]
pub type MACA1L_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACA1LR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - MACA1LR"]
    #[inline(always)]
    pub fn maca1l(&self) -> MACA1L_R {
        MACA1L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MACA1LR"]
    #[inline(always)]
    pub fn maca1l(&mut self) -> MACA1L_W<0> {
        MACA1L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Ethernet MAC address1 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca1lr](index.html) module"]
pub struct MACA1LR_SPEC;
impl crate::RegisterSpec for MACA1LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maca1lr::R](R) reader structure"]
impl crate::Readable for MACA1LR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maca1lr::W](W) writer structure"]
impl crate::Writable for MACA1LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACA1LR to value 0xffff_ffff"]
impl crate::Resettable for MACA1LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
