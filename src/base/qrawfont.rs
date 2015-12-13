// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qchar::QChar;
use super::qpointf::QPointF;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: double QRawFont::averageCharWidth();
  fn _ZNK8QRawFont16averageCharWidthEv() -> i32;
  // proto: double QRawFont::ascent();
  fn _ZNK8QRawFont6ascentEv() -> i32;
  // proto: double QRawFont::leading();
  fn _ZNK8QRawFont7leadingEv() -> i32;
  // proto: double QRawFont::lineThickness();
  fn _ZNK8QRawFont13lineThicknessEv() -> i32;
  // proto: bool QRawFont::isValid();
  fn _ZNK8QRawFont7isValidEv() -> i32;
  // proto: QRectF QRawFont::boundingRect(quint32 glyphIndex);
  fn _ZNK8QRawFont12boundingRectEj(arg0: c_uint) -> i32;
  // proto: bool QRawFont::supportsCharacter(uint ucs4);
  fn _ZNK8QRawFont17supportsCharacterEj(arg0: c_uint) -> i32;
  // proto: void QRawFont::swap(QRawFont & other);
  fn _ZN8QRawFont4swapERS_(arg0: *mut c_void) -> i32;
  // proto: double QRawFont::descent();
  fn _ZNK8QRawFont7descentEv() -> i32;
  // proto: void QRawFont::NewQRawFont();
  fn _ZN8QRawFontC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QRawFont::setPixelSize(qreal pixelSize);
  fn _ZN8QRawFont12setPixelSizeEd(arg0: c_double) -> i32;
  // proto: bool QRawFont::glyphIndexesForChars(const QChar * chars, int numChars, quint32 * glyphIndexes, int * numGlyphs);
  fn _ZNK8QRawFont20glyphIndexesForCharsEPK5QChariPjPi(arg0: *const c_void, arg1: c_int, arg2: *mut c_uint, arg3: *mut c_int) -> i32;
  // proto: QString QRawFont::styleName();
  fn _ZNK8QRawFont9styleNameEv() -> i32;
  // proto: double QRawFont::underlinePosition();
  fn _ZNK8QRawFont17underlinePositionEv() -> i32;
  // proto: double QRawFont::unitsPerEm();
  fn _ZNK8QRawFont10unitsPerEmEv() -> i32;
  // proto: bool QRawFont::supportsCharacter(QChar character);
  fn _ZNK8QRawFont17supportsCharacterE5QChar(arg0: *mut c_void) -> i32;
  // proto: QString QRawFont::familyName();
  fn _ZNK8QRawFont10familyNameEv() -> i32;
  // proto: bool QRawFont::advancesForGlyphIndexes(const quint32 * glyphIndexes, QPointF * advances, int numGlyphs);
  fn _ZNK8QRawFont23advancesForGlyphIndexesEPKjP7QPointFi(arg0: *const c_uint, arg1: *mut c_void, arg2: c_int) -> i32;
  // proto: double QRawFont::pixelSize();
  fn _ZNK8QRawFont9pixelSizeEv() -> i32;
  // proto: int QRawFont::weight();
  fn _ZNK8QRawFont6weightEv() -> i32;
  // proto: void QRawFont::NewQRawFont(const QRawFont & other);
  fn _ZN8QRawFontC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: double QRawFont::xHeight();
  fn _ZNK8QRawFont7xHeightEv() -> i32;
  // proto: void QRawFont::FreeQRawFont();
  fn _ZN8QRawFontD0Ev() -> i32;
  // proto: QPainterPath QRawFont::pathForGlyph(quint32 glyphIndex);
  fn _ZNK8QRawFont12pathForGlyphEj(arg0: c_uint) -> i32;
  // proto: QByteArray QRawFont::fontTable(const char * tagName);
  fn _ZNK8QRawFont9fontTableEPKc(arg0: *const c_char) -> i32;
  // proto: double QRawFont::maxCharWidth();
  fn _ZNK8QRawFont12maxCharWidthEv() -> i32;
  // proto: QVector<quint32> QRawFont::glyphIndexesForString(const QString & text);
  fn _ZNK8QRawFont21glyphIndexesForStringERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QRawFont)=1
pub struct QRawFont {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRawFont {
  pub fn averageCharWidth<T: QRawFont_averageCharWidth>(&mut self, value: T) -> i32 {
    value.averageCharWidth(self);
    return 1;
  }
}

pub trait QRawFont_averageCharWidth {
  fn averageCharWidth(self, this: &mut QRawFont) -> i32;
}

// proto: double QRawFont::averageCharWidth();
impl<'a> /*trait*/ QRawFont_averageCharWidth for () {
  fn averageCharWidth(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont16averageCharWidthEv()};
    unsafe {_ZNK8QRawFont16averageCharWidthEv()};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn ascent<T: QRawFont_ascent>(&mut self, value: T) -> i32 {
    value.ascent(self);
    return 1;
  }
}

