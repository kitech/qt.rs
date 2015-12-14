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
  pub fn setAscent<T: QTextInlineObject_setAscent>(&mut self, value: T)  {
     value.setAscent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_setAscent {
  fn setAscent(self, rsthis: &mut QTextInlineObject) ;
}

// proto:  void QTextInlineObject::setAscent(qreal a);
impl<'a> /*trait*/ QTextInlineObject_setAscent for (f64) {
  fn setAscent(self, rsthis: &mut QTextInlineObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject9setAscentEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN17QTextInlineObject9setAscentEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn width<T: QTextInlineObject_width>(&mut self, value: T) -> f64 {
    return value.width(self);
    // return 1;
  }
}

pub trait QTextInlineObject_width {
  fn width(self, rsthis: &mut QTextInlineObject) -> f64;
}

// proto:  double QTextInlineObject::width();
impl<'a> /*trait*/ QTextInlineObject_width for () {
  fn width(self, rsthis: &mut QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject5widthEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn formatIndex<T: QTextInlineObject_formatIndex>(&mut self, value: T) -> i32 {
    return value.formatIndex(self);
    // return 1;
  }
}

pub trait QTextInlineObject_formatIndex {
  fn formatIndex(self, rsthis: &mut QTextInlineObject) -> i32;
}

// proto:  int QTextInlineObject::formatIndex();
impl<'a> /*trait*/ QTextInlineObject_formatIndex for () {
  fn formatIndex(self, rsthis: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject11formatIndexEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject11formatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn rect<T: QTextInlineObject_rect>(&mut self, value: T) -> QRectF {
    return value.rect(self);
    // return 1;
  }
}

pub trait QTextInlineObject_rect {
  fn rect(self, rsthis: &mut QTextInlineObject) -> QRectF;
}

// proto:  QRectF QTextInlineObject::rect();
impl<'a> /*trait*/ QTextInlineObject_rect for () {
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
  pub fn textPosition<T: QTextInlineObject_textPosition>(&mut self, value: T) -> i32 {
    return value.textPosition(self);
    // return 1;
  }
}

pub trait QTextInlineObject_textPosition {
  fn textPosition(self, rsthis: &mut QTextInlineObject) -> i32;
}

// proto:  int QTextInlineObject::textPosition();
impl<'a> /*trait*/ QTextInlineObject_textPosition for () {
  fn textPosition(self, rsthis: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject12textPositionEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject12textPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn setDescent<T: QTextInlineObject_setDescent>(&mut self, value: T)  {
     value.setDescent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_setDescent {
  fn setDescent(self, rsthis: &mut QTextInlineObject) ;
}

// proto:  void QTextInlineObject::setDescent(qreal d);
impl<'a> /*trait*/ QTextInlineObject_setDescent for (f64) {
  fn setDescent(self, rsthis: &mut QTextInlineObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject10setDescentEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN17QTextInlineObject10setDescentEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn height<T: QTextInlineObject_height>(&mut self, value: T) -> f64 {
    return value.height(self);
    // return 1;
  }
}

pub trait QTextInlineObject_height {
  fn height(self, rsthis: &mut QTextInlineObject) -> f64;
}

// proto:  double QTextInlineObject::height();
impl<'a> /*trait*/ QTextInlineObject_height for () {
  fn height(self, rsthis: &mut QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6heightEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn isValid<T: QTextInlineObject_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextInlineObject_isValid {
  fn isValid(self, rsthis: &mut QTextInlineObject) -> i8;
}

// proto:  bool QTextInlineObject::isValid();
impl<'a> /*trait*/ QTextInlineObject_isValid for () {
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
  pub fn format<T: QTextInlineObject_format>(&mut self, value: T) -> QTextFormat {
    return value.format(self);
    // return 1;
  }
}

pub trait QTextInlineObject_format {
  fn format(self, rsthis: &mut QTextInlineObject) -> QTextFormat;
}

// proto:  QTextFormat QTextInlineObject::format();
impl<'a> /*trait*/ QTextInlineObject_format for () {
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
  pub fn descent<T: QTextInlineObject_descent>(&mut self, value: T) -> f64 {
    return value.descent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_descent {
  fn descent(self, rsthis: &mut QTextInlineObject) -> f64;
}

// proto:  double QTextInlineObject::descent();
impl<'a> /*trait*/ QTextInlineObject_descent for () {
  fn descent(self, rsthis: &mut QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject7descentEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject7descentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn ascent<T: QTextInlineObject_ascent>(&mut self, value: T) -> f64 {
    return value.ascent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_ascent {
  fn ascent(self, rsthis: &mut QTextInlineObject) -> f64;
}

// proto:  double QTextInlineObject::ascent();
impl<'a> /*trait*/ QTextInlineObject_ascent for () {
  fn ascent(self, rsthis: &mut QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6ascentEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject6ascentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn setWidth<T: QTextInlineObject_setWidth>(&mut self, value: T)  {
     value.setWidth(self);
    // return 1;
  }
}

pub trait QTextInlineObject_setWidth {
  fn setWidth(self, rsthis: &mut QTextInlineObject) ;
}

// proto:  void QTextInlineObject::setWidth(qreal w);
impl<'a> /*trait*/ QTextInlineObject_setWidth for (f64) {
  fn setWidth(self, rsthis: &mut QTextInlineObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN17QTextInlineObject8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

