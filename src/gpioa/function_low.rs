#[doc = "Register `function_low` reader"]
pub struct R(crate::R<FUNCTION_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNCTION_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNCTION_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNCTION_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `function_low` writer"]
pub struct W(crate::W<FUNCTION_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNCTION_LOW_SPEC>;
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
impl From<crate::W<FUNCTION_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNCTION_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fields `pin(0-15)` reader - Alternate function low bit."]
pub struct PIN_R(crate::FieldReader<bool, bool>);
impl PIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `pin(0-15)` writer - Alternate function low bit."]
pub struct PIN_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> PIN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | ((value as u32 & 0x01) << self.offset);
        self.w
    }
}
#[doc = "Fields `pin(0-15)` const generic writer - Alternate function low bit."]
pub struct PIN_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> PIN_CGW<'a, O> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << O)) | ((value as u32 & 0x01) << O);
        self.w
    }
}
impl R {
    #[doc = "Alternate function low bit."]
    #[inline(always)]
    pub unsafe fn pin(&self, n: usize) -> PIN_R {
        PIN_R::new(((self.bits >> n) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin0(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin1(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin2(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin3(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin4(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin5(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin6(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin7(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin8(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin9(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin10(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin11(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin12(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin13(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin14(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin15(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Alternate function low bit."]
    #[inline(always)]
    pub unsafe fn pin(&mut self, n: usize) -> PIN_W {
        PIN_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN_CGW<0> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 1 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN_CGW<1> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 2 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN_CGW<2> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 3 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN_CGW<3> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 4 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN_CGW<4> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 5 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN_CGW<5> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 6 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN_CGW<6> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 7 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN_CGW<7> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 8 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin8(&mut self) -> PIN_CGW<8> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 9 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin9(&mut self) -> PIN_CGW<9> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 10 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin10(&mut self) -> PIN_CGW<10> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 11 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin11(&mut self) -> PIN_CGW<11> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 12 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin12(&mut self) -> PIN_CGW<12> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 13 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin13(&mut self) -> PIN_CGW<13> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 14 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin14(&mut self) -> PIN_CGW<14> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 15 - Alternate function low bit."]
    #[inline(always)]
    pub fn pin15(&mut self) -> PIN_CGW<15> {
        PIN_CGW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate function select low register\n\n This register and `function_high` jointly determine multiplexing function number.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [function_low](index.html) module"]
pub struct FUNCTION_LOW_SPEC;
impl crate::RegisterSpec for FUNCTION_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [function_low::R](R) reader structure"]
impl crate::Readable for FUNCTION_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [function_low::W](W) writer structure"]
impl crate::Writable for FUNCTION_LOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets function_low to value 0"]
impl crate::Resettable for FUNCTION_LOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
