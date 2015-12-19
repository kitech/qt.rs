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
  // proto:  void QScrollerProperties::NewQScrollerProperties(const QScrollerProperties & sp);
  fn _ZN19QScrollerPropertiesC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static void QScrollerProperties::setDefaultScrollerProperties(const QScrollerProperties & sp);
  fn _ZN19QScrollerProperties28setDefaultScrollerPropertiesERKS_(arg0: *mut c_void) ;
  // proto:  void QScrollerProperties::FreeQScrollerProperties();
  fn _ZN19QScrollerPropertiesD0Ev(qthis: *mut c_void) ;
  // proto: static void QScrollerProperties::unsetDefaultScrollerProperties();
  fn _ZN19QScrollerProperties30unsetDefaultScrollerPropertiesEv() ;
  // proto:  void QScrollerProperties::NewQScrollerProperties();
  fn _ZN19QScrollerPropertiesC1Ev(qthis: *mut c_void) ;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QScrollerPropertiesC1ERKS_(qthis, arg0)};
    let rsthis = QScrollerProperties{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: static void QScrollerProperties::setDefaultScrollerProperties(const QScrollerProperties & sp);
impl /*struct*/ QScrollerProperties {
  pub fn setDefaultScrollerProperties_s<RetType, T: QScrollerProperties_setDefaultScrollerProperties_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefaultScrollerProperties_s();
    // return 1;
  }
}

pub trait QScrollerProperties_setDefaultScrollerProperties_s<RetType> {
  fn setDefaultScrollerProperties_s(self ) -> RetType;
}

// proto: static void QScrollerProperties::setDefaultScrollerProperties(const QScrollerProperties & sp);
impl<'a> /*trait*/ QScrollerProperties_setDefaultScrollerProperties_s<()> for (&'a  QScrollerProperties) {
  fn setDefaultScrollerProperties_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerProperties28setDefaultScrollerPropertiesERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QScrollerProperties28setDefaultScrollerPropertiesERKS_(arg0)};
    // return 1;
  }
}

// proto:  void QScrollerProperties::FreeQScrollerProperties();
impl /*struct*/ QScrollerProperties {
  pub fn FreeQScrollerProperties<RetType, T: QScrollerProperties_FreeQScrollerProperties<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQScrollerProperties(self);
    // return 1;
  }
}

pub trait QScrollerProperties_FreeQScrollerProperties<RetType> {
  fn FreeQScrollerProperties(self , rsthis: &mut QScrollerProperties) -> RetType;
}

// proto:  void QScrollerProperties::FreeQScrollerProperties();
impl<'a> /*trait*/ QScrollerProperties_FreeQScrollerProperties<()> for () {
  fn FreeQScrollerProperties(self , rsthis: &mut QScrollerProperties) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerPropertiesD0Ev()};
     unsafe {_ZN19QScrollerPropertiesD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: static void QScrollerProperties::unsetDefaultScrollerProperties();
impl /*struct*/ QScrollerProperties {
  pub fn unsetDefaultScrollerProperties_s<RetType, T: QScrollerProperties_unsetDefaultScrollerProperties_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.unsetDefaultScrollerProperties_s();
    // return 1;
  }
}

pub trait QScrollerProperties_unsetDefaultScrollerProperties_s<RetType> {
  fn unsetDefaultScrollerProperties_s(self ) -> RetType;
}

// proto: static void QScrollerProperties::unsetDefaultScrollerProperties();
impl<'a> /*trait*/ QScrollerProperties_unsetDefaultScrollerProperties_s<()> for () {
  fn unsetDefaultScrollerProperties_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerProperties30unsetDefaultScrollerPropertiesEv()};
     unsafe {_ZN19QScrollerProperties30unsetDefaultScrollerPropertiesEv()};
    // return 1;
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

