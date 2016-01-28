// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtGui/qopengldebug.h
// dst-file: /src/gui/qopengldebug.rs
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
use std::ops::Deref;
use super::super::core::qstring::*; // 771
use super::super::core::qobject::*; // 771
// use super::qopengldebug::QOpenGLDebugMessage; // 773
use super::super::core::qobjectdefs::*; // 771
// use super::qlist::*; // 775
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLDebugMessage_Class_Size() -> c_int;
  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage();
  fn C_ZN19QOpenGLDebugMessageC2Ev() -> u64;
  // proto:  void QOpenGLDebugMessage::~QOpenGLDebugMessage();
  fn C_ZN19QOpenGLDebugMessageD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  GLuint QOpenGLDebugMessage::id();
  fn C_ZNK19QOpenGLDebugMessage2idEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage(const QOpenGLDebugMessage & debugMessage);
  fn C_ZN19QOpenGLDebugMessageC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  QString QOpenGLDebugMessage::message();
  fn C_ZNK19QOpenGLDebugMessage7messageEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLDebugMessage::swap(QOpenGLDebugMessage & debugMessage);
  fn C_ZN19QOpenGLDebugMessage4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLDebugLogger_Class_Size() -> c_int;
  // proto:  void QOpenGLDebugLogger::~QOpenGLDebugLogger();
  fn C_ZN18QOpenGLDebugLoggerD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  qint64 QOpenGLDebugLogger::maximumMessageLength();
  fn C_ZNK18QOpenGLDebugLogger20maximumMessageLengthEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  bool QOpenGLDebugLogger::isLogging();
  fn C_ZNK18QOpenGLDebugLogger9isLoggingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLDebugLogger::stopLogging();
  fn C_ZN18QOpenGLDebugLogger11stopLoggingEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLDebugLogger::logMessage(const QOpenGLDebugMessage & debugMessage);
  fn C_ZN18QOpenGLDebugLogger10logMessageERK19QOpenGLDebugMessage(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QOpenGLDebugLogger::QOpenGLDebugLogger(QObject * parent);
  fn C_ZN18QOpenGLDebugLoggerC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  const QMetaObject * QOpenGLDebugLogger::metaObject();
  fn C_ZNK18QOpenGLDebugLogger10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QOpenGLDebugMessage> QOpenGLDebugLogger::loggedMessages();
  fn C_ZNK18QOpenGLDebugLogger14loggedMessagesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLDebugLogger::popGroup();
  fn C_ZN18QOpenGLDebugLogger8popGroupEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QOpenGLDebugLogger::initialize();
  fn C_ZN18QOpenGLDebugLogger10initializeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QOpenGLDebugLogger_SlotProxy_connect__ZN18QOpenGLDebugLogger13messageLoggedERK19QOpenGLDebugMessage(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLDebugMessage)=1
#[derive(Default)]
pub struct QOpenGLDebugMessage {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLDebugLogger)=1
#[derive(Default)]
pub struct QOpenGLDebugLogger {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _messageLogged: QOpenGLDebugLogger_messageLogged_signal,
}

impl /*struct*/ QOpenGLDebugMessage {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLDebugMessage {
    return QOpenGLDebugMessage{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage();
impl /*struct*/ QOpenGLDebugMessage {
  pub fn new<T: QOpenGLDebugMessage_new>(value: T) -> QOpenGLDebugMessage {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_new {
  fn new(self) -> QOpenGLDebugMessage;
}

  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage();
impl<'a> /*trait*/ QOpenGLDebugMessage_new for () {
  fn new(self) -> QOpenGLDebugMessage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLDebugMessageC2Ev()};
    let ctysz: c_int = unsafe{QOpenGLDebugMessage_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN19QOpenGLDebugMessageC2Ev()};
    let rsthis = QOpenGLDebugMessage{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLDebugMessage::~QOpenGLDebugMessage();
impl /*struct*/ QOpenGLDebugMessage {
  pub fn free<RetType, T: QOpenGLDebugMessage_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_free<RetType> {
  fn free(self , rsthis: & QOpenGLDebugMessage) -> RetType;
}

  // proto:  void QOpenGLDebugMessage::~QOpenGLDebugMessage();
impl<'a> /*trait*/ QOpenGLDebugMessage_free<()> for () {
  fn free(self , rsthis: & QOpenGLDebugMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLDebugMessageD2Ev()};
     unsafe {C_ZN19QOpenGLDebugMessageD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  GLuint QOpenGLDebugMessage::id();
impl /*struct*/ QOpenGLDebugMessage {
  pub fn id<RetType, T: QOpenGLDebugMessage_id<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.id(self);
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_id<RetType> {
  fn id(self , rsthis: & QOpenGLDebugMessage) -> RetType;
}

  // proto:  GLuint QOpenGLDebugMessage::id();
impl<'a> /*trait*/ QOpenGLDebugMessage_id<u32> for () {
  fn id(self , rsthis: & QOpenGLDebugMessage) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLDebugMessage2idEv()};
    let mut ret = unsafe {C_ZNK19QOpenGLDebugMessage2idEv(rsthis.qclsinst)};
    return ret as u32; // 1
    // return 1;
  }
}

  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage(const QOpenGLDebugMessage & debugMessage);
impl<'a> /*trait*/ QOpenGLDebugMessage_new for (&'a QOpenGLDebugMessage) {
  fn new(self) -> QOpenGLDebugMessage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLDebugMessageC2ERKS_()};
    let ctysz: c_int = unsafe{QOpenGLDebugMessage_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QOpenGLDebugMessageC2ERKS_(arg0)};
    let rsthis = QOpenGLDebugMessage{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QOpenGLDebugMessage::message();
impl /*struct*/ QOpenGLDebugMessage {
  pub fn message<RetType, T: QOpenGLDebugMessage_message<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.message(self);
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_message<RetType> {
  fn message(self , rsthis: & QOpenGLDebugMessage) -> RetType;
}

  // proto:  QString QOpenGLDebugMessage::message();
impl<'a> /*trait*/ QOpenGLDebugMessage_message<QString> for () {
  fn message(self , rsthis: & QOpenGLDebugMessage) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLDebugMessage7messageEv()};
    let mut ret = unsafe {C_ZNK19QOpenGLDebugMessage7messageEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLDebugMessage::swap(QOpenGLDebugMessage & debugMessage);
impl /*struct*/ QOpenGLDebugMessage {
  pub fn swap<RetType, T: QOpenGLDebugMessage_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_swap<RetType> {
  fn swap(self , rsthis: & QOpenGLDebugMessage) -> RetType;
}

  // proto:  void QOpenGLDebugMessage::swap(QOpenGLDebugMessage & debugMessage);
impl<'a> /*trait*/ QOpenGLDebugMessage_swap<()> for (&'a QOpenGLDebugMessage) {
  fn swap(self , rsthis: & QOpenGLDebugMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLDebugMessage4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN19QOpenGLDebugMessage4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugLogger {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLDebugLogger {
    return QOpenGLDebugLogger{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLDebugLogger {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QOpenGLDebugLogger {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLDebugLogger::~QOpenGLDebugLogger();
impl /*struct*/ QOpenGLDebugLogger {
  pub fn free<RetType, T: QOpenGLDebugLogger_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_free<RetType> {
  fn free(self , rsthis: & QOpenGLDebugLogger) -> RetType;
}

  // proto:  void QOpenGLDebugLogger::~QOpenGLDebugLogger();
impl<'a> /*trait*/ QOpenGLDebugLogger_free<()> for () {
  fn free(self , rsthis: & QOpenGLDebugLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLoggerD2Ev()};
     unsafe {C_ZN18QOpenGLDebugLoggerD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qint64 QOpenGLDebugLogger::maximumMessageLength();
impl /*struct*/ QOpenGLDebugLogger {
  pub fn maximumMessageLength<RetType, T: QOpenGLDebugLogger_maximumMessageLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumMessageLength(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_maximumMessageLength<RetType> {
  fn maximumMessageLength(self , rsthis: & QOpenGLDebugLogger) -> RetType;
}

  // proto:  qint64 QOpenGLDebugLogger::maximumMessageLength();
impl<'a> /*trait*/ QOpenGLDebugLogger_maximumMessageLength<i64> for () {
  fn maximumMessageLength(self , rsthis: & QOpenGLDebugLogger) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLDebugLogger20maximumMessageLengthEv()};
    let mut ret = unsafe {C_ZNK18QOpenGLDebugLogger20maximumMessageLengthEv(rsthis.qclsinst)};
    return ret as i64; // 1
    // return 1;
  }
}

  // proto:  bool QOpenGLDebugLogger::isLogging();
impl /*struct*/ QOpenGLDebugLogger {
  pub fn isLogging<RetType, T: QOpenGLDebugLogger_isLogging<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLogging(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_isLogging<RetType> {
  fn isLogging(self , rsthis: & QOpenGLDebugLogger) -> RetType;
}

  // proto:  bool QOpenGLDebugLogger::isLogging();
impl<'a> /*trait*/ QOpenGLDebugLogger_isLogging<i8> for () {
  fn isLogging(self , rsthis: & QOpenGLDebugLogger) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLDebugLogger9isLoggingEv()};
    let mut ret = unsafe {C_ZNK18QOpenGLDebugLogger9isLoggingEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QOpenGLDebugLogger::stopLogging();
impl /*struct*/ QOpenGLDebugLogger {
  pub fn stopLogging<RetType, T: QOpenGLDebugLogger_stopLogging<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stopLogging(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_stopLogging<RetType> {
  fn stopLogging(self , rsthis: & QOpenGLDebugLogger) -> RetType;
}

  // proto:  void QOpenGLDebugLogger::stopLogging();
impl<'a> /*trait*/ QOpenGLDebugLogger_stopLogging<()> for () {
  fn stopLogging(self , rsthis: & QOpenGLDebugLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLogger11stopLoggingEv()};
     unsafe {C_ZN18QOpenGLDebugLogger11stopLoggingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLDebugLogger::logMessage(const QOpenGLDebugMessage & debugMessage);
impl /*struct*/ QOpenGLDebugLogger {
  pub fn logMessage<RetType, T: QOpenGLDebugLogger_logMessage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.logMessage(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_logMessage<RetType> {
  fn logMessage(self , rsthis: & QOpenGLDebugLogger) -> RetType;
}

  // proto:  void QOpenGLDebugLogger::logMessage(const QOpenGLDebugMessage & debugMessage);
impl<'a> /*trait*/ QOpenGLDebugLogger_logMessage<()> for (&'a QOpenGLDebugMessage) {
  fn logMessage(self , rsthis: & QOpenGLDebugLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLogger10logMessageERK19QOpenGLDebugMessage()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QOpenGLDebugLogger10logMessageERK19QOpenGLDebugMessage(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLDebugLogger::QOpenGLDebugLogger(QObject * parent);
impl /*struct*/ QOpenGLDebugLogger {
  pub fn new<T: QOpenGLDebugLogger_new>(value: T) -> QOpenGLDebugLogger {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_new {
  fn new(self) -> QOpenGLDebugLogger;
}

  // proto:  void QOpenGLDebugLogger::QOpenGLDebugLogger(QObject * parent);
impl<'a> /*trait*/ QOpenGLDebugLogger_new for (&'a QObject) {
  fn new(self) -> QOpenGLDebugLogger {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLoggerC2EP7QObject()};
    let ctysz: c_int = unsafe{QOpenGLDebugLogger_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QOpenGLDebugLoggerC2EP7QObject(arg0)};
    let rsthis = QOpenGLDebugLogger{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLDebugLogger::metaObject();
impl /*struct*/ QOpenGLDebugLogger {
  pub fn metaObject<RetType, T: QOpenGLDebugLogger_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOpenGLDebugLogger) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLDebugLogger::metaObject();
impl<'a> /*trait*/ QOpenGLDebugLogger_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QOpenGLDebugLogger) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLDebugLogger10metaObjectEv()};
    let mut ret = unsafe {C_ZNK18QOpenGLDebugLogger10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QOpenGLDebugMessage> QOpenGLDebugLogger::loggedMessages();
impl /*struct*/ QOpenGLDebugLogger {
  pub fn loggedMessages<RetType, T: QOpenGLDebugLogger_loggedMessages<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loggedMessages(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_loggedMessages<RetType> {
  fn loggedMessages(self , rsthis: & QOpenGLDebugLogger) -> RetType;
}

  // proto:  QList<QOpenGLDebugMessage> QOpenGLDebugLogger::loggedMessages();
impl<'a> /*trait*/ QOpenGLDebugLogger_loggedMessages<u64> for () {
  fn loggedMessages(self , rsthis: & QOpenGLDebugLogger) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLDebugLogger14loggedMessagesEv()};
    let mut ret = unsafe {C_ZNK18QOpenGLDebugLogger14loggedMessagesEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QOpenGLDebugLogger::popGroup();
impl /*struct*/ QOpenGLDebugLogger {
  pub fn popGroup<RetType, T: QOpenGLDebugLogger_popGroup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.popGroup(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_popGroup<RetType> {
  fn popGroup(self , rsthis: & QOpenGLDebugLogger) -> RetType;
}

  // proto:  void QOpenGLDebugLogger::popGroup();
impl<'a> /*trait*/ QOpenGLDebugLogger_popGroup<()> for () {
  fn popGroup(self , rsthis: & QOpenGLDebugLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLogger8popGroupEv()};
     unsafe {C_ZN18QOpenGLDebugLogger8popGroupEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLDebugLogger::initialize();
impl /*struct*/ QOpenGLDebugLogger {
  pub fn initialize<RetType, T: QOpenGLDebugLogger_initialize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.initialize(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_initialize<RetType> {
  fn initialize(self , rsthis: & QOpenGLDebugLogger) -> RetType;
}

  // proto:  bool QOpenGLDebugLogger::initialize();
impl<'a> /*trait*/ QOpenGLDebugLogger_initialize<i8> for () {
  fn initialize(self , rsthis: & QOpenGLDebugLogger) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLogger10initializeEv()};
    let mut ret = unsafe {C_ZN18QOpenGLDebugLogger10initializeEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

#[derive(Default)] // for QOpenGLDebugLogger_messageLogged
pub struct QOpenGLDebugLogger_messageLogged_signal{poi:u64}
impl /* struct */ QOpenGLDebugLogger {
  pub fn messageLogged(&self) -> QOpenGLDebugLogger_messageLogged_signal {
     return QOpenGLDebugLogger_messageLogged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QOpenGLDebugLogger_messageLogged_signal {
  pub fn connect<T: QOpenGLDebugLogger_messageLogged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QOpenGLDebugLogger_messageLogged_signal_connect {
  fn connect(self, sigthis: QOpenGLDebugLogger_messageLogged_signal);
}

// messageLogged(const class QOpenGLDebugMessage &)
extern fn QOpenGLDebugLogger_messageLogged_signal_connect_cb_0(rsfptr:fn(QOpenGLDebugMessage), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QOpenGLDebugMessage::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QOpenGLDebugLogger_messageLogged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QOpenGLDebugMessage)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QOpenGLDebugMessage::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QOpenGLDebugLogger_messageLogged_signal_connect for fn(QOpenGLDebugMessage) {
  fn connect(self, sigthis: QOpenGLDebugLogger_messageLogged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLDebugLogger_messageLogged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QOpenGLDebugLogger_SlotProxy_connect__ZN18QOpenGLDebugLogger13messageLoggedERK19QOpenGLDebugMessage(arg0, arg1, arg2)};
  }
}
impl /* trait */ QOpenGLDebugLogger_messageLogged_signal_connect for Box<Fn(QOpenGLDebugMessage)> {
  fn connect(self, sigthis: QOpenGLDebugLogger_messageLogged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLDebugLogger_messageLogged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QOpenGLDebugLogger_SlotProxy_connect__ZN18QOpenGLDebugLogger13messageLoggedERK19QOpenGLDebugMessage(arg0, arg1, arg2)};
  }
}
// <= body block end

