// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
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
// use super::qcoreevent::QEvent; // 773
use std::ops::Deref;
use super::qbytearray::QByteArray; // 773
use super::qobject::QObject; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDeferredDeleteEvent_Class_Size() -> c_int;
  // proto:  void QDeferredDeleteEvent::~QDeferredDeleteEvent();
  fn _ZN20QDeferredDeleteEventD0Ev(qthis: *mut c_void);
  // proto:  void QDeferredDeleteEvent::QDeferredDeleteEvent();
  fn dector_ZN20QDeferredDeleteEventC1Ev() -> *mut c_void;
  fn _ZN20QDeferredDeleteEventC1Ev(qthis: *mut c_void);
  fn QDynamicPropertyChangeEvent_Class_Size() -> c_int;
  // proto:  void QDynamicPropertyChangeEvent::~QDynamicPropertyChangeEvent();
  fn _ZN27QDynamicPropertyChangeEventD0Ev(qthis: *mut c_void);
  // proto:  void QDynamicPropertyChangeEvent::QDynamicPropertyChangeEvent(const QByteArray & name);
  fn dector_ZN27QDynamicPropertyChangeEventC1ERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  fn _ZN27QDynamicPropertyChangeEventC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  fn QTimerEvent_Class_Size() -> c_int;
  // proto:  void QTimerEvent::QTimerEvent(int timerId);
  fn dector_ZN11QTimerEventC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN11QTimerEventC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTimerEvent::~QTimerEvent();
  fn _ZN11QTimerEventD0Ev(qthis: *mut c_void);
  fn QChildEvent_Class_Size() -> c_int;
  // proto:  void QChildEvent::~QChildEvent();
  fn _ZN11QChildEventD0Ev(qthis: *mut c_void);
  fn QEvent_Class_Size() -> c_int;
  // proto:  void QEvent::~QEvent();
  fn _ZN6QEventD0Ev(qthis: *mut c_void);
  // proto:  void QEvent::QEvent(const QEvent & other);
  fn dector_ZN6QEventC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN6QEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static int QEvent::registerEventType(int hint);
  fn _ZN6QEvent17registerEventTypeEi(arg0: c_int) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QDeferredDeleteEvent)=24
pub struct QDeferredDeleteEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QDynamicPropertyChangeEvent)=32
pub struct QDynamicPropertyChangeEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTimerEvent)=24
pub struct QTimerEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QChildEvent)=32
pub struct QChildEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QEvent)=24
pub struct QEvent {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDeferredDeleteEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QDeferredDeleteEvent {
    return QDeferredDeleteEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QDeferredDeleteEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QDeferredDeleteEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QDeferredDeleteEvent::~QDeferredDeleteEvent();
impl /*struct*/ QDeferredDeleteEvent {
  pub fn Free<RetType, T: QDeferredDeleteEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDeferredDeleteEvent_Free<RetType> {
  fn Free(self , rsthis: & QDeferredDeleteEvent) -> RetType;
}

