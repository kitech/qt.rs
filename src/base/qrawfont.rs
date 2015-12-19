// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrectf::QRectF;
use super::qchar::QChar;
use super::qstring::QString;
use super::qpointf::QPointF;
use super::qpainterpath::QPainterPath;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  double QRawFont::averageCharWidth();
  fn _ZNK8QRawFont16averageCharWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  double QRawFont::ascent();
  fn _ZNK8QRawFont6ascentEv(qthis: *mut c_void) -> c_double;
  // proto:  double QRawFont::leading();
  fn _ZNK8QRawFont7leadingEv(qthis: *mut c_void) -> c_double;
  // proto:  double QRawFont::lineThickness();
  fn _ZNK8QRawFont13lineThicknessEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QRawFont::isValid();
  fn _ZNK8QRawFont7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QRectF QRawFont::boundingRect(quint32 glyphIndex);
  fn _ZNK8QRawFont12boundingRectEj(qthis: *mut c_void, arg0: c_uint) -> *mut c_void;
  // proto:  bool QRawFont::supportsCharacter(uint ucs4);
  fn _ZNK8QRawFont17supportsCharacterEj(qthis: *mut c_void, arg0: c_uint) -> int8_t;
  // proto:  void QRawFont::swap(QRawFont & other);
  fn _ZN8QRawFont4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QRawFont::descent();
  fn _ZNK8QRawFont7descentEv(qthis: *mut c_void) -> c_double;
  // proto:  void QRawFont::NewQRawFont();
  fn _ZN8QRawFontC1Ev(qthis: *mut c_void) ;
  // proto:  void QRawFont::setPixelSize(qreal pixelSize);
  fn _ZN8QRawFont12setPixelSizeEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  bool QRawFont::glyphIndexesForChars(const QChar * chars, int numChars, quint32 * glyphIndexes, int * numGlyphs);
  fn _ZNK8QRawFont20glyphIndexesForCharsEPK5QChariPjPi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_uint, arg3: *mut c_int) -> int8_t;
  // proto:  QString QRawFont::styleName();
  fn _ZNK8QRawFont9styleNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QRawFont::underlinePosition();
  fn _ZNK8QRawFont17underlinePositionEv(qthis: *mut c_void) -> c_double;
  // proto:  double QRawFont::unitsPerEm();
  fn _ZNK8QRawFont10unitsPerEmEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QRawFont::supportsCharacter(QChar character);
  fn _ZNK8QRawFont17supportsCharacterE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QString QRawFont::familyName();
  fn _ZNK8QRawFont10familyNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QRawFont::advancesForGlyphIndexes(const quint32 * glyphIndexes, QPointF * advances, int numGlyphs);
  fn _ZNK8QRawFont23advancesForGlyphIndexesEPKjP7QPointFi(qthis: *mut c_void, arg0: *const c_uint, arg1: *mut c_void, arg2: c_int) -> int8_t;
  // proto:  double QRawFont::pixelSize();
  fn _ZNK8QRawFont9pixelSizeEv(qthis: *mut c_void) -> c_double;
  // proto:  int QRawFont::weight();
  fn _ZNK8QRawFont6weightEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRawFont::NewQRawFont(const QRawFont & other);
  fn _ZN8QRawFontC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QRawFont::xHeight();
  fn _ZNK8QRawFont7xHeightEv(qthis: *mut c_void) -> c_double;
  // proto:  void QRawFont::FreeQRawFont();
  fn _ZN8QRawFontD0Ev(qthis: *mut c_void) ;
  // proto:  QPainterPath QRawFont::pathForGlyph(quint32 glyphIndex);
  fn _ZNK8QRawFont12pathForGlyphEj(qthis: *mut c_void, arg0: c_uint) -> *mut c_void;
  // proto:  QByteArray QRawFont::fontTable(const char * tagName);
  fn _ZNK8QRawFont9fontTableEPKc(qthis: *mut c_void, arg0: *const c_char) -> *mut c_void;
  // proto:  double QRawFont::maxCharWidth();
  fn _ZNK8QRawFont12maxCharWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  QVector<quint32> QRawFont::glyphIndexesForString(const QString & text);
  fn _ZNK8QRawFont21glyphIndexesForStringERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QRawFont)=1
