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
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN8QProcess4openE6QFlagsIN9QIODevice12OpenModeFlagEE(arg0: c_int) -> i32;
  fn _ZN8QProcess5closeEv() -> i32;
  fn _ZN8QProcess14setEnvironmentERK11QStringList(arg0: *const c_void) -> i32;
  fn _ZN8QProcess13startDetachedERK7QStringRK11QStringList(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN8QProcess13startDetachedERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK8QProcess5atEndEv() -> i32;
  fn _ZN8QProcess17systemEnvironmentEv() -> i32;
  fn _ZN8QProcess21setProcessEnvironmentERK19QProcessEnvironment(arg0: *const c_void) -> i32;
  fn _ZN8QProcessC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK8QProcess3pidEv() -> i32;
  fn _ZN8QProcess12setArgumentsERK11QStringList(arg0: *const c_void) -> i32;
  fn _ZN8QProcessD0Ev() -> i32;
  fn _ZN8QProcess7executeERK7QString(arg0: *const c_void) -> i32;
  fn _ZN8QProcess17closeWriteChannelEv() -> i32;
  fn _ZN8QProcess5startE6QFlagsIN9QIODevice12OpenModeFlagEE(arg0: c_int) -> i32;
  fn _ZN8QProcess13startDetachedERK7QStringRK11QStringListS2_Px(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *mut c_longlong) -> i32;
  fn _ZN8QProcess20setStandardErrorFileERK7QString6QFlagsIN9QIODevice12OpenModeFlagEE(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZNK8QProcess18processEnvironmentEv() -> i32;
  fn _ZN8QProcess21readAllStandardOutputEv() -> i32;
  fn _ZN8QProcess10nullDeviceEv() -> i32;
  fn _ZN8QProcess7executeERK7QStringRK11QStringList(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN8QProcess19waitForBytesWrittenEi(arg0: c_int) -> i32;
  fn _ZN8QProcessC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK8QProcess7programEv() -> i32;
  fn _ZNK8QProcess9processIdEv() -> i32;
  fn _ZN8QProcess5startERK7QString6QFlagsIN9QIODevice12OpenModeFlagEE(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN8QProcess5startERK7QStringRK11QStringList6QFlagsIN9QIODevice12OpenModeFlagEE(arg0: *const c_void, arg1: *const c_void, arg2: c_int) -> i32;
  fn _ZNK8QProcess9argumentsEv() -> i32;
  fn _ZNK8QProcess12isSequentialEv() -> i32;
  fn _ZN8QProcess16waitForReadyReadEi(arg0: c_int) -> i32;
  fn _ZN8QProcess19setWorkingDirectoryERK7QString(arg0: *const c_void) -> i32;
  fn _ZN8QProcess9terminateEv() -> i32;
  fn _ZN8QProcess4killEv() -> i32;
  fn _ZNK8QProcess14bytesAvailableEv() -> i32;
  fn _ZNK8QProcess10metaObjectEv() -> i32;
  fn _ZN8QProcess14waitForStartedEi(arg0: c_int) -> i32;
  fn _ZN8QProcess20readAllStandardErrorEv() -> i32;
  fn _ZNK8QProcess8exitCodeEv() -> i32;
  fn _ZN8QProcess8finishedEi(arg0: c_int) -> i32;
  fn _ZNK8QProcess11environmentEv() -> i32;
  fn _ZNK8QProcess11canReadLineEv() -> i32;
  fn _ZN8QProcess24setStandardOutputProcessEPS_(arg0: *mut c_void) -> i32;
  fn _ZN8QProcess21setStandardOutputFileERK7QString6QFlagsIN9QIODevice12OpenModeFlagEE(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN8QProcess15waitForFinishedEi(arg0: c_int) -> i32;
  fn _ZNK8QProcess12bytesToWriteEv() -> i32;
  fn _ZNK8QProcess16workingDirectoryEv() -> i32;
  fn _ZN8QProcess10setProgramERK7QString(arg0: *const c_void) -> i32;
  fn _ZN8QProcess20setStandardInputFileERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QProcess)=1
pub struct QProcess {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QProcess {
  pub fn open<T: QProcess_open>(&mut self, value: T) -> i32 {
    value.open(self);
    return 1;
  }
}

pub trait QProcess_open {
  fn open(self, this: &mut QProcess) -> i32;
}

// proto: bool QProcess::open(OpenMode mode);
impl<'a> /*trait*/ QProcess_open for (i32) {
  fn open(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess4openE6QFlagsIN9QIODevice12OpenModeFlagEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QProcess4openE6QFlagsIN9QIODevice12OpenModeFlagEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn close<T: QProcess_close>(&mut self, value: T) -> i32 {
    value.close(self);
    return 1;
  }
}

pub trait QProcess_close {
  fn close(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::close();
impl<'a> /*trait*/ QProcess_close for () {
  fn close(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess5closeEv()};
    unsafe {_ZN8QProcess5closeEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setEnvironment<T: QProcess_setEnvironment>(&mut self, value: T) -> i32 {
    value.setEnvironment(self);
    return 1;
  }
}

pub trait QProcess_setEnvironment {
  fn setEnvironment(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::setEnvironment(const QStringList & environment);
impl<'a> /*trait*/ QProcess_setEnvironment for (&'a  QStringList) {
  fn setEnvironment(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess14setEnvironmentERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QProcess14setEnvironmentERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn startDetached<T: QProcess_startDetached>(&mut self, value: T) -> i32 {
    value.startDetached(self);
    return 1;
  }
}

pub trait QProcess_startDetached {
  fn startDetached(self, this: &mut QProcess) -> i32;
}

// proto: bool QProcess::startDetached(const QString & program, const QStringList & arguments);
impl<'a> /*trait*/ QProcess_startDetached for (&'a  QString, &'a  QStringList) {
  fn startDetached(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess13startDetachedERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QProcess13startDetachedERK7QStringRK11QStringList(arg0, arg1)};
    return 1;
  }
}

// proto: bool QProcess::startDetached(const QString & command);
impl<'a> /*trait*/ QProcess_startDetached for (&'a  QString) {
  fn startDetached(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess13startDetachedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QProcess13startDetachedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn atEnd<T: QProcess_atEnd>(&mut self, value: T) -> i32 {
    value.atEnd(self);
    return 1;
  }
}

pub trait QProcess_atEnd {
  fn atEnd(self, this: &mut QProcess) -> i32;
}

// proto: bool QProcess::atEnd();
impl<'a> /*trait*/ QProcess_atEnd for () {
  fn atEnd(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess5atEndEv()};
    unsafe {_ZNK8QProcess5atEndEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn systemEnvironment<T: QProcess_systemEnvironment>(&mut self, value: T) -> i32 {
    value.systemEnvironment(self);
    return 1;
  }
}

pub trait QProcess_systemEnvironment {
  fn systemEnvironment(self, this: &mut QProcess) -> i32;
}

// proto: QStringList QProcess::systemEnvironment();
impl<'a> /*trait*/ QProcess_systemEnvironment for () {
  fn systemEnvironment(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess17systemEnvironmentEv()};
    unsafe {_ZN8QProcess17systemEnvironmentEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setProcessEnvironment<T: QProcess_setProcessEnvironment>(&mut self, value: T) -> i32 {
    value.setProcessEnvironment(self);
    return 1;
  }
}

pub trait QProcess_setProcessEnvironment {
  fn setProcessEnvironment(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::setProcessEnvironment(const QProcessEnvironment & environment);
impl<'a> /*trait*/ QProcess_setProcessEnvironment for (&'a  QProcessEnvironment) {
  fn setProcessEnvironment(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess21setProcessEnvironmentERK19QProcessEnvironment()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QProcess21setProcessEnvironmentERK19QProcessEnvironment(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QProcessC1ERKS_(qthis, arg0)};
    let rsthis = QProcess{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn pid<T: QProcess_pid>(&mut self, value: T) -> i32 {
    value.pid(self);
    return 1;
  }
}

pub trait QProcess_pid {
  fn pid(self, this: &mut QProcess) -> i32;
}

// proto: qint64 QProcess::pid();
impl<'a> /*trait*/ QProcess_pid for () {
  fn pid(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess3pidEv()};
    unsafe {_ZNK8QProcess3pidEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setArguments<T: QProcess_setArguments>(&mut self, value: T) -> i32 {
    value.setArguments(self);
    return 1;
  }
}

pub trait QProcess_setArguments {
  fn setArguments(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::setArguments(const QStringList & arguments);
impl<'a> /*trait*/ QProcess_setArguments for (&'a  QStringList) {
  fn setArguments(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess12setArgumentsERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QProcess12setArgumentsERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn FreeQProcess<T: QProcess_FreeQProcess>(&mut self, value: T) -> i32 {
    value.FreeQProcess(self);
    return 1;
  }
}

pub trait QProcess_FreeQProcess {
  fn FreeQProcess(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::FreeQProcess();
impl<'a> /*trait*/ QProcess_FreeQProcess for () {
  fn FreeQProcess(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcessD0Ev()};
    unsafe {_ZN8QProcessD0Ev()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn execute<T: QProcess_execute>(&mut self, value: T) -> i32 {
    value.execute(self);
    return 1;
  }
}

pub trait QProcess_execute {
  fn execute(self, this: &mut QProcess) -> i32;
}

// proto: int QProcess::execute(const QString & command);
impl<'a> /*trait*/ QProcess_execute for (&'a  QString) {
  fn execute(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess7executeERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QProcess7executeERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn closeWriteChannel<T: QProcess_closeWriteChannel>(&mut self, value: T) -> i32 {
    value.closeWriteChannel(self);
    return 1;
  }
}

pub trait QProcess_closeWriteChannel {
  fn closeWriteChannel(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::closeWriteChannel();
impl<'a> /*trait*/ QProcess_closeWriteChannel for () {
  fn closeWriteChannel(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess17closeWriteChannelEv()};
    unsafe {_ZN8QProcess17closeWriteChannelEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn start<T: QProcess_start>(&mut self, value: T) -> i32 {
    value.start(self);
    return 1;
  }
}

pub trait QProcess_start {
  fn start(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::start(OpenMode mode);
impl<'a> /*trait*/ QProcess_start for (i32) {
  fn start(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess5startE6QFlagsIN9QIODevice12OpenModeFlagEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QProcess5startE6QFlagsIN9QIODevice12OpenModeFlagEE(arg0)};
    return 1;
  }
}

// proto: bool QProcess::startDetached(const QString & program, const QStringList & arguments, const QString & workingDirectory, qint64 * pid);
impl<'a> /*trait*/ QProcess_startDetached for (&'a  QString, &'a  QStringList, &'a  QString, &'a mut i64) {
  fn startDetached(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess13startDetachedERK7QStringRK11QStringListS2_Px()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3  as *mut c_longlong;
    unsafe {_ZN8QProcess13startDetachedERK7QStringRK11QStringListS2_Px(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setStandardErrorFile<T: QProcess_setStandardErrorFile>(&mut self, value: T) -> i32 {
    value.setStandardErrorFile(self);
    return 1;
  }
}

pub trait QProcess_setStandardErrorFile {
  fn setStandardErrorFile(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::setStandardErrorFile(const QString & fileName, OpenMode mode);
impl<'a> /*trait*/ QProcess_setStandardErrorFile for (&'a  QString, i32) {
  fn setStandardErrorFile(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess20setStandardErrorFileERK7QString6QFlagsIN9QIODevice12OpenModeFlagEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QProcess20setStandardErrorFileERK7QString6QFlagsIN9QIODevice12OpenModeFlagEE(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn processEnvironment<T: QProcess_processEnvironment>(&mut self, value: T) -> i32 {
    value.processEnvironment(self);
    return 1;
  }
}

pub trait QProcess_processEnvironment {
  fn processEnvironment(self, this: &mut QProcess) -> i32;
}

// proto: QProcessEnvironment QProcess::processEnvironment();
impl<'a> /*trait*/ QProcess_processEnvironment for () {
  fn processEnvironment(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess18processEnvironmentEv()};
    unsafe {_ZNK8QProcess18processEnvironmentEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn readAllStandardOutput<T: QProcess_readAllStandardOutput>(&mut self, value: T) -> i32 {
    value.readAllStandardOutput(self);
    return 1;
  }
}

pub trait QProcess_readAllStandardOutput {
  fn readAllStandardOutput(self, this: &mut QProcess) -> i32;
}

// proto: QByteArray QProcess::readAllStandardOutput();
impl<'a> /*trait*/ QProcess_readAllStandardOutput for () {
  fn readAllStandardOutput(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess21readAllStandardOutputEv()};
    unsafe {_ZN8QProcess21readAllStandardOutputEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn nullDevice<T: QProcess_nullDevice>(&mut self, value: T) -> i32 {
    value.nullDevice(self);
    return 1;
  }
}

pub trait QProcess_nullDevice {
  fn nullDevice(self, this: &mut QProcess) -> i32;
}

// proto: QString QProcess::nullDevice();
impl<'a> /*trait*/ QProcess_nullDevice for () {
  fn nullDevice(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess10nullDeviceEv()};
    unsafe {_ZN8QProcess10nullDeviceEv()};
    return 1;
  }
}

// proto: int QProcess::execute(const QString & program, const QStringList & arguments);
impl<'a> /*trait*/ QProcess_execute for (&'a  QString, &'a  QStringList) {
  fn execute(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess7executeERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QProcess7executeERK7QStringRK11QStringList(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn waitForBytesWritten<T: QProcess_waitForBytesWritten>(&mut self, value: T) -> i32 {
    value.waitForBytesWritten(self);
    return 1;
  }
}

pub trait QProcess_waitForBytesWritten {
  fn waitForBytesWritten(self, this: &mut QProcess) -> i32;
}

// proto: bool QProcess::waitForBytesWritten(int msecs);
impl<'a> /*trait*/ QProcess_waitForBytesWritten for (i32) {
  fn waitForBytesWritten(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess19waitForBytesWrittenEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QProcess19waitForBytesWrittenEi(arg0)};
    return 1;
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
  pub fn program<T: QProcess_program>(&mut self, value: T) -> i32 {
    value.program(self);
    return 1;
  }
}

pub trait QProcess_program {
  fn program(self, this: &mut QProcess) -> i32;
}

// proto: QString QProcess::program();
impl<'a> /*trait*/ QProcess_program for () {
  fn program(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess7programEv()};
    unsafe {_ZNK8QProcess7programEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn processId<T: QProcess_processId>(&mut self, value: T) -> i32 {
    value.processId(self);
    return 1;
  }
}

pub trait QProcess_processId {
  fn processId(self, this: &mut QProcess) -> i32;
}

// proto: long long QProcess::processId();
impl<'a> /*trait*/ QProcess_processId for () {
  fn processId(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess9processIdEv()};
    unsafe {_ZNK8QProcess9processIdEv()};
    return 1;
  }
}

// proto: void QProcess::start(const QString & command, OpenMode mode);
impl<'a> /*trait*/ QProcess_start for (&'a  QString, i32) {
  fn start(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess5startERK7QString6QFlagsIN9QIODevice12OpenModeFlagEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QProcess5startERK7QString6QFlagsIN9QIODevice12OpenModeFlagEE(arg0, arg1)};
    return 1;
  }
}

// proto: void QProcess::start(const QString & program, const QStringList & arguments, OpenMode mode);
impl<'a> /*trait*/ QProcess_start for (&'a  QString, &'a  QStringList, i32) {
  fn start(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess5startERK7QStringRK11QStringList6QFlagsIN9QIODevice12OpenModeFlagEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN8QProcess5startERK7QStringRK11QStringList6QFlagsIN9QIODevice12OpenModeFlagEE(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn arguments<T: QProcess_arguments>(&mut self, value: T) -> i32 {
    value.arguments(self);
    return 1;
  }
}

pub trait QProcess_arguments {
  fn arguments(self, this: &mut QProcess) -> i32;
}

// proto: QStringList QProcess::arguments();
impl<'a> /*trait*/ QProcess_arguments for () {
  fn arguments(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess9argumentsEv()};
    unsafe {_ZNK8QProcess9argumentsEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn isSequential<T: QProcess_isSequential>(&mut self, value: T) -> i32 {
    value.isSequential(self);
    return 1;
  }
}

pub trait QProcess_isSequential {
  fn isSequential(self, this: &mut QProcess) -> i32;
}

// proto: bool QProcess::isSequential();
impl<'a> /*trait*/ QProcess_isSequential for () {
  fn isSequential(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess12isSequentialEv()};
    unsafe {_ZNK8QProcess12isSequentialEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn waitForReadyRead<T: QProcess_waitForReadyRead>(&mut self, value: T) -> i32 {
    value.waitForReadyRead(self);
    return 1;
  }
}

pub trait QProcess_waitForReadyRead {
  fn waitForReadyRead(self, this: &mut QProcess) -> i32;
}

// proto: bool QProcess::waitForReadyRead(int msecs);
impl<'a> /*trait*/ QProcess_waitForReadyRead for (i32) {
  fn waitForReadyRead(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess16waitForReadyReadEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QProcess16waitForReadyReadEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setWorkingDirectory<T: QProcess_setWorkingDirectory>(&mut self, value: T) -> i32 {
    value.setWorkingDirectory(self);
    return 1;
  }
}

pub trait QProcess_setWorkingDirectory {
  fn setWorkingDirectory(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::setWorkingDirectory(const QString & dir);
impl<'a> /*trait*/ QProcess_setWorkingDirectory for (&'a  QString) {
  fn setWorkingDirectory(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess19setWorkingDirectoryERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QProcess19setWorkingDirectoryERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn terminate<T: QProcess_terminate>(&mut self, value: T) -> i32 {
    value.terminate(self);
    return 1;
  }
}

pub trait QProcess_terminate {
  fn terminate(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::terminate();
impl<'a> /*trait*/ QProcess_terminate for () {
  fn terminate(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess9terminateEv()};
    unsafe {_ZN8QProcess9terminateEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn kill<T: QProcess_kill>(&mut self, value: T) -> i32 {
    value.kill(self);
    return 1;
  }
}

pub trait QProcess_kill {
  fn kill(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::kill();
impl<'a> /*trait*/ QProcess_kill for () {
  fn kill(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess4killEv()};
    unsafe {_ZN8QProcess4killEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn bytesAvailable<T: QProcess_bytesAvailable>(&mut self, value: T) -> i32 {
    value.bytesAvailable(self);
    return 1;
  }
}

pub trait QProcess_bytesAvailable {
  fn bytesAvailable(self, this: &mut QProcess) -> i32;
}

// proto: long long QProcess::bytesAvailable();
impl<'a> /*trait*/ QProcess_bytesAvailable for () {
  fn bytesAvailable(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess14bytesAvailableEv()};
    unsafe {_ZNK8QProcess14bytesAvailableEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn metaObject<T: QProcess_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QProcess_metaObject {
  fn metaObject(self, this: &mut QProcess) -> i32;
}

// proto: const QMetaObject * QProcess::metaObject();
impl<'a> /*trait*/ QProcess_metaObject for () {
  fn metaObject(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess10metaObjectEv()};
    unsafe {_ZNK8QProcess10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn waitForStarted<T: QProcess_waitForStarted>(&mut self, value: T) -> i32 {
    value.waitForStarted(self);
    return 1;
  }
}

pub trait QProcess_waitForStarted {
  fn waitForStarted(self, this: &mut QProcess) -> i32;
}

// proto: bool QProcess::waitForStarted(int msecs);
impl<'a> /*trait*/ QProcess_waitForStarted for (i32) {
  fn waitForStarted(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess14waitForStartedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QProcess14waitForStartedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn readAllStandardError<T: QProcess_readAllStandardError>(&mut self, value: T) -> i32 {
    value.readAllStandardError(self);
    return 1;
  }
}

pub trait QProcess_readAllStandardError {
  fn readAllStandardError(self, this: &mut QProcess) -> i32;
}

// proto: QByteArray QProcess::readAllStandardError();
impl<'a> /*trait*/ QProcess_readAllStandardError for () {
  fn readAllStandardError(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess20readAllStandardErrorEv()};
    unsafe {_ZN8QProcess20readAllStandardErrorEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn exitCode<T: QProcess_exitCode>(&mut self, value: T) -> i32 {
    value.exitCode(self);
    return 1;
  }
}

pub trait QProcess_exitCode {
  fn exitCode(self, this: &mut QProcess) -> i32;
}

// proto: int QProcess::exitCode();
impl<'a> /*trait*/ QProcess_exitCode for () {
  fn exitCode(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess8exitCodeEv()};
    unsafe {_ZNK8QProcess8exitCodeEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn finished<T: QProcess_finished>(&mut self, value: T) -> i32 {
    value.finished(self);
    return 1;
  }
}

pub trait QProcess_finished {
  fn finished(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::finished(int exitCode);
impl<'a> /*trait*/ QProcess_finished for (i32) {
  fn finished(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess8finishedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QProcess8finishedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn environment<T: QProcess_environment>(&mut self, value: T) -> i32 {
    value.environment(self);
    return 1;
  }
}

pub trait QProcess_environment {
  fn environment(self, this: &mut QProcess) -> i32;
}

// proto: QStringList QProcess::environment();
impl<'a> /*trait*/ QProcess_environment for () {
  fn environment(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess11environmentEv()};
    unsafe {_ZNK8QProcess11environmentEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn canReadLine<T: QProcess_canReadLine>(&mut self, value: T) -> i32 {
    value.canReadLine(self);
    return 1;
  }
}

pub trait QProcess_canReadLine {
  fn canReadLine(self, this: &mut QProcess) -> i32;
}

// proto: bool QProcess::canReadLine();
impl<'a> /*trait*/ QProcess_canReadLine for () {
  fn canReadLine(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess11canReadLineEv()};
    unsafe {_ZNK8QProcess11canReadLineEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setStandardOutputProcess<T: QProcess_setStandardOutputProcess>(&mut self, value: T) -> i32 {
    value.setStandardOutputProcess(self);
    return 1;
  }
}

pub trait QProcess_setStandardOutputProcess {
  fn setStandardOutputProcess(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::setStandardOutputProcess(QProcess * destination);
impl<'a> /*trait*/ QProcess_setStandardOutputProcess for (&'a mut QProcess) {
  fn setStandardOutputProcess(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess24setStandardOutputProcessEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QProcess24setStandardOutputProcessEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setStandardOutputFile<T: QProcess_setStandardOutputFile>(&mut self, value: T) -> i32 {
    value.setStandardOutputFile(self);
    return 1;
  }
}

pub trait QProcess_setStandardOutputFile {
  fn setStandardOutputFile(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::setStandardOutputFile(const QString & fileName, OpenMode mode);
impl<'a> /*trait*/ QProcess_setStandardOutputFile for (&'a  QString, i32) {
  fn setStandardOutputFile(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess21setStandardOutputFileERK7QString6QFlagsIN9QIODevice12OpenModeFlagEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QProcess21setStandardOutputFileERK7QString6QFlagsIN9QIODevice12OpenModeFlagEE(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn waitForFinished<T: QProcess_waitForFinished>(&mut self, value: T) -> i32 {
    value.waitForFinished(self);
    return 1;
  }
}

pub trait QProcess_waitForFinished {
  fn waitForFinished(self, this: &mut QProcess) -> i32;
}

// proto: bool QProcess::waitForFinished(int msecs);
impl<'a> /*trait*/ QProcess_waitForFinished for (i32) {
  fn waitForFinished(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess15waitForFinishedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QProcess15waitForFinishedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn bytesToWrite<T: QProcess_bytesToWrite>(&mut self, value: T) -> i32 {
    value.bytesToWrite(self);
    return 1;
  }
}

pub trait QProcess_bytesToWrite {
  fn bytesToWrite(self, this: &mut QProcess) -> i32;
}

// proto: long long QProcess::bytesToWrite();
impl<'a> /*trait*/ QProcess_bytesToWrite for () {
  fn bytesToWrite(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess12bytesToWriteEv()};
    unsafe {_ZNK8QProcess12bytesToWriteEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn workingDirectory<T: QProcess_workingDirectory>(&mut self, value: T) -> i32 {
    value.workingDirectory(self);
    return 1;
  }
}

pub trait QProcess_workingDirectory {
  fn workingDirectory(self, this: &mut QProcess) -> i32;
}

// proto: QString QProcess::workingDirectory();
impl<'a> /*trait*/ QProcess_workingDirectory for () {
  fn workingDirectory(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QProcess16workingDirectoryEv()};
    unsafe {_ZNK8QProcess16workingDirectoryEv()};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setProgram<T: QProcess_setProgram>(&mut self, value: T) -> i32 {
    value.setProgram(self);
    return 1;
  }
}

pub trait QProcess_setProgram {
  fn setProgram(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::setProgram(const QString & program);
impl<'a> /*trait*/ QProcess_setProgram for (&'a  QString) {
  fn setProgram(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess10setProgramERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QProcess10setProgramERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QProcess {
  pub fn setStandardInputFile<T: QProcess_setStandardInputFile>(&mut self, value: T) -> i32 {
    value.setStandardInputFile(self);
    return 1;
  }
}

pub trait QProcess_setStandardInputFile {
  fn setStandardInputFile(self, this: &mut QProcess) -> i32;
}

// proto: void QProcess::setStandardInputFile(const QString & fileName);
impl<'a> /*trait*/ QProcess_setStandardInputFile for (&'a  QString) {
  fn setStandardInputFile(self, this: &mut QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QProcess20setStandardInputFileERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QProcess20setStandardInputFileERK7QString(arg0)};
    return 1;
  }
}