pub trait QRawFont_ascent {
  fn ascent(self, this: &mut QRawFont) -> i32;
}

// proto: double QRawFont::ascent();
impl<'a> /*trait*/ QRawFont_ascent for () {
  fn ascent(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont6ascentEv()};
    unsafe {_ZNK8QRawFont6ascentEv()};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn leading<T: QRawFont_leading>(&mut self, value: T) -> i32 {
    value.leading(self);
    return 1;
  }
}

pub trait QRawFont_leading {
  fn leading(self, this: &mut QRawFont) -> i32;
}

// proto: double QRawFont::leading();
impl<'a> /*trait*/ QRawFont_leading for () {
  fn leading(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont7leadingEv()};
    unsafe {_ZNK8QRawFont7leadingEv()};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn lineThickness<T: QRawFont_lineThickness>(&mut self, value: T) -> i32 {
    value.lineThickness(self);
    return 1;
  }
}

pub trait QRawFont_lineThickness {
  fn lineThickness(self, this: &mut QRawFont) -> i32;
}

// proto: double QRawFont::lineThickness();
impl<'a> /*trait*/ QRawFont_lineThickness for () {
  fn lineThickness(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont13lineThicknessEv()};
    unsafe {_ZNK8QRawFont13lineThicknessEv()};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn isValid<T: QRawFont_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QRawFont_isValid {
  fn isValid(self, this: &mut QRawFont) -> i32;
}

// proto: bool QRawFont::isValid();
impl<'a> /*trait*/ QRawFont_isValid for () {
  fn isValid(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont7isValidEv()};
    unsafe {_ZNK8QRawFont7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn boundingRect<T: QRawFont_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QRawFont_boundingRect {
  fn boundingRect(self, this: &mut QRawFont) -> i32;
}

// proto: QRectF QRawFont::boundingRect(quint32 glyphIndex);
impl<'a> /*trait*/ QRawFont_boundingRect for (u32) {
  fn boundingRect(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont12boundingRectEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZNK8QRawFont12boundingRectEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn supportsCharacter<T: QRawFont_supportsCharacter>(&mut self, value: T) -> i32 {
    value.supportsCharacter(self);
    return 1;
  }
}

pub trait QRawFont_supportsCharacter {
  fn supportsCharacter(self, this: &mut QRawFont) -> i32;
}

// proto: bool QRawFont::supportsCharacter(uint ucs4);
impl<'a> /*trait*/ QRawFont_supportsCharacter for (u32) {
  fn supportsCharacter(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont17supportsCharacterEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZNK8QRawFont17supportsCharacterEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn swap<T: QRawFont_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QRawFont_swap {
  fn swap(self, this: &mut QRawFont) -> i32;
}

// proto: void QRawFont::swap(QRawFont & other);
impl<'a> /*trait*/ QRawFont_swap for (&'a mut QRawFont) {
  fn swap(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFont4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QRawFont4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn descent<T: QRawFont_descent>(&mut self, value: T) -> i32 {
    value.descent(self);
    return 1;
  }
}

pub trait QRawFont_descent {
  fn descent(self, this: &mut QRawFont) -> i32;
}

// proto: double QRawFont::descent();
impl<'a> /*trait*/ QRawFont_descent for () {
  fn descent(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont7descentEv()};
    unsafe {_ZNK8QRawFont7descentEv()};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn NewQRawFont<T: QRawFont_NewQRawFont>(value: T) -> QRawFont {
    let rsthis = value.NewQRawFont();
    return rsthis;
    // return 1;
  }
}

pub trait QRawFont_NewQRawFont {
  fn NewQRawFont(self) -> QRawFont;
}

// proto: void QRawFont::NewQRawFont();
impl<'a> /*trait*/ QRawFont_NewQRawFont for () {
  fn NewQRawFont(self) -> QRawFont {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFontC1Ev()};
    unsafe {_ZN8QRawFontC1Ev(qthis)};
    let rsthis = QRawFont{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn setPixelSize<T: QRawFont_setPixelSize>(&mut self, value: T) -> i32 {
    value.setPixelSize(self);
    return 1;
  }
}

pub trait QRawFont_setPixelSize {
  fn setPixelSize(self, this: &mut QRawFont) -> i32;
}

// proto: void QRawFont::setPixelSize(qreal pixelSize);
impl<'a> /*trait*/ QRawFont_setPixelSize for (f64) {
  fn setPixelSize(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFont12setPixelSizeEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN8QRawFont12setPixelSizeEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn glyphIndexesForChars<T: QRawFont_glyphIndexesForChars>(&mut self, value: T) -> i32 {
    value.glyphIndexesForChars(self);
    return 1;
  }
}

pub trait QRawFont_glyphIndexesForChars {
  fn glyphIndexesForChars(self, this: &mut QRawFont) -> i32;
}

// proto: bool QRawFont::glyphIndexesForChars(const QChar * chars, int numChars, quint32 * glyphIndexes, int * numGlyphs);
impl<'a> /*trait*/ QRawFont_glyphIndexesForChars for (&'a  QChar, i32, &'a mut u32, &'a mut i32) {
  fn glyphIndexesForChars(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont20glyphIndexesForCharsEPK5QChariPjPi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_uint;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZNK8QRawFont20glyphIndexesForCharsEPK5QChariPjPi(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn styleName<T: QRawFont_styleName>(&mut self, value: T) -> i32 {
    value.styleName(self);
    return 1;
  }
}

pub trait QRawFont_styleName {
  fn styleName(self, this: &mut QRawFont) -> i32;
}

// proto: QString QRawFont::styleName();
impl<'a> /*trait*/ QRawFont_styleName for () {
  fn styleName(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont9styleNameEv()};
    unsafe {_ZNK8QRawFont9styleNameEv()};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn underlinePosition<T: QRawFont_underlinePosition>(&mut self, value: T) -> i32 {
    value.underlinePosition(self);
    return 1;
  }
}

pub trait QRawFont_underlinePosition {
  fn underlinePosition(self, this: &mut QRawFont) -> i32;
}

// proto: double QRawFont::underlinePosition();
impl<'a> /*trait*/ QRawFont_underlinePosition for () {
  fn underlinePosition(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont17underlinePositionEv()};
    unsafe {_ZNK8QRawFont17underlinePositionEv()};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn unitsPerEm<T: QRawFont_unitsPerEm>(&mut self, value: T) -> i32 {
    value.unitsPerEm(self);
    return 1;
  }
}

