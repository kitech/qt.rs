

// mod ::core::QCoreApplication
// package qtcore
// /usr/include/qt/QtCore/qcoreapplication.h
// #include <qcoreapplication.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 3
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// bool event(QEvent *)
// func (this *QCoreApplication) InheritEvent(f func(arg0 *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QCoreApplication)=16
pub struct QCoreApplication {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCoreApplication_ITF interface {
//    QObject_ITF
//    QCoreApplication_PTR() *QCoreApplication
//}
//func (ptr *QCoreApplication) QCoreApplication_PTR() *QCoreApplication { return ptr }

impl /*struct*/ QCoreApplication {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCoreApplication {
    return QCoreApplication{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCoreApplication {
//  type Target = QCoreApplicationBASE;
//
//  fn deref(&self) -> &QCoreApplicationBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCoreApplicationBASE> for QCoreApplication {
//  fn as_ref(& self) -> & QCoreApplicationBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qcoreapplication.h:78
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn metaObject_0<RetType, T: QCoreApplication_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QCoreApplication_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QCoreApplication) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QCoreApplication) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QCoreApplication10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QCoreApplication(int &, char **, int)

/*
Constructs a Qt core application. Core applications are applications without a graphical user interface. Such applications are used at the console or as server processes.

The argc and argv arguments are processed by the application, and made available in a more convenient form by the arguments() function.

Warning: The data referred to by argc and argv must stay valid for the entire lifetime of the QCoreApplication object. In addition, argc must be greater than zero and argv must contain at least one valid character string.
*/
// QCoreApplication(int &, char **, int) ctx.fn_proto_cpp
impl /*struct*/ QCoreApplication {
  pub fn QCoreApplication_0<T: QCoreApplication_QCoreApplication_0>(value: T) -> QCoreApplication {
    let rsthis = value.QCoreApplication_0();
    return rsthis;
    // return 1;
  }
}

pub trait QCoreApplication_QCoreApplication_0 {
  fn QCoreApplication_0(self) -> QCoreApplication;
}
// QCoreApplication(int &, char **, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCoreApplication_QCoreApplication_0 for (usize,usize,i32) {
  fn QCoreApplication_0(self) -> QCoreApplication {
    // unsafe{_ZN16QCoreApplicationC2ERiPPci()};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QCoreApplicationC2ERiPPci", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCoreApplication{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:97
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QCoreApplication()

/*

*/
pub fn DeleteQCoreApplication(this :*mut QCoreApplication) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QCoreApplicationD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qcoreapplication.h:99
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList arguments()

/*
Returns the list of command-line arguments.

Usually arguments().at(0) is the program name, arguments().at(1) is the first argument, and arguments().last() is the last argument. See the note below about Windows.

Calling this function is slow - you should store the result in a variable when parsing the command line.

Warning: On Unix, this list is built from the argc and argv parameters passed to the constructor in the main() function. The string-data in argv is interpreted using QString::fromLocal8Bit(); hence it is not possible to pass, for example, Japanese command line arguments on a system that runs in a Latin1 locale. Most modern Unix systems do not have this limitation, as they are Unicode-based.

On Windows, the list is built from the argc and argv parameters only if modified argv/argc parameters are passed to the constructor. In that case, encoding problems might occur.

Otherwise, the arguments() are constructed from the return value of GetCommandLine(). As a result of this, the string given by arguments().at(0) might not be the program name on Windows, depending on how the application was started.

This function was introduced in  Qt 4.1.

See also applicationFilePath() and QCommandLineParser.
*/
impl /*struct*/ QCoreApplication {
  pub fn arguments_0<RetType, T: QCoreApplication_arguments_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.arguments_0();
    // return 1;
  }
}
pub trait QCoreApplication_arguments_0<RetType> {
  fn arguments_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_arguments_0<usize> for () {
  fn arguments_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication9argumentsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:101
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setAttribute(Qt::ApplicationAttribute, bool)

/*
Sets the attribute attribute if on is true; otherwise clears the attribute.

See also testAttribute().
*/
impl /*struct*/ QCoreApplication {
  pub fn setAttribute_0<RetType, T: QCoreApplication_setAttribute_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setAttribute_0();
    // return 1;
  }
}
pub trait QCoreApplication_setAttribute_0<RetType> {
  fn setAttribute_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_setAttribute_0<(/*void*/)> for (i32,bool) {
  fn setAttribute_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication12setAttributeEN2Qt20ApplicationAttributeEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:102
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool testAttribute(Qt::ApplicationAttribute)

/*
Returns true if attribute attribute is set; otherwise returns false.

See also setAttribute().
*/
impl /*struct*/ QCoreApplication {
  pub fn testAttribute_0<RetType, T: QCoreApplication_testAttribute_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.testAttribute_0();
    // return 1;
  }
}
pub trait QCoreApplication_testAttribute_0<RetType> {
  fn testAttribute_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_testAttribute_0<bool> for (i32) {
  fn testAttribute_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication13testAttributeEN2Qt20ApplicationAttributeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:104
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setOrganizationDomain(const QString &)

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn setOrganizationDomain_0<RetType, T: QCoreApplication_setOrganizationDomain_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setOrganizationDomain_0();
    // return 1;
  }
}
pub trait QCoreApplication_setOrganizationDomain_0<RetType> {
  fn setOrganizationDomain_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_setOrganizationDomain_0<(/*void*/)> for (usize) {
  fn setOrganizationDomain_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication21setOrganizationDomainERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:105
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString organizationDomain()

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn organizationDomain_0<RetType, T: QCoreApplication_organizationDomain_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.organizationDomain_0();
    // return 1;
  }
}
pub trait QCoreApplication_organizationDomain_0<RetType> {
  fn organizationDomain_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_organizationDomain_0<usize> for () {
  fn organizationDomain_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication18organizationDomainEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:106
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setOrganizationName(const QString &)

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn setOrganizationName_0<RetType, T: QCoreApplication_setOrganizationName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setOrganizationName_0();
    // return 1;
  }
}
pub trait QCoreApplication_setOrganizationName_0<RetType> {
  fn setOrganizationName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_setOrganizationName_0<(/*void*/)> for (usize) {
  fn setOrganizationName_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication19setOrganizationNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:107
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString organizationName()

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn organizationName_0<RetType, T: QCoreApplication_organizationName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.organizationName_0();
    // return 1;
  }
}
pub trait QCoreApplication_organizationName_0<RetType> {
  fn organizationName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_organizationName_0<usize> for () {
  fn organizationName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication16organizationNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:108
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setApplicationName(const QString &)

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn setApplicationName_0<RetType, T: QCoreApplication_setApplicationName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setApplicationName_0();
    // return 1;
  }
}
pub trait QCoreApplication_setApplicationName_0<RetType> {
  fn setApplicationName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_setApplicationName_0<(/*void*/)> for (usize) {
  fn setApplicationName_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication18setApplicationNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:109
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString applicationName()

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn applicationName_0<RetType, T: QCoreApplication_applicationName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationName_0();
    // return 1;
  }
}
pub trait QCoreApplication_applicationName_0<RetType> {
  fn applicationName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_applicationName_0<usize> for () {
  fn applicationName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication15applicationNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:110
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setApplicationVersion(const QString &)

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn setApplicationVersion_0<RetType, T: QCoreApplication_setApplicationVersion_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setApplicationVersion_0();
    // return 1;
  }
}
pub trait QCoreApplication_setApplicationVersion_0<RetType> {
  fn setApplicationVersion_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_setApplicationVersion_0<(/*void*/)> for (usize) {
  fn setApplicationVersion_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication21setApplicationVersionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:111
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString applicationVersion()

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn applicationVersion_0<RetType, T: QCoreApplication_applicationVersion_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationVersion_0();
    // return 1;
  }
}
pub trait QCoreApplication_applicationVersion_0<RetType> {
  fn applicationVersion_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_applicationVersion_0<usize> for () {
  fn applicationVersion_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication18applicationVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:113
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setSetuidAllowed(bool)

/*
Allows the application to run setuid on UNIX platforms if allow is true.

If allow is false (the default) and Qt detects the application is running with an effective user id different than the real user id, the application will be aborted when a QCoreApplication instance is created.

Qt is not an appropriate solution for setuid programs due to its large attack surface. However some applications may be required to run in this manner for historical reasons. This flag will prevent Qt from aborting the application when this is detected, and must be set before a QCoreApplication instance is created.

Note: It is strongly recommended not to enable this option since it introduces security risks.

This function was introduced in  Qt 5.3.

See also isSetuidAllowed().
*/
impl /*struct*/ QCoreApplication {
  pub fn setSetuidAllowed_0<RetType, T: QCoreApplication_setSetuidAllowed_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setSetuidAllowed_0();
    // return 1;
  }
}
pub trait QCoreApplication_setSetuidAllowed_0<RetType> {
  fn setSetuidAllowed_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_setSetuidAllowed_0<(/*void*/)> for (bool) {
  fn setSetuidAllowed_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication16setSetuidAllowedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:114
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isSetuidAllowed()

/*
Returns true if the application is allowed to run setuid on UNIX platforms.

This function was introduced in  Qt 5.3.

See also QCoreApplication::setSetuidAllowed().
*/
impl /*struct*/ QCoreApplication {
  pub fn isSetuidAllowed_0<RetType, T: QCoreApplication_isSetuidAllowed_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isSetuidAllowed_0();
    // return 1;
  }
}
pub trait QCoreApplication_isSetuidAllowed_0<RetType> {
  fn isSetuidAllowed_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_isSetuidAllowed_0<bool> for () {
  fn isSetuidAllowed_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication15isSetuidAllowedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:116
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QCoreApplication * instance()

/*
Returns a pointer to the application's QCoreApplication (or QGuiApplication/QApplication) instance.

If no instance has been allocated, null is returned.
*/
impl /*struct*/ QCoreApplication {
  pub fn instance_0<RetType, T: QCoreApplication_instance_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.instance_0();
    // return 1;
  }
}
pub trait QCoreApplication_instance_0<RetType> {
  fn instance_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_instance_0<usize> for () {
  fn instance_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication8instanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:119
// index:0
// Public static Visibility=Default Availability=Available
// [4] int exec()

/*
Enters the main event loop and waits until exit() is called. Returns the value that was passed to exit() (which is 0 if exit() is called via quit()).

It is necessary to call this function to start event handling. The main event loop receives events from the window system and dispatches these to the application widgets.

To make your application perform idle processing (by executing a special function whenever there are no pending events), use a QTimer with 0 timeout. More advanced idle processing schemes can be achieved using processEvents().

We recommend that you connect clean-up code to the aboutToQuit() signal, instead of putting it in your application's main() function because on some platforms the exec() call may not return. For example, on Windows when the user logs off, the system terminates the process after Qt closes all top-level windows. Hence, there is no guarantee that the application will have time to exit its event loop and execute code at the end of the main() function after the exec() call.

See also quit(), exit(), processEvents(), and QApplication::exec().
*/
impl /*struct*/ QCoreApplication {
  pub fn exec_0<RetType, T: QCoreApplication_exec_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.exec_0();
    // return 1;
  }
}
pub trait QCoreApplication_exec_0<RetType> {
  fn exec_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_exec_0<i32> for () {
  fn exec_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication4execEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:120
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void processEvents(QEventLoop::ProcessEventsFlags)

/*
Processes all pending events for the calling thread according to the specified flags until there are no more events to process.

You can call this function occasionally when your program is busy performing a long operation (e.g. copying a file).

In the event that you are running a local loop which calls this function continuously, without an event loop, the DeferredDelete events will not be processed. This can affect the behaviour of widgets, e.g. QToolTip, that rely on DeferredDelete events to function properly. An alternative would be to call sendPostedEvents() from within that local loop.

Calling this function processes events only for the calling thread.

Note: This function is thread-safe.

See also exec(), QTimer, QEventLoop::processEvents(), flush(), and sendPostedEvents().
*/
impl /*struct*/ QCoreApplication {
  pub fn processEvents_0<RetType, T: QCoreApplication_processEvents_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.processEvents_0();
    // return 1;
  }
}
pub trait QCoreApplication_processEvents_0<RetType> {
  fn processEvents_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_processEvents_0<(/*void*/)> for (i32) {
  fn processEvents_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication13processEventsE6QFlagsIN10QEventLoop17ProcessEventsFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:121
// index:1
// Public static Visibility=Default Availability=Available
// [-2] void processEvents(QEventLoop::ProcessEventsFlags, int)

/*
Processes all pending events for the calling thread according to the specified flags until there are no more events to process.

You can call this function occasionally when your program is busy performing a long operation (e.g. copying a file).

In the event that you are running a local loop which calls this function continuously, without an event loop, the DeferredDelete events will not be processed. This can affect the behaviour of widgets, e.g. QToolTip, that rely on DeferredDelete events to function properly. An alternative would be to call sendPostedEvents() from within that local loop.

Calling this function processes events only for the calling thread.

Note: This function is thread-safe.

See also exec(), QTimer, QEventLoop::processEvents(), flush(), and sendPostedEvents().
*/
impl /*struct*/ QCoreApplication {
  pub fn processEvents_1<RetType, T: QCoreApplication_processEvents_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.processEvents_1();
    // return 1;
  }
}
pub trait QCoreApplication_processEvents_1<RetType> {
  fn processEvents_1(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_processEvents_1<(/*void*/)> for (i32,i32) {
  fn processEvents_1(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication13processEventsE6QFlagsIN10QEventLoop17ProcessEventsFlagEEi", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:122
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void exit(int)

/*
Tells the application to exit with a return code.

After this function has been called, the application leaves the main event loop and returns from the call to exec(). The exec() function returns returnCode. If the event loop is not running, this function does nothing.

By convention, a returnCode of 0 means success, and any non-zero value indicates an error.

Note that unlike the C library function of the same name, this function does return to the caller -- it is event processing that stops.

See also quit() and exec().
*/
impl /*struct*/ QCoreApplication {
  pub fn exit_0<RetType, T: QCoreApplication_exit_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.exit_0();
    // return 1;
  }
}
pub trait QCoreApplication_exit_0<RetType> {
  fn exit_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_exit_0<(/*void*/)> for (i32) {
  fn exit_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication4exitEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:124
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool sendEvent(QObject *, QEvent *)

/*
Sends event event directly to receiver receiver, using the notify() function. Returns the value that was returned from the event handler.

The event is not deleted when the event has been sent. The normal approach is to create the event on the stack, for example:


  QMouseEvent event(QEvent::MouseButtonPress, pos, 0, 0, 0);
  QApplication::sendEvent(mainWindow, &event);



See also postEvent() and notify().
*/
impl /*struct*/ QCoreApplication {
  pub fn sendEvent_0<RetType, T: QCoreApplication_sendEvent_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.sendEvent_0();
    // return 1;
  }
}
pub trait QCoreApplication_sendEvent_0<RetType> {
  fn sendEvent_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_sendEvent_0<bool> for (usize,usize) {
  fn sendEvent_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication9sendEventEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:125
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void postEvent(QObject *, QEvent *, int)

/*
Adds the event event, with the object receiver as the receiver of the event, to an event queue and returns immediately.

The event must be allocated on the heap since the post event queue will take ownership of the event and delete it once it has been posted. It is not safe to access the event after it has been posted.

When control returns to the main event loop, all events that are stored in the queue will be sent using the notify() function.

Events are sorted in descending priority order, i.e. events with a high priority are queued before events with a lower priority. The priority can be any integer value, i.e. between INT_MAX and INT_MIN, inclusive; see Qt::EventPriority for more details. Events with equal priority will be processed in the order posted.

Note: This function is thread-safe.

This function was introduced in  Qt 4.3.

See also sendEvent(), notify(), sendPostedEvents(), and Qt::EventPriority.
*/
impl /*struct*/ QCoreApplication {
  pub fn postEvent_0<RetType, T: QCoreApplication_postEvent_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.postEvent_0();
    // return 1;
  }
}
pub trait QCoreApplication_postEvent_0<RetType> {
  fn postEvent_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_postEvent_0<(/*void*/)> for (usize,usize,i32) {
  fn postEvent_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication9postEventEP7QObjectP6QEventi", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:126
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void sendPostedEvents(QObject *, int)

/*
Immediately dispatches all events which have been previously queued with QCoreApplication::postEvent() and which are for the object receiver and have the event type event_type.

Events from the window system are not dispatched by this function, but by processEvents().

If receiver is null, the events of event_type are sent for all objects. If event_type is 0, all the events are sent for receiver.

Note: This method must be called from the thread in which its QObject parameter, receiver, lives.

See also flush() and postEvent().
*/
impl /*struct*/ QCoreApplication {
  pub fn sendPostedEvents_0<RetType, T: QCoreApplication_sendPostedEvents_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.sendPostedEvents_0();
    // return 1;
  }
}
pub trait QCoreApplication_sendPostedEvents_0<RetType> {
  fn sendPostedEvents_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_sendPostedEvents_0<(/*void*/)> for (usize,i32) {
  fn sendPostedEvents_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication16sendPostedEventsEP7QObjecti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:127
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void removePostedEvents(QObject *, int)

/*
Removes all events of the given eventType that were posted using postEvent() for receiver.

The events are not dispatched, instead they are removed from the queue. You should never need to call this function. If you do call it, be aware that killing events may cause receiver to break one or more invariants.

If receiver is null, the events of eventType are removed for all objects. If eventType is 0, all the events are removed for receiver. You should never call this function with eventType of 0. If you do call it in this way, be aware that killing events may cause receiver to break one or more invariants.

Note: This function is thread-safe.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QCoreApplication {
  pub fn removePostedEvents_0<RetType, T: QCoreApplication_removePostedEvents_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.removePostedEvents_0();
    // return 1;
  }
}
pub trait QCoreApplication_removePostedEvents_0<RetType> {
  fn removePostedEvents_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_removePostedEvents_0<(/*void*/)> for (usize,i32) {
  fn removePostedEvents_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication18removePostedEventsEP7QObjecti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:129
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool hasPendingEvents()

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn hasPendingEvents_0<RetType, T: QCoreApplication_hasPendingEvents_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasPendingEvents_0();
    // return 1;
  }
}
pub trait QCoreApplication_hasPendingEvents_0<RetType> {
  fn hasPendingEvents_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_hasPendingEvents_0<bool> for () {
  fn hasPendingEvents_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication16hasPendingEventsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:131
// index:0
// Public static Visibility=Default Availability=Available
// [8] QAbstractEventDispatcher * eventDispatcher()

/*
Returns a pointer to the event dispatcher object for the main thread. If no event dispatcher exists for the thread, this function returns 0.

See also setEventDispatcher().
*/
impl /*struct*/ QCoreApplication {
  pub fn eventDispatcher_0<RetType, T: QCoreApplication_eventDispatcher_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.eventDispatcher_0();
    // return 1;
  }
}
pub trait QCoreApplication_eventDispatcher_0<RetType> {
  fn eventDispatcher_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_eventDispatcher_0<usize> for () {
  fn eventDispatcher_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication15eventDispatcherEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:132
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setEventDispatcher(QAbstractEventDispatcher *)

/*
Sets the event dispatcher for the main thread to eventDispatcher. This is only possible as long as there is no event dispatcher installed yet. That is, before QCoreApplication has been instantiated. This method takes ownership of the object.

See also eventDispatcher().
*/
impl /*struct*/ QCoreApplication {
  pub fn setEventDispatcher_0<RetType, T: QCoreApplication_setEventDispatcher_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setEventDispatcher_0();
    // return 1;
  }
}
pub trait QCoreApplication_setEventDispatcher_0<RetType> {
  fn setEventDispatcher_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_setEventDispatcher_0<(/*void*/)> for (usize) {
  fn setEventDispatcher_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication18setEventDispatcherEP24QAbstractEventDispatcher", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:134
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool notify(QObject *, QEvent *)

/*
Sends event to receiver: receiver->event(event). Returns the value that is returned from the receiver's event handler. Note that this function is called for all events sent to any object in any thread.

For certain types of events (e.g. mouse and key events), the event will be propagated to the receiver's parent and so on up to the top-level object if the receiver is not interested in the event (i.e., it returns false).

There are five different ways that events can be processed; reimplementing this virtual function is just one of them. All five approaches are listed below:
*/
impl /*struct*/ QCoreApplication {
  pub fn notify_0<RetType, T: QCoreApplication_notify_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notify_0(self);
    // return 1;
  }
}
pub trait QCoreApplication_notify_0<RetType> {
  fn notify_0(self , rsthis: & QCoreApplication) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_notify_0<bool> for (usize,usize) {
  fn notify_0(self , rsthis: & QCoreApplication) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication6notifyEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:136
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool startingUp()

/*
Returns true if an application object has not been created yet; otherwise returns false.

See also closingDown().
*/
impl /*struct*/ QCoreApplication {
  pub fn startingUp_0<RetType, T: QCoreApplication_startingUp_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.startingUp_0();
    // return 1;
  }
}
pub trait QCoreApplication_startingUp_0<RetType> {
  fn startingUp_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_startingUp_0<bool> for () {
  fn startingUp_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication10startingUpEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:137
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool closingDown()

/*
Returns true if the application objects are being destroyed; otherwise returns false.

See also startingUp().
*/
impl /*struct*/ QCoreApplication {
  pub fn closingDown_0<RetType, T: QCoreApplication_closingDown_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.closingDown_0();
    // return 1;
  }
}
pub trait QCoreApplication_closingDown_0<RetType> {
  fn closingDown_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_closingDown_0<bool> for () {
  fn closingDown_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication11closingDownEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:140
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString applicationDirPath()

/*
Returns the directory that contains the application executable.

For example, if you have installed Qt in the C:\Qt directory, and you run the regexp example, this function will return "C:/Qt/examples/tools/regexp".

On macOS and iOS this will point to the directory actually containing the executable, which may be inside an application bundle (if the application is bundled).

Warning: On Linux, this function will try to get the path from the /proc file system. If that fails, it assumes that argv[0] contains the absolute file name of the executable. The function also assumes that the current directory has not been changed by the application.

See also applicationFilePath().
*/
impl /*struct*/ QCoreApplication {
  pub fn applicationDirPath_0<RetType, T: QCoreApplication_applicationDirPath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationDirPath_0();
    // return 1;
  }
}
pub trait QCoreApplication_applicationDirPath_0<RetType> {
  fn applicationDirPath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_applicationDirPath_0<usize> for () {
  fn applicationDirPath_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication18applicationDirPathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:141
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString applicationFilePath()

/*
Returns the file path of the application executable.

For example, if you have installed Qt in the /usr/local/qt directory, and you run the regexp example, this function will return "/usr/local/qt/examples/tools/regexp/regexp".

Warning: On Linux, this function will try to get the path from the /proc file system. If that fails, it assumes that argv[0] contains the absolute file name of the executable. The function also assumes that the current directory has not been changed by the application.

See also applicationDirPath().
*/
impl /*struct*/ QCoreApplication {
  pub fn applicationFilePath_0<RetType, T: QCoreApplication_applicationFilePath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationFilePath_0();
    // return 1;
  }
}
pub trait QCoreApplication_applicationFilePath_0<RetType> {
  fn applicationFilePath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_applicationFilePath_0<usize> for () {
  fn applicationFilePath_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication19applicationFilePathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:142
// index:0
// Public static Visibility=Default Availability=Available
// [8] qint64 applicationPid()

/*
Returns the current process ID for the application.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QCoreApplication {
  pub fn applicationPid_0<RetType, T: QCoreApplication_applicationPid_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationPid_0();
    // return 1;
  }
}
pub trait QCoreApplication_applicationPid_0<RetType> {
  fn applicationPid_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_applicationPid_0<i64> for () {
  fn applicationPid_0(self ) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication14applicationPidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:145
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setLibraryPaths(const QStringList &)

/*
Sets the list of directories to search when loading libraries to paths. All existing paths will be deleted and the path list will consist of the paths given in paths.

The library paths are reset to the default when an instance of QCoreApplication is destructed.

See also libraryPaths(), addLibraryPath(), removeLibraryPath(), and QLibrary.
*/
impl /*struct*/ QCoreApplication {
  pub fn setLibraryPaths_0<RetType, T: QCoreApplication_setLibraryPaths_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setLibraryPaths_0();
    // return 1;
  }
}
pub trait QCoreApplication_setLibraryPaths_0<RetType> {
  fn setLibraryPaths_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_setLibraryPaths_0<(/*void*/)> for (usize) {
  fn setLibraryPaths_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication15setLibraryPathsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:146
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList libraryPaths()

/*
Returns a list of paths that the application will search when dynamically loading libraries.

The return value of this function may change when a QCoreApplication is created. It is not recommended to call it before creating a QCoreApplication. The directory of the application executable (not the working directory) is part of the list if it is known. In order to make it known a QCoreApplication has to be constructed as it will use argv[0] to find it.

Qt provides default library paths, but they can also be set using a qt.conf file. Paths specified in this file will override default values. Note that if the qt.conf file is in the directory of the application executable, it may not be found until a QCoreApplication is created. If it is not found when calling this function, the default library paths will be used.

The list will include the installation directory for plugins if it exists (the default installation directory for plugins is INSTALL/plugins, where INSTALL is the directory where Qt was installed). The colon separated entries of the QT_PLUGIN_PATH environment variable are always added. The plugin installation directory (and its existence) may change when the directory of the application executable becomes known.

If you want to iterate over the list, you can use the foreach pseudo-keyword:


  foreach (const QString &path, app.libraryPaths())
      do_something(path);



See also setLibraryPaths(), addLibraryPath(), removeLibraryPath(), QLibrary, and How to Create Qt Plugins.
*/
impl /*struct*/ QCoreApplication {
  pub fn libraryPaths_0<RetType, T: QCoreApplication_libraryPaths_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.libraryPaths_0();
    // return 1;
  }
}
pub trait QCoreApplication_libraryPaths_0<RetType> {
  fn libraryPaths_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_libraryPaths_0<usize> for () {
  fn libraryPaths_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication12libraryPathsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:147
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void addLibraryPath(const QString &)

/*
Prepends path to the beginning of the library path list, ensuring that it is searched for libraries first. If path is empty or already in the path list, the path list is not changed.

The default path list consists of a single entry, the installation directory for plugins. The default installation directory for plugins is INSTALL/plugins, where INSTALL is the directory where Qt was installed.

The library paths are reset to the default when an instance of QCoreApplication is destructed.

See also removeLibraryPath(), libraryPaths(), and setLibraryPaths().
*/
impl /*struct*/ QCoreApplication {
  pub fn addLibraryPath_0<RetType, T: QCoreApplication_addLibraryPath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.addLibraryPath_0();
    // return 1;
  }
}
pub trait QCoreApplication_addLibraryPath_0<RetType> {
  fn addLibraryPath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_addLibraryPath_0<(/*void*/)> for (usize) {
  fn addLibraryPath_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication14addLibraryPathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:148
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void removeLibraryPath(const QString &)

/*
Removes path from the library path list. If path is empty or not in the path list, the list is not changed.

The library paths are reset to the default when an instance of QCoreApplication is destructed.

See also addLibraryPath(), libraryPaths(), and setLibraryPaths().
*/
impl /*struct*/ QCoreApplication {
  pub fn removeLibraryPath_0<RetType, T: QCoreApplication_removeLibraryPath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.removeLibraryPath_0();
    // return 1;
  }
}
pub trait QCoreApplication_removeLibraryPath_0<RetType> {
  fn removeLibraryPath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_removeLibraryPath_0<(/*void*/)> for (usize) {
  fn removeLibraryPath_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication17removeLibraryPathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:152
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool installTranslator(QTranslator *)

/*
Adds the translation file translationFile to the list of translation files to be used for translations.

Multiple translation files can be installed. Translations are searched for in the reverse order in which they were installed, so the most recently installed translation file is searched first and the first translation file installed is searched last. The search stops as soon as a translation containing a matching string is found.

Installing or removing a QTranslator, or changing an installed QTranslator generates a LanguageChange event for the QCoreApplication instance. A QApplication instance will propagate the event to all toplevel widgets, where a reimplementation of changeEvent can re-translate the user interface by passing user-visible strings via the tr() function to the respective property setters. User-interface classes generated by Qt Designer provide a retranslateUi() function that can be called.

The function returns true on success and false on failure.

See also removeTranslator(), translate(), QTranslator::load(), and Dynamic Translation.
*/
impl /*struct*/ QCoreApplication {
  pub fn installTranslator_0<RetType, T: QCoreApplication_installTranslator_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.installTranslator_0();
    // return 1;
  }
}
pub trait QCoreApplication_installTranslator_0<RetType> {
  fn installTranslator_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_installTranslator_0<bool> for (usize) {
  fn installTranslator_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication17installTranslatorEP11QTranslator", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:153
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool removeTranslator(QTranslator *)

/*
Removes the translation file translationFile from the list of translation files used by this application. (It does not delete the translation file from the file system.)

The function returns true on success and false on failure.

See also installTranslator(), translate(), and QObject::tr().
*/
impl /*struct*/ QCoreApplication {
  pub fn removeTranslator_0<RetType, T: QCoreApplication_removeTranslator_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.removeTranslator_0();
    // return 1;
  }
}
pub trait QCoreApplication_removeTranslator_0<RetType> {
  fn removeTranslator_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_removeTranslator_0<bool> for (usize) {
  fn removeTranslator_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication16removeTranslatorEP11QTranslator", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:156
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString translate(const char *, const char *, const char *, int)

/*
Returns the translation text for sourceText, by querying the installed translation files. The translation files are searched from the most recently installed file back to the first installed file.

QObject::tr() provides this functionality more conveniently.

context is typically a class name (e.g., "MyDialog") and sourceText is either English text or a short identifying text.

disambiguation is an identifying string, for when the same sourceText is used in different roles within the same context. By default, it is null.

See the QTranslator and QObject::tr() documentation for more information about contexts, disambiguations and comments.

n is used in conjunction with %n to support plural forms. See QObject::tr() for details.

If none of the translation files contain a translation for sourceText in context, this function returns a QString equivalent of sourceText.

This function is not virtual. You can use alternative translation techniques by subclassing QTranslator.

Note: This function is thread-safe.

See also QObject::tr(), installTranslator(), removeTranslator(), and translate().
*/
impl /*struct*/ QCoreApplication {
  pub fn translate_0<RetType, T: QCoreApplication_translate_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.translate_0();
    // return 1;
  }
}
pub trait QCoreApplication_translate_0<RetType> {
  fn translate_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_translate_0<usize> for (usize,usize,usize,i32) {
  fn translate_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication9translateEPKcS1_S1_i", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:169
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void flush()

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn flush_0<RetType, T: QCoreApplication_flush_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.flush_0();
    // return 1;
  }
}
pub trait QCoreApplication_flush_0<RetType> {
  fn flush_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_flush_0<(/*void*/)> for () {
  fn flush_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication5flushEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:172
// index:0
// Public Visibility=Default Availability=Available
// [-2] void installNativeEventFilter(QAbstractNativeEventFilter *)

/*
Installs an event filter filterObj for all native events received by the application in the main thread.

The event filter filterObj receives events via its nativeEventFilter() function, which is called for all native events received in the main thread.

The QAbstractNativeEventFilter::nativeEventFilter() function should return true if the event should be filtered, i.e. stopped. It should return false to allow normal Qt processing to continue: the native event can then be translated into a QEvent and handled by the standard Qt event filtering, e.g. QObject::installEventFilter().

If multiple event filters are installed, the filter that was installed last is activated first.

Note: The filter function set here receives native messages, i.e. MSG or XCB event structs.

Note: Native event filters will be disabled in the application when the Qt::AA_PluginApplication attribute is set.

For maximum portability, you should always try to use QEvent and QObject::installEventFilter() whenever possible.

This function was introduced in  Qt 5.0.

See also QObject::installEventFilter().
*/
impl /*struct*/ QCoreApplication {
  pub fn installNativeEventFilter_0<RetType, T: QCoreApplication_installNativeEventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.installNativeEventFilter_0(self);
    // return 1;
  }
}
pub trait QCoreApplication_installNativeEventFilter_0<RetType> {
  fn installNativeEventFilter_0(self , rsthis: & QCoreApplication) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_installNativeEventFilter_0<(/*void*/)> for (usize) {
  fn installNativeEventFilter_0(self , rsthis: & QCoreApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication24installNativeEventFilterEP26QAbstractNativeEventFilter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:173
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeNativeEventFilter(QAbstractNativeEventFilter *)

/*
Removes an event filterObject from this object. The request is ignored if such an event filter has not been installed.

All event filters for this object are automatically removed when this object is destroyed.

It is always safe to remove an event filter, even during event filter activation (i.e. from the nativeEventFilter() function).

This function was introduced in  Qt 5.0.

See also installNativeEventFilter().
*/
impl /*struct*/ QCoreApplication {
  pub fn removeNativeEventFilter_0<RetType, T: QCoreApplication_removeNativeEventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeNativeEventFilter_0(self);
    // return 1;
  }
}
pub trait QCoreApplication_removeNativeEventFilter_0<RetType> {
  fn removeNativeEventFilter_0(self , rsthis: & QCoreApplication) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_removeNativeEventFilter_0<(/*void*/)> for (usize) {
  fn removeNativeEventFilter_0(self , rsthis: & QCoreApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication23removeNativeEventFilterEP26QAbstractNativeEventFilter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:175
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isQuitLockEnabled()

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn isQuitLockEnabled_0<RetType, T: QCoreApplication_isQuitLockEnabled_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isQuitLockEnabled_0();
    // return 1;
  }
}
pub trait QCoreApplication_isQuitLockEnabled_0<RetType> {
  fn isQuitLockEnabled_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_isQuitLockEnabled_0<bool> for () {
  fn isQuitLockEnabled_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication17isQuitLockEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:176
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setQuitLockEnabled(bool)

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn setQuitLockEnabled_0<RetType, T: QCoreApplication_setQuitLockEnabled_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setQuitLockEnabled_0();
    // return 1;
  }
}
pub trait QCoreApplication_setQuitLockEnabled_0<RetType> {
  fn setQuitLockEnabled_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_setQuitLockEnabled_0<(/*void*/)> for (bool) {
  fn setQuitLockEnabled_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication18setQuitLockEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:179
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void quit()

/*
Tells the application to exit with return code 0 (success). Equivalent to calling QCoreApplication::exit(0).

It's common to connect the QGuiApplication::lastWindowClosed() signal to quit(), and you also often connect e.g. QAbstractButton::clicked() or signals in QAction, QMenu, or QMenuBar to it.

Example:


  QPushButton *quitButton = new QPushButton("Quit");
  connect(quitButton, SIGNAL(clicked()), &app, SLOT(quit()));



See also exit(), aboutToQuit(), and QGuiApplication::lastWindowClosed().
*/
impl /*struct*/ QCoreApplication {
  pub fn quit_0<RetType, T: QCoreApplication_quit_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.quit_0();
    // return 1;
  }
}
pub trait QCoreApplication_quit_0<RetType> {
  fn quit_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_quit_0<(/*void*/)> for () {
  fn quit_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication4quitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:184
// index:0
// Public Visibility=Default Availability=Available
// [-2] void organizationNameChanged()

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn organizationNameChanged_0<RetType, T: QCoreApplication_organizationNameChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.organizationNameChanged_0(self);
    // return 1;
  }
}
pub trait QCoreApplication_organizationNameChanged_0<RetType> {
  fn organizationNameChanged_0(self , rsthis: & QCoreApplication) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_organizationNameChanged_0<(/*void*/)> for () {
  fn organizationNameChanged_0(self , rsthis: & QCoreApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication23organizationNameChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:185
// index:0
// Public Visibility=Default Availability=Available
// [-2] void organizationDomainChanged()

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn organizationDomainChanged_0<RetType, T: QCoreApplication_organizationDomainChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.organizationDomainChanged_0(self);
    // return 1;
  }
}
pub trait QCoreApplication_organizationDomainChanged_0<RetType> {
  fn organizationDomainChanged_0(self , rsthis: & QCoreApplication) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_organizationDomainChanged_0<(/*void*/)> for () {
  fn organizationDomainChanged_0(self , rsthis: & QCoreApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication25organizationDomainChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:186
// index:0
// Public Visibility=Default Availability=Available
// [-2] void applicationNameChanged()

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn applicationNameChanged_0<RetType, T: QCoreApplication_applicationNameChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.applicationNameChanged_0(self);
    // return 1;
  }
}
pub trait QCoreApplication_applicationNameChanged_0<RetType> {
  fn applicationNameChanged_0(self , rsthis: & QCoreApplication) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_applicationNameChanged_0<(/*void*/)> for () {
  fn applicationNameChanged_0(self , rsthis: & QCoreApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication22applicationNameChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void applicationVersionChanged()

/*

*/
impl /*struct*/ QCoreApplication {
  pub fn applicationVersionChanged_0<RetType, T: QCoreApplication_applicationVersionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.applicationVersionChanged_0(self);
    // return 1;
  }
}
pub trait QCoreApplication_applicationVersionChanged_0<RetType> {
  fn applicationVersionChanged_0(self , rsthis: & QCoreApplication) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_applicationVersionChanged_0<(/*void*/)> for () {
  fn applicationVersionChanged_0(self , rsthis: & QCoreApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QCoreApplication25applicationVersionChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreapplication.h:190
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QCoreApplication {
  pub fn event_0<RetType, T: QCoreApplication_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QCoreApplication_event_0<RetType> {
  fn event_0(self , rsthis: & QCoreApplication) -> RetType;
}
impl<'a> /*trait*/ QCoreApplication_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QCoreApplication) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCoreApplication5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


*/
pub type QCoreApplication__ = i32;
// 
pub const QCoreApplication__ApplicationFlags :QCoreApplication__ = 330241;
pub fn QCoreApplication_ItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QCoreApplication", val);
}
pub fn QCoreApplication_ItemName_s(val: i32) ->String {
  //var nilthis *QCoreApplication
  //return nilthis.ItemName(val);
  return QCoreApplication_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
