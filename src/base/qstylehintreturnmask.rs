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
  // proto:  void QStyleHintReturnMask::NewQStyleHintReturnMask();
  fn _ZN20QStyleHintReturnMaskC1Ev(qthis: *mut c_void) ;
  // proto:  void QStyleHintReturnMask::FreeQStyleHintReturnMask();
  fn _ZN20QStyleHintReturnMaskD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QStyleHintReturnMask)=16
pub struct QStyleHintReturnMask {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleHintReturnMask {
  pub fn NewQStyleHintReturnMask<T: QStyleHintReturnMask_NewQStyleHintReturnMask>(value: T) -> QStyleHintReturnMask {
    let rsthis = value.NewQStyleHintReturnMask();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturnMask_NewQStyleHintReturnMask {
  fn NewQStyleHintReturnMask(self) -> QStyleHintReturnMask;
}

// proto: void QStyleHintReturnMask::NewQStyleHintReturnMask();
impl<'a> /*trait*/ QStyleHintReturnMask_NewQStyleHintReturnMask for () {
  fn NewQStyleHintReturnMask(self) -> QStyleHintReturnMask {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleHintReturnMaskC1Ev()};
    unsafe {_ZN20QStyleHintReturnMaskC1Ev(qthis)};
    let rsthis = QStyleHintReturnMask{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QStyleHintReturnMask::FreeQStyleHintReturnMask();
impl /*struct*/ QStyleHintReturnMask {
  pub fn FreeQStyleHintReturnMask<RetType, T: QStyleHintReturnMask_FreeQStyleHintReturnMask<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQStyleHintReturnMask(self);
    // return 1;
  }
}

pub trait QStyleHintReturnMask_FreeQStyleHintReturnMask<RetType> {
  fn FreeQStyleHintReturnMask(self , rsthis: &mut QStyleHintReturnMask) -> RetType;
}

// proto:  void QStyleHintReturnMask::FreeQStyleHintReturnMask();
impl<'a> /*trait*/ QStyleHintReturnMask_FreeQStyleHintReturnMask<()> for () {
  fn FreeQStyleHintReturnMask(self , rsthis: &mut QStyleHintReturnMask) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleHintReturnMaskD0Ev()};
     unsafe {_ZN20QStyleHintReturnMaskD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