pub trait QRawFont_unitsPerEm {
  fn unitsPerEm(self, this: &mut QRawFont) -> i32;
}

// proto: double QRawFont::unitsPerEm();
impl<'a> /*trait*/ QRawFont_unitsPerEm for () {
  fn unitsPerEm(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont10unitsPerEmEv()};
    unsafe {_ZNK8QRawFont10unitsPerEmEv()};
    return 1;
  }
}

// proto: bool QRawFont::supportsCharacter(QChar character);
impl<'a> /*trait*/ QRawFont_supportsCharacter for (QChar) {
  fn supportsCharacter(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont17supportsCharacterE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK8QRawFont17supportsCharacterE5QChar(arg0)};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn familyName<T: QRawFont_familyName>(&mut self, value: T) -> i32 {
    value.familyName(self);
    return 1;
  }
}

pub trait QRawFont_familyName {
  fn familyName(self, this: &mut QRawFont) -> i32;
}

// proto: QString QRawFont::familyName();
impl<'a> /*trait*/ QRawFont_familyName for () {
  fn familyName(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont10familyNameEv()};
    unsafe {_ZNK8QRawFont10familyNameEv()};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn advancesForGlyphIndexes<T: QRawFont_advancesForGlyphIndexes>(&mut self, value: T) -> i32 {
    value.advancesForGlyphIndexes(self);
    return 1;
  }
}

pub trait QRawFont_advancesForGlyphIndexes {
  fn advancesForGlyphIndexes(self, this: &mut QRawFont) -> i32;
}

// proto: bool QRawFont::advancesForGlyphIndexes(const quint32 * glyphIndexes, QPointF * advances, int numGlyphs);
impl<'a> /*trait*/ QRawFont_advancesForGlyphIndexes for (&'a  u32, &'a mut QPointF, i32) {
  fn advancesForGlyphIndexes(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont23advancesForGlyphIndexesEPKjP7QPointFi()};
    let arg0 = self.0  as *const c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK8QRawFont23advancesForGlyphIndexesEPKjP7QPointFi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn pixelSize<T: QRawFont_pixelSize>(&mut self, value: T) -> i32 {
    value.pixelSize(self);
    return 1;
  }
}

pub trait QRawFont_pixelSize {
  fn pixelSize(self, this: &mut QRawFont) -> i32;
}

