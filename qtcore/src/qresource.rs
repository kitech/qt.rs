

// mod ::core::QResource
// package qtcore
// /usr/include/qt/QtCore/qresource.h
// #include <qresource.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 12
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

// bool isDir()
// func (this *QResource) InheritIsDir(f func() bool) {
//  qtrt.SetAllInheritCallback(this, "isDir", f)
// }

// bool isFile()
// func (this *QResource) InheritIsFile(f func() bool) {
//  qtrt.SetAllInheritCallback(this, "isFile", f)
// }

// QStringList children()
// func (this *QResource) InheritChildren(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "children", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QResource)=8
pub struct QResource {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QResource_ITF interface {
//    QResource_PTR() *QResource
//}
//func (ptr *QResource) QResource_PTR() *QResource { return ptr }

impl /*struct*/ QResource {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QResource {
    return QResource{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QResource {
//  type Target = QResourceBASE;
//
//  fn deref(&self) -> &QResourceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QResourceBASE> for QResource {
//  fn as_ref(& self) -> & QResourceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qresource.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QResource(const QString &, const QLocale &)

/*
Constructs a QResource pointing to file. locale is used to load a specific localization of a resource data.

See also QFileInfo, QDir::searchPaths(), setFileName(), and setLocale().
*/
// QResource(const QString &, const QLocale &) ctx.fn_proto_cpp
impl /*struct*/ QResource {
  pub fn QResource_0<T: QResource_QResource_0>(value: T) -> QResource {
    let rsthis = value.QResource_0();
    return rsthis;
    // return 1;
  }
}

pub trait QResource_QResource_0 {
  fn QResource_0(self) -> QResource;
}
// QResource(const QString &, const QLocale &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QResource_QResource_0 for (usize,usize) {
  fn QResource_0(self) -> QResource {
    // unsafe{_ZN9QResourceC2ERK7QStringRK7QLocale()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QResourceC2ERK7QStringRK7QLocale", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QResource{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qresource.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QResource()

/*

*/
pub fn DeleteQResource(this :*mut QResource) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QResourceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qresource.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFileName(const QString &)

/*
Sets a QResource to point to file. file can either be absolute, in which case it is opened directly, if relative then the file will be tried to be found in QDir::searchPaths().

See also fileName() and absoluteFilePath().
*/
impl /*struct*/ QResource {
  pub fn setFileName_0<RetType, T: QResource_setFileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileName_0(self);
    // return 1;
  }
}
pub trait QResource_setFileName_0<RetType> {
  fn setFileName_0(self , rsthis: & QResource) -> RetType;
}
impl<'a> /*trait*/ QResource_setFileName_0<(/*void*/)> for (usize) {
  fn setFileName_0(self , rsthis: & QResource) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QResource11setFileNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qresource.h:61
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fileName() const

/*
Returns the full path to the file that this QResource represents as it was passed.

See also setFileName() and absoluteFilePath().
*/
impl /*struct*/ QResource {
  pub fn fileName_0<RetType, T: QResource_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QResource_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QResource) -> RetType;
}
impl<'a> /*trait*/ QResource_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QResource) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QResource8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:62
// index:0
// Public Visibility=Default Availability=Available
// [8] QString absoluteFilePath() const

/*
Returns the real path that this QResource represents, if the resource was found via the QDir::searchPaths() it will be indicated in the path.

See also fileName().
*/
impl /*struct*/ QResource {
  pub fn absoluteFilePath_0<RetType, T: QResource_absoluteFilePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.absoluteFilePath_0(self);
    // return 1;
  }
}
pub trait QResource_absoluteFilePath_0<RetType> {
  fn absoluteFilePath_0(self , rsthis: & QResource) -> RetType;
}
impl<'a> /*trait*/ QResource_absoluteFilePath_0<usize> for () {
  fn absoluteFilePath_0(self , rsthis: & QResource) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QResource16absoluteFilePathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLocale(const QLocale &)

/*
Sets a QResource to only load the localization of resource to for locale. If a resource for the specific locale is not found then the C locale is used.

See also locale() and setFileName().
*/
impl /*struct*/ QResource {
  pub fn setLocale_0<RetType, T: QResource_setLocale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLocale_0(self);
    // return 1;
  }
}
pub trait QResource_setLocale_0<RetType> {
  fn setLocale_0(self , rsthis: & QResource) -> RetType;
}
impl<'a> /*trait*/ QResource_setLocale_0<(/*void*/)> for (usize) {
  fn setLocale_0(self , rsthis: & QResource) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QResource9setLocaleERK7QLocale", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qresource.h:65
// index:0
// Public Visibility=Default Availability=Available
// [8] QLocale locale() const

/*
Returns the locale used to locate the data for the QResource.

See also setLocale().
*/
impl /*struct*/ QResource {
  pub fn locale_0<RetType, T: QResource_locale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.locale_0(self);
    // return 1;
  }
}
pub trait QResource_locale_0<RetType> {
  fn locale_0(self , rsthis: & QResource) -> RetType;
}
impl<'a> /*trait*/ QResource_locale_0<usize> for () {
  fn locale_0(self , rsthis: & QResource) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QResource6localeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:67
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the resource really exists in the resource hierarchy, false otherwise.
*/
impl /*struct*/ QResource {
  pub fn isValid_0<RetType, T: QResource_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QResource_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QResource) -> RetType;
}
impl<'a> /*trait*/ QResource_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QResource) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QResource7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:69
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCompressed() const

/*
Returns true if the resource represents a file and the data backing it is in a compressed format, false otherwise.

See also data() and isFile().
*/
impl /*struct*/ QResource {
  pub fn isCompressed_0<RetType, T: QResource_isCompressed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCompressed_0(self);
    // return 1;
  }
}
pub trait QResource_isCompressed_0<RetType> {
  fn isCompressed_0(self , rsthis: & QResource) -> RetType;
}
impl<'a> /*trait*/ QResource_isCompressed_0<bool> for () {
  fn isCompressed_0(self , rsthis: & QResource) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QResource12isCompressedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:70
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 size() const

