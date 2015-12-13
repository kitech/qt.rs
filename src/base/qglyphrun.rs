// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrectf::QRectF;
use super::qpointf::QPointF;
use super::qrawfont::QRawFont;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGlyphRun::FreeQGlyphRun();
  fn _ZN9QGlyphRunD0Ev() -> i32;
  // proto: void QGlyphRun::setBoundingRect(const QRectF & boundingRect);
  fn _ZN9QGlyphRun15setBoundingRectERK6QRectF(arg0: *const c_void) -> i32;
  // proto: bool QGlyphRun::overline();
  fn _ZNK9QGlyphRun8overlineEv() -> i32;
  // proto: void QGlyphRun::setRawData(const quint32 * glyphIndexArray, const QPointF * glyphPositionArray, int size);
  fn _ZN9QGlyphRun10setRawDataEPKjPK7QPointFi(arg0: *const c_uint, arg1: *const c_void, arg2: c_int) -> i32;
  // proto: void QGlyphRun::setOverline(bool overline);
  fn _ZN9QGlyphRun11setOverlineEb(arg0: int8_t) -> i32;
  // proto: void QGlyphRun::swap(QGlyphRun & other);
  fn _ZN9QGlyphRun4swapERS_(arg0: *mut c_void) -> i32;
  // proto: void QGlyphRun::setUnderline(bool underline);
  fn _ZN9QGlyphRun12setUnderlineEb(arg0: int8_t) -> i32;
  // proto: QVector<QPointF> QGlyphRun::positions();
  fn _ZNK9QGlyphRun9positionsEv() -> i32;
  // proto: void QGlyphRun::clear();
  fn _ZN9QGlyphRun5clearEv() -> i32;
  // proto: bool QGlyphRun::strikeOut();
  fn _ZNK9QGlyphRun9strikeOutEv() -> i32;
  // proto: void QGlyphRun::NewQGlyphRun();
  fn _ZN9QGlyphRunC1Ev(qthis: *mut c_void) -> i32;
  // proto: QRawFont QGlyphRun::rawFont();
  fn _ZNK9QGlyphRun7rawFontEv() -> i32;
  // proto: void QGlyphRun::setRawFont(const QRawFont & rawFont);
  fn _ZN9QGlyphRun10setRawFontERK8QRawFont(arg0: *const c_void) -> i32;
  // proto: void QGlyphRun::NewQGlyphRun(const QGlyphRun & other);
  fn _ZN9QGlyphRunC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QGlyphRun::isRightToLeft();
  fn _ZNK9QGlyphRun13isRightToLeftEv() -> i32;
  // proto: QVector<quint32> QGlyphRun::glyphIndexes();
  fn _ZNK9QGlyphRun12glyphIndexesEv() -> i32;
  // proto: QRectF QGlyphRun::boundingRect();
  fn _ZNK9QGlyphRun12boundingRectEv() -> i32;
  // proto: void QGlyphRun::setRightToLeft(bool on);
  fn _ZN9QGlyphRun14setRightToLeftEb(arg0: int8_t) -> i32;
  // proto: bool QGlyphRun::underline();
  fn _ZNK9QGlyphRun9underlineEv() -> i32;
  // proto: void QGlyphRun::setStrikeOut(bool strikeOut);
  fn _ZN9QGlyphRun12setStrikeOutEb(arg0: int8_t) -> i32;
  // proto: bool QGlyphRun::isEmpty();
  fn _ZNK9QGlyphRun7isEmptyEv() -> i32;
}

// body block begin
// class sizeof(QGlyphRun)=1
pub struct QGlyphRun {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGlyphRun {
  pub fn FreeQGlyphRun<T: QGlyphRun_FreeQGlyphRun>(&mut self, value: T) -> i32 {
    value.FreeQGlyphRun(self);
    return 1;
  }
}

pub trait QGlyphRun_FreeQGlyphRun {
  fn FreeQGlyphRun(self, this: &mut QGlyphRun) -> i32;
}

// proto: void QGlyphRun::FreeQGlyphRun();
impl<'a> /*trait*/ QGlyphRun_FreeQGlyphRun for () {
  fn FreeQGlyphRun(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRunD0Ev()};
    unsafe {_ZN9QGlyphRunD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn setBoundingRect<T: QGlyphRun_setBoundingRect>(&mut self, value: T) -> i32 {
    value.setBoundingRect(self);
    return 1;
  }
}

pub trait QGlyphRun_setBoundingRect {
  fn setBoundingRect(self, this: &mut QGlyphRun) -> i32;
}

// proto: void QGlyphRun::setBoundingRect(const QRectF & boundingRect);
impl<'a> /*trait*/ QGlyphRun_setBoundingRect for (&'a  QRectF) {
  fn setBoundingRect(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun15setBoundingRectERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QGlyphRun15setBoundingRectERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn overline<T: QGlyphRun_overline>(&mut self, value: T) -> i32 {
    value.overline(self);
    return 1;
  }
}

pub trait QGlyphRun_overline {
  fn overline(self, this: &mut QGlyphRun) -> i32;
}

// proto: bool QGlyphRun::overline();
impl<'a> /*trait*/ QGlyphRun_overline for () {
  fn overline(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun8overlineEv()};
    unsafe {_ZNK9QGlyphRun8overlineEv()};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn setRawData<T: QGlyphRun_setRawData>(&mut self, value: T) -> i32 {
    value.setRawData(self);
    return 1;
  }
}

