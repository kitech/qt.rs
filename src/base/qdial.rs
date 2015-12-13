// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QDial::wrapping();
  fn _ZNK5QDial8wrappingEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QDial::FreeQDial();
  fn _ZN5QDialD0Ev(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QDial::metaObject();
  fn _ZNK5QDial10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QDial::notchesVisible();
  fn _ZNK5QDial14notchesVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QDial::setNotchTarget(double target);
  fn _ZN5QDial14setNotchTargetEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QDial::setWrapping(bool on);
  fn _ZN5QDial11setWrappingEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QDial::notchSize();
  fn _ZNK5QDial9notchSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDial::setNotchesVisible(bool visible);
  fn _ZN5QDial17setNotchesVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QSize QDial::minimumSizeHint();
  fn _ZNK5QDial15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDial::NewQDial(const QDial & );
  fn _ZN5QDialC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QDial::notchTarget();
  fn _ZNK5QDial11notchTargetEv(qthis: *mut c_void) -> c_double;
  // proto:  QSize QDial::sizeHint();
  fn _ZNK5QDial8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDial::NewQDial(QWidget * parent);
  fn _ZN5QDialC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QDial)=1
pub struct QDial {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDial {
  pub fn wrapping<T: QDial_wrapping>(&mut self, value: T) -> i8 {
    return value.wrapping(self);
    // return 1;
  }
}

pub trait QDial_wrapping {
  fn wrapping(self, rsthis: &mut QDial) -> i8;
}

// proto:  bool QDial::wrapping();
impl<'a> /*trait*/ QDial_wrapping for () {
  fn wrapping(self, rsthis: &mut QDial) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial8wrappingEv()};
    let mut ret = unsafe {_ZNK5QDial8wrappingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDial {
  pub fn FreeQDial<T: QDial_FreeQDial>(&mut self, value: T)  {
     value.FreeQDial(self);
    // return 1;
  }
}

pub trait QDial_FreeQDial {
  fn FreeQDial(self, rsthis: &mut QDial) ;
}

// proto:  void QDial::FreeQDial();
impl<'a> /*trait*/ QDial_FreeQDial for () {
  fn FreeQDial(self, rsthis: &mut QDial)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDialD0Ev()};
     unsafe {_ZN5QDialD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDial {
  pub fn metaObject<T: QDial_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QDial_metaObject {
  fn metaObject(self, rsthis: &mut QDial) ;
}

// proto:  const QMetaObject * QDial::metaObject();
impl<'a> /*trait*/ QDial_metaObject for () {
  fn metaObject(self, rsthis: &mut QDial)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial10metaObjectEv()};
     unsafe {_ZNK5QDial10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDial {
  pub fn notchesVisible<T: QDial_notchesVisible>(&mut self, value: T) -> i8 {
    return value.notchesVisible(self);
    // return 1;
  }
}

pub trait QDial_notchesVisible {
  fn notchesVisible(self, rsthis: &mut QDial) -> i8;
}

// proto:  bool QDial::notchesVisible();
impl<'a> /*trait*/ QDial_notchesVisible for () {
  fn notchesVisible(self, rsthis: &mut QDial) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial14notchesVisibleEv()};
    let mut ret = unsafe {_ZNK5QDial14notchesVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDial {
  pub fn setNotchTarget<T: QDial_setNotchTarget>(&mut self, value: T)  {
     value.setNotchTarget(self);
    // return 1;
  }
}

pub trait QDial_setNotchTarget {
  fn setNotchTarget(self, rsthis: &mut QDial) ;
}

// proto:  void QDial::setNotchTarget(double target);
impl<'a> /*trait*/ QDial_setNotchTarget for (f64) {
  fn setNotchTarget(self, rsthis: &mut QDial)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDial14setNotchTargetEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN5QDial14setNotchTargetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDial {
  pub fn setWrapping<T: QDial_setWrapping>(&mut self, value: T)  {
     value.setWrapping(self);
    // return 1;
  }
}

pub trait QDial_setWrapping {
  fn setWrapping(self, rsthis: &mut QDial) ;
}

// proto:  void QDial::setWrapping(bool on);
impl<'a> /*trait*/ QDial_setWrapping for (i8) {
  fn setWrapping(self, rsthis: &mut QDial)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDial11setWrappingEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QDial11setWrappingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDial {
  pub fn notchSize<T: QDial_notchSize>(&mut self, value: T) -> i32 {
    return value.notchSize(self);
    // return 1;
  }
}

pub trait QDial_notchSize {
  fn notchSize(self, rsthis: &mut QDial) -> i32;
}

// proto:  int QDial::notchSize();
impl<'a> /*trait*/ QDial_notchSize for () {
  fn notchSize(self, rsthis: &mut QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial9notchSizeEv()};
    let mut ret = unsafe {_ZNK5QDial9notchSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDial {
  pub fn setNotchesVisible<T: QDial_setNotchesVisible>(&mut self, value: T)  {
     value.setNotchesVisible(self);
    // return 1;
  }
}

pub trait QDial_setNotchesVisible {
  fn setNotchesVisible(self, rsthis: &mut QDial) ;
}

// proto:  void QDial::setNotchesVisible(bool visible);
impl<'a> /*trait*/ QDial_setNotchesVisible for (i8) {
  fn setNotchesVisible(self, rsthis: &mut QDial)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDial17setNotchesVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QDial17setNotchesVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDial {
  pub fn minimumSizeHint<T: QDial_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QDial_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QDial) -> QSize;
}

// proto:  QSize QDial::minimumSizeHint();
impl<'a> /*trait*/ QDial_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QDial) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK5QDial15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QDialC1ERKS_(qthis, arg0)};
    let rsthis = QDial{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDial {
  pub fn notchTarget<T: QDial_notchTarget>(&mut self, value: T) -> f64 {
    return value.notchTarget(self);
    // return 1;
  }
}

pub trait QDial_notchTarget {
  fn notchTarget(self, rsthis: &mut QDial) -> f64;
}

// proto:  double QDial::notchTarget();
impl<'a> /*trait*/ QDial_notchTarget for () {
  fn notchTarget(self, rsthis: &mut QDial) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial11notchTargetEv()};
    let mut ret = unsafe {_ZNK5QDial11notchTargetEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QDial {
  pub fn sizeHint<T: QDial_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QDial_sizeHint {
  fn sizeHint(self, rsthis: &mut QDial) -> QSize;
}

// proto:  QSize QDial::sizeHint();
impl<'a> /*trait*/ QDial_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QDial) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial8sizeHintEv()};
    let mut ret = unsafe {_ZNK5QDial8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
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

