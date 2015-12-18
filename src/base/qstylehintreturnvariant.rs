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
  // proto:  void QStyleHintReturnVariant::FreeQStyleHintReturnVariant();
  fn _ZN23QStyleHintReturnVariantD0Ev(qthis: *mut c_void) ;
  // proto:  void QStyleHintReturnVariant::NewQStyleHintReturnVariant();
  fn _ZN23QStyleHintReturnVariantC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QStyleHintReturnVariant)=24
pub struct QStyleHintReturnVariant {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleHintReturnVariant {
  pub fn FreeQStyleHintReturnVariant<RetType, T: QStyleHintReturnVariant_FreeQStyleHintReturnVariant<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQStyleHintReturnVariant(self);
    // return 1;
  }
}

pub trait QStyleHintReturnVariant_FreeQStyleHintReturnVariant<RetType> {
  fn FreeQStyleHintReturnVariant(self, rsthis: &mut QStyleHintReturnVariant) -> RetType;
}

// proto:  void QStyleHintReturnVariant::FreeQStyleHintReturnVariant();
impl<'a> /*trait*/ QStyleHintReturnVariant_FreeQStyleHintReturnVariant<()> for () {
  fn FreeQStyleHintReturnVariant(self, rsthis: &mut QStyleHintReturnVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleHintReturnVariantD0Ev()};
     unsafe {_ZN23QStyleHintReturnVariantD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStyleHintReturnVariant {
  pub fn NewQStyleHintReturnVariant<T: QStyleHintReturnVariant_NewQStyleHintReturnVariant>(value: T) -> QStyleHintReturnVariant {
    let rsthis = value.NewQStyleHintReturnVariant();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturnVariant_NewQStyleHintReturnVariant {
  fn NewQStyleHintReturnVariant(self) -> QStyleHintReturnVariant;
}

// proto: void QStyleHintReturnVariant::NewQStyleHintReturnVariant();
impl<'a> /*trait*/ QStyleHintReturnVariant_NewQStyleHintReturnVariant for () {
  fn NewQStyleHintReturnVariant(self) -> QStyleHintReturnVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleHintReturnVariantC1Ev()};
    unsafe {_ZN23QStyleHintReturnVariantC1Ev(qthis)};
    let rsthis = QStyleHintReturnVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

