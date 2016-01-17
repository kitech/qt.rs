// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtCore/qvariantanimation.h
// dst-file: /src/core/qvariantanimation.rs
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
use super::qabstractanimation::QAbstractAnimation; // 773
use std::ops::Deref;
use super::qvariant::QVariant; // 773
use super::qobject::QObject; // 773
use super::qeasingcurve::QEasingCurve; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QVariantAnimation_Class_Size() -> c_int;
  // proto:  void QVariantAnimation::setDuration(int msecs);
  fn _ZN17QVariantAnimation11setDurationEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QVariantAnimation::setKeyValueAt(qreal step, const QVariant & value);
  fn _ZN17QVariantAnimation13setKeyValueAtEdRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: *mut c_void);
  // proto:  QVariant QVariantAnimation::endValue();
  fn _ZNK17QVariantAnimation8endValueEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QVariant QVariantAnimation::keyValueAt(qreal step);
  fn _ZNK17QVariantAnimation10keyValueAtEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> *mut c_void;
  // proto:  void QVariantAnimation::~QVariantAnimation();
  fn _ZN17QVariantAnimationD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QVariant QVariantAnimation::currentValue();
  fn _ZNK17QVariantAnimation12currentValueEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QVariantAnimation::duration();
  fn _ZNK17QVariantAnimation8durationEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  KeyValues QVariantAnimation::keyValues();
  fn _ZNK17QVariantAnimation9keyValuesEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QVariantAnimation::setStartValue(const QVariant & value);
  fn _ZN17QVariantAnimation13setStartValueERK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QVariant QVariantAnimation::startValue();
  fn _ZNK17QVariantAnimation10startValueEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QVariantAnimation::metaObject();
  fn _ZNK17QVariantAnimation10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QVariantAnimation::setEndValue(const QVariant & value);
  fn _ZN17QVariantAnimation11setEndValueERK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QEasingCurve QVariantAnimation::easingCurve();
  fn _ZNK17QVariantAnimation11easingCurveEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QVariantAnimation::QVariantAnimation(const QVariantAnimation & );
  fn _ZN17QVariantAnimationC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QVariantAnimation::QVariantAnimation(QObject * parent);
  fn _ZN17QVariantAnimationC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QVariantAnimation::setEasingCurve(const QEasingCurve & easing);
  fn _ZN17QVariantAnimation14setEasingCurveERK12QEasingCurve(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QVariantAnimation_SlotProxy_connect__ZN17QVariantAnimation12valueChangedERK8QVariant(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QVariantAnimation)=1
#[derive(Default)]
pub struct QVariantAnimation {
  qbase: QAbstractAnimation,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _valueChanged: QVariantAnimation_valueChanged_signal,
}

impl /*struct*/ QVariantAnimation {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QVariantAnimation {
    return QVariantAnimation{qbase: QAbstractAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QVariantAnimation {
  type Target = QAbstractAnimation;

  fn deref(&self) -> &QAbstractAnimation {
    return & self.qbase;
  }
}
impl AsRef<QAbstractAnimation> for QVariantAnimation {
  fn as_ref(& self) -> & QAbstractAnimation {
    return & self.qbase;
  }
}
  // proto:  void QVariantAnimation::setDuration(int msecs);
impl /*struct*/ QVariantAnimation {
  pub fn setDuration<RetType, T: QVariantAnimation_setDuration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDuration(self);
    // return 1;
  }
}

pub trait QVariantAnimation_setDuration<RetType> {
  fn setDuration(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  void QVariantAnimation::setDuration(int msecs);
impl<'a> /*trait*/ QVariantAnimation_setDuration<()> for (i32) {
  fn setDuration(self , rsthis: & QVariantAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QVariantAnimation11setDurationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN17QVariantAnimation11setDurationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QVariantAnimation::setKeyValueAt(qreal step, const QVariant & value);
impl /*struct*/ QVariantAnimation {
  pub fn setKeyValueAt<RetType, T: QVariantAnimation_setKeyValueAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKeyValueAt(self);
    // return 1;
  }
}

pub trait QVariantAnimation_setKeyValueAt<RetType> {
  fn setKeyValueAt(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  void QVariantAnimation::setKeyValueAt(qreal step, const QVariant & value);
impl<'a> /*trait*/ QVariantAnimation_setKeyValueAt<()> for (f64, &'a QVariant) {
  fn setKeyValueAt(self , rsthis: & QVariantAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QVariantAnimation13setKeyValueAtEdRK8QVariant()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN17QVariantAnimation13setKeyValueAtEdRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QVariant QVariantAnimation::endValue();
impl /*struct*/ QVariantAnimation {
  pub fn endValue<RetType, T: QVariantAnimation_endValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.endValue(self);
    // return 1;
  }
}

pub trait QVariantAnimation_endValue<RetType> {
  fn endValue(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  QVariant QVariantAnimation::endValue();
impl<'a> /*trait*/ QVariantAnimation_endValue<QVariant> for () {
  fn endValue(self , rsthis: & QVariantAnimation) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QVariantAnimation8endValueEv()};
    let mut ret = unsafe {_ZNK17QVariantAnimation8endValueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QVariantAnimation::keyValueAt(qreal step);
impl /*struct*/ QVariantAnimation {
  pub fn keyValueAt<RetType, T: QVariantAnimation_keyValueAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyValueAt(self);
    // return 1;
  }
}

pub trait QVariantAnimation_keyValueAt<RetType> {
  fn keyValueAt(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  QVariant QVariantAnimation::keyValueAt(qreal step);
impl<'a> /*trait*/ QVariantAnimation_keyValueAt<QVariant> for (f64) {
  fn keyValueAt(self , rsthis: & QVariantAnimation) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QVariantAnimation10keyValueAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK17QVariantAnimation10keyValueAtEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariantAnimation::~QVariantAnimation();
impl /*struct*/ QVariantAnimation {
  pub fn free<RetType, T: QVariantAnimation_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QVariantAnimation_free<RetType> {
  fn free(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  void QVariantAnimation::~QVariantAnimation();
impl<'a> /*trait*/ QVariantAnimation_free<()> for () {
  fn free(self , rsthis: & QVariantAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QVariantAnimationD2Ev()};
     unsafe {_ZN17QVariantAnimationD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVariant QVariantAnimation::currentValue();
impl /*struct*/ QVariantAnimation {
  pub fn currentValue<RetType, T: QVariantAnimation_currentValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentValue(self);
    // return 1;
  }
}

pub trait QVariantAnimation_currentValue<RetType> {
  fn currentValue(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  QVariant QVariantAnimation::currentValue();
impl<'a> /*trait*/ QVariantAnimation_currentValue<QVariant> for () {
  fn currentValue(self , rsthis: & QVariantAnimation) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QVariantAnimation12currentValueEv()};
    let mut ret = unsafe {_ZNK17QVariantAnimation12currentValueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QVariantAnimation::duration();
impl /*struct*/ QVariantAnimation {
  pub fn duration<RetType, T: QVariantAnimation_duration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.duration(self);
    // return 1;
  }
}

pub trait QVariantAnimation_duration<RetType> {
  fn duration(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  int QVariantAnimation::duration();
impl<'a> /*trait*/ QVariantAnimation_duration<i32> for () {
  fn duration(self , rsthis: & QVariantAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QVariantAnimation8durationEv()};
    let mut ret = unsafe {_ZNK17QVariantAnimation8durationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  KeyValues QVariantAnimation::keyValues();
impl /*struct*/ QVariantAnimation {
  pub fn keyValues<RetType, T: QVariantAnimation_keyValues<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyValues(self);
    // return 1;
  }
}

pub trait QVariantAnimation_keyValues<RetType> {
  fn keyValues(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  KeyValues QVariantAnimation::keyValues();
impl<'a> /*trait*/ QVariantAnimation_keyValues<()> for () {
  fn keyValues(self , rsthis: & QVariantAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QVariantAnimation9keyValuesEv()};
     unsafe {_ZNK17QVariantAnimation9keyValuesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QVariantAnimation::setStartValue(const QVariant & value);
impl /*struct*/ QVariantAnimation {
  pub fn setStartValue<RetType, T: QVariantAnimation_setStartValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStartValue(self);
    // return 1;
  }
}

pub trait QVariantAnimation_setStartValue<RetType> {
  fn setStartValue(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  void QVariantAnimation::setStartValue(const QVariant & value);
impl<'a> /*trait*/ QVariantAnimation_setStartValue<()> for (&'a QVariant) {
  fn setStartValue(self , rsthis: & QVariantAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QVariantAnimation13setStartValueERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QVariantAnimation13setStartValueERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVariant QVariantAnimation::startValue();
impl /*struct*/ QVariantAnimation {
  pub fn startValue<RetType, T: QVariantAnimation_startValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startValue(self);
    // return 1;
  }
}

pub trait QVariantAnimation_startValue<RetType> {
  fn startValue(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  QVariant QVariantAnimation::startValue();
impl<'a> /*trait*/ QVariantAnimation_startValue<QVariant> for () {
  fn startValue(self , rsthis: & QVariantAnimation) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QVariantAnimation10startValueEv()};
    let mut ret = unsafe {_ZNK17QVariantAnimation10startValueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QVariantAnimation::metaObject();
impl /*struct*/ QVariantAnimation {
  pub fn metaObject<RetType, T: QVariantAnimation_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QVariantAnimation_metaObject<RetType> {
  fn metaObject(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  const QMetaObject * QVariantAnimation::metaObject();
impl<'a> /*trait*/ QVariantAnimation_metaObject<()> for () {
  fn metaObject(self , rsthis: & QVariantAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QVariantAnimation10metaObjectEv()};
     unsafe {_ZNK17QVariantAnimation10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QVariantAnimation::setEndValue(const QVariant & value);
impl /*struct*/ QVariantAnimation {
  pub fn setEndValue<RetType, T: QVariantAnimation_setEndValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEndValue(self);
    // return 1;
  }
}

pub trait QVariantAnimation_setEndValue<RetType> {
  fn setEndValue(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  void QVariantAnimation::setEndValue(const QVariant & value);
impl<'a> /*trait*/ QVariantAnimation_setEndValue<()> for (&'a QVariant) {
  fn setEndValue(self , rsthis: & QVariantAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QVariantAnimation11setEndValueERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QVariantAnimation11setEndValueERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QEasingCurve QVariantAnimation::easingCurve();
impl /*struct*/ QVariantAnimation {
  pub fn easingCurve<RetType, T: QVariantAnimation_easingCurve<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.easingCurve(self);
    // return 1;
  }
}

pub trait QVariantAnimation_easingCurve<RetType> {
  fn easingCurve(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  QEasingCurve QVariantAnimation::easingCurve();
impl<'a> /*trait*/ QVariantAnimation_easingCurve<QEasingCurve> for () {
  fn easingCurve(self , rsthis: & QVariantAnimation) -> QEasingCurve {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QVariantAnimation11easingCurveEv()};
    let mut ret = unsafe {_ZNK17QVariantAnimation11easingCurveEv(rsthis.qclsinst)};
    let mut ret1 = QEasingCurve::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariantAnimation::QVariantAnimation(const QVariantAnimation & );
impl /*struct*/ QVariantAnimation {
  pub fn new<T: QVariantAnimation_new>(value: T) -> QVariantAnimation {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QVariantAnimation_new {
  fn new(self) -> QVariantAnimation;
}

  // proto:  void QVariantAnimation::QVariantAnimation(const QVariantAnimation & );
impl<'a> /*trait*/ QVariantAnimation_new for (&'a QVariantAnimation) {
  fn new(self) -> QVariantAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QVariantAnimationC2ERKS_()};
    let ctysz: c_int = unsafe{QVariantAnimation_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QVariantAnimationC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QVariantAnimation{qbase: QAbstractAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVariantAnimation::QVariantAnimation(QObject * parent);
impl<'a> /*trait*/ QVariantAnimation_new for (&'a QObject) {
  fn new(self) -> QVariantAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QVariantAnimationC2EP7QObject()};
    let ctysz: c_int = unsafe{QVariantAnimation_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QVariantAnimationC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QVariantAnimation{qbase: QAbstractAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVariantAnimation::setEasingCurve(const QEasingCurve & easing);
impl /*struct*/ QVariantAnimation {
  pub fn setEasingCurve<RetType, T: QVariantAnimation_setEasingCurve<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEasingCurve(self);
    // return 1;
  }
}

pub trait QVariantAnimation_setEasingCurve<RetType> {
  fn setEasingCurve(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  void QVariantAnimation::setEasingCurve(const QEasingCurve & easing);
impl<'a> /*trait*/ QVariantAnimation_setEasingCurve<()> for (&'a QEasingCurve) {
  fn setEasingCurve(self , rsthis: & QVariantAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QVariantAnimation14setEasingCurveERK12QEasingCurve()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QVariantAnimation14setEasingCurveERK12QEasingCurve(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QVariantAnimation_valueChanged
pub struct QVariantAnimation_valueChanged_signal{poi:u64}
impl /* struct */ QVariantAnimation {
  pub fn valueChanged(&self) -> QVariantAnimation_valueChanged_signal {
     return QVariantAnimation_valueChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QVariantAnimation_valueChanged_signal {
  pub fn connect<T: QVariantAnimation_valueChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QVariantAnimation_valueChanged_signal_connect {
  fn connect(self, sigthis: QVariantAnimation_valueChanged_signal);
}

// valueChanged(const class QVariant &)
extern fn QVariantAnimation_valueChanged_signal_connect_cb_0(rsfptr:fn(QVariant), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QVariant::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QVariantAnimation_valueChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QVariant)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QVariant::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QVariantAnimation_valueChanged_signal_connect for fn(QVariant) {
  fn connect(self, sigthis: QVariantAnimation_valueChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QVariantAnimation_valueChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QVariantAnimation_SlotProxy_connect__ZN17QVariantAnimation12valueChangedERK8QVariant(arg0, arg1, arg2)};
  }
}
impl /* trait */ QVariantAnimation_valueChanged_signal_connect for Box<Fn(QVariant)> {
  fn connect(self, sigthis: QVariantAnimation_valueChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QVariantAnimation_valueChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QVariantAnimation_SlotProxy_connect__ZN17QVariantAnimation12valueChangedERK8QVariant(arg0, arg1, arg2)};
  }
}
// <= body block end

