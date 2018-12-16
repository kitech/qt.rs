

// mod ::core::QFile
// package qtcore
// /usr/include/qt/QtCore/qfile.h
// #include <qfile.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 25
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



/*

*/
#[derive(Default)] // class sizeof(QFile)=16
pub struct QFile {
  qbase: QFileDevice,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFile_ITF interface {
//    QFileDevice_ITF
//    QFile_PTR() *QFile
//}
//func (ptr *QFile) QFile_PTR() *QFile { return ptr }

impl /*struct*/ QFile {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFile {
    return QFile{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFile {
//  type Target = QFileBASE;
//
//  fn deref(&self) -> &QFileBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFileBASE> for QFile {
//  fn as_ref(& self) -> & QFileBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qfile.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QFile {
  pub fn metaObject_0<RetType, T: QFile_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QFile_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QFile) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFile10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFile()

/*
Constructs a QFile object.
*/
// QFile() ctx.fn_proto_cpp
impl /*struct*/ QFile {
  pub fn QFile_0<T: QFile_QFile_0>(value: T) -> QFile {
    let rsthis = value.QFile_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFile_QFile_0 {
  fn QFile_0(self) -> QFile;
}
// QFile() ctx.fn_proto_cpp
impl<'a> /*trait*/ QFile_QFile_0 for () {
  fn QFile_0(self) -> QFile {
    // unsafe{_ZN5QFileC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QFileC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfile.h:66
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QFile(const QString &)

/*
Constructs a QFile object.
*/
// QFile(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QFile {
  pub fn QFile_1<T: QFile_QFile_1>(value: T) -> QFile {
    let rsthis = value.QFile_1();
    return rsthis;
    // return 1;
  }
}

pub trait QFile_QFile_1 {
  fn QFile_1(self) -> QFile;
}
// QFile(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFile_QFile_1 for (usize) {
  fn QFile_1(self) -> QFile {
    // unsafe{_ZN5QFileC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QFileC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfile.h:68
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QFile(QObject *)

/*
Constructs a QFile object.
*/
// QFile(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QFile {
  pub fn QFile_2<T: QFile_QFile_2>(value: T) -> QFile {
    let rsthis = value.QFile_2();
    return rsthis;
    // return 1;
  }
}

pub trait QFile_QFile_2 {
  fn QFile_2(self) -> QFile;
}
// QFile(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFile_QFile_2 for (usize) {
  fn QFile_2(self) -> QFile {
    // unsafe{_ZN5QFileC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QFileC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfile.h:69
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QFile(const QString &, QObject *)

/*
Constructs a QFile object.
*/
// QFile(const QString &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QFile {
  pub fn QFile_3<T: QFile_QFile_3>(value: T) -> QFile {
    let rsthis = value.QFile_3();
    return rsthis;
    // return 1;
  }
}

pub trait QFile_QFile_3 {
  fn QFile_3(self) -> QFile;
}
// QFile(const QString &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFile_QFile_3 for (usize,usize) {
  fn QFile_3(self) -> QFile {
    // unsafe{_ZN5QFileC2ERK7QStringP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QFileC2ERK7QStringP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfile.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFile()

/*

*/
pub fn DeleteQFile(this :*mut QFile) {
    // let rv = qtrt::InvokeQtFunc6("_ZN5QFileD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qfile.h:73
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString fileName() const

/*
Reimplemented from QFileDevice::fileName().

Returns the name set by setFileName() or to the QFile constructors.

See also setFileName() and QFileInfo::fileName().
*/
impl /*struct*/ QFile {
  pub fn fileName_0<RetType, T: QFile_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QFile_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QFile) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFile8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFileName(const QString &)

/*
Sets the name of the file. The name can have no path, a relative path, or an absolute path.

Do not call this function if the file has already been opened.

If the file name has no path or a relative path, the path used will be the application's current directory path at the time of the open() call.

Example:


  QFile file;
  QDir::setCurrent("/tmp");
  file.setFileName("readme.txt");
  QDir::setCurrent("/home");
  file.open(QIODevice::ReadOnly);      // opens "/home/readme.txt" under Unix



Note that the directory separator "/" works for all operating systems supported by Qt.

See also fileName(), QFileInfo, and QDir.
*/
impl /*struct*/ QFile {
  pub fn setFileName_0<RetType, T: QFile_setFileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileName_0(self);
    // return 1;
  }
}
pub trait QFile_setFileName_0<RetType> {
  fn setFileName_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_setFileName_0<(/*void*/)> for (usize) {
  fn setFileName_0(self , rsthis: & QFile) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QFile11setFileNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfile.h:88
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QByteArray encodeName(const QString &)

/*
Converts fileName to the local 8-bit encoding determined by the user's locale. This is sufficient for file names that the user chooses. File names hard-coded into the application should only use 7-bit ASCII filename characters.

See also decodeName().
*/
impl /*struct*/ QFile {
  pub fn encodeName_0<RetType, T: QFile_encodeName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.encodeName_0();
    // return 1;
  }
}
pub trait QFile_encodeName_0<RetType> {
  fn encodeName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFile_encodeName_0<usize> for (usize) {
  fn encodeName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile10encodeNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:92
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QString decodeName(const QByteArray &)

/*
This does the reverse of QFile::encodeName() using localFileName.

See also encodeName().
*/
impl /*struct*/ QFile {
  pub fn decodeName_0<RetType, T: QFile_decodeName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.decodeName_0();
    // return 1;
  }
}
pub trait QFile_decodeName_0<RetType> {
  fn decodeName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFile_decodeName_0<usize> for (usize) {
  fn decodeName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile10decodeNameERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:97
// index:1
// Public static inline Visibility=Default Availability=Available
// [8] QString decodeName(const char *)

/*
This does the reverse of QFile::encodeName() using localFileName.

See also encodeName().
*/
impl /*struct*/ QFile {
  pub fn decodeName_1<RetType, T: QFile_decodeName_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.decodeName_1();
    // return 1;
  }
}
pub trait QFile_decodeName_1<RetType> {
  fn decodeName_1(self ) -> RetType;
}
impl<'a> /*trait*/ QFile_decodeName_1<usize> for (usize) {
  fn decodeName_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile10decodeNameEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:107
// index:0
// Public Visibility=Default Availability=Available
// [1] bool exists() const

/*
Returns true if the file specified by fileName exists; otherwise returns false.

Note: If fileName is a symlink that points to a non-existing file, false is returned.
*/
impl /*struct*/ QFile {
  pub fn exists_0<RetType, T: QFile_exists_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exists_0(self);
    // return 1;
  }
}
pub trait QFile_exists_0<RetType> {
  fn exists_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_exists_0<bool> for () {
  fn exists_0(self , rsthis: & QFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFile6existsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:108
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool exists(const QString &)

/*
Returns true if the file specified by fileName exists; otherwise returns false.

Note: If fileName is a symlink that points to a non-existing file, false is returned.
*/
impl /*struct*/ QFile {
  pub fn exists_1<RetType, T: QFile_exists_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.exists_1();
    // return 1;
  }
}
pub trait QFile_exists_1<RetType> {
  fn exists_1(self ) -> RetType;
}
impl<'a> /*trait*/ QFile_exists_1<bool> for (usize) {
  fn exists_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile6existsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:110
// index:0
// Public Visibility=Default Availability=Available
// [8] QString readLink() const

/*

*/
impl /*struct*/ QFile {
  pub fn readLink_0<RetType, T: QFile_readLink_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readLink_0(self);
    // return 1;
  }
}
pub trait QFile_readLink_0<RetType> {
  fn readLink_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_readLink_0<usize> for () {
  fn readLink_0(self , rsthis: & QFile) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFile8readLinkEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:111
// index:1
// Public static Visibility=Default Availability=Available
// [8] QString readLink(const QString &)

/*

*/
impl /*struct*/ QFile {
  pub fn readLink_1<RetType, T: QFile_readLink_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.readLink_1();
    // return 1;
  }
}
pub trait QFile_readLink_1<RetType> {
  fn readLink_1(self ) -> RetType;
}
impl<'a> /*trait*/ QFile_readLink_1<usize> for (usize) {
  fn readLink_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile8readLinkERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:112
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString symLinkTarget() const

/*
Returns the absolute path of the file or directory referred to by the symlink (or shortcut on Windows) specified by fileName, or returns an empty string if the fileName does not correspond to a symbolic link.

This name may not represent an existing file; it is only a string. QFile::exists() returns true if the symlink points to an existing file.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QFile {
  pub fn symLinkTarget_0<RetType, T: QFile_symLinkTarget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.symLinkTarget_0(self);
    // return 1;
  }
}
pub trait QFile_symLinkTarget_0<RetType> {
  fn symLinkTarget_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_symLinkTarget_0<usize> for () {
  fn symLinkTarget_0(self , rsthis: & QFile) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFile13symLinkTargetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:113
// index:1
// Public static inline Visibility=Default Availability=Available
// [8] QString symLinkTarget(const QString &)

/*
Returns the absolute path of the file or directory referred to by the symlink (or shortcut on Windows) specified by fileName, or returns an empty string if the fileName does not correspond to a symbolic link.

This name may not represent an existing file; it is only a string. QFile::exists() returns true if the symlink points to an existing file.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QFile {
  pub fn symLinkTarget_1<RetType, T: QFile_symLinkTarget_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.symLinkTarget_1();
    // return 1;
  }
}
pub trait QFile_symLinkTarget_1<RetType> {
  fn symLinkTarget_1(self ) -> RetType;
}
impl<'a> /*trait*/ QFile_symLinkTarget_1<usize> for (usize) {
  fn symLinkTarget_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile13symLinkTargetERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:115
// index:0
// Public Visibility=Default Availability=Available
// [1] bool remove()

/*
Removes the file specified by fileName(). Returns true if successful; otherwise returns false.

The file is closed before it is removed.

See also setFileName().
*/
impl /*struct*/ QFile {
  pub fn remove_0<RetType, T: QFile_remove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_0(self);
    // return 1;
  }
}
pub trait QFile_remove_0<RetType> {
  fn remove_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_remove_0<bool> for () {
  fn remove_0(self , rsthis: & QFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile6removeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:116
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool remove(const QString &)

/*
Removes the file specified by fileName(). Returns true if successful; otherwise returns false.

The file is closed before it is removed.

See also setFileName().
*/
impl /*struct*/ QFile {
  pub fn remove_1<RetType, T: QFile_remove_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.remove_1();
    // return 1;
  }
}
pub trait QFile_remove_1<RetType> {
  fn remove_1(self ) -> RetType;
}
impl<'a> /*trait*/ QFile_remove_1<bool> for (usize) {
  fn remove_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile6removeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:118
// index:0
// Public Visibility=Default Availability=Available
// [1] bool rename(const QString &)

/*
Renames the file currently specified by fileName() to newName. Returns true if successful; otherwise returns false.

If a file with the name newName already exists, rename() returns false (i.e., QFile will not overwrite it).

The file is closed before it is renamed.

If the rename operation fails, Qt will attempt to copy this file's contents to newName, and then remove this file, keeping only newName. If that copy operation fails or this file can't be removed, the destination file newName is removed to restore the old state.

See also setFileName().
*/
impl /*struct*/ QFile {
  pub fn rename_0<RetType, T: QFile_rename_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rename_0(self);
    // return 1;
  }
}
pub trait QFile_rename_0<RetType> {
  fn rename_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_rename_0<bool> for (usize) {
  fn rename_0(self , rsthis: & QFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile6renameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:119
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool rename(const QString &, const QString &)

/*
Renames the file currently specified by fileName() to newName. Returns true if successful; otherwise returns false.

If a file with the name newName already exists, rename() returns false (i.e., QFile will not overwrite it).

The file is closed before it is renamed.

If the rename operation fails, Qt will attempt to copy this file's contents to newName, and then remove this file, keeping only newName. If that copy operation fails or this file can't be removed, the destination file newName is removed to restore the old state.

See also setFileName().
*/
impl /*struct*/ QFile {
  pub fn rename_1<RetType, T: QFile_rename_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.rename_1();
    // return 1;
  }
}
pub trait QFile_rename_1<RetType> {
  fn rename_1(self ) -> RetType;
}
impl<'a> /*trait*/ QFile_rename_1<bool> for (usize,usize) {
  fn rename_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile6renameERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:121
// index:0
// Public Visibility=Default Availability=Available
// [1] bool link(const QString &)

/*
Creates a link named linkName that points to the file currently specified by fileName(). What a link is depends on the underlying filesystem (be it a shortcut on Windows or a symbolic link on Unix). Returns true if successful; otherwise returns false.

This function will not overwrite an already existing entity in the file system; in this case, link() will return false and set error() to return RenameError.

Note: To create a valid link on Windows, linkName must have a .lnk file extension.

See also setFileName().
*/
impl /*struct*/ QFile {
  pub fn link_0<RetType, T: QFile_link_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.link_0(self);
    // return 1;
  }
}
pub trait QFile_link_0<RetType> {
  fn link_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_link_0<bool> for (usize) {
  fn link_0(self , rsthis: & QFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile4linkERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:122
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool link(const QString &, const QString &)

/*
Creates a link named linkName that points to the file currently specified by fileName(). What a link is depends on the underlying filesystem (be it a shortcut on Windows or a symbolic link on Unix). Returns true if successful; otherwise returns false.

This function will not overwrite an already existing entity in the file system; in this case, link() will return false and set error() to return RenameError.

Note: To create a valid link on Windows, linkName must have a .lnk file extension.

See also setFileName().
*/
impl /*struct*/ QFile {
  pub fn link_1<RetType, T: QFile_link_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.link_1();
    // return 1;
  }
}
pub trait QFile_link_1<RetType> {
  fn link_1(self ) -> RetType;
}
impl<'a> /*trait*/ QFile_link_1<bool> for (usize,usize) {
  fn link_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile4linkERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:124
// index:0
// Public Visibility=Default Availability=Available
// [1] bool copy(const QString &)

/*
Copies the file currently specified by fileName() to a file called newName. Returns true if successful; otherwise returns false.

Note that if a file with the name newName already exists, copy() returns false (i.e. QFile will not overwrite it).

The source file is closed before it is copied.

See also setFileName().
*/
impl /*struct*/ QFile {
  pub fn copy_0<RetType, T: QFile_copy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.copy_0(self);
    // return 1;
  }
}
pub trait QFile_copy_0<RetType> {
  fn copy_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_copy_0<bool> for (usize) {
  fn copy_0(self , rsthis: & QFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile4copyERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:125
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool copy(const QString &, const QString &)

/*
Copies the file currently specified by fileName() to a file called newName. Returns true if successful; otherwise returns false.

Note that if a file with the name newName already exists, copy() returns false (i.e. QFile will not overwrite it).

The source file is closed before it is copied.

See also setFileName().
*/
impl /*struct*/ QFile {
  pub fn copy_1<RetType, T: QFile_copy_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.copy_1();
    // return 1;
  }
}
pub trait QFile_copy_1<RetType> {
  fn copy_1(self ) -> RetType;
}
impl<'a> /*trait*/ QFile_copy_1<bool> for (usize,usize) {
  fn copy_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile4copyERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:127
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool open(QIODevice::OpenMode)

/*
Reimplemented from QIODevice::open().

Opens the file using OpenMode mode, returning true if successful; otherwise false.

The mode must be QIODevice::ReadOnly, QIODevice::WriteOnly, or QIODevice::ReadWrite. It may also have additional flags, such as QIODevice::Text and QIODevice::Unbuffered.

Note: In WriteOnly or ReadWrite mode, if the relevant file does not already exist, this function will try to create a new file before opening it.

See also QIODevice::OpenMode and setFileName().
*/
impl /*struct*/ QFile {
  pub fn open_0<RetType, T: QFile_open_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_0(self);
    // return 1;
  }
}
pub trait QFile_open_0<RetType> {
  fn open_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_open_0<bool> for (i32) {
  fn open_0(self , rsthis: & QFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile4openE6QFlagsIN9QIODevice12OpenModeFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:129
// index:1
// Public Visibility=Default Availability=Available
// [1] bool open(int, QIODevice::OpenMode, QFileDevice::FileHandleFlags)

/*
Reimplemented from QIODevice::open().

Opens the file using OpenMode mode, returning true if successful; otherwise false.

The mode must be QIODevice::ReadOnly, QIODevice::WriteOnly, or QIODevice::ReadWrite. It may also have additional flags, such as QIODevice::Text and QIODevice::Unbuffered.

Note: In WriteOnly or ReadWrite mode, if the relevant file does not already exist, this function will try to create a new file before opening it.

See also QIODevice::OpenMode and setFileName().
*/
impl /*struct*/ QFile {
  pub fn open_1<RetType, T: QFile_open_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_1(self);
    // return 1;
  }
}
pub trait QFile_open_1<RetType> {
  fn open_1(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_open_1<bool> for (i32,i32,i32) {
  fn open_1(self , rsthis: & QFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile4openEi6QFlagsIN9QIODevice12OpenModeFlagEES0_IN11QFileDevice14FileHandleFlagEE", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:131
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] qint64 size() const

/*
Reimplemented from QIODevice::size().
*/
impl /*struct*/ QFile {
  pub fn size_0<RetType, T: QFile_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QFile_size_0<RetType> {
  fn size_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_size_0<i64> for () {
  fn size_0(self , rsthis: & QFile) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFile4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:133
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool resize(qint64)

/*
Reimplemented from QFileDevice::resize().
*/
impl /*struct*/ QFile {
  pub fn resize_0<RetType, T: QFile_resize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_0(self);
    // return 1;
  }
}
pub trait QFile_resize_0<RetType> {
  fn resize_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_resize_0<bool> for (i64) {
  fn resize_0(self , rsthis: & QFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile6resizeEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:134
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool resize(const QString &, qint64)

/*
Reimplemented from QFileDevice::resize().
*/
impl /*struct*/ QFile {
  pub fn resize_1<RetType, T: QFile_resize_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.resize_1();
    // return 1;
  }
}
pub trait QFile_resize_1<RetType> {
  fn resize_1(self ) -> RetType;
}
impl<'a> /*trait*/ QFile_resize_1<bool> for (usize,i64) {
  fn resize_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile6resizeERK7QStringx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:136
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QFileDevice::Permissions permissions() const

/*
Reimplemented from QFileDevice::permissions().

See also setPermissions().
*/
impl /*struct*/ QFile {
  pub fn permissions_0<RetType, T: QFile_permissions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.permissions_0(self);
    // return 1;
  }
}
pub trait QFile_permissions_0<RetType> {
  fn permissions_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_permissions_0<i32> for () {
  fn permissions_0(self , rsthis: & QFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFile11permissionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:137
// index:1
// Public static Visibility=Default Availability=Available
// [4] QFileDevice::Permissions permissions(const QString &)

/*
Reimplemented from QFileDevice::permissions().

See also setPermissions().
*/
impl /*struct*/ QFile {
  pub fn permissions_1<RetType, T: QFile_permissions_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.permissions_1();
    // return 1;
  }
}
pub trait QFile_permissions_1<RetType> {
  fn permissions_1(self ) -> RetType;
}
impl<'a> /*trait*/ QFile_permissions_1<i32> for (usize) {
  fn permissions_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile11permissionsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:138
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool setPermissions(QFileDevice::Permissions)

/*
Reimplemented from QFileDevice::setPermissions().

Sets the permissions for the file to the permissions specified. Returns true if successful, or false if the permissions cannot be modified.

Warning: This function does not manipulate ACLs, which may limit its effectiveness.

See also permissions() and setFileName().
*/
impl /*struct*/ QFile {
  pub fn setPermissions_0<RetType, T: QFile_setPermissions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPermissions_0(self);
    // return 1;
  }
}
pub trait QFile_setPermissions_0<RetType> {
  fn setPermissions_0(self , rsthis: & QFile) -> RetType;
}
impl<'a> /*trait*/ QFile_setPermissions_0<bool> for (i32) {
  fn setPermissions_0(self , rsthis: & QFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile14setPermissionsE6QFlagsIN11QFileDevice10PermissionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfile.h:139
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool setPermissions(const QString &, QFileDevice::Permissions)

/*
Reimplemented from QFileDevice::setPermissions().

Sets the permissions for the file to the permissions specified. Returns true if successful, or false if the permissions cannot be modified.

Warning: This function does not manipulate ACLs, which may limit its effectiveness.

See also permissions() and setFileName().
*/
impl /*struct*/ QFile {
  pub fn setPermissions_1<RetType, T: QFile_setPermissions_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPermissions_1();
    // return 1;
  }
}
pub trait QFile_setPermissions_1<RetType> {
  fn setPermissions_1(self ) -> RetType;
}
impl<'a> /*trait*/ QFile_setPermissions_1<bool> for (usize,i32) {
  fn setPermissions_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFile14setPermissionsERK7QString6QFlagsIN11QFileDevice10PermissionEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
