

// mod ::core::QMimeDatabase
// package qtcore
// /usr/include/qt/QtCore/qmimedatabase.h
// #include <qmimedatabase.h>
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



/*

*/
#[derive(Default)] // class sizeof(QMimeDatabase)=8
pub struct QMimeDatabase {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMimeDatabase_ITF interface {
//    QMimeDatabase_PTR() *QMimeDatabase
//}
//func (ptr *QMimeDatabase) QMimeDatabase_PTR() *QMimeDatabase { return ptr }

impl /*struct*/ QMimeDatabase {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMimeDatabase {
    return QMimeDatabase{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMimeDatabase {
//  type Target = QMimeDatabaseBASE;
//
//  fn deref(&self) -> &QMimeDatabaseBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMimeDatabaseBASE> for QMimeDatabase {
//  fn as_ref(& self) -> & QMimeDatabaseBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmimedatabase.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMimeDatabase()

/*
Constructs a QMimeDatabase object.

It is perfectly OK to create an instance of QMimeDatabase every time you need to perform a lookup. The parsing of mimetypes is done on demand (when shared-mime-info is installed) or when the very first instance is constructed (when parsing XML files directly).
*/
// QMimeDatabase() ctx.fn_proto_cpp
impl /*struct*/ QMimeDatabase {
  pub fn QMimeDatabase_0<T: QMimeDatabase_QMimeDatabase_0>(value: T) -> QMimeDatabase {
    let rsthis = value.QMimeDatabase_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMimeDatabase_QMimeDatabase_0 {
  fn QMimeDatabase_0(self) -> QMimeDatabase;
}
// QMimeDatabase() ctx.fn_proto_cpp
impl<'a> /*trait*/ QMimeDatabase_QMimeDatabase_0 for () {
  fn QMimeDatabase_0(self) -> QMimeDatabase {
    // unsafe{_ZN13QMimeDatabaseC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QMimeDatabaseC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMimeDatabase{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmimedatabase.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QMimeDatabase()

/*

*/
pub fn DeleteQMimeDatabase(this :*mut QMimeDatabase) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QMimeDatabaseD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qmimedatabase.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] QMimeType mimeTypeForName(const QString &) const

/*
Returns a MIME type for nameOrAlias or an invalid one if none found.
*/
impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForName_0<RetType, T: QMimeDatabase_mimeTypeForName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypeForName_0(self);
    // return 1;
  }
}
pub trait QMimeDatabase_mimeTypeForName_0<RetType> {
  fn mimeTypeForName_0(self , rsthis: & QMimeDatabase) -> RetType;
}
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForName_0<usize> for (usize) {
  fn mimeTypeForName_0(self , rsthis: & QMimeDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMimeDatabase15mimeTypeForNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedatabase.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QMimeType mimeTypeForFile(const QString &, QMimeDatabase::MatchMode) const

/*
Returns a MIME type for fileInfo.

A valid MIME type is always returned.

The default matching algorithm looks at both the file name and the file contents, if necessary. The file extension has priority over the contents, but the contents will be used if the file extension is unknown, or matches multiple MIME types. If fileInfo is a Unix symbolic link, the file that it refers to will be used instead. If the file doesn't match any known pattern or data, the default MIME type (application/octet-stream) is returned.

When mode is set to MatchExtension, only the file name is used, not the file contents. The file doesn't even have to exist. If the file name doesn't match any known pattern, the default MIME type (application/octet-stream) is returned. If multiple MIME types match this file, the first one (alphabetically) is returned.

When mode is set to MatchContent, and the file is readable, only the file contents are used to determine the MIME type. This is equivalent to calling mimeTypeForData with a QFile as input device.

fileInfo may refer to an absolute or relative path.

See also QMimeType::isDefault() and mimeTypeForData().
*/
impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForFile_0<RetType, T: QMimeDatabase_mimeTypeForFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypeForFile_0(self);
    // return 1;
  }
}
pub trait QMimeDatabase_mimeTypeForFile_0<RetType> {
  fn mimeTypeForFile_0(self , rsthis: & QMimeDatabase) -> RetType;
}
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForFile_0<usize> for (usize,i32) {
  fn mimeTypeForFile_0(self , rsthis: & QMimeDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMimeDatabase15mimeTypeForFileERK7QStringNS_9MatchModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedatabase.h:75
// index:1
// Public Visibility=Default Availability=Available
// [8] QMimeType mimeTypeForFile(const QFileInfo &, QMimeDatabase::MatchMode) const

/*
Returns a MIME type for fileInfo.

A valid MIME type is always returned.

The default matching algorithm looks at both the file name and the file contents, if necessary. The file extension has priority over the contents, but the contents will be used if the file extension is unknown, or matches multiple MIME types. If fileInfo is a Unix symbolic link, the file that it refers to will be used instead. If the file doesn't match any known pattern or data, the default MIME type (application/octet-stream) is returned.

When mode is set to MatchExtension, only the file name is used, not the file contents. The file doesn't even have to exist. If the file name doesn't match any known pattern, the default MIME type (application/octet-stream) is returned. If multiple MIME types match this file, the first one (alphabetically) is returned.

When mode is set to MatchContent, and the file is readable, only the file contents are used to determine the MIME type. This is equivalent to calling mimeTypeForData with a QFile as input device.

fileInfo may refer to an absolute or relative path.

See also QMimeType::isDefault() and mimeTypeForData().
*/
impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForFile_1<RetType, T: QMimeDatabase_mimeTypeForFile_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypeForFile_1(self);
    // return 1;
  }
}
pub trait QMimeDatabase_mimeTypeForFile_1<RetType> {
  fn mimeTypeForFile_1(self , rsthis: & QMimeDatabase) -> RetType;
}
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForFile_1<usize> for (usize,i32) {
  fn mimeTypeForFile_1(self , rsthis: & QMimeDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMimeDatabase15mimeTypeForFileERK9QFileInfoNS_9MatchModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedatabase.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QMimeType mimeTypeForData(const QByteArray &) const

/*
Returns a MIME type for data.

A valid MIME type is always returned. If data doesn't match any known MIME type data, the default MIME type (application/octet-stream) is returned.
*/
impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForData_0<RetType, T: QMimeDatabase_mimeTypeForData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypeForData_0(self);
    // return 1;
  }
}
pub trait QMimeDatabase_mimeTypeForData_0<RetType> {
  fn mimeTypeForData_0(self , rsthis: & QMimeDatabase) -> RetType;
}
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForData_0<usize> for (usize) {
  fn mimeTypeForData_0(self , rsthis: & QMimeDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMimeDatabase15mimeTypeForDataERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedatabase.h:79
// index:1
// Public Visibility=Default Availability=Available
// [8] QMimeType mimeTypeForData(QIODevice *) const

/*
Returns a MIME type for data.

A valid MIME type is always returned. If data doesn't match any known MIME type data, the default MIME type (application/octet-stream) is returned.
*/
impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForData_1<RetType, T: QMimeDatabase_mimeTypeForData_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypeForData_1(self);
    // return 1;
  }
}
pub trait QMimeDatabase_mimeTypeForData_1<RetType> {
  fn mimeTypeForData_1(self , rsthis: & QMimeDatabase) -> RetType;
}
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForData_1<usize> for (usize) {
  fn mimeTypeForData_1(self , rsthis: & QMimeDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMimeDatabase15mimeTypeForDataEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedatabase.h:81
// index:0
// Public Visibility=Default Availability=Available
// [8] QMimeType mimeTypeForUrl(const QUrl &) const

/*
Returns a MIME type for url.

If the URL is a local file, this calls mimeTypeForFile.

Otherwise the matching is done based on the file name only, except for schemes where file names don't mean much, like HTTP. This method always returns the default mimetype for HTTP URLs, use QNetworkAccessManager to handle HTTP URLs properly.

A valid MIME type is always returned. If url doesn't match any known MIME type data, the default MIME type (application/octet-stream) is returned.
*/
impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForUrl_0<RetType, T: QMimeDatabase_mimeTypeForUrl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypeForUrl_0(self);
    // return 1;
  }
}
pub trait QMimeDatabase_mimeTypeForUrl_0<RetType> {
  fn mimeTypeForUrl_0(self , rsthis: & QMimeDatabase) -> RetType;
}
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForUrl_0<usize> for (usize) {
  fn mimeTypeForUrl_0(self , rsthis: & QMimeDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMimeDatabase14mimeTypeForUrlERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedatabase.h:82
// index:0
// Public Visibility=Default Availability=Available
// [8] QMimeType mimeTypeForFileNameAndData(const QString &, QIODevice *) const

/*
Returns a MIME type for the given fileName and device data.

This overload can be useful when the file is remote, and we started to download some of its data in a device. This allows to do full MIME type matching for remote files as well.

If the device is not open, it will be opened by this function, and closed after the MIME type detection is completed.

A valid MIME type is always returned. If device data doesn't match any known MIME type data, the default MIME type (application/octet-stream) is returned.

This method looks at both the file name and the file contents, if necessary. The file extension has priority over the contents, but the contents will be used if the file extension is unknown, or matches multiple MIME types.
*/
impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForFileNameAndData_0<RetType, T: QMimeDatabase_mimeTypeForFileNameAndData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypeForFileNameAndData_0(self);
    // return 1;
  }
}
pub trait QMimeDatabase_mimeTypeForFileNameAndData_0<RetType> {
  fn mimeTypeForFileNameAndData_0(self , rsthis: & QMimeDatabase) -> RetType;
}
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForFileNameAndData_0<usize> for (usize,usize) {
  fn mimeTypeForFileNameAndData_0(self , rsthis: & QMimeDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringP9QIODevice", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedatabase.h:83
// index:1
// Public Visibility=Default Availability=Available
// [8] QMimeType mimeTypeForFileNameAndData(const QString &, const QByteArray &) const

/*
Returns a MIME type for the given fileName and device data.

This overload can be useful when the file is remote, and we started to download some of its data in a device. This allows to do full MIME type matching for remote files as well.

If the device is not open, it will be opened by this function, and closed after the MIME type detection is completed.

A valid MIME type is always returned. If device data doesn't match any known MIME type data, the default MIME type (application/octet-stream) is returned.

This method looks at both the file name and the file contents, if necessary. The file extension has priority over the contents, but the contents will be used if the file extension is unknown, or matches multiple MIME types.
*/
impl /*struct*/ QMimeDatabase {
  pub fn mimeTypeForFileNameAndData_1<RetType, T: QMimeDatabase_mimeTypeForFileNameAndData_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypeForFileNameAndData_1(self);
    // return 1;
  }
}
pub trait QMimeDatabase_mimeTypeForFileNameAndData_1<RetType> {
  fn mimeTypeForFileNameAndData_1(self , rsthis: & QMimeDatabase) -> RetType;
}
impl<'a> /*trait*/ QMimeDatabase_mimeTypeForFileNameAndData_1<usize> for (usize,usize) {
  fn mimeTypeForFileNameAndData_1(self , rsthis: & QMimeDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMimeDatabase26mimeTypeForFileNameAndDataERK7QStringRK10QByteArray", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedatabase.h:85
// index:0
// Public Visibility=Default Availability=Available
// [8] QString suffixForFileName(const QString &) const

/*
Returns the suffix for the file fileName, as known by the MIME database.

This allows to pre-select "tar.bz2" for foo.tar.bz2, but still only "txt" for my.file.with.dots.txt.
*/
impl /*struct*/ QMimeDatabase {
  pub fn suffixForFileName_0<RetType, T: QMimeDatabase_suffixForFileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.suffixForFileName_0(self);
    // return 1;
  }
}
pub trait QMimeDatabase_suffixForFileName_0<RetType> {
  fn suffixForFileName_0(self , rsthis: & QMimeDatabase) -> RetType;
}
impl<'a> /*trait*/ QMimeDatabase_suffixForFileName_0<usize> for (usize) {
  fn suffixForFileName_0(self , rsthis: & QMimeDatabase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMimeDatabase17suffixForFileNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum specifies how matching a file to a MIME type is performed.


*/
pub type QMimeDatabase__MatchMode = i32;
// 
pub const QMimeDatabase__MatchDefault :QMimeDatabase__MatchMode = 0;
// 
pub const QMimeDatabase__MatchExtension :QMimeDatabase__MatchMode = 1;
// 
pub const QMimeDatabase__MatchContent :QMimeDatabase__MatchMode = 2;
pub fn QMimeDatabase_MatchModeItemName(val: i32) ->String {
  match val {
     QMimeDatabase__MatchDefault => // 0
     {return String::from("MatchDefault");}
     QMimeDatabase__MatchExtension => // 1
     {return String::from("MatchExtension");}
     QMimeDatabase__MatchContent => // 2
     {return String::from("MatchContent");}
  _ => {return format!("{}", val);}
}
}
pub fn QMimeDatabase_MatchModeItemName_s(val: i32) ->String {
  //var nilthis *QMimeDatabase
  //return nilthis.MatchModeItemName(val);
  return QMimeDatabase_MatchModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
