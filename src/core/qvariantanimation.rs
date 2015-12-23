// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
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
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QVariantAnimation::setDuration(int msecs);
  fn _ZN17QVariantAnimation11setDurationEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QVariantAnimation::setKeyValueAt(qreal step, const QVariant & value);
  fn _ZN17QVariantAnimation13setKeyValueAtEdRK8QVariant(qthis: *mut c_void, arg0: c_double, arg1: *mut c_void);
  // proto:  void QVariantAnimation::valueChanged(const QVariant & value);
  fn _ZN17QVariantAnimation12valueChangedERK8QVariant(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QVariant QVariantAnimation::endValue();
  fn _ZNK17QVariantAnimation8endValueEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QVariantAnimation::keyValueAt(qreal step);
  fn _ZNK17QVariantAnimation10keyValueAtEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  void QVariantAnimation::~QVariantAnimation();
  fn _ZN17QVariantAnimationD0Ev(qthis: *mut c_void);
  // proto:  QVariant QVariantAnimation::currentValue();
  fn _ZNK17QVariantAnimation12currentValueEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QVariantAnimation::duration();
  fn _ZNK17QVariantAnimation8durationEv(qthis: *mut c_void) -> c_int;
  // proto:  KeyValues QVariantAnimation::keyValues();
  fn _ZNK17QVariantAnimation9keyValuesEv(qthis: *mut c_void);
  // proto:  void QVariantAnimation::setStartValue(const QVariant & value);
  fn _ZN17QVariantAnimation13setStartValueERK8QVariant(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QVariant QVariantAnimation::startValue();
  fn _ZNK17QVariantAnimation10startValueEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QVariantAnimation::metaObject();
  fn _ZNK17QVariantAnimation10metaObjectEv(qthis: *mut c_void);
  // proto:  void QVariantAnimation::setEndValue(const QVariant & value);
  fn _ZN17QVariantAnimation11setEndValueERK8QVariant(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QEasingCurve QVariantAnimation::easingCurve();
  fn _ZNK17QVariantAnimation11easingCurveEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariantAnimation::QVariantAnimation(const QVariantAnimation & );
  fn _ZN17QVariantAnimationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariantAnimation::QVariantAnimation(QObject * parent);
  fn _ZN17QVariantAnimationC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariantAnimation::setEasingCurve(const QEasingCurve & easing);
  fn _ZN17QVariantAnimation14setEasingCurveERK12QEasingCurve(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QVariantAnimation)=1
pub struct QVariantAnimation {
  qbase: QAbstractAnimation,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QVariantAnimation {
  pub fn inheritFrom(qthis: *mut c_void) -> QVariantAnimation {
    return QVariantAnimation{qbase: QAbstractAnimation::inheritFrom(qthis), qclsinst: qthis};
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

  // proto:  void QVariantAnimation::valueChanged(const QVariant & value);
impl /*struct*/ QVariantAnimation {
  pub fn valueChanged<RetType, T: QVariantAnimation_valueChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.valueChanged(self);
    // return 1;
  }
}

pub trait QVariantAnimation_valueChanged<RetType> {
  fn valueChanged(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  void QVariantAnimation::valueChanged(const QVariant & value);
impl<'a> /*trait*/ QVariantAnimation_valueChanged<()> for (&'a QVariant) {
  fn valueChanged(self , rsthis: & QVariantAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QVariantAnimation12valueChangedERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QVariantAnimation12valueChangedERK8QVariant(rsthis.qclsinst, arg0)};
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
    let mut ret1 = QVariant::inheritFrom(ret);
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
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariantAnimation::~QVariantAnimation();
impl /*struct*/ QVariantAnimation {
  pub fn Free<RetType, T: QVariantAnimation_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QVariantAnimation_Free<RetType> {
  fn Free(self , rsthis: & QVariantAnimation) -> RetType;
}

  // proto:  void QVariantAnimation::~QVariantAnimation();
impl<'a> /*trait*/ QVariantAnimation_Free<()> for () {
  fn Free(self , rsthis: & QVariantAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QVariantAnimationD0Ev()};
     unsafe {_ZN17QVariantAnimationD0Ev(rsthis.qclsinst)};
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
    let mut ret1 = QVariant::inheritFrom(ret);
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
    let mut ret1 = QVariant::inheritFrom(ret);
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
    let mut ret1 = QEasingCurve::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariantAnimation::QVariantAnimation(const QVariantAnimation & );
impl /*struct*/ QVariantAnimation {
  pub fn New<T: QVariantAnimation_New>(value: T) -> QVariantAnimation {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QVariantAnimation_New {
  fn New(self) -> QVariantAnimation;
}

  // proto:  void QVariantAnimation::QVariantAnimation(const QVariantAnimation & );
impl<'a> /*trait*/ QVariantAnimation_New for (&'a QVariantAnimation) {
  fn New(self) -> QVariantAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QVariantAnimationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QVariantAnimationC1ERKS_(qthis, arg0)};
    let rsthis = QVariantAnimation{/**/qbase: QAbstractAnimation::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVariantAnimation::QVariantAnimation(QObject * parent);
impl<'a> /*trait*/ QVariantAnimation_New for (&'a QObject) {
  fn New(self) -> QVariantAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QVariantAnimationC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QVariantAnimationC1EP7QObject(qthis, arg0)};
    let rsthis = QVariantAnimation{/**/qbase: QAbstractAnimation::inheritFrom(qthis), /**/qclsinst: qthis};
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

// <= body block end

