// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtCore/qprocess.h
// dst-file: /src/core/qprocess.rs
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
use super::qiodevice::QIODevice; // 773
use std::ops::Deref;
use super::qstringlist::QStringList; // 773
use super::qstring::QString; // 773
// use super::qprocess::QProcessEnvironment; // 773
use super::qbytearray::QByteArray; // 773
use super::qobject::QObject; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QProcess_Class_Size() -> c_int;
  // proto:  void QProcess::close();
  fn _ZN8QProcess5closeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QProcess::setEnvironment(const QStringList & environment);
  fn _ZN8QProcess14setEnvironmentERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static bool QProcess::startDetached(const QString & program, const QStringList & arguments);
  fn _ZN8QProcess13startDetachedERK7QStringRK11QStringList(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static bool QProcess::startDetached(const QString & command);
  fn _ZN8QProcess13startDetachedERK7QString(arg0: *mut c_void) -> c_char;
  // proto:  bool QProcess::atEnd();
  fn _ZNK8QProcess5atEndEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QStringList QProcess::systemEnvironment();
  fn _ZN8QProcess17systemEnvironmentEv();
  // proto:  void QProcess::setProcessEnvironment(const QProcessEnvironment & environment);
  fn _ZN8QProcess21setProcessEnvironmentERK19QProcessEnvironment(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QProcess::QProcess(const QProcess & );
  fn dector_ZN8QProcessC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN8QProcessC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  Q_PID QProcess::pid();
  fn _ZNK8QProcess3pidEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QProcess::setArguments(const QStringList & arguments);
  fn _ZN8QProcess12setArgumentsERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QProcess::~QProcess();
  fn _ZN8QProcessD0Ev(qthis: u64 /* *mut c_void*/);
  // proto: static int QProcess::execute(const QString & command);
  fn _ZN8QProcess7executeERK7QString(arg0: *mut c_void) -> c_int;
  // proto:  void QProcess::closeWriteChannel();
  fn _ZN8QProcess17closeWriteChannelEv(qthis: u64 /* *mut c_void*/);
  // proto: static bool QProcess::startDetached(const QString & program, const QStringList & arguments, const QString & workingDirectory, qint64 * pid);
  fn _ZN8QProcess13startDetachedERK7QStringRK11QStringListS2_Px(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_longlong) -> c_char;
  // proto:  QProcessEnvironment QProcess::processEnvironment();
  fn _ZNK8QProcess18processEnvironmentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QByteArray QProcess::readAllStandardOutput();
  fn _ZN8QProcess21readAllStandardOutputEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QString QProcess::nullDevice();
  fn _ZN8QProcess10nullDeviceEv() -> *mut c_void;
  // proto: static int QProcess::execute(const QString & program, const QStringList & arguments);
  fn _ZN8QProcess7executeERK7QStringRK11QStringList(arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  bool QProcess::waitForBytesWritten(int msecs);
  fn _ZN8QProcess19waitForBytesWrittenEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QProcess::QProcess(QObject * parent);
  fn dector_ZN8QProcessC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN8QProcessC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QProcess::program();
  fn _ZNK8QProcess7programEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qint64 QProcess::processId();
  fn _ZNK8QProcess9processIdEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  QStringList QProcess::arguments();
  fn _ZNK8QProcess9argumentsEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QProcess::isSequential();
  fn _ZNK8QProcess12isSequentialEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QProcess::waitForReadyRead(int msecs);
  fn _ZN8QProcess16waitForReadyReadEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QProcess::setWorkingDirectory(const QString & dir);
  fn _ZN8QProcess19setWorkingDirectoryERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QProcess::terminate();
  fn _ZN8QProcess9terminateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QProcess::kill();
  fn _ZN8QProcess4killEv(qthis: u64 /* *mut c_void*/);
  // proto:  qint64 QProcess::bytesAvailable();
  fn _ZNK8QProcess14bytesAvailableEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  const QMetaObject * QProcess::metaObject();
  fn _ZNK8QProcess10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QProcess::waitForStarted(int msecs);
  fn _ZN8QProcess14waitForStartedEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  QByteArray QProcess::readAllStandardError();
  fn _ZN8QProcess20readAllStandardErrorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QProcess::exitCode();
  fn _ZNK8QProcess8exitCodeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QProcess::finished(int exitCode);
  fn _ZN8QProcess8finishedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QStringList QProcess::environment();
  fn _ZNK8QProcess11environmentEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QProcess::canReadLine();
  fn _ZNK8QProcess11canReadLineEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QProcess::setStandardOutputProcess(QProcess * destination);
  fn _ZN8QProcess24setStandardOutputProcessEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QProcess::waitForFinished(int msecs);
  fn _ZN8QProcess15waitForFinishedEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  qint64 QProcess::bytesToWrite();
  fn _ZNK8QProcess12bytesToWriteEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  QString QProcess::workingDirectory();
  fn _ZNK8QProcess16workingDirectoryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QProcess::setProgram(const QString & program);
  fn _ZN8QProcess10setProgramERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QProcess::setStandardInputFile(const QString & fileName);
  fn _ZN8QProcess20setStandardInputFileERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QProcessEnvironment_Class_Size() -> c_int;
  // proto:  bool QProcessEnvironment::contains(const QString & name);
  fn _ZNK19QProcessEnvironment8containsERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QStringList QProcessEnvironment::keys();
  fn _ZNK19QProcessEnvironment4keysEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QProcessEnvironment::remove(const QString & name);
  fn _ZN19QProcessEnvironment6removeERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QProcessEnvironment::clear();
  fn _ZN19QProcessEnvironment5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QProcessEnvironment::value(const QString & name, const QString & defaultValue);
  fn _ZNK19QProcessEnvironment5valueERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QProcessEnvironment::isEmpty();
  fn _ZNK19QProcessEnvironment7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QProcessEnvironment::~QProcessEnvironment();
  fn _ZN19QProcessEnvironmentD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QProcessEnvironment::swap(QProcessEnvironment & other);
  fn demth_ZN19QProcessEnvironment4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QProcessEnvironment::QProcessEnvironment(const QProcessEnvironment & other);
  fn dector_ZN19QProcessEnvironmentC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QProcessEnvironmentC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QProcessEnvironment QProcessEnvironment::systemEnvironment();
  fn _ZN19QProcessEnvironment17systemEnvironmentEv() -> *mut c_void;
  // proto:  void QProcessEnvironment::insert(const QString & name, const QString & value);
  fn _ZN19QProcessEnvironment6insertERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QStringList QProcessEnvironment::toStringList();
  fn _ZNK19QProcessEnvironment12toStringListEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QProcessEnvironment::QProcessEnvironment();
  fn dector_ZN19QProcessEnvironmentC1Ev() -> *mut c_void;
  fn _ZN19QProcessEnvironmentC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QProcessEnvironment::insert(const QProcessEnvironment & e);
  fn _ZN19QProcessEnvironment6insertERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QProcess_SlotProxy_connect__ZN8QProcess5errorENS_12ProcessErrorE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QProcess_SlotProxy_connect__ZN8QProcess8finishedEiNS_10ExitStatusE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QProcess_SlotProxy_connect__ZN8QProcess8finishedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QProcess)=1
#[derive(Default)]
pub struct QProcess {
  qbase: QIODevice,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _stateChanged_1: QProcess_stateChanged_signal,
  pub _started_1: QProcess_started_signal,
  pub _finished_1: QProcess_finished_signal,
  pub _readyReadStandardError_1: QProcess_readyReadStandardError_signal,
  pub _error_1: QProcess_error_signal,
  pub _readyReadStandardOutput_1: QProcess_readyReadStandardOutput_signal,
}

// class sizeof(QProcessEnvironment)=1
#[derive(Default)]
pub struct QProcessEnvironment {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QProcess {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QProcess {
    return QProcess{qbase: QIODevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QProcess {
  type Target = QIODevice;

  fn deref(&self) -> &QIODevice {
    return & self.qbase;
  }
}
impl AsRef<QIODevice> for QProcess {
  fn as_ref(& self) -> & QIODevice {
    return & self.qbase;
  }
}
  // proto:  void QProcess::close();
impl /*struct*/ QProcess {
  pub fn close<RetType, T: QProcess_close<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QProcess_close<RetType> {
  fn close(self , rsthis: & QProcess) -> RetType;
}

  // proto:  void QProcess::close();
impl<'a> /*trait*/ QProcess_close<()> for () {
  fn close(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess5closeEv()};
     unsafe {_ZN8QProcess5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QProcess::setEnvironment(const QStringList & environment);
impl /*struct*/ QProcess {
  pub fn setEnvironment<RetType, T: QProcess_setEnvironment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEnvironment(self);
    // return 1;
  }
}

pub trait QProcess_setEnvironment<RetType> {
  fn setEnvironment(self , rsthis: & QProcess) -> RetType;
}

  // proto:  void QProcess::setEnvironment(const QStringList & environment);
impl<'a> /*trait*/ QProcess_setEnvironment<()> for (&'a QStringList) {
  fn setEnvironment(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess14setEnvironmentERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess14setEnvironmentERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static bool QProcess::startDetached(const QString & program, const QStringList & arguments);
impl /*struct*/ QProcess {
  pub fn startDetached_s<RetType, T: QProcess_startDetached_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.startDetached_s();
    // return 1;
  }
}

pub trait QProcess_startDetached_s<RetType> {
  fn startDetached_s(self ) -> RetType;
}

  // proto: static bool QProcess::startDetached(const QString & program, const QStringList & arguments);
impl<'a> /*trait*/ QProcess_startDetached_s<i8> for (&'a QString, &'a QStringList) {
  fn startDetached_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess13startDetachedERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QProcess13startDetachedERK7QStringRK11QStringList(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QProcess::startDetached(const QString & command);
impl<'a> /*trait*/ QProcess_startDetached_s<i8> for (&'a QString) {
  fn startDetached_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess13startDetachedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QProcess13startDetachedERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QProcess::atEnd();
impl /*struct*/ QProcess {
  pub fn atEnd<RetType, T: QProcess_atEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.atEnd(self);
    // return 1;
  }
}

pub trait QProcess_atEnd<RetType> {
  fn atEnd(self , rsthis: & QProcess) -> RetType;
}

  // proto:  bool QProcess::atEnd();
impl<'a> /*trait*/ QProcess_atEnd<i8> for () {
  fn atEnd(self , rsthis: & QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess5atEndEv()};
    let mut ret = unsafe {_ZNK8QProcess5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QStringList QProcess::systemEnvironment();
impl /*struct*/ QProcess {
  pub fn systemEnvironment_s<RetType, T: QProcess_systemEnvironment_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.systemEnvironment_s();
    // return 1;
  }
}

pub trait QProcess_systemEnvironment_s<RetType> {
  fn systemEnvironment_s(self ) -> RetType;
}

  // proto: static QStringList QProcess::systemEnvironment();
impl<'a> /*trait*/ QProcess_systemEnvironment_s<()> for () {
  fn systemEnvironment_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess17systemEnvironmentEv()};
     unsafe {_ZN8QProcess17systemEnvironmentEv()};
    // return 1;
  }
}

  // proto:  void QProcess::setProcessEnvironment(const QProcessEnvironment & environment);
impl /*struct*/ QProcess {
  pub fn setProcessEnvironment<RetType, T: QProcess_setProcessEnvironment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProcessEnvironment(self);
    // return 1;
  }
}

pub trait QProcess_setProcessEnvironment<RetType> {
  fn setProcessEnvironment(self , rsthis: & QProcess) -> RetType;
}

  // proto:  void QProcess::setProcessEnvironment(const QProcessEnvironment & environment);
impl<'a> /*trait*/ QProcess_setProcessEnvironment<()> for (&'a QProcessEnvironment) {
  fn setProcessEnvironment(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess21setProcessEnvironmentERK19QProcessEnvironment()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess21setProcessEnvironmentERK19QProcessEnvironment(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProcess::QProcess(const QProcess & );
impl /*struct*/ QProcess {
  pub fn New<T: QProcess_New>(value: T) -> QProcess {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QProcess_New {
  fn New(self) -> QProcess;
}

  // proto:  void QProcess::QProcess(const QProcess & );
impl<'a> /*trait*/ QProcess_New for (&'a QProcess) {
  fn New(self) -> QProcess {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcessC1ERKS_()};
    let ctysz: c_int = unsafe{QProcess_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN8QProcessC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN8QProcessC1ERKS_(arg0)} as u64;
    let rsthis = QProcess{qbase: QIODevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  Q_PID QProcess::pid();
impl /*struct*/ QProcess {
  pub fn pid<RetType, T: QProcess_pid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pid(self);
    // return 1;
  }
}

pub trait QProcess_pid<RetType> {
  fn pid(self , rsthis: & QProcess) -> RetType;
}

  // proto:  Q_PID QProcess::pid();
impl<'a> /*trait*/ QProcess_pid<i64> for () {
  fn pid(self , rsthis: & QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess3pidEv()};
    let mut ret = unsafe {_ZNK8QProcess3pidEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QProcess::setArguments(const QStringList & arguments);
impl /*struct*/ QProcess {
  pub fn setArguments<RetType, T: QProcess_setArguments<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setArguments(self);
    // return 1;
  }
}

pub trait QProcess_setArguments<RetType> {
  fn setArguments(self , rsthis: & QProcess) -> RetType;
}

  // proto:  void QProcess::setArguments(const QStringList & arguments);
impl<'a> /*trait*/ QProcess_setArguments<()> for (&'a QStringList) {
  fn setArguments(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess12setArgumentsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess12setArgumentsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProcess::~QProcess();
impl /*struct*/ QProcess {
  pub fn Free<RetType, T: QProcess_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QProcess_Free<RetType> {
  fn Free(self , rsthis: & QProcess) -> RetType;
}

  // proto:  void QProcess::~QProcess();
impl<'a> /*trait*/ QProcess_Free<()> for () {
  fn Free(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcessD0Ev()};
     unsafe {_ZN8QProcessD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static int QProcess::execute(const QString & command);
impl /*struct*/ QProcess {
  pub fn execute_s<RetType, T: QProcess_execute_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.execute_s();
    // return 1;
  }
}

pub trait QProcess_execute_s<RetType> {
  fn execute_s(self ) -> RetType;
}

  // proto: static int QProcess::execute(const QString & command);
impl<'a> /*trait*/ QProcess_execute_s<i32> for (&'a QString) {
  fn execute_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess7executeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QProcess7executeERK7QString(arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QProcess::closeWriteChannel();
impl /*struct*/ QProcess {
  pub fn closeWriteChannel<RetType, T: QProcess_closeWriteChannel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.closeWriteChannel(self);
    // return 1;
  }
}

pub trait QProcess_closeWriteChannel<RetType> {
  fn closeWriteChannel(self , rsthis: & QProcess) -> RetType;
}

  // proto:  void QProcess::closeWriteChannel();
impl<'a> /*trait*/ QProcess_closeWriteChannel<()> for () {
  fn closeWriteChannel(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess17closeWriteChannelEv()};
     unsafe {_ZN8QProcess17closeWriteChannelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static bool QProcess::startDetached(const QString & program, const QStringList & arguments, const QString & workingDirectory, qint64 * pid);
impl<'a> /*trait*/ QProcess_startDetached_s<i8> for (&'a QString, &'a QStringList, &'a QString, &'a mut Vec<i64>) {
  fn startDetached_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess13startDetachedERK7QStringRK11QStringListS2_Px()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.as_ptr()  as *mut c_longlong;
    let mut ret = unsafe {_ZN8QProcess13startDetachedERK7QStringRK11QStringListS2_Px(arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QProcessEnvironment QProcess::processEnvironment();
impl /*struct*/ QProcess {
  pub fn processEnvironment<RetType, T: QProcess_processEnvironment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.processEnvironment(self);
    // return 1;
  }
}

pub trait QProcess_processEnvironment<RetType> {
  fn processEnvironment(self , rsthis: & QProcess) -> RetType;
}

  // proto:  QProcessEnvironment QProcess::processEnvironment();
impl<'a> /*trait*/ QProcess_processEnvironment<QProcessEnvironment> for () {
  fn processEnvironment(self , rsthis: & QProcess) -> QProcessEnvironment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess18processEnvironmentEv()};
    let mut ret = unsafe {_ZNK8QProcess18processEnvironmentEv(rsthis.qclsinst)};
    let mut ret1 = QProcessEnvironment::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QProcess::readAllStandardOutput();
impl /*struct*/ QProcess {
  pub fn readAllStandardOutput<RetType, T: QProcess_readAllStandardOutput<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readAllStandardOutput(self);
    // return 1;
  }
}

pub trait QProcess_readAllStandardOutput<RetType> {
  fn readAllStandardOutput(self , rsthis: & QProcess) -> RetType;
}

  // proto:  QByteArray QProcess::readAllStandardOutput();
impl<'a> /*trait*/ QProcess_readAllStandardOutput<QByteArray> for () {
  fn readAllStandardOutput(self , rsthis: & QProcess) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess21readAllStandardOutputEv()};
    let mut ret = unsafe {_ZN8QProcess21readAllStandardOutputEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QProcess::nullDevice();
impl /*struct*/ QProcess {
  pub fn nullDevice_s<RetType, T: QProcess_nullDevice_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.nullDevice_s();
    // return 1;
  }
}

pub trait QProcess_nullDevice_s<RetType> {
  fn nullDevice_s(self ) -> RetType;
}

  // proto: static QString QProcess::nullDevice();
impl<'a> /*trait*/ QProcess_nullDevice_s<QString> for () {
  fn nullDevice_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess10nullDeviceEv()};
    let mut ret = unsafe {_ZN8QProcess10nullDeviceEv()};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static int QProcess::execute(const QString & program, const QStringList & arguments);
impl<'a> /*trait*/ QProcess_execute_s<i32> for (&'a QString, &'a QStringList) {
  fn execute_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess7executeERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QProcess7executeERK7QStringRK11QStringList(arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QProcess::waitForBytesWritten(int msecs);
impl /*struct*/ QProcess {
  pub fn waitForBytesWritten<RetType, T: QProcess_waitForBytesWritten<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForBytesWritten(self);
    // return 1;
  }
}

pub trait QProcess_waitForBytesWritten<RetType> {
  fn waitForBytesWritten(self , rsthis: & QProcess) -> RetType;
}

  // proto:  bool QProcess::waitForBytesWritten(int msecs);
impl<'a> /*trait*/ QProcess_waitForBytesWritten<i8> for (i32) {
  fn waitForBytesWritten(self , rsthis: & QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess19waitForBytesWrittenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN8QProcess19waitForBytesWrittenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QProcess::QProcess(QObject * parent);
impl<'a> /*trait*/ QProcess_New for (&'a QObject) {
  fn New(self) -> QProcess {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcessC1EP7QObject()};
    let ctysz: c_int = unsafe{QProcess_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN8QProcessC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN8QProcessC1EP7QObject(arg0)} as u64;
    let rsthis = QProcess{qbase: QIODevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QProcess::program();
impl /*struct*/ QProcess {
  pub fn program<RetType, T: QProcess_program<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.program(self);
    // return 1;
  }
}

pub trait QProcess_program<RetType> {
  fn program(self , rsthis: & QProcess) -> RetType;
}

  // proto:  QString QProcess::program();
impl<'a> /*trait*/ QProcess_program<QString> for () {
  fn program(self , rsthis: & QProcess) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess7programEv()};
    let mut ret = unsafe {_ZNK8QProcess7programEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qint64 QProcess::processId();
impl /*struct*/ QProcess {
  pub fn processId<RetType, T: QProcess_processId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.processId(self);
    // return 1;
  }
}

pub trait QProcess_processId<RetType> {
  fn processId(self , rsthis: & QProcess) -> RetType;
}

  // proto:  qint64 QProcess::processId();
impl<'a> /*trait*/ QProcess_processId<i64> for () {
  fn processId(self , rsthis: & QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess9processIdEv()};
    let mut ret = unsafe {_ZNK8QProcess9processIdEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QStringList QProcess::arguments();
impl /*struct*/ QProcess {
  pub fn arguments<RetType, T: QProcess_arguments<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.arguments(self);
    // return 1;
  }
}

pub trait QProcess_arguments<RetType> {
  fn arguments(self , rsthis: & QProcess) -> RetType;
}

  // proto:  QStringList QProcess::arguments();
impl<'a> /*trait*/ QProcess_arguments<()> for () {
  fn arguments(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess9argumentsEv()};
     unsafe {_ZNK8QProcess9argumentsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QProcess::isSequential();
impl /*struct*/ QProcess {
  pub fn isSequential<RetType, T: QProcess_isSequential<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSequential(self);
    // return 1;
  }
}

pub trait QProcess_isSequential<RetType> {
  fn isSequential(self , rsthis: & QProcess) -> RetType;
}

  // proto:  bool QProcess::isSequential();
impl<'a> /*trait*/ QProcess_isSequential<i8> for () {
  fn isSequential(self , rsthis: & QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess12isSequentialEv()};
    let mut ret = unsafe {_ZNK8QProcess12isSequentialEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QProcess::waitForReadyRead(int msecs);
impl /*struct*/ QProcess {
  pub fn waitForReadyRead<RetType, T: QProcess_waitForReadyRead<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForReadyRead(self);
    // return 1;
  }
}

pub trait QProcess_waitForReadyRead<RetType> {
  fn waitForReadyRead(self , rsthis: & QProcess) -> RetType;
}

  // proto:  bool QProcess::waitForReadyRead(int msecs);
impl<'a> /*trait*/ QProcess_waitForReadyRead<i8> for (i32) {
  fn waitForReadyRead(self , rsthis: & QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess16waitForReadyReadEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN8QProcess16waitForReadyReadEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QProcess::setWorkingDirectory(const QString & dir);
impl /*struct*/ QProcess {
  pub fn setWorkingDirectory<RetType, T: QProcess_setWorkingDirectory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWorkingDirectory(self);
    // return 1;
  }
}

pub trait QProcess_setWorkingDirectory<RetType> {
  fn setWorkingDirectory(self , rsthis: & QProcess) -> RetType;
}

  // proto:  void QProcess::setWorkingDirectory(const QString & dir);
impl<'a> /*trait*/ QProcess_setWorkingDirectory<()> for (&'a QString) {
  fn setWorkingDirectory(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess19setWorkingDirectoryERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess19setWorkingDirectoryERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProcess::terminate();
impl /*struct*/ QProcess {
  pub fn terminate<RetType, T: QProcess_terminate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.terminate(self);
    // return 1;
  }
}

pub trait QProcess_terminate<RetType> {
  fn terminate(self , rsthis: & QProcess) -> RetType;
}

  // proto:  void QProcess::terminate();
impl<'a> /*trait*/ QProcess_terminate<()> for () {
  fn terminate(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess9terminateEv()};
     unsafe {_ZN8QProcess9terminateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QProcess::kill();
impl /*struct*/ QProcess {
  pub fn kill<RetType, T: QProcess_kill<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.kill(self);
    // return 1;
  }
}

pub trait QProcess_kill<RetType> {
  fn kill(self , rsthis: & QProcess) -> RetType;
}

  // proto:  void QProcess::kill();
impl<'a> /*trait*/ QProcess_kill<()> for () {
  fn kill(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess4killEv()};
     unsafe {_ZN8QProcess4killEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qint64 QProcess::bytesAvailable();
impl /*struct*/ QProcess {
  pub fn bytesAvailable<RetType, T: QProcess_bytesAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesAvailable(self);
    // return 1;
  }
}

pub trait QProcess_bytesAvailable<RetType> {
  fn bytesAvailable(self , rsthis: & QProcess) -> RetType;
}

  // proto:  qint64 QProcess::bytesAvailable();
impl<'a> /*trait*/ QProcess_bytesAvailable<i64> for () {
  fn bytesAvailable(self , rsthis: & QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess14bytesAvailableEv()};
    let mut ret = unsafe {_ZNK8QProcess14bytesAvailableEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  const QMetaObject * QProcess::metaObject();
impl /*struct*/ QProcess {
  pub fn metaObject<RetType, T: QProcess_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QProcess_metaObject<RetType> {
  fn metaObject(self , rsthis: & QProcess) -> RetType;
}

  // proto:  const QMetaObject * QProcess::metaObject();
impl<'a> /*trait*/ QProcess_metaObject<()> for () {
  fn metaObject(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess10metaObjectEv()};
     unsafe {_ZNK8QProcess10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QProcess::waitForStarted(int msecs);
impl /*struct*/ QProcess {
  pub fn waitForStarted<RetType, T: QProcess_waitForStarted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForStarted(self);
    // return 1;
  }
}

pub trait QProcess_waitForStarted<RetType> {
  fn waitForStarted(self , rsthis: & QProcess) -> RetType;
}

  // proto:  bool QProcess::waitForStarted(int msecs);
impl<'a> /*trait*/ QProcess_waitForStarted<i8> for (i32) {
  fn waitForStarted(self , rsthis: & QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess14waitForStartedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN8QProcess14waitForStartedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray QProcess::readAllStandardError();
impl /*struct*/ QProcess {
  pub fn readAllStandardError<RetType, T: QProcess_readAllStandardError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readAllStandardError(self);
    // return 1;
  }
}

pub trait QProcess_readAllStandardError<RetType> {
  fn readAllStandardError(self , rsthis: & QProcess) -> RetType;
}

  // proto:  QByteArray QProcess::readAllStandardError();
impl<'a> /*trait*/ QProcess_readAllStandardError<QByteArray> for () {
  fn readAllStandardError(self , rsthis: & QProcess) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess20readAllStandardErrorEv()};
    let mut ret = unsafe {_ZN8QProcess20readAllStandardErrorEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QProcess::exitCode();
impl /*struct*/ QProcess {
  pub fn exitCode<RetType, T: QProcess_exitCode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exitCode(self);
    // return 1;
  }
}

pub trait QProcess_exitCode<RetType> {
  fn exitCode(self , rsthis: & QProcess) -> RetType;
}

  // proto:  int QProcess::exitCode();
impl<'a> /*trait*/ QProcess_exitCode<i32> for () {
  fn exitCode(self , rsthis: & QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess8exitCodeEv()};
    let mut ret = unsafe {_ZNK8QProcess8exitCodeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QProcess::finished(int exitCode);
impl /*struct*/ QProcess {
  pub fn finished<RetType, T: QProcess_finished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.finished(self);
    // return 1;
  }
}

pub trait QProcess_finished<RetType> {
  fn finished(self , rsthis: & QProcess) -> RetType;
}

  // proto:  void QProcess::finished(int exitCode);
impl<'a> /*trait*/ QProcess_finished<()> for (i32) {
  fn finished(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess8finishedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QProcess8finishedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QProcess::environment();
impl /*struct*/ QProcess {
  pub fn environment<RetType, T: QProcess_environment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.environment(self);
    // return 1;
  }
}

pub trait QProcess_environment<RetType> {
  fn environment(self , rsthis: & QProcess) -> RetType;
}

  // proto:  QStringList QProcess::environment();
impl<'a> /*trait*/ QProcess_environment<()> for () {
  fn environment(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess11environmentEv()};
     unsafe {_ZNK8QProcess11environmentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QProcess::canReadLine();
impl /*struct*/ QProcess {
  pub fn canReadLine<RetType, T: QProcess_canReadLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canReadLine(self);
    // return 1;
  }
}

pub trait QProcess_canReadLine<RetType> {
  fn canReadLine(self , rsthis: & QProcess) -> RetType;
}

  // proto:  bool QProcess::canReadLine();
impl<'a> /*trait*/ QProcess_canReadLine<i8> for () {
  fn canReadLine(self , rsthis: & QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess11canReadLineEv()};
    let mut ret = unsafe {_ZNK8QProcess11canReadLineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QProcess::setStandardOutputProcess(QProcess * destination);
impl /*struct*/ QProcess {
  pub fn setStandardOutputProcess<RetType, T: QProcess_setStandardOutputProcess<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStandardOutputProcess(self);
    // return 1;
  }
}

pub trait QProcess_setStandardOutputProcess<RetType> {
  fn setStandardOutputProcess(self , rsthis: & QProcess) -> RetType;
}

  // proto:  void QProcess::setStandardOutputProcess(QProcess * destination);
impl<'a> /*trait*/ QProcess_setStandardOutputProcess<()> for (&'a QProcess) {
  fn setStandardOutputProcess(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess24setStandardOutputProcessEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess24setStandardOutputProcessEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QProcess::waitForFinished(int msecs);
impl /*struct*/ QProcess {
  pub fn waitForFinished<RetType, T: QProcess_waitForFinished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForFinished(self);
    // return 1;
  }
}

pub trait QProcess_waitForFinished<RetType> {
  fn waitForFinished(self , rsthis: & QProcess) -> RetType;
}

  // proto:  bool QProcess::waitForFinished(int msecs);
impl<'a> /*trait*/ QProcess_waitForFinished<i8> for (i32) {
  fn waitForFinished(self , rsthis: & QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess15waitForFinishedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN8QProcess15waitForFinishedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QProcess::bytesToWrite();
impl /*struct*/ QProcess {
  pub fn bytesToWrite<RetType, T: QProcess_bytesToWrite<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesToWrite(self);
    // return 1;
  }
}

pub trait QProcess_bytesToWrite<RetType> {
  fn bytesToWrite(self , rsthis: & QProcess) -> RetType;
}

  // proto:  qint64 QProcess::bytesToWrite();
impl<'a> /*trait*/ QProcess_bytesToWrite<i64> for () {
  fn bytesToWrite(self , rsthis: & QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess12bytesToWriteEv()};
    let mut ret = unsafe {_ZNK8QProcess12bytesToWriteEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QString QProcess::workingDirectory();
impl /*struct*/ QProcess {
  pub fn workingDirectory<RetType, T: QProcess_workingDirectory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.workingDirectory(self);
    // return 1;
  }
}

pub trait QProcess_workingDirectory<RetType> {
  fn workingDirectory(self , rsthis: & QProcess) -> RetType;
}

  // proto:  QString QProcess::workingDirectory();
impl<'a> /*trait*/ QProcess_workingDirectory<QString> for () {
  fn workingDirectory(self , rsthis: & QProcess) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess16workingDirectoryEv()};
    let mut ret = unsafe {_ZNK8QProcess16workingDirectoryEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProcess::setProgram(const QString & program);
impl /*struct*/ QProcess {
  pub fn setProgram<RetType, T: QProcess_setProgram<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProgram(self);
    // return 1;
  }
}

pub trait QProcess_setProgram<RetType> {
  fn setProgram(self , rsthis: & QProcess) -> RetType;
}

  // proto:  void QProcess::setProgram(const QString & program);
impl<'a> /*trait*/ QProcess_setProgram<()> for (&'a QString) {
  fn setProgram(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess10setProgramERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess10setProgramERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProcess::setStandardInputFile(const QString & fileName);
impl /*struct*/ QProcess {
  pub fn setStandardInputFile<RetType, T: QProcess_setStandardInputFile<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStandardInputFile(self);
    // return 1;
  }
}

pub trait QProcess_setStandardInputFile<RetType> {
  fn setStandardInputFile(self , rsthis: & QProcess) -> RetType;
}

  // proto:  void QProcess::setStandardInputFile(const QString & fileName);
impl<'a> /*trait*/ QProcess_setStandardInputFile<()> for (&'a QString) {
  fn setStandardInputFile(self , rsthis: & QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess20setStandardInputFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess20setStandardInputFileERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcessEnvironment {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QProcessEnvironment {
    return QProcessEnvironment{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QProcessEnvironment::contains(const QString & name);
impl /*struct*/ QProcessEnvironment {
  pub fn contains<RetType, T: QProcessEnvironment_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_contains<RetType> {
  fn contains(self , rsthis: & QProcessEnvironment) -> RetType;
}

  // proto:  bool QProcessEnvironment::contains(const QString & name);
impl<'a> /*trait*/ QProcessEnvironment_contains<i8> for (&'a QString) {
  fn contains(self , rsthis: & QProcessEnvironment) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment8containsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QProcessEnvironment8containsERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QStringList QProcessEnvironment::keys();
impl /*struct*/ QProcessEnvironment {
  pub fn keys<RetType, T: QProcessEnvironment_keys<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keys(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_keys<RetType> {
  fn keys(self , rsthis: & QProcessEnvironment) -> RetType;
}

  // proto:  QStringList QProcessEnvironment::keys();
impl<'a> /*trait*/ QProcessEnvironment_keys<()> for () {
  fn keys(self , rsthis: & QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment4keysEv()};
     unsafe {_ZNK19QProcessEnvironment4keysEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::remove(const QString & name);
impl /*struct*/ QProcessEnvironment {
  pub fn remove<RetType, T: QProcessEnvironment_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_remove<RetType> {
  fn remove(self , rsthis: & QProcessEnvironment) -> RetType;
}

  // proto:  void QProcessEnvironment::remove(const QString & name);
impl<'a> /*trait*/ QProcessEnvironment_remove<()> for (&'a QString) {
  fn remove(self , rsthis: & QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment6removeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QProcessEnvironment6removeERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::clear();
impl /*struct*/ QProcessEnvironment {
  pub fn clear<RetType, T: QProcessEnvironment_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_clear<RetType> {
  fn clear(self , rsthis: & QProcessEnvironment) -> RetType;
}

  // proto:  void QProcessEnvironment::clear();
impl<'a> /*trait*/ QProcessEnvironment_clear<()> for () {
  fn clear(self , rsthis: & QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment5clearEv()};
     unsafe {_ZN19QProcessEnvironment5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QProcessEnvironment::value(const QString & name, const QString & defaultValue);
impl /*struct*/ QProcessEnvironment {
  pub fn value<RetType, T: QProcessEnvironment_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_value<RetType> {
  fn value(self , rsthis: & QProcessEnvironment) -> RetType;
}

  // proto:  QString QProcessEnvironment::value(const QString & name, const QString & defaultValue);
impl<'a> /*trait*/ QProcessEnvironment_value<QString> for (&'a QString, &'a QString) {
  fn value(self , rsthis: & QProcessEnvironment) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment5valueERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QProcessEnvironment5valueERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QProcessEnvironment::isEmpty();
impl /*struct*/ QProcessEnvironment {
  pub fn isEmpty<RetType, T: QProcessEnvironment_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QProcessEnvironment) -> RetType;
}

  // proto:  bool QProcessEnvironment::isEmpty();
impl<'a> /*trait*/ QProcessEnvironment_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QProcessEnvironment) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment7isEmptyEv()};
    let mut ret = unsafe {_ZNK19QProcessEnvironment7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::~QProcessEnvironment();
impl /*struct*/ QProcessEnvironment {
  pub fn Free<RetType, T: QProcessEnvironment_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_Free<RetType> {
  fn Free(self , rsthis: & QProcessEnvironment) -> RetType;
}

  // proto:  void QProcessEnvironment::~QProcessEnvironment();
impl<'a> /*trait*/ QProcessEnvironment_Free<()> for () {
  fn Free(self , rsthis: & QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironmentD0Ev()};
     unsafe {_ZN19QProcessEnvironmentD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::swap(QProcessEnvironment & other);
impl /*struct*/ QProcessEnvironment {
  pub fn swap<RetType, T: QProcessEnvironment_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_swap<RetType> {
  fn swap(self , rsthis: & QProcessEnvironment) -> RetType;
}

  // proto:  void QProcessEnvironment::swap(QProcessEnvironment & other);
impl<'a> /*trait*/ QProcessEnvironment_swap<()> for (&'a QProcessEnvironment) {
  fn swap(self , rsthis: & QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {demth_ZN19QProcessEnvironment4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::QProcessEnvironment(const QProcessEnvironment & other);
impl /*struct*/ QProcessEnvironment {
  pub fn New<T: QProcessEnvironment_New>(value: T) -> QProcessEnvironment {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QProcessEnvironment_New {
  fn New(self) -> QProcessEnvironment;
}

  // proto:  void QProcessEnvironment::QProcessEnvironment(const QProcessEnvironment & other);
impl<'a> /*trait*/ QProcessEnvironment_New for (&'a QProcessEnvironment) {
  fn New(self) -> QProcessEnvironment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironmentC1ERKS_()};
    let ctysz: c_int = unsafe{QProcessEnvironment_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QProcessEnvironmentC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QProcessEnvironmentC1ERKS_(arg0)} as u64;
    let rsthis = QProcessEnvironment{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QProcessEnvironment QProcessEnvironment::systemEnvironment();
impl /*struct*/ QProcessEnvironment {
  pub fn systemEnvironment_s<RetType, T: QProcessEnvironment_systemEnvironment_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.systemEnvironment_s();
    // return 1;
  }
}

pub trait QProcessEnvironment_systemEnvironment_s<RetType> {
  fn systemEnvironment_s(self ) -> RetType;
}

  // proto: static QProcessEnvironment QProcessEnvironment::systemEnvironment();
impl<'a> /*trait*/ QProcessEnvironment_systemEnvironment_s<QProcessEnvironment> for () {
  fn systemEnvironment_s(self ) -> QProcessEnvironment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment17systemEnvironmentEv()};
    let mut ret = unsafe {_ZN19QProcessEnvironment17systemEnvironmentEv()};
    let mut ret1 = QProcessEnvironment::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::insert(const QString & name, const QString & value);
impl /*struct*/ QProcessEnvironment {
  pub fn insert<RetType, T: QProcessEnvironment_insert<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insert(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_insert<RetType> {
  fn insert(self , rsthis: & QProcessEnvironment) -> RetType;
}

  // proto:  void QProcessEnvironment::insert(const QString & name, const QString & value);
impl<'a> /*trait*/ QProcessEnvironment_insert<()> for (&'a QString, &'a QString) {
  fn insert(self , rsthis: & QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment6insertERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QProcessEnvironment6insertERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QStringList QProcessEnvironment::toStringList();
impl /*struct*/ QProcessEnvironment {
  pub fn toStringList<RetType, T: QProcessEnvironment_toStringList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toStringList(self);
    // return 1;
  }
}

pub trait QProcessEnvironment_toStringList<RetType> {
  fn toStringList(self , rsthis: & QProcessEnvironment) -> RetType;
}

  // proto:  QStringList QProcessEnvironment::toStringList();
impl<'a> /*trait*/ QProcessEnvironment_toStringList<()> for () {
  fn toStringList(self , rsthis: & QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QProcessEnvironment12toStringListEv()};
     unsafe {_ZNK19QProcessEnvironment12toStringListEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::QProcessEnvironment();
impl<'a> /*trait*/ QProcessEnvironment_New for () {
  fn New(self) -> QProcessEnvironment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironmentC1Ev()};
    let ctysz: c_int = unsafe{QProcessEnvironment_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN19QProcessEnvironmentC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN19QProcessEnvironmentC1Ev()} as u64;
    let rsthis = QProcessEnvironment{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QProcessEnvironment::insert(const QProcessEnvironment & e);
impl<'a> /*trait*/ QProcessEnvironment_insert<()> for (&'a QProcessEnvironment) {
  fn insert(self , rsthis: & QProcessEnvironment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QProcessEnvironment6insertERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QProcessEnvironment6insertERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QProcess_stateChanged
pub struct QProcess_stateChanged_signal{poi:u64}
impl /* struct */ QProcess {
  pub fn stateChanged_1(&self) -> QProcess_stateChanged_signal {
     return QProcess_stateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QProcess_stateChanged_signal {
  pub fn connect<T: QProcess_stateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QProcess_stateChanged_signal_connect {
  fn connect(self, sigthis: QProcess_stateChanged_signal);
}

#[derive(Default)] // for QProcess_started
pub struct QProcess_started_signal{poi:u64}
impl /* struct */ QProcess {
  pub fn started_1(&self) -> QProcess_started_signal {
     return QProcess_started_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QProcess_started_signal {
  pub fn connect<T: QProcess_started_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QProcess_started_signal_connect {
  fn connect(self, sigthis: QProcess_started_signal);
}

#[derive(Default)] // for QProcess_finished
pub struct QProcess_finished_signal{poi:u64}
impl /* struct */ QProcess {
  pub fn finished_1(&self) -> QProcess_finished_signal {
     return QProcess_finished_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QProcess_finished_signal {
  pub fn connect<T: QProcess_finished_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QProcess_finished_signal_connect {
  fn connect(self, sigthis: QProcess_finished_signal);
}

#[derive(Default)] // for QProcess_readyReadStandardError
pub struct QProcess_readyReadStandardError_signal{poi:u64}
impl /* struct */ QProcess {
  pub fn readyReadStandardError_1(&self) -> QProcess_readyReadStandardError_signal {
     return QProcess_readyReadStandardError_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QProcess_readyReadStandardError_signal {
  pub fn connect<T: QProcess_readyReadStandardError_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QProcess_readyReadStandardError_signal_connect {
  fn connect(self, sigthis: QProcess_readyReadStandardError_signal);
}

#[derive(Default)] // for QProcess_error
pub struct QProcess_error_signal{poi:u64}
impl /* struct */ QProcess {
  pub fn error_1(&self) -> QProcess_error_signal {
     return QProcess_error_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QProcess_error_signal {
  pub fn connect<T: QProcess_error_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QProcess_error_signal_connect {
  fn connect(self, sigthis: QProcess_error_signal);
}

#[derive(Default)] // for QProcess_readyReadStandardOutput
pub struct QProcess_readyReadStandardOutput_signal{poi:u64}
impl /* struct */ QProcess {
  pub fn readyReadStandardOutput_1(&self) -> QProcess_readyReadStandardOutput_signal {
     return QProcess_readyReadStandardOutput_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QProcess_readyReadStandardOutput_signal {
  pub fn connect<T: QProcess_readyReadStandardOutput_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QProcess_readyReadStandardOutput_signal_connect {
  fn connect(self, sigthis: QProcess_readyReadStandardOutput_signal);
}

// error(class QProcess::ProcessError)
extern fn QProcess_error_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QProcess_error_signal_connect_cb_box_0(rsfptr_raw:*mut fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QProcess_error_signal_connect for fn(i32) {
  fn connect(self, sigthis: QProcess_error_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QProcess_error_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QProcess_SlotProxy_connect__ZN8QProcess5errorENS_12ProcessErrorE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QProcess_error_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QProcess_error_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QProcess_error_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QProcess_SlotProxy_connect__ZN8QProcess5errorENS_12ProcessErrorE(arg0, arg1, arg2)};
  }
}
// finished(int, class QProcess::ExitStatus)
extern fn QProcess_finished_signal_connect_cb_1(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QProcess_finished_signal_connect_cb_box_1(rsfptr_raw:*mut fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
impl /* trait */ QProcess_finished_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QProcess_finished_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QProcess_finished_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QProcess_SlotProxy_connect__ZN8QProcess8finishedEiNS_10ExitStatusE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QProcess_finished_signal_connect for Box<fn(i32, i32)> {
  fn connect(self, sigthis: QProcess_finished_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QProcess_finished_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QProcess_SlotProxy_connect__ZN8QProcess8finishedEiNS_10ExitStatusE(arg0, arg1, arg2)};
  }
}
// finished(int)
extern fn QProcess_finished_signal_connect_cb_2(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QProcess_finished_signal_connect_cb_box_2(rsfptr_raw:*mut fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QProcess_finished_signal_connect for fn(i32) {
  fn connect(self, sigthis: QProcess_finished_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QProcess_finished_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QProcess_SlotProxy_connect__ZN8QProcess8finishedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QProcess_finished_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QProcess_finished_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QProcess_finished_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QProcess_SlotProxy_connect__ZN8QProcess8finishedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

