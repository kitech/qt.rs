// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QDial::wrapping();
  fn _ZNK5QDial8wrappingEv() -> i32;
  // proto: void QDial::FreeQDial();
  fn _ZN5QDialD0Ev() -> i32;
  // proto: const QMetaObject * QDial::metaObject();
  fn _ZNK5QDial10metaObjectEv() -> i32;
  // proto: bool QDial::notchesVisible();
  fn _ZNK5QDial14notchesVisibleEv() -> i32;
  // proto: void QDial::setNotchTarget(double target);
  fn _ZN5QDial14setNotchTargetEd(arg0: c_double) -> i32;
  // proto: void QDial::setWrapping(bool on);
  fn _ZN5QDial11setWrappingEb(arg0: int8_t) -> i32;
  // proto: int QDial::notchSize();
  fn _ZNK5QDial9notchSizeEv() -> i32;
  // proto: void QDial::setNotchesVisible(bool visible);
  fn _ZN5QDial17setNotchesVisibleEb(arg0: int8_t) -> i32;
  // proto: QSize QDial::minimumSizeHint();
  fn _ZNK5QDial15minimumSizeHintEv() -> i32;
  // proto: void QDial::NewQDial(const QDial & );
  fn _ZN5QDialC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: double QDial::notchTarget();
  fn _ZNK5QDial11notchTargetEv() -> i32;
  // proto: QSize QDial::sizeHint();
  fn _ZNK5QDial8sizeHintEv() -> i32;
  // proto: void QDial::NewQDial(QWidget * parent);
  fn _ZN5QDialC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QDial)=1
pub struct QDial {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDial {
  pub fn wrapping<T: QDial_wrapping>(&mut self, value: T) -> i32 {
    value.wrapping(self);
    return 1;
  }
}

pub trait QDial_wrapping {
  fn wrapping(self, this: &mut QDial) -> i32;
}

// proto: bool QDial::wrapping();
impl<'a> /*trait*/ QDial_wrapping for () {
  fn wrapping(self, this: &mut QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial8wrappingEv()};
    unsafe {_ZNK5QDial8wrappingEv()};
    return 1;
  }
}

impl /*struct*/ QDial {
  pub fn FreeQDial<T: QDial_FreeQDial>(&mut self, value: T) -> i32 {
    value.FreeQDial(self);
    return 1;
  }
}

pub trait QDial_FreeQDial {
  fn FreeQDial(self, this: &mut QDial) -> i32;
}

// proto: void QDial::FreeQDial();
impl<'a> /*trait*/ QDial_FreeQDial for () {
  fn FreeQDial(self, this: &mut QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDialD0Ev()};
    unsafe {_ZN5QDialD0Ev()};
    return 1;
  }
}

