// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtWidgets/qdial.h
// dst-file: /src/widgets/qdial.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qabstractslider::QAbstractSlider; // 773
use std::ops::Deref;
use super::super::core::qsize::QSize; // 771
use super::qwidget::QWidget; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  bool QDial::wrapping();
  fn _ZNK5QDial8wrappingEv(qthis: *mut c_void) -> c_char;
  // proto:  void QDial::~QDial();
  fn _ZN5QDialD0Ev(qthis: *mut c_void);
  // proto:  const QMetaObject * QDial::metaObject();
  fn _ZNK5QDial10metaObjectEv(qthis: *mut c_void);
  // proto:  bool QDial::notchesVisible();
  fn _ZNK5QDial14notchesVisibleEv(qthis: *mut c_void) -> c_char;
  // proto:  void QDial::setNotchTarget(double target);
  fn _ZN5QDial14setNotchTargetEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QDial::setWrapping(bool on);
  fn _ZN5QDial11setWrappingEb(qthis: *mut c_void, arg0: c_char);
  // proto:  int QDial::notchSize();
  fn _ZNK5QDial9notchSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDial::setNotchesVisible(bool visible);
  fn _ZN5QDial17setNotchesVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QSize QDial::minimumSizeHint();
  fn _ZNK5QDial15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDial::QDial(const QDial & );
  fn _ZN5QDialC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QDial::notchTarget();
  fn _ZNK5QDial11notchTargetEv(qthis: *mut c_void) -> c_double;
  // proto:  QSize QDial::sizeHint();
  fn _ZNK5QDial8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDial::QDial(QWidget * parent);
  fn _ZN5QDialC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QDial)=1
pub struct QDial {
  qbase: QAbstractSlider,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDial {
  pub fn inheritFrom(qthis: *mut c_void) -> QDial {
    return QDial{qbase: QAbstractSlider::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QDial {
  type Target = QAbstractSlider;

  fn deref(&self) -> &QAbstractSlider {
    return &self.qbase;
  }
}
impl AsRef<QAbstractSlider> for QDial {
  fn as_ref(&self) -> &QAbstractSlider {
    return &self.qbase;
  }
}
  // proto:  bool QDial::wrapping();
impl /*struct*/ QDial {
  pub fn wrapping<RetType, T: QDial_wrapping<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.wrapping(self);
    // return 1;
  }
}

pub trait QDial_wrapping<RetType> {
  fn wrapping(self , rsthis: &mut QDial) -> RetType;
}

  // proto:  bool QDial::wrapping();
impl<'a> /*trait*/ QDial_wrapping<i8> for () {
  fn wrapping(self , rsthis: &mut QDial) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial8wrappingEv()};
    let mut ret = unsafe {_ZNK5QDial8wrappingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QDial::~QDial();
impl /*struct*/ QDial {
  pub fn FreeQDial<RetType, T: QDial_FreeQDial<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQDial(self);
    // return 1;
  }
}

pub trait QDial_FreeQDial<RetType> {
  fn FreeQDial(self , rsthis: &mut QDial) -> RetType;
}

  // proto:  void QDial::~QDial();
impl<'a> /*trait*/ QDial_FreeQDial<()> for () {
  fn FreeQDial(self , rsthis: &mut QDial) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDialD0Ev()};
     unsafe {_ZN5QDialD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QDial::metaObject();
impl /*struct*/ QDial {
  pub fn metaObject<RetType, T: QDial_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDial_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QDial) -> RetType;
}

  // proto:  const QMetaObject * QDial::metaObject();
impl<'a> /*trait*/ QDial_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QDial) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial10metaObjectEv()};
     unsafe {_ZNK5QDial10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QDial::notchesVisible();
impl /*struct*/ QDial {
  pub fn notchesVisible<RetType, T: QDial_notchesVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.notchesVisible(self);
    // return 1;
  }
}

pub trait QDial_notchesVisible<RetType> {
  fn notchesVisible(self , rsthis: &mut QDial) -> RetType;
}

  // proto:  bool QDial::notchesVisible();
impl<'a> /*trait*/ QDial_notchesVisible<i8> for () {
  fn notchesVisible(self , rsthis: &mut QDial) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial14notchesVisibleEv()};
    let mut ret = unsafe {_ZNK5QDial14notchesVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QDial::setNotchTarget(double target);
impl /*struct*/ QDial {
  pub fn setNotchTarget<RetType, T: QDial_setNotchTarget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setNotchTarget(self);
    // return 1;
  }
}

