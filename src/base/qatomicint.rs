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
  // proto:  void QAtomicInt::NewQAtomicInt(int value);
  fn _ZN10QAtomicIntC1Ei(qthis: *mut c_void, arg0: c_int) ;
}

// body block begin
// class sizeof(QAtomicInt)=1
pub struct QAtomicInt {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAtomicInt {
  pub fn NewQAtomicInt<T: QAtomicInt_NewQAtomicInt>(value: T) -> QAtomicInt {
    let rsthis = value.NewQAtomicInt();
    return rsthis;
    // return 1;
  }
}

pub trait QAtomicInt_NewQAtomicInt {
  fn NewQAtomicInt(self) -> QAtomicInt;
}

// proto: void QAtomicInt::NewQAtomicInt(int value);
impl<'a> /*trait*/ QAtomicInt_NewQAtomicInt for (i32) {
  fn NewQAtomicInt(self) -> QAtomicInt {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QAtomicIntC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QAtomicIntC1Ei(qthis, arg0)};
    let rsthis = QAtomicInt{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

