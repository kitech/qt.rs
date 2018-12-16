

// mod ::core::QSaveFile
// package qtcore
// /usr/include/qt/QtCore/qsavefile.h
// #include <qsavefile.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 21
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

// long long writeData(const char *, qint64)
// func (this *QSaveFile) InheritWriteData(f func(data string, len_ int64) int64) {
//  qtrt.SetAllInheritCallback(this, "writeData", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSaveFile)=16
pub struct QSaveFile {
  qbase: QFileDevice,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSaveFile_ITF interface {
//    QFileDevice_ITF
//    QSaveFile_PTR() *QSaveFile
//}
//func (ptr *QSaveFile) QSaveFile_PTR() *QSaveFile { return ptr }

impl /*struct*/ QSaveFile {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSaveFile {
    return QSaveFile{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSaveFile {
//  type Target = QSaveFileBASE;
//
//  fn deref(&self) -> &QSaveFileBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSaveFileBASE> for QSaveFile {
//  fn as_ref(& self) -> & QSaveFileBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsavefile.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSaveFile {
  pub fn metaObject_0<RetType, T: QSaveFile_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSaveFile_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSaveFile) -> RetType;
}
impl<'a> /*trait*/ QSaveFile_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSaveFile) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSaveFile10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsavefile.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSaveFile(const QString &)

/*
Constructs a new file object to represent the file with the given name.
*/
// QSaveFile(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QSaveFile {
  pub fn QSaveFile_0<T: QSaveFile_QSaveFile_0>(value: T) -> QSaveFile {
    let rsthis = value.QSaveFile_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSaveFile_QSaveFile_0 {
  fn QSaveFile_0(self) -> QSaveFile;
}
// QSaveFile(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSaveFile_QSaveFile_0 for (usize) {
  fn QSaveFile_0(self) -> QSaveFile {
    // unsafe{_ZN9QSaveFileC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QSaveFileC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSaveFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsavefile.h:70
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QSaveFile(QObject *)

/*
Constructs a new file object to represent the file with the given name.
*/
// QSaveFile(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSaveFile {
  pub fn QSaveFile_1<T: QSaveFile_QSaveFile_1>(value: T) -> QSaveFile {
    let rsthis = value.QSaveFile_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSaveFile_QSaveFile_1 {
  fn QSaveFile_1(self) -> QSaveFile;
}
// QSaveFile(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSaveFile_QSaveFile_1 for (usize) {
  fn QSaveFile_1(self) -> QSaveFile {
    // unsafe{_ZN9QSaveFileC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QSaveFileC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSaveFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsavefile.h:71
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QSaveFile(const QString &, QObject *)

/*
Constructs a new file object to represent the file with the given name.
*/
// QSaveFile(const QString &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSaveFile {
  pub fn QSaveFile_2<T: QSaveFile_QSaveFile_2>(value: T) -> QSaveFile {
    let rsthis = value.QSaveFile_2();
    return rsthis;
    // return 1;
  }
}

pub trait QSaveFile_QSaveFile_2 {
  fn QSaveFile_2(self) -> QSaveFile;
}
// QSaveFile(const QString &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSaveFile_QSaveFile_2 for (usize,usize) {
  fn QSaveFile_2(self) -> QSaveFile {
    // unsafe{_ZN9QSaveFileC2ERK7QStringP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QSaveFileC2ERK7QStringP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSaveFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsavefile.h:73
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSaveFile()

/*

*/
pub fn DeleteQSaveFile(this :*mut QSaveFile) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QSaveFileD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qsavefile.h:75
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString fileName() const

/*
Reimplemented from QFileDevice::fileName().

Returns the name set by setFileName() or to the QSaveFile constructor.

See also setFileName().
*/
impl /*struct*/ QSaveFile {
  pub fn fileName_0<RetType, T: QSaveFile_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QSaveFile_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QSaveFile) -> RetType;
}
impl<'a> /*trait*/ QSaveFile_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QSaveFile) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSaveFile8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsavefile.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFileName(const QString &)

/*
Sets the name of the file. The name can have no path, a relative path, or an absolute path.

See also QFile::setFileName() and fileName().
*/
impl /*struct*/ QSaveFile {
  pub fn setFileName_0<RetType, T: QSaveFile_setFileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileName_0(self);
    // return 1;
  }
}
pub trait QSaveFile_setFileName_0<RetType> {
  fn setFileName_0(self , rsthis: & QSaveFile) -> RetType;
}
impl<'a> /*trait*/ QSaveFile_setFileName_0<(/*void*/)> for (usize) {
  fn setFileName_0(self , rsthis: & QSaveFile) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSaveFile11setFileNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsavefile.h:78
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool open(QIODevice::OpenMode)

/*
Reimplemented from QIODevice::open().

Opens the file using OpenMode mode, returning true if successful; otherwise false.

Important: the mode must include QIODevice::WriteOnly. It may also have additional flags, such as QIODevice::Text and QIODevice::Unbuffered.

QIODevice::ReadWrite and QIODevice::Append are not supported at the moment.

See also QIODevice::OpenMode and setFileName().
*/
impl /*struct*/ QSaveFile {
  pub fn open_0<RetType, T: QSaveFile_open_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_0(self);
    // return 1;
  }
}
pub trait QSaveFile_open_0<RetType> {
  fn open_0(self , rsthis: & QSaveFile) -> RetType;
}
impl<'a> /*trait*/ QSaveFile_open_0<bool> for (i32) {
  fn open_0(self , rsthis: & QSaveFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QSaveFile4openE6QFlagsIN9QIODevice12OpenModeFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsavefile.h:79
// index:0
// Public Visibility=Default Availability=Available
// [1] bool commit()

/*
Commits the changes to disk, if all previous writes were successful.

It is mandatory to call this at the end of the saving operation, otherwise the file will be discarded.

If an error happened during writing, deletes the temporary file and returns false. Otherwise, renames it to the final fileName and returns true on success. Finally, closes the device.

See also cancelWriting().
*/
impl /*struct*/ QSaveFile {
  pub fn commit_0<RetType, T: QSaveFile_commit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.commit_0(self);
    // return 1;
  }
}
pub trait QSaveFile_commit_0<RetType> {
  fn commit_0(self , rsthis: & QSaveFile) -> RetType;
}
impl<'a> /*trait*/ QSaveFile_commit_0<bool> for () {
  fn commit_0(self , rsthis: & QSaveFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QSaveFile6commitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsavefile.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cancelWriting()

/*
Cancels writing the new file.

If the application changes its mind while saving, it can call cancelWriting(), which sets an error code so that commit() will discard the temporary file.

Alternatively, it can simply make sure not to call commit().

Further write operations are possible after calling this method, but none of it will have any effect, the written file will be discarded.

This method has no effect when direct write fallback is used. This is the case when saving over an existing file in a readonly directory: no temporary file can be created, so the existing file is overwritten no matter what, and cancelWriting() cannot do anything about that, the contents of the existing file will be lost.

See also commit().
*/
impl /*struct*/ QSaveFile {
  pub fn cancelWriting_0<RetType, T: QSaveFile_cancelWriting_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cancelWriting_0(self);
    // return 1;
  }
}
pub trait QSaveFile_cancelWriting_0<RetType> {
  fn cancelWriting_0(self , rsthis: & QSaveFile) -> RetType;
}
impl<'a> /*trait*/ QSaveFile_cancelWriting_0<(/*void*/)> for () {
  fn cancelWriting_0(self , rsthis: & QSaveFile) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QSaveFile13cancelWritingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsavefile.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDirectWriteFallback(bool)

/*
Allows writing over the existing file if necessary.

QSaveFile creates a temporary file in the same directory as the final file and atomically renames it. However this is not possible if the directory permissions do not allow creating new files. In order to preserve atomicity guarantees, open() fails when it cannot create the temporary file.

In order to allow users to edit files with write permissions in a directory with restricted permissions, call setDirectWriteFallback() with enabled set to true, and the following calls to open() will fallback to opening the existing file directly and writing into it, without the use of a temporary file. This does not have atomicity guarantees, i.e. an application crash or for instance a power failure could lead to a partially-written file on disk. It also means cancelWriting() has no effect, in such a case.

Typically, to save documents edited by the user, call setDirectWriteFallback(true), and to save application internal files (configuration files, data files, ...), keep the default setting which ensures atomicity.

See also directWriteFallback().
*/
impl /*struct*/ QSaveFile {
  pub fn setDirectWriteFallback_0<RetType, T: QSaveFile_setDirectWriteFallback_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDirectWriteFallback_0(self);
    // return 1;
  }
}
pub trait QSaveFile_setDirectWriteFallback_0<RetType> {
  fn setDirectWriteFallback_0(self , rsthis: & QSaveFile) -> RetType;
}
impl<'a> /*trait*/ QSaveFile_setDirectWriteFallback_0<(/*void*/)> for (bool) {
  fn setDirectWriteFallback_0(self , rsthis: & QSaveFile) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QSaveFile22setDirectWriteFallbackEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsavefile.h:84
// index:0
// Public Visibility=Default Availability=Available
// [1] bool directWriteFallback() const

/*
Returns true if the fallback solution for saving files in read-only directories is enabled.

See also setDirectWriteFallback().
*/
impl /*struct*/ QSaveFile {
  pub fn directWriteFallback_0<RetType, T: QSaveFile_directWriteFallback_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.directWriteFallback_0(self);
    // return 1;
  }
}
pub trait QSaveFile_directWriteFallback_0<RetType> {
  fn directWriteFallback_0(self , rsthis: & QSaveFile) -> RetType;
}
impl<'a> /*trait*/ QSaveFile_directWriteFallback_0<bool> for () {
  fn directWriteFallback_0(self , rsthis: & QSaveFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSaveFile19directWriteFallbackEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsavefile.h:87
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] qint64 writeData(const char *, qint64)

/*
Reimplemented from QIODevice::writeData().
*/
impl /*struct*/ QSaveFile {
  pub fn writeData_0<RetType, T: QSaveFile_writeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeData_0(self);
    // return 1;
  }
}
pub trait QSaveFile_writeData_0<RetType> {
  fn writeData_0(self , rsthis: & QSaveFile) -> RetType;
}
impl<'a> /*trait*/ QSaveFile_writeData_0<i64> for (usize,i64) {
  fn writeData_0(self , rsthis: & QSaveFile) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QSaveFile9writeDataEPKcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
