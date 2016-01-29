// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtGui/qrawfont.h
// dst-file: /src/gui/qrawfont.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
use super::super::core::qstring::*; // 771
use super::super::core::qrect::*; // 771
use super::super::core::qbytearray::*; // 771
use super::super::core::qchar::*; // 771
use super::qtransform::*; // 773
use super::super::core::qpoint::*; // 771
use super::qpainterpath::*; // 773
use super::qfont::*; // 773
// use super::qvector::*; // 775
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QRawFont_Class_Size() -> c_int;
  // proto:  qreal QRawFont::averageCharWidth();
  fn C_ZNK8QRawFont16averageCharWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QRawFont::ascent();
  fn C_ZNK8QRawFont6ascentEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QRawFont::leading();
  fn C_ZNK8QRawFont7leadingEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QRawFont::lineThickness();
  fn C_ZNK8QRawFont13lineThicknessEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QRawFont::isValid();
  fn C_ZNK8QRawFont7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRectF QRawFont::boundingRect(quint32 glyphIndex);
  fn C_ZNK8QRawFont12boundingRectEj(qthis: u64 /* *mut c_void*/, arg0: c_uint) -> *mut c_void;
  // proto:  bool QRawFont::supportsCharacter(uint ucs4);
  fn C_ZNK8QRawFont17supportsCharacterEj(qthis: u64 /* *mut c_void*/, arg0: c_uint) -> c_char;
  // proto:  void QRawFont::swap(QRawFont & other);
  fn C_ZN8QRawFont4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QRawFont::descent();
  fn C_ZNK8QRawFont7descentEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QRawFont::QRawFont();
  fn C_ZN8QRawFontC2Ev() -> u64;
  // proto:  void QRawFont::setPixelSize(qreal pixelSize);
  fn C_ZN8QRawFont12setPixelSizeEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  bool QRawFont::glyphIndexesForChars(const QChar * chars, int numChars, quint32 * glyphIndexes, int * numGlyphs);
  fn C_ZNK8QRawFont20glyphIndexesForCharsEPK5QChariPjPi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: *mut c_uint, arg3: *mut c_int) -> c_char;
  // proto:  QString QRawFont::styleName();
  fn C_ZNK8QRawFont9styleNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QRawFont::underlinePosition();
  fn C_ZNK8QRawFont17underlinePositionEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QRawFont::unitsPerEm();
  fn C_ZNK8QRawFont10unitsPerEmEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QRawFont::supportsCharacter(QChar character);
  fn C_ZNK8QRawFont17supportsCharacterE5QChar(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QString QRawFont::familyName();
  fn C_ZNK8QRawFont10familyNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QRawFont::advancesForGlyphIndexes(const quint32 * glyphIndexes, QPointF * advances, int numGlyphs);
  fn C_ZNK8QRawFont23advancesForGlyphIndexesEPKjP7QPointFi(qthis: u64 /* *mut c_void*/, arg0: *mut c_uint, arg1: *mut c_void, arg2: c_int) -> c_char;
  // proto:  qreal QRawFont::pixelSize();
  fn C_ZNK8QRawFont9pixelSizeEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  int QRawFont::weight();
  fn C_ZNK8QRawFont6weightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QRawFont::QRawFont(const QRawFont & other);
  fn C_ZN8QRawFontC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  qreal QRawFont::xHeight();
  fn C_ZNK8QRawFont7xHeightEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QRawFont::~QRawFont();
  fn C_ZN8QRawFontD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QPainterPath QRawFont::pathForGlyph(quint32 glyphIndex);
  fn C_ZNK8QRawFont12pathForGlyphEj(qthis: u64 /* *mut c_void*/, arg0: c_uint) -> *mut c_void;
  // proto:  QByteArray QRawFont::fontTable(const char * tagName);
  fn C_ZNK8QRawFont9fontTableEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> *mut c_void;
  // proto:  qreal QRawFont::maxCharWidth();
  fn C_ZNK8QRawFont12maxCharWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QVector<quint32> QRawFont::glyphIndexesForString(const QString & text);
  fn C_ZNK8QRawFont21glyphIndexesForStringERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QRawFont)=1