pub struct QRawFont {
  pub qclsinst: *mut c_void,
}

// proto:  double QRawFont::averageCharWidth();
impl /*struct*/ QRawFont {
  pub fn averageCharWidth<RetType, T: QRawFont_averageCharWidth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.averageCharWidth(self);
    // return 1;
  }
}

pub trait QRawFont_averageCharWidth<RetType> {
  fn averageCharWidth(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  double QRawFont::averageCharWidth();
impl<'a> /*trait*/ QRawFont_averageCharWidth<f64> for () {
  fn averageCharWidth(self , rsthis: &mut QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont16averageCharWidthEv()};
    let mut ret = unsafe {_ZNK8QRawFont16averageCharWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  double QRawFont::ascent();
impl /*struct*/ QRawFont {
  pub fn ascent<RetType, T: QRawFont_ascent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.ascent(self);
    // return 1;
  }
}

pub trait QRawFont_ascent<RetType> {
  fn ascent(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  double QRawFont::ascent();
impl<'a> /*trait*/ QRawFont_ascent<f64> for () {
  fn ascent(self , rsthis: &mut QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont6ascentEv()};
    let mut ret = unsafe {_ZNK8QRawFont6ascentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  double QRawFont::leading();
impl /*struct*/ QRawFont {
  pub fn leading<RetType, T: QRawFont_leading<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.leading(self);
    // return 1;
  }
}

pub trait QRawFont_leading<RetType> {
  fn leading(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  double QRawFont::leading();
impl<'a> /*trait*/ QRawFont_leading<f64> for () {
  fn leading(self , rsthis: &mut QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont7leadingEv()};
    let mut ret = unsafe {_ZNK8QRawFont7leadingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  double QRawFont::lineThickness();
impl /*struct*/ QRawFont {
  pub fn lineThickness<RetType, T: QRawFont_lineThickness<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lineThickness(self);
    // return 1;
  }
}

pub trait QRawFont_lineThickness<RetType> {
  fn lineThickness(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  double QRawFont::lineThickness();
impl<'a> /*trait*/ QRawFont_lineThickness<f64> for () {
  fn lineThickness(self , rsthis: &mut QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont13lineThicknessEv()};
    let mut ret = unsafe {_ZNK8QRawFont13lineThicknessEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  bool QRawFont::isValid();
impl /*struct*/ QRawFont {
  pub fn isValid<RetType, T: QRawFont_isValid<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QRawFont_isValid<RetType> {
  fn isValid(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  bool QRawFont::isValid();
impl<'a> /*trait*/ QRawFont_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QRawFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont7isValidEv()};
    let mut ret = unsafe {_ZNK8QRawFont7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QRectF QRawFont::boundingRect(quint32 glyphIndex);
impl /*struct*/ QRawFont {
  pub fn boundingRect<RetType, T: QRawFont_boundingRect<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QRawFont_boundingRect<RetType> {
  fn boundingRect(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  QRectF QRawFont::boundingRect(quint32 glyphIndex);
impl<'a> /*trait*/ QRawFont_boundingRect<QRectF> for (u32) {
  fn boundingRect(self , rsthis: &mut QRawFont) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont12boundingRectEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZNK8QRawFont12boundingRectEj(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QRawFont::supportsCharacter(uint ucs4);
impl /*struct*/ QRawFont {
  pub fn supportsCharacter<RetType, T: QRawFont_supportsCharacter<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.supportsCharacter(self);
    // return 1;
  }
}

pub trait QRawFont_supportsCharacter<RetType> {
  fn supportsCharacter(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  bool QRawFont::supportsCharacter(uint ucs4);
impl<'a> /*trait*/ QRawFont_supportsCharacter<i8> for (u32) {
  fn supportsCharacter(self , rsthis: &mut QRawFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont17supportsCharacterEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZNK8QRawFont17supportsCharacterEj(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QRawFont::swap(QRawFont & other);
impl /*struct*/ QRawFont {
  pub fn swap<RetType, T: QRawFont_swap<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QRawFont_swap<RetType> {
  fn swap(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  void QRawFont::swap(QRawFont & other);
impl<'a> /*trait*/ QRawFont_swap<()> for (&'a mut QRawFont) {
  fn swap(self , rsthis: &mut QRawFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFont4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QRawFont4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  double QRawFont::descent();
impl /*struct*/ QRawFont {
  pub fn descent<RetType, T: QRawFont_descent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.descent(self);
    // return 1;
  }
}

pub trait QRawFont_descent<RetType> {
  fn descent(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  double QRawFont::descent();
impl<'a> /*trait*/ QRawFont_descent<f64> for () {
  fn descent(self , rsthis: &mut QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont7descentEv()};
    let mut ret = unsafe {_ZNK8QRawFont7descentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
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

// proto:  void QRawFont::setPixelSize(qreal pixelSize);
impl /*struct*/ QRawFont {
  pub fn setPixelSize<RetType, T: QRawFont_setPixelSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setPixelSize(self);
    // return 1;
  }
}

pub trait QRawFont_setPixelSize<RetType> {
  fn setPixelSize(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  void QRawFont::setPixelSize(qreal pixelSize);
impl<'a> /*trait*/ QRawFont_setPixelSize<()> for (f64) {
  fn setPixelSize(self , rsthis: &mut QRawFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFont12setPixelSizeEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN8QRawFont12setPixelSizeEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QRawFont::glyphIndexesForChars(const QChar * chars, int numChars, quint32 * glyphIndexes, int * numGlyphs);
impl /*struct*/ QRawFont {
  pub fn glyphIndexesForChars<RetType, T: QRawFont_glyphIndexesForChars<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.glyphIndexesForChars(self);
    // return 1;
  }
}

pub trait QRawFont_glyphIndexesForChars<RetType> {
  fn glyphIndexesForChars(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  bool QRawFont::glyphIndexesForChars(const QChar * chars, int numChars, quint32 * glyphIndexes, int * numGlyphs);
impl<'a> /*trait*/ QRawFont_glyphIndexesForChars<i8> for (&'a  QChar, i32, &'a mut u32, &'a mut i32) {
  fn glyphIndexesForChars(self , rsthis: &mut QRawFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont20glyphIndexesForCharsEPK5QChariPjPi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_uint;
    let arg3 = self.3  as *mut c_int;
    let mut ret = unsafe {_ZNK8QRawFont20glyphIndexesForCharsEPK5QChariPjPi(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QString QRawFont::styleName();
impl /*struct*/ QRawFont {
  pub fn styleName<RetType, T: QRawFont_styleName<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.styleName(self);
    // return 1;
  }
}

pub trait QRawFont_styleName<RetType> {
  fn styleName(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  QString QRawFont::styleName();
impl<'a> /*trait*/ QRawFont_styleName<QString> for () {
  fn styleName(self , rsthis: &mut QRawFont) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont9styleNameEv()};
    let mut ret = unsafe {_ZNK8QRawFont9styleNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  double QRawFont::underlinePosition();
impl /*struct*/ QRawFont {
  pub fn underlinePosition<RetType, T: QRawFont_underlinePosition<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.underlinePosition(self);
    // return 1;
  }
}

pub trait QRawFont_underlinePosition<RetType> {
  fn underlinePosition(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  double QRawFont::underlinePosition();
impl<'a> /*trait*/ QRawFont_underlinePosition<f64> for () {
  fn underlinePosition(self , rsthis: &mut QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont17underlinePositionEv()};
    let mut ret = unsafe {_ZNK8QRawFont17underlinePositionEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  double QRawFont::unitsPerEm();
impl /*struct*/ QRawFont {
  pub fn unitsPerEm<RetType, T: QRawFont_unitsPerEm<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.unitsPerEm(self);
    // return 1;
  }
}

pub trait QRawFont_unitsPerEm<RetType> {
  fn unitsPerEm(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  double QRawFont::unitsPerEm();
impl<'a> /*trait*/ QRawFont_unitsPerEm<f64> for () {
  fn unitsPerEm(self , rsthis: &mut QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont10unitsPerEmEv()};
    let mut ret = unsafe {_ZNK8QRawFont10unitsPerEmEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  bool QRawFont::supportsCharacter(QChar character);
impl<'a> /*trait*/ QRawFont_supportsCharacter<i8> for (QChar) {
  fn supportsCharacter(self , rsthis: &mut QRawFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont17supportsCharacterE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QRawFont17supportsCharacterE5QChar(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QString QRawFont::familyName();
impl /*struct*/ QRawFont {
  pub fn familyName<RetType, T: QRawFont_familyName<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.familyName(self);
    // return 1;
  }
}

pub trait QRawFont_familyName<RetType> {
  fn familyName(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  QString QRawFont::familyName();
impl<'a> /*trait*/ QRawFont_familyName<QString> for () {
  fn familyName(self , rsthis: &mut QRawFont) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont10familyNameEv()};
    let mut ret = unsafe {_ZNK8QRawFont10familyNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QRawFont::advancesForGlyphIndexes(const quint32 * glyphIndexes, QPointF * advances, int numGlyphs);
impl /*struct*/ QRawFont {
  pub fn advancesForGlyphIndexes<RetType, T: QRawFont_advancesForGlyphIndexes<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.advancesForGlyphIndexes(self);
    // return 1;
  }
}

pub trait QRawFont_advancesForGlyphIndexes<RetType> {
  fn advancesForGlyphIndexes(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  bool QRawFont::advancesForGlyphIndexes(const quint32 * glyphIndexes, QPointF * advances, int numGlyphs);
impl<'a> /*trait*/ QRawFont_advancesForGlyphIndexes<i8> for (&'a  u32, &'a mut QPointF, i32) {
  fn advancesForGlyphIndexes(self , rsthis: &mut QRawFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont23advancesForGlyphIndexesEPKjP7QPointFi()};
    let arg0 = self.0  as *const c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK8QRawFont23advancesForGlyphIndexesEPKjP7QPointFi(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

// proto:  double QRawFont::pixelSize();
impl /*struct*/ QRawFont {
  pub fn pixelSize<RetType, T: QRawFont_pixelSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.pixelSize(self);
    // return 1;
  }
}

pub trait QRawFont_pixelSize<RetType> {
  fn pixelSize(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  double QRawFont::pixelSize();
impl<'a> /*trait*/ QRawFont_pixelSize<f64> for () {
  fn pixelSize(self , rsthis: &mut QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont9pixelSizeEv()};
    let mut ret = unsafe {_ZNK8QRawFont9pixelSizeEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  int QRawFont::weight();
impl /*struct*/ QRawFont {
  pub fn weight<RetType, T: QRawFont_weight<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.weight(self);
    // return 1;
  }
}

pub trait QRawFont_weight<RetType> {
  fn weight(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  int QRawFont::weight();
impl<'a> /*trait*/ QRawFont_weight<i32> for () {
  fn weight(self , rsthis: &mut QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont6weightEv()};
    let mut ret = unsafe {_ZNK8QRawFont6weightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QRawFont::NewQRawFont(const QRawFont & other);
impl<'a> /*trait*/ QRawFont_NewQRawFont for (&'a  QRawFont) {
  fn NewQRawFont(self) -> QRawFont {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFontC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QRawFontC1ERKS_(qthis, arg0)};
    let rsthis = QRawFont{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  double QRawFont::xHeight();
impl /*struct*/ QRawFont {
  pub fn xHeight<RetType, T: QRawFont_xHeight<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.xHeight(self);
    // return 1;
  }
}

pub trait QRawFont_xHeight<RetType> {
  fn xHeight(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  double QRawFont::xHeight();
impl<'a> /*trait*/ QRawFont_xHeight<f64> for () {
  fn xHeight(self , rsthis: &mut QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont7xHeightEv()};
    let mut ret = unsafe {_ZNK8QRawFont7xHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QRawFont::FreeQRawFont();
impl /*struct*/ QRawFont {
  pub fn FreeQRawFont<RetType, T: QRawFont_FreeQRawFont<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQRawFont(self);
    // return 1;
  }
}

pub trait QRawFont_FreeQRawFont<RetType> {
  fn FreeQRawFont(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  void QRawFont::FreeQRawFont();
impl<'a> /*trait*/ QRawFont_FreeQRawFont<()> for () {
  fn FreeQRawFont(self , rsthis: &mut QRawFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFontD0Ev()};
     unsafe {_ZN8QRawFontD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QPainterPath QRawFont::pathForGlyph(quint32 glyphIndex);
impl /*struct*/ QRawFont {
  pub fn pathForGlyph<RetType, T: QRawFont_pathForGlyph<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.pathForGlyph(self);
    // return 1;
  }
}

pub trait QRawFont_pathForGlyph<RetType> {
  fn pathForGlyph(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  QPainterPath QRawFont::pathForGlyph(quint32 glyphIndex);
impl<'a> /*trait*/ QRawFont_pathForGlyph<QPainterPath> for (u32) {
  fn pathForGlyph(self , rsthis: &mut QRawFont) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont12pathForGlyphEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZNK8QRawFont12pathForGlyphEj(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QByteArray QRawFont::fontTable(const char * tagName);
impl /*struct*/ QRawFont {
  pub fn fontTable<RetType, T: QRawFont_fontTable<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fontTable(self);
    // return 1;
  }
}

pub trait QRawFont_fontTable<RetType> {
  fn fontTable(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  QByteArray QRawFont::fontTable(const char * tagName);
impl<'a> /*trait*/ QRawFont_fontTable<QByteArray> for (&'a  String) {
  fn fontTable(self , rsthis: &mut QRawFont) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont9fontTableEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZNK8QRawFont9fontTableEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  double QRawFont::maxCharWidth();
impl /*struct*/ QRawFont {
  pub fn maxCharWidth<RetType, T: QRawFont_maxCharWidth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.maxCharWidth(self);
    // return 1;
  }
}

pub trait QRawFont_maxCharWidth<RetType> {
  fn maxCharWidth(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  double QRawFont::maxCharWidth();
impl<'a> /*trait*/ QRawFont_maxCharWidth<f64> for () {
  fn maxCharWidth(self , rsthis: &mut QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont12maxCharWidthEv()};
    let mut ret = unsafe {_ZNK8QRawFont12maxCharWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  QVector<quint32> QRawFont::glyphIndexesForString(const QString & text);
impl /*struct*/ QRawFont {
  pub fn glyphIndexesForString<RetType, T: QRawFont_glyphIndexesForString<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.glyphIndexesForString(self);
    // return 1;
  }
}

pub trait QRawFont_glyphIndexesForString<RetType> {
  fn glyphIndexesForString(self , rsthis: &mut QRawFont) -> RetType;
}

// proto:  QVector<quint32> QRawFont::glyphIndexesForString(const QString & text);
impl<'a> /*trait*/ QRawFont_glyphIndexesForString<()> for (&'a  QString) {
  fn glyphIndexesForString(self , rsthis: &mut QRawFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont21glyphIndexesForStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK8QRawFont21glyphIndexesForStringERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

