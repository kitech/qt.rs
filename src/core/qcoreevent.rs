// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qcoreevent.h
// dst-file: /src/core/qcoreevent.rs
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
use super::qbytearray::QByteArray; // 773
use super::qobject::QObject; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  int QDeferredDeleteEvent::loopLevel();
  fn _ZNK20QDeferredDeleteEvent9loopLevelEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDeferredDeleteEvent::~QDeferredDeleteEvent();
  fn _ZN20QDeferredDeleteEventD0Ev(qthis: *mut c_void);
  // proto:  void QDeferredDeleteEvent::QDeferredDeleteEvent();
  fn _ZN20QDeferredDeleteEventC1Ev(qthis: *mut c_void);
  // proto:  void QDynamicPropertyChangeEvent::~QDynamicPropertyChangeEvent();
  fn _ZN27QDynamicPropertyChangeEventD0Ev(qthis: *mut c_void);
  // proto:  void QDynamicPropertyChangeEvent::QDynamicPropertyChangeEvent(const QByteArray & name);
  fn _ZN27QDynamicPropertyChangeEventC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QByteArray QDynamicPropertyChangeEvent::propertyName();
  fn _ZNK27QDynamicPropertyChangeEvent12propertyNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTimerEvent::QTimerEvent(int timerId);
  fn _ZN11QTimerEventC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTimerEvent::~QTimerEvent();
  fn _ZN11QTimerEventD0Ev(qthis: *mut c_void);
  // proto:  int QTimerEvent::timerId();
  fn _ZNK11QTimerEvent7timerIdEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QChildEvent::added();
  fn _ZNK11QChildEvent5addedEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QChildEvent::polished();
  fn _ZNK11QChildEvent8polishedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QChildEvent::~QChildEvent();
  fn _ZN11QChildEventD0Ev(qthis: *mut c_void);
  // proto:  bool QChildEvent::removed();
  fn _ZNK11QChildEvent7removedEv(qthis: *mut c_void) -> c_char;
  // proto:  QObject * QChildEvent::child();
  fn _ZNK11QChildEvent5childEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QEvent::setAccepted(bool accepted);
  fn _ZN6QEvent11setAcceptedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QEvent::ignore();
  fn _ZN6QEvent6ignoreEv(qthis: *mut c_void);
  // proto:  bool QEvent::isAccepted();
  fn _ZNK6QEvent10isAcceptedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QEvent::~QEvent();
  fn _ZN6QEventD0Ev(qthis: *mut c_void);
  // proto:  void QEvent::QEvent(const QEvent & other);
  fn _ZN6QEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QEvent::accept();
  fn _ZN6QEvent6acceptEv(qthis: *mut c_void);
  // proto: static int QEvent::registerEventType(int hint);
  fn _ZN6QEvent17registerEventTypeEi(arg0: c_int) -> c_int;
  // proto:  bool QEvent::spontaneous();
  fn _ZNK6QEvent11spontaneousEv(qthis: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QDeferredDeleteEvent)=24
pub struct QDeferredDeleteEvent {
  pub qclsinst: *mut c_void,
}

// class sizeof(QDynamicPropertyChangeEvent)=32
pub struct QDynamicPropertyChangeEvent {
  pub qclsinst: *mut c_void,
}

// class sizeof(QTimerEvent)=24
pub struct QTimerEvent {
  pub qclsinst: *mut c_void,
}

// class sizeof(QChildEvent)=32
pub struct QChildEvent {
  pub qclsinst: *mut c_void,
}

// class sizeof(QEvent)=24
pub struct QEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  int QDeferredDeleteEvent::loopLevel();
impl /*struct*/ QDeferredDeleteEvent {
  pub fn loopLevel<RetType, T: QDeferredDeleteEvent_loopLevel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.loopLevel(self);
    // return 1;
  }
}

pub trait QDeferredDeleteEvent_loopLevel<RetType> {
  fn loopLevel(self , rsthis: &mut QDeferredDeleteEvent) -> RetType;
}

  // proto:  int QDeferredDeleteEvent::loopLevel();
impl<'a> /*trait*/ QDeferredDeleteEvent_loopLevel<i32> for () {
  fn loopLevel(self , rsthis: &mut QDeferredDeleteEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QDeferredDeleteEvent9loopLevelEv()};
    let mut ret = unsafe {_ZNK20QDeferredDeleteEvent9loopLevelEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QDeferredDeleteEvent::~QDeferredDeleteEvent();
impl /*struct*/ QDeferredDeleteEvent {
  pub fn FreeQDeferredDeleteEvent<RetType, T: QDeferredDeleteEvent_FreeQDeferredDeleteEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQDeferredDeleteEvent(self);
    // return 1;
  }
}

