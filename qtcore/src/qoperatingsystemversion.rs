

// mod ::core::QOperatingSystemVersion
// package qtcore
// /usr/include/qt/QtCore/qoperatingsystemversion.h
// #include <qoperatingsystemversion.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 7
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
#[derive(Default)] // class sizeof(QOperatingSystemVersion)=16
pub struct QOperatingSystemVersion {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QOperatingSystemVersion_ITF interface {
//    QOperatingSystemVersion_PTR() *QOperatingSystemVersion
//}
//func (ptr *QOperatingSystemVersion) QOperatingSystemVersion_PTR() *QOperatingSystemVersion { return ptr }

impl /*struct*/ QOperatingSystemVersion {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QOperatingSystemVersion {
    return QOperatingSystemVersion{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QOperatingSystemVersion {
//  type Target = QOperatingSystemVersionBASE;
//
//  fn deref(&self) -> &QOperatingSystemVersionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QOperatingSystemVersionBASE> for QOperatingSystemVersion {
//  fn as_ref(& self) -> & QOperatingSystemVersionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qoperatingsystemversion.h:85
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QOperatingSystemVersion(QOperatingSystemVersion::OSType, int, int, int)

/*
Constructs a QOperatingSystemVersion consisting of the OS type osType, and major, minor, and micro version numbers vmajor, vminor and vmicro, respectively.
*/
// QOperatingSystemVersion(QOperatingSystemVersion::OSType, int, int, int) ctx.fn_proto_cpp
impl /*struct*/ QOperatingSystemVersion {
  pub fn QOperatingSystemVersion_0<T: QOperatingSystemVersion_QOperatingSystemVersion_0>(value: T) -> QOperatingSystemVersion {
    let rsthis = value.QOperatingSystemVersion_0();
    return rsthis;
    // return 1;
  }
}

pub trait QOperatingSystemVersion_QOperatingSystemVersion_0 {
  fn QOperatingSystemVersion_0(self) -> QOperatingSystemVersion;
}
// QOperatingSystemVersion(QOperatingSystemVersion::OSType, int, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QOperatingSystemVersion_QOperatingSystemVersion_0 for (i32,i32,i32,i32) {
  fn QOperatingSystemVersion_0(self) -> QOperatingSystemVersion {
    // unsafe{_ZN23QOperatingSystemVersionC2ENS_6OSTypeEiii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QOperatingSystemVersionC2ENS_6OSTypeEiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QOperatingSystemVersion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qoperatingsystemversion.h:93
// index:0
// Public static Visibility=Default Availability=Available
// [16] QOperatingSystemVersion current()

/*
Returns a QOperatingSystemVersion indicating the current OS and its version number.

See also currentType().
*/
impl /*struct*/ QOperatingSystemVersion {
  pub fn current_0<RetType, T: QOperatingSystemVersion_current_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.current_0();
    // return 1;
  }
}
pub trait QOperatingSystemVersion_current_0<RetType> {
  fn current_0(self ) -> RetType;
}
impl<'a> /*trait*/ QOperatingSystemVersion_current_0<usize> for () {
  fn current_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QOperatingSystemVersion7currentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qoperatingsystemversion.h:95
// index:0
// Public static inline Visibility=Default Availability=Available
// [4] QOperatingSystemVersion::OSType currentType()

/*
Returns the current OS type without constructing a QOperatingSystemVersion instance.

See also current().
*/
impl /*struct*/ QOperatingSystemVersion {
  pub fn currentType_0<RetType, T: QOperatingSystemVersion_currentType_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentType_0();
    // return 1;
  }
}
pub trait QOperatingSystemVersion_currentType_0<RetType> {
  fn currentType_0(self ) -> RetType;
}
impl<'a> /*trait*/ QOperatingSystemVersion_currentType_0<i32> for () {
  fn currentType_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QOperatingSystemVersion11currentTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qoperatingsystemversion.h:114
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int majorVersion() const

/*
Returns the major version number, that is, the first segment of the operating system's version number.

See the main class documentation for what the major version number is on a given operating system.

-1 indicates an unknown or absent version number component.

See also minorVersion() and microVersion().
*/
impl /*struct*/ QOperatingSystemVersion {
  pub fn majorVersion_0<RetType, T: QOperatingSystemVersion_majorVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.majorVersion_0(self);
    // return 1;
  }
}
pub trait QOperatingSystemVersion_majorVersion_0<RetType> {
  fn majorVersion_0(self , rsthis: & QOperatingSystemVersion) -> RetType;
}
impl<'a> /*trait*/ QOperatingSystemVersion_majorVersion_0<i32> for () {
  fn majorVersion_0(self , rsthis: & QOperatingSystemVersion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QOperatingSystemVersion12majorVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qoperatingsystemversion.h:115
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int minorVersion() const

/*
Returns the minor version number, that is, the second segment of the operating system's version number.

See the main class documentation for what the minor version number is on a given operating system.

-1 indicates an unknown or absent version number component.

See also majorVersion() and microVersion().
*/
impl /*struct*/ QOperatingSystemVersion {
  pub fn minorVersion_0<RetType, T: QOperatingSystemVersion_minorVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minorVersion_0(self);
    // return 1;
  }
}
pub trait QOperatingSystemVersion_minorVersion_0<RetType> {
  fn minorVersion_0(self , rsthis: & QOperatingSystemVersion) -> RetType;
}
impl<'a> /*trait*/ QOperatingSystemVersion_minorVersion_0<i32> for () {
  fn minorVersion_0(self , rsthis: & QOperatingSystemVersion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QOperatingSystemVersion12minorVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qoperatingsystemversion.h:116
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int microVersion() const

/*
Returns the micro version number, that is, the third segment of the operating system's version number.

See the main class documentation for what the micro version number is on a given operating system.

-1 indicates an unknown or absent version number component.

See also majorVersion() and minorVersion().
*/
impl /*struct*/ QOperatingSystemVersion {
  pub fn microVersion_0<RetType, T: QOperatingSystemVersion_microVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.microVersion_0(self);
    // return 1;
  }
}
pub trait QOperatingSystemVersion_microVersion_0<RetType> {
  fn microVersion_0(self , rsthis: & QOperatingSystemVersion) -> RetType;
}
impl<'a> /*trait*/ QOperatingSystemVersion_microVersion_0<i32> for () {
  fn microVersion_0(self , rsthis: & QOperatingSystemVersion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QOperatingSystemVersion12microVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qoperatingsystemversion.h:118
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int segmentCount() const

/*
Returns the number of integers stored in the version number.
*/
impl /*struct*/ QOperatingSystemVersion {
  pub fn segmentCount_0<RetType, T: QOperatingSystemVersion_segmentCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.segmentCount_0(self);
    // return 1;
  }
}
pub trait QOperatingSystemVersion_segmentCount_0<RetType> {
  fn segmentCount_0(self , rsthis: & QOperatingSystemVersion) -> RetType;
}
impl<'a> /*trait*/ QOperatingSystemVersion_segmentCount_0<i32> for () {
  fn segmentCount_0(self , rsthis: & QOperatingSystemVersion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QOperatingSystemVersion12segmentCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qoperatingsystemversion.h:124
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QOperatingSystemVersion::OSType type() const

/*
Returns the OS type identified by the QOperatingSystemVersion.

See also name().
*/
impl /*struct*/ QOperatingSystemVersion {
  pub fn type__0<RetType, T: QOperatingSystemVersion_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QOperatingSystemVersion_type__0<RetType> {
  fn type__0(self , rsthis: & QOperatingSystemVersion) -> RetType;
}
impl<'a> /*trait*/ QOperatingSystemVersion_type__0<i32> for () {
  fn type__0(self , rsthis: & QOperatingSystemVersion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QOperatingSystemVersion4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qoperatingsystemversion.h:125
// index:0
// Public Visibility=Default Availability=Available
// [8] QString name() const

/*
Returns a string representation of the OS type identified by the QOperatingSystemVersion.

See also type().
*/
impl /*struct*/ QOperatingSystemVersion {
  pub fn name_0<RetType, T: QOperatingSystemVersion_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QOperatingSystemVersion_name_0<RetType> {
  fn name_0(self , rsthis: & QOperatingSystemVersion) -> RetType;
}
impl<'a> /*trait*/ QOperatingSystemVersion_name_0<usize> for () {
  fn name_0(self , rsthis: & QOperatingSystemVersion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QOperatingSystemVersion4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQOperatingSystemVersion(this :*mut QOperatingSystemVersion) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN23QOperatingSystemVersionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum provides symbolic names for the various operating system families supported by QOperatingSystemVersion.


*/
pub type QOperatingSystemVersion__OSType = i32;
// An unknown or unsupported operating system.
pub const QOperatingSystemVersion__Unknown :QOperatingSystemVersion__OSType = 0;
// The Microsoft Windows operating system.
pub const QOperatingSystemVersion__Windows :QOperatingSystemVersion__OSType = 1;
// The Apple macOS operating system.
pub const QOperatingSystemVersion__MacOS :QOperatingSystemVersion__OSType = 2;
// The Apple iOS operating system.
pub const QOperatingSystemVersion__IOS :QOperatingSystemVersion__OSType = 3;
// The Apple tvOS operating system.
pub const QOperatingSystemVersion__TvOS :QOperatingSystemVersion__OSType = 4;
// The Apple watchOS operating system.
pub const QOperatingSystemVersion__WatchOS :QOperatingSystemVersion__OSType = 5;
// The Google Android operating system.
pub const QOperatingSystemVersion__Android :QOperatingSystemVersion__OSType = 6;
pub fn QOperatingSystemVersion_OSTypeItemName(val: i32) ->String {
  match val {
     QOperatingSystemVersion__Unknown => // 0
     {return String::from("Unknown");}
     QOperatingSystemVersion__Windows => // 1
     {return String::from("Windows");}
     QOperatingSystemVersion__MacOS => // 2
     {return String::from("MacOS");}
     QOperatingSystemVersion__IOS => // 3
     {return String::from("IOS");}
     QOperatingSystemVersion__TvOS => // 4
     {return String::from("TvOS");}
     QOperatingSystemVersion__WatchOS => // 5
     {return String::from("WatchOS");}
     QOperatingSystemVersion__Android => // 6
     {return String::from("Android");}
  _ => {return format!("{}", val);}
}
}
pub fn QOperatingSystemVersion_OSTypeItemName_s(val: i32) ->String {
  //var nilthis *QOperatingSystemVersion
  //return nilthis.OSTypeItemName(val);
  return QOperatingSystemVersion_OSTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