/*
Returns the size of the data backing the resource.

See also data() and isFile().
*/
impl /*struct*/ QResource {
  pub fn size_0<RetType, T: QResource_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QResource_size_0<RetType> {
  fn size_0(self , rsthis: & QResource) -> RetType;
}
impl<'a> /*trait*/ QResource_size_0<i64> for () {
  fn size_0(self , rsthis: & QResource) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QResource4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:71
// index:0
// Public Visibility=Default Availability=Available
// [8] const uchar * data() const

/*
Returns direct access to a read only segment of data that this resource represents. If the resource is compressed the data returns is compressed and qUncompress() must be used to access the data. If the resource is a directory 0 is returned.

See also size(), isCompressed(), and isFile().
*/
impl /*struct*/ QResource {
  pub fn data_0<RetType, T: QResource_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QResource_data_0<RetType> {
  fn data_0(self , rsthis: & QResource) -> RetType;
}
impl<'a> /*trait*/ QResource_data_0<usize> for () {
  fn data_0(self , rsthis: & QResource) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QResource4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:72
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime lastModified() const

/*
Returns the date and time when the file was last modified before packaging into a resource.
*/
impl /*struct*/ QResource {
  pub fn lastModified_0<RetType, T: QResource_lastModified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastModified_0(self);
    // return 1;
  }
}
pub trait QResource_lastModified_0<RetType> {
  fn lastModified_0(self , rsthis: & QResource) -> RetType;
}
impl<'a> /*trait*/ QResource_lastModified_0<usize> for () {
  fn lastModified_0(self , rsthis: & QResource) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QResource12lastModifiedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:74
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void addSearchPath(const QString &)

/*

*/
impl /*struct*/ QResource {
  pub fn addSearchPath_0<RetType, T: QResource_addSearchPath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.addSearchPath_0();
    // return 1;
  }
}
pub trait QResource_addSearchPath_0<RetType> {
  fn addSearchPath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QResource_addSearchPath_0<(/*void*/)> for (usize) {
  fn addSearchPath_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QResource13addSearchPathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qresource.h:75
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList searchPaths()

/*

*/
impl /*struct*/ QResource {
  pub fn searchPaths_0<RetType, T: QResource_searchPaths_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.searchPaths_0();
    // return 1;
  }
}
pub trait QResource_searchPaths_0<RetType> {
  fn searchPaths_0(self ) -> RetType;
}
impl<'a> /*trait*/ QResource_searchPaths_0<usize> for () {
  fn searchPaths_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QResource11searchPathsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:77
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool registerResource(const QString &, const QString &)

/*
Registers the resource with the given rccFileName at the location in the resource tree specified by mapRoot, and returns true if the file is successfully opened; otherwise returns false.

See also unregisterResource().
*/
impl /*struct*/ QResource {
  pub fn registerResource_0<RetType, T: QResource_registerResource_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.registerResource_0();
    // return 1;
  }
}
pub trait QResource_registerResource_0<RetType> {
  fn registerResource_0(self ) -> RetType;
}
impl<'a> /*trait*/ QResource_registerResource_0<bool> for (usize,usize) {
  fn registerResource_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QResource16registerResourceERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:80
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool registerResource(const uchar *, const QString &)

