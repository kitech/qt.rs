// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtGui/qstylehints.h
// dst-file: /src/gui/qstylehints.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qchar::QChar; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStyleHints_Class_Size() -> c_int;
  // proto:  void QStyleHints::setMouseDoubleClickInterval(int mouseDoubleClickInterval);
  fn _ZN11QStyleHints27setMouseDoubleClickIntervalEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QStyleHints::mousePressAndHoldInterval();
  fn _ZNK11QStyleHints25mousePressAndHoldIntervalEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QStyleHints::passwordMaskDelay();
  fn _ZNK11QStyleHints17passwordMaskDelayEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QMetaObject * QStyleHints::metaObject();
  fn _ZNK11QStyleHints10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QStyleHints::setKeyboardInputInterval(int keyboardInputInterval);
  fn _ZN11QStyleHints24setKeyboardInputIntervalEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QStyleHints::QStyleHints();
  fn dector_ZN11QStyleHintsC1Ev() -> *mut c_void;
  fn _ZN11QStyleHintsC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QStyleHints::startDragDistanceChanged(int startDragDistance);
  fn _ZN11QStyleHints24startDragDistanceChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QStyleHints::showIsFullScreen();
  fn _ZNK11QStyleHints16showIsFullScreenEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QStyleHints::useRtlExtensions();
  fn _ZNK11QStyleHints16useRtlExtensionsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QStyleHints::mouseDoubleClickIntervalChanged(int mouseDoubleClickInterval);
  fn _ZN11QStyleHints31mouseDoubleClickIntervalChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QStyleHints::setStartDragDistance(int startDragDistance);
  fn _ZN11QStyleHints20setStartDragDistanceEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QStyleHints::setFocusOnTouchRelease();
  fn _ZNK11QStyleHints22setFocusOnTouchReleaseEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QStyleHints::startDragVelocity();
  fn _ZNK11QStyleHints17startDragVelocityEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QStyleHints::startDragTime();
  fn _ZNK11QStyleHints13startDragTimeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QStyleHints::keyboardInputInterval();
  fn _ZNK11QStyleHints21keyboardInputIntervalEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QStyleHints::keyboardInputIntervalChanged(int keyboardInputInterval);
  fn _ZN11QStyleHints28keyboardInputIntervalChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QStyleHints::setStartDragTime(int startDragTime);
  fn _ZN11QStyleHints16setStartDragTimeEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QStyleHints::setCursorFlashTime(int cursorFlashTime);
  fn _ZN11QStyleHints18setCursorFlashTimeEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QStyleHints::cursorFlashTime();
  fn _ZNK11QStyleHints15cursorFlashTimeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QStyleHints::cursorFlashTimeChanged(int cursorFlashTime);
  fn _ZN11QStyleHints22cursorFlashTimeChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QChar QStyleHints::passwordMaskCharacter();
  fn _ZNK11QStyleHints21passwordMaskCharacterEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QStyleHints::keyboardAutoRepeatRate();
  fn _ZNK11QStyleHints22keyboardAutoRepeatRateEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QStyleHints::startDragDistance();
  fn _ZNK11QStyleHints17startDragDistanceEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  qreal QStyleHints::fontSmoothingGamma();
  fn _ZNK11QStyleHints18fontSmoothingGammaEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QStyleHints::singleClickActivation();
  fn _ZNK11QStyleHints21singleClickActivationEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QStyleHints::mouseDoubleClickInterval();
  fn _ZNK11QStyleHints24mouseDoubleClickIntervalEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QStyleHints::startDragTimeChanged(int startDragTime);
  fn _ZN11QStyleHints20startDragTimeChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  fn QStyleHints_SlotProxy_connect__ZN11QStyleHints31mouseDoubleClickIntervalChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QStyleHints_SlotProxy_connect__ZN11QStyleHints20startDragTimeChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QStyleHints_SlotProxy_connect__ZN11QStyleHints28keyboardInputIntervalChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QStyleHints_SlotProxy_connect__ZN11QStyleHints22cursorFlashTimeChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QStyleHints_SlotProxy_connect__ZN11QStyleHints24startDragDistanceChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStyleHints)=1
