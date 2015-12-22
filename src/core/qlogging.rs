// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtCore/qlogging.h
// dst-file: /src/core/qlogging.rs
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
use super::qloggingcategory::QLoggingCategory; // 773
use super::qdebug::QDebug; // 773
use super::qdebug::QNoDebug; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QMessageLogContext::QMessageLogContext(const char * fileName, int lineNumber, const char * functionName, const char * categoryName);
  fn _ZN18QMessageLogContextC1EPKciS1_S1_(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int, arg2: *mut c_char, arg3: *mut c_char);
  // proto:  void QMessageLogContext::copy(const QMessageLogContext & logContext);
  fn _ZN18QMessageLogContext4copyERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMessageLogContext::QMessageLogContext();
  fn _ZN18QMessageLogContextC1Ev(qthis: *mut c_void);
  // proto:  void QMessageLogContext::QMessageLogContext(const QMessageLogContext & );
  fn _ZN18QMessageLogContextC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMessageLogger::info(const QLoggingCategory & cat, const char * msg);
  fn _ZNK14QMessageLogger4infoERK16QLoggingCategoryPKcz(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  void QMessageLogger::debug(const char * msg);
  fn _ZNK14QMessageLogger5debugEPKcz(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  void QMessageLogger::warning(const QLoggingCategory & cat, const char * msg);
  fn _ZNK14QMessageLogger7warningERK16QLoggingCategoryPKcz(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  void QMessageLogger::fatal(const char * msg);
  fn _ZNK14QMessageLogger5fatalEPKcz(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  QDebug QMessageLogger::info(const QLoggingCategory & cat);
  fn _ZNK14QMessageLogger4infoERK16QLoggingCategory(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMessageLogger::debug(const QLoggingCategory & cat, const char * msg);
  fn _ZNK14QMessageLogger5debugERK16QLoggingCategoryPKcz(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  void QMessageLogger::critical(const char * msg);
  fn _ZNK14QMessageLogger8criticalEPKcz(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  void QMessageLogger::QMessageLogger();
  fn _ZN14QMessageLoggerC1Ev(qthis: *mut c_void);
  // proto:  QDebug QMessageLogger::info();
  fn _ZNK14QMessageLogger4infoEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMessageLogger::warning(const char * msg);
  fn _ZNK14QMessageLogger7warningEPKcz(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  QDebug QMessageLogger::critical(const QLoggingCategory & cat);
  fn _ZNK14QMessageLogger8criticalERK16QLoggingCategory(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QDebug QMessageLogger::critical();
  fn _ZNK14QMessageLogger8criticalEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMessageLogger::QMessageLogger(const char * file, int line, const char * function);
  fn _ZN14QMessageLoggerC1EPKciS1_(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int, arg2: *mut c_char);
  // proto:  QDebug QMessageLogger::debug();
  fn _ZNK14QMessageLogger5debugEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QDebug QMessageLogger::debug(const QLoggingCategory & cat);
  fn _ZNK14QMessageLogger5debugERK16QLoggingCategory(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMessageLogger::QMessageLogger(const QMessageLogger & );
  fn _ZN14QMessageLoggerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QDebug QMessageLogger::warning(const QLoggingCategory & cat);
  fn _ZNK14QMessageLogger7warningERK16QLoggingCategory(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMessageLogger::info(const char * msg);
  fn _ZNK14QMessageLogger4infoEPKcz(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  void QMessageLogger::noDebug(const char * );
  fn _ZNK14QMessageLogger7noDebugEPKcz(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  QDebug QMessageLogger::warning();
  fn _ZNK14QMessageLogger7warningEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QNoDebug QMessageLogger::noDebug();
  fn _ZNK14QMessageLogger7noDebugEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMessageLogger::critical(const QLoggingCategory & cat, const char * msg);
  fn _ZNK14QMessageLogger8criticalERK16QLoggingCategoryPKcz(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  void QMessageLogger::QMessageLogger(const char * file, int line, const char * function, const char * category);
  fn _ZN14QMessageLoggerC1EPKciS1_S1_(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int, arg2: *mut c_char, arg3: *mut c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QMessageLogContext)=32
pub struct QMessageLogContext {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QMessageLogger)=32
pub struct QMessageLogger {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMessageLogContext {
  pub fn inheritFrom(qthis: *mut c_void) -> QMessageLogContext {
    return QMessageLogContext{qclsinst: qthis};
  }
}
  // proto:  void QMessageLogContext::QMessageLogContext(const char * fileName, int lineNumber, const char * functionName, const char * categoryName);
impl /*struct*/ QMessageLogContext {
  pub fn NewQMessageLogContext<T: QMessageLogContext_NewQMessageLogContext>(value: T) -> QMessageLogContext {
    let rsthis = value.NewQMessageLogContext();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageLogContext_NewQMessageLogContext {
  fn NewQMessageLogContext(self) -> QMessageLogContext;
}

  // proto:  void QMessageLogContext::QMessageLogContext(const char * fileName, int lineNumber, const char * functionName, const char * categoryName);
impl<'a> /*trait*/ QMessageLogContext_NewQMessageLogContext for (&'a  String, i32, &'a  String, &'a  String) {
  fn NewQMessageLogContext(self) -> QMessageLogContext {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QMessageLogContextC1EPKciS1_S1_()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    let arg3 = self.3.as_ptr()  as *mut c_char;
    unsafe {_ZN18QMessageLogContextC1EPKciS1_S1_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QMessageLogContext{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMessageLogContext::copy(const QMessageLogContext & logContext);
impl /*struct*/ QMessageLogContext {
  pub fn copy<RetType, T: QMessageLogContext_copy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.copy(self);
    // return 1;
  }
}

pub trait QMessageLogContext_copy<RetType> {
  fn copy(self , rsthis: &mut QMessageLogContext) -> RetType;
}

  // proto:  void QMessageLogContext::copy(const QMessageLogContext & logContext);
impl<'a> /*trait*/ QMessageLogContext_copy<()> for (QMessageLogContext) {
  fn copy(self , rsthis: &mut QMessageLogContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QMessageLogContext4copyERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QMessageLogContext4copyERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMessageLogContext::QMessageLogContext();
impl<'a> /*trait*/ QMessageLogContext_NewQMessageLogContext for () {
  fn NewQMessageLogContext(self) -> QMessageLogContext {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QMessageLogContextC1Ev()};
    unsafe {_ZN18QMessageLogContextC1Ev(qthis)};
    let rsthis = QMessageLogContext{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMessageLogContext::QMessageLogContext(const QMessageLogContext & );
impl<'a> /*trait*/ QMessageLogContext_NewQMessageLogContext for (QMessageLogContext) {
  fn NewQMessageLogContext(self) -> QMessageLogContext {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QMessageLogContextC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QMessageLogContextC1ERKS_(qthis, arg0)};
    let rsthis = QMessageLogContext{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMessageLogger {
  pub fn inheritFrom(qthis: *mut c_void) -> QMessageLogger {
    return QMessageLogger{qclsinst: qthis};
  }
}
  // proto:  void QMessageLogger::info(const QLoggingCategory & cat, const char * msg);
impl /*struct*/ QMessageLogger {
  pub fn info<RetType, T: QMessageLogger_info<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.info(self);
    // return 1;
  }
}

pub trait QMessageLogger_info<RetType> {
  fn info(self , rsthis: &mut QMessageLogger) -> RetType;
}

  // proto:  void QMessageLogger::info(const QLoggingCategory & cat, const char * msg);
impl<'a> /*trait*/ QMessageLogger_info<()> for (QLoggingCategory, &'a  String) {
  fn info(self , rsthis: &mut QMessageLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger4infoERK16QLoggingCategoryPKcz()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZNK14QMessageLogger4infoERK16QLoggingCategoryPKcz(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QMessageLogger::debug(const char * msg);
impl /*struct*/ QMessageLogger {
  pub fn debug<RetType, T: QMessageLogger_debug<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.debug(self);
    // return 1;
  }
}

pub trait QMessageLogger_debug<RetType> {
  fn debug(self , rsthis: &mut QMessageLogger) -> RetType;
}

  // proto:  void QMessageLogger::debug(const char * msg);
impl<'a> /*trait*/ QMessageLogger_debug<()> for (&'a  String) {
  fn debug(self , rsthis: &mut QMessageLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger5debugEPKcz()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZNK14QMessageLogger5debugEPKcz(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMessageLogger::warning(const QLoggingCategory & cat, const char * msg);
impl /*struct*/ QMessageLogger {
  pub fn warning<RetType, T: QMessageLogger_warning<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.warning(self);
    // return 1;
  }
}

pub trait QMessageLogger_warning<RetType> {
  fn warning(self , rsthis: &mut QMessageLogger) -> RetType;
}

  // proto:  void QMessageLogger::warning(const QLoggingCategory & cat, const char * msg);
impl<'a> /*trait*/ QMessageLogger_warning<()> for (QLoggingCategory, &'a  String) {
  fn warning(self , rsthis: &mut QMessageLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger7warningERK16QLoggingCategoryPKcz()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZNK14QMessageLogger7warningERK16QLoggingCategoryPKcz(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QMessageLogger::fatal(const char * msg);
impl /*struct*/ QMessageLogger {
  pub fn fatal<RetType, T: QMessageLogger_fatal<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fatal(self);
    // return 1;
  }
}

pub trait QMessageLogger_fatal<RetType> {
  fn fatal(self , rsthis: &mut QMessageLogger) -> RetType;
}

  // proto:  void QMessageLogger::fatal(const char * msg);
impl<'a> /*trait*/ QMessageLogger_fatal<()> for (&'a  String) {
  fn fatal(self , rsthis: &mut QMessageLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger5fatalEPKcz()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZNK14QMessageLogger5fatalEPKcz(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QDebug QMessageLogger::info(const QLoggingCategory & cat);
impl<'a> /*trait*/ QMessageLogger_info<QDebug> for (QLoggingCategory) {
  fn info(self , rsthis: &mut QMessageLogger) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger4infoERK16QLoggingCategory()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QMessageLogger4infoERK16QLoggingCategory(rsthis.qclsinst, arg0)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMessageLogger::debug(const QLoggingCategory & cat, const char * msg);
impl<'a> /*trait*/ QMessageLogger_debug<()> for (QLoggingCategory, &'a  String) {
  fn debug(self , rsthis: &mut QMessageLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger5debugERK16QLoggingCategoryPKcz()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZNK14QMessageLogger5debugERK16QLoggingCategoryPKcz(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QMessageLogger::critical(const char * msg);
impl /*struct*/ QMessageLogger {
  pub fn critical<RetType, T: QMessageLogger_critical<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.critical(self);
    // return 1;
  }
}

pub trait QMessageLogger_critical<RetType> {
  fn critical(self , rsthis: &mut QMessageLogger) -> RetType;
}

  // proto:  void QMessageLogger::critical(const char * msg);
impl<'a> /*trait*/ QMessageLogger_critical<()> for (&'a  String) {
  fn critical(self , rsthis: &mut QMessageLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger8criticalEPKcz()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZNK14QMessageLogger8criticalEPKcz(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMessageLogger::QMessageLogger();
impl /*struct*/ QMessageLogger {
  pub fn NewQMessageLogger<T: QMessageLogger_NewQMessageLogger>(value: T) -> QMessageLogger {
    let rsthis = value.NewQMessageLogger();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageLogger_NewQMessageLogger {
  fn NewQMessageLogger(self) -> QMessageLogger;
}

  // proto:  void QMessageLogger::QMessageLogger();
impl<'a> /*trait*/ QMessageLogger_NewQMessageLogger for () {
  fn NewQMessageLogger(self) -> QMessageLogger {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QMessageLoggerC1Ev()};
    unsafe {_ZN14QMessageLoggerC1Ev(qthis)};
    let rsthis = QMessageLogger{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QDebug QMessageLogger::info();
impl<'a> /*trait*/ QMessageLogger_info<QDebug> for () {
  fn info(self , rsthis: &mut QMessageLogger) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger4infoEv()};
    let mut ret = unsafe {_ZNK14QMessageLogger4infoEv(rsthis.qclsinst)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMessageLogger::warning(const char * msg);
impl<'a> /*trait*/ QMessageLogger_warning<()> for (&'a  String) {
  fn warning(self , rsthis: &mut QMessageLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger7warningEPKcz()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZNK14QMessageLogger7warningEPKcz(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QDebug QMessageLogger::critical(const QLoggingCategory & cat);
impl<'a> /*trait*/ QMessageLogger_critical<QDebug> for (QLoggingCategory) {
  fn critical(self , rsthis: &mut QMessageLogger) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger8criticalERK16QLoggingCategory()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QMessageLogger8criticalERK16QLoggingCategory(rsthis.qclsinst, arg0)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QDebug QMessageLogger::critical();
impl<'a> /*trait*/ QMessageLogger_critical<QDebug> for () {
  fn critical(self , rsthis: &mut QMessageLogger) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger8criticalEv()};
    let mut ret = unsafe {_ZNK14QMessageLogger8criticalEv(rsthis.qclsinst)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMessageLogger::QMessageLogger(const char * file, int line, const char * function);
impl<'a> /*trait*/ QMessageLogger_NewQMessageLogger for (&'a  String, i32, &'a  String) {
  fn NewQMessageLogger(self) -> QMessageLogger {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QMessageLoggerC1EPKciS1_()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    unsafe {_ZN14QMessageLoggerC1EPKciS1_(qthis, arg0, arg1, arg2)};
    let rsthis = QMessageLogger{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QDebug QMessageLogger::debug();
impl<'a> /*trait*/ QMessageLogger_debug<QDebug> for () {
  fn debug(self , rsthis: &mut QMessageLogger) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger5debugEv()};
    let mut ret = unsafe {_ZNK14QMessageLogger5debugEv(rsthis.qclsinst)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QDebug QMessageLogger::debug(const QLoggingCategory & cat);
impl<'a> /*trait*/ QMessageLogger_debug<QDebug> for (QLoggingCategory) {
  fn debug(self , rsthis: &mut QMessageLogger) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger5debugERK16QLoggingCategory()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QMessageLogger5debugERK16QLoggingCategory(rsthis.qclsinst, arg0)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMessageLogger::QMessageLogger(const QMessageLogger & );
impl<'a> /*trait*/ QMessageLogger_NewQMessageLogger for (QMessageLogger) {
  fn NewQMessageLogger(self) -> QMessageLogger {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QMessageLoggerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QMessageLoggerC1ERKS_(qthis, arg0)};
    let rsthis = QMessageLogger{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QDebug QMessageLogger::warning(const QLoggingCategory & cat);
impl<'a> /*trait*/ QMessageLogger_warning<QDebug> for (QLoggingCategory) {
  fn warning(self , rsthis: &mut QMessageLogger) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger7warningERK16QLoggingCategory()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QMessageLogger7warningERK16QLoggingCategory(rsthis.qclsinst, arg0)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMessageLogger::info(const char * msg);
impl<'a> /*trait*/ QMessageLogger_info<()> for (&'a  String) {
  fn info(self , rsthis: &mut QMessageLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger4infoEPKcz()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZNK14QMessageLogger4infoEPKcz(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMessageLogger::noDebug(const char * );
impl /*struct*/ QMessageLogger {
  pub fn noDebug<RetType, T: QMessageLogger_noDebug<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.noDebug(self);
    // return 1;
  }
}

pub trait QMessageLogger_noDebug<RetType> {
  fn noDebug(self , rsthis: &mut QMessageLogger) -> RetType;
}

  // proto:  void QMessageLogger::noDebug(const char * );
impl<'a> /*trait*/ QMessageLogger_noDebug<()> for (&'a  String) {
  fn noDebug(self , rsthis: &mut QMessageLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger7noDebugEPKcz()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZNK14QMessageLogger7noDebugEPKcz(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QDebug QMessageLogger::warning();
impl<'a> /*trait*/ QMessageLogger_warning<QDebug> for () {
  fn warning(self , rsthis: &mut QMessageLogger) -> QDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger7warningEv()};
    let mut ret = unsafe {_ZNK14QMessageLogger7warningEv(rsthis.qclsinst)};
    let mut ret1 = QDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QNoDebug QMessageLogger::noDebug();
impl<'a> /*trait*/ QMessageLogger_noDebug<QNoDebug> for () {
  fn noDebug(self , rsthis: &mut QMessageLogger) -> QNoDebug {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger7noDebugEv()};
    let mut ret = unsafe {_ZNK14QMessageLogger7noDebugEv(rsthis.qclsinst)};
    let mut ret1 = QNoDebug::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMessageLogger::critical(const QLoggingCategory & cat, const char * msg);
impl<'a> /*trait*/ QMessageLogger_critical<()> for (QLoggingCategory, &'a  String) {
  fn critical(self , rsthis: &mut QMessageLogger) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMessageLogger8criticalERK16QLoggingCategoryPKcz()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZNK14QMessageLogger8criticalERK16QLoggingCategoryPKcz(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QMessageLogger::QMessageLogger(const char * file, int line, const char * function, const char * category);
impl<'a> /*trait*/ QMessageLogger_NewQMessageLogger for (&'a  String, i32, &'a  String, &'a  String) {
  fn NewQMessageLogger(self) -> QMessageLogger {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QMessageLoggerC1EPKciS1_S1_()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    let arg3 = self.3.as_ptr()  as *mut c_char;
    unsafe {_ZN14QMessageLoggerC1EPKciS1_S1_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QMessageLogger{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

