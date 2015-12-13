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
  // proto: void QScrollerProperties::NewQScrollerProperties(const QScrollerProperties & sp);
  fn _ZN19QScrollerPropertiesC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QScrollerProperties::setDefaultScrollerProperties(const QScrollerProperties & sp);
  fn _ZN19QScrollerProperties28setDefaultScrollerPropertiesERKS_(arg0: *const c_void) -> i32;
  // proto: void QScrollerProperties::FreeQScrollerProperties();
  fn _ZN19QScrollerPropertiesD0Ev() -> i32;
  // proto: void QScrollerProperties::unsetDefaultScrollerProperties();
  fn _ZN19QScrollerProperties30unsetDefaultScrollerPropertiesEv() -> i32;
  // proto: void QScrollerProperties::NewQScrollerProperties();
  fn _ZN19QScrollerPropertiesC1Ev(qthis: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QScrollerProperties)=1
pub struct QScrollerProperties {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScrollerProperties {
  pub fn NewQScrollerProperties<T: QScrollerProperties_NewQScrollerProperties>(value: T) -> QScrollerProperties {
    let rsthis = value.NewQScrollerProperties();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollerProperties_NewQScrollerProperties {
  fn NewQScrollerProperties(self) -> QScrollerProperties;
}

// proto: void QScrollerProperties::NewQScrollerProperties(const QScrollerProperties & sp);
impl<'a> /*trait*/ QScrollerProperties_NewQScrollerProperties for (&'a  QScrollerProperties) {
  fn NewQScrollerProperties(self) -> QScrollerProperties {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerPropertiesC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QScrollerPropertiesC1ERKS_(qthis, arg0)};
    let rsthis = QScrollerProperties{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QScrollerProperties {
  pub fn setDefaultScrollerProperties<T: QScrollerProperties_setDefaultScrollerProperties>(&mut self, value: T) -> i32 {
    value.setDefaultScrollerProperties(self);
    return 1;
  }
}

pub trait QScrollerProperties_setDefaultScrollerProperties {
  fn setDefaultScrollerProperties(self, this: &mut QScrollerProperties) -> i32;
}

// proto: void QScrollerProperties::setDefaultScrollerProperties(const QScrollerProperties & sp);
impl<'a> /*trait*/ QScrollerProperties_setDefaultScrollerProperties for (&'a  QScrollerProperties) {
  fn setDefaultScrollerProperties(self, this: &mut QScrollerProperties) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerProperties28setDefaultScrollerPropertiesERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QScrollerProperties28setDefaultScrollerPropertiesERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QScrollerProperties {
  pub fn FreeQScrollerProperties<T: QScrollerProperties_FreeQScrollerProperties>(&mut self, value: T) -> i32 {
    value.FreeQScrollerProperties(self);
    return 1;
  }
}

pub trait QScrollerProperties_FreeQScrollerProperties {
  fn FreeQScrollerProperties(self, this: &mut QScrollerProperties) -> i32;
}

// proto: void QScrollerProperties::FreeQScrollerProperties();
impl<'a> /*trait*/ QScrollerProperties_FreeQScrollerProperties for () {
  fn FreeQScrollerProperties(self, this: &mut QScrollerProperties) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerPropertiesD0Ev()};
    unsafe {_ZN19QScrollerPropertiesD0Ev()};
    return 1;
  }
}

impl /*struct*/ QScrollerProperties {
  pub fn unsetDefaultScrollerProperties<T: QScrollerProperties_unsetDefaultScrollerProperties>(&mut self, value: T) -> i32 {
    value.unsetDefaultScrollerProperties(self);
    return 1;
  }
}

pub trait QScrollerProperties_unsetDefaultScrollerProperties {
  fn unsetDefaultScrollerProperties(self, this: &mut QScrollerProperties) -> i32;
}

// proto: void QScrollerProperties::unsetDefaultScrollerProperties();
impl<'a> /*trait*/ QScrollerProperties_unsetDefaultScrollerProperties for () {
  fn unsetDefaultScrollerProperties(self, this: &mut QScrollerProperties) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerProperties30unsetDefaultScrollerPropertiesEv()};
    unsafe {_ZN19QScrollerProperties30unsetDefaultScrollerPropertiesEv()};
    return 1;
  }
}

// proto: void QScrollerProperties::NewQScrollerProperties();
impl<'a> /*trait*/ QScrollerProperties_NewQScrollerProperties for () {
  fn NewQScrollerProperties(self) -> QScrollerProperties {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerPropertiesC1Ev()};
    unsafe {_ZN19QScrollerPropertiesC1Ev(qthis)};
    let rsthis = QScrollerProperties{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

