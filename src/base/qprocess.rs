// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstringlist::QStringList;
use super::qstring::QString;
use super::qprocessenvironment::QProcessEnvironment;
use super::qbytearray::QByteArray;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QProcess::close();
  fn _ZN8QProcess5closeEv(qthis: *mut c_void) ;
  // proto:  void QProcess::setEnvironment(const QStringList & environment);
  fn _ZN8QProcess14setEnvironmentERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static bool QProcess::startDetached(const QString & program, const QStringList & arguments);
  fn _ZN8QProcess13startDetachedERK7QStringRK11QStringList(arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto: static bool QProcess::startDetached(const QString & command);
  fn _ZN8QProcess13startDetachedERK7QString(arg0: *mut c_void) -> int8_t;
  // proto:  bool QProcess::atEnd();
  fn _ZNK8QProcess5atEndEv(qthis: *mut c_void) -> int8_t;
  // proto: static QStringList QProcess::systemEnvironment();
  fn _ZN8QProcess17systemEnvironmentEv() ;
  // proto:  void QProcess::setProcessEnvironment(const QProcessEnvironment & environment);
  fn _ZN8QProcess21setProcessEnvironmentERK19QProcessEnvironment(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProcess::NewQProcess(const QProcess & );
  fn _ZN8QProcessC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  qint64 QProcess::pid();
  fn _ZNK8QProcess3pidEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QProcess::setArguments(const QStringList & arguments);
  fn _ZN8QProcess12setArgumentsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProcess::FreeQProcess();
  fn _ZN8QProcessD0Ev(qthis: *mut c_void) ;
  // proto: static int QProcess::execute(const QString & command);
  fn _ZN8QProcess7executeERK7QString(arg0: *mut c_void) -> c_int;
  // proto:  void QProcess::closeWriteChannel();
  fn _ZN8QProcess17closeWriteChannelEv(qthis: *mut c_void) ;
  // proto: static bool QProcess::startDetached(const QString & program, const QStringList & arguments, const QString & workingDirectory, qint64 * pid);
  fn _ZN8QProcess13startDetachedERK7QStringRK11QStringListS2_Px(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_longlong) -> int8_t;
  // proto:  QProcessEnvironment QProcess::processEnvironment();
  fn _ZNK8QProcess18processEnvironmentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QProcess::readAllStandardOutput();
  fn _ZN8QProcess21readAllStandardOutputEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QString QProcess::nullDevice();
  fn _ZN8QProcess10nullDeviceEv() -> *mut c_void;
  // proto: static int QProcess::execute(const QString & program, const QStringList & arguments);
  fn _ZN8QProcess7executeERK7QStringRK11QStringList(arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  bool QProcess::waitForBytesWritten(int msecs);
  fn _ZN8QProcess19waitForBytesWrittenEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QProcess::NewQProcess(QObject * parent);
  fn _ZN8QProcessC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QProcess::program();
  fn _ZNK8QProcess7programEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  long long QProcess::processId();
  fn _ZNK8QProcess9processIdEv(qthis: *mut c_void) -> c_longlong;
  // proto:  QStringList QProcess::arguments();
  fn _ZNK8QProcess9argumentsEv(qthis: *mut c_void) ;
  // proto:  bool QProcess::isSequential();
  fn _ZNK8QProcess12isSequentialEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QProcess::waitForReadyRead(int msecs);
  fn _ZN8QProcess16waitForReadyReadEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QProcess::setWorkingDirectory(const QString & dir);
  fn _ZN8QProcess19setWorkingDirectoryERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProcess::terminate();
  fn _ZN8QProcess9terminateEv(qthis: *mut c_void) ;
  // proto:  void QProcess::kill();
  fn _ZN8QProcess4killEv(qthis: *mut c_void) ;
  // proto:  long long QProcess::bytesAvailable();
  fn _ZNK8QProcess14bytesAvailableEv(qthis: *mut c_void) -> c_longlong;
  // proto:  const QMetaObject * QProcess::metaObject();
  fn _ZNK8QProcess10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QProcess::waitForStarted(int msecs);
  fn _ZN8QProcess14waitForStartedEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  QByteArray QProcess::readAllStandardError();
  fn _ZN8QProcess20readAllStandardErrorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QProcess::exitCode();
  fn _ZNK8QProcess8exitCodeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QProcess::finished(int exitCode);
  fn _ZN8QProcess8finishedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QStringList QProcess::environment();
  fn _ZNK8QProcess11environmentEv(qthis: *mut c_void) ;
  // proto:  bool QProcess::canReadLine();
  fn _ZNK8QProcess11canReadLineEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QProcess::setStandardOutputProcess(QProcess * destination);
  fn _ZN8QProcess24setStandardOutputProcessEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QProcess::waitForFinished(int msecs);
  fn _ZN8QProcess15waitForFinishedEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  long long QProcess::bytesToWrite();
  fn _ZNK8QProcess12bytesToWriteEv(qthis: *mut c_void) -> c_longlong;
  // proto:  QString QProcess::workingDirectory();
  fn _ZNK8QProcess16workingDirectoryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QProcess::setProgram(const QString & program);
  fn _ZN8QProcess10setProgramERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProcess::setStandardInputFile(const QString & fileName);
  fn _ZN8QProcess20setStandardInputFileERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QProcess)=1
pub struct QProcess {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QProcess {
  pub fn close<T: QProcess_close>(&mut self, value: T)  {
     value.close(self);
    // return 1;
  }
}

pub trait QProcess_close {
  fn close(self, rsthis: &mut QProcess) ;
}

// proto:  void QProcess::close();
impl<'a> /*trait*/ QProcess_close for () {
  fn close(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess5closeEv()};
     unsafe {_ZN8QProcess5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setEnvironment<T: QProcess_setEnvironment>(&mut self, value: T)  {
     value.setEnvironment(self);
    // return 1;
  }
}

pub trait QProcess_setEnvironment {
  fn setEnvironment(self, rsthis: &mut QProcess) ;
}

// proto:  void QProcess::setEnvironment(const QStringList & environment);
impl<'a> /*trait*/ QProcess_setEnvironment for (&'a  QStringList) {
  fn setEnvironment(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess14setEnvironmentERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess14setEnvironmentERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn startDetached<T: QProcess_startDetached>(&mut self, value: T) -> i8 {
    return value.startDetached(self);
    // return 1;
  }
}

pub trait QProcess_startDetached {
  fn startDetached(self, rsthis: &mut QProcess) -> i8;
}

// proto: static bool QProcess::startDetached(const QString & program, const QStringList & arguments);
impl<'a> /*trait*/ QProcess_startDetached for (&'a  QString, &'a  QStringList) {
  fn startDetached(self, rsthis: &mut QProcess) -> i8 {
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
impl<'a> /*trait*/ QProcess_startDetached for (&'a  QString) {
  fn startDetached(self, rsthis: &mut QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess13startDetachedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QProcess13startDetachedERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn atEnd<T: QProcess_atEnd>(&mut self, value: T) -> i8 {
    return value.atEnd(self);
    // return 1;
  }
}

pub trait QProcess_atEnd {
  fn atEnd(self, rsthis: &mut QProcess) -> i8;
}

// proto:  bool QProcess::atEnd();
impl<'a> /*trait*/ QProcess_atEnd for () {
  fn atEnd(self, rsthis: &mut QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess5atEndEv()};
    let mut ret = unsafe {_ZNK8QProcess5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn systemEnvironment<T: QProcess_systemEnvironment>(&mut self, value: T)  {
     value.systemEnvironment(self);
    // return 1;
  }
}

pub trait QProcess_systemEnvironment {
  fn systemEnvironment(self, rsthis: &mut QProcess) ;
}

// proto: static QStringList QProcess::systemEnvironment();
impl<'a> /*trait*/ QProcess_systemEnvironment for () {
  fn systemEnvironment(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess17systemEnvironmentEv()};
     unsafe {_ZN8QProcess17systemEnvironmentEv()};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setProcessEnvironment<T: QProcess_setProcessEnvironment>(&mut self, value: T)  {
     value.setProcessEnvironment(self);
    // return 1;
  }
}

pub trait QProcess_setProcessEnvironment {
  fn setProcessEnvironment(self, rsthis: &mut QProcess) ;
}

// proto:  void QProcess::setProcessEnvironment(const QProcessEnvironment & environment);
impl<'a> /*trait*/ QProcess_setProcessEnvironment for (&'a  QProcessEnvironment) {
  fn setProcessEnvironment(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess21setProcessEnvironmentERK19QProcessEnvironment()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess21setProcessEnvironmentERK19QProcessEnvironment(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn NewQProcess<T: QProcess_NewQProcess>(value: T) -> QProcess {
    let rsthis = value.NewQProcess();
    return rsthis;
    // return 1;
  }
}

pub trait QProcess_NewQProcess {
  fn NewQProcess(self) -> QProcess;
}

// proto: void QProcess::NewQProcess(const QProcess & );
impl<'a> /*trait*/ QProcess_NewQProcess for (&'a  QProcess) {
  fn NewQProcess(self) -> QProcess {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcessC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QProcessC1ERKS_(qthis, arg0)};
    let rsthis = QProcess{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn pid<T: QProcess_pid>(&mut self, value: T) -> i64 {
    return value.pid(self);
    // return 1;
  }
}

pub trait QProcess_pid {
  fn pid(self, rsthis: &mut QProcess) -> i64;
}

// proto:  qint64 QProcess::pid();
impl<'a> /*trait*/ QProcess_pid for () {
  fn pid(self, rsthis: &mut QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess3pidEv()};
    let mut ret = unsafe {_ZNK8QProcess3pidEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setArguments<T: QProcess_setArguments>(&mut self, value: T)  {
     value.setArguments(self);
    // return 1;
  }
}

pub trait QProcess_setArguments {
  fn setArguments(self, rsthis: &mut QProcess) ;
}

// proto:  void QProcess::setArguments(const QStringList & arguments);
impl<'a> /*trait*/ QProcess_setArguments for (&'a  QStringList) {
  fn setArguments(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess12setArgumentsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess12setArgumentsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn FreeQProcess<T: QProcess_FreeQProcess>(&mut self, value: T)  {
     value.FreeQProcess(self);
    // return 1;
  }
}

pub trait QProcess_FreeQProcess {
  fn FreeQProcess(self, rsthis: &mut QProcess) ;
}

// proto:  void QProcess::FreeQProcess();
impl<'a> /*trait*/ QProcess_FreeQProcess for () {
  fn FreeQProcess(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcessD0Ev()};
     unsafe {_ZN8QProcessD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn execute<T: QProcess_execute>(&mut self, value: T) -> i32 {
    return value.execute(self);
    // return 1;
  }
}

pub trait QProcess_execute {
  fn execute(self, rsthis: &mut QProcess) -> i32;
}

// proto: static int QProcess::execute(const QString & command);
impl<'a> /*trait*/ QProcess_execute for (&'a  QString) {
  fn execute(self, rsthis: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess7executeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QProcess7executeERK7QString(arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn closeWriteChannel<T: QProcess_closeWriteChannel>(&mut self, value: T)  {
     value.closeWriteChannel(self);
    // return 1;
  }
}

pub trait QProcess_closeWriteChannel {
  fn closeWriteChannel(self, rsthis: &mut QProcess) ;
}

// proto:  void QProcess::closeWriteChannel();
impl<'a> /*trait*/ QProcess_closeWriteChannel for () {
  fn closeWriteChannel(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess17closeWriteChannelEv()};
     unsafe {_ZN8QProcess17closeWriteChannelEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: static bool QProcess::startDetached(const QString & program, const QStringList & arguments, const QString & workingDirectory, qint64 * pid);
impl<'a> /*trait*/ QProcess_startDetached for (&'a  QString, &'a  QStringList, &'a  QString, &'a mut i64) {
  fn startDetached(self, rsthis: &mut QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess13startDetachedERK7QStringRK11QStringListS2_Px()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as *mut c_longlong;
    let mut ret = unsafe {_ZN8QProcess13startDetachedERK7QStringRK11QStringListS2_Px(arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn processEnvironment<T: QProcess_processEnvironment>(&mut self, value: T) -> QProcessEnvironment {
    return value.processEnvironment(self);
    // return 1;
  }
}

pub trait QProcess_processEnvironment {
  fn processEnvironment(self, rsthis: &mut QProcess) -> QProcessEnvironment;
}

// proto:  QProcessEnvironment QProcess::processEnvironment();
impl<'a> /*trait*/ QProcess_processEnvironment for () {
  fn processEnvironment(self, rsthis: &mut QProcess) -> QProcessEnvironment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess18processEnvironmentEv()};
    let mut ret = unsafe {_ZNK8QProcess18processEnvironmentEv(rsthis.qclsinst)};
    let mut ret1 = QProcessEnvironment{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn readAllStandardOutput<T: QProcess_readAllStandardOutput>(&mut self, value: T) -> QByteArray {
    return value.readAllStandardOutput(self);
    // return 1;
  }
}

pub trait QProcess_readAllStandardOutput {
  fn readAllStandardOutput(self, rsthis: &mut QProcess) -> QByteArray;
}

// proto:  QByteArray QProcess::readAllStandardOutput();
impl<'a> /*trait*/ QProcess_readAllStandardOutput for () {
  fn readAllStandardOutput(self, rsthis: &mut QProcess) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess21readAllStandardOutputEv()};
    let mut ret = unsafe {_ZN8QProcess21readAllStandardOutputEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn nullDevice<T: QProcess_nullDevice>(&mut self, value: T) -> QString {
    return value.nullDevice(self);
    // return 1;
  }
}

pub trait QProcess_nullDevice {
  fn nullDevice(self, rsthis: &mut QProcess) -> QString;
}

// proto: static QString QProcess::nullDevice();
impl<'a> /*trait*/ QProcess_nullDevice for () {
  fn nullDevice(self, rsthis: &mut QProcess) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess10nullDeviceEv()};
    let mut ret = unsafe {_ZN8QProcess10nullDeviceEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static int QProcess::execute(const QString & program, const QStringList & arguments);
impl<'a> /*trait*/ QProcess_execute for (&'a  QString, &'a  QStringList) {
  fn execute(self, rsthis: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess7executeERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QProcess7executeERK7QStringRK11QStringList(arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn waitForBytesWritten<T: QProcess_waitForBytesWritten>(&mut self, value: T) -> i8 {
    return value.waitForBytesWritten(self);
    // return 1;
  }
}

pub trait QProcess_waitForBytesWritten {
  fn waitForBytesWritten(self, rsthis: &mut QProcess) -> i8;
}

// proto:  bool QProcess::waitForBytesWritten(int msecs);
impl<'a> /*trait*/ QProcess_waitForBytesWritten for (i32) {
  fn waitForBytesWritten(self, rsthis: &mut QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess19waitForBytesWrittenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN8QProcess19waitForBytesWrittenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QProcess::NewQProcess(QObject * parent);
impl<'a> /*trait*/ QProcess_NewQProcess for (&'a mut QObject) {
  fn NewQProcess(self) -> QProcess {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcessC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QProcessC1EP7QObject(qthis, arg0)};
    let rsthis = QProcess{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn program<T: QProcess_program>(&mut self, value: T) -> QString {
    return value.program(self);
    // return 1;
  }
}

pub trait QProcess_program {
  fn program(self, rsthis: &mut QProcess) -> QString;
}

// proto:  QString QProcess::program();
impl<'a> /*trait*/ QProcess_program for () {
  fn program(self, rsthis: &mut QProcess) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess7programEv()};
    let mut ret = unsafe {_ZNK8QProcess7programEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn processId<T: QProcess_processId>(&mut self, value: T) -> i64 {
    return value.processId(self);
    // return 1;
  }
}

pub trait QProcess_processId {
  fn processId(self, rsthis: &mut QProcess) -> i64;
}

// proto:  long long QProcess::processId();
impl<'a> /*trait*/ QProcess_processId for () {
  fn processId(self, rsthis: &mut QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess9processIdEv()};
    let mut ret = unsafe {_ZNK8QProcess9processIdEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn arguments<T: QProcess_arguments>(&mut self, value: T)  {
     value.arguments(self);
    // return 1;
  }
}

pub trait QProcess_arguments {
  fn arguments(self, rsthis: &mut QProcess) ;
}

// proto:  QStringList QProcess::arguments();
impl<'a> /*trait*/ QProcess_arguments for () {
  fn arguments(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess9argumentsEv()};
     unsafe {_ZNK8QProcess9argumentsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn isSequential<T: QProcess_isSequential>(&mut self, value: T) -> i8 {
    return value.isSequential(self);
    // return 1;
  }
}

pub trait QProcess_isSequential {
  fn isSequential(self, rsthis: &mut QProcess) -> i8;
}

// proto:  bool QProcess::isSequential();
impl<'a> /*trait*/ QProcess_isSequential for () {
  fn isSequential(self, rsthis: &mut QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess12isSequentialEv()};
    let mut ret = unsafe {_ZNK8QProcess12isSequentialEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn waitForReadyRead<T: QProcess_waitForReadyRead>(&mut self, value: T) -> i8 {
    return value.waitForReadyRead(self);
    // return 1;
  }
}

pub trait QProcess_waitForReadyRead {
  fn waitForReadyRead(self, rsthis: &mut QProcess) -> i8;
}

// proto:  bool QProcess::waitForReadyRead(int msecs);
impl<'a> /*trait*/ QProcess_waitForReadyRead for (i32) {
  fn waitForReadyRead(self, rsthis: &mut QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess16waitForReadyReadEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN8QProcess16waitForReadyReadEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setWorkingDirectory<T: QProcess_setWorkingDirectory>(&mut self, value: T)  {
     value.setWorkingDirectory(self);
    // return 1;
  }
}

pub trait QProcess_setWorkingDirectory {
  fn setWorkingDirectory(self, rsthis: &mut QProcess) ;
}

// proto:  void QProcess::setWorkingDirectory(const QString & dir);
impl<'a> /*trait*/ QProcess_setWorkingDirectory for (&'a  QString) {
  fn setWorkingDirectory(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess19setWorkingDirectoryERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess19setWorkingDirectoryERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn terminate<T: QProcess_terminate>(&mut self, value: T)  {
     value.terminate(self);
    // return 1;
  }
}

pub trait QProcess_terminate {
  fn terminate(self, rsthis: &mut QProcess) ;
}

// proto:  void QProcess::terminate();
impl<'a> /*trait*/ QProcess_terminate for () {
  fn terminate(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess9terminateEv()};
     unsafe {_ZN8QProcess9terminateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn kill<T: QProcess_kill>(&mut self, value: T)  {
     value.kill(self);
    // return 1;
  }
}

pub trait QProcess_kill {
  fn kill(self, rsthis: &mut QProcess) ;
}

// proto:  void QProcess::kill();
impl<'a> /*trait*/ QProcess_kill for () {
  fn kill(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess4killEv()};
     unsafe {_ZN8QProcess4killEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn bytesAvailable<T: QProcess_bytesAvailable>(&mut self, value: T) -> i64 {
    return value.bytesAvailable(self);
    // return 1;
  }
}

pub trait QProcess_bytesAvailable {
  fn bytesAvailable(self, rsthis: &mut QProcess) -> i64;
}

// proto:  long long QProcess::bytesAvailable();
impl<'a> /*trait*/ QProcess_bytesAvailable for () {
  fn bytesAvailable(self, rsthis: &mut QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess14bytesAvailableEv()};
    let mut ret = unsafe {_ZNK8QProcess14bytesAvailableEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn metaObject<T: QProcess_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QProcess_metaObject {
  fn metaObject(self, rsthis: &mut QProcess) ;
}

// proto:  const QMetaObject * QProcess::metaObject();
impl<'a> /*trait*/ QProcess_metaObject for () {
  fn metaObject(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess10metaObjectEv()};
     unsafe {_ZNK8QProcess10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn waitForStarted<T: QProcess_waitForStarted>(&mut self, value: T) -> i8 {
    return value.waitForStarted(self);
    // return 1;
  }
}

pub trait QProcess_waitForStarted {
  fn waitForStarted(self, rsthis: &mut QProcess) -> i8;
}

// proto:  bool QProcess::waitForStarted(int msecs);
impl<'a> /*trait*/ QProcess_waitForStarted for (i32) {
  fn waitForStarted(self, rsthis: &mut QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess14waitForStartedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN8QProcess14waitForStartedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn readAllStandardError<T: QProcess_readAllStandardError>(&mut self, value: T) -> QByteArray {
    return value.readAllStandardError(self);
    // return 1;
  }
}

pub trait QProcess_readAllStandardError {
  fn readAllStandardError(self, rsthis: &mut QProcess) -> QByteArray;
}

// proto:  QByteArray QProcess::readAllStandardError();
impl<'a> /*trait*/ QProcess_readAllStandardError for () {
  fn readAllStandardError(self, rsthis: &mut QProcess) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess20readAllStandardErrorEv()};
    let mut ret = unsafe {_ZN8QProcess20readAllStandardErrorEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn exitCode<T: QProcess_exitCode>(&mut self, value: T) -> i32 {
    return value.exitCode(self);
    // return 1;
  }
}

pub trait QProcess_exitCode {
  fn exitCode(self, rsthis: &mut QProcess) -> i32;
}

// proto:  int QProcess::exitCode();
impl<'a> /*trait*/ QProcess_exitCode for () {
  fn exitCode(self, rsthis: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess8exitCodeEv()};
    let mut ret = unsafe {_ZNK8QProcess8exitCodeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn finished<T: QProcess_finished>(&mut self, value: T)  {
     value.finished(self);
    // return 1;
  }
}

pub trait QProcess_finished {
  fn finished(self, rsthis: &mut QProcess) ;
}

// proto:  void QProcess::finished(int exitCode);
impl<'a> /*trait*/ QProcess_finished for (i32) {
  fn finished(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess8finishedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QProcess8finishedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn environment<T: QProcess_environment>(&mut self, value: T)  {
     value.environment(self);
    // return 1;
  }
}

pub trait QProcess_environment {
  fn environment(self, rsthis: &mut QProcess) ;
}

// proto:  QStringList QProcess::environment();
impl<'a> /*trait*/ QProcess_environment for () {
  fn environment(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess11environmentEv()};
     unsafe {_ZNK8QProcess11environmentEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn canReadLine<T: QProcess_canReadLine>(&mut self, value: T) -> i8 {
    return value.canReadLine(self);
    // return 1;
  }
}

pub trait QProcess_canReadLine {
  fn canReadLine(self, rsthis: &mut QProcess) -> i8;
}

// proto:  bool QProcess::canReadLine();
impl<'a> /*trait*/ QProcess_canReadLine for () {
  fn canReadLine(self, rsthis: &mut QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess11canReadLineEv()};
    let mut ret = unsafe {_ZNK8QProcess11canReadLineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setStandardOutputProcess<T: QProcess_setStandardOutputProcess>(&mut self, value: T)  {
     value.setStandardOutputProcess(self);
    // return 1;
  }
}

pub trait QProcess_setStandardOutputProcess {
  fn setStandardOutputProcess(self, rsthis: &mut QProcess) ;
}

// proto:  void QProcess::setStandardOutputProcess(QProcess * destination);
impl<'a> /*trait*/ QProcess_setStandardOutputProcess for (&'a mut QProcess) {
  fn setStandardOutputProcess(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess24setStandardOutputProcessEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess24setStandardOutputProcessEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn waitForFinished<T: QProcess_waitForFinished>(&mut self, value: T) -> i8 {
    return value.waitForFinished(self);
    // return 1;
  }
}

pub trait QProcess_waitForFinished {
  fn waitForFinished(self, rsthis: &mut QProcess) -> i8;
}

// proto:  bool QProcess::waitForFinished(int msecs);
impl<'a> /*trait*/ QProcess_waitForFinished for (i32) {
  fn waitForFinished(self, rsthis: &mut QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess15waitForFinishedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN8QProcess15waitForFinishedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn bytesToWrite<T: QProcess_bytesToWrite>(&mut self, value: T) -> i64 {
    return value.bytesToWrite(self);
    // return 1;
  }
}

pub trait QProcess_bytesToWrite {
  fn bytesToWrite(self, rsthis: &mut QProcess) -> i64;
}

// proto:  long long QProcess::bytesToWrite();
impl<'a> /*trait*/ QProcess_bytesToWrite for () {
  fn bytesToWrite(self, rsthis: &mut QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess12bytesToWriteEv()};
    let mut ret = unsafe {_ZNK8QProcess12bytesToWriteEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn workingDirectory<T: QProcess_workingDirectory>(&mut self, value: T) -> QString {
    return value.workingDirectory(self);
    // return 1;
  }
}

pub trait QProcess_workingDirectory {
  fn workingDirectory(self, rsthis: &mut QProcess) -> QString;
}

// proto:  QString QProcess::workingDirectory();
impl<'a> /*trait*/ QProcess_workingDirectory for () {
  fn workingDirectory(self, rsthis: &mut QProcess) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess16workingDirectoryEv()};
    let mut ret = unsafe {_ZNK8QProcess16workingDirectoryEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setProgram<T: QProcess_setProgram>(&mut self, value: T)  {
     value.setProgram(self);
    // return 1;
  }
}

pub trait QProcess_setProgram {
  fn setProgram(self, rsthis: &mut QProcess) ;
}

// proto:  void QProcess::setProgram(const QString & program);
impl<'a> /*trait*/ QProcess_setProgram for (&'a  QString) {
  fn setProgram(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess10setProgramERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess10setProgramERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setStandardInputFile<T: QProcess_setStandardInputFile>(&mut self, value: T)  {
     value.setStandardInputFile(self);
    // return 1;
  }
}

pub trait QProcess_setStandardInputFile {
  fn setStandardInputFile(self, rsthis: &mut QProcess) ;
}

// proto:  void QProcess::setStandardInputFile(const QString & fileName);
impl<'a> /*trait*/ QProcess_setStandardInputFile for (&'a  QString) {
  fn setStandardInputFile(self, rsthis: &mut QProcess)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess20setStandardInputFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess20setStandardInputFileERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

