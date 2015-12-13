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
  // proto: void QStyleHintReturn::FreeQStyleHintReturn();
  fn _ZN16QStyleHintReturnD0Ev() -> i32;
  // proto: void QStyleHintReturn::NewQStyleHintReturn(int version, int type);
  fn _ZN16QStyleHintReturnC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QStyleHintReturn)=8
pub struct QStyleHintReturn {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleHintReturn {
  pub fn FreeQStyleHintReturn<T: QStyleHintReturn_FreeQStyleHintReturn>(&mut self, value: T) -> i32 {
    value.FreeQStyleHintReturn(self);
    return 1;
  }
}

pub trait QStyleHintReturn_FreeQStyleHintReturn {
  fn FreeQStyleHintReturn(self, this: &mut QStyleHintReturn) -> i32;
}

// proto: void QStyleHintReturn::FreeQStyleHintReturn();
impl<'a> /*trait*/ QStyleHintReturn_FreeQStyleHintReturn for () {
  fn FreeQStyleHintReturn(self, this: &mut QStyleHintReturn) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStyleHintReturnD0Ev()};
    unsafe {_ZN16QStyleHintReturnD0Ev()};
    return 1;
  }
}

impl /*struct*/ QStyleHintReturn {
  pub fn NewQStyleHintReturn<T: QStyleHintReturn_NewQStyleHintReturn>(value: T) -> QStyleHintReturn {
    let rsthis = value.NewQStyleHintReturn();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturn_NewQStyleHintReturn {
  fn NewQStyleHintReturn(self) -> QStyleHintReturn;
}

// proto: void QStyleHintReturn::NewQStyleHintReturn(int version, int type);
impl<'a> /*trait*/ QStyleHintReturn_NewQStyleHintReturn for (i32, i32) {
  fn NewQStyleHintReturn(self) -> QStyleHintReturn {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStyleHintReturnC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN16QStyleHintReturnC1Eii(qthis, arg0, arg1)};
    let rsthis = QStyleHintReturn{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

