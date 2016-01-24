// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
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
  // proto:  int QDeferredDeleteEvent::loopLevel();
  fn C_ZNK20QDeferredDeleteEvent9loopLevelEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QDeferredDeleteEvent::~QDeferredDeleteEvent();
  fn C_ZN20QDeferredDeleteEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDeferredDeleteEvent::QDeferredDeleteEvent();
  fn C_ZN20QDeferredDeleteEventC2Ev() -> u64;
  fn QDynamicPropertyChangeEvent_Class_Size() -> c_int;
  // proto:  void QDynamicPropertyChangeEvent::~QDynamicPropertyChangeEvent();
  fn C_ZN27QDynamicPropertyChangeEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDynamicPropertyChangeEvent::QDynamicPropertyChangeEvent(const QByteArray & name);
  fn C_ZN27QDynamicPropertyChangeEventC2ERK10QByteArray(arg0: *mut c_void) -> u64;
  // proto:  QByteArray QDynamicPropertyChangeEvent::propertyName();
  fn C_ZNK27QDynamicPropertyChangeEvent12propertyNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QTimerEvent_Class_Size() -> c_int;
  // proto:  void QTimerEvent::QTimerEvent(int timerId);
  fn C_ZN11QTimerEventC2Ei(arg0: c_int) -> u64;
  // proto:  void QTimerEvent::~QTimerEvent();
  fn C_ZN11QTimerEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QTimerEvent::timerId();
  fn C_ZNK11QTimerEvent7timerIdEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QChildEvent_Class_Size() -> c_int;
  // proto:  bool QChildEvent::added();
  fn C_ZNK11QChildEvent5addedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QChildEvent::polished();
  fn C_ZNK11QChildEvent8polishedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QChildEvent::~QChildEvent();
  fn C_ZN11QChildEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QChildEvent::removed();
  fn C_ZNK11QChildEvent7removedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QObject * QChildEvent::child();
  fn C_ZNK11QChildEvent5childEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QEvent_Class_Size() -> c_int;
  // proto:  void QEvent::setAccepted(bool accepted);
  fn C_ZN6QEvent11setAcceptedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QEvent::ignore();
  fn C_ZN6QEvent6ignoreEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QEvent::isAccepted();
  fn C_ZNK6QEvent10isAcceptedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QEvent::~QEvent();
  fn C_ZN6QEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QEvent::QEvent(const QEvent & other);
  fn C_ZN6QEventC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QEvent::accept();
  fn C_ZN6QEvent6acceptEv(qthis: u64 /* *mut c_void*/);
  // proto: static int QEvent::registerEventType(int hint);
  fn C_ZN6QEvent17registerEventTypeEi(arg0: c_int) -> c_int;
  // proto:  bool QEvent::spontaneous();
  fn C_ZNK6QEvent11spontaneousEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QDeferredDeleteEvent)=24
#[derive(Default)]
pub struct QDeferredDeleteEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QDynamicPropertyChangeEvent)=32
#[derive(Default)]
pub struct QDynamicPropertyChangeEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTimerEvent)=24
#[derive(Default)]
pub struct QTimerEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QChildEvent)=32
#[derive(Default)]
pub struct QChildEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QEvent)=24
#[derive(Default)]
pub struct QEvent {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QDeferredDeleteEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDeferredDeleteEvent {
    return QDeferredDeleteEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  // proto:  int QDeferredDeleteEvent::loopLevel();
impl /*struct*/ QDeferredDeleteEvent {
  pub fn loopLevel<RetType, T: QDeferredDeleteEvent_loopLevel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loopLevel(self);
    // return 1;
  }
}

pub trait QDeferredDeleteEvent_loopLevel<RetType> {
  fn loopLevel(self , rsthis: & QDeferredDeleteEvent) -> RetType;
}

