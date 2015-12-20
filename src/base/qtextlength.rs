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
  // proto:  qreal QTextLength::value(qreal maximumLength);
  fn _ZNK11QTextLength5valueEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QTextLength::QTextLength();
  fn _ZN11QTextLengthC1Ev(qthis: *mut c_void);
  // proto:  qreal QTextLength::rawValue();
  fn _ZNK11QTextLength8rawValueEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QTextLength)=16
pub struct QTextLength {
  pub qclsinst: *mut c_void,
}

  // proto:  qreal QTextLength::value(qreal maximumLength);
impl /*struct*/ QTextLength {
  pub fn value<RetType, T: QTextLength_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QTextLength_value<RetType> {
  fn value(self , rsthis: &mut QTextLength) -> RetType;
}

  // proto:  qreal QTextLength::value(qreal maximumLength);
impl<'a> /*trait*/ QTextLength_value<f64> for (f64) {
  fn value(self , rsthis: &mut QTextLength) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLength5valueEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK11QTextLength5valueEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextLength::QTextLength();
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

  // proto:  void QTextLength::QTextLength();
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

  // proto:  qreal QTextLength::rawValue();
impl /*struct*/ QTextLength {
  pub fn rawValue<RetType, T: QTextLength_rawValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rawValue(self);
    // return 1;
  }
}

pub trait QTextLength_rawValue<RetType> {
  fn rawValue(self , rsthis: &mut QTextLength) -> RetType;
}

  // proto:  qreal QTextLength::rawValue();
impl<'a> /*trait*/ QTextLength_rawValue<f64> for () {
  fn rawValue(self , rsthis: &mut QTextLength) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLength8rawValueEv()};
    let mut ret = unsafe {_ZNK11QTextLength8rawValueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