// proto: double QRawFont::pixelSize();
impl<'a> /*trait*/ QRawFont_pixelSize for () {
  fn pixelSize(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont9pixelSizeEv()};
    unsafe {_ZNK8QRawFont9pixelSizeEv()};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn weight<T: QRawFont_weight>(&mut self, value: T) -> i32 {
    value.weight(self);
    return 1;
  }
}

pub trait QRawFont_weight {
  fn weight(self, this: &mut QRawFont) -> i32;
}

// proto: int QRawFont::weight();
impl<'a> /*trait*/ QRawFont_weight for () {
  fn weight(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont6weightEv()};
    unsafe {_ZNK8QRawFont6weightEv()};
    return 1;
  }
}

// proto: void QRawFont::NewQRawFont(const QRawFont & other);
impl<'a> /*trait*/ QRawFont_NewQRawFont for (&'a  QRawFont) {
  fn NewQRawFont(self) -> QRawFont {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFontC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QRawFontC1ERKS_(qthis, arg0)};
    let rsthis = QRawFont{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn xHeight<T: QRawFont_xHeight>(&mut self, value: T) -> i32 {
    value.xHeight(self);
    return 1;
  }
}

pub trait QRawFont_xHeight {
  fn xHeight(self, this: &mut QRawFont) -> i32;
}

// proto: double QRawFont::xHeight();
impl<'a> /*trait*/ QRawFont_xHeight for () {
  fn xHeight(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont7xHeightEv()};
    unsafe {_ZNK8QRawFont7xHeightEv()};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn FreeQRawFont<T: QRawFont_FreeQRawFont>(&mut self, value: T) -> i32 {
    value.FreeQRawFont(self);
    return 1;
  }
}

pub trait QRawFont_FreeQRawFont {
  fn FreeQRawFont(self, this: &mut QRawFont) -> i32;
}

// proto: void QRawFont::FreeQRawFont();
impl<'a> /*trait*/ QRawFont_FreeQRawFont for () {
  fn FreeQRawFont(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFontD0Ev()};
    unsafe {_ZN8QRawFontD0Ev()};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn pathForGlyph<T: QRawFont_pathForGlyph>(&mut self, value: T) -> i32 {
    value.pathForGlyph(self);
    return 1;
  }
}

pub trait QRawFont_pathForGlyph {
  fn pathForGlyph(self, this: &mut QRawFont) -> i32;
}

// proto: QPainterPath QRawFont::pathForGlyph(quint32 glyphIndex);
impl<'a> /*trait*/ QRawFont_pathForGlyph for (u32) {
  fn pathForGlyph(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont12pathForGlyphEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZNK8QRawFont12pathForGlyphEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn fontTable<T: QRawFont_fontTable>(&mut self, value: T) -> i32 {
    value.fontTable(self);
    return 1;
  }
}

pub trait QRawFont_fontTable {
  fn fontTable(self, this: &mut QRawFont) -> i32;
}

// proto: QByteArray QRawFont::fontTable(const char * tagName);
impl<'a> /*trait*/ QRawFont_fontTable for (&'a  String) {
  fn fontTable(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont9fontTableEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZNK8QRawFont9fontTableEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn maxCharWidth<T: QRawFont_maxCharWidth>(&mut self, value: T) -> i32 {
    value.maxCharWidth(self);
    return 1;
  }
}

pub trait QRawFont_maxCharWidth {
  fn maxCharWidth(self, this: &mut QRawFont) -> i32;
}

// proto: double QRawFont::maxCharWidth();
impl<'a> /*trait*/ QRawFont_maxCharWidth for () {
  fn maxCharWidth(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont12maxCharWidthEv()};
    unsafe {_ZNK8QRawFont12maxCharWidthEv()};
    return 1;
  }
}

impl /*struct*/ QRawFont {
  pub fn glyphIndexesForString<T: QRawFont_glyphIndexesForString>(&mut self, value: T) -> i32 {
    value.glyphIndexesForString(self);
    return 1;
  }
}

pub trait QRawFont_glyphIndexesForString {
  fn glyphIndexesForString(self, this: &mut QRawFont) -> i32;
}

// proto: QVector<quint32> QRawFont::glyphIndexesForString(const QString & text);
impl<'a> /*trait*/ QRawFont_glyphIndexesForString for (&'a  QString) {
  fn glyphIndexesForString(self, this: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont21glyphIndexesForStringERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK8QRawFont21glyphIndexesForStringERK7QString(arg0)};
    return 1;
  }
}

