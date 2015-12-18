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
  pub fn close<RetType, T: QProcess_close<RetType>>(&mut self, value: T) -> RetType {
    return value.close(self);
    // return 1;
  }
}

pub trait QProcess_close<RetType> {
  fn close(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  void QProcess::close();
impl<'a> /*trait*/ QProcess_close<()> for () {
  fn close(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess5closeEv()};
     unsafe {_ZN8QProcess5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setEnvironment<RetType, T: QProcess_setEnvironment<RetType>>(&mut self, value: T) -> RetType {
    return value.setEnvironment(self);
    // return 1;
  }
}

pub trait QProcess_setEnvironment<RetType> {
  fn setEnvironment(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  void QProcess::setEnvironment(const QStringList & environment);
impl<'a> /*trait*/ QProcess_setEnvironment<()> for (&'a  QStringList) {
  fn setEnvironment(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess14setEnvironmentERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess14setEnvironmentERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn startDetached<RetType, T: QProcess_startDetached<RetType>>(&mut self, value: T) -> RetType {
    return value.startDetached(self);
    // return 1;
  }
}

pub trait QProcess_startDetached<RetType> {
  fn startDetached(self, rsthis: &mut QProcess) -> RetType;
}

// proto: static bool QProcess::startDetached(const QString & program, const QStringList & arguments);
impl<'a> /*trait*/ QProcess_startDetached<i8> for (&'a  QString, &'a  QStringList) {
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
impl<'a> /*trait*/ QProcess_startDetached<i8> for (&'a  QString) {
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
  pub fn atEnd<RetType, T: QProcess_atEnd<RetType>>(&mut self, value: T) -> RetType {
    return value.atEnd(self);
    // return 1;
  }
}

pub trait QProcess_atEnd<RetType> {
  fn atEnd(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  bool QProcess::atEnd();
impl<'a> /*trait*/ QProcess_atEnd<i8> for () {
  fn atEnd(self, rsthis: &mut QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess5atEndEv()};
    let mut ret = unsafe {_ZNK8QProcess5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn systemEnvironment<RetType, T: QProcess_systemEnvironment<RetType>>(&mut self, value: T) -> RetType {
    return value.systemEnvironment(self);
    // return 1;
  }
}

pub trait QProcess_systemEnvironment<RetType> {
  fn systemEnvironment(self, rsthis: &mut QProcess) -> RetType;
}

// proto: static QStringList QProcess::systemEnvironment();
impl<'a> /*trait*/ QProcess_systemEnvironment<()> for () {
  fn systemEnvironment(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess17systemEnvironmentEv()};
     unsafe {_ZN8QProcess17systemEnvironmentEv()};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setProcessEnvironment<RetType, T: QProcess_setProcessEnvironment<RetType>>(&mut self, value: T) -> RetType {
    return value.setProcessEnvironment(self);
    // return 1;
  }
}

pub trait QProcess_setProcessEnvironment<RetType> {
  fn setProcessEnvironment(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  void QProcess::setProcessEnvironment(const QProcessEnvironment & environment);
impl<'a> /*trait*/ QProcess_setProcessEnvironment<()> for (&'a  QProcessEnvironment) {
  fn setProcessEnvironment(self, rsthis: &mut QProcess) -> () {
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
  pub fn pid<RetType, T: QProcess_pid<RetType>>(&mut self, value: T) -> RetType {
    return value.pid(self);
    // return 1;
  }
}

pub trait QProcess_pid<RetType> {
  fn pid(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  qint64 QProcess::pid();
impl<'a> /*trait*/ QProcess_pid<i64> for () {
  fn pid(self, rsthis: &mut QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess3pidEv()};
    let mut ret = unsafe {_ZNK8QProcess3pidEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setArguments<RetType, T: QProcess_setArguments<RetType>>(&mut self, value: T) -> RetType {
    return value.setArguments(self);
    // return 1;
  }
}

pub trait QProcess_setArguments<RetType> {
  fn setArguments(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  void QProcess::setArguments(const QStringList & arguments);
impl<'a> /*trait*/ QProcess_setArguments<()> for (&'a  QStringList) {
  fn setArguments(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess12setArgumentsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess12setArgumentsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn FreeQProcess<RetType, T: QProcess_FreeQProcess<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQProcess(self);
    // return 1;
  }
}

pub trait QProcess_FreeQProcess<RetType> {
  fn FreeQProcess(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  void QProcess::FreeQProcess();
impl<'a> /*trait*/ QProcess_FreeQProcess<()> for () {
  fn FreeQProcess(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcessD0Ev()};
     unsafe {_ZN8QProcessD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn execute<RetType, T: QProcess_execute<RetType>>(&mut self, value: T) -> RetType {
    return value.execute(self);
    // return 1;
  }
}

pub trait QProcess_execute<RetType> {
  fn execute(self, rsthis: &mut QProcess) -> RetType;
}

// proto: static int QProcess::execute(const QString & command);
impl<'a> /*trait*/ QProcess_execute<i32> for (&'a  QString) {
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
  pub fn closeWriteChannel<RetType, T: QProcess_closeWriteChannel<RetType>>(&mut self, value: T) -> RetType {
    return value.closeWriteChannel(self);
    // return 1;
  }
}

pub trait QProcess_closeWriteChannel<RetType> {
  fn closeWriteChannel(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  void QProcess::closeWriteChannel();
impl<'a> /*trait*/ QProcess_closeWriteChannel<()> for () {
  fn closeWriteChannel(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess17closeWriteChannelEv()};
     unsafe {_ZN8QProcess17closeWriteChannelEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: static bool QProcess::startDetached(const QString & program, const QStringList & arguments, const QString & workingDirectory, qint64 * pid);
impl<'a> /*trait*/ QProcess_startDetached<i8> for (&'a  QString, &'a  QStringList, &'a  QString, &'a mut i64) {
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
  pub fn processEnvironment<RetType, T: QProcess_processEnvironment<RetType>>(&mut self, value: T) -> RetType {
    return value.processEnvironment(self);
    // return 1;
  }
}

pub trait QProcess_processEnvironment<RetType> {
  fn processEnvironment(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  QProcessEnvironment QProcess::processEnvironment();
impl<'a> /*trait*/ QProcess_processEnvironment<QProcessEnvironment> for () {
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
  pub fn readAllStandardOutput<RetType, T: QProcess_readAllStandardOutput<RetType>>(&mut self, value: T) -> RetType {
    return value.readAllStandardOutput(self);
    // return 1;
  }
}

pub trait QProcess_readAllStandardOutput<RetType> {
  fn readAllStandardOutput(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  QByteArray QProcess::readAllStandardOutput();
impl<'a> /*trait*/ QProcess_readAllStandardOutput<QByteArray> for () {
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
  pub fn nullDevice<RetType, T: QProcess_nullDevice<RetType>>(&mut self, value: T) -> RetType {
    return value.nullDevice(self);
    // return 1;
  }
}

pub trait QProcess_nullDevice<RetType> {
  fn nullDevice(self, rsthis: &mut QProcess) -> RetType;
}

// proto: static QString QProcess::nullDevice();
impl<'a> /*trait*/ QProcess_nullDevice<QString> for () {
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
impl<'a> /*trait*/ QProcess_execute<i32> for (&'a  QString, &'a  QStringList) {
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
  pub fn waitForBytesWritten<RetType, T: QProcess_waitForBytesWritten<RetType>>(&mut self, value: T) -> RetType {
    return value.waitForBytesWritten(self);
    // return 1;
  }
}

pub trait QProcess_waitForBytesWritten<RetType> {
  fn waitForBytesWritten(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  bool QProcess::waitForBytesWritten(int msecs);
impl<'a> /*trait*/ QProcess_waitForBytesWritten<i8> for (i32) {
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
  pub fn program<RetType, T: QProcess_program<RetType>>(&mut self, value: T) -> RetType {
    return value.program(self);
    // return 1;
  }
}

pub trait QProcess_program<RetType> {
  fn program(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  QString QProcess::program();
impl<'a> /*trait*/ QProcess_program<QString> for () {
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
  pub fn processId<RetType, T: QProcess_processId<RetType>>(&mut self, value: T) -> RetType {
    return value.processId(self);
    // return 1;
  }
}

pub trait QProcess_processId<RetType> {
  fn processId(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  long long QProcess::processId();
impl<'a> /*trait*/ QProcess_processId<i64> for () {
  fn processId(self, rsthis: &mut QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess9processIdEv()};
    let mut ret = unsafe {_ZNK8QProcess9processIdEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn arguments<RetType, T: QProcess_arguments<RetType>>(&mut self, value: T) -> RetType {
    return value.arguments(self);
    // return 1;
  }
}

pub trait QProcess_arguments<RetType> {
  fn arguments(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  QStringList QProcess::arguments();
impl<'a> /*trait*/ QProcess_arguments<()> for () {
  fn arguments(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess9argumentsEv()};
     unsafe {_ZNK8QProcess9argumentsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn isSequential<RetType, T: QProcess_isSequential<RetType>>(&mut self, value: T) -> RetType {
    return value.isSequential(self);
    // return 1;
  }
}

pub trait QProcess_isSequential<RetType> {
  fn isSequential(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  bool QProcess::isSequential();
impl<'a> /*trait*/ QProcess_isSequential<i8> for () {
  fn isSequential(self, rsthis: &mut QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess12isSequentialEv()};
    let mut ret = unsafe {_ZNK8QProcess12isSequentialEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn waitForReadyRead<RetType, T: QProcess_waitForReadyRead<RetType>>(&mut self, value: T) -> RetType {
    return value.waitForReadyRead(self);
    // return 1;
  }
}

pub trait QProcess_waitForReadyRead<RetType> {
  fn waitForReadyRead(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  bool QProcess::waitForReadyRead(int msecs);
impl<'a> /*trait*/ QProcess_waitForReadyRead<i8> for (i32) {
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
  pub fn setWorkingDirectory<RetType, T: QProcess_setWorkingDirectory<RetType>>(&mut self, value: T) -> RetType {
    return value.setWorkingDirectory(self);
    // return 1;
  }
}

pub trait QProcess_setWorkingDirectory<RetType> {
  fn setWorkingDirectory(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  void QProcess::setWorkingDirectory(const QString & dir);
impl<'a> /*trait*/ QProcess_setWorkingDirectory<()> for (&'a  QString) {
  fn setWorkingDirectory(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess19setWorkingDirectoryERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess19setWorkingDirectoryERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn terminate<RetType, T: QProcess_terminate<RetType>>(&mut self, value: T) -> RetType {
    return value.terminate(self);
    // return 1;
  }
}

pub trait QProcess_terminate<RetType> {
  fn terminate(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  void QProcess::terminate();
impl<'a> /*trait*/ QProcess_terminate<()> for () {
  fn terminate(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess9terminateEv()};
     unsafe {_ZN8QProcess9terminateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn kill<RetType, T: QProcess_kill<RetType>>(&mut self, value: T) -> RetType {
    return value.kill(self);
    // return 1;
  }
}

pub trait QProcess_kill<RetType> {
  fn kill(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  void QProcess::kill();
impl<'a> /*trait*/ QProcess_kill<()> for () {
  fn kill(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess4killEv()};
     unsafe {_ZN8QProcess4killEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn bytesAvailable<RetType, T: QProcess_bytesAvailable<RetType>>(&mut self, value: T) -> RetType {
    return value.bytesAvailable(self);
    // return 1;
  }
}

pub trait QProcess_bytesAvailable<RetType> {
  fn bytesAvailable(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  long long QProcess::bytesAvailable();
impl<'a> /*trait*/ QProcess_bytesAvailable<i64> for () {
  fn bytesAvailable(self, rsthis: &mut QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess14bytesAvailableEv()};
    let mut ret = unsafe {_ZNK8QProcess14bytesAvailableEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn metaObject<RetType, T: QProcess_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QProcess_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  const QMetaObject * QProcess::metaObject();
impl<'a> /*trait*/ QProcess_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess10metaObjectEv()};
     unsafe {_ZNK8QProcess10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn waitForStarted<RetType, T: QProcess_waitForStarted<RetType>>(&mut self, value: T) -> RetType {
    return value.waitForStarted(self);
    // return 1;
  }
}

pub trait QProcess_waitForStarted<RetType> {
  fn waitForStarted(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  bool QProcess::waitForStarted(int msecs);
impl<'a> /*trait*/ QProcess_waitForStarted<i8> for (i32) {
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
  pub fn readAllStandardError<RetType, T: QProcess_readAllStandardError<RetType>>(&mut self, value: T) -> RetType {
    return value.readAllStandardError(self);
    // return 1;
  }
}

pub trait QProcess_readAllStandardError<RetType> {
  fn readAllStandardError(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  QByteArray QProcess::readAllStandardError();
impl<'a> /*trait*/ QProcess_readAllStandardError<QByteArray> for () {
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
  pub fn exitCode<RetType, T: QProcess_exitCode<RetType>>(&mut self, value: T) -> RetType {
    return value.exitCode(self);
    // return 1;
  }
}

pub trait QProcess_exitCode<RetType> {
  fn exitCode(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  int QProcess::exitCode();
impl<'a> /*trait*/ QProcess_exitCode<i32> for () {
  fn exitCode(self, rsthis: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess8exitCodeEv()};
    let mut ret = unsafe {_ZNK8QProcess8exitCodeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn finished<RetType, T: QProcess_finished<RetType>>(&mut self, value: T) -> RetType {
    return value.finished(self);
    // return 1;
  }
}

pub trait QProcess_finished<RetType> {
  fn finished(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  void QProcess::finished(int exitCode);
impl<'a> /*trait*/ QProcess_finished<()> for (i32) {
  fn finished(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess8finishedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QProcess8finishedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn environment<RetType, T: QProcess_environment<RetType>>(&mut self, value: T) -> RetType {
    return value.environment(self);
    // return 1;
  }
}

pub trait QProcess_environment<RetType> {
  fn environment(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  QStringList QProcess::environment();
impl<'a> /*trait*/ QProcess_environment<()> for () {
  fn environment(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess11environmentEv()};
     unsafe {_ZNK8QProcess11environmentEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn canReadLine<RetType, T: QProcess_canReadLine<RetType>>(&mut self, value: T) -> RetType {
    return value.canReadLine(self);
    // return 1;
  }
}

pub trait QProcess_canReadLine<RetType> {
  fn canReadLine(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  bool QProcess::canReadLine();
impl<'a> /*trait*/ QProcess_canReadLine<i8> for () {
  fn canReadLine(self, rsthis: &mut QProcess) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess11canReadLineEv()};
    let mut ret = unsafe {_ZNK8QProcess11canReadLineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setStandardOutputProcess<RetType, T: QProcess_setStandardOutputProcess<RetType>>(&mut self, value: T) -> RetType {
    return value.setStandardOutputProcess(self);
    // return 1;
  }
}

pub trait QProcess_setStandardOutputProcess<RetType> {
  fn setStandardOutputProcess(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  void QProcess::setStandardOutputProcess(QProcess * destination);
impl<'a> /*trait*/ QProcess_setStandardOutputProcess<()> for (&'a mut QProcess) {
  fn setStandardOutputProcess(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess24setStandardOutputProcessEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess24setStandardOutputProcessEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn waitForFinished<RetType, T: QProcess_waitForFinished<RetType>>(&mut self, value: T) -> RetType {
    return value.waitForFinished(self);
    // return 1;
  }
}

pub trait QProcess_waitForFinished<RetType> {
  fn waitForFinished(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  bool QProcess::waitForFinished(int msecs);
impl<'a> /*trait*/ QProcess_waitForFinished<i8> for (i32) {
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
  pub fn bytesToWrite<RetType, T: QProcess_bytesToWrite<RetType>>(&mut self, value: T) -> RetType {
    return value.bytesToWrite(self);
    // return 1;
  }
}

pub trait QProcess_bytesToWrite<RetType> {
  fn bytesToWrite(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  long long QProcess::bytesToWrite();
impl<'a> /*trait*/ QProcess_bytesToWrite<i64> for () {
  fn bytesToWrite(self, rsthis: &mut QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess12bytesToWriteEv()};
    let mut ret = unsafe {_ZNK8QProcess12bytesToWriteEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn workingDirectory<RetType, T: QProcess_workingDirectory<RetType>>(&mut self, value: T) -> RetType {
    return value.workingDirectory(self);
    // return 1;
  }
}

pub trait QProcess_workingDirectory<RetType> {
  fn workingDirectory(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  QString QProcess::workingDirectory();
impl<'a> /*trait*/ QProcess_workingDirectory<QString> for () {
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
  pub fn setProgram<RetType, T: QProcess_setProgram<RetType>>(&mut self, value: T) -> RetType {
    return value.setProgram(self);
    // return 1;
  }
}

pub trait QProcess_setProgram<RetType> {
  fn setProgram(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  void QProcess::setProgram(const QString & program);
impl<'a> /*trait*/ QProcess_setProgram<()> for (&'a  QString) {
  fn setProgram(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess10setProgramERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess10setProgramERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setStandardInputFile<RetType, T: QProcess_setStandardInputFile<RetType>>(&mut self, value: T) -> RetType {
    return value.setStandardInputFile(self);
    // return 1;
  }
}

pub trait QProcess_setStandardInputFile<RetType> {
  fn setStandardInputFile(self, rsthis: &mut QProcess) -> RetType;
}

// proto:  void QProcess::setStandardInputFile(const QString & fileName);
impl<'a> /*trait*/ QProcess_setStandardInputFile<()> for (&'a  QString) {
  fn setStandardInputFile(self, rsthis: &mut QProcess) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess20setStandardInputFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QProcess20setStandardInputFileERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

