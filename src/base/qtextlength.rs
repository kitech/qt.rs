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
  // proto:  double QTextLength::value(qreal maximumLength);
  fn _ZNK11QTextLength5valueEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QTextLength::NewQTextLength();
  fn _ZN11QTextLengthC1Ev(qthis: *mut c_void) ;
  // proto:  double QTextLength::rawValue();
  fn _ZNK11QTextLength8rawValueEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QTextLength)=16
pub struct QTextLength {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextLength {
  pub fn value<T: QTextLength_value>(&mut self, value: T) -> f64 {
    return value.value(self);
    // return 1;
  }
}

pub trait QTextLength_value {
  fn value(self, rsthis: &mut QTextLength) -> f64;
}

// proto:  double QTextLength::value(qreal maximumLength);
impl<'a> /*trait*/ QTextLength_value for (f64) {
  fn value(self, rsthis: &mut QTextLength) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLength5valueEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK11QTextLength5valueEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLength {
  pub fn NewQTextLength<T: QTextLength_NewQTextLength>(value: T) -> QTextLength {
    let rsthis = value.NewQTextLength();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLength_NewQTextLength {
  fn NewQTextLength(self) -> QTextLength;
}

// proto: void QTextLength::NewQTextLength();
impl<'a> /*trait*/ QTextLength_NewQTextLength for () {
  fn NewQTextLength(self) -> QTextLength {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLengthC1Ev()};
    unsafe {_ZN11QTextLengthC1Ev(qthis)};
    let rsthis = QTextLength{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextLength {
  pub fn rawValue<T: QTextLength_rawValue>(&mut self, value: T) -> f64 {
    return value.rawValue(self);
    // return 1;
  }
}

pub trait QTextLength_rawValue {
  fn rawValue(self, rsthis: &mut QTextLength) -> f64;
}

// proto:  double QTextLength::rawValue();
impl<'a> /*trait*/ QTextLength_rawValue for () {
  fn rawValue(self, rsthis: &mut QTextLength) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLength8rawValueEv()};
    let mut ret = unsafe {_ZNK11QTextLength8rawValueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

