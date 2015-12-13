// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTextInlineObject::setAscent(qreal a);
  fn _ZN17QTextInlineObject9setAscentEd(arg0: c_double) -> i32;
  // proto: double QTextInlineObject::width();
  fn _ZNK17QTextInlineObject5widthEv() -> i32;
  // proto: int QTextInlineObject::formatIndex();
  fn _ZNK17QTextInlineObject11formatIndexEv() -> i32;
  // proto: QRectF QTextInlineObject::rect();
  fn _ZNK17QTextInlineObject4rectEv() -> i32;
  // proto: int QTextInlineObject::textPosition();
  fn _ZNK17QTextInlineObject12textPositionEv() -> i32;
  // proto: void QTextInlineObject::setDescent(qreal d);
  fn _ZN17QTextInlineObject10setDescentEd(arg0: c_double) -> i32;
  // proto: double QTextInlineObject::height();
  fn _ZNK17QTextInlineObject6heightEv() -> i32;
  // proto: bool QTextInlineObject::isValid();
  fn _ZNK17QTextInlineObject7isValidEv() -> i32;
  // proto: void QTextInlineObject::NewQTextInlineObject();
  fn _ZN17QTextInlineObjectC1Ev(qthis: *mut c_void) -> i32;
  // proto: QTextFormat QTextInlineObject::format();
  fn _ZNK17QTextInlineObject6formatEv() -> i32;
  // proto: double QTextInlineObject::descent();
  fn _ZNK17QTextInlineObject7descentEv() -> i32;
  // proto: double QTextInlineObject::ascent();
  fn _ZNK17QTextInlineObject6ascentEv() -> i32;
  // proto: void QTextInlineObject::setWidth(qreal w);
  fn _ZN17QTextInlineObject8setWidthEd(arg0: c_double) -> i32;
}

// body block begin
// class sizeof(QTextInlineObject)=16
pub struct QTextInlineObject {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextInlineObject {
  pub fn setAscent<T: QTextInlineObject_setAscent>(&mut self, value: T) -> i32 {
    value.setAscent(self);
    return 1;
  }
}

pub trait QTextInlineObject_setAscent {
  fn setAscent(self, this: &mut QTextInlineObject) -> i32;
}

// proto: void QTextInlineObject::setAscent(qreal a);
impl<'a> /*trait*/ QTextInlineObject_setAscent for (f64) {
  fn setAscent(self, this: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject9setAscentEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN17QTextInlineObject9setAscentEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn width<T: QTextInlineObject_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QTextInlineObject_width {
  fn width(self, this: &mut QTextInlineObject) -> i32;
}

// proto: double QTextInlineObject::width();
impl<'a> /*trait*/ QTextInlineObject_width for () {
  fn width(self, this: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject5widthEv()};
    unsafe {_ZNK17QTextInlineObject5widthEv()};
    return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn formatIndex<T: QTextInlineObject_formatIndex>(&mut self, value: T) -> i32 {
    value.formatIndex(self);
    return 1;
  }
}

pub trait QTextInlineObject_formatIndex {
  fn formatIndex(self, this: &mut QTextInlineObject) -> i32;
}

// proto: int QTextInlineObject::formatIndex();
impl<'a> /*trait*/ QTextInlineObject_formatIndex for () {
  fn formatIndex(self, this: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject11formatIndexEv()};
    unsafe {_ZNK17QTextInlineObject11formatIndexEv()};
    return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn rect<T: QTextInlineObject_rect>(&mut self, value: T) -> i32 {
    value.rect(self);
    return 1;
  }
}

pub trait QTextInlineObject_rect {
  fn rect(self, this: &mut QTextInlineObject) -> i32;
}

// proto: QRectF QTextInlineObject::rect();
impl<'a> /*trait*/ QTextInlineObject_rect for () {
  fn rect(self, this: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject4rectEv()};
    unsafe {_ZNK17QTextInlineObject4rectEv()};
    return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn textPosition<T: QTextInlineObject_textPosition>(&mut self, value: T) -> i32 {
    value.textPosition(self);
    return 1;
  }
}

