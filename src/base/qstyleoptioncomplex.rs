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
  // proto: void QStyleOptionComplex::NewQStyleOptionComplex(int version, int type);
  fn _ZN19QStyleOptionComplexC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> i32;
  // proto: void QStyleOptionComplex::NewQStyleOptionComplex(const QStyleOptionComplex & other);
  fn _ZN19QStyleOptionComplexC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QStyleOptionComplex)=1
pub struct QStyleOptionComplex {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleOptionComplex {
  pub fn NewQStyleOptionComplex<T: QStyleOptionComplex_NewQStyleOptionComplex>(value: T) -> QStyleOptionComplex {
    let rsthis = value.NewQStyleOptionComplex();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionComplex_NewQStyleOptionComplex {
  fn NewQStyleOptionComplex(self) -> QStyleOptionComplex;
}

// proto: void QStyleOptionComplex::NewQStyleOptionComplex(int version, int type);
impl<'a> /*trait*/ QStyleOptionComplex_NewQStyleOptionComplex for (i32, i32) {
  fn NewQStyleOptionComplex(self) -> QStyleOptionComplex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionComplexC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN19QStyleOptionComplexC1Eii(qthis, arg0, arg1)};
    let rsthis = QStyleOptionComplex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionComplex::NewQStyleOptionComplex(const QStyleOptionComplex & other);
impl<'a> /*trait*/ QStyleOptionComplex_NewQStyleOptionComplex for (&'a  QStyleOptionComplex) {
  fn NewQStyleOptionComplex(self) -> QStyleOptionComplex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionComplexC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QStyleOptionComplexC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionComplex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