impl /*struct*/ QDial {
  pub fn metaObject<T: QDial_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QDial_metaObject {
  fn metaObject(self, this: &mut QDial) -> i32;
}

// proto: const QMetaObject * QDial::metaObject();
impl<'a> /*trait*/ QDial_metaObject for () {
  fn metaObject(self, this: &mut QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial10metaObjectEv()};
    unsafe {_ZNK5QDial10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QDial {
  pub fn notchesVisible<T: QDial_notchesVisible>(&mut self, value: T) -> i32 {
    value.notchesVisible(self);
    return 1;
  }
}

pub trait QDial_notchesVisible {
  fn notchesVisible(self, this: &mut QDial) -> i32;
}

// proto: bool QDial::notchesVisible();
impl<'a> /*trait*/ QDial_notchesVisible for () {
  fn notchesVisible(self, this: &mut QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial14notchesVisibleEv()};
    unsafe {_ZNK5QDial14notchesVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QDial {
  pub fn setNotchTarget<T: QDial_setNotchTarget>(&mut self, value: T) -> i32 {
    value.setNotchTarget(self);
    return 1;
  }
}

pub trait QDial_setNotchTarget {
  fn setNotchTarget(self, this: &mut QDial) -> i32;
}

// proto: void QDial::setNotchTarget(double target);
impl<'a> /*trait*/ QDial_setNotchTarget for (f64) {
  fn setNotchTarget(self, this: &mut QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDial14setNotchTargetEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN5QDial14setNotchTargetEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QDial {
  pub fn setWrapping<T: QDial_setWrapping>(&mut self, value: T) -> i32 {
    value.setWrapping(self);
    return 1;
  }
}

pub trait QDial_setWrapping {
  fn setWrapping(self, this: &mut QDial) -> i32;
}

// proto: void QDial::setWrapping(bool on);
impl<'a> /*trait*/ QDial_setWrapping for (i8) {
  fn setWrapping(self, this: &mut QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDial11setWrappingEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN5QDial11setWrappingEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QDial {
  pub fn notchSize<T: QDial_notchSize>(&mut self, value: T) -> i32 {
    value.notchSize(self);
    return 1;
  }
}

pub trait QDial_notchSize {
  fn notchSize(self, this: &mut QDial) -> i32;
}

// proto: int QDial::notchSize();
impl<'a> /*trait*/ QDial_notchSize for () {
  fn notchSize(self, this: &mut QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial9notchSizeEv()};
    unsafe {_ZNK5QDial9notchSizeEv()};
    return 1;
  }
}

impl /*struct*/ QDial {
  pub fn setNotchesVisible<T: QDial_setNotchesVisible>(&mut self, value: T) -> i32 {
    value.setNotchesVisible(self);
    return 1;
  }
}

pub trait QDial_setNotchesVisible {
  fn setNotchesVisible(self, this: &mut QDial) -> i32;
}

// proto: void QDial::setNotchesVisible(bool visible);
impl<'a> /*trait*/ QDial_setNotchesVisible for (i8) {
  fn setNotchesVisible(self, this: &mut QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDial17setNotchesVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN5QDial17setNotchesVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QDial {
  pub fn minimumSizeHint<T: QDial_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QDial_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QDial) -> i32;
}

// proto: QSize QDial::minimumSizeHint();
impl<'a> /*trait*/ QDial_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial15minimumSizeHintEv()};
    unsafe {_ZNK5QDial15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QDial {
  pub fn NewQDial<T: QDial_NewQDial>(value: T) -> QDial {
    let rsthis = value.NewQDial();
    return rsthis;
    // return 1;
  }
}

pub trait QDial_NewQDial {
  fn NewQDial(self) -> QDial;
}

// proto: void QDial::NewQDial(const QDial & );
impl<'a> /*trait*/ QDial_NewQDial for (&'a  QDial) {
  fn NewQDial(self) -> QDial {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDialC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QDialC1ERKS_(qthis, arg0)};
    let rsthis = QDial{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDial {
  pub fn notchTarget<T: QDial_notchTarget>(&mut self, value: T) -> i32 {
    value.notchTarget(self);
    return 1;
  }
}

pub trait QDial_notchTarget {
  fn notchTarget(self, this: &mut QDial) -> i32;
}

// proto: double QDial::notchTarget();
impl<'a> /*trait*/ QDial_notchTarget for () {
  fn notchTarget(self, this: &mut QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial11notchTargetEv()};
    unsafe {_ZNK5QDial11notchTargetEv()};
    return 1;
  }
}

impl /*struct*/ QDial {
  pub fn sizeHint<T: QDial_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QDial_sizeHint {
  fn sizeHint(self, this: &mut QDial) -> i32;
}

// proto: QSize QDial::sizeHint();
impl<'a> /*trait*/ QDial_sizeHint for () {
  fn sizeHint(self, this: &mut QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial8sizeHintEv()};
    unsafe {_ZNK5QDial8sizeHintEv()};
    return 1;
  }
}

// proto: void QDial::NewQDial(QWidget * parent);
impl<'a> /*trait*/ QDial_NewQDial for (&'a mut QWidget) {
  fn NewQDial(self) -> QDial {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDialC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QDialC1EP7QWidget(qthis, arg0)};
    let rsthis = QDial{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

