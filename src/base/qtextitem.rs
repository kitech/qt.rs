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
  // proto: double QTextItem::descent();
  fn _ZNK9QTextItem7descentEv() -> i32;
  // proto: double QTextItem::width();
  fn _ZNK9QTextItem5widthEv() -> i32;
  // proto: QFont QTextItem::font();
  fn _ZNK9QTextItem4fontEv() -> i32;
  // proto: double QTextItem::ascent();
  fn _ZNK9QTextItem6ascentEv() -> i32;
  // proto: QString QTextItem::text();
  fn _ZNK9QTextItem4textEv() -> i32;
}

// body block begin
// class sizeof(QTextItem)=1
pub struct QTextItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextItem {
  pub fn descent<T: QTextItem_descent>(&mut self, value: T) -> i32 {
    value.descent(self);
    return 1;
  }
}

pub trait QTextItem_descent {
  fn descent(self, this: &mut QTextItem) -> i32;
}

// proto: double QTextItem::descent();
impl<'a> /*trait*/ QTextItem_descent for () {
  fn descent(self, this: &mut QTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem7descentEv()};
    unsafe {_ZNK9QTextItem7descentEv()};
    return 1;
  }
}

impl /*struct*/ QTextItem {
  pub fn width<T: QTextItem_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QTextItem_width {
  fn width(self, this: &mut QTextItem) -> i32;
}

// proto: double QTextItem::width();
impl<'a> /*trait*/ QTextItem_width for () {
  fn width(self, this: &mut QTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem5widthEv()};
    unsafe {_ZNK9QTextItem5widthEv()};
    return 1;
  }
}

impl /*struct*/ QTextItem {
  pub fn font<T: QTextItem_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QTextItem_font {
  fn font(self, this: &mut QTextItem) -> i32;
}

// proto: QFont QTextItem::font();
impl<'a> /*trait*/ QTextItem_font for () {
  fn font(self, this: &mut QTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem4fontEv()};
    unsafe {_ZNK9QTextItem4fontEv()};
    return 1;
  }
}

impl /*struct*/ QTextItem {
  pub fn ascent<T: QTextItem_ascent>(&mut self, value: T) -> i32 {
    value.ascent(self);
    return 1;
  }
}

pub trait QTextItem_ascent {
  fn ascent(self, this: &mut QTextItem) -> i32;
}

// proto: double QTextItem::ascent();
impl<'a> /*trait*/ QTextItem_ascent for () {
  fn ascent(self, this: &mut QTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem6ascentEv()};
    unsafe {_ZNK9QTextItem6ascentEv()};
    return 1;
  }
}

impl /*struct*/ QTextItem {
  pub fn text<T: QTextItem_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QTextItem_text {
  fn text(self, this: &mut QTextItem) -> i32;
}

// proto: QString QTextItem::text();
impl<'a> /*trait*/ QTextItem_text for () {
  fn text(self, this: &mut QTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem4textEv()};
    unsafe {_ZNK9QTextItem4textEv()};
    return 1;
  }
}