pub trait QDeferredDeleteEvent_FreeQDeferredDeleteEvent<RetType> {
  fn FreeQDeferredDeleteEvent(self , rsthis: &mut QDeferredDeleteEvent) -> RetType;
}

  // proto:  void QDeferredDeleteEvent::~QDeferredDeleteEvent();
impl<'a> /*trait*/ QDeferredDeleteEvent_FreeQDeferredDeleteEvent<()> for () {
  fn FreeQDeferredDeleteEvent(self , rsthis: &mut QDeferredDeleteEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QDeferredDeleteEventD0Ev()};
     unsafe {_ZN20QDeferredDeleteEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDeferredDeleteEvent::QDeferredDeleteEvent();
impl /*struct*/ QDeferredDeleteEvent {
  pub fn NewQDeferredDeleteEvent<T: QDeferredDeleteEvent_NewQDeferredDeleteEvent>(value: T) -> QDeferredDeleteEvent {
    let rsthis = value.NewQDeferredDeleteEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QDeferredDeleteEvent_NewQDeferredDeleteEvent {
  fn NewQDeferredDeleteEvent(self) -> QDeferredDeleteEvent;
}

  // proto:  void QDeferredDeleteEvent::QDeferredDeleteEvent();
impl<'a> /*trait*/ QDeferredDeleteEvent_NewQDeferredDeleteEvent for () {
  fn NewQDeferredDeleteEvent(self) -> QDeferredDeleteEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QDeferredDeleteEventC1Ev()};
    unsafe {_ZN20QDeferredDeleteEventC1Ev(qthis)};
    let rsthis = QDeferredDeleteEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDynamicPropertyChangeEvent::~QDynamicPropertyChangeEvent();
impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn FreeQDynamicPropertyChangeEvent<RetType, T: QDynamicPropertyChangeEvent_FreeQDynamicPropertyChangeEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQDynamicPropertyChangeEvent(self);
    // return 1;
  }
}

pub trait QDynamicPropertyChangeEvent_FreeQDynamicPropertyChangeEvent<RetType> {
  fn FreeQDynamicPropertyChangeEvent(self , rsthis: &mut QDynamicPropertyChangeEvent) -> RetType;
}

  // proto:  void QDynamicPropertyChangeEvent::~QDynamicPropertyChangeEvent();
impl<'a> /*trait*/ QDynamicPropertyChangeEvent_FreeQDynamicPropertyChangeEvent<()> for () {
  fn FreeQDynamicPropertyChangeEvent(self , rsthis: &mut QDynamicPropertyChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QDynamicPropertyChangeEventD0Ev()};
     unsafe {_ZN27QDynamicPropertyChangeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDynamicPropertyChangeEvent::QDynamicPropertyChangeEvent(const QByteArray & name);
impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn NewQDynamicPropertyChangeEvent<T: QDynamicPropertyChangeEvent_NewQDynamicPropertyChangeEvent>(value: T) -> QDynamicPropertyChangeEvent {
    let rsthis = value.NewQDynamicPropertyChangeEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QDynamicPropertyChangeEvent_NewQDynamicPropertyChangeEvent {
  fn NewQDynamicPropertyChangeEvent(self) -> QDynamicPropertyChangeEvent;
}

  // proto:  void QDynamicPropertyChangeEvent::QDynamicPropertyChangeEvent(const QByteArray & name);
impl<'a> /*trait*/ QDynamicPropertyChangeEvent_NewQDynamicPropertyChangeEvent for (QByteArray) {
  fn NewQDynamicPropertyChangeEvent(self) -> QDynamicPropertyChangeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QDynamicPropertyChangeEventC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN27QDynamicPropertyChangeEventC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QDynamicPropertyChangeEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray QDynamicPropertyChangeEvent::propertyName();
impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn propertyName<RetType, T: QDynamicPropertyChangeEvent_propertyName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.propertyName(self);
    // return 1;
  }
}

pub trait QDynamicPropertyChangeEvent_propertyName<RetType> {
  fn propertyName(self , rsthis: &mut QDynamicPropertyChangeEvent) -> RetType;
}

  // proto:  QByteArray QDynamicPropertyChangeEvent::propertyName();