#[derive(Default)]
pub struct QStyleHints {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _startDragDistanceChanged_1: QStyleHints_startDragDistanceChanged_signal,
  pub _startDragTimeChanged_1: QStyleHints_startDragTimeChanged_signal,
  pub _mouseDoubleClickIntervalChanged_1: QStyleHints_mouseDoubleClickIntervalChanged_signal,
  pub _cursorFlashTimeChanged_1: QStyleHints_cursorFlashTimeChanged_signal,
  pub _keyboardInputIntervalChanged_1: QStyleHints_keyboardInputIntervalChanged_signal,
}

impl /*struct*/ QStyleHints {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleHints {
    return QStyleHints{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QStyleHints {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QStyleHints {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QStyleHints::setMouseDoubleClickInterval(int mouseDoubleClickInterval);
impl /*struct*/ QStyleHints {
  pub fn setMouseDoubleClickInterval<RetType, T: QStyleHints_setMouseDoubleClickInterval<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMouseDoubleClickInterval(self);
    // return 1;
  }
}

pub trait QStyleHints_setMouseDoubleClickInterval<RetType> {
  fn setMouseDoubleClickInterval(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  void QStyleHints::setMouseDoubleClickInterval(int mouseDoubleClickInterval);
impl<'a> /*trait*/ QStyleHints_setMouseDoubleClickInterval<()> for (i32) {
  fn setMouseDoubleClickInterval(self , rsthis: & QStyleHints) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints27setMouseDoubleClickIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QStyleHints27setMouseDoubleClickIntervalEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QStyleHints::mousePressAndHoldInterval();
impl /*struct*/ QStyleHints {
  pub fn mousePressAndHoldInterval<RetType, T: QStyleHints_mousePressAndHoldInterval<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mousePressAndHoldInterval(self);
    // return 1;
  }
}

pub trait QStyleHints_mousePressAndHoldInterval<RetType> {
  fn mousePressAndHoldInterval(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  int QStyleHints::mousePressAndHoldInterval();
impl<'a> /*trait*/ QStyleHints_mousePressAndHoldInterval<i32> for () {
  fn mousePressAndHoldInterval(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints25mousePressAndHoldIntervalEv()};
    let mut ret = unsafe {_ZNK11QStyleHints25mousePressAndHoldIntervalEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStyleHints::passwordMaskDelay();
impl /*struct*/ QStyleHints {
  pub fn passwordMaskDelay<RetType, T: QStyleHints_passwordMaskDelay<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.passwordMaskDelay(self);
    // return 1;
  }
}

pub trait QStyleHints_passwordMaskDelay<RetType> {
  fn passwordMaskDelay(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  int QStyleHints::passwordMaskDelay();
impl<'a> /*trait*/ QStyleHints_passwordMaskDelay<i32> for () {
  fn passwordMaskDelay(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints17passwordMaskDelayEv()};
    let mut ret = unsafe {_ZNK11QStyleHints17passwordMaskDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QStyleHints::metaObject();
impl /*struct*/ QStyleHints {
  pub fn metaObject<RetType, T: QStyleHints_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStyleHints_metaObject<RetType> {
  fn metaObject(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  const QMetaObject * QStyleHints::metaObject();
impl<'a> /*trait*/ QStyleHints_metaObject<()> for () {
  fn metaObject(self , rsthis: & QStyleHints) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints10metaObjectEv()};
     unsafe {_ZNK11QStyleHints10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyleHints::setKeyboardInputInterval(int keyboardInputInterval);
impl /*struct*/ QStyleHints {
  pub fn setKeyboardInputInterval<RetType, T: QStyleHints_setKeyboardInputInterval<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKeyboardInputInterval(self);
    // return 1;
  }
}

pub trait QStyleHints_setKeyboardInputInterval<RetType> {
  fn setKeyboardInputInterval(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  void QStyleHints::setKeyboardInputInterval(int keyboardInputInterval);
impl<'a> /*trait*/ QStyleHints_setKeyboardInputInterval<()> for (i32) {
  fn setKeyboardInputInterval(self , rsthis: & QStyleHints) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints24setKeyboardInputIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QStyleHints24setKeyboardInputIntervalEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyleHints::QStyleHints();
impl /*struct*/ QStyleHints {
  pub fn New<T: QStyleHints_New>(value: T) -> QStyleHints {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHints_New {
  fn New(self) -> QStyleHints;
}

  // proto:  void QStyleHints::QStyleHints();
impl<'a> /*trait*/ QStyleHints_New for () {
  fn New(self) -> QStyleHints {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHintsC1Ev()};
    let ctysz: c_int = unsafe{QStyleHints_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN11QStyleHintsC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN11QStyleHintsC1Ev()} as u64;
    let rsthis = QStyleHints{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleHints::startDragDistanceChanged(int startDragDistance);
impl /*struct*/ QStyleHints {
  pub fn startDragDistanceChanged<RetType, T: QStyleHints_startDragDistanceChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startDragDistanceChanged(self);
    // return 1;
  }
}

pub trait QStyleHints_startDragDistanceChanged<RetType> {
  fn startDragDistanceChanged(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  void QStyleHints::startDragDistanceChanged(int startDragDistance);
impl<'a> /*trait*/ QStyleHints_startDragDistanceChanged<()> for (i32) {
  fn startDragDistanceChanged(self , rsthis: & QStyleHints) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints24startDragDistanceChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QStyleHints24startDragDistanceChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QStyleHints::showIsFullScreen();
impl /*struct*/ QStyleHints {
  pub fn showIsFullScreen<RetType, T: QStyleHints_showIsFullScreen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showIsFullScreen(self);
    // return 1;
  }
}

pub trait QStyleHints_showIsFullScreen<RetType> {
  fn showIsFullScreen(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  bool QStyleHints::showIsFullScreen();
impl<'a> /*trait*/ QStyleHints_showIsFullScreen<i8> for () {
  fn showIsFullScreen(self , rsthis: & QStyleHints) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints16showIsFullScreenEv()};
    let mut ret = unsafe {_ZNK11QStyleHints16showIsFullScreenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QStyleHints::useRtlExtensions();
impl /*struct*/ QStyleHints {
  pub fn useRtlExtensions<RetType, T: QStyleHints_useRtlExtensions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.useRtlExtensions(self);
    // return 1;
  }
}

pub trait QStyleHints_useRtlExtensions<RetType> {
  fn useRtlExtensions(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  bool QStyleHints::useRtlExtensions();
impl<'a> /*trait*/ QStyleHints_useRtlExtensions<i8> for () {
  fn useRtlExtensions(self , rsthis: & QStyleHints) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints16useRtlExtensionsEv()};
    let mut ret = unsafe {_ZNK11QStyleHints16useRtlExtensionsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QStyleHints::mouseDoubleClickIntervalChanged(int mouseDoubleClickInterval);
impl /*struct*/ QStyleHints {
  pub fn mouseDoubleClickIntervalChanged<RetType, T: QStyleHints_mouseDoubleClickIntervalChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickIntervalChanged(self);
    // return 1;
  }
}

pub trait QStyleHints_mouseDoubleClickIntervalChanged<RetType> {
  fn mouseDoubleClickIntervalChanged(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  void QStyleHints::mouseDoubleClickIntervalChanged(int mouseDoubleClickInterval);
impl<'a> /*trait*/ QStyleHints_mouseDoubleClickIntervalChanged<()> for (i32) {
  fn mouseDoubleClickIntervalChanged(self , rsthis: & QStyleHints) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints31mouseDoubleClickIntervalChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QStyleHints31mouseDoubleClickIntervalChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyleHints::setStartDragDistance(int startDragDistance);
impl /*struct*/ QStyleHints {
  pub fn setStartDragDistance<RetType, T: QStyleHints_setStartDragDistance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStartDragDistance(self);
    // return 1;
  }
}

pub trait QStyleHints_setStartDragDistance<RetType> {
  fn setStartDragDistance(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  void QStyleHints::setStartDragDistance(int startDragDistance);
impl<'a> /*trait*/ QStyleHints_setStartDragDistance<()> for (i32) {
  fn setStartDragDistance(self , rsthis: & QStyleHints) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints20setStartDragDistanceEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QStyleHints20setStartDragDistanceEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QStyleHints::setFocusOnTouchRelease();
impl /*struct*/ QStyleHints {
  pub fn setFocusOnTouchRelease<RetType, T: QStyleHints_setFocusOnTouchRelease<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFocusOnTouchRelease(self);
    // return 1;
  }
}

pub trait QStyleHints_setFocusOnTouchRelease<RetType> {
  fn setFocusOnTouchRelease(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  bool QStyleHints::setFocusOnTouchRelease();
impl<'a> /*trait*/ QStyleHints_setFocusOnTouchRelease<i8> for () {
  fn setFocusOnTouchRelease(self , rsthis: & QStyleHints) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints22setFocusOnTouchReleaseEv()};
    let mut ret = unsafe {_ZNK11QStyleHints22setFocusOnTouchReleaseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QStyleHints::startDragVelocity();
impl /*struct*/ QStyleHints {
  pub fn startDragVelocity<RetType, T: QStyleHints_startDragVelocity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startDragVelocity(self);
    // return 1;
  }
}

pub trait QStyleHints_startDragVelocity<RetType> {
  fn startDragVelocity(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  int QStyleHints::startDragVelocity();
impl<'a> /*trait*/ QStyleHints_startDragVelocity<i32> for () {
  fn startDragVelocity(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints17startDragVelocityEv()};
    let mut ret = unsafe {_ZNK11QStyleHints17startDragVelocityEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStyleHints::startDragTime();
impl /*struct*/ QStyleHints {
  pub fn startDragTime<RetType, T: QStyleHints_startDragTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startDragTime(self);
    // return 1;
  }
}

pub trait QStyleHints_startDragTime<RetType> {
  fn startDragTime(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  int QStyleHints::startDragTime();
impl<'a> /*trait*/ QStyleHints_startDragTime<i32> for () {
  fn startDragTime(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints13startDragTimeEv()};
    let mut ret = unsafe {_ZNK11QStyleHints13startDragTimeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStyleHints::keyboardInputInterval();
impl /*struct*/ QStyleHints {
  pub fn keyboardInputInterval<RetType, T: QStyleHints_keyboardInputInterval<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyboardInputInterval(self);
    // return 1;
  }
}

pub trait QStyleHints_keyboardInputInterval<RetType> {
  fn keyboardInputInterval(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  int QStyleHints::keyboardInputInterval();
impl<'a> /*trait*/ QStyleHints_keyboardInputInterval<i32> for () {
  fn keyboardInputInterval(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints21keyboardInputIntervalEv()};
    let mut ret = unsafe {_ZNK11QStyleHints21keyboardInputIntervalEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStyleHints::keyboardInputIntervalChanged(int keyboardInputInterval);
impl /*struct*/ QStyleHints {
  pub fn keyboardInputIntervalChanged<RetType, T: QStyleHints_keyboardInputIntervalChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyboardInputIntervalChanged(self);
    // return 1;
  }
}

pub trait QStyleHints_keyboardInputIntervalChanged<RetType> {
  fn keyboardInputIntervalChanged(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  void QStyleHints::keyboardInputIntervalChanged(int keyboardInputInterval);
impl<'a> /*trait*/ QStyleHints_keyboardInputIntervalChanged<()> for (i32) {
  fn keyboardInputIntervalChanged(self , rsthis: & QStyleHints) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints28keyboardInputIntervalChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QStyleHints28keyboardInputIntervalChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyleHints::setStartDragTime(int startDragTime);
impl /*struct*/ QStyleHints {
  pub fn setStartDragTime<RetType, T: QStyleHints_setStartDragTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStartDragTime(self);
    // return 1;
  }
}

pub trait QStyleHints_setStartDragTime<RetType> {
  fn setStartDragTime(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  void QStyleHints::setStartDragTime(int startDragTime);
impl<'a> /*trait*/ QStyleHints_setStartDragTime<()> for (i32) {
  fn setStartDragTime(self , rsthis: & QStyleHints) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints16setStartDragTimeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QStyleHints16setStartDragTimeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyleHints::setCursorFlashTime(int cursorFlashTime);
impl /*struct*/ QStyleHints {
  pub fn setCursorFlashTime<RetType, T: QStyleHints_setCursorFlashTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCursorFlashTime(self);
    // return 1;
  }
}

pub trait QStyleHints_setCursorFlashTime<RetType> {
  fn setCursorFlashTime(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  void QStyleHints::setCursorFlashTime(int cursorFlashTime);
impl<'a> /*trait*/ QStyleHints_setCursorFlashTime<()> for (i32) {
  fn setCursorFlashTime(self , rsthis: & QStyleHints) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints18setCursorFlashTimeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QStyleHints18setCursorFlashTimeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QStyleHints::cursorFlashTime();
impl /*struct*/ QStyleHints {
  pub fn cursorFlashTime<RetType, T: QStyleHints_cursorFlashTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorFlashTime(self);
    // return 1;
  }
}

pub trait QStyleHints_cursorFlashTime<RetType> {
  fn cursorFlashTime(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  int QStyleHints::cursorFlashTime();
impl<'a> /*trait*/ QStyleHints_cursorFlashTime<i32> for () {
  fn cursorFlashTime(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints15cursorFlashTimeEv()};
    let mut ret = unsafe {_ZNK11QStyleHints15cursorFlashTimeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStyleHints::cursorFlashTimeChanged(int cursorFlashTime);
impl /*struct*/ QStyleHints {
  pub fn cursorFlashTimeChanged<RetType, T: QStyleHints_cursorFlashTimeChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursorFlashTimeChanged(self);
    // return 1;
  }
}

pub trait QStyleHints_cursorFlashTimeChanged<RetType> {
  fn cursorFlashTimeChanged(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  void QStyleHints::cursorFlashTimeChanged(int cursorFlashTime);
impl<'a> /*trait*/ QStyleHints_cursorFlashTimeChanged<()> for (i32) {
  fn cursorFlashTimeChanged(self , rsthis: & QStyleHints) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints22cursorFlashTimeChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QStyleHints22cursorFlashTimeChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QChar QStyleHints::passwordMaskCharacter();
impl /*struct*/ QStyleHints {
  pub fn passwordMaskCharacter<RetType, T: QStyleHints_passwordMaskCharacter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.passwordMaskCharacter(self);
    // return 1;
  }
}

pub trait QStyleHints_passwordMaskCharacter<RetType> {
  fn passwordMaskCharacter(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  QChar QStyleHints::passwordMaskCharacter();
impl<'a> /*trait*/ QStyleHints_passwordMaskCharacter<QChar> for () {
  fn passwordMaskCharacter(self , rsthis: & QStyleHints) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints21passwordMaskCharacterEv()};
    let mut ret = unsafe {_ZNK11QStyleHints21passwordMaskCharacterEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QStyleHints::keyboardAutoRepeatRate();
impl /*struct*/ QStyleHints {
  pub fn keyboardAutoRepeatRate<RetType, T: QStyleHints_keyboardAutoRepeatRate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyboardAutoRepeatRate(self);
    // return 1;
  }
}

pub trait QStyleHints_keyboardAutoRepeatRate<RetType> {
  fn keyboardAutoRepeatRate(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  int QStyleHints::keyboardAutoRepeatRate();
impl<'a> /*trait*/ QStyleHints_keyboardAutoRepeatRate<i32> for () {
  fn keyboardAutoRepeatRate(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints22keyboardAutoRepeatRateEv()};
    let mut ret = unsafe {_ZNK11QStyleHints22keyboardAutoRepeatRateEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStyleHints::startDragDistance();
impl /*struct*/ QStyleHints {
  pub fn startDragDistance<RetType, T: QStyleHints_startDragDistance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startDragDistance(self);
    // return 1;
  }
}

pub trait QStyleHints_startDragDistance<RetType> {
  fn startDragDistance(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  int QStyleHints::startDragDistance();
impl<'a> /*trait*/ QStyleHints_startDragDistance<i32> for () {
  fn startDragDistance(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints17startDragDistanceEv()};
    let mut ret = unsafe {_ZNK11QStyleHints17startDragDistanceEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qreal QStyleHints::fontSmoothingGamma();
impl /*struct*/ QStyleHints {
  pub fn fontSmoothingGamma<RetType, T: QStyleHints_fontSmoothingGamma<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontSmoothingGamma(self);
    // return 1;
  }
}

pub trait QStyleHints_fontSmoothingGamma<RetType> {
  fn fontSmoothingGamma(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  qreal QStyleHints::fontSmoothingGamma();
impl<'a> /*trait*/ QStyleHints_fontSmoothingGamma<f64> for () {
  fn fontSmoothingGamma(self , rsthis: & QStyleHints) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints18fontSmoothingGammaEv()};
    let mut ret = unsafe {_ZNK11QStyleHints18fontSmoothingGammaEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QStyleHints::singleClickActivation();
impl /*struct*/ QStyleHints {
  pub fn singleClickActivation<RetType, T: QStyleHints_singleClickActivation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.singleClickActivation(self);
    // return 1;
  }
}

pub trait QStyleHints_singleClickActivation<RetType> {
  fn singleClickActivation(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  bool QStyleHints::singleClickActivation();
impl<'a> /*trait*/ QStyleHints_singleClickActivation<i8> for () {
  fn singleClickActivation(self , rsthis: & QStyleHints) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints21singleClickActivationEv()};
    let mut ret = unsafe {_ZNK11QStyleHints21singleClickActivationEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QStyleHints::mouseDoubleClickInterval();
impl /*struct*/ QStyleHints {
  pub fn mouseDoubleClickInterval<RetType, T: QStyleHints_mouseDoubleClickInterval<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickInterval(self);
    // return 1;
  }
}

pub trait QStyleHints_mouseDoubleClickInterval<RetType> {
  fn mouseDoubleClickInterval(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  int QStyleHints::mouseDoubleClickInterval();
impl<'a> /*trait*/ QStyleHints_mouseDoubleClickInterval<i32> for () {
  fn mouseDoubleClickInterval(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints24mouseDoubleClickIntervalEv()};
    let mut ret = unsafe {_ZNK11QStyleHints24mouseDoubleClickIntervalEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStyleHints::startDragTimeChanged(int startDragTime);
impl /*struct*/ QStyleHints {
  pub fn startDragTimeChanged<RetType, T: QStyleHints_startDragTimeChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startDragTimeChanged(self);
    // return 1;
  }
}

pub trait QStyleHints_startDragTimeChanged<RetType> {
  fn startDragTimeChanged(self , rsthis: & QStyleHints) -> RetType;
}

  // proto:  void QStyleHints::startDragTimeChanged(int startDragTime);
impl<'a> /*trait*/ QStyleHints_startDragTimeChanged<()> for (i32) {
  fn startDragTimeChanged(self , rsthis: & QStyleHints) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints20startDragTimeChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QStyleHints20startDragTimeChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QStyleHints_startDragDistanceChanged
pub struct QStyleHints_startDragDistanceChanged_signal{poi:u64}
impl /* struct */ QStyleHints {
  pub fn startDragDistanceChanged_1(&self) -> QStyleHints_startDragDistanceChanged_signal {
     return QStyleHints_startDragDistanceChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStyleHints_startDragDistanceChanged_signal {
  pub fn connect<T: QStyleHints_startDragDistanceChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStyleHints_startDragDistanceChanged_signal_connect {
  fn connect(self, sigthis: QStyleHints_startDragDistanceChanged_signal);
}

#[derive(Default)] // for QStyleHints_startDragTimeChanged
pub struct QStyleHints_startDragTimeChanged_signal{poi:u64}
impl /* struct */ QStyleHints {
  pub fn startDragTimeChanged_1(&self) -> QStyleHints_startDragTimeChanged_signal {
     return QStyleHints_startDragTimeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStyleHints_startDragTimeChanged_signal {
  pub fn connect<T: QStyleHints_startDragTimeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStyleHints_startDragTimeChanged_signal_connect {
  fn connect(self, sigthis: QStyleHints_startDragTimeChanged_signal);
}

#[derive(Default)] // for QStyleHints_mouseDoubleClickIntervalChanged
pub struct QStyleHints_mouseDoubleClickIntervalChanged_signal{poi:u64}
impl /* struct */ QStyleHints {
  pub fn mouseDoubleClickIntervalChanged_1(&self) -> QStyleHints_mouseDoubleClickIntervalChanged_signal {
     return QStyleHints_mouseDoubleClickIntervalChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStyleHints_mouseDoubleClickIntervalChanged_signal {
  pub fn connect<T: QStyleHints_mouseDoubleClickIntervalChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStyleHints_mouseDoubleClickIntervalChanged_signal_connect {
  fn connect(self, sigthis: QStyleHints_mouseDoubleClickIntervalChanged_signal);
}

#[derive(Default)] // for QStyleHints_cursorFlashTimeChanged
pub struct QStyleHints_cursorFlashTimeChanged_signal{poi:u64}
impl /* struct */ QStyleHints {
  pub fn cursorFlashTimeChanged_1(&self) -> QStyleHints_cursorFlashTimeChanged_signal {
     return QStyleHints_cursorFlashTimeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStyleHints_cursorFlashTimeChanged_signal {
  pub fn connect<T: QStyleHints_cursorFlashTimeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStyleHints_cursorFlashTimeChanged_signal_connect {
  fn connect(self, sigthis: QStyleHints_cursorFlashTimeChanged_signal);
}

#[derive(Default)] // for QStyleHints_keyboardInputIntervalChanged
pub struct QStyleHints_keyboardInputIntervalChanged_signal{poi:u64}
impl /* struct */ QStyleHints {
  pub fn keyboardInputIntervalChanged_1(&self) -> QStyleHints_keyboardInputIntervalChanged_signal {
     return QStyleHints_keyboardInputIntervalChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStyleHints_keyboardInputIntervalChanged_signal {
  pub fn connect<T: QStyleHints_keyboardInputIntervalChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStyleHints_keyboardInputIntervalChanged_signal_connect {
  fn connect(self, sigthis: QStyleHints_keyboardInputIntervalChanged_signal);
}

// mouseDoubleClickIntervalChanged(int)
extern fn QStyleHints_mouseDoubleClickIntervalChanged_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QStyleHints_mouseDoubleClickIntervalChanged_signal_connect_cb_box_0(rsfptr_raw:*mut fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QStyleHints_mouseDoubleClickIntervalChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QStyleHints_mouseDoubleClickIntervalChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStyleHints_mouseDoubleClickIntervalChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QStyleHints_SlotProxy_connect__ZN11QStyleHints31mouseDoubleClickIntervalChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QStyleHints_mouseDoubleClickIntervalChanged_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QStyleHints_mouseDoubleClickIntervalChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStyleHints_mouseDoubleClickIntervalChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QStyleHints_SlotProxy_connect__ZN11QStyleHints31mouseDoubleClickIntervalChangedEi(arg0, arg1, arg2)};
  }
}
// startDragTimeChanged(int)
extern fn QStyleHints_startDragTimeChanged_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QStyleHints_startDragTimeChanged_signal_connect_cb_box_1(rsfptr_raw:*mut fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QStyleHints_startDragTimeChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QStyleHints_startDragTimeChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStyleHints_startDragTimeChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QStyleHints_SlotProxy_connect__ZN11QStyleHints20startDragTimeChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QStyleHints_startDragTimeChanged_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QStyleHints_startDragTimeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStyleHints_startDragTimeChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QStyleHints_SlotProxy_connect__ZN11QStyleHints20startDragTimeChangedEi(arg0, arg1, arg2)};
  }
}
// keyboardInputIntervalChanged(int)
extern fn QStyleHints_keyboardInputIntervalChanged_signal_connect_cb_2(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QStyleHints_keyboardInputIntervalChanged_signal_connect_cb_box_2(rsfptr_raw:*mut fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QStyleHints_keyboardInputIntervalChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QStyleHints_keyboardInputIntervalChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStyleHints_keyboardInputIntervalChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QStyleHints_SlotProxy_connect__ZN11QStyleHints28keyboardInputIntervalChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QStyleHints_keyboardInputIntervalChanged_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QStyleHints_keyboardInputIntervalChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStyleHints_keyboardInputIntervalChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QStyleHints_SlotProxy_connect__ZN11QStyleHints28keyboardInputIntervalChangedEi(arg0, arg1, arg2)};
  }
}
// cursorFlashTimeChanged(int)
extern fn QStyleHints_cursorFlashTimeChanged_signal_connect_cb_3(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QStyleHints_cursorFlashTimeChanged_signal_connect_cb_box_3(rsfptr_raw:*mut fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QStyleHints_cursorFlashTimeChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QStyleHints_cursorFlashTimeChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStyleHints_cursorFlashTimeChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QStyleHints_SlotProxy_connect__ZN11QStyleHints22cursorFlashTimeChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QStyleHints_cursorFlashTimeChanged_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QStyleHints_cursorFlashTimeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStyleHints_cursorFlashTimeChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QStyleHints_SlotProxy_connect__ZN11QStyleHints22cursorFlashTimeChangedEi(arg0, arg1, arg2)};
  }
}
// startDragDistanceChanged(int)
extern fn QStyleHints_startDragDistanceChanged_signal_connect_cb_4(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QStyleHints_startDragDistanceChanged_signal_connect_cb_box_4(rsfptr_raw:*mut fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QStyleHints_startDragDistanceChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QStyleHints_startDragDistanceChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStyleHints_startDragDistanceChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QStyleHints_SlotProxy_connect__ZN11QStyleHints24startDragDistanceChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QStyleHints_startDragDistanceChanged_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QStyleHints_startDragDistanceChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStyleHints_startDragDistanceChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QStyleHints_SlotProxy_connect__ZN11QStyleHints24startDragDistanceChangedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