#[derive(Default)]
pub struct QRawFont {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QRawFont {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QRawFont {
    return QRawFont{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  qreal QRawFont::averageCharWidth();
impl /*struct*/ QRawFont {
  pub fn averageCharWidth<RetType, T: QRawFont_averageCharWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.averageCharWidth(self);
    // return 1;
  }
}

pub trait QRawFont_averageCharWidth<RetType> {
  fn averageCharWidth(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  qreal QRawFont::averageCharWidth();
impl<'a> /*trait*/ QRawFont_averageCharWidth<f64> for () {
  fn averageCharWidth(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont16averageCharWidthEv()};
    let mut ret = unsafe {C_ZNK8QRawFont16averageCharWidthEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QRawFont::ascent();
impl /*struct*/ QRawFont {
  pub fn ascent<RetType, T: QRawFont_ascent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ascent(self);
    // return 1;
  }
}

pub trait QRawFont_ascent<RetType> {
  fn ascent(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  qreal QRawFont::ascent();
impl<'a> /*trait*/ QRawFont_ascent<f64> for () {
  fn ascent(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont6ascentEv()};
    let mut ret = unsafe {C_ZNK8QRawFont6ascentEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QRawFont::leading();
impl /*struct*/ QRawFont {
  pub fn leading<RetType, T: QRawFont_leading<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leading(self);
    // return 1;
  }
}

pub trait QRawFont_leading<RetType> {
  fn leading(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  qreal QRawFont::leading();
impl<'a> /*trait*/ QRawFont_leading<f64> for () {
  fn leading(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont7leadingEv()};
    let mut ret = unsafe {C_ZNK8QRawFont7leadingEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QRawFont::lineThickness();
impl /*struct*/ QRawFont {
  pub fn lineThickness<RetType, T: QRawFont_lineThickness<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineThickness(self);
    // return 1;
  }
}

pub trait QRawFont_lineThickness<RetType> {
  fn lineThickness(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  qreal QRawFont::lineThickness();
impl<'a> /*trait*/ QRawFont_lineThickness<f64> for () {
  fn lineThickness(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont13lineThicknessEv()};
    let mut ret = unsafe {C_ZNK8QRawFont13lineThicknessEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  bool QRawFont::isValid();
impl /*struct*/ QRawFont {
  pub fn isValid<RetType, T: QRawFont_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QRawFont_isValid<RetType> {
  fn isValid(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  bool QRawFont::isValid();
impl<'a> /*trait*/ QRawFont_isValid<i8> for () {
  fn isValid(self , rsthis: & QRawFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont7isValidEv()};
    let mut ret = unsafe {C_ZNK8QRawFont7isValidEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QRectF QRawFont::boundingRect(quint32 glyphIndex);
impl /*struct*/ QRawFont {
  pub fn boundingRect<RetType, T: QRawFont_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QRawFont_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  QRectF QRawFont::boundingRect(quint32 glyphIndex);
impl<'a> /*trait*/ QRawFont_boundingRect<QRectF> for (u32) {
  fn boundingRect(self , rsthis: & QRawFont) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont12boundingRectEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {C_ZNK8QRawFont12boundingRectEj(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRawFont::supportsCharacter(uint ucs4);
impl /*struct*/ QRawFont {
  pub fn supportsCharacter<RetType, T: QRawFont_supportsCharacter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.supportsCharacter(self);
    // return 1;
  }
}

pub trait QRawFont_supportsCharacter<RetType> {
  fn supportsCharacter(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  bool QRawFont::supportsCharacter(uint ucs4);
impl<'a> /*trait*/ QRawFont_supportsCharacter<i8> for (u32) {
  fn supportsCharacter(self , rsthis: & QRawFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont17supportsCharacterEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {C_ZNK8QRawFont17supportsCharacterEj(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QRawFont::swap(QRawFont & other);
impl /*struct*/ QRawFont {
  pub fn swap<RetType, T: QRawFont_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QRawFont_swap<RetType> {
  fn swap(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  void QRawFont::swap(QRawFont & other);
impl<'a> /*trait*/ QRawFont_swap<()> for (&'a QRawFont) {
  fn swap(self , rsthis: & QRawFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFont4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN8QRawFont4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QRawFont::descent();
impl /*struct*/ QRawFont {
  pub fn descent<RetType, T: QRawFont_descent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.descent(self);
    // return 1;
  }
}

pub trait QRawFont_descent<RetType> {
  fn descent(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  qreal QRawFont::descent();
impl<'a> /*trait*/ QRawFont_descent<f64> for () {
  fn descent(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont7descentEv()};
    let mut ret = unsafe {C_ZNK8QRawFont7descentEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QRawFont::QRawFont();
impl /*struct*/ QRawFont {
  pub fn new<T: QRawFont_new>(value: T) -> QRawFont {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QRawFont_new {
  fn new(self) -> QRawFont;
}

  // proto:  void QRawFont::QRawFont();
impl<'a> /*trait*/ QRawFont_new for () {
  fn new(self) -> QRawFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFontC2Ev()};
    let ctysz: c_int = unsafe{QRawFont_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN8QRawFontC2Ev()};
    let rsthis = QRawFont{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRawFont::setPixelSize(qreal pixelSize);
impl /*struct*/ QRawFont {
  pub fn setPixelSize<RetType, T: QRawFont_setPixelSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPixelSize(self);
    // return 1;
  }
}

pub trait QRawFont_setPixelSize<RetType> {
  fn setPixelSize(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  void QRawFont::setPixelSize(qreal pixelSize);
impl<'a> /*trait*/ QRawFont_setPixelSize<()> for (f64) {
  fn setPixelSize(self , rsthis: & QRawFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFont12setPixelSizeEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN8QRawFont12setPixelSizeEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QRawFont::glyphIndexesForChars(const QChar * chars, int numChars, quint32 * glyphIndexes, int * numGlyphs);
impl /*struct*/ QRawFont {
  pub fn glyphIndexesForChars<RetType, T: QRawFont_glyphIndexesForChars<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glyphIndexesForChars(self);
    // return 1;
  }
}

pub trait QRawFont_glyphIndexesForChars<RetType> {
  fn glyphIndexesForChars(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  bool QRawFont::glyphIndexesForChars(const QChar * chars, int numChars, quint32 * glyphIndexes, int * numGlyphs);
impl<'a> /*trait*/ QRawFont_glyphIndexesForChars<i8> for (&'a QChar, i32, &'a mut Vec<u32>, &'a mut Vec<i32>) {
  fn glyphIndexesForChars(self , rsthis: & QRawFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont20glyphIndexesForCharsEPK5QChariPjPi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_uint;
    let arg3 = self.3.as_ptr()  as *mut c_int;
    let mut ret = unsafe {C_ZNK8QRawFont20glyphIndexesForCharsEPK5QChariPjPi(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QRawFont::styleName();
impl /*struct*/ QRawFont {
  pub fn styleName<RetType, T: QRawFont_styleName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.styleName(self);
    // return 1;
  }
}

pub trait QRawFont_styleName<RetType> {
  fn styleName(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  QString QRawFont::styleName();
impl<'a> /*trait*/ QRawFont_styleName<QString> for () {
  fn styleName(self , rsthis: & QRawFont) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont9styleNameEv()};
    let mut ret = unsafe {C_ZNK8QRawFont9styleNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QRawFont::underlinePosition();
impl /*struct*/ QRawFont {
  pub fn underlinePosition<RetType, T: QRawFont_underlinePosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.underlinePosition(self);
    // return 1;
  }
}

pub trait QRawFont_underlinePosition<RetType> {
  fn underlinePosition(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  qreal QRawFont::underlinePosition();
impl<'a> /*trait*/ QRawFont_underlinePosition<f64> for () {
  fn underlinePosition(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont17underlinePositionEv()};
    let mut ret = unsafe {C_ZNK8QRawFont17underlinePositionEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QRawFont::unitsPerEm();
impl /*struct*/ QRawFont {
  pub fn unitsPerEm<RetType, T: QRawFont_unitsPerEm<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unitsPerEm(self);
    // return 1;
  }
}

pub trait QRawFont_unitsPerEm<RetType> {
  fn unitsPerEm(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  qreal QRawFont::unitsPerEm();
impl<'a> /*trait*/ QRawFont_unitsPerEm<f64> for () {
  fn unitsPerEm(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont10unitsPerEmEv()};
    let mut ret = unsafe {C_ZNK8QRawFont10unitsPerEmEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  bool QRawFont::supportsCharacter(QChar character);
impl<'a> /*trait*/ QRawFont_supportsCharacter<i8> for (QChar) {
  fn supportsCharacter(self , rsthis: & QRawFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont17supportsCharacterE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK8QRawFont17supportsCharacterE5QChar(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QRawFont::familyName();
impl /*struct*/ QRawFont {
  pub fn familyName<RetType, T: QRawFont_familyName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.familyName(self);
    // return 1;
  }
}

pub trait QRawFont_familyName<RetType> {
  fn familyName(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  QString QRawFont::familyName();
impl<'a> /*trait*/ QRawFont_familyName<QString> for () {
  fn familyName(self , rsthis: & QRawFont) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont10familyNameEv()};
    let mut ret = unsafe {C_ZNK8QRawFont10familyNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRawFont::advancesForGlyphIndexes(const quint32 * glyphIndexes, QPointF * advances, int numGlyphs);
impl /*struct*/ QRawFont {
  pub fn advancesForGlyphIndexes<RetType, T: QRawFont_advancesForGlyphIndexes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.advancesForGlyphIndexes(self);
    // return 1;
  }
}

pub trait QRawFont_advancesForGlyphIndexes<RetType> {
  fn advancesForGlyphIndexes(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  bool QRawFont::advancesForGlyphIndexes(const quint32 * glyphIndexes, QPointF * advances, int numGlyphs);
impl<'a> /*trait*/ QRawFont_advancesForGlyphIndexes<i8> for (&'a  Vec<u32>, &'a QPointF, i32) {
  fn advancesForGlyphIndexes(self , rsthis: & QRawFont) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont23advancesForGlyphIndexesEPKjP7QPointFi()};
    let arg0 = self.0.as_ptr()  as *mut c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {C_ZNK8QRawFont23advancesForGlyphIndexesEPKjP7QPointFi(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  qreal QRawFont::pixelSize();
impl /*struct*/ QRawFont {
  pub fn pixelSize<RetType, T: QRawFont_pixelSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pixelSize(self);
    // return 1;
  }
}

pub trait QRawFont_pixelSize<RetType> {
  fn pixelSize(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  qreal QRawFont::pixelSize();
impl<'a> /*trait*/ QRawFont_pixelSize<f64> for () {
  fn pixelSize(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont9pixelSizeEv()};
    let mut ret = unsafe {C_ZNK8QRawFont9pixelSizeEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  int QRawFont::weight();
impl /*struct*/ QRawFont {
  pub fn weight<RetType, T: QRawFont_weight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.weight(self);
    // return 1;
  }
}

pub trait QRawFont_weight<RetType> {
  fn weight(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  int QRawFont::weight();
impl<'a> /*trait*/ QRawFont_weight<i32> for () {
  fn weight(self , rsthis: & QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont6weightEv()};
    let mut ret = unsafe {C_ZNK8QRawFont6weightEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QRawFont::QRawFont(const QRawFont & other);
impl<'a> /*trait*/ QRawFont_new for (&'a QRawFont) {
  fn new(self) -> QRawFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFontC2ERKS_()};
    let ctysz: c_int = unsafe{QRawFont_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN8QRawFontC2ERKS_(arg0)};
    let rsthis = QRawFont{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QRawFont::xHeight();
impl /*struct*/ QRawFont {
  pub fn xHeight<RetType, T: QRawFont_xHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.xHeight(self);
    // return 1;
  }
}

pub trait QRawFont_xHeight<RetType> {
  fn xHeight(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  qreal QRawFont::xHeight();
impl<'a> /*trait*/ QRawFont_xHeight<f64> for () {
  fn xHeight(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont7xHeightEv()};
    let mut ret = unsafe {C_ZNK8QRawFont7xHeightEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QRawFont::~QRawFont();
impl /*struct*/ QRawFont {
  pub fn free<RetType, T: QRawFont_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QRawFont_free<RetType> {
  fn free(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  void QRawFont::~QRawFont();
impl<'a> /*trait*/ QRawFont_free<()> for () {
  fn free(self , rsthis: & QRawFont) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QRawFontD2Ev()};
     unsafe {C_ZN8QRawFontD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QRawFont::pathForGlyph(quint32 glyphIndex);
impl /*struct*/ QRawFont {
  pub fn pathForGlyph<RetType, T: QRawFont_pathForGlyph<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pathForGlyph(self);
    // return 1;
  }
}

pub trait QRawFont_pathForGlyph<RetType> {
  fn pathForGlyph(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  QPainterPath QRawFont::pathForGlyph(quint32 glyphIndex);
impl<'a> /*trait*/ QRawFont_pathForGlyph<QPainterPath> for (u32) {
  fn pathForGlyph(self , rsthis: & QRawFont) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont12pathForGlyphEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {C_ZNK8QRawFont12pathForGlyphEj(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QRawFont::fontTable(const char * tagName);
impl /*struct*/ QRawFont {
  pub fn fontTable<RetType, T: QRawFont_fontTable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontTable(self);
    // return 1;
  }
}

pub trait QRawFont_fontTable<RetType> {
  fn fontTable(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  QByteArray QRawFont::fontTable(const char * tagName);
impl<'a> /*trait*/ QRawFont_fontTable<QByteArray> for (&'a  String) {
  fn fontTable(self , rsthis: & QRawFont) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont9fontTableEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK8QRawFont9fontTableEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QRawFont::maxCharWidth();
impl /*struct*/ QRawFont {
  pub fn maxCharWidth<RetType, T: QRawFont_maxCharWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maxCharWidth(self);
    // return 1;
  }
}

pub trait QRawFont_maxCharWidth<RetType> {
  fn maxCharWidth(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  qreal QRawFont::maxCharWidth();
impl<'a> /*trait*/ QRawFont_maxCharWidth<f64> for () {
  fn maxCharWidth(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont12maxCharWidthEv()};
    let mut ret = unsafe {C_ZNK8QRawFont12maxCharWidthEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  QVector<quint32> QRawFont::glyphIndexesForString(const QString & text);
impl /*struct*/ QRawFont {
  pub fn glyphIndexesForString<RetType, T: QRawFont_glyphIndexesForString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glyphIndexesForString(self);
    // return 1;
  }
}

pub trait QRawFont_glyphIndexesForString<RetType> {
  fn glyphIndexesForString(self , rsthis: & QRawFont) -> RetType;
}

  // proto:  QVector<quint32> QRawFont::glyphIndexesForString(const QString & text);
impl<'a> /*trait*/ QRawFont_glyphIndexesForString<u64> for (&'a QString) {
  fn glyphIndexesForString(self , rsthis: & QRawFont) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QRawFont21glyphIndexesForStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK8QRawFont21glyphIndexesForStringERK7QString(rsthis.qclsinst, arg0)};
    return ret as u64; // 5
    // return 1;
  }
}

// <= body block end