impl<'a> /*trait*/ QDynamicPropertyChangeEvent_propertyName<QByteArray> for () {
  fn propertyName(self , rsthis: &mut QDynamicPropertyChangeEvent) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QDynamicPropertyChangeEvent12propertyNameEv()};
    let mut ret = unsafe {_ZNK27QDynamicPropertyChangeEvent12propertyNameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTimerEvent::QTimerEvent(int timerId);
impl /*struct*/ QTimerEvent {
  pub fn NewQTimerEvent<T: QTimerEvent_NewQTimerEvent>(value: T) -> QTimerEvent {
    let rsthis = value.NewQTimerEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QTimerEvent_NewQTimerEvent {
  fn NewQTimerEvent(self) -> QTimerEvent;
}

  // proto:  void QTimerEvent::QTimerEvent(int timerId);
impl<'a> /*trait*/ QTimerEvent_NewQTimerEvent for (i32) {
  fn NewQTimerEvent(self) -> QTimerEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTimerEventC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QTimerEventC1Ei(qthis, arg0)};
    let rsthis = QTimerEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTimerEvent::~QTimerEvent();
impl /*struct*/ QTimerEvent {
  pub fn FreeQTimerEvent<RetType, T: QTimerEvent_FreeQTimerEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTimerEvent(self);
    // return 1;
  }
}

pub trait QTimerEvent_FreeQTimerEvent<RetType> {
  fn FreeQTimerEvent(self , rsthis: &mut QTimerEvent) -> RetType;
}

  // proto:  void QTimerEvent::~QTimerEvent();
impl<'a> /*trait*/ QTimerEvent_FreeQTimerEvent<()> for () {
  fn FreeQTimerEvent(self , rsthis: &mut QTimerEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTimerEventD0Ev()};
     unsafe {_ZN11QTimerEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTimerEvent::timerId();
impl /*struct*/ QTimerEvent {
  pub fn timerId<RetType, T: QTimerEvent_timerId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.timerId(self);
    // return 1;
  }
}

pub trait QTimerEvent_timerId<RetType> {
  fn timerId(self , rsthis: &mut QTimerEvent) -> RetType;
}

  // proto:  int QTimerEvent::timerId();
impl<'a> /*trait*/ QTimerEvent_timerId<i32> for () {
  fn timerId(self , rsthis: &mut QTimerEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTimerEvent7timerIdEv()};
    let mut ret = unsafe {_ZNK11QTimerEvent7timerIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QChildEvent::added();
impl /*struct*/ QChildEvent {
  pub fn added<RetType, T: QChildEvent_added<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.added(self);
    // return 1;
  }
}

pub trait QChildEvent_added<RetType> {
  fn added(self , rsthis: &mut QChildEvent) -> RetType;
}

  // proto:  bool QChildEvent::added();
impl<'a> /*trait*/ QChildEvent_added<i8> for () {
  fn added(self , rsthis: &mut QChildEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent5addedEv()};
    let mut ret = unsafe {_ZNK11QChildEvent5addedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QChildEvent::polished();
impl /*struct*/ QChildEvent {
  pub fn polished<RetType, T: QChildEvent_polished<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.polished(self);
    // return 1;
  }
}

pub trait QChildEvent_polished<RetType> {
  fn polished(self , rsthis: &mut QChildEvent) -> RetType;
}

  // proto:  bool QChildEvent::polished();
impl<'a> /*trait*/ QChildEvent_polished<i8> for () {
  fn polished(self , rsthis: &mut QChildEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent8polishedEv()};
    let mut ret = unsafe {_ZNK11QChildEvent8polishedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QChildEvent::~QChildEvent();
impl /*struct*/ QChildEvent {
  pub fn FreeQChildEvent<RetType, T: QChildEvent_FreeQChildEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQChildEvent(self);
    // return 1;
  }
}

pub trait QChildEvent_FreeQChildEvent<RetType> {
  fn FreeQChildEvent(self , rsthis: &mut QChildEvent) -> RetType;
}

  // proto:  void QChildEvent::~QChildEvent();
impl<'a> /*trait*/ QChildEvent_FreeQChildEvent<()> for () {
  fn FreeQChildEvent(self , rsthis: &mut QChildEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QChildEventD0Ev()};
     unsafe {_ZN11QChildEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QChildEvent::removed();
impl /*struct*/ QChildEvent {
  pub fn removed<RetType, T: QChildEvent_removed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removed(self);
    // return 1;
  }
}

pub trait QChildEvent_removed<RetType> {
  fn removed(self , rsthis: &mut QChildEvent) -> RetType;
}

  // proto:  bool QChildEvent::removed();
impl<'a> /*trait*/ QChildEvent_removed<i8> for () {
  fn removed(self , rsthis: &mut QChildEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent7removedEv()};
    let mut ret = unsafe {_ZNK11QChildEvent7removedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QObject * QChildEvent::child();
impl /*struct*/ QChildEvent {
  pub fn child<RetType, T: QChildEvent_child<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.child(self);
    // return 1;
  }
}

pub trait QChildEvent_child<RetType> {
  fn child(self , rsthis: &mut QChildEvent) -> RetType;
}

  // proto:  QObject * QChildEvent::child();
impl<'a> /*trait*/ QChildEvent_child<QObject> for () {
  fn child(self , rsthis: &mut QChildEvent) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent5childEv()};
    let mut ret = unsafe {_ZNK11QChildEvent5childEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QEvent::setAccepted(bool accepted);
impl /*struct*/ QEvent {
  pub fn setAccepted<RetType, T: QEvent_setAccepted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAccepted(self);
    // return 1;
  }
}