pub trait QDial_setNotchTarget<RetType> {
  fn setNotchTarget(self , rsthis: &mut QDial) -> RetType;
}

  // proto:  void QDial::setNotchTarget(double target);
impl<'a> /*trait*/ QDial_setNotchTarget<()> for (f64) {
  fn setNotchTarget(self , rsthis: &mut QDial) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDial14setNotchTargetEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN5QDial14setNotchTargetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDial::setWrapping(bool on);
impl /*struct*/ QDial {
  pub fn setWrapping<RetType, T: QDial_setWrapping<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWrapping(self);
    // return 1;
  }
}

pub trait QDial_setWrapping<RetType> {
  fn setWrapping(self , rsthis: &mut QDial) -> RetType;
}

  // proto:  void QDial::setWrapping(bool on);
impl<'a> /*trait*/ QDial_setWrapping<()> for (i8) {
  fn setWrapping(self , rsthis: &mut QDial) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDial11setWrappingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN5QDial11setWrappingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QDial::notchSize();
impl /*struct*/ QDial {
  pub fn notchSize<RetType, T: QDial_notchSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.notchSize(self);
    // return 1;
  }
}

pub trait QDial_notchSize<RetType> {
  fn notchSize(self , rsthis: &mut QDial) -> RetType;
}

  // proto:  int QDial::notchSize();
impl<'a> /*trait*/ QDial_notchSize<i32> for () {
  fn notchSize(self , rsthis: &mut QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial9notchSizeEv()};
    let mut ret = unsafe {_ZNK5QDial9notchSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QDial::setNotchesVisible(bool visible);
impl /*struct*/ QDial {
  pub fn setNotchesVisible<RetType, T: QDial_setNotchesVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setNotchesVisible(self);
    // return 1;
  }
}

pub trait QDial_setNotchesVisible<RetType> {
  fn setNotchesVisible(self , rsthis: &mut QDial) -> RetType;
}

  // proto:  void QDial::setNotchesVisible(bool visible);
impl<'a> /*trait*/ QDial_setNotchesVisible<()> for (i8) {
  fn setNotchesVisible(self , rsthis: &mut QDial) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDial17setNotchesVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN5QDial17setNotchesVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QDial::minimumSizeHint();
impl /*struct*/ QDial {
  pub fn minimumSizeHint<RetType, T: QDial_minimumSizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QDial_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: &mut QDial) -> RetType;
}

  // proto:  QSize QDial::minimumSizeHint();
impl<'a> /*trait*/ QDial_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: &mut QDial) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK5QDial15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDial::QDial(const QDial & );
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

  // proto:  void QDial::QDial(const QDial & );
impl<'a> /*trait*/ QDial_NewQDial for (QDial) {
  fn NewQDial(self) -> QDial {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDialC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QDialC1ERKS_(qthis, arg0)};
    let rsthis = QDial{/**/qbase: QAbstractSlider::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QDial::notchTarget();
impl /*struct*/ QDial {
  pub fn notchTarget<RetType, T: QDial_notchTarget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.notchTarget(self);
    // return 1;
  }
}

pub trait QDial_notchTarget<RetType> {
  fn notchTarget(self , rsthis: &mut QDial) -> RetType;
}

  // proto:  qreal QDial::notchTarget();
impl<'a> /*trait*/ QDial_notchTarget<f64> for () {
  fn notchTarget(self , rsthis: &mut QDial) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial11notchTargetEv()};
    let mut ret = unsafe {_ZNK5QDial11notchTargetEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QSize QDial::sizeHint();
impl /*struct*/ QDial {
  pub fn sizeHint<RetType, T: QDial_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QDial_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QDial) -> RetType;
}

  // proto:  QSize QDial::sizeHint();
impl<'a> /*trait*/ QDial_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QDial) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDial8sizeHintEv()};
    let mut ret = unsafe {_ZNK5QDial8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDial::QDial(QWidget * parent);
impl<'a> /*trait*/ QDial_NewQDial for (QWidget) {
  fn NewQDial(self) -> QDial {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDialC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QDialC1EP7QWidget(qthis, arg0)};
    let rsthis = QDial{/**/qbase: QAbstractSlider::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