  // proto:  int QDeferredDeleteEvent::loopLevel();
impl<'a> /*trait*/ QDeferredDeleteEvent_loopLevel<i32> for () {
  fn loopLevel(self , rsthis: & QDeferredDeleteEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QDeferredDeleteEvent9loopLevelEv()};
    let mut ret = unsafe {C_ZNK20QDeferredDeleteEvent9loopLevelEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QDeferredDeleteEvent::~QDeferredDeleteEvent();
impl /*struct*/ QDeferredDeleteEvent {
  pub fn free<RetType, T: QDeferredDeleteEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDeferredDeleteEvent_free<RetType> {
  fn free(self , rsthis: & QDeferredDeleteEvent) -> RetType;
}

  // proto:  void QDeferredDeleteEvent::~QDeferredDeleteEvent();
impl<'a> /*trait*/ QDeferredDeleteEvent_free<()> for () {
  fn free(self , rsthis: & QDeferredDeleteEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QDeferredDeleteEventD2Ev()};
     unsafe {C_ZN20QDeferredDeleteEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDeferredDeleteEvent::QDeferredDeleteEvent();
impl /*struct*/ QDeferredDeleteEvent {
  pub fn new<T: QDeferredDeleteEvent_new>(value: T) -> QDeferredDeleteEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDeferredDeleteEvent_new {
  fn new(self) -> QDeferredDeleteEvent;
}

  // proto:  void QDeferredDeleteEvent::QDeferredDeleteEvent();
impl<'a> /*trait*/ QDeferredDeleteEvent_new for () {
  fn new(self) -> QDeferredDeleteEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QDeferredDeleteEventC2Ev()};
    let ctysz: c_int = unsafe{QDeferredDeleteEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN20QDeferredDeleteEventC2Ev()};
    let rsthis = QDeferredDeleteEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDynamicPropertyChangeEvent {
    return QDynamicPropertyChangeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn free<RetType, T: QDynamicPropertyChangeEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDynamicPropertyChangeEvent_free<RetType> {
  fn free(self , rsthis: & QDynamicPropertyChangeEvent) -> RetType;
}

  // proto:  void QDynamicPropertyChangeEvent::~QDynamicPropertyChangeEvent();
impl<'a> /*trait*/ QDynamicPropertyChangeEvent_free<()> for () {
  fn free(self , rsthis: & QDynamicPropertyChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QDynamicPropertyChangeEventD2Ev()};
     unsafe {C_ZN27QDynamicPropertyChangeEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDynamicPropertyChangeEvent::QDynamicPropertyChangeEvent(const QByteArray & name);
impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn new<T: QDynamicPropertyChangeEvent_new>(value: T) -> QDynamicPropertyChangeEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDynamicPropertyChangeEvent_new {
  fn new(self) -> QDynamicPropertyChangeEvent;
}

  // proto:  void QDynamicPropertyChangeEvent::QDynamicPropertyChangeEvent(const QByteArray & name);
impl<'a> /*trait*/ QDynamicPropertyChangeEvent_new for (&'a QByteArray) {
  fn new(self) -> QDynamicPropertyChangeEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QDynamicPropertyChangeEventC2ERK10QByteArray()};
    let ctysz: c_int = unsafe{QDynamicPropertyChangeEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN27QDynamicPropertyChangeEventC2ERK10QByteArray(arg0)};
    let rsthis = QDynamicPropertyChangeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray QDynamicPropertyChangeEvent::propertyName();
impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn propertyName<RetType, T: QDynamicPropertyChangeEvent_propertyName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.propertyName(self);
    // return 1;
  }
}

pub trait QDynamicPropertyChangeEvent_propertyName<RetType> {
  fn propertyName(self , rsthis: & QDynamicPropertyChangeEvent) -> RetType;
}

  // proto:  QByteArray QDynamicPropertyChangeEvent::propertyName();
impl<'a> /*trait*/ QDynamicPropertyChangeEvent_propertyName<QByteArray> for () {
  fn propertyName(self , rsthis: & QDynamicPropertyChangeEvent) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QDynamicPropertyChangeEvent12propertyNameEv()};
    let mut ret = unsafe {C_ZNK27QDynamicPropertyChangeEvent12propertyNameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTimerEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTimerEvent {
    return QTimerEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QTimerEvent_new>(value: T) -> QTimerEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTimerEvent_new {
  fn new(self) -> QTimerEvent;
}

  // proto:  void QTimerEvent::QTimerEvent(int timerId);
impl<'a> /*trait*/ QTimerEvent_new for (i32) {
  fn new(self) -> QTimerEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTimerEventC2Ei()};
    let ctysz: c_int = unsafe{QTimerEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    let qthis: u64 = unsafe {C_ZN11QTimerEventC2Ei(arg0)};
    let rsthis = QTimerEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTimerEvent::~QTimerEvent();
impl /*struct*/ QTimerEvent {
  pub fn free<RetType, T: QTimerEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTimerEvent_free<RetType> {
  fn free(self , rsthis: & QTimerEvent) -> RetType;
}

  // proto:  void QTimerEvent::~QTimerEvent();
impl<'a> /*trait*/ QTimerEvent_free<()> for () {
  fn free(self , rsthis: & QTimerEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTimerEventD2Ev()};
     unsafe {C_ZN11QTimerEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTimerEvent::timerId();
impl /*struct*/ QTimerEvent {
  pub fn timerId<RetType, T: QTimerEvent_timerId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.timerId(self);
    // return 1;
  }
}

pub trait QTimerEvent_timerId<RetType> {
  fn timerId(self , rsthis: & QTimerEvent) -> RetType;
}

  // proto:  int QTimerEvent::timerId();
impl<'a> /*trait*/ QTimerEvent_timerId<i32> for () {
  fn timerId(self , rsthis: & QTimerEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTimerEvent7timerIdEv()};
    let mut ret = unsafe {C_ZNK11QTimerEvent7timerIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QChildEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QChildEvent {
    return QChildEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  // proto:  bool QChildEvent::added();
impl /*struct*/ QChildEvent {
  pub fn added<RetType, T: QChildEvent_added<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.added(self);
    // return 1;
  }
}

pub trait QChildEvent_added<RetType> {
  fn added(self , rsthis: & QChildEvent) -> RetType;
}

  // proto:  bool QChildEvent::added();
impl<'a> /*trait*/ QChildEvent_added<i8> for () {
  fn added(self , rsthis: & QChildEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent5addedEv()};
    let mut ret = unsafe {C_ZNK11QChildEvent5addedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QChildEvent::polished();
impl /*struct*/ QChildEvent {
  pub fn polished<RetType, T: QChildEvent_polished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.polished(self);
    // return 1;
  }
}

pub trait QChildEvent_polished<RetType> {
  fn polished(self , rsthis: & QChildEvent) -> RetType;
}

  // proto:  bool QChildEvent::polished();
impl<'a> /*trait*/ QChildEvent_polished<i8> for () {
  fn polished(self , rsthis: & QChildEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent8polishedEv()};
    let mut ret = unsafe {C_ZNK11QChildEvent8polishedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QChildEvent::~QChildEvent();
impl /*struct*/ QChildEvent {
  pub fn free<RetType, T: QChildEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QChildEvent_free<RetType> {
  fn free(self , rsthis: & QChildEvent) -> RetType;
}

  // proto:  void QChildEvent::~QChildEvent();
impl<'a> /*trait*/ QChildEvent_free<()> for () {
  fn free(self , rsthis: & QChildEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QChildEventD2Ev()};
     unsafe {C_ZN11QChildEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QChildEvent::removed();
impl /*struct*/ QChildEvent {
  pub fn removed<RetType, T: QChildEvent_removed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removed(self);
    // return 1;
  }
}

pub trait QChildEvent_removed<RetType> {
  fn removed(self , rsthis: & QChildEvent) -> RetType;
}

  // proto:  bool QChildEvent::removed();
impl<'a> /*trait*/ QChildEvent_removed<i8> for () {
  fn removed(self , rsthis: & QChildEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent7removedEv()};
    let mut ret = unsafe {C_ZNK11QChildEvent7removedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QObject * QChildEvent::child();
impl /*struct*/ QChildEvent {
  pub fn child<RetType, T: QChildEvent_child<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.child(self);
    // return 1;
  }
}

pub trait QChildEvent_child<RetType> {
  fn child(self , rsthis: & QChildEvent) -> RetType;
}

  // proto:  QObject * QChildEvent::child();
impl<'a> /*trait*/ QChildEvent_child<QObject> for () {
  fn child(self , rsthis: & QChildEvent) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent5childEv()};
    let mut ret = unsafe {C_ZNK11QChildEvent5childEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QEvent {
    return QEvent{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QEvent::setAccepted(bool accepted);
impl /*struct*/ QEvent {
  pub fn setAccepted<RetType, T: QEvent_setAccepted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAccepted(self);
    // return 1;
  }
}

pub trait QEvent_setAccepted<RetType> {
  fn setAccepted(self , rsthis: & QEvent) -> RetType;
}

  // proto:  void QEvent::setAccepted(bool accepted);
impl<'a> /*trait*/ QEvent_setAccepted<()> for (i8) {
  fn setAccepted(self , rsthis: & QEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEvent11setAcceptedEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN6QEvent11setAcceptedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QEvent::ignore();
impl /*struct*/ QEvent {
  pub fn ignore<RetType, T: QEvent_ignore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ignore(self);
    // return 1;
  }
}

pub trait QEvent_ignore<RetType> {
  fn ignore(self , rsthis: & QEvent) -> RetType;
}

  // proto:  void QEvent::ignore();
impl<'a> /*trait*/ QEvent_ignore<()> for () {
  fn ignore(self , rsthis: & QEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEvent6ignoreEv()};
     unsafe {C_ZN6QEvent6ignoreEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QEvent::isAccepted();
impl /*struct*/ QEvent {
  pub fn isAccepted<RetType, T: QEvent_isAccepted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAccepted(self);
    // return 1;
  }
}

pub trait QEvent_isAccepted<RetType> {
  fn isAccepted(self , rsthis: & QEvent) -> RetType;
}

  // proto:  bool QEvent::isAccepted();
impl<'a> /*trait*/ QEvent_isAccepted<i8> for () {
  fn isAccepted(self , rsthis: & QEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QEvent10isAcceptedEv()};
    let mut ret = unsafe {C_ZNK6QEvent10isAcceptedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QEvent::~QEvent();
impl /*struct*/ QEvent {
  pub fn free<RetType, T: QEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QEvent_free<RetType> {
  fn free(self , rsthis: & QEvent) -> RetType;
}

  // proto:  void QEvent::~QEvent();
impl<'a> /*trait*/ QEvent_free<()> for () {
  fn free(self , rsthis: & QEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEventD2Ev()};
     unsafe {C_ZN6QEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QEvent::QEvent(const QEvent & other);
impl /*struct*/ QEvent {
  pub fn new<T: QEvent_new>(value: T) -> QEvent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QEvent_new {
  fn new(self) -> QEvent;
}

  // proto:  void QEvent::QEvent(const QEvent & other);
impl<'a> /*trait*/ QEvent_new for (&'a QEvent) {
  fn new(self) -> QEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEventC2ERKS_()};
    let ctysz: c_int = unsafe{QEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN6QEventC2ERKS_(arg0)};
    let rsthis = QEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEvent::accept();
impl /*struct*/ QEvent {
  pub fn accept<RetType, T: QEvent_accept<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.accept(self);
    // return 1;
  }
}

pub trait QEvent_accept<RetType> {
  fn accept(self , rsthis: & QEvent) -> RetType;
}

  // proto:  void QEvent::accept();
impl<'a> /*trait*/ QEvent_accept<()> for () {
  fn accept(self , rsthis: & QEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QEvent6acceptEv()};
     unsafe {C_ZN6QEvent6acceptEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZN6QEvent17registerEventTypeEi(arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QEvent::spontaneous();
impl /*struct*/ QEvent {
  pub fn spontaneous<RetType, T: QEvent_spontaneous<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.spontaneous(self);
    // return 1;
  }
}

pub trait QEvent_spontaneous<RetType> {
  fn spontaneous(self , rsthis: & QEvent) -> RetType;
}

  // proto:  bool QEvent::spontaneous();
impl<'a> /*trait*/ QEvent_spontaneous<i8> for () {
  fn spontaneous(self , rsthis: & QEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QEvent11spontaneousEv()};
    let mut ret = unsafe {C_ZNK6QEvent11spontaneousEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

