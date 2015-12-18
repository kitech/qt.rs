// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrectf::QRectF;
use super::qtextformat::QTextFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTextInlineObject::setAscent(qreal a);
  fn _ZN17QTextInlineObject9setAscentEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QTextInlineObject::width();
  fn _ZNK17QTextInlineObject5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  int QTextInlineObject::formatIndex();
  fn _ZNK17QTextInlineObject11formatIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  QRectF QTextInlineObject::rect();
  fn _ZNK17QTextInlineObject4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextInlineObject::textPosition();
  fn _ZNK17QTextInlineObject12textPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextInlineObject::setDescent(qreal d);
  fn _ZN17QTextInlineObject10setDescentEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QTextInlineObject::height();
  fn _ZNK17QTextInlineObject6heightEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QTextInlineObject::isValid();
  fn _ZNK17QTextInlineObject7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextInlineObject::NewQTextInlineObject();
  fn _ZN17QTextInlineObjectC1Ev(qthis: *mut c_void) ;
  // proto:  QTextFormat QTextInlineObject::format();
  fn _ZNK17QTextInlineObject6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QTextInlineObject::descent();
  fn _ZNK17QTextInlineObject7descentEv(qthis: *mut c_void) -> c_double;
  // proto:  double QTextInlineObject::ascent();
  fn _ZNK17QTextInlineObject6ascentEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextInlineObject::setWidth(qreal w);
  fn _ZN17QTextInlineObject8setWidthEd(qthis: *mut c_void, arg0: c_double) ;
}