/*
Registers the resource with the given rccFileName at the location in the resource tree specified by mapRoot, and returns true if the file is successfully opened; otherwise returns false.

See also unregisterResource().
*/
impl /*struct*/ QResource {
  pub fn registerResource_1<RetType, T: QResource_registerResource_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.registerResource_1();
    // return 1;
  }
}
pub trait QResource_registerResource_1<RetType> {
  fn registerResource_1(self ) -> RetType;
}
impl<'a> /*trait*/ QResource_registerResource_1<bool> for (usize,usize) {
  fn registerResource_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QResource16registerResourceEPKhRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:78
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool unregisterResource(const QString &, const QString &)

/*
Unregisters the resource with the given rccFileName at the location in the resource tree specified by mapRoot, and returns true if the resource is successfully unloaded and no references exist for the resource; otherwise returns false.

See also registerResource().
*/
impl /*struct*/ QResource {
  pub fn unregisterResource_0<RetType, T: QResource_unregisterResource_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.unregisterResource_0();
    // return 1;
  }
}
pub trait QResource_unregisterResource_0<RetType> {
  fn unregisterResource_0(self ) -> RetType;
}
impl<'a> /*trait*/ QResource_unregisterResource_0<bool> for (usize,usize) {
  fn unregisterResource_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QResource18unregisterResourceERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:81
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool unregisterResource(const uchar *, const QString &)

/*
Unregisters the resource with the given rccFileName at the location in the resource tree specified by mapRoot, and returns true if the resource is successfully unloaded and no references exist for the resource; otherwise returns false.

See also registerResource().
*/
impl /*struct*/ QResource {
  pub fn unregisterResource_1<RetType, T: QResource_unregisterResource_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.unregisterResource_1();
    // return 1;
  }
}
pub trait QResource_unregisterResource_1<RetType> {
  fn unregisterResource_1(self ) -> RetType;
}
impl<'a> /*trait*/ QResource_unregisterResource_1<bool> for (usize,usize) {
  fn unregisterResource_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QResource18unregisterResourceEPKhRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:86
// index:0
// Protected Visibility=Default Availability=Available
// [1] bool isDir() const

/*
Returns true if the resource represents a directory and thus may have children() in it, false if it represents a file.

See also isFile().
*/
impl /*struct*/ QResource {
  pub fn isDir_0<RetType, T: QResource_isDir_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDir_0(self);
    // return 1;
  }
}
pub trait QResource_isDir_0<RetType> {
  fn isDir_0(self , rsthis: & QResource) -> RetType;
}
impl<'a> /*trait*/ QResource_isDir_0<bool> for () {
  fn isDir_0(self , rsthis: & QResource) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QResource5isDirEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:87
// index:0
// Protected inline Visibility=Default Availability=Available
// [1] bool isFile() const

/*
Returns true if the resource represents a file and thus has data backing it, false if it represents a directory.

See also isDir().
*/
impl /*struct*/ QResource {
  pub fn isFile_0<RetType, T: QResource_isFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFile_0(self);
    // return 1;
  }
}
pub trait QResource_isFile_0<RetType> {
  fn isFile_0(self , rsthis: & QResource) -> RetType;
}
impl<'a> /*trait*/ QResource_isFile_0<bool> for () {
  fn isFile_0(self , rsthis: & QResource) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QResource6isFileEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qresource.h:88
// index:0
// Protected Visibility=Default Availability=Available
// [8] QStringList children() const

/*
Returns a list of all resources in this directory, if the resource represents a file the list will be empty.

See also isDir().
*/
impl /*struct*/ QResource {
  pub fn children_0<RetType, T: QResource_children_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.children_0(self);
    // return 1;
  }
}
pub trait QResource_children_0<RetType> {
  fn children_0(self , rsthis: & QResource) -> RetType;
}
impl<'a> /*trait*/ QResource_children_0<usize> for () {
  fn children_0(self , rsthis: & QResource) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QResource8childrenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