pub trait QEvent_setAccepted<RetType> {
  fn setAccepted(self , rsthis: &mut QEvent) -> RetType;
}

  // proto:  void QEvent::setAccepted(bool accepted);
impl<'a> /*trait*/ QEvent_setAccepted<()> for (i8) {
  fn setAccepted(self , rsthis: &mut QEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEvent11setAcceptedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN6QEvent11setAcceptedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QEvent::ignore();
impl /*struct*/ QEvent {
  pub fn ignore<RetType, T: QEvent_ignore<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ignore(self);
    // return 1;
  }
}

pub trait QEvent_ignore<RetType> {
  fn ignore(self , rsthis: &mut QEvent) -> RetType;
}

  // proto:  void QEvent::ignore();
impl<'a> /*trait*/ QEvent_ignore<()> for () {
  fn ignore(self , rsthis: &mut QEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEvent6ignoreEv()};
     unsafe {_ZN6QEvent6ignoreEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QEvent::isAccepted();
impl /*struct*/ QEvent {
  pub fn isAccepted<RetType, T: QEvent_isAccepted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isAccepted(self);
    // return 1;
  }
}

pub trait QEvent_isAccepted<RetType> {
  fn isAccepted(self , rsthis: &mut QEvent) -> RetType;
}

  // proto:  bool QEvent::isAccepted();
impl<'a> /*trait*/ QEvent_isAccepted<i8> for () {
  fn isAccepted(self , rsthis: &mut QEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QEvent10isAcceptedEv()};
    let mut ret = unsafe {_ZNK6QEvent10isAcceptedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QEvent::~QEvent();
impl /*struct*/ QEvent {
  pub fn FreeQEvent<RetType, T: QEvent_FreeQEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQEvent(self);
    // return 1;
  }
}

pub trait QEvent_FreeQEvent<RetType> {
  fn FreeQEvent(self , rsthis: &mut QEvent) -> RetType;
}

  // proto:  void QEvent::~QEvent();
impl<'a> /*trait*/ QEvent_FreeQEvent<()> for () {
  fn FreeQEvent(self , rsthis: &mut QEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEventD0Ev()};
     unsafe {_ZN6QEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QEvent::QEvent(const QEvent & other);
impl /*struct*/ QEvent {
  pub fn NewQEvent<T: QEvent_NewQEvent>(value: T) -> QEvent {
    let rsthis = value.NewQEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QEvent_NewQEvent {
  fn NewQEvent(self) -> QEvent;
}

  // proto:  void QEvent::QEvent(const QEvent & other);
impl<'a> /*trait*/ QEvent_NewQEvent for (QEvent) {
  fn NewQEvent(self) -> QEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QEventC1ERKS_(qthis, arg0)};
    let rsthis = QEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEvent::accept();
impl /*struct*/ QEvent {
  pub fn accept<RetType, T: QEvent_accept<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.accept(self);
    // return 1;
  }
}

pub trait QEvent_accept<RetType> {
  fn accept(self , rsthis: &mut QEvent) -> RetType;
}

  // proto:  void QEvent::accept();
impl<'a> /*trait*/ QEvent_accept<()> for () {
  fn accept(self , rsthis: &mut QEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEvent6acceptEv()};
     unsafe {_ZN6QEvent6acceptEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static int QEvent::registerEventType(int hint);
impl /*struct*/ QEvent {
  pub fn registerEventType_s<RetType, T: QEvent_registerEventType_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.registerEventType_s();
    // return 1;
  }
}

pub trait QEvent_registerEventType_s<RetType> {
  fn registerEventType_s(self ) -> RetType;
}

  // proto: static int QEvent::registerEventType(int hint);
impl<'a> /*trait*/ QEvent_registerEventType_s<i32> for (i32) {
  fn registerEventType_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEvent17registerEventTypeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN6QEvent17registerEventTypeEi(arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QEvent::spontaneous();
impl /*struct*/ QEvent {
  pub fn spontaneous<RetType, T: QEvent_spontaneous<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.spontaneous(self);
    // return 1;
  }
}

pub trait QEvent_spontaneous<RetType> {
  fn spontaneous(self , rsthis: &mut QEvent) -> RetType;
}

  // proto:  bool QEvent::spontaneous();
impl<'a> /*trait*/ QEvent_spontaneous<i8> for () {
  fn spontaneous(self , rsthis: &mut QEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QEvent11spontaneousEv()};
    let mut ret = unsafe {_ZNK6QEvent11spontaneousEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