pub trait QGlyphRun_setRawData {
  fn setRawData(self, this: &mut QGlyphRun) -> i32;
}

// proto: void QGlyphRun::setRawData(const quint32 * glyphIndexArray, const QPointF * glyphPositionArray, int size);
impl<'a> /*trait*/ QGlyphRun_setRawData for (&'a  u32, &'a  QPointF, i32) {
  fn setRawData(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun10setRawDataEPKjPK7QPointFi()};
    let arg0 = self.0  as *const c_uint;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN9QGlyphRun10setRawDataEPKjPK7QPointFi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn setOverline<T: QGlyphRun_setOverline>(&mut self, value: T) -> i32 {
    value.setOverline(self);
    return 1;
  }
}

pub trait QGlyphRun_setOverline {
  fn setOverline(self, this: &mut QGlyphRun) -> i32;
}

// proto: void QGlyphRun::setOverline(bool overline);
impl<'a> /*trait*/ QGlyphRun_setOverline for (i8) {
  fn setOverline(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun11setOverlineEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QGlyphRun11setOverlineEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn swap<T: QGlyphRun_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QGlyphRun_swap {
  fn swap(self, this: &mut QGlyphRun) -> i32;
}

// proto: void QGlyphRun::swap(QGlyphRun & other);
impl<'a> /*trait*/ QGlyphRun_swap for (&'a mut QGlyphRun) {
  fn swap(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QGlyphRun4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn setUnderline<T: QGlyphRun_setUnderline>(&mut self, value: T) -> i32 {
    value.setUnderline(self);
    return 1;
  }
}

pub trait QGlyphRun_setUnderline {
  fn setUnderline(self, this: &mut QGlyphRun) -> i32;
}

// proto: void QGlyphRun::setUnderline(bool underline);
impl<'a> /*trait*/ QGlyphRun_setUnderline for (i8) {
  fn setUnderline(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun12setUnderlineEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QGlyphRun12setUnderlineEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn positions<T: QGlyphRun_positions>(&mut self, value: T) -> i32 {
    value.positions(self);
    return 1;
  }
}

pub trait QGlyphRun_positions {
  fn positions(self, this: &mut QGlyphRun) -> i32;
}

// proto: QVector<QPointF> QGlyphRun::positions();
impl<'a> /*trait*/ QGlyphRun_positions for () {
  fn positions(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun9positionsEv()};
    unsafe {_ZNK9QGlyphRun9positionsEv()};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn clear<T: QGlyphRun_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QGlyphRun_clear {
  fn clear(self, this: &mut QGlyphRun) -> i32;
}

// proto: void QGlyphRun::clear();
impl<'a> /*trait*/ QGlyphRun_clear for () {
  fn clear(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun5clearEv()};
    unsafe {_ZN9QGlyphRun5clearEv()};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn strikeOut<T: QGlyphRun_strikeOut>(&mut self, value: T) -> i32 {
    value.strikeOut(self);
    return 1;
  }
}

pub trait QGlyphRun_strikeOut {
  fn strikeOut(self, this: &mut QGlyphRun) -> i32;
}

// proto: bool QGlyphRun::strikeOut();
impl<'a> /*trait*/ QGlyphRun_strikeOut for () {
  fn strikeOut(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun9strikeOutEv()};
    unsafe {_ZNK9QGlyphRun9strikeOutEv()};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn NewQGlyphRun<T: QGlyphRun_NewQGlyphRun>(value: T) -> QGlyphRun {
    let rsthis = value.NewQGlyphRun();
    return rsthis;
    // return 1;
  }
}

pub trait QGlyphRun_NewQGlyphRun {
  fn NewQGlyphRun(self) -> QGlyphRun;
}

// proto: void QGlyphRun::NewQGlyphRun();
impl<'a> /*trait*/ QGlyphRun_NewQGlyphRun for () {
  fn NewQGlyphRun(self) -> QGlyphRun {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRunC1Ev()};
    unsafe {_ZN9QGlyphRunC1Ev(qthis)};
    let rsthis = QGlyphRun{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn rawFont<T: QGlyphRun_rawFont>(&mut self, value: T) -> i32 {
    value.rawFont(self);
    return 1;
  }
}

pub trait QGlyphRun_rawFont {
  fn rawFont(self, this: &mut QGlyphRun) -> i32;
}

