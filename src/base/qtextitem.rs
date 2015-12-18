// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfont::QFont;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  double QTextItem::descent();
  fn _ZNK9QTextItem7descentEv(qthis: *mut c_void) -> c_double;
  // proto:  double QTextItem::width();
  fn _ZNK9QTextItem5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  QFont QTextItem::font();
  fn _ZNK9QTextItem4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QTextItem::ascent();
  fn _ZNK9QTextItem6ascentEv(qthis: *mut c_void) -> c_double;
  // proto:  QString QTextItem::text();
  fn _ZNK9QTextItem4textEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QTextItem)=1
pub struct QTextItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextItem {
  pub fn descent<RetType, T: QTextItem_descent<RetType>>(&mut self, value: T) -> RetType {
    return value.descent(self);
    // return 1;
  }
}

pub trait QTextItem_descent<RetType> {
  fn descent(self, rsthis: &mut QTextItem) -> RetType;
}

// proto:  double QTextItem::descent();
impl<'a> /*trait*/ QTextItem_descent<f64> for () {
  fn descent(self, rsthis: &mut QTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem7descentEv()};
    let mut ret = unsafe {_ZNK9QTextItem7descentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextItem {
  pub fn width<RetType, T: QTextItem_width<RetType>>(&mut self, value: T) -> RetType {
    return value.width(self);
    // return 1;
  }
}

pub trait QTextItem_width<RetType> {
  fn width(self, rsthis: &mut QTextItem) -> RetType;
}

// proto:  double QTextItem::width();
impl<'a> /*trait*/ QTextItem_width<f64> for () {
  fn width(self, rsthis: &mut QTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem5widthEv()};
    let mut ret = unsafe {_ZNK9QTextItem5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextItem {
  pub fn font<RetType, T: QTextItem_font<RetType>>(&mut self, value: T) -> RetType {
    return value.font(self);
    // return 1;
  }
}

pub trait QTextItem_font<RetType> {
  fn font(self, rsthis: &mut QTextItem) -> RetType;
}

// proto:  QFont QTextItem::font();
impl<'a> /*trait*/ QTextItem_font<QFont> for () {
  fn font(self, rsthis: &mut QTextItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem4fontEv()};
    let mut ret = unsafe {_ZNK9QTextItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextItem {
  pub fn ascent<RetType, T: QTextItem_ascent<RetType>>(&mut self, value: T) -> RetType {
    return value.ascent(self);
    // return 1;
  }
}

pub trait QTextItem_ascent<RetType> {
  fn ascent(self, rsthis: &mut QTextItem) -> RetType;
}

// proto:  double QTextItem::ascent();
impl<'a> /*trait*/ QTextItem_ascent<f64> for () {
  fn ascent(self, rsthis: &mut QTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem6ascentEv()};
    let mut ret = unsafe {_ZNK9QTextItem6ascentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextItem {
  pub fn text<RetType, T: QTextItem_text<RetType>>(&mut self, value: T) -> RetType {
    return value.text(self);
    // return 1;
  }
}

pub trait QTextItem_text<RetType> {
  fn text(self, rsthis: &mut QTextItem) -> RetType;
}

// proto:  QString QTextItem::text();
impl<'a> /*trait*/ QTextItem_text<QString> for () {
  fn text(self, rsthis: &mut QTextItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem4textEv()};
    let mut ret = unsafe {_ZNK9QTextItem4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

