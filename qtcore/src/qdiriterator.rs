

// mod ::core::QDirIterator
// package qtcore
// /usr/include/qt/QtCore/qdiriterator.h
// #include <qdiriterator.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 71
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
#[derive(Default)] // class sizeof(QDirIterator)=8
pub struct QDirIterator {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDirIterator_ITF interface {
//    QDirIterator_PTR() *QDirIterator
//}
//func (ptr *QDirIterator) QDirIterator_PTR() *QDirIterator { return ptr }

impl /*struct*/ QDirIterator {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDirIterator {
    return QDirIterator{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDirIterator {
//  type Target = QDirIteratorBASE;
//
//  fn deref(&self) -> &QDirIteratorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDirIteratorBASE> for QDirIterator {
//  fn as_ref(& self) -> & QDirIteratorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qdiriterator.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDirIterator(const QDir &, QDirIterator::IteratorFlags)

/*
Constructs a QDirIterator that can iterate over dir's entrylist, using dir's name filters and regular filters. You can pass options via flags to decide how the directory should be iterated.

By default, flags is NoIteratorFlags, which provides the same behavior as in QDir::entryList().

The sorting in dir is ignored.

Note: To list symlinks that point to non existing files, QDir::System must be passed to the flags.

See also hasNext(), next(), and IteratorFlags.
*/
// QDirIterator(const QDir &, QDirIterator::IteratorFlags) ctx.fn_proto_cpp
impl /*struct*/ QDirIterator {
  pub fn QDirIterator_0<T: QDirIterator_QDirIterator_0>(value: T) -> QDirIterator {
    let rsthis = value.QDirIterator_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDirIterator_QDirIterator_0 {
  fn QDirIterator_0(self) -> QDirIterator;
}
// QDirIterator(const QDir &, QDirIterator::IteratorFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDirIterator_QDirIterator_0 for (usize,i32) {
  fn QDirIterator_0(self) -> QDirIterator {
    // unsafe{_ZN12QDirIteratorC2ERK4QDir6QFlagsINS_12IteratorFlagEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QDirIteratorC2ERK4QDir6QFlagsINS_12IteratorFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDirIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdiriterator.h:59
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QDirIterator(const QString &, QDirIterator::IteratorFlags)

/*
Constructs a QDirIterator that can iterate over dir's entrylist, using dir's name filters and regular filters. You can pass options via flags to decide how the directory should be iterated.

By default, flags is NoIteratorFlags, which provides the same behavior as in QDir::entryList().

The sorting in dir is ignored.

Note: To list symlinks that point to non existing files, QDir::System must be passed to the flags.

See also hasNext(), next(), and IteratorFlags.
*/
// QDirIterator(const QString &, QDirIterator::IteratorFlags) ctx.fn_proto_cpp
impl /*struct*/ QDirIterator {
  pub fn QDirIterator_1<T: QDirIterator_QDirIterator_1>(value: T) -> QDirIterator {
    let rsthis = value.QDirIterator_1();
    return rsthis;
    // return 1;
  }
}

pub trait QDirIterator_QDirIterator_1 {
  fn QDirIterator_1(self) -> QDirIterator;
}
// QDirIterator(const QString &, QDirIterator::IteratorFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDirIterator_QDirIterator_1 for (usize,i32) {
  fn QDirIterator_1(self) -> QDirIterator {
    // unsafe{_ZN12QDirIteratorC2ERK7QString6QFlagsINS_12IteratorFlagEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QDirIteratorC2ERK7QString6QFlagsINS_12IteratorFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDirIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdiriterator.h:61
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QDirIterator(const QString &, QDir::Filters, QDirIterator::IteratorFlags)

/*
Constructs a QDirIterator that can iterate over dir's entrylist, using dir's name filters and regular filters. You can pass options via flags to decide how the directory should be iterated.

By default, flags is NoIteratorFlags, which provides the same behavior as in QDir::entryList().

The sorting in dir is ignored.

Note: To list symlinks that point to non existing files, QDir::System must be passed to the flags.

See also hasNext(), next(), and IteratorFlags.
*/
// QDirIterator(const QString &, QDir::Filters, QDirIterator::IteratorFlags) ctx.fn_proto_cpp
impl /*struct*/ QDirIterator {
  pub fn QDirIterator_2<T: QDirIterator_QDirIterator_2>(value: T) -> QDirIterator {
    let rsthis = value.QDirIterator_2();
    return rsthis;
    // return 1;
  }
}

pub trait QDirIterator_QDirIterator_2 {
  fn QDirIterator_2(self) -> QDirIterator;
}
// QDirIterator(const QString &, QDir::Filters, QDirIterator::IteratorFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDirIterator_QDirIterator_2 for (usize,i32,i32) {
  fn QDirIterator_2(self) -> QDirIterator {
    // unsafe{_ZN12QDirIteratorC2ERK7QString6QFlagsIN4QDir6FilterEES3_INS_12IteratorFlagEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QDirIteratorC2ERK7QString6QFlagsIN4QDir6FilterEES3_INS_12IteratorFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDirIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdiriterator.h:64
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QDirIterator(const QString &, const QStringList &, QDir::Filters, QDirIterator::IteratorFlags)

/*
Constructs a QDirIterator that can iterate over dir's entrylist, using dir's name filters and regular filters. You can pass options via flags to decide how the directory should be iterated.

By default, flags is NoIteratorFlags, which provides the same behavior as in QDir::entryList().

The sorting in dir is ignored.

Note: To list symlinks that point to non existing files, QDir::System must be passed to the flags.

See also hasNext(), next(), and IteratorFlags.
*/
// QDirIterator(const QString &, const QStringList &, QDir::Filters, QDirIterator::IteratorFlags) ctx.fn_proto_cpp
impl /*struct*/ QDirIterator {
  pub fn QDirIterator_3<T: QDirIterator_QDirIterator_3>(value: T) -> QDirIterator {
    let rsthis = value.QDirIterator_3();
    return rsthis;
    // return 1;
  }
}

pub trait QDirIterator_QDirIterator_3 {
  fn QDirIterator_3(self) -> QDirIterator;
}
// QDirIterator(const QString &, const QStringList &, QDir::Filters, QDirIterator::IteratorFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDirIterator_QDirIterator_3 for (usize,usize,i32,i32) {
  fn QDirIterator_3(self) -> QDirIterator {
    // unsafe{_ZN12QDirIteratorC2ERK7QStringRK11QStringList6QFlagsIN4QDir6FilterEES6_INS_12IteratorFlagEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QDirIteratorC2ERK7QStringRK11QStringList6QFlagsIN4QDir6FilterEES6_INS_12IteratorFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDirIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdiriterator.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QDirIterator()

/*

*/
pub fn DeleteQDirIterator(this :*mut QDirIterator) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QDirIteratorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qdiriterator.h:71
// index:0
// Public Visibility=Default Availability=Available
// [8] QString next()

/*
Advances the iterator to the next entry, and returns the file path of this new entry. If hasNext() returns false, this function does nothing, and returns an empty QString.

You can call fileName() or filePath() to get the current entry file name or path, or fileInfo() to get a QFileInfo for the current entry.

See also hasNext(), fileName(), filePath(), and fileInfo().
*/
impl /*struct*/ QDirIterator {
  pub fn next_0<RetType, T: QDirIterator_next_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.next_0(self);
    // return 1;
  }
}
pub trait QDirIterator_next_0<RetType> {
  fn next_0(self , rsthis: & QDirIterator) -> RetType;
}
impl<'a> /*trait*/ QDirIterator_next_0<usize> for () {
  fn next_0(self , rsthis: & QDirIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QDirIterator4nextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdiriterator.h:72
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasNext() const

/*
Returns true if there is at least one more entry in the directory; otherwise, false is returned.

See also next(), fileName(), filePath(), and fileInfo().
*/
impl /*struct*/ QDirIterator {
  pub fn hasNext_0<RetType, T: QDirIterator_hasNext_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasNext_0(self);
    // return 1;
  }
}
pub trait QDirIterator_hasNext_0<RetType> {
  fn hasNext_0(self , rsthis: & QDirIterator) -> RetType;
}
impl<'a> /*trait*/ QDirIterator_hasNext_0<bool> for () {
  fn hasNext_0(self , rsthis: & QDirIterator) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QDirIterator7hasNextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdiriterator.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fileName() const

/*
Returns the file name for the current directory entry, without the path prepended.

This function is convenient when iterating a single directory. When using the QDirIterator::Subdirectories flag, you can use filePath() to get the full path.

See also filePath() and fileInfo().
*/
impl /*struct*/ QDirIterator {
  pub fn fileName_0<RetType, T: QDirIterator_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QDirIterator_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QDirIterator) -> RetType;
}
impl<'a> /*trait*/ QDirIterator_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QDirIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QDirIterator8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdiriterator.h:75
// index:0
// Public Visibility=Default Availability=Available
// [8] QString filePath() const

/*
Returns the full file path for the current directory entry.

See also fileInfo() and fileName().
*/
impl /*struct*/ QDirIterator {
  pub fn filePath_0<RetType, T: QDirIterator_filePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filePath_0(self);
    // return 1;
  }
}
pub trait QDirIterator_filePath_0<RetType> {
  fn filePath_0(self , rsthis: & QDirIterator) -> RetType;
}
impl<'a> /*trait*/ QDirIterator_filePath_0<usize> for () {
  fn filePath_0(self , rsthis: & QDirIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QDirIterator8filePathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdiriterator.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] QFileInfo fileInfo() const

/*
Returns a QFileInfo for the current directory entry.

See also filePath() and fileName().
*/
impl /*struct*/ QDirIterator {
  pub fn fileInfo_0<RetType, T: QDirIterator_fileInfo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileInfo_0(self);
    // return 1;
  }
}
pub trait QDirIterator_fileInfo_0<RetType> {
  fn fileInfo_0(self , rsthis: & QDirIterator) -> RetType;
}
impl<'a> /*trait*/ QDirIterator_fileInfo_0<usize> for () {
  fn fileInfo_0(self , rsthis: & QDirIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QDirIterator8fileInfoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdiriterator.h:77
// index:0
// Public Visibility=Default Availability=Available
// [8] QString path() const

/*
Returns the base directory of the iterator.
*/
impl /*struct*/ QDirIterator {
  pub fn path_0<RetType, T: QDirIterator_path_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.path_0(self);
    // return 1;
  }
}
pub trait QDirIterator_path_0<RetType> {
  fn path_0(self , rsthis: & QDirIterator) -> RetType;
}
impl<'a> /*trait*/ QDirIterator_path_0<usize> for () {
  fn path_0(self , rsthis: & QDirIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QDirIterator4pathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*


*/
pub type QDirIterator__IteratorFlag = i32;
// 
pub const QDirIterator__NoIteratorFlags :QDirIterator__IteratorFlag = 0;
// 
pub const QDirIterator__FollowSymlinks :QDirIterator__IteratorFlag = 1;
// 
pub const QDirIterator__Subdirectories :QDirIterator__IteratorFlag = 2;
pub fn QDirIterator_IteratorFlagItemName(val: i32) ->String {
  match val {
     QDirIterator__NoIteratorFlags => // 0
     {return String::from("NoIteratorFlags");}
     QDirIterator__FollowSymlinks => // 1
     {return String::from("FollowSymlinks");}
     QDirIterator__Subdirectories => // 2
     {return String::from("Subdirectories");}
  _ => {return format!("{}", val);}
}
}
pub fn QDirIterator_IteratorFlagItemName_s(val: i32) ->String {
  //var nilthis *QDirIterator
  //return nilthis.IteratorFlagItemName(val);
  return QDirIterator_IteratorFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