// proto: QRawFont QGlyphRun::rawFont();
impl<'a> /*trait*/ QGlyphRun_rawFont for () {
  fn rawFont(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun7rawFontEv()};
    unsafe {_ZNK9QGlyphRun7rawFontEv()};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn setRawFont<T: QGlyphRun_setRawFont>(&mut self, value: T) -> i32 {
    value.setRawFont(self);
    return 1;
  }
}

pub trait QGlyphRun_setRawFont {
  fn setRawFont(self, this: &mut QGlyphRun) -> i32;
}

// proto: void QGlyphRun::setRawFont(const QRawFont & rawFont);
impl<'a> /*trait*/ QGlyphRun_setRawFont for (&'a  QRawFont) {
  fn setRawFont(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun10setRawFontERK8QRawFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QGlyphRun10setRawFontERK8QRawFont(arg0)};
    return 1;
  }
}

// proto: void QGlyphRun::NewQGlyphRun(const QGlyphRun & other);
impl<'a> /*trait*/ QGlyphRun_NewQGlyphRun for (&'a  QGlyphRun) {
  fn NewQGlyphRun(self) -> QGlyphRun {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRunC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QGlyphRunC1ERKS_(qthis, arg0)};
    let rsthis = QGlyphRun{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn isRightToLeft<T: QGlyphRun_isRightToLeft>(&mut self, value: T) -> i32 {
    value.isRightToLeft(self);
    return 1;
  }
}

pub trait QGlyphRun_isRightToLeft {
  fn isRightToLeft(self, this: &mut QGlyphRun) -> i32;
}

// proto: bool QGlyphRun::isRightToLeft();
impl<'a> /*trait*/ QGlyphRun_isRightToLeft for () {
  fn isRightToLeft(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun13isRightToLeftEv()};
    unsafe {_ZNK9QGlyphRun13isRightToLeftEv()};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn glyphIndexes<T: QGlyphRun_glyphIndexes>(&mut self, value: T) -> i32 {
    value.glyphIndexes(self);
    return 1;
  }
}

pub trait QGlyphRun_glyphIndexes {
  fn glyphIndexes(self, this: &mut QGlyphRun) -> i32;
}

// proto: QVector<quint32> QGlyphRun::glyphIndexes();
impl<'a> /*trait*/ QGlyphRun_glyphIndexes for () {
  fn glyphIndexes(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun12glyphIndexesEv()};
    unsafe {_ZNK9QGlyphRun12glyphIndexesEv()};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn boundingRect<T: QGlyphRun_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QGlyphRun_boundingRect {
  fn boundingRect(self, this: &mut QGlyphRun) -> i32;
}

// proto: QRectF QGlyphRun::boundingRect();
impl<'a> /*trait*/ QGlyphRun_boundingRect for () {
  fn boundingRect(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun12boundingRectEv()};
    unsafe {_ZNK9QGlyphRun12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn setRightToLeft<T: QGlyphRun_setRightToLeft>(&mut self, value: T) -> i32 {
    value.setRightToLeft(self);
    return 1;
  }
}

pub trait QGlyphRun_setRightToLeft {
  fn setRightToLeft(self, this: &mut QGlyphRun) -> i32;
}

// proto: void QGlyphRun::setRightToLeft(bool on);
impl<'a> /*trait*/ QGlyphRun_setRightToLeft for (i8) {
  fn setRightToLeft(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun14setRightToLeftEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QGlyphRun14setRightToLeftEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn underline<T: QGlyphRun_underline>(&mut self, value: T) -> i32 {
    value.underline(self);
    return 1;
  }
}

pub trait QGlyphRun_underline {
  fn underline(self, this: &mut QGlyphRun) -> i32;
}

// proto: bool QGlyphRun::underline();
impl<'a> /*trait*/ QGlyphRun_underline for () {
  fn underline(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun9underlineEv()};
    unsafe {_ZNK9QGlyphRun9underlineEv()};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn setStrikeOut<T: QGlyphRun_setStrikeOut>(&mut self, value: T) -> i32 {
    value.setStrikeOut(self);
    return 1;
  }
}

pub trait QGlyphRun_setStrikeOut {
  fn setStrikeOut(self, this: &mut QGlyphRun) -> i32;
}

// proto: void QGlyphRun::setStrikeOut(bool strikeOut);
impl<'a> /*trait*/ QGlyphRun_setStrikeOut for (i8) {
  fn setStrikeOut(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun12setStrikeOutEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QGlyphRun12setStrikeOutEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGlyphRun {
  pub fn isEmpty<T: QGlyphRun_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QGlyphRun_isEmpty {
  fn isEmpty(self, this: &mut QGlyphRun) -> i32;
}

// proto: bool QGlyphRun::isEmpty();
impl<'a> /*trait*/ QGlyphRun_isEmpty for () {
  fn isEmpty(self, this: &mut QGlyphRun) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun7isEmptyEv()};
    unsafe {_ZNK9QGlyphRun7isEmptyEv()};
    return 1;
  }
}

