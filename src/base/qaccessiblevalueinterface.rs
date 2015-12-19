// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QVariant QAccessibleValueInterface::maximumValue();
  fn _ZNK25QAccessibleValueInterface12maximumValueEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QAccessibleValueInterface::minimumStepSize();
  fn _ZNK25QAccessibleValueInterface15minimumStepSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QAccessibleValueInterface::currentValue();
  fn _ZNK25QAccessibleValueInterface12currentValueEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QAccessibleValueInterface::minimumValue();
  fn _ZNK25QAccessibleValueInterface12minimumValueEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleValueInterface::FreeQAccessibleValueInterface();
  fn _ZN25QAccessibleValueInterfaceD0Ev(qthis: *mut c_void) ;
  // proto:  void QAccessibleValueInterface::setCurrentValue(const QVariant & value);
  fn _ZN25QAccessibleValueInterface15setCurrentValueERK8QVariant(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QAccessibleValueInterface)=8
pub struct QAccessibleValueInterface {
  pub qclsinst: *mut c_void,
}

// proto:  QVariant QAccessibleValueInterface::maximumValue();
impl /*struct*/ QAccessibleValueInterface {
  pub fn maximumValue<RetType, T: QAccessibleValueInterface_maximumValue<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.maximumValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_maximumValue<RetType> {
  fn maximumValue(self , rsthis: &mut QAccessibleValueInterface) -> RetType;
}

// proto:  QVariant QAccessibleValueInterface::maximumValue();
impl<'a> /*trait*/ QAccessibleValueInterface_maximumValue<QVariant> for () {
  fn maximumValue(self , rsthis: &mut QAccessibleValueInterface) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface12maximumValueEv()};
    let mut ret = unsafe {_ZNK25QAccessibleValueInterface12maximumValueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QVariant QAccessibleValueInterface::minimumStepSize();
impl /*struct*/ QAccessibleValueInterface {
  pub fn minimumStepSize<RetType, T: QAccessibleValueInterface_minimumStepSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.minimumStepSize(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_minimumStepSize<RetType> {
  fn minimumStepSize(self , rsthis: &mut QAccessibleValueInterface) -> RetType;
}

// proto:  QVariant QAccessibleValueInterface::minimumStepSize();
impl<'a> /*trait*/ QAccessibleValueInterface_minimumStepSize<QVariant> for () {
  fn minimumStepSize(self , rsthis: &mut QAccessibleValueInterface) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface15minimumStepSizeEv()};
    let mut ret = unsafe {_ZNK25QAccessibleValueInterface15minimumStepSizeEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QVariant QAccessibleValueInterface::currentValue();
impl /*struct*/ QAccessibleValueInterface {
  pub fn currentValue<RetType, T: QAccessibleValueInterface_currentValue<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.currentValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_currentValue<RetType> {
  fn currentValue(self , rsthis: &mut QAccessibleValueInterface) -> RetType;
}

// proto:  QVariant QAccessibleValueInterface::currentValue();
impl<'a> /*trait*/ QAccessibleValueInterface_currentValue<QVariant> for () {
  fn currentValue(self , rsthis: &mut QAccessibleValueInterface) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface12currentValueEv()};
    let mut ret = unsafe {_ZNK25QAccessibleValueInterface12currentValueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QVariant QAccessibleValueInterface::minimumValue();
impl /*struct*/ QAccessibleValueInterface {
  pub fn minimumValue<RetType, T: QAccessibleValueInterface_minimumValue<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.minimumValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_minimumValue<RetType> {
  fn minimumValue(self , rsthis: &mut QAccessibleValueInterface) -> RetType;
}

// proto:  QVariant QAccessibleValueInterface::minimumValue();
impl<'a> /*trait*/ QAccessibleValueInterface_minimumValue<QVariant> for () {
  fn minimumValue(self , rsthis: &mut QAccessibleValueInterface) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface12minimumValueEv()};
    let mut ret = unsafe {_ZNK25QAccessibleValueInterface12minimumValueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QAccessibleValueInterface::FreeQAccessibleValueInterface();
impl /*struct*/ QAccessibleValueInterface {
  pub fn FreeQAccessibleValueInterface<RetType, T: QAccessibleValueInterface_FreeQAccessibleValueInterface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleValueInterface(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_FreeQAccessibleValueInterface<RetType> {
  fn FreeQAccessibleValueInterface(self , rsthis: &mut QAccessibleValueInterface) -> RetType;
}

// proto:  void QAccessibleValueInterface::FreeQAccessibleValueInterface();
impl<'a> /*trait*/ QAccessibleValueInterface_FreeQAccessibleValueInterface<()> for () {
  fn FreeQAccessibleValueInterface(self , rsthis: &mut QAccessibleValueInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleValueInterfaceD0Ev()};
     unsafe {_ZN25QAccessibleValueInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QAccessibleValueInterface::setCurrentValue(const QVariant & value);
impl /*struct*/ QAccessibleValueInterface {
  pub fn setCurrentValue<RetType, T: QAccessibleValueInterface_setCurrentValue<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setCurrentValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_setCurrentValue<RetType> {
  fn setCurrentValue(self , rsthis: &mut QAccessibleValueInterface) -> RetType;
}

// proto:  void QAccessibleValueInterface::setCurrentValue(const QVariant & value);
impl<'a> /*trait*/ QAccessibleValueInterface_setCurrentValue<()> for (&'a  QVariant) {
  fn setCurrentValue(self , rsthis: &mut QAccessibleValueInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleValueInterface15setCurrentValueERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QAccessibleValueInterface15setCurrentValueERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