pub trait QTextInlineObject_textPosition {
  fn textPosition(self, this: &mut QTextInlineObject) -> i32;
}

// proto: int QTextInlineObject::textPosition();
impl<'a> /*trait*/ QTextInlineObject_textPosition for () {
  fn textPosition(self, this: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject12textPositionEv()};
    unsafe {_ZNK17QTextInlineObject12textPositionEv()};
    return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn setDescent<T: QTextInlineObject_setDescent>(&mut self, value: T) -> i32 {
    value.setDescent(self);
    return 1;
  }
}

pub trait QTextInlineObject_setDescent {
  fn setDescent(self, this: &mut QTextInlineObject) -> i32;
}

// proto: void QTextInlineObject::setDescent(qreal d);
impl<'a> /*trait*/ QTextInlineObject_setDescent for (f64) {
  fn setDescent(self, this: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject10setDescentEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN17QTextInlineObject10setDescentEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn height<T: QTextInlineObject_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QTextInlineObject_height {
  fn height(self, this: &mut QTextInlineObject) -> i32;
}

// proto: double QTextInlineObject::height();
impl<'a> /*trait*/ QTextInlineObject_height for () {
  fn height(self, this: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6heightEv()};
    unsafe {_ZNK17QTextInlineObject6heightEv()};
    return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn isValid<T: QTextInlineObject_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextInlineObject_isValid {
  fn isValid(self, this: &mut QTextInlineObject) -> i32;
}

// proto: bool QTextInlineObject::isValid();
impl<'a> /*trait*/ QTextInlineObject_isValid for () {
  fn isValid(self, this: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject7isValidEv()};
    unsafe {_ZNK17QTextInlineObject7isValidEv()};
    return 1;
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
  pub fn format<T: QTextInlineObject_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QTextInlineObject_format {
  fn format(self, this: &mut QTextInlineObject) -> i32;
}

// proto: QTextFormat QTextInlineObject::format();
impl<'a> /*trait*/ QTextInlineObject_format for () {
  fn format(self, this: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6formatEv()};
    unsafe {_ZNK17QTextInlineObject6formatEv()};
    return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn descent<T: QTextInlineObject_descent>(&mut self, value: T) -> i32 {
    value.descent(self);
    return 1;
  }
}

pub trait QTextInlineObject_descent {
  fn descent(self, this: &mut QTextInlineObject) -> i32;
}

// proto: double QTextInlineObject::descent();
impl<'a> /*trait*/ QTextInlineObject_descent for () {
  fn descent(self, this: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject7descentEv()};
    unsafe {_ZNK17QTextInlineObject7descentEv()};
    return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn ascent<T: QTextInlineObject_ascent>(&mut self, value: T) -> i32 {
    value.ascent(self);
    return 1;
  }
}

pub trait QTextInlineObject_ascent {
  fn ascent(self, this: &mut QTextInlineObject) -> i32;
}

// proto: double QTextInlineObject::ascent();
impl<'a> /*trait*/ QTextInlineObject_ascent for () {
  fn ascent(self, this: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6ascentEv()};
    unsafe {_ZNK17QTextInlineObject6ascentEv()};
    return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn setWidth<T: QTextInlineObject_setWidth>(&mut self, value: T) -> i32 {
    value.setWidth(self);
    return 1;
  }
}

pub trait QTextInlineObject_setWidth {
  fn setWidth(self, this: &mut QTextInlineObject) -> i32;
}

// proto: void QTextInlineObject::setWidth(qreal w);
impl<'a> /*trait*/ QTextInlineObject_setWidth for (f64) {
  fn setWidth(self, this: &mut QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject8setWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN17QTextInlineObject8setWidthEd(arg0)};
    return 1;
  }
}

