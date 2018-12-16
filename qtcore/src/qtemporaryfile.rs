

// mod ::core::QTemporaryFile
// package qtcore
// /usr/include/qt/QtCore/qtemporaryfile.h
// #include <qtemporaryfile.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 10
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

// bool open(QIODevice::OpenMode)
// func (this *QTemporaryFile) InheritOpen(f func(flags int) bool) {
//  qtrt.SetAllInheritCallback(this, "open", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTemporaryFile)=16
pub struct QTemporaryFile {
  qbase: QFile,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTemporaryFile_ITF interface {
//    QFile_ITF
//    QTemporaryFile_PTR() *QTemporaryFile
//}
//func (ptr *QTemporaryFile) QTemporaryFile_PTR() *QTemporaryFile { return ptr }

impl /*struct*/ QTemporaryFile {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTemporaryFile {
    return QTemporaryFile{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTemporaryFile {
//  type Target = QTemporaryFileBASE;
//
//  fn deref(&self) -> &QTemporaryFileBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTemporaryFileBASE> for QTemporaryFile {
//  fn as_ref(& self) -> & QTemporaryFileBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qtemporaryfile.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTemporaryFile {
  pub fn metaObject_0<RetType, T: QTemporaryFile_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTemporaryFile_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTemporaryFile) -> RetType;
}
impl<'a> /*trait*/ QTemporaryFile_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTemporaryFile) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTemporaryFile10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTemporaryFile()

/*
Constructs a QTemporaryFile using as file template the application name returned by QCoreApplication::applicationName() (otherwise qt_temp) followed by ".XXXXXX". The file is stored in the system's temporary directory, QDir::tempPath().

See also setFileTemplate() and QDir::tempPath().
*/
// QTemporaryFile() ctx.fn_proto_cpp
impl /*struct*/ QTemporaryFile {
  pub fn QTemporaryFile_0<T: QTemporaryFile_QTemporaryFile_0>(value: T) -> QTemporaryFile {
    let rsthis = value.QTemporaryFile_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTemporaryFile_QTemporaryFile_0 {
  fn QTemporaryFile_0(self) -> QTemporaryFile;
}
// QTemporaryFile() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTemporaryFile_QTemporaryFile_0 for () {
  fn QTemporaryFile_0(self) -> QTemporaryFile {
    // unsafe{_ZN14QTemporaryFileC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QTemporaryFileC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTemporaryFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:67
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTemporaryFile(const QString &)

/*
Constructs a QTemporaryFile using as file template the application name returned by QCoreApplication::applicationName() (otherwise qt_temp) followed by ".XXXXXX". The file is stored in the system's temporary directory, QDir::tempPath().

See also setFileTemplate() and QDir::tempPath().
*/
// QTemporaryFile(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QTemporaryFile {
  pub fn QTemporaryFile_1<T: QTemporaryFile_QTemporaryFile_1>(value: T) -> QTemporaryFile {
    let rsthis = value.QTemporaryFile_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTemporaryFile_QTemporaryFile_1 {
  fn QTemporaryFile_1(self) -> QTemporaryFile;
}
// QTemporaryFile(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTemporaryFile_QTemporaryFile_1 for (usize) {
  fn QTemporaryFile_1(self) -> QTemporaryFile {
    // unsafe{_ZN14QTemporaryFileC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QTemporaryFileC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTemporaryFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:69
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QTemporaryFile(QObject *)

/*
Constructs a QTemporaryFile using as file template the application name returned by QCoreApplication::applicationName() (otherwise qt_temp) followed by ".XXXXXX". The file is stored in the system's temporary directory, QDir::tempPath().

See also setFileTemplate() and QDir::tempPath().
*/
// QTemporaryFile(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QTemporaryFile {
  pub fn QTemporaryFile_2<T: QTemporaryFile_QTemporaryFile_2>(value: T) -> QTemporaryFile {
    let rsthis = value.QTemporaryFile_2();
    return rsthis;
    // return 1;
  }
}

pub trait QTemporaryFile_QTemporaryFile_2 {
  fn QTemporaryFile_2(self) -> QTemporaryFile;
}
// QTemporaryFile(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTemporaryFile_QTemporaryFile_2 for (usize) {
  fn QTemporaryFile_2(self) -> QTemporaryFile {
    // unsafe{_ZN14QTemporaryFileC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QTemporaryFileC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTemporaryFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:70
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QTemporaryFile(const QString &, QObject *)

/*
Constructs a QTemporaryFile using as file template the application name returned by QCoreApplication::applicationName() (otherwise qt_temp) followed by ".XXXXXX". The file is stored in the system's temporary directory, QDir::tempPath().

See also setFileTemplate() and QDir::tempPath().
*/
// QTemporaryFile(const QString &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QTemporaryFile {
  pub fn QTemporaryFile_3<T: QTemporaryFile_QTemporaryFile_3>(value: T) -> QTemporaryFile {
    let rsthis = value.QTemporaryFile_3();
    return rsthis;
    // return 1;
  }
}

pub trait QTemporaryFile_QTemporaryFile_3 {
  fn QTemporaryFile_3(self) -> QTemporaryFile;
}
// QTemporaryFile(const QString &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTemporaryFile_QTemporaryFile_3 for (usize,usize) {
  fn QTemporaryFile_3(self) -> QTemporaryFile {
    // unsafe{_ZN14QTemporaryFileC2ERK7QStringP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QTemporaryFileC2ERK7QStringP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTemporaryFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTemporaryFile()

/*

*/
pub fn DeleteQTemporaryFile(this :*mut QTemporaryFile) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QTemporaryFileD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qtemporaryfile.h:74
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoRemove() const

/*
Returns true if the QTemporaryFile is in auto remove mode. Auto-remove mode will automatically delete the filename from disk upon destruction. This makes it very easy to create your QTemporaryFile object on the stack, fill it with data, read from it, and finally on function return it will automatically clean up after itself.

Auto-remove is on by default.

See also setAutoRemove() and remove().
*/
impl /*struct*/ QTemporaryFile {
  pub fn autoRemove_0<RetType, T: QTemporaryFile_autoRemove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoRemove_0(self);
    // return 1;
  }
}
pub trait QTemporaryFile_autoRemove_0<RetType> {
  fn autoRemove_0(self , rsthis: & QTemporaryFile) -> RetType;
}
impl<'a> /*trait*/ QTemporaryFile_autoRemove_0<bool> for () {
  fn autoRemove_0(self , rsthis: & QTemporaryFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTemporaryFile10autoRemoveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoRemove(bool)

/*
Sets the QTemporaryFile into auto-remove mode if b is true.

Auto-remove is on by default.

If you set this property to false, ensure the application provides a way to remove the file once it is no longer needed, including passing the responsibility on to another process. Always use the fileName() function to obtain the name and never try to guess the name that QTemporaryFile has generated.

On some systems, if fileName() is not called before closing the file, the temporary file may be removed regardless of the state of this property. This behavior should not be relied upon, so application code should either call fileName() or leave the auto removal functionality enabled.

See also autoRemove() and remove().
*/
impl /*struct*/ QTemporaryFile {
  pub fn setAutoRemove_0<RetType, T: QTemporaryFile_setAutoRemove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoRemove_0(self);
    // return 1;
  }
}
pub trait QTemporaryFile_setAutoRemove_0<RetType> {
  fn setAutoRemove_0(self , rsthis: & QTemporaryFile) -> RetType;
}
impl<'a> /*trait*/ QTemporaryFile_setAutoRemove_0<(/*void*/)> for (bool) {
  fn setAutoRemove_0(self , rsthis: & QTemporaryFile) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QTemporaryFile13setAutoRemoveEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool open()

/*
A QTemporaryFile will always be opened in QIODevice::ReadWrite mode, this allows easy access to the data in the file. This function will return true upon success and will set the fileName() to the unique filename used.

See also fileName().
*/
impl /*struct*/ QTemporaryFile {
  pub fn open_0<RetType, T: QTemporaryFile_open_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_0(self);
    // return 1;
  }
}
pub trait QTemporaryFile_open_0<RetType> {
  fn open_0(self , rsthis: & QTemporaryFile) -> RetType;
}
impl<'a> /*trait*/ QTemporaryFile_open_0<bool> for () {
  fn open_0(self , rsthis: & QTemporaryFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QTemporaryFile4openEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:98
// index:1
// Protected virtual Visibility=Default Availability=Available
// [1] bool open(QIODevice::OpenMode)

/*
A QTemporaryFile will always be opened in QIODevice::ReadWrite mode, this allows easy access to the data in the file. This function will return true upon success and will set the fileName() to the unique filename used.

See also fileName().
*/
impl /*struct*/ QTemporaryFile {
  pub fn open_1<RetType, T: QTemporaryFile_open_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_1(self);
    // return 1;
  }
}
pub trait QTemporaryFile_open_1<RetType> {
  fn open_1(self , rsthis: & QTemporaryFile) -> RetType;
}
impl<'a> /*trait*/ QTemporaryFile_open_1<bool> for (i32) {
  fn open_1(self , rsthis: & QTemporaryFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QTemporaryFile4openE6QFlagsIN9QIODevice12OpenModeFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString fileName() const

/*
Reimplemented from QFileDevice::fileName().

Returns the complete unique filename backing the QTemporaryFile object. This string is null before the QTemporaryFile is opened, afterwards it will contain the fileTemplate() plus additional characters to make it unique.

See also fileTemplate().
*/
impl /*struct*/ QTemporaryFile {
  pub fn fileName_0<RetType, T: QTemporaryFile_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QTemporaryFile_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QTemporaryFile) -> RetType;
}
impl<'a> /*trait*/ QTemporaryFile_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QTemporaryFile) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTemporaryFile8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:81
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fileTemplate() const

/*
Returns the set file template. The default file template will be called qcoreappname.XXXXXX and be placed in QDir::tempPath().

See also setFileTemplate().
*/
impl /*struct*/ QTemporaryFile {
  pub fn fileTemplate_0<RetType, T: QTemporaryFile_fileTemplate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileTemplate_0(self);
    // return 1;
  }
}
pub trait QTemporaryFile_fileTemplate_0<RetType> {
  fn fileTemplate_0(self , rsthis: & QTemporaryFile) -> RetType;
}
impl<'a> /*trait*/ QTemporaryFile_fileTemplate_0<usize> for () {
  fn fileTemplate_0(self , rsthis: & QTemporaryFile) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTemporaryFile12fileTemplateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFileTemplate(const QString &)

/*
Sets the static portion of the file name to name. If the file template contains XXXXXX that will automatically be replaced with the unique part of the filename, otherwise a filename will be determined automatically based on the static portion specified.

If name contains a relative file path, the path will be relative to the current working directory. You can use QDir::tempPath() to construct name if you want use the system's temporary directory.

See also fileTemplate().
*/
impl /*struct*/ QTemporaryFile {
  pub fn setFileTemplate_0<RetType, T: QTemporaryFile_setFileTemplate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileTemplate_0(self);
    // return 1;
  }
}
pub trait QTemporaryFile_setFileTemplate_0<RetType> {
  fn setFileTemplate_0(self , rsthis: & QTemporaryFile) -> RetType;
}
impl<'a> /*trait*/ QTemporaryFile_setFileTemplate_0<(/*void*/)> for (usize) {
  fn setFileTemplate_0(self , rsthis: & QTemporaryFile) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QTemporaryFile15setFileTemplateERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:85
// index:0
// Public Visibility=Default Availability=Available
// [1] bool rename(const QString &)

/*

*/
impl /*struct*/ QTemporaryFile {
  pub fn rename_0<RetType, T: QTemporaryFile_rename_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rename_0(self);
    // return 1;
  }
}
pub trait QTemporaryFile_rename_0<RetType> {
  fn rename_0(self , rsthis: & QTemporaryFile) -> RetType;
}
impl<'a> /*trait*/ QTemporaryFile_rename_0<bool> for (usize) {
  fn rename_0(self , rsthis: & QTemporaryFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QTemporaryFile6renameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:88
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QTemporaryFile * createLocalFile(const QString &)

/*

*/
impl /*struct*/ QTemporaryFile {
  pub fn createLocalFile_0<RetType, T: QTemporaryFile_createLocalFile_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.createLocalFile_0();
    // return 1;
  }
}
pub trait QTemporaryFile_createLocalFile_0<RetType> {
  fn createLocalFile_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTemporaryFile_createLocalFile_0<usize> for (usize) {
  fn createLocalFile_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QTemporaryFile15createLocalFileERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:90
// index:1
// Public static inline Visibility=Default Availability=Available
// [8] QTemporaryFile * createLocalFile(QFile &)

/*

*/
impl /*struct*/ QTemporaryFile {
  pub fn createLocalFile_1<RetType, T: QTemporaryFile_createLocalFile_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.createLocalFile_1();
    // return 1;
  }
}
pub trait QTemporaryFile_createLocalFile_1<RetType> {
  fn createLocalFile_1(self ) -> RetType;
}
impl<'a> /*trait*/ QTemporaryFile_createLocalFile_1<usize> for (usize) {
  fn createLocalFile_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QTemporaryFile15createLocalFileER5QFile", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:93
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QTemporaryFile * createNativeFile(const QString &)

/*
If file is not already a native file, then a QTemporaryFile is created in QDir::tempPath(), the contents of file is copied into it, and a pointer to the temporary file is returned. Does nothing and returns 0 if file is already a native file.

For example:


  QFile f(":/resources/file.txt");
  QTemporaryFile::createNativeFile(f); // Returns a pointer to a temporary file

  QFile f("/users/qt/file.txt");
  QTemporaryFile::createNativeFile(f); // Returns 0



See also QFileInfo::isNativePath().
*/
impl /*struct*/ QTemporaryFile {
  pub fn createNativeFile_0<RetType, T: QTemporaryFile_createNativeFile_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.createNativeFile_0();
    // return 1;
  }
}
pub trait QTemporaryFile_createNativeFile_0<RetType> {
  fn createNativeFile_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTemporaryFile_createNativeFile_0<usize> for (usize) {
  fn createNativeFile_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QTemporaryFile16createNativeFileERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporaryfile.h:95
// index:1
// Public static Visibility=Default Availability=Available
// [8] QTemporaryFile * createNativeFile(QFile &)

/*
If file is not already a native file, then a QTemporaryFile is created in QDir::tempPath(), the contents of file is copied into it, and a pointer to the temporary file is returned. Does nothing and returns 0 if file is already a native file.

For example:


  QFile f(":/resources/file.txt");
  QTemporaryFile::createNativeFile(f); // Returns a pointer to a temporary file

  QFile f("/users/qt/file.txt");
  QTemporaryFile::createNativeFile(f); // Returns 0



See also QFileInfo::isNativePath().
*/
impl /*struct*/ QTemporaryFile {
  pub fn createNativeFile_1<RetType, T: QTemporaryFile_createNativeFile_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.createNativeFile_1();
    // return 1;
  }
}
pub trait QTemporaryFile_createNativeFile_1<RetType> {
  fn createNativeFile_1(self ) -> RetType;
}
impl<'a> /*trait*/ QTemporaryFile_createNativeFile_1<usize> for (usize) {
  fn createNativeFile_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QTemporaryFile16createNativeFileER5QFile", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
