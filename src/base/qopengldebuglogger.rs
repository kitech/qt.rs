// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qopengldebugmessage::QOpenGLDebugMessage;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QOpenGLDebugLogger::FreeQOpenGLDebugLogger();
  fn _ZN18QOpenGLDebugLoggerD0Ev(qthis: *mut c_void) ;
  // proto:  long long QOpenGLDebugLogger::maximumMessageLength();
  fn _ZNK18QOpenGLDebugLogger20maximumMessageLengthEv(qthis: *mut c_void) -> c_longlong;
  // proto:  bool QOpenGLDebugLogger::isLogging();
  fn _ZNK18QOpenGLDebugLogger9isLoggingEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLDebugLogger::stopLogging();
  fn _ZN18QOpenGLDebugLogger11stopLoggingEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLDebugLogger::logMessage(const QOpenGLDebugMessage & debugMessage);
  fn _ZN18QOpenGLDebugLogger10logMessageERK19QOpenGLDebugMessage(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLDebugLogger::NewQOpenGLDebugLogger(const QOpenGLDebugLogger & );
  fn _ZN18QOpenGLDebugLoggerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLDebugLogger::messageLogged(const QOpenGLDebugMessage & debugMessage);
  fn _ZN18QOpenGLDebugLogger13messageLoggedERK19QOpenGLDebugMessage(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLDebugLogger::NewQOpenGLDebugLogger(QObject * parent);
  fn _ZN18QOpenGLDebugLoggerC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QOpenGLDebugLogger::metaObject();
  fn _ZNK18QOpenGLDebugLogger10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QList<QOpenGLDebugMessage> QOpenGLDebugLogger::loggedMessages();
  fn _ZNK18QOpenGLDebugLogger14loggedMessagesEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLDebugLogger::popGroup();
  fn _ZN18QOpenGLDebugLogger8popGroupEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLDebugLogger::initialize();
  fn _ZN18QOpenGLDebugLogger10initializeEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QOpenGLDebugLogger)=1
pub struct QOpenGLDebugLogger {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLDebugLogger {
  pub fn FreeQOpenGLDebugLogger<T: QOpenGLDebugLogger_FreeQOpenGLDebugLogger>(&mut self, value: T)  {
     value.FreeQOpenGLDebugLogger(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_FreeQOpenGLDebugLogger {
  fn FreeQOpenGLDebugLogger(self, rsthis: &mut QOpenGLDebugLogger) ;
}

// proto:  void QOpenGLDebugLogger::FreeQOpenGLDebugLogger();
impl<'a> /*trait*/ QOpenGLDebugLogger_FreeQOpenGLDebugLogger for () {
  fn FreeQOpenGLDebugLogger(self, rsthis: &mut QOpenGLDebugLogger)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLoggerD0Ev()};
     unsafe {_ZN18QOpenGLDebugLoggerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugLogger {
  pub fn maximumMessageLength<T: QOpenGLDebugLogger_maximumMessageLength>(&mut self, value: T) -> i64 {
    return value.maximumMessageLength(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_maximumMessageLength {
  fn maximumMessageLength(self, rsthis: &mut QOpenGLDebugLogger) -> i64;
}

// proto:  long long QOpenGLDebugLogger::maximumMessageLength();
impl<'a> /*trait*/ QOpenGLDebugLogger_maximumMessageLength for () {
  fn maximumMessageLength(self, rsthis: &mut QOpenGLDebugLogger) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLDebugLogger20maximumMessageLengthEv()};
    let mut ret = unsafe {_ZNK18QOpenGLDebugLogger20maximumMessageLengthEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugLogger {
  pub fn isLogging<T: QOpenGLDebugLogger_isLogging>(&mut self, value: T) -> i8 {
    return value.isLogging(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_isLogging {
  fn isLogging(self, rsthis: &mut QOpenGLDebugLogger) -> i8;
}

// proto:  bool QOpenGLDebugLogger::isLogging();
impl<'a> /*trait*/ QOpenGLDebugLogger_isLogging for () {
  fn isLogging(self, rsthis: &mut QOpenGLDebugLogger) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLDebugLogger9isLoggingEv()};
    let mut ret = unsafe {_ZNK18QOpenGLDebugLogger9isLoggingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugLogger {
  pub fn stopLogging<T: QOpenGLDebugLogger_stopLogging>(&mut self, value: T)  {
     value.stopLogging(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_stopLogging {
  fn stopLogging(self, rsthis: &mut QOpenGLDebugLogger) ;
}

// proto:  void QOpenGLDebugLogger::stopLogging();
impl<'a> /*trait*/ QOpenGLDebugLogger_stopLogging for () {
  fn stopLogging(self, rsthis: &mut QOpenGLDebugLogger)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLogger11stopLoggingEv()};
     unsafe {_ZN18QOpenGLDebugLogger11stopLoggingEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugLogger {
  pub fn logMessage<T: QOpenGLDebugLogger_logMessage>(&mut self, value: T)  {
     value.logMessage(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_logMessage {
  fn logMessage(self, rsthis: &mut QOpenGLDebugLogger) ;
}

// proto:  void QOpenGLDebugLogger::logMessage(const QOpenGLDebugMessage & debugMessage);
impl<'a> /*trait*/ QOpenGLDebugLogger_logMessage for (&'a  QOpenGLDebugMessage) {
  fn logMessage(self, rsthis: &mut QOpenGLDebugLogger)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLogger10logMessageERK19QOpenGLDebugMessage()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QOpenGLDebugLogger10logMessageERK19QOpenGLDebugMessage(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugLogger {
  pub fn NewQOpenGLDebugLogger<T: QOpenGLDebugLogger_NewQOpenGLDebugLogger>(value: T) -> QOpenGLDebugLogger {
    let rsthis = value.NewQOpenGLDebugLogger();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_NewQOpenGLDebugLogger {
  fn NewQOpenGLDebugLogger(self) -> QOpenGLDebugLogger;
}

// proto: void QOpenGLDebugLogger::NewQOpenGLDebugLogger(const QOpenGLDebugLogger & );
impl<'a> /*trait*/ QOpenGLDebugLogger_NewQOpenGLDebugLogger for (&'a  QOpenGLDebugLogger) {
  fn NewQOpenGLDebugLogger(self) -> QOpenGLDebugLogger {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLoggerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QOpenGLDebugLoggerC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLDebugLogger{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugLogger {
  pub fn messageLogged<T: QOpenGLDebugLogger_messageLogged>(&mut self, value: T)  {
     value.messageLogged(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_messageLogged {
  fn messageLogged(self, rsthis: &mut QOpenGLDebugLogger) ;
}

// proto:  void QOpenGLDebugLogger::messageLogged(const QOpenGLDebugMessage & debugMessage);
impl<'a> /*trait*/ QOpenGLDebugLogger_messageLogged for (&'a  QOpenGLDebugMessage) {
  fn messageLogged(self, rsthis: &mut QOpenGLDebugLogger)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLogger13messageLoggedERK19QOpenGLDebugMessage()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QOpenGLDebugLogger13messageLoggedERK19QOpenGLDebugMessage(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QOpenGLDebugLogger::NewQOpenGLDebugLogger(QObject * parent);
impl<'a> /*trait*/ QOpenGLDebugLogger_NewQOpenGLDebugLogger for (&'a mut QObject) {
  fn NewQOpenGLDebugLogger(self) -> QOpenGLDebugLogger {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLoggerC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QOpenGLDebugLoggerC1EP7QObject(qthis, arg0)};
    let rsthis = QOpenGLDebugLogger{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugLogger {
  pub fn metaObject<T: QOpenGLDebugLogger_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_metaObject {
  fn metaObject(self, rsthis: &mut QOpenGLDebugLogger) ;
}

// proto:  const QMetaObject * QOpenGLDebugLogger::metaObject();
impl<'a> /*trait*/ QOpenGLDebugLogger_metaObject for () {
  fn metaObject(self, rsthis: &mut QOpenGLDebugLogger)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLDebugLogger10metaObjectEv()};
     unsafe {_ZNK18QOpenGLDebugLogger10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugLogger {
  pub fn loggedMessages<T: QOpenGLDebugLogger_loggedMessages>(&mut self, value: T)  {
     value.loggedMessages(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_loggedMessages {
  fn loggedMessages(self, rsthis: &mut QOpenGLDebugLogger) ;
}

// proto:  QList<QOpenGLDebugMessage> QOpenGLDebugLogger::loggedMessages();
impl<'a> /*trait*/ QOpenGLDebugLogger_loggedMessages for () {
  fn loggedMessages(self, rsthis: &mut QOpenGLDebugLogger)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QOpenGLDebugLogger14loggedMessagesEv()};
     unsafe {_ZNK18QOpenGLDebugLogger14loggedMessagesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugLogger {
  pub fn popGroup<T: QOpenGLDebugLogger_popGroup>(&mut self, value: T)  {
     value.popGroup(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_popGroup {
  fn popGroup(self, rsthis: &mut QOpenGLDebugLogger) ;
}

// proto:  void QOpenGLDebugLogger::popGroup();
impl<'a> /*trait*/ QOpenGLDebugLogger_popGroup for () {
  fn popGroup(self, rsthis: &mut QOpenGLDebugLogger)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLogger8popGroupEv()};
     unsafe {_ZN18QOpenGLDebugLogger8popGroupEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugLogger {
  pub fn initialize<T: QOpenGLDebugLogger_initialize>(&mut self, value: T) -> i8 {
    return value.initialize(self);
    // return 1;
  }
}

pub trait QOpenGLDebugLogger_initialize {
  fn initialize(self, rsthis: &mut QOpenGLDebugLogger) -> i8;
}

// proto:  bool QOpenGLDebugLogger::initialize();
impl<'a> /*trait*/ QOpenGLDebugLogger_initialize for () {
  fn initialize(self, rsthis: &mut QOpenGLDebugLogger) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QOpenGLDebugLogger10initializeEv()};
    let mut ret = unsafe {_ZN18QOpenGLDebugLogger10initializeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