  // proto:  void QDeferredDeleteEvent::~QDeferredDeleteEvent();
impl<'a> /*trait*/ QDeferredDeleteEvent_Free<()> for () {
  fn Free(self , rsthis: & QDeferredDeleteEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QDeferredDeleteEventD0Ev()};
     unsafe {_ZN20QDeferredDeleteEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDeferredDeleteEvent::QDeferredDeleteEvent();
impl /*struct*/ QDeferredDeleteEvent {
  pub fn New<T: QDeferredDeleteEvent_New>(value: T) -> QDeferredDeleteEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDeferredDeleteEvent_New {
  fn New(self) -> QDeferredDeleteEvent;
}

  // proto:  void QDeferredDeleteEvent::QDeferredDeleteEvent();
impl<'a> /*trait*/ QDeferredDeleteEvent_New for () {
  fn New(self) -> QDeferredDeleteEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QDeferredDeleteEventC1Ev()};
    let ctysz: c_int = unsafe{QDeferredDeleteEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN20QDeferredDeleteEventC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN20QDeferredDeleteEventC1Ev()};
    let rsthis = QDeferredDeleteEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QDynamicPropertyChangeEvent {
    return QDynamicPropertyChangeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QDynamicPropertyChangeEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QDynamicPropertyChangeEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QDynamicPropertyChangeEvent::~QDynamicPropertyChangeEvent();
impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn Free<RetType, T: QDynamicPropertyChangeEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDynamicPropertyChangeEvent_Free<RetType> {
  fn Free(self , rsthis: & QDynamicPropertyChangeEvent) -> RetType;
}

  // proto:  void QDynamicPropertyChangeEvent::~QDynamicPropertyChangeEvent();
impl<'a> /*trait*/ QDynamicPropertyChangeEvent_Free<()> for () {
  fn Free(self , rsthis: & QDynamicPropertyChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QDynamicPropertyChangeEventD0Ev()};
     unsafe {_ZN27QDynamicPropertyChangeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDynamicPropertyChangeEvent::QDynamicPropertyChangeEvent(const QByteArray & name);
impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn New<T: QDynamicPropertyChangeEvent_New>(value: T) -> QDynamicPropertyChangeEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDynamicPropertyChangeEvent_New {
  fn New(self) -> QDynamicPropertyChangeEvent;
}

  // proto:  void QDynamicPropertyChangeEvent::QDynamicPropertyChangeEvent(const QByteArray & name);
impl<'a> /*trait*/ QDynamicPropertyChangeEvent_New for (&'a QByteArray) {
  fn New(self) -> QDynamicPropertyChangeEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QDynamicPropertyChangeEventC1ERK10QByteArray()};
    let ctysz: c_int = unsafe{QDynamicPropertyChangeEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN27QDynamicPropertyChangeEventC1ERK10QByteArray(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN27QDynamicPropertyChangeEventC1ERK10QByteArray(arg0)};
    let rsthis = QDynamicPropertyChangeEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTimerEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QTimerEvent {
    return QTimerEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTimerEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QTimerEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QTimerEvent::QTimerEvent(int timerId);
impl /*struct*/ QTimerEvent {
  pub fn New<T: QTimerEvent_New>(value: T) -> QTimerEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTimerEvent_New {
  fn New(self) -> QTimerEvent;
}

  // proto:  void QTimerEvent::QTimerEvent(int timerId);
impl<'a> /*trait*/ QTimerEvent_New for (i32) {
  fn New(self) -> QTimerEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTimerEventC1Ei()};
    let ctysz: c_int = unsafe{QTimerEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self  as c_int;
    // unsafe {_ZN11QTimerEventC1Ei(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTimerEventC1Ei(arg0)};
    let rsthis = QTimerEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTimerEvent::~QTimerEvent();
impl /*struct*/ QTimerEvent {
  pub fn Free<RetType, T: QTimerEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTimerEvent_Free<RetType> {
  fn Free(self , rsthis: & QTimerEvent) -> RetType;
}

  // proto:  void QTimerEvent::~QTimerEvent();
impl<'a> /*trait*/ QTimerEvent_Free<()> for () {
  fn Free(self , rsthis: & QTimerEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTimerEventD0Ev()};
     unsafe {_ZN11QTimerEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QChildEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QChildEvent {
    return QChildEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QChildEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QChildEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QChildEvent::~QChildEvent();
impl /*struct*/ QChildEvent {
  pub fn Free<RetType, T: QChildEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QChildEvent_Free<RetType> {
  fn Free(self , rsthis: & QChildEvent) -> RetType;
}

  // proto:  void QChildEvent::~QChildEvent();
impl<'a> /*trait*/ QChildEvent_Free<()> for () {
  fn Free(self , rsthis: & QChildEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QChildEventD0Ev()};
     unsafe {_ZN11QChildEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QEvent {
    return QEvent{qclsinst: qthis};
  }
}
  // proto:  void QEvent::~QEvent();
impl /*struct*/ QEvent {
  pub fn Free<RetType, T: QEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QEvent_Free<RetType> {
  fn Free(self , rsthis: & QEvent) -> RetType;
}

  // proto:  void QEvent::~QEvent();
impl<'a> /*trait*/ QEvent_Free<()> for () {
  fn Free(self , rsthis: & QEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEventD0Ev()};
     unsafe {_ZN6QEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QEvent::QEvent(const QEvent & other);
impl /*struct*/ QEvent {
  pub fn New<T: QEvent_New>(value: T) -> QEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QEvent_New {
  fn New(self) -> QEvent;
}

  // proto:  void QEvent::QEvent(const QEvent & other);
impl<'a> /*trait*/ QEvent_New for (&'a QEvent) {
  fn New(self) -> QEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEventC1ERKS_()};
    let ctysz: c_int = unsafe{QEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN6QEventC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN6QEventC1ERKS_(arg0)};
    let rsthis = QEvent{qclsinst: qthis};
    return rsthis;
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

// <= body block end

