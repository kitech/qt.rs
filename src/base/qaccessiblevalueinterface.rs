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

impl /*struct*/ QAccessibleValueInterface {
  pub fn maximumValue<T: QAccessibleValueInterface_maximumValue>(&mut self, value: T) -> QVariant {
    return value.maximumValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_maximumValue {
  fn maximumValue(self, rsthis: &mut QAccessibleValueInterface) -> QVariant;
}

// proto:  QVariant QAccessibleValueInterface::maximumValue();
impl<'a> /*trait*/ QAccessibleValueInterface_maximumValue for () {
  fn maximumValue(self, rsthis: &mut QAccessibleValueInterface) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface12maximumValueEv()};
    let mut ret = unsafe {_ZNK25QAccessibleValueInterface12maximumValueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleValueInterface {
  pub fn minimumStepSize<T: QAccessibleValueInterface_minimumStepSize>(&mut self, value: T) -> QVariant {
    return value.minimumStepSize(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_minimumStepSize {
  fn minimumStepSize(self, rsthis: &mut QAccessibleValueInterface) -> QVariant;
}

// proto:  QVariant QAccessibleValueInterface::minimumStepSize();
impl<'a> /*trait*/ QAccessibleValueInterface_minimumStepSize for () {
  fn minimumStepSize(self, rsthis: &mut QAccessibleValueInterface) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface15minimumStepSizeEv()};
    let mut ret = unsafe {_ZNK25QAccessibleValueInterface15minimumStepSizeEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleValueInterface {
  pub fn currentValue<T: QAccessibleValueInterface_currentValue>(&mut self, value: T) -> QVariant {
    return value.currentValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_currentValue {
  fn currentValue(self, rsthis: &mut QAccessibleValueInterface) -> QVariant;
}

// proto:  QVariant QAccessibleValueInterface::currentValue();
impl<'a> /*trait*/ QAccessibleValueInterface_currentValue for () {
  fn currentValue(self, rsthis: &mut QAccessibleValueInterface) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface12currentValueEv()};
    let mut ret = unsafe {_ZNK25QAccessibleValueInterface12currentValueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleValueInterface {
  pub fn minimumValue<T: QAccessibleValueInterface_minimumValue>(&mut self, value: T) -> QVariant {
    return value.minimumValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_minimumValue {
  fn minimumValue(self, rsthis: &mut QAccessibleValueInterface) -> QVariant;
}

// proto:  QVariant QAccessibleValueInterface::minimumValue();
impl<'a> /*trait*/ QAccessibleValueInterface_minimumValue for () {
  fn minimumValue(self, rsthis: &mut QAccessibleValueInterface) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface12minimumValueEv()};
    let mut ret = unsafe {_ZNK25QAccessibleValueInterface12minimumValueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleValueInterface {
  pub fn FreeQAccessibleValueInterface<T: QAccessibleValueInterface_FreeQAccessibleValueInterface>(&mut self, value: T)  {
     value.FreeQAccessibleValueInterface(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_FreeQAccessibleValueInterface {
  fn FreeQAccessibleValueInterface(self, rsthis: &mut QAccessibleValueInterface) ;
}

// proto:  void QAccessibleValueInterface::FreeQAccessibleValueInterface();
impl<'a> /*trait*/ QAccessibleValueInterface_FreeQAccessibleValueInterface for () {
  fn FreeQAccessibleValueInterface(self, rsthis: &mut QAccessibleValueInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleValueInterfaceD0Ev()};
     unsafe {_ZN25QAccessibleValueInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleValueInterface {
  pub fn setCurrentValue<T: QAccessibleValueInterface_setCurrentValue>(&mut self, value: T)  {
     value.setCurrentValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_setCurrentValue {
  fn setCurrentValue(self, rsthis: &mut QAccessibleValueInterface) ;
}

// proto:  void QAccessibleValueInterface::setCurrentValue(const QVariant & value);
impl<'a> /*trait*/ QAccessibleValueInterface_setCurrentValue for (&'a  QVariant) {
  fn setCurrentValue(self, rsthis: &mut QAccessibleValueInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleValueInterface15setCurrentValueERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QAccessibleValueInterface15setCurrentValueERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

