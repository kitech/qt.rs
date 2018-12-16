

// mod ::core::QProcess
// package qtcore
// /usr/include/qt/QtCore/qprocess.h
// #include <qprocess.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 15
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// void setProcessState(QProcess::ProcessState)
// func (this *QProcess) InheritSetProcessState(f func(state int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setProcessState", f)
// }

// void setupChildProcess()
// func (this *QProcess) InheritSetupChildProcess(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setupChildProcess", f)
// }

// long long readData(char *, qint64)
// func (this *QProcess) InheritReadData(f func(data string, maxlen int64) int64) {
//  qtrt.SetAllInheritCallback(this, "readData", f)
// }

// long long writeData(const char *, qint64)
// func (this *QProcess) InheritWriteData(f func(data string, len_ int64) int64) {
//  qtrt.SetAllInheritCallback(this, "writeData", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QProcess)=16
pub struct QProcess {
  qbase: QIODevice,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QProcess_ITF interface {
//    QIODevice_ITF
//    QProcess_PTR() *QProcess
//}
//func (ptr *QProcess) QProcess_PTR() *QProcess { return ptr }

impl /*struct*/ QProcess {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QProcess {
    return QProcess{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QProcess {
//  type Target = QProcessBASE;
//
//  fn deref(&self) -> &QProcessBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QProcessBASE> for QProcess {
//  fn as_ref(& self) -> & QProcessBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qprocess.h:112
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QProcess {
  pub fn metaObject_0<RetType, T: QProcess_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QProcess_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QProcess) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QProcess(QObject *)

/*
Constructs a QProcess object with the given parent.
*/
// QProcess(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QProcess {
  pub fn QProcess_0<T: QProcess_QProcess_0>(value: T) -> QProcess {
    let rsthis = value.QProcess_0();
    return rsthis;
    // return 1;
  }
}

pub trait QProcess_QProcess_0 {
  fn QProcess_0(self) -> QProcess;
}
// QProcess(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QProcess_QProcess_0 for (usize) {
  fn QProcess_0(self) -> QProcess {
    // unsafe{_ZN8QProcessC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QProcessC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QProcess{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:159
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QProcess()

/*

*/
pub fn DeleteQProcess(this :*mut QProcess) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QProcessD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qprocess.h:161
// index:0
// Public Visibility=Default Availability=Available
// [-2] void start(const QString &, const QStringList &, QIODevice::OpenMode)

/*
Starts the given program in a new process, passing the command line arguments in arguments.

The QProcess object will immediately enter the Starting state. If the process starts successfully, QProcess will emit started(); otherwise, errorOccurred() will be emitted.

Note: Processes are started asynchronously, which means the started() and errorOccurred() signals may be delayed. Call waitForStarted() to make sure the process has started (or has failed to start) and those signals have been emitted.

Note: No further splitting of the arguments is performed.

Windows: The arguments are quoted and joined into a command line that is compatible with the CommandLineToArgvW() Windows function. For programs that have different command line quoting requirements, you need to use setNativeArguments(). One notable program that does not follow the CommandLineToArgvW() rules is cmd.exe and, by consequence, all batch scripts.

The OpenMode is set to mode.

If the QProcess object is already running a process, a warning may be printed at the console, and the existing process will continue running unaffected.

See also processId(), started(), waitForStarted(), and setNativeArguments().
*/
impl /*struct*/ QProcess {
  pub fn start_0<RetType, T: QProcess_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QProcess_start_0<RetType> {
  fn start_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_start_0<(/*void*/)> for (usize,usize,i32) {
  fn start_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess5startERK7QStringRK11QStringList6QFlagsIN9QIODevice12OpenModeFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:163
// index:1
// Public Visibility=Default Availability=Available
// [-2] void start(const QString &, QIODevice::OpenMode)

/*
Starts the given program in a new process, passing the command line arguments in arguments.

The QProcess object will immediately enter the Starting state. If the process starts successfully, QProcess will emit started(); otherwise, errorOccurred() will be emitted.

Note: Processes are started asynchronously, which means the started() and errorOccurred() signals may be delayed. Call waitForStarted() to make sure the process has started (or has failed to start) and those signals have been emitted.

Note: No further splitting of the arguments is performed.

Windows: The arguments are quoted and joined into a command line that is compatible with the CommandLineToArgvW() Windows function. For programs that have different command line quoting requirements, you need to use setNativeArguments(). One notable program that does not follow the CommandLineToArgvW() rules is cmd.exe and, by consequence, all batch scripts.

The OpenMode is set to mode.

If the QProcess object is already running a process, a warning may be printed at the console, and the existing process will continue running unaffected.

See also processId(), started(), waitForStarted(), and setNativeArguments().
*/
impl /*struct*/ QProcess {
  pub fn start_1<RetType, T: QProcess_start_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_1(self);
    // return 1;
  }
}
pub trait QProcess_start_1<RetType> {
  fn start_1(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_start_1<(/*void*/)> for (usize,i32) {
  fn start_1(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess5startERK7QString6QFlagsIN9QIODevice12OpenModeFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:165
// index:2
// Public Visibility=Default Availability=Available
// [-2] void start(QIODevice::OpenMode)

/*
Starts the given program in a new process, passing the command line arguments in arguments.

The QProcess object will immediately enter the Starting state. If the process starts successfully, QProcess will emit started(); otherwise, errorOccurred() will be emitted.

Note: Processes are started asynchronously, which means the started() and errorOccurred() signals may be delayed. Call waitForStarted() to make sure the process has started (or has failed to start) and those signals have been emitted.

Note: No further splitting of the arguments is performed.

Windows: The arguments are quoted and joined into a command line that is compatible with the CommandLineToArgvW() Windows function. For programs that have different command line quoting requirements, you need to use setNativeArguments(). One notable program that does not follow the CommandLineToArgvW() rules is cmd.exe and, by consequence, all batch scripts.

The OpenMode is set to mode.

If the QProcess object is already running a process, a warning may be printed at the console, and the existing process will continue running unaffected.

See also processId(), started(), waitForStarted(), and setNativeArguments().
*/
impl /*struct*/ QProcess {
  pub fn start_2<RetType, T: QProcess_start_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_2(self);
    // return 1;
  }
}
pub trait QProcess_start_2<RetType> {
  fn start_2(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_start_2<(/*void*/)> for (i32) {
  fn start_2(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess5startE6QFlagsIN9QIODevice12OpenModeFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:166
// index:0
// Public Visibility=Default Availability=Available
// [1] bool startDetached(qint64 *)

/*
Starts the program set by setProgram() with arguments set by setArguments() in a new process, and detaches from it. Returns true on success; otherwise returns false. If the calling process exits, the detached process will continue to run unaffected.

Unix: The started process will run in its own session and act like a daemon.

The process will be started in the directory set by setWorkingDirectory(). If workingDirectory() is empty, the working directory is inherited from the calling process.

Note: On QNX, this may cause all application threads to temporarily freeze.

If the function is successful then *pid is set to the process identifier of the started process. Note that the child process may exit and the PID may become invalid without notice. Furthermore, after the child process exits, the same PID may be recycled and used by a completely different process. User code should be careful when using this variable, especially if one intends to forcibly terminate the process by operating system means.

Only the following property setters are supported by startDetached():


setArguments()
setCreateProcessArgumentsModifier()
setNativeArguments()
setProcessEnvironment()
setProgram()
setStandardErrorFile()
setStandardInputFile()
setStandardOutputFile()
setWorkingDirectory()


All other properties of the QProcess object are ignored.

This function was introduced in  Qt 5.10.

See also start(), startDetached(const QString &program, const QStringList &arguments, const QString &workingDirectory, qint64 *pid), and startDetached(const QString &command).
*/
impl /*struct*/ QProcess {
  pub fn startDetached_0<RetType, T: QProcess_startDetached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startDetached_0(self);
    // return 1;
  }
}
pub trait QProcess_startDetached_0<RetType> {
  fn startDetached_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_startDetached_0<bool> for (usize) {
  fn startDetached_0(self , rsthis: & QProcess) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess13startDetachedEPx", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:251
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool startDetached(const QString &, const QStringList &, const QString &, qint64 *)

/*
Starts the program set by setProgram() with arguments set by setArguments() in a new process, and detaches from it. Returns true on success; otherwise returns false. If the calling process exits, the detached process will continue to run unaffected.

Unix: The started process will run in its own session and act like a daemon.

The process will be started in the directory set by setWorkingDirectory(). If workingDirectory() is empty, the working directory is inherited from the calling process.

Note: On QNX, this may cause all application threads to temporarily freeze.

If the function is successful then *pid is set to the process identifier of the started process. Note that the child process may exit and the PID may become invalid without notice. Furthermore, after the child process exits, the same PID may be recycled and used by a completely different process. User code should be careful when using this variable, especially if one intends to forcibly terminate the process by operating system means.

Only the following property setters are supported by startDetached():


setArguments()
setCreateProcessArgumentsModifier()
setNativeArguments()
setProcessEnvironment()
setProgram()
setStandardErrorFile()
setStandardInputFile()
setStandardOutputFile()
setWorkingDirectory()


All other properties of the QProcess object are ignored.

This function was introduced in  Qt 5.10.

See also start(), startDetached(const QString &program, const QStringList &arguments, const QString &workingDirectory, qint64 *pid), and startDetached(const QString &command).
*/
impl /*struct*/ QProcess {
  pub fn startDetached_1<RetType, T: QProcess_startDetached_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.startDetached_1();
    // return 1;
  }
}
pub trait QProcess_startDetached_1<RetType> {
  fn startDetached_1(self ) -> RetType;
}
impl<'a> /*trait*/ QProcess_startDetached_1<bool> for (usize,usize,usize,usize) {
  fn startDetached_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess13startDetachedERK7QStringRK11QStringListS2_Px", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:258
// index:2
// Public static Visibility=Default Availability=Available
// [1] bool startDetached(const QString &, const QStringList &)

/*
Starts the program set by setProgram() with arguments set by setArguments() in a new process, and detaches from it. Returns true on success; otherwise returns false. If the calling process exits, the detached process will continue to run unaffected.

Unix: The started process will run in its own session and act like a daemon.

The process will be started in the directory set by setWorkingDirectory(). If workingDirectory() is empty, the working directory is inherited from the calling process.

Note: On QNX, this may cause all application threads to temporarily freeze.

If the function is successful then *pid is set to the process identifier of the started process. Note that the child process may exit and the PID may become invalid without notice. Furthermore, after the child process exits, the same PID may be recycled and used by a completely different process. User code should be careful when using this variable, especially if one intends to forcibly terminate the process by operating system means.

Only the following property setters are supported by startDetached():


setArguments()
setCreateProcessArgumentsModifier()
setNativeArguments()
setProcessEnvironment()
setProgram()
setStandardErrorFile()
setStandardInputFile()
setStandardOutputFile()
setWorkingDirectory()


All other properties of the QProcess object are ignored.

This function was introduced in  Qt 5.10.

See also start(), startDetached(const QString &program, const QStringList &arguments, const QString &workingDirectory, qint64 *pid), and startDetached(const QString &command).
*/
impl /*struct*/ QProcess {
  pub fn startDetached_2<RetType, T: QProcess_startDetached_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.startDetached_2();
    // return 1;
  }
}
pub trait QProcess_startDetached_2<RetType> {
  fn startDetached_2(self ) -> RetType;
}
impl<'a> /*trait*/ QProcess_startDetached_2<bool> for (usize,usize) {
  fn startDetached_2(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess13startDetachedERK7QStringRK11QStringList", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:260
// index:3
// Public static Visibility=Default Availability=Available
// [1] bool startDetached(const QString &)

/*
Starts the program set by setProgram() with arguments set by setArguments() in a new process, and detaches from it. Returns true on success; otherwise returns false. If the calling process exits, the detached process will continue to run unaffected.

Unix: The started process will run in its own session and act like a daemon.

The process will be started in the directory set by setWorkingDirectory(). If workingDirectory() is empty, the working directory is inherited from the calling process.

Note: On QNX, this may cause all application threads to temporarily freeze.

If the function is successful then *pid is set to the process identifier of the started process. Note that the child process may exit and the PID may become invalid without notice. Furthermore, after the child process exits, the same PID may be recycled and used by a completely different process. User code should be careful when using this variable, especially if one intends to forcibly terminate the process by operating system means.

Only the following property setters are supported by startDetached():


setArguments()
setCreateProcessArgumentsModifier()
setNativeArguments()
setProcessEnvironment()
setProgram()
setStandardErrorFile()
setStandardInputFile()
setStandardOutputFile()
setWorkingDirectory()


All other properties of the QProcess object are ignored.

This function was introduced in  Qt 5.10.

See also start(), startDetached(const QString &program, const QStringList &arguments, const QString &workingDirectory, qint64 *pid), and startDetached(const QString &command).
*/
impl /*struct*/ QProcess {
  pub fn startDetached_3<RetType, T: QProcess_startDetached_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.startDetached_3();
    // return 1;
  }
}
pub trait QProcess_startDetached_3<RetType> {
  fn startDetached_3(self ) -> RetType;
}
impl<'a> /*trait*/ QProcess_startDetached_3<bool> for (usize) {
  fn startDetached_3(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess13startDetachedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:167
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool open(QIODevice::OpenMode)

/*
Reimplemented from QIODevice::open().

Starts the program set by setProgram() with arguments set by setArguments(). The OpenMode is set to mode.

This method is an alias for start(), and exists only to fully implement the interface defined by QIODevice.

See also start(), setProgram(), and setArguments().
*/
impl /*struct*/ QProcess {
  pub fn open_0<RetType, T: QProcess_open_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_0(self);
    // return 1;
  }
}
pub trait QProcess_open_0<RetType> {
  fn open_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_open_0<bool> for (i32) {
  fn open_0(self , rsthis: & QProcess) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess4openE6QFlagsIN9QIODevice12OpenModeFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:169
// index:0
// Public Visibility=Default Availability=Available
// [8] QString program() const

/*
Returns the program the process was last started with.

This function was introduced in  Qt 5.0.

See also setProgram() and start().
*/
impl /*struct*/ QProcess {
  pub fn program_0<RetType, T: QProcess_program_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.program_0(self);
    // return 1;
  }
}
pub trait QProcess_program_0<RetType> {
  fn program_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_program_0<usize> for () {
  fn program_0(self , rsthis: & QProcess) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess7programEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:170
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setProgram(const QString &)

/*
Set the program to use when starting the process. This function must be called before start().

This function was introduced in  Qt 5.1.

See also start(), setArguments(), and program().
*/
impl /*struct*/ QProcess {
  pub fn setProgram_0<RetType, T: QProcess_setProgram_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setProgram_0(self);
    // return 1;
  }
}
pub trait QProcess_setProgram_0<RetType> {
  fn setProgram_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setProgram_0<(/*void*/)> for (usize) {
  fn setProgram_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess10setProgramERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:172
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList arguments() const

/*
Returns the command line arguments the process was last started with.

This function was introduced in  Qt 5.0.

See also setArguments() and start().
*/
impl /*struct*/ QProcess {
  pub fn arguments_0<RetType, T: QProcess_arguments_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arguments_0(self);
    // return 1;
  }
}
pub trait QProcess_arguments_0<RetType> {
  fn arguments_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_arguments_0<usize> for () {
  fn arguments_0(self , rsthis: & QProcess) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess9argumentsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:173
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setArguments(const QStringList &)

/*
Set the arguments to pass to the called program when starting the process. This function must be called before start().

This function was introduced in  Qt 5.1.

See also start(), setProgram(), and arguments().
*/
impl /*struct*/ QProcess {
  pub fn setArguments_0<RetType, T: QProcess_setArguments_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setArguments_0(self);
    // return 1;
  }
}
pub trait QProcess_setArguments_0<RetType> {
  fn setArguments_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setArguments_0<(/*void*/)> for (usize) {
  fn setArguments_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess12setArgumentsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:175
// index:0
// Public Visibility=Default Availability=Available
// [4] QProcess::ProcessChannelMode readChannelMode() const

/*

*/
impl /*struct*/ QProcess {
  pub fn readChannelMode_0<RetType, T: QProcess_readChannelMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readChannelMode_0(self);
    // return 1;
  }
}
pub trait QProcess_readChannelMode_0<RetType> {
  fn readChannelMode_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_readChannelMode_0<i32> for () {
  fn readChannelMode_0(self , rsthis: & QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess15readChannelModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:176
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setReadChannelMode(QProcess::ProcessChannelMode)

/*

*/
impl /*struct*/ QProcess {
  pub fn setReadChannelMode_0<RetType, T: QProcess_setReadChannelMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setReadChannelMode_0(self);
    // return 1;
  }
}
pub trait QProcess_setReadChannelMode_0<RetType> {
  fn setReadChannelMode_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setReadChannelMode_0<(/*void*/)> for (i32) {
  fn setReadChannelMode_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess18setReadChannelModeENS_18ProcessChannelModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:177
// index:0
// Public Visibility=Default Availability=Available
// [4] QProcess::ProcessChannelMode processChannelMode() const

/*
Returns the channel mode of the QProcess standard output and standard error channels.

This function was introduced in  Qt 4.2.

See also setProcessChannelMode(), ProcessChannelMode, and setReadChannel().
*/
impl /*struct*/ QProcess {
  pub fn processChannelMode_0<RetType, T: QProcess_processChannelMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.processChannelMode_0(self);
    // return 1;
  }
}
pub trait QProcess_processChannelMode_0<RetType> {
  fn processChannelMode_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_processChannelMode_0<i32> for () {
  fn processChannelMode_0(self , rsthis: & QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess18processChannelModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setProcessChannelMode(QProcess::ProcessChannelMode)

/*
Sets the channel mode of the QProcess standard output and standard error channels to the mode specified. This mode will be used the next time start() is called. For example:


  QProcess builder;
  builder.setProcessChannelMode(QProcess::MergedChannels);
  builder.start("make", QStringList() << "-j2");

  if (!builder.waitForFinished())
      qDebug() << "Make failed:" << builder.errorString();
  else
      qDebug() << "Make output:" << builder.readAll();



This function was introduced in  Qt 4.2.

See also processChannelMode(), ProcessChannelMode, and setReadChannel().
*/
impl /*struct*/ QProcess {
  pub fn setProcessChannelMode_0<RetType, T: QProcess_setProcessChannelMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setProcessChannelMode_0(self);
    // return 1;
  }
}
pub trait QProcess_setProcessChannelMode_0<RetType> {
  fn setProcessChannelMode_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setProcessChannelMode_0<(/*void*/)> for (i32) {
  fn setProcessChannelMode_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess21setProcessChannelModeENS_18ProcessChannelModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:179
// index:0
// Public Visibility=Default Availability=Available
// [4] QProcess::InputChannelMode inputChannelMode() const

/*
Returns the channel mode of the QProcess standard input channel.

This function was introduced in  Qt 5.2.

See also setInputChannelMode() and InputChannelMode.
*/
impl /*struct*/ QProcess {
  pub fn inputChannelMode_0<RetType, T: QProcess_inputChannelMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputChannelMode_0(self);
    // return 1;
  }
}
pub trait QProcess_inputChannelMode_0<RetType> {
  fn inputChannelMode_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_inputChannelMode_0<i32> for () {
  fn inputChannelMode_0(self , rsthis: & QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess16inputChannelModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:180
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInputChannelMode(QProcess::InputChannelMode)

/*
Sets the channel mode of the QProcess standard input channel to the mode specified. This mode will be used the next time start() is called.

This function was introduced in  Qt 5.2.

See also inputChannelMode() and InputChannelMode.
*/
impl /*struct*/ QProcess {
  pub fn setInputChannelMode_0<RetType, T: QProcess_setInputChannelMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInputChannelMode_0(self);
    // return 1;
  }
}
pub trait QProcess_setInputChannelMode_0<RetType> {
  fn setInputChannelMode_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setInputChannelMode_0<(/*void*/)> for (i32) {
  fn setInputChannelMode_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess19setInputChannelModeENS_16InputChannelModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:182
// index:0
// Public Visibility=Default Availability=Available
// [4] QProcess::ProcessChannel readChannel() const

/*
Returns the current read channel of the QProcess.

See also setReadChannel().
*/
impl /*struct*/ QProcess {
  pub fn readChannel_0<RetType, T: QProcess_readChannel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readChannel_0(self);
    // return 1;
  }
}
pub trait QProcess_readChannel_0<RetType> {
  fn readChannel_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_readChannel_0<i32> for () {
  fn readChannel_0(self , rsthis: & QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess11readChannelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:183
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setReadChannel(QProcess::ProcessChannel)

/*
Sets the current read channel of the QProcess to the given channel. The current input channel is used by the functions read(), readAll(), readLine(), and getChar(). It also determines which channel triggers QProcess to emit readyRead().

See also readChannel().
*/
impl /*struct*/ QProcess {
  pub fn setReadChannel_0<RetType, T: QProcess_setReadChannel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setReadChannel_0(self);
    // return 1;
  }
}
pub trait QProcess_setReadChannel_0<RetType> {
  fn setReadChannel_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setReadChannel_0<(/*void*/)> for (i32) {
  fn setReadChannel_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess14setReadChannelENS_14ProcessChannelE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:185
// index:0
// Public Visibility=Default Availability=Available
// [-2] void closeReadChannel(QProcess::ProcessChannel)

/*
Closes the read channel channel. After calling this function, QProcess will no longer receive data on the channel. Any data that has already been received is still available for reading.

Call this function to save memory, if you are not interested in the output of the process.

See also closeWriteChannel() and setReadChannel().
*/
impl /*struct*/ QProcess {
  pub fn closeReadChannel_0<RetType, T: QProcess_closeReadChannel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeReadChannel_0(self);
    // return 1;
  }
}
pub trait QProcess_closeReadChannel_0<RetType> {
  fn closeReadChannel_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_closeReadChannel_0<(/*void*/)> for (i32) {
  fn closeReadChannel_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess16closeReadChannelENS_14ProcessChannelE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:186
// index:0
// Public Visibility=Default Availability=Available
// [-2] void closeWriteChannel()

/*
Schedules the write channel of QProcess to be closed. The channel will close once all data has been written to the process. After calling this function, any attempts to write to the process will fail.

Closing the write channel is necessary for programs that read input data until the channel has been closed. For example, the program "more" is used to display text data in a console on both Unix and Windows. But it will not display the text data until QProcess's write channel has been closed. Example:


  QProcess more;
  more.start("more");
  more.write("Text to display");
  more.closeWriteChannel();
  // QProcess will emit readyRead() once "more" starts printing



The write channel is implicitly opened when start() is called.

See also closeReadChannel().
*/
impl /*struct*/ QProcess {
  pub fn closeWriteChannel_0<RetType, T: QProcess_closeWriteChannel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeWriteChannel_0(self);
    // return 1;
  }
}
pub trait QProcess_closeWriteChannel_0<RetType> {
  fn closeWriteChannel_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_closeWriteChannel_0<(/*void*/)> for () {
  fn closeWriteChannel_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QProcess17closeWriteChannelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:188
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStandardInputFile(const QString &)

/*
Redirects the process' standard input to the file indicated by fileName. When an input redirection is in place, the QProcess object will be in read-only mode (calling write() will result in error).

To make the process read EOF right away, pass nullDevice() here. This is cleaner than using closeWriteChannel() before writing any data, because it can be set up prior to starting the process.

If the file fileName does not exist at the moment start() is called or is not readable, starting the process will fail.

Calling setStandardInputFile() after the process has started has no effect.

This function was introduced in  Qt 4.2.

See also setStandardOutputFile(), setStandardErrorFile(), and setStandardOutputProcess().
*/
impl /*struct*/ QProcess {
  pub fn setStandardInputFile_0<RetType, T: QProcess_setStandardInputFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStandardInputFile_0(self);
    // return 1;
  }
}
pub trait QProcess_setStandardInputFile_0<RetType> {
  fn setStandardInputFile_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setStandardInputFile_0<(/*void*/)> for (usize) {
  fn setStandardInputFile_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess20setStandardInputFileERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:189
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStandardOutputFile(const QString &, QIODevice::OpenMode)

/*
Redirects the process' standard output to the file fileName. When the redirection is in place, the standard output read channel is closed: reading from it using read() will always fail, as will readAllStandardOutput().

To discard all standard output from the process, pass nullDevice() here. This is more efficient than simply never reading the standard output, as no QProcess buffers are filled.

If the file fileName doesn't exist at the moment start() is called, it will be created. If it cannot be created, the starting will fail.

If the file exists and mode is QIODevice::Truncate, the file will be truncated. Otherwise (if mode is QIODevice::Append), the file will be appended to.

Calling setStandardOutputFile() after the process has started has no effect.

This function was introduced in  Qt 4.2.

See also setStandardInputFile(), setStandardErrorFile(), and setStandardOutputProcess().
*/
impl /*struct*/ QProcess {
  pub fn setStandardOutputFile_0<RetType, T: QProcess_setStandardOutputFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStandardOutputFile_0(self);
    // return 1;
  }
}
pub trait QProcess_setStandardOutputFile_0<RetType> {
  fn setStandardOutputFile_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setStandardOutputFile_0<(/*void*/)> for (usize,i32) {
  fn setStandardOutputFile_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess21setStandardOutputFileERK7QString6QFlagsIN9QIODevice12OpenModeFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStandardErrorFile(const QString &, QIODevice::OpenMode)

/*
Redirects the process' standard error to the file fileName. When the redirection is in place, the standard error read channel is closed: reading from it using read() will always fail, as will readAllStandardError(). The file will be appended to if mode is Append, otherwise, it will be truncated.

See setStandardOutputFile() for more information on how the file is opened.

Note: if setProcessChannelMode() was called with an argument of QProcess::MergedChannels, this function has no effect.

This function was introduced in  Qt 4.2.

See also setStandardInputFile(), setStandardOutputFile(), and setStandardOutputProcess().
*/
impl /*struct*/ QProcess {
  pub fn setStandardErrorFile_0<RetType, T: QProcess_setStandardErrorFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStandardErrorFile_0(self);
    // return 1;
  }
}
pub trait QProcess_setStandardErrorFile_0<RetType> {
  fn setStandardErrorFile_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setStandardErrorFile_0<(/*void*/)> for (usize,i32) {
  fn setStandardErrorFile_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess20setStandardErrorFileERK7QString6QFlagsIN9QIODevice12OpenModeFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:191
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStandardOutputProcess(QProcess *)

/*
Pipes the standard output stream of this process to the destination process' standard input.

The following shell command:


  command1 | command2



Can be accomplished with QProcess with the following code:


  QProcess process1;
  QProcess process2;

  process1.setStandardOutputProcess(&process2);

  process1.start("command1");
  process2.start("command2");



This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QProcess {
  pub fn setStandardOutputProcess_0<RetType, T: QProcess_setStandardOutputProcess_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStandardOutputProcess_0(self);
    // return 1;
  }
}
pub trait QProcess_setStandardOutputProcess_0<RetType> {
  fn setStandardOutputProcess_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setStandardOutputProcess_0<(/*void*/)> for (usize) {
  fn setStandardOutputProcess_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess24setStandardOutputProcessEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:214
// index:0
// Public Visibility=Default Availability=Available
// [8] QString workingDirectory() const

/*
If QProcess has been assigned a working directory, this function returns the working directory that the QProcess will enter before the program has started. Otherwise, (i.e., no directory has been assigned,) an empty string is returned, and QProcess will use the application's current working directory instead.

See also setWorkingDirectory().
*/
impl /*struct*/ QProcess {
  pub fn workingDirectory_0<RetType, T: QProcess_workingDirectory_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.workingDirectory_0(self);
    // return 1;
  }
}
pub trait QProcess_workingDirectory_0<RetType> {
  fn workingDirectory_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_workingDirectory_0<usize> for () {
  fn workingDirectory_0(self , rsthis: & QProcess) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess16workingDirectoryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:215
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWorkingDirectory(const QString &)

/*
Sets the working directory to dir. QProcess will start the process in this directory. The default behavior is to start the process in the working directory of the calling process.

Note: On QNX, this may cause all application threads to temporarily freeze.

See also workingDirectory() and start().
*/
impl /*struct*/ QProcess {
  pub fn setWorkingDirectory_0<RetType, T: QProcess_setWorkingDirectory_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWorkingDirectory_0(self);
    // return 1;
  }
}
pub trait QProcess_setWorkingDirectory_0<RetType> {
  fn setWorkingDirectory_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setWorkingDirectory_0<(/*void*/)> for (usize) {
  fn setWorkingDirectory_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess19setWorkingDirectoryERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:217
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEnvironment(const QStringList &)

/*

*/
impl /*struct*/ QProcess {
  pub fn setEnvironment_0<RetType, T: QProcess_setEnvironment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEnvironment_0(self);
    // return 1;
  }
}
pub trait QProcess_setEnvironment_0<RetType> {
  fn setEnvironment_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setEnvironment_0<(/*void*/)> for (usize) {
  fn setEnvironment_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess14setEnvironmentERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:218
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList environment() const

/*

*/
impl /*struct*/ QProcess {
  pub fn environment_0<RetType, T: QProcess_environment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.environment_0(self);
    // return 1;
  }
}
pub trait QProcess_environment_0<RetType> {
  fn environment_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_environment_0<usize> for () {
  fn environment_0(self , rsthis: & QProcess) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess11environmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:219
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setProcessEnvironment(const QProcessEnvironment &)

/*
Sets the environment that QProcess will pass to the child process.

For example, the following code adds the environment variable TMPDIR:


  QProcess process;
  QProcessEnvironment env = QProcessEnvironment::systemEnvironment();
  env.insert("TMPDIR", "C:\\MyApp\\temp"); // Add an environment variable
  process.setProcessEnvironment(env);
  process.start("myapp");



Note how, on Windows, environment variable names are case-insensitive.

This function was introduced in  Qt 4.6.

See also processEnvironment(), QProcessEnvironment::systemEnvironment(), and setEnvironment().
*/
impl /*struct*/ QProcess {
  pub fn setProcessEnvironment_0<RetType, T: QProcess_setProcessEnvironment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setProcessEnvironment_0(self);
    // return 1;
  }
}
pub trait QProcess_setProcessEnvironment_0<RetType> {
  fn setProcessEnvironment_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setProcessEnvironment_0<(/*void*/)> for (usize) {
  fn setProcessEnvironment_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess21setProcessEnvironmentERK19QProcessEnvironment", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:220
// index:0
// Public Visibility=Default Availability=Available
// [8] QProcessEnvironment processEnvironment() const

/*
Returns the environment that QProcess will pass to its child process, or an empty object if no environment has been set using setEnvironment() or setProcessEnvironment(). If no environment has been set, the environment of the calling process will be used.

This function was introduced in  Qt 4.6.

See also setProcessEnvironment(), setEnvironment(), and QProcessEnvironment::isEmpty().
*/
impl /*struct*/ QProcess {
  pub fn processEnvironment_0<RetType, T: QProcess_processEnvironment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.processEnvironment_0(self);
    // return 1;
  }
}
pub trait QProcess_processEnvironment_0<RetType> {
  fn processEnvironment_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_processEnvironment_0<usize> for () {
  fn processEnvironment_0(self , rsthis: & QProcess) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess18processEnvironmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:222
// index:0
// Public Visibility=Default Availability=Available
// [4] QProcess::ProcessError error() const

/*
Returns the type of error that occurred last.

See also state().
*/
impl /*struct*/ QProcess {
  pub fn error_0<RetType, T: QProcess_error_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.error_0(self);
    // return 1;
  }
}
pub trait QProcess_error_0<RetType> {
  fn error_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_error_0<i32> for () {
  fn error_0(self , rsthis: & QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess5errorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:275
// index:1
// Public Visibility=Default Availability=Available
// [-2] void error(QProcess::ProcessError)

/*
Returns the type of error that occurred last.

See also state().
*/
impl /*struct*/ QProcess {
  pub fn error_1<RetType, T: QProcess_error_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.error_1(self);
    // return 1;
  }
}
pub trait QProcess_error_1<RetType> {
  fn error_1(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_error_1<(/*void*/)> for (i32) {
  fn error_1(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess5errorENS_12ProcessErrorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:223
// index:0
// Public Visibility=Default Availability=Available
// [4] QProcess::ProcessState state() const

/*
Returns the current state of the process.

See also stateChanged() and error().
*/
impl /*struct*/ QProcess {
  pub fn state_0<RetType, T: QProcess_state_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.state_0(self);
    // return 1;
  }
}
pub trait QProcess_state_0<RetType> {
  fn state_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_state_0<i32> for () {
  fn state_0(self , rsthis: & QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess5stateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:226
// index:0
// Public Visibility=Default Availability=Available
// [8] Q_PID pid() const

/*

*/
impl /*struct*/ QProcess {
  pub fn pid_0<RetType, T: QProcess_pid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pid_0(self);
    // return 1;
  }
}
pub trait QProcess_pid_0<RetType> {
  fn pid_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_pid_0<i64> for () {
  fn pid_0(self , rsthis: & QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess3pidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:227
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 processId() const

/*
Returns the native process identifier for the running process, if available. If no process is currently running, 0 is returned.

This function was introduced in  Qt 5.3.
*/
impl /*struct*/ QProcess {
  pub fn processId_0<RetType, T: QProcess_processId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.processId_0(self);
    // return 1;
  }
}
pub trait QProcess_processId_0<RetType> {
  fn processId_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_processId_0<i64> for () {
  fn processId_0(self , rsthis: & QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess9processIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:229
// index:0
// Public Visibility=Default Availability=Available
// [1] bool waitForStarted(int)

/*
Blocks until the process has started and the started() signal has been emitted, or until msecs milliseconds have passed.

Returns true if the process was started successfully; otherwise returns false (if the operation timed out or if an error occurred).

This function can operate without an event loop. It is useful when writing non-GUI applications and when performing I/O operations in a non-GUI thread.

Warning: Calling this function from the main (GUI) thread might cause your user interface to freeze.

If msecs is -1, this function will not time out.

Note: On some UNIX operating systems, this function may return true but the process may later report a QProcess::FailedToStart error.

See also started(), waitForReadyRead(), waitForBytesWritten(), and waitForFinished().
*/
impl /*struct*/ QProcess {
  pub fn waitForStarted_0<RetType, T: QProcess_waitForStarted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.waitForStarted_0(self);
    // return 1;
  }
}
pub trait QProcess_waitForStarted_0<RetType> {
  fn waitForStarted_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_waitForStarted_0<bool> for (i32) {
  fn waitForStarted_0(self , rsthis: & QProcess) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess14waitForStartedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:230
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool waitForReadyRead(int)

/*
Reimplemented from QIODevice::waitForReadyRead().
*/
impl /*struct*/ QProcess {
  pub fn waitForReadyRead_0<RetType, T: QProcess_waitForReadyRead_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.waitForReadyRead_0(self);
    // return 1;
  }
}
pub trait QProcess_waitForReadyRead_0<RetType> {
  fn waitForReadyRead_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_waitForReadyRead_0<bool> for (i32) {
  fn waitForReadyRead_0(self , rsthis: & QProcess) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess16waitForReadyReadEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:231
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool waitForBytesWritten(int)

/*
Reimplemented from QIODevice::waitForBytesWritten().
*/
impl /*struct*/ QProcess {
  pub fn waitForBytesWritten_0<RetType, T: QProcess_waitForBytesWritten_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.waitForBytesWritten_0(self);
    // return 1;
  }
}
pub trait QProcess_waitForBytesWritten_0<RetType> {
  fn waitForBytesWritten_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_waitForBytesWritten_0<bool> for (i32) {
  fn waitForBytesWritten_0(self , rsthis: & QProcess) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess19waitForBytesWrittenEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:232
// index:0
// Public Visibility=Default Availability=Available
// [1] bool waitForFinished(int)

/*
Blocks until the process has finished and the finished() signal has been emitted, or until msecs milliseconds have passed.

Returns true if the process finished; otherwise returns false (if the operation timed out, if an error occurred, or if this QProcess is already finished).

This function can operate without an event loop. It is useful when writing non-GUI applications and when performing I/O operations in a non-GUI thread.

Warning: Calling this function from the main (GUI) thread might cause your user interface to freeze.

If msecs is -1, this function will not time out.

See also finished(), waitForStarted(), waitForReadyRead(), and waitForBytesWritten().
*/
impl /*struct*/ QProcess {
  pub fn waitForFinished_0<RetType, T: QProcess_waitForFinished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.waitForFinished_0(self);
    // return 1;
  }
}
pub trait QProcess_waitForFinished_0<RetType> {
  fn waitForFinished_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_waitForFinished_0<bool> for (i32) {
  fn waitForFinished_0(self , rsthis: & QProcess) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess15waitForFinishedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:234
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray readAllStandardOutput()

/*
Regardless of the current read channel, this function returns all data available from the standard output of the process as a QByteArray.

See also readyReadStandardOutput(), readAllStandardError(), readChannel(), and setReadChannel().
*/
impl /*struct*/ QProcess {
  pub fn readAllStandardOutput_0<RetType, T: QProcess_readAllStandardOutput_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readAllStandardOutput_0(self);
    // return 1;
  }
}
pub trait QProcess_readAllStandardOutput_0<RetType> {
  fn readAllStandardOutput_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_readAllStandardOutput_0<usize> for () {
  fn readAllStandardOutput_0(self , rsthis: & QProcess) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess21readAllStandardOutputEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:235
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray readAllStandardError()

/*
Regardless of the current read channel, this function returns all data available from the standard error of the process as a QByteArray.

See also readyReadStandardError(), readAllStandardOutput(), readChannel(), and setReadChannel().
*/
impl /*struct*/ QProcess {
  pub fn readAllStandardError_0<RetType, T: QProcess_readAllStandardError_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readAllStandardError_0(self);
    // return 1;
  }
}
pub trait QProcess_readAllStandardError_0<RetType> {
  fn readAllStandardError_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_readAllStandardError_0<usize> for () {
  fn readAllStandardError_0(self , rsthis: & QProcess) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess20readAllStandardErrorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:237
// index:0
// Public Visibility=Default Availability=Available
// [4] int exitCode() const

/*
Returns the exit code of the last process that finished.

This value is not valid unless exitStatus() returns NormalExit.
*/
impl /*struct*/ QProcess {
  pub fn exitCode_0<RetType, T: QProcess_exitCode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exitCode_0(self);
    // return 1;
  }
}
pub trait QProcess_exitCode_0<RetType> {
  fn exitCode_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_exitCode_0<i32> for () {
  fn exitCode_0(self , rsthis: & QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess8exitCodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:238
// index:0
// Public Visibility=Default Availability=Available
// [4] QProcess::ExitStatus exitStatus() const

/*
Returns the exit status of the last process that finished.

On Windows, if the process was terminated with TerminateProcess() from another application, this function will still return NormalExit unless the exit code is less than 0.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QProcess {
  pub fn exitStatus_0<RetType, T: QProcess_exitStatus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exitStatus_0(self);
    // return 1;
  }
}
pub trait QProcess_exitStatus_0<RetType> {
  fn exitStatus_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_exitStatus_0<i32> for () {
  fn exitStatus_0(self , rsthis: & QProcess) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess10exitStatusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:241
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] qint64 bytesAvailable() const

/*
Reimplemented from QIODevice::bytesAvailable().
*/
impl /*struct*/ QProcess {
  pub fn bytesAvailable_0<RetType, T: QProcess_bytesAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bytesAvailable_0(self);
    // return 1;
  }
}
pub trait QProcess_bytesAvailable_0<RetType> {
  fn bytesAvailable_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_bytesAvailable_0<i64> for () {
  fn bytesAvailable_0(self , rsthis: & QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess14bytesAvailableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:242
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] qint64 bytesToWrite() const

/*
Reimplemented from QIODevice::bytesToWrite().
*/
impl /*struct*/ QProcess {
  pub fn bytesToWrite_0<RetType, T: QProcess_bytesToWrite_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bytesToWrite_0(self);
    // return 1;
  }
}
pub trait QProcess_bytesToWrite_0<RetType> {
  fn bytesToWrite_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_bytesToWrite_0<i64> for () {
  fn bytesToWrite_0(self , rsthis: & QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess12bytesToWriteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:243
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isSequential() const

/*
Reimplemented from QIODevice::isSequential().
*/
impl /*struct*/ QProcess {
  pub fn isSequential_0<RetType, T: QProcess_isSequential_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSequential_0(self);
    // return 1;
  }
}
pub trait QProcess_isSequential_0<RetType> {
  fn isSequential_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_isSequential_0<bool> for () {
  fn isSequential_0(self , rsthis: & QProcess) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess12isSequentialEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:244
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool canReadLine() const

/*
Reimplemented from QIODevice::canReadLine().

This function operates on the current read channel.

See also readChannel() and setReadChannel().
*/
impl /*struct*/ QProcess {
  pub fn canReadLine_0<RetType, T: QProcess_canReadLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canReadLine_0(self);
    // return 1;
  }
}
pub trait QProcess_canReadLine_0<RetType> {
  fn canReadLine_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_canReadLine_0<bool> for () {
  fn canReadLine_0(self , rsthis: & QProcess) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess11canReadLineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:245
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void close()

/*
Reimplemented from QIODevice::close().

Closes all communication with the process and kills it. After calling this function, QProcess will no longer emit readyRead(), and data can no longer be read or written.
*/
impl /*struct*/ QProcess {
  pub fn close_0<RetType, T: QProcess_close_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.close_0(self);
    // return 1;
  }
}
pub trait QProcess_close_0<RetType> {
  fn close_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_close_0<(/*void*/)> for () {
  fn close_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QProcess5closeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:246
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool atEnd() const

/*
Reimplemented from QIODevice::atEnd().

Returns true if the process is not running, and no more data is available for reading; otherwise returns false.
*/
impl /*struct*/ QProcess {
  pub fn atEnd_0<RetType, T: QProcess_atEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.atEnd_0(self);
    // return 1;
  }
}
pub trait QProcess_atEnd_0<RetType> {
  fn atEnd_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_atEnd_0<bool> for () {
  fn atEnd_0(self , rsthis: & QProcess) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QProcess5atEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:248
// index:0
// Public static Visibility=Default Availability=Available
// [4] int execute(const QString &, const QStringList &)

/*
Starts the program program with the arguments arguments in a new process, waits for it to finish, and then returns the exit code of the process. Any data the new process writes to the console is forwarded to the calling process.

The environment and working directory are inherited from the calling process.

Argument handling is identical to the respective start() overload.

If the process cannot be started, -2 is returned. If the process crashes, -1 is returned. Otherwise, the process' exit code is returned.

See also start().
*/
impl /*struct*/ QProcess {
  pub fn execute_0<RetType, T: QProcess_execute_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.execute_0();
    // return 1;
  }
}
pub trait QProcess_execute_0<RetType> {
  fn execute_0(self ) -> RetType;
}
impl<'a> /*trait*/ QProcess_execute_0<i32> for (usize,usize) {
  fn execute_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess7executeERK7QStringRK11QStringList", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:249
// index:1
// Public static Visibility=Default Availability=Available
// [4] int execute(const QString &)

/*
Starts the program program with the arguments arguments in a new process, waits for it to finish, and then returns the exit code of the process. Any data the new process writes to the console is forwarded to the calling process.

The environment and working directory are inherited from the calling process.

Argument handling is identical to the respective start() overload.

If the process cannot be started, -2 is returned. If the process crashes, -1 is returned. Otherwise, the process' exit code is returned.

See also start().
*/
impl /*struct*/ QProcess {
  pub fn execute_1<RetType, T: QProcess_execute_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.execute_1();
    // return 1;
  }
}
pub trait QProcess_execute_1<RetType> {
  fn execute_1(self ) -> RetType;
}
impl<'a> /*trait*/ QProcess_execute_1<i32> for (usize) {
  fn execute_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess7executeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:262
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList systemEnvironment()

/*
Returns the environment of the calling process as a list of key=value pairs. Example:


  QStringList environment = QProcess::systemEnvironment();
  // environment = {"PATH=/usr/bin:/usr/local/bin",
  //                "USER=greg", "HOME=/home/greg"}



This function does not cache the system environment. Therefore, it's possible to obtain an updated version of the environment if low-level C library functions like setenv or putenv have been called.

However, note that repeated calls to this function will recreate the list of environment variables, which is a non-trivial operation.

Note: For new code, it is recommended to use QProcessEnvironment::systemEnvironment()

This function was introduced in  Qt 4.1.

See also QProcessEnvironment::systemEnvironment() and setProcessEnvironment().
*/
impl /*struct*/ QProcess {
  pub fn systemEnvironment_0<RetType, T: QProcess_systemEnvironment_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.systemEnvironment_0();
    // return 1;
  }
}
pub trait QProcess_systemEnvironment_0<RetType> {
  fn systemEnvironment_0(self ) -> RetType;
}
impl<'a> /*trait*/ QProcess_systemEnvironment_0<usize> for () {
  fn systemEnvironment_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess17systemEnvironmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:264
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString nullDevice()

/*
The null device of the operating system.

The returned file path uses native directory separators.

This function was introduced in  Qt 5.2.

See also QProcess::setStandardInputFile(), QProcess::setStandardOutputFile(), and QProcess::setStandardErrorFile().
*/
impl /*struct*/ QProcess {
  pub fn nullDevice_0<RetType, T: QProcess_nullDevice_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.nullDevice_0();
    // return 1;
  }
}
pub trait QProcess_nullDevice_0<RetType> {
  fn nullDevice_0(self ) -> RetType;
}
impl<'a> /*trait*/ QProcess_nullDevice_0<usize> for () {
  fn nullDevice_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess10nullDeviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:267
// index:0
// Public Visibility=Default Availability=Available
// [-2] void terminate()

/*
Attempts to terminate the process.

The process may not exit as a result of calling this function (it is given the chance to prompt the user for any unsaved files, etc).

On Windows, terminate() posts a WM_CLOSE message to all top-level windows of the process and then to the main thread of the process itself. On Unix and macOS the SIGTERM signal is sent.

Console applications on Windows that do not run an event loop, or whose event loop does not handle the WM_CLOSE message, can only be terminated by calling kill().

See also kill().
*/
impl /*struct*/ QProcess {
  pub fn terminate_0<RetType, T: QProcess_terminate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.terminate_0(self);
    // return 1;
  }
}
pub trait QProcess_terminate_0<RetType> {
  fn terminate_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_terminate_0<(/*void*/)> for () {
  fn terminate_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QProcess9terminateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:268
// index:0
// Public Visibility=Default Availability=Available
// [-2] void kill()

/*
Kills the current process, causing it to exit immediately.

On Windows, kill() uses TerminateProcess, and on Unix and macOS, the SIGKILL signal is sent to the process.

See also terminate().
*/
impl /*struct*/ QProcess {
  pub fn kill_0<RetType, T: QProcess_kill_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.kill_0(self);
    // return 1;
  }
}
pub trait QProcess_kill_0<RetType> {
  fn kill_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_kill_0<(/*void*/)> for () {
  fn kill_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QProcess4killEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:272
// index:0
// Public Visibility=Default Availability=Available
// [-2] void finished(int)

/*
This signal is emitted when the process finishes. exitCode is the exit code of the process (only valid for normal exits), and exitStatus is the exit status. After the process has finished, the buffers in QProcess are still intact. You can still read any data that the process may have written before it finished.

Note: Signal finished is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(process, QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
      [=](int exitCode, QProcess::ExitStatus exitStatus){ /-* ... *-/ });



See also exitStatus().
*/
impl /*struct*/ QProcess {
  pub fn finished_0<RetType, T: QProcess_finished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.finished_0(self);
    // return 1;
  }
}
pub trait QProcess_finished_0<RetType> {
  fn finished_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_finished_0<(/*void*/)> for (i32) {
  fn finished_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess8finishedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:273
// index:1
// Public Visibility=Default Availability=Available
// [-2] void finished(int, QProcess::ExitStatus)

/*
This signal is emitted when the process finishes. exitCode is the exit code of the process (only valid for normal exits), and exitStatus is the exit status. After the process has finished, the buffers in QProcess are still intact. You can still read any data that the process may have written before it finished.

Note: Signal finished is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(process, QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
      [=](int exitCode, QProcess::ExitStatus exitStatus){ /-* ... *-/ });



See also exitStatus().
*/
impl /*struct*/ QProcess {
  pub fn finished_1<RetType, T: QProcess_finished_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.finished_1(self);
    // return 1;
  }
}
pub trait QProcess_finished_1<RetType> {
  fn finished_1(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_finished_1<(/*void*/)> for (i32,i32) {
  fn finished_1(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess8finishedEiNS_10ExitStatusE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:277
// index:0
// Public Visibility=Default Availability=Available
// [-2] void errorOccurred(QProcess::ProcessError)

/*
This signal is emitted when an error occurs with the process. The specified error describes the type of error that occurred.

This function was introduced in  Qt 5.6.
*/
impl /*struct*/ QProcess {
  pub fn errorOccurred_0<RetType, T: QProcess_errorOccurred_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorOccurred_0(self);
    // return 1;
  }
}
pub trait QProcess_errorOccurred_0<RetType> {
  fn errorOccurred_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_errorOccurred_0<(/*void*/)> for (i32) {
  fn errorOccurred_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess13errorOccurredENS_12ProcessErrorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:284
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setProcessState(QProcess::ProcessState)

/*
Sets the current state of the QProcess to the state specified.

See also state().
*/
impl /*struct*/ QProcess {
  pub fn setProcessState_0<RetType, T: QProcess_setProcessState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setProcessState_0(self);
    // return 1;
  }
}
pub trait QProcess_setProcessState_0<RetType> {
  fn setProcessState_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setProcessState_0<(/*void*/)> for (i32) {
  fn setProcessState_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QProcess15setProcessStateENS_12ProcessStateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:286
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void setupChildProcess()

/*
This function is called in the child process context just before the program is executed on Unix or macOS (i.e., after fork(), but before execve()). Reimplement this function to do last minute initialization of the child process. Example:


  class SandboxProcess : public QProcess
  {
      ...
   protected:
       void setupChildProcess();
      ...
  };

  void SandboxProcess::setupChildProcess()
  {
      // Drop all privileges in the child process, and enter
      // a chroot jail.
  #if defined Q_OS_UNIX
      ::setgroups(0, 0);
      ::chroot("/etc/safe");
      ::chdir("/");
      ::setgid(safeGid);
      ::setuid(safeUid);
      ::umask(0);
  #endif
  }



You cannot exit the process (by calling exit(), for instance) from this function. If you need to stop the program before it starts execution, your workaround is to emit finished() and then call exit().

Warning: This function is called by QProcess on Unix and macOS only. On Windows and QNX, it is not called.
*/
impl /*struct*/ QProcess {
  pub fn setupChildProcess_0<RetType, T: QProcess_setupChildProcess_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setupChildProcess_0(self);
    // return 1;
  }
}
pub trait QProcess_setupChildProcess_0<RetType> {
  fn setupChildProcess_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_setupChildProcess_0<(/*void*/)> for () {
  fn setupChildProcess_0(self , rsthis: & QProcess) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QProcess17setupChildProcessEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:289
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] qint64 readData(char *, qint64)

/*
Reimplemented from QIODevice::readData().
*/
impl /*struct*/ QProcess {
  pub fn readData_0<RetType, T: QProcess_readData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readData_0(self);
    // return 1;
  }
}
pub trait QProcess_readData_0<RetType> {
  fn readData_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_readData_0<i64> for (usize,i64) {
  fn readData_0(self , rsthis: & QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess8readDataEPcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:290
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] qint64 writeData(const char *, qint64)

/*
Reimplemented from QIODevice::writeData().
*/
impl /*struct*/ QProcess {
  pub fn writeData_0<RetType, T: QProcess_writeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeData_0(self);
    // return 1;
  }
}
pub trait QProcess_writeData_0<RetType> {
  fn writeData_0(self , rsthis: & QProcess) -> RetType;
}
impl<'a> /*trait*/ QProcess_writeData_0<i64> for (usize,i64) {
  fn writeData_0(self , rsthis: & QProcess) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QProcess9writeDataEPKcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}


/*
This enum describes the different types of errors that are reported by QProcess.



See also error().

*/
pub type QProcess__ProcessError = i32;
// The process failed to start. Either the invoked program is missing, or you may have insufficient permissions to invoke the program.
pub const QProcess__FailedToStart :QProcess__ProcessError = 0;
// The process crashed some time after starting successfully.
pub const QProcess__Crashed :QProcess__ProcessError = 1;
// The last waitFor...() function timed out. The state of QProcess is unchanged, and you can try calling waitFor...() again.
pub const QProcess__Timedout :QProcess__ProcessError = 2;
// An error occurred when attempting to read from the process. For example, the process may not be running.
pub const QProcess__ReadError :QProcess__ProcessError = 3;
// An error occurred when attempting to write to the process. For example, the process may not be running, or it may have closed its input channel.
pub const QProcess__WriteError :QProcess__ProcessError = 4;
// An unknown error occurred. This is the default return value of error().
pub const QProcess__UnknownError :QProcess__ProcessError = 5;
pub fn QProcess_ProcessErrorItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QProcess", val);
}
pub fn QProcess_ProcessErrorItemName_s(val: i32) ->String {
  //var nilthis *QProcess
  //return nilthis.ProcessErrorItemName(val);
  return QProcess_ProcessErrorItemName(val);
}


/*
This enum describes the different states of QProcess.



See also state().

*/
pub type QProcess__ProcessState = i32;
// The process is not running.
pub const QProcess__NotRunning :QProcess__ProcessState = 0;
// The process is starting, but the program has not yet been invoked.
pub const QProcess__Starting :QProcess__ProcessState = 1;
// The process is running and is ready for reading and writing.
pub const QProcess__Running :QProcess__ProcessState = 2;
pub fn QProcess_ProcessStateItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QProcess", val);
}
pub fn QProcess_ProcessStateItemName_s(val: i32) ->String {
  //var nilthis *QProcess
  //return nilthis.ProcessStateItemName(val);
  return QProcess_ProcessStateItemName(val);
}


/*
This enum describes the process channels used by the running process. Pass one of these values to setReadChannel() to set the current read channel of QProcess.



See also setReadChannel().

*/
pub type QProcess__ProcessChannel = i32;
// The standard output (stdout) of the running process.
pub const QProcess__StandardOutput :QProcess__ProcessChannel = 0;
// The standard error (stderr) of the running process.
pub const QProcess__StandardError :QProcess__ProcessChannel = 1;
pub fn QProcess_ProcessChannelItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QProcess", val);
}
pub fn QProcess_ProcessChannelItemName_s(val: i32) ->String {
  //var nilthis *QProcess
  //return nilthis.ProcessChannelItemName(val);
  return QProcess_ProcessChannelItemName(val);
}


/*
This enum describes the process output channel modes of QProcess. Pass one of these values to setProcessChannelMode() to set the current read channel mode.



Note: Windows intentionally suppresses output from GUI-only applications to inherited consoles. This does not apply to output redirected to files or pipes. To forward the output of GUI-only applications on the console nonetheless, you must use SeparateChannels and do the forwarding yourself by reading the output and writing it to the appropriate output channels.

See also setProcessChannelMode().

*/
pub type QProcess__ProcessChannelMode = i32;
// QProcess manages the output of the running process, keeping standard output and standard error data in separate internal buffers. You can select the QProcess's current read channel by calling setReadChannel(). This is the default channel mode of QProcess.
pub const QProcess__SeparateChannels :QProcess__ProcessChannelMode = 0;
// QProcess merges the output of the running process into the standard output channel (stdout). The standard error channel (stderr) will not receive any data. The standard output and standard error data of the running process are interleaved.
pub const QProcess__MergedChannels :QProcess__ProcessChannelMode = 1;
// QProcess forwards the output of the running process onto the main process. Anything the child process writes to its standard output and standard error will be written to the standard output and standard error of the main process.
pub const QProcess__ForwardedChannels :QProcess__ProcessChannelMode = 2;
// 
pub const QProcess__ForwardedOutputChannel :QProcess__ProcessChannelMode = 3;
// 
pub const QProcess__ForwardedErrorChannel :QProcess__ProcessChannelMode = 4;
pub fn QProcess_ProcessChannelModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QProcess", val);
}
pub fn QProcess_ProcessChannelModeItemName_s(val: i32) ->String {
  //var nilthis *QProcess
  //return nilthis.ProcessChannelModeItemName(val);
  return QProcess_ProcessChannelModeItemName(val);
}


/*
This enum describes the process input channel modes of QProcess. Pass one of these values to setInputChannelMode() to set the current write channel mode.



This enum was introduced or modified in  Qt 5.2.

See also setInputChannelMode().

*/
pub type QProcess__InputChannelMode = i32;
// QProcess manages the input of the running process. This is the default input channel mode of QProcess.
pub const QProcess__ManagedInputChannel :QProcess__InputChannelMode = 0;
// QProcess forwards the input of the main process onto the running process. The child process reads its standard input from the same source as the main process. Note that the main process must not try to read its standard input while the child process is running.
pub const QProcess__ForwardedInputChannel :QProcess__InputChannelMode = 1;
pub fn QProcess_InputChannelModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QProcess", val);
}
pub fn QProcess_InputChannelModeItemName_s(val: i32) ->String {
  //var nilthis *QProcess
  //return nilthis.InputChannelModeItemName(val);
  return QProcess_InputChannelModeItemName(val);
}


/*
This enum describes the different exit statuses of QProcess.



See also exitStatus().

*/
pub type QProcess__ExitStatus = i32;
// The process exited normally.
pub const QProcess__NormalExit :QProcess__ExitStatus = 0;
// The process crashed.
pub const QProcess__CrashExit :QProcess__ExitStatus = 1;
pub fn QProcess_ExitStatusItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QProcess", val);
}
pub fn QProcess_ExitStatusItemName_s(val: i32) ->String {
  //var nilthis *QProcess
  //return nilthis.ExitStatusItemName(val);
  return QProcess_ExitStatusItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