// body block begin
// class sizeof(QTextInlineObject)=16
pub struct QTextInlineObject {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextInlineObject {
  pub fn setAscent<RetType, T: QTextInlineObject_setAscent<RetType>>(&mut self, value: T) -> RetType {
    return value.setAscent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_setAscent<RetType> {
  fn setAscent(self, rsthis: &mut QTextInlineObject) -> RetType;
}

// proto:  void QTextInlineObject::setAscent(qreal a);
impl<'a> /*trait*/ QTextInlineObject_setAscent<()> for (f64) {
  fn setAscent(self, rsthis: &mut QTextInlineObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject9setAscentEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN17QTextInlineObject9setAscentEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn width<RetType, T: QTextInlineObject_width<RetType>>(&mut self, value: T) -> RetType {
    return value.width(self);
    // return 1;
  }
}

pub trait QTextInlineObject_width<RetType> {
  fn width(self, rsthis: &mut QTextInlineObject) -> RetType;
}

// proto:  double QTextInlineObject::width();
impl<'a> /*trait*/ QTextInlineObject_width<f64> for () {
  fn width(self, rsthis: &mut QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject5widthEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn formatIndex<RetType, T: QTextInlineObject_formatIndex<RetType>>(&mut self, value: T) -> RetType {
    return value.formatIndex(self);
    // return 1;
  }
}

pub trait QTextInlineObject_formatIndex<RetType> {
  fn formatIndex(self, rsthis: &mut QTextInlineObject) -> RetType;
}

// proto:  int QTextInlineObject::formatIndex();
impl<'a> /*trait*/ QTextInlineObject_formatIndex<i32> for () {
  fn formatIndex(self, rsthis: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject11formatIndexEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject11formatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn rect<RetType, T: QTextInlineObject_rect<RetType>>(&mut self, value: T) -> RetType {
    return value.rect(self);
    // return 1;
  }
}

pub trait QTextInlineObject_rect<RetType> {
  fn rect(self, rsthis: &mut QTextInlineObject) -> RetType;
}

// proto:  QRectF QTextInlineObject::rect();
impl<'a> /*trait*/ QTextInlineObject_rect<QRectF> for () {
  fn rect(self, rsthis: &mut QTextInlineObject) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject4rectEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn textPosition<RetType, T: QTextInlineObject_textPosition<RetType>>(&mut self, value: T) -> RetType {
    return value.textPosition(self);
    // return 1;
  }
}

pub trait QTextInlineObject_textPosition<RetType> {
  fn textPosition(self, rsthis: &mut QTextInlineObject) -> RetType;
}

// proto:  int QTextInlineObject::textPosition();
impl<'a> /*trait*/ QTextInlineObject_textPosition<i32> for () {
  fn textPosition(self, rsthis: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject12textPositionEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject12textPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn setDescent<RetType, T: QTextInlineObject_setDescent<RetType>>(&mut self, value: T) -> RetType {
    return value.setDescent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_setDescent<RetType> {
  fn setDescent(self, rsthis: &mut QTextInlineObject) -> RetType;
}

// proto:  void QTextInlineObject::setDescent(qreal d);
impl<'a> /*trait*/ QTextInlineObject_setDescent<()> for (f64) {
  fn setDescent(self, rsthis: &mut QTextInlineObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject10setDescentEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN17QTextInlineObject10setDescentEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn height<RetType, T: QTextInlineObject_height<RetType>>(&mut self, value: T) -> RetType {
    return value.height(self);
    // return 1;
  }
}

pub trait QTextInlineObject_height<RetType> {
  fn height(self, rsthis: &mut QTextInlineObject) -> RetType;
}

// proto:  double QTextInlineObject::height();
impl<'a> /*trait*/ QTextInlineObject_height<f64> for () {
  fn height(self, rsthis: &mut QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6heightEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn isValid<RetType, T: QTextInlineObject_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextInlineObject_isValid<RetType> {
  fn isValid(self, rsthis: &mut QTextInlineObject) -> RetType;
}

// proto:  bool QTextInlineObject::isValid();
impl<'a> /*trait*/ QTextInlineObject_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QTextInlineObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject7isValidEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn NewQTextInlineObject<T: QTextInlineObject_NewQTextInlineObject>(value: T) -> QTextInlineObject {
    let rsthis = value.NewQTextInlineObject();
    return rsthis;
    // return 1;
  }
}

pub trait QTextInlineObject_NewQTextInlineObject {
  fn NewQTextInlineObject(self) -> QTextInlineObject;
}

// proto: void QTextInlineObject::NewQTextInlineObject();
impl<'a> /*trait*/ QTextInlineObject_NewQTextInlineObject for () {
  fn NewQTextInlineObject(self) -> QTextInlineObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObjectC1Ev()};
    unsafe {_ZN17QTextInlineObjectC1Ev(qthis)};
    let rsthis = QTextInlineObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn format<RetType, T: QTextInlineObject_format<RetType>>(&mut self, value: T) -> RetType {
    return value.format(self);
    // return 1;
  }
}

pub trait QTextInlineObject_format<RetType> {
  fn format(self, rsthis: &mut QTextInlineObject) -> RetType;
}

// proto:  QTextFormat QTextInlineObject::format();
impl<'a> /*trait*/ QTextInlineObject_format<QTextFormat> for () {
  fn format(self, rsthis: &mut QTextInlineObject) -> QTextFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6formatEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject6formatEv(rsthis.qclsinst)};
    let mut ret1 = QTextFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn descent<RetType, T: QTextInlineObject_descent<RetType>>(&mut self, value: T) -> RetType {
    return value.descent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_descent<RetType> {
  fn descent(self, rsthis: &mut QTextInlineObject) -> RetType;
}

// proto:  double QTextInlineObject::descent();
impl<'a> /*trait*/ QTextInlineObject_descent<f64> for () {
  fn descent(self, rsthis: &mut QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject7descentEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject7descentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn ascent<RetType, T: QTextInlineObject_ascent<RetType>>(&mut self, value: T) -> RetType {
    return value.ascent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_ascent<RetType> {
  fn ascent(self, rsthis: &mut QTextInlineObject) -> RetType;
}

// proto:  double QTextInlineObject::ascent();
impl<'a> /*trait*/ QTextInlineObject_ascent<f64> for () {
  fn ascent(self, rsthis: &mut QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6ascentEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject6ascentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn setWidth<RetType, T: QTextInlineObject_setWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setWidth(self);
    // return 1;
  }
}

pub trait QTextInlineObject_setWidth<RetType> {
  fn setWidth(self, rsthis: &mut QTextInlineObject) -> RetType;
}

// proto:  void QTextInlineObject::setWidth(qreal w);
impl<'a> /*trait*/ QTextInlineObject_setWidth<()> for (f64) {
  fn setWidth(self, rsthis: &mut QTextInlineObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN17QTextInlineObject8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

