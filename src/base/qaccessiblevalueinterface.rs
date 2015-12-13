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
  // proto: QVariant QAccessibleValueInterface::maximumValue();
  fn _ZNK25QAccessibleValueInterface12maximumValueEv() -> i32;
  // proto: QVariant QAccessibleValueInterface::minimumStepSize();
  fn _ZNK25QAccessibleValueInterface15minimumStepSizeEv() -> i32;
  // proto: QVariant QAccessibleValueInterface::currentValue();
  fn _ZNK25QAccessibleValueInterface12currentValueEv() -> i32;
  // proto: QVariant QAccessibleValueInterface::minimumValue();
  fn _ZNK25QAccessibleValueInterface12minimumValueEv() -> i32;
  // proto: void QAccessibleValueInterface::FreeQAccessibleValueInterface();
  fn _ZN25QAccessibleValueInterfaceD0Ev() -> i32;
  // proto: void QAccessibleValueInterface::setCurrentValue(const QVariant & value);
  fn _ZN25QAccessibleValueInterface15setCurrentValueERK8QVariant(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QAccessibleValueInterface)=8
pub struct QAccessibleValueInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleValueInterface {
  pub fn maximumValue<T: QAccessibleValueInterface_maximumValue>(&mut self, value: T) -> i32 {
    value.maximumValue(self);
    return 1;
  }
}

pub trait QAccessibleValueInterface_maximumValue {
  fn maximumValue(self, this: &mut QAccessibleValueInterface) -> i32;
}

// proto: QVariant QAccessibleValueInterface::maximumValue();
impl<'a> /*trait*/ QAccessibleValueInterface_maximumValue for () {
  fn maximumValue(self, this: &mut QAccessibleValueInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface12maximumValueEv()};
    unsafe {_ZNK25QAccessibleValueInterface12maximumValueEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleValueInterface {
  pub fn minimumStepSize<T: QAccessibleValueInterface_minimumStepSize>(&mut self, value: T) -> i32 {
    value.minimumStepSize(self);
    return 1;
  }
}

pub trait QAccessibleValueInterface_minimumStepSize {
  fn minimumStepSize(self, this: &mut QAccessibleValueInterface) -> i32;
}

// proto: QVariant QAccessibleValueInterface::minimumStepSize();
impl<'a> /*trait*/ QAccessibleValueInterface_minimumStepSize for () {
  fn minimumStepSize(self, this: &mut QAccessibleValueInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface15minimumStepSizeEv()};
    unsafe {_ZNK25QAccessibleValueInterface15minimumStepSizeEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleValueInterface {
  pub fn currentValue<T: QAccessibleValueInterface_currentValue>(&mut self, value: T) -> i32 {
    value.currentValue(self);
    return 1;
  }
}

pub trait QAccessibleValueInterface_currentValue {
  fn currentValue(self, this: &mut QAccessibleValueInterface) -> i32;
}

// proto: QVariant QAccessibleValueInterface::currentValue();
impl<'a> /*trait*/ QAccessibleValueInterface_currentValue for () {
  fn currentValue(self, this: &mut QAccessibleValueInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface12currentValueEv()};
    unsafe {_ZNK25QAccessibleValueInterface12currentValueEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleValueInterface {
  pub fn minimumValue<T: QAccessibleValueInterface_minimumValue>(&mut self, value: T) -> i32 {
    value.minimumValue(self);
    return 1;
  }
}

pub trait QAccessibleValueInterface_minimumValue {
  fn minimumValue(self, this: &mut QAccessibleValueInterface) -> i32;
}

// proto: QVariant QAccessibleValueInterface::minimumValue();
impl<'a> /*trait*/ QAccessibleValueInterface_minimumValue for () {
  fn minimumValue(self, this: &mut QAccessibleValueInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface12minimumValueEv()};
    unsafe {_ZNK25QAccessibleValueInterface12minimumValueEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleValueInterface {
  pub fn FreeQAccessibleValueInterface<T: QAccessibleValueInterface_FreeQAccessibleValueInterface>(&mut self, value: T) -> i32 {
    value.FreeQAccessibleValueInterface(self);
    return 1;
  }
}

pub trait QAccessibleValueInterface_FreeQAccessibleValueInterface {
  fn FreeQAccessibleValueInterface(self, this: &mut QAccessibleValueInterface) -> i32;
}

// proto: void QAccessibleValueInterface::FreeQAccessibleValueInterface();
impl<'a> /*trait*/ QAccessibleValueInterface_FreeQAccessibleValueInterface for () {
  fn FreeQAccessibleValueInterface(self, this: &mut QAccessibleValueInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleValueInterfaceD0Ev()};
    unsafe {_ZN25QAccessibleValueInterfaceD0Ev()};
    return 1;
  }
}

impl /*struct*/ QAccessibleValueInterface {
  pub fn setCurrentValue<T: QAccessibleValueInterface_setCurrentValue>(&mut self, value: T) -> i32 {
    value.setCurrentValue(self);
    return 1;
  }
}

pub trait QAccessibleValueInterface_setCurrentValue {
  fn setCurrentValue(self, this: &mut QAccessibleValueInterface) -> i32;
}

// proto: void QAccessibleValueInterface::setCurrentValue(const QVariant & value);
impl<'a> /*trait*/ QAccessibleValueInterface_setCurrentValue for (&'a  QVariant) {
  fn setCurrentValue(self, this: &mut QAccessibleValueInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleValueInterface15setCurrentValueERK8QVariant()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN25QAccessibleValueInterface15setCurrentValueERK8QVariant(arg0)};
    return 1;
  }
}

