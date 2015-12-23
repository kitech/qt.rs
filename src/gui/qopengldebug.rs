// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
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
use super::super::core::qstring::QString; // 771
use super::super::core::qobject::QObject; // 771
// use super::qopengldebug::QOpenGLDebugMessage; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage();
  fn _ZN19QOpenGLDebugMessageC1Ev(qthis: *mut c_void);
  // proto:  void QOpenGLDebugMessage::~QOpenGLDebugMessage();
  fn _ZN19QOpenGLDebugMessageD0Ev(qthis: *mut c_void);
  // proto:  GLuint QOpenGLDebugMessage::id();
  fn _ZNK19QOpenGLDebugMessage2idEv(qthis: *mut c_void);
  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage(const QOpenGLDebugMessage & debugMessage);
  fn _ZN19QOpenGLDebugMessageC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QOpenGLDebugMessage::message();
  fn _ZNK19QOpenGLDebugMessage7messageEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOpenGLDebugMessage::swap(QOpenGLDebugMessage & debugMessage);
  fn _ZN19QOpenGLDebugMessage4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLDebugLogger::~QOpenGLDebugLogger();
  fn _ZN18QOpenGLDebugLoggerD0Ev(qthis: *mut c_void);
  // proto:  qint64 QOpenGLDebugLogger::maximumMessageLength();
  fn _ZNK18QOpenGLDebugLogger20maximumMessageLengthEv(qthis: *mut c_void) -> c_longlong;
  // proto:  bool QOpenGLDebugLogger::isLogging();
  fn _ZNK18QOpenGLDebugLogger9isLoggingEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLDebugLogger::stopLogging();
  fn _ZN18QOpenGLDebugLogger11stopLoggingEv(qthis: *mut c_void);
  // proto:  void QOpenGLDebugLogger::logMessage(const QOpenGLDebugMessage & debugMessage);
  fn _ZN18QOpenGLDebugLogger10logMessageERK19QOpenGLDebugMessage(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLDebugLogger::QOpenGLDebugLogger(const QOpenGLDebugLogger & );
  fn _ZN18QOpenGLDebugLoggerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLDebugLogger::messageLogged(const QOpenGLDebugMessage & debugMessage);
  fn _ZN18QOpenGLDebugLogger13messageLoggedERK19QOpenGLDebugMessage(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLDebugLogger::QOpenGLDebugLogger(QObject * parent);
  fn _ZN18QOpenGLDebugLoggerC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QOpenGLDebugLogger::metaObject();
  fn _ZNK18QOpenGLDebugLogger10metaObjectEv(qthis: *mut c_void);
  // proto:  QList<QOpenGLDebugMessage> QOpenGLDebugLogger::loggedMessages();
  fn _ZNK18QOpenGLDebugLogger14loggedMessagesEv(qthis: *mut c_void);
  // proto:  void QOpenGLDebugLogger::popGroup();
  fn _ZN18QOpenGLDebugLogger8popGroupEv(qthis: *mut c_void);
  // proto:  bool QOpenGLDebugLogger::initialize();
  fn _ZN18QOpenGLDebugLogger10initializeEv(qthis: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLDebugMessage)=1
pub struct QOpenGLDebugMessage {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLDebugLogger)=1
pub struct QOpenGLDebugLogger {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLDebugMessage {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLDebugMessage {
    return QOpenGLDebugMessage{qclsinst: qthis};
  }
}
  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage();
impl /*struct*/ QOpenGLDebugMessage {
  pub fn New<T: QOpenGLDebugMessage_New>(value: T) -> QOpenGLDebugMessage {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_New {
  fn New(self) -> QOpenGLDebugMessage;
}

  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage();
impl<'a> /*trait*/ QOpenGLDebugMessage_New for () {
  fn New(self) -> QOpenGLDebugMessage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLDebugMessageC1Ev()};
    unsafe {_ZN19QOpenGLDebugMessageC1Ev(qthis)};
    let rsthis = QOpenGLDebugMessage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLDebugMessage::~QOpenGLDebugMessage();
impl /*struct*/ QOpenGLDebugMessage {
  pub fn Free<RetType, T: QOpenGLDebugMessage_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLDebugMessage) -> RetType;
}

  // proto:  void QOpenGLDebugMessage::~QOpenGLDebugMessage();
impl<'a> /*trait*/ QOpenGLDebugMessage_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLDebugMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLDebugMessageD0Ev()};
     unsafe {_ZN19QOpenGLDebugMessageD0Ev(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QOpenGLDebugMessage_id<()> for () {
  fn id(self , rsthis: & QOpenGLDebugMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLDebugMessage2idEv()};
     unsafe {_ZNK19QOpenGLDebugMessage2idEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage(const QOpenGLDebugMessage & debugMessage);
impl<'a> /*trait*/ QOpenGLDebugMessage_New for (&'a QOpenGLDebugMessage) {
  fn New(self) -> QOpenGLDebugMessage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLDebugMessageC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QOpenGLDebugMessageC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLDebugMessage{qclsinst: qthis};
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
    let mut ret = unsafe {_ZNK19QOpenGLDebugMessage7messageEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
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
     unsafe {_ZN19QOpenGLDebugMessage4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugLogger {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLDebugLogger {
    return QOpenGLDebugLogger{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
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
  pub fn Free<RetType, T: QOpenGLDebugLogger_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLDebugLogger) -> RetType;
}

  // proto:  void QOpenGLDebugLogger::~QOpenGLDebugLogger();
impl<'a> /*trait*/ QOpenGLDebugLogger_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLDebugLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLoggerD0Ev()};
     unsafe {_ZN18QOpenGLDebugLoggerD0Ev(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK18QOpenGLDebugLogger20maximumMessageLengthEv(rsthis.qclsinst)};
    return ret as i64;
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
    let mut ret = unsafe {_ZNK18QOpenGLDebugLogger9isLoggingEv(rsthis.qclsinst)};
    return ret as i8;
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
     unsafe {_ZN18QOpenGLDebugLogger11stopLoggingEv(rsthis.qclsinst)};
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
     unsafe {_ZN18QOpenGLDebugLogger10logMessageERK19QOpenGLDebugMessage(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLDebugLogger::QOpenGLDebugLogger(const QOpenGLDebugLogger & );
impl /*struct*/ QOpenGLDebugLogger {
  pub fn New<T: QOpenGLDebugLogger_New>(value: T) -> QOpenGLDebugLogger {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_New {
  fn New(self) -> QOpenGLDebugLogger;
}

  // proto:  void QOpenGLDebugLogger::QOpenGLDebugLogger(const QOpenGLDebugLogger & );
impl<'a> /*trait*/ QOpenGLDebugLogger_New for (&'a QOpenGLDebugLogger) {
  fn New(self) -> QOpenGLDebugLogger {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLoggerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QOpenGLDebugLoggerC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLDebugLogger{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLDebugLogger::messageLogged(const QOpenGLDebugMessage & debugMessage);
impl /*struct*/ QOpenGLDebugLogger {
  pub fn messageLogged<RetType, T: QOpenGLDebugLogger_messageLogged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.messageLogged(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_messageLogged<RetType> {
  fn messageLogged(self , rsthis: & QOpenGLDebugLogger) -> RetType;
}

  // proto:  void QOpenGLDebugLogger::messageLogged(const QOpenGLDebugMessage & debugMessage);
impl<'a> /*trait*/ QOpenGLDebugLogger_messageLogged<()> for (&'a QOpenGLDebugMessage) {
  fn messageLogged(self , rsthis: & QOpenGLDebugLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLogger13messageLoggedERK19QOpenGLDebugMessage()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QOpenGLDebugLogger13messageLoggedERK19QOpenGLDebugMessage(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLDebugLogger::QOpenGLDebugLogger(QObject * parent);
impl<'a> /*trait*/ QOpenGLDebugLogger_New for (&'a QObject) {
  fn New(self) -> QOpenGLDebugLogger {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLoggerC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QOpenGLDebugLoggerC1EP7QObject(qthis, arg0)};
    let rsthis = QOpenGLDebugLogger{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
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
impl<'a> /*trait*/ QOpenGLDebugLogger_metaObject<()> for () {
  fn metaObject(self , rsthis: & QOpenGLDebugLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLDebugLogger10metaObjectEv()};
     unsafe {_ZNK18QOpenGLDebugLogger10metaObjectEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QOpenGLDebugLogger_loggedMessages<()> for () {
  fn loggedMessages(self , rsthis: & QOpenGLDebugLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLDebugLogger14loggedMessagesEv()};
     unsafe {_ZNK18QOpenGLDebugLogger14loggedMessagesEv(rsthis.qclsinst)};
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
     unsafe {_ZN18QOpenGLDebugLogger8popGroupEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZN18QOpenGLDebugLogger10initializeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

