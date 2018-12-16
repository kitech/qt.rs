

// mod ::gui::QPageSize
// package qtgui
// /usr/include/qt/QtGui/qpagesize.h
// #include <qpagesize.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QPageSize)=8
pub struct QPageSize {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPageSize_ITF interface {
//    QPageSize_PTR() *QPageSize
//}
//func (ptr *QPageSize) QPageSize_PTR() *QPageSize { return ptr }

impl /*struct*/ QPageSize {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPageSize {
    return QPageSize{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPageSize {
//  type Target = QPageSizeBASE;
//
//  fn deref(&self) -> &QPageSizeBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPageSizeBASE> for QPageSize {
//  fn as_ref(& self) -> & QPageSizeBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpagesize.h:230
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPageSize()

/*
Creates a null QPageSize.
*/
// QPageSize() ctx.fn_proto_cpp
impl /*struct*/ QPageSize {
  pub fn QPageSize_0<T: QPageSize_QPageSize_0>(value: T) -> QPageSize {
    let rsthis = value.QPageSize_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPageSize_QPageSize_0 {
  fn QPageSize_0(self) -> QPageSize;
}
// QPageSize() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPageSize_QPageSize_0 for () {
  fn QPageSize_0(self) -> QPageSize {
    // unsafe{_ZN9QPageSizeC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QPageSizeC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPageSize{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:231
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPageSize(QPageSize::PageSizeId)

/*
Creates a null QPageSize.
*/
// QPageSize(QPageSize::PageSizeId) ctx.fn_proto_cpp
impl /*struct*/ QPageSize {
  pub fn QPageSize_1<T: QPageSize_QPageSize_1>(value: T) -> QPageSize {
    let rsthis = value.QPageSize_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPageSize_QPageSize_1 {
  fn QPageSize_1(self) -> QPageSize;
}
// QPageSize(QPageSize::PageSizeId) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPageSize_QPageSize_1 for (i32) {
  fn QPageSize_1(self) -> QPageSize {
    // unsafe{_ZN9QPageSizeC2ENS_10PageSizeIdE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QPageSizeC2ENS_10PageSizeIdE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPageSize{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:232
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QPageSize(const QSize &, const QString &, QPageSize::SizeMatchPolicy)

/*
Creates a null QPageSize.
*/
// QPageSize(const QSize &, const QString &, QPageSize::SizeMatchPolicy) ctx.fn_proto_cpp
impl /*struct*/ QPageSize {
  pub fn QPageSize_2<T: QPageSize_QPageSize_2>(value: T) -> QPageSize {
    let rsthis = value.QPageSize_2();
    return rsthis;
    // return 1;
  }
}

pub trait QPageSize_QPageSize_2 {
  fn QPageSize_2(self) -> QPageSize;
}
// QPageSize(const QSize &, const QString &, QPageSize::SizeMatchPolicy) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPageSize_QPageSize_2 for (usize,usize,i32) {
  fn QPageSize_2(self) -> QPageSize {
    // unsafe{_ZN9QPageSizeC2ERK5QSizeRK7QStringNS_15SizeMatchPolicyE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QPageSizeC2ERK5QSizeRK7QStringNS_15SizeMatchPolicyE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPageSize{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:235
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QPageSize(const QSizeF &, QPageSize::Unit, const QString &, QPageSize::SizeMatchPolicy)

/*
Creates a null QPageSize.
*/
// QPageSize(const QSizeF &, QPageSize::Unit, const QString &, QPageSize::SizeMatchPolicy) ctx.fn_proto_cpp
impl /*struct*/ QPageSize {
  pub fn QPageSize_3<T: QPageSize_QPageSize_3>(value: T) -> QPageSize {
    let rsthis = value.QPageSize_3();
    return rsthis;
    // return 1;
  }
}

pub trait QPageSize_QPageSize_3 {
  fn QPageSize_3(self) -> QPageSize;
}
// QPageSize(const QSizeF &, QPageSize::Unit, const QString &, QPageSize::SizeMatchPolicy) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPageSize_QPageSize_3 for (usize,i32,usize,i32) {
  fn QPageSize_3(self) -> QPageSize {
    // unsafe{_ZN9QPageSizeC2ERK6QSizeFNS_4UnitERK7QStringNS_15SizeMatchPolicyE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QPageSizeC2ERK6QSizeFNS_4UnitERK7QStringNS_15SizeMatchPolicyE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPageSize{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:240
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPageSize & operator=(QPageSize &&)

/*

*/
impl /*struct*/ QPageSize {
  pub fn operator_equal_0<RetType, T: QPageSize_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QPageSize_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QPageSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPageSizeaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:242
// index:1
// Public Visibility=Default Availability=Available
// [8] QPageSize & operator=(const QPageSize &)

/*

*/
impl /*struct*/ QPageSize {
  pub fn operator_equal_1<RetType, T: QPageSize_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QPageSize_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QPageSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPageSizeaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:243
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QPageSize()

/*

*/
pub fn DeleteQPageSize(this :*mut QPageSize) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QPageSizeD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpagesize.h:246
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QPageSize &)

/*
Swaps this QPageSize with other. This function is very fast and never fails.
*/
impl /*struct*/ QPageSize {
  pub fn swap_0<RetType, T: QPageSize_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QPageSize_swap_0<RetType> {
  fn swap_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QPageSize) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QPageSize4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:249
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEquivalentTo(const QPageSize &) const

/*
Returns true if this page is equivalent to the other page, i.e. if the page has the same size regardless of other attributes like name.
*/
impl /*struct*/ QPageSize {
  pub fn isEquivalentTo_0<RetType, T: QPageSize_isEquivalentTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEquivalentTo_0(self);
    // return 1;
  }
}
pub trait QPageSize_isEquivalentTo_0<RetType> {
  fn isEquivalentTo_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_isEquivalentTo_0<bool> for (usize) {
  fn isEquivalentTo_0(self , rsthis: & QPageSize) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize14isEquivalentToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:251
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if this page size is valid.

The page size may be invalid if created with an invalid PageSizeId, or a negative or invalid QSize or QSizeF, or the null constructor.
*/
impl /*struct*/ QPageSize {
  pub fn isValid_0<RetType, T: QPageSize_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QPageSize_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QPageSize) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:253
// index:0
// Public Visibility=Default Availability=Available
// [8] QString key() const

/*
Returns the unique key of the page size.

By default this is the PPD standard mediaOption keyword for the page size, or the PPD custom format key. If the QPageSize instance was obtained from a print device then this will be the key provided by the print device and may differ from the standard key.

If the QPageSize is invalid then the key will be an empty string.

This key should never be shown to end users, it is an internal key only. For a human-readable name use name().

See also name().
*/
impl /*struct*/ QPageSize {
  pub fn key_0<RetType, T: QPageSize_key_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.key_0(self);
    // return 1;
  }
}
pub trait QPageSize_key_0<RetType> {
  fn key_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_key_0<usize> for () {
  fn key_0(self , rsthis: & QPageSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize3keyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:271
// index:1
// Public static Visibility=Default Availability=Available
// [8] QString key(QPageSize::PageSizeId)

/*
Returns the unique key of the page size.

By default this is the PPD standard mediaOption keyword for the page size, or the PPD custom format key. If the QPageSize instance was obtained from a print device then this will be the key provided by the print device and may differ from the standard key.

If the QPageSize is invalid then the key will be an empty string.

This key should never be shown to end users, it is an internal key only. For a human-readable name use name().

See also name().
*/
impl /*struct*/ QPageSize {
  pub fn key_1<RetType, T: QPageSize_key_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.key_1();
    // return 1;
  }
}
pub trait QPageSize_key_1<RetType> {
  fn key_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPageSize_key_1<usize> for (i32) {
  fn key_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPageSize3keyENS_10PageSizeIdE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:254
// index:0
// Public Visibility=Default Availability=Available
// [8] QString name() const

/*
Returns a localized human-readable name for the page size.

If the QPageSize instance was obtained from a print device then the name used is that provided by the print device. Note that a print device may not support the current default locale language.

If the QPageSize is invalid then the name will be an empty string.
*/
impl /*struct*/ QPageSize {
  pub fn name_0<RetType, T: QPageSize_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QPageSize_name_0<RetType> {
  fn name_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_name_0<usize> for () {
  fn name_0(self , rsthis: & QPageSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:272
// index:1
// Public static Visibility=Default Availability=Available
// [8] QString name(QPageSize::PageSizeId)

/*
Returns a localized human-readable name for the page size.

If the QPageSize instance was obtained from a print device then the name used is that provided by the print device. Note that a print device may not support the current default locale language.

If the QPageSize is invalid then the name will be an empty string.
*/
impl /*struct*/ QPageSize {
  pub fn name_1<RetType, T: QPageSize_name_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.name_1();
    // return 1;
  }
}
pub trait QPageSize_name_1<RetType> {
  fn name_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPageSize_name_1<usize> for (i32) {
  fn name_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPageSize4nameENS_10PageSizeIdE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:256
// index:0
// Public Visibility=Default Availability=Available
// [4] QPageSize::PageSizeId id() const

/*
Returns the standard QPageSize::PageSizeId of the page, or QPageSize::Custom.

If the QPageSize is invalid then the ID will be QPageSize::Custom.
*/
impl /*struct*/ QPageSize {
  pub fn id_0<RetType, T: QPageSize_id_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.id_0(self);
    // return 1;
  }
}
pub trait QPageSize_id_0<RetType> {
  fn id_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_id_0<i32> for () {
  fn id_0(self , rsthis: & QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize2idEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:274
// index:1
// Public static Visibility=Default Availability=Available
// [4] QPageSize::PageSizeId id(const QSize &, QPageSize::SizeMatchPolicy)

/*
Returns the standard QPageSize::PageSizeId of the page, or QPageSize::Custom.

If the QPageSize is invalid then the ID will be QPageSize::Custom.
*/
impl /*struct*/ QPageSize {
  pub fn id_1<RetType, T: QPageSize_id_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.id_1();
    // return 1;
  }
}
pub trait QPageSize_id_1<RetType> {
  fn id_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPageSize_id_1<i32> for (usize,i32) {
  fn id_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPageSize2idERK5QSizeNS_15SizeMatchPolicyE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:276
// index:2
// Public static Visibility=Default Availability=Available
// [4] QPageSize::PageSizeId id(const QSizeF &, QPageSize::Unit, QPageSize::SizeMatchPolicy)

/*
Returns the standard QPageSize::PageSizeId of the page, or QPageSize::Custom.

If the QPageSize is invalid then the ID will be QPageSize::Custom.
*/
impl /*struct*/ QPageSize {
  pub fn id_2<RetType, T: QPageSize_id_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.id_2();
    // return 1;
  }
}
pub trait QPageSize_id_2<RetType> {
  fn id_2(self ) -> RetType;
}
impl<'a> /*trait*/ QPageSize_id_2<i32> for (usize,i32,i32) {
  fn id_2(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPageSize2idERK6QSizeFNS_4UnitENS_15SizeMatchPolicyE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:279
// index:3
// Public static Visibility=Default Availability=Available
// [4] QPageSize::PageSizeId id(int)

/*
Returns the standard QPageSize::PageSizeId of the page, or QPageSize::Custom.

If the QPageSize is invalid then the ID will be QPageSize::Custom.
*/
impl /*struct*/ QPageSize {
  pub fn id_3<RetType, T: QPageSize_id_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.id_3();
    // return 1;
  }
}
pub trait QPageSize_id_3<RetType> {
  fn id_3(self ) -> RetType;
}
impl<'a> /*trait*/ QPageSize_id_3<i32> for (i32) {
  fn id_3(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPageSize2idEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:258
// index:0
// Public Visibility=Default Availability=Available
// [4] int windowsId() const

/*
Returns the Windows DMPAPER enum value for the page size.

Not all valid PPD page sizes have a Windows equivalent, in which case 0 will be returned.

If the QPageSize is invalid then the Windows ID will be 0.

See also id().
*/
impl /*struct*/ QPageSize {
  pub fn windowsId_0<RetType, T: QPageSize_windowsId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowsId_0(self);
    // return 1;
  }
}
pub trait QPageSize_windowsId_0<RetType> {
  fn windowsId_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_windowsId_0<i32> for () {
  fn windowsId_0(self , rsthis: & QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize9windowsIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:280
// index:1
// Public static Visibility=Default Availability=Available
// [4] int windowsId(QPageSize::PageSizeId)

/*
Returns the Windows DMPAPER enum value for the page size.

Not all valid PPD page sizes have a Windows equivalent, in which case 0 will be returned.

If the QPageSize is invalid then the Windows ID will be 0.

See also id().
*/
impl /*struct*/ QPageSize {
  pub fn windowsId_1<RetType, T: QPageSize_windowsId_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.windowsId_1();
    // return 1;
  }
}
pub trait QPageSize_windowsId_1<RetType> {
  fn windowsId_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPageSize_windowsId_1<i32> for (i32) {
  fn windowsId_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPageSize9windowsIdENS_10PageSizeIdE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:260
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF definitionSize() const

/*
Returns the definition size of the page size.

For a standard page size this will be the size as defined in the relevant standard, i.e. ISO A4 will be defined in millimeters while ANSI Letter will be defined in inches.

For a custom page size this will be the original size used to create the page size object.

If the QPageSize is invalid then the QSizeF will be invalid.

See also definitionUnits().
*/
impl /*struct*/ QPageSize {
  pub fn definitionSize_0<RetType, T: QPageSize_definitionSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.definitionSize_0(self);
    // return 1;
  }
}
pub trait QPageSize_definitionSize_0<RetType> {
  fn definitionSize_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_definitionSize_0<usize> for () {
  fn definitionSize_0(self , rsthis: & QPageSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize14definitionSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:282
// index:1
// Public static Visibility=Default Availability=Available
// [16] QSizeF definitionSize(QPageSize::PageSizeId)

/*
Returns the definition size of the page size.

For a standard page size this will be the size as defined in the relevant standard, i.e. ISO A4 will be defined in millimeters while ANSI Letter will be defined in inches.

For a custom page size this will be the original size used to create the page size object.

If the QPageSize is invalid then the QSizeF will be invalid.

See also definitionUnits().
*/
impl /*struct*/ QPageSize {
  pub fn definitionSize_1<RetType, T: QPageSize_definitionSize_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.definitionSize_1();
    // return 1;
  }
}
pub trait QPageSize_definitionSize_1<RetType> {
  fn definitionSize_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPageSize_definitionSize_1<usize> for (i32) {
  fn definitionSize_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPageSize14definitionSizeENS_10PageSizeIdE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:261
// index:0
// Public Visibility=Default Availability=Available
// [4] QPageSize::Unit definitionUnits() const

/*
Returns the definition units of the page size.

For a standard page size this will be the units as defined in the relevant standard, i.e. ISO A4 will be defined in millimeters while ANSI Letter will be defined in inches.

For a custom page size this will be the original units used to create the page size object.

If the QPageSize is invalid then the QPageSize::Unit will be invalid.

See also definitionSize().
*/
impl /*struct*/ QPageSize {
  pub fn definitionUnits_0<RetType, T: QPageSize_definitionUnits_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.definitionUnits_0(self);
    // return 1;
  }
}
pub trait QPageSize_definitionUnits_0<RetType> {
  fn definitionUnits_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_definitionUnits_0<i32> for () {
  fn definitionUnits_0(self , rsthis: & QPageSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize15definitionUnitsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:283
// index:1
// Public static Visibility=Default Availability=Available
// [4] QPageSize::Unit definitionUnits(QPageSize::PageSizeId)

/*
Returns the definition units of the page size.

For a standard page size this will be the units as defined in the relevant standard, i.e. ISO A4 will be defined in millimeters while ANSI Letter will be defined in inches.

For a custom page size this will be the original units used to create the page size object.

If the QPageSize is invalid then the QPageSize::Unit will be invalid.

See also definitionSize().
*/
impl /*struct*/ QPageSize {
  pub fn definitionUnits_1<RetType, T: QPageSize_definitionUnits_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.definitionUnits_1();
    // return 1;
  }
}
pub trait QPageSize_definitionUnits_1<RetType> {
  fn definitionUnits_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPageSize_definitionUnits_1<i32> for (i32) {
  fn definitionUnits_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPageSize15definitionUnitsENS_10PageSizeIdE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:263
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF size(QPageSize::Unit) const

/*
Returns the size of the page in the required units.

If the QPageSize is invalid then the QSizeF will be invalid.
*/
impl /*struct*/ QPageSize {
  pub fn size_0<RetType, T: QPageSize_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QPageSize_size_0<RetType> {
  fn size_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_size_0<usize> for (i32) {
  fn size_0(self , rsthis: & QPageSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize4sizeENS_4UnitE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:285
// index:1
// Public static Visibility=Default Availability=Available
// [16] QSizeF size(QPageSize::PageSizeId, QPageSize::Unit)

/*
Returns the size of the page in the required units.

If the QPageSize is invalid then the QSizeF will be invalid.
*/
impl /*struct*/ QPageSize {
  pub fn size_1<RetType, T: QPageSize_size_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.size_1();
    // return 1;
  }
}
pub trait QPageSize_size_1<RetType> {
  fn size_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPageSize_size_1<usize> for (i32,i32) {
  fn size_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPageSize4sizeENS_10PageSizeIdENS_4UnitE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:264
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize sizePoints() const

/*
Returns the size of the page in Postscript Points (1/72 of an inch).

If the QPageSize is invalid then the QSize will be invalid.
*/
impl /*struct*/ QPageSize {
  pub fn sizePoints_0<RetType, T: QPageSize_sizePoints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizePoints_0(self);
    // return 1;
  }
}
pub trait QPageSize_sizePoints_0<RetType> {
  fn sizePoints_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_sizePoints_0<usize> for () {
  fn sizePoints_0(self , rsthis: & QPageSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize10sizePointsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:286
// index:1
// Public static Visibility=Default Availability=Available
// [8] QSize sizePoints(QPageSize::PageSizeId)

/*
Returns the size of the page in Postscript Points (1/72 of an inch).

If the QPageSize is invalid then the QSize will be invalid.
*/
impl /*struct*/ QPageSize {
  pub fn sizePoints_1<RetType, T: QPageSize_sizePoints_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.sizePoints_1();
    // return 1;
  }
}
pub trait QPageSize_sizePoints_1<RetType> {
  fn sizePoints_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPageSize_sizePoints_1<usize> for (i32) {
  fn sizePoints_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPageSize10sizePointsENS_10PageSizeIdE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:265
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize sizePixels(int) const

/*
Returns the size of the page in Device Pixels at the given resolution.

If the QPageSize is invalid then the QSize will be invalid.
*/
impl /*struct*/ QPageSize {
  pub fn sizePixels_0<RetType, T: QPageSize_sizePixels_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizePixels_0(self);
    // return 1;
  }
}
pub trait QPageSize_sizePixels_0<RetType> {
  fn sizePixels_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_sizePixels_0<usize> for (i32) {
  fn sizePixels_0(self , rsthis: & QPageSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize10sizePixelsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:287
// index:1
// Public static Visibility=Default Availability=Available
// [8] QSize sizePixels(QPageSize::PageSizeId, int)

/*
Returns the size of the page in Device Pixels at the given resolution.

If the QPageSize is invalid then the QSize will be invalid.
*/
impl /*struct*/ QPageSize {
  pub fn sizePixels_1<RetType, T: QPageSize_sizePixels_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.sizePixels_1();
    // return 1;
  }
}
pub trait QPageSize_sizePixels_1<RetType> {
  fn sizePixels_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPageSize_sizePixels_1<usize> for (i32,i32) {
  fn sizePixels_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPageSize10sizePixelsENS_10PageSizeIdEi", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:267
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF rect(QPageSize::Unit) const

/*
Returns the page rectangle in the required units.

If the QPageSize is invalid then the QRect will be invalid.
*/
impl /*struct*/ QPageSize {
  pub fn rect_0<RetType, T: QPageSize_rect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rect_0(self);
    // return 1;
  }
}
pub trait QPageSize_rect_0<RetType> {
  fn rect_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_rect_0<usize> for (i32) {
  fn rect_0(self , rsthis: & QPageSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize4rectENS_4UnitE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:268
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect rectPoints() const

/*
Returns the page rectangle in Postscript Points (1/72 of an inch).

If the QPageSize is invalid then the QRect will be invalid.
*/
impl /*struct*/ QPageSize {
  pub fn rectPoints_0<RetType, T: QPageSize_rectPoints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rectPoints_0(self);
    // return 1;
  }
}
pub trait QPageSize_rectPoints_0<RetType> {
  fn rectPoints_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_rectPoints_0<usize> for () {
  fn rectPoints_0(self , rsthis: & QPageSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize10rectPointsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagesize.h:269
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect rectPixels(int) const

/*
Returns the page rectangle in Device Pixels at the given resolution.

If the QPageSize is invalid then the QRect will be invalid.
*/
impl /*struct*/ QPageSize {
  pub fn rectPixels_0<RetType, T: QPageSize_rectPixels_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rectPixels_0(self);
    // return 1;
  }
}
pub trait QPageSize_rectPixels_0<RetType> {
  fn rectPixels_0(self , rsthis: & QPageSize) -> RetType;
}
impl<'a> /*trait*/ QPageSize_rectPixels_0<usize> for (i32) {
  fn rectPixels_0(self , rsthis: & QPageSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPageSize10rectPixelsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum type lists the available page sizes as defined in the Postscript PPD standard. These values are duplicated in QPagedPaintDevice and QPrinter.

The defined sizes are:

QPageSize::AnsiALetter= Letter
QPageSize::AnsiBLedger= Ledger
QPageSize::EnvelopeDLDLE= DLE


Due to historic reasons QPageSize::Executive is not the same as the standard Postscript and Windows Executive size, use QPageSize::ExecutiveStandard instead.

The Postscript standard size QPageSize::Folio is different to the Windows DMPAPER_FOLIO size, use the Postscript standard size QPageSize::FanFoldGermanLegal if needed.

*/
pub type QPageSize__PageSizeId = i32;
// 
pub const QPageSize__A4 :QPageSize__PageSizeId = 0;
// 
pub const QPageSize__B5 :QPageSize__PageSizeId = 1;
// 
pub const QPageSize__Letter :QPageSize__PageSizeId = 2;
// 
pub const QPageSize__Legal :QPageSize__PageSizeId = 3;
// 
pub const QPageSize__Executive :QPageSize__PageSizeId = 4;
// 
pub const QPageSize__A0 :QPageSize__PageSizeId = 5;
// 
pub const QPageSize__A1 :QPageSize__PageSizeId = 6;
// 
pub const QPageSize__A2 :QPageSize__PageSizeId = 7;
// 
pub const QPageSize__A3 :QPageSize__PageSizeId = 8;
// 
pub const QPageSize__A5 :QPageSize__PageSizeId = 9;
// 
pub const QPageSize__A6 :QPageSize__PageSizeId = 10;
// 
pub const QPageSize__A7 :QPageSize__PageSizeId = 11;
// 
pub const QPageSize__A8 :QPageSize__PageSizeId = 12;
// 
pub const QPageSize__A9 :QPageSize__PageSizeId = 13;
// 
pub const QPageSize__B0 :QPageSize__PageSizeId = 14;
// 
pub const QPageSize__B1 :QPageSize__PageSizeId = 15;
// 
pub const QPageSize__B10 :QPageSize__PageSizeId = 16;
// 
pub const QPageSize__B2 :QPageSize__PageSizeId = 17;
// 
pub const QPageSize__B3 :QPageSize__PageSizeId = 18;
// 
pub const QPageSize__B4 :QPageSize__PageSizeId = 19;
// 
pub const QPageSize__B6 :QPageSize__PageSizeId = 20;
// 
pub const QPageSize__B7 :QPageSize__PageSizeId = 21;
// 
pub const QPageSize__B8 :QPageSize__PageSizeId = 22;
// 
pub const QPageSize__B9 :QPageSize__PageSizeId = 23;
// 
pub const QPageSize__C5E :QPageSize__PageSizeId = 24;
// 
pub const QPageSize__Comm10E :QPageSize__PageSizeId = 25;
// 
pub const QPageSize__DLE :QPageSize__PageSizeId = 26;
// 
pub const QPageSize__Folio :QPageSize__PageSizeId = 27;
// 
pub const QPageSize__Ledger :QPageSize__PageSizeId = 28;
// 
pub const QPageSize__Tabloid :QPageSize__PageSizeId = 29;
// 
pub const QPageSize__Custom :QPageSize__PageSizeId = 30;
// 
pub const QPageSize__A10 :QPageSize__PageSizeId = 31;
// 
pub const QPageSize__A3Extra :QPageSize__PageSizeId = 32;
// 
pub const QPageSize__A4Extra :QPageSize__PageSizeId = 33;
// 
pub const QPageSize__A4Plus :QPageSize__PageSizeId = 34;
// 
pub const QPageSize__A4Small :QPageSize__PageSizeId = 35;
// 
pub const QPageSize__A5Extra :QPageSize__PageSizeId = 36;
// 
pub const QPageSize__B5Extra :QPageSize__PageSizeId = 37;
// 
pub const QPageSize__JisB0 :QPageSize__PageSizeId = 38;
// 
pub const QPageSize__JisB1 :QPageSize__PageSizeId = 39;
// 
pub const QPageSize__JisB2 :QPageSize__PageSizeId = 40;
// 
pub const QPageSize__JisB3 :QPageSize__PageSizeId = 41;
// 
pub const QPageSize__JisB4 :QPageSize__PageSizeId = 42;
// 
pub const QPageSize__JisB5 :QPageSize__PageSizeId = 43;
// 
pub const QPageSize__JisB6 :QPageSize__PageSizeId = 44;
// 
pub const QPageSize__JisB7 :QPageSize__PageSizeId = 45;
// 
pub const QPageSize__JisB8 :QPageSize__PageSizeId = 46;
// 
pub const QPageSize__JisB9 :QPageSize__PageSizeId = 47;
// 
pub const QPageSize__JisB10 :QPageSize__PageSizeId = 48;
// 
pub const QPageSize__AnsiC :QPageSize__PageSizeId = 49;
// 
pub const QPageSize__AnsiD :QPageSize__PageSizeId = 50;
// 
pub const QPageSize__AnsiE :QPageSize__PageSizeId = 51;
// 
pub const QPageSize__LegalExtra :QPageSize__PageSizeId = 52;
// 
pub const QPageSize__LetterExtra :QPageSize__PageSizeId = 53;
// 
pub const QPageSize__LetterPlus :QPageSize__PageSizeId = 54;
// 
pub const QPageSize__LetterSmall :QPageSize__PageSizeId = 55;
// 
pub const QPageSize__TabloidExtra :QPageSize__PageSizeId = 56;
// 
pub const QPageSize__ArchA :QPageSize__PageSizeId = 57;
// 
pub const QPageSize__ArchB :QPageSize__PageSizeId = 58;
// 
pub const QPageSize__ArchC :QPageSize__PageSizeId = 59;
// 
pub const QPageSize__ArchD :QPageSize__PageSizeId = 60;
// 
pub const QPageSize__ArchE :QPageSize__PageSizeId = 61;
// 
pub const QPageSize__Imperial7x9 :QPageSize__PageSizeId = 62;
// 
pub const QPageSize__Imperial8x10 :QPageSize__PageSizeId = 63;
// 
pub const QPageSize__Imperial9x11 :QPageSize__PageSizeId = 64;
// 
pub const QPageSize__Imperial9x12 :QPageSize__PageSizeId = 65;
// 
pub const QPageSize__Imperial10x11 :QPageSize__PageSizeId = 66;
// 
pub const QPageSize__Imperial10x13 :QPageSize__PageSizeId = 67;
// 
pub const QPageSize__Imperial10x14 :QPageSize__PageSizeId = 68;
// 
pub const QPageSize__Imperial12x11 :QPageSize__PageSizeId = 69;
// 
pub const QPageSize__Imperial15x11 :QPageSize__PageSizeId = 70;
// 
pub const QPageSize__ExecutiveStandard :QPageSize__PageSizeId = 71;
// 
pub const QPageSize__Note :QPageSize__PageSizeId = 72;
// 
pub const QPageSize__Quarto :QPageSize__PageSizeId = 73;
// 
pub const QPageSize__Statement :QPageSize__PageSizeId = 74;
// 
pub const QPageSize__SuperA :QPageSize__PageSizeId = 75;
// 
pub const QPageSize__SuperB :QPageSize__PageSizeId = 76;
// 
pub const QPageSize__Postcard :QPageSize__PageSizeId = 77;
// 
pub const QPageSize__DoublePostcard :QPageSize__PageSizeId = 78;
// 
pub const QPageSize__Prc16K :QPageSize__PageSizeId = 79;
// 
pub const QPageSize__Prc32K :QPageSize__PageSizeId = 80;
// 
pub const QPageSize__Prc32KBig :QPageSize__PageSizeId = 81;
// 
pub const QPageSize__FanFoldUS :QPageSize__PageSizeId = 82;
// 
pub const QPageSize__FanFoldGerman :QPageSize__PageSizeId = 83;
// 
pub const QPageSize__FanFoldGermanLegal :QPageSize__PageSizeId = 84;
// 
pub const QPageSize__EnvelopeB4 :QPageSize__PageSizeId = 85;
// 
pub const QPageSize__EnvelopeB5 :QPageSize__PageSizeId = 86;
// 
pub const QPageSize__EnvelopeB6 :QPageSize__PageSizeId = 87;
// 
pub const QPageSize__EnvelopeC0 :QPageSize__PageSizeId = 88;
// 
pub const QPageSize__EnvelopeC1 :QPageSize__PageSizeId = 89;
// 
pub const QPageSize__EnvelopeC2 :QPageSize__PageSizeId = 90;
// 
pub const QPageSize__EnvelopeC3 :QPageSize__PageSizeId = 91;
// 
pub const QPageSize__EnvelopeC4 :QPageSize__PageSizeId = 92;
// 
pub const QPageSize__EnvelopeC6 :QPageSize__PageSizeId = 93;
// 
pub const QPageSize__EnvelopeC65 :QPageSize__PageSizeId = 94;
// 
pub const QPageSize__EnvelopeC7 :QPageSize__PageSizeId = 95;
// 
pub const QPageSize__Envelope9 :QPageSize__PageSizeId = 96;
// 
pub const QPageSize__Envelope11 :QPageSize__PageSizeId = 97;
// 
pub const QPageSize__Envelope12 :QPageSize__PageSizeId = 98;
// 
pub const QPageSize__Envelope14 :QPageSize__PageSizeId = 99;
// 
pub const QPageSize__EnvelopeMonarch :QPageSize__PageSizeId = 100;
// 
pub const QPageSize__EnvelopePersonal :QPageSize__PageSizeId = 101;
// 
pub const QPageSize__EnvelopeChou3 :QPageSize__PageSizeId = 102;
// 
pub const QPageSize__EnvelopeChou4 :QPageSize__PageSizeId = 103;
// 
pub const QPageSize__EnvelopeInvite :QPageSize__PageSizeId = 104;
// 
pub const QPageSize__EnvelopeItalian :QPageSize__PageSizeId = 105;
// 
pub const QPageSize__EnvelopeKaku2 :QPageSize__PageSizeId = 106;
// 
pub const QPageSize__EnvelopeKaku3 :QPageSize__PageSizeId = 107;
// 
pub const QPageSize__EnvelopePrc1 :QPageSize__PageSizeId = 108;
// 
pub const QPageSize__EnvelopePrc2 :QPageSize__PageSizeId = 109;
// 
pub const QPageSize__EnvelopePrc3 :QPageSize__PageSizeId = 110;
// 
pub const QPageSize__EnvelopePrc4 :QPageSize__PageSizeId = 111;
// 
pub const QPageSize__EnvelopePrc5 :QPageSize__PageSizeId = 112;
// 
pub const QPageSize__EnvelopePrc6 :QPageSize__PageSizeId = 113;
// 
pub const QPageSize__EnvelopePrc7 :QPageSize__PageSizeId = 114;
// 
pub const QPageSize__EnvelopePrc8 :QPageSize__PageSizeId = 115;
// 
pub const QPageSize__EnvelopePrc9 :QPageSize__PageSizeId = 116;
// 
pub const QPageSize__EnvelopePrc10 :QPageSize__PageSizeId = 117;
// 
pub const QPageSize__EnvelopeYou4 :QPageSize__PageSizeId = 118;
// 
pub const QPageSize__LastPageSize :QPageSize__PageSizeId = 118;
// 
pub const QPageSize__NPageSize :QPageSize__PageSizeId = 118;
// 
pub const QPageSize__NPaperSize :QPageSize__PageSizeId = 118;
// 
pub const QPageSize__AnsiA :QPageSize__PageSizeId = 2;
// 
pub const QPageSize__AnsiB :QPageSize__PageSizeId = 28;
// 
pub const QPageSize__EnvelopeC5 :QPageSize__PageSizeId = 24;
// 
pub const QPageSize__EnvelopeDL :QPageSize__PageSizeId = 26;
// 
pub const QPageSize__Envelope10 :QPageSize__PageSizeId = 25;
pub fn QPageSize_PageSizeIdItemName(val: i32) ->String {
  match val {
     QPageSize__A4 => // 0
     {return String::from("A4");}
     QPageSize__B5 => // 1
     {return String::from("B5");}
     QPageSize__Letter => // 2
     {return String::from("Letter,AnsiA");}
     QPageSize__Legal => // 3
     {return String::from("Legal");}
     QPageSize__Executive => // 4
     {return String::from("Executive");}
     QPageSize__A0 => // 5
     {return String::from("A0");}
     QPageSize__A1 => // 6
     {return String::from("A1");}
     QPageSize__A2 => // 7
     {return String::from("A2");}
     QPageSize__A3 => // 8
     {return String::from("A3");}
     QPageSize__A5 => // 9
     {return String::from("A5");}
     QPageSize__A6 => // 10
     {return String::from("A6");}
     QPageSize__A7 => // 11
     {return String::from("A7");}
     QPageSize__A8 => // 12
     {return String::from("A8");}
     QPageSize__A9 => // 13
     {return String::from("A9");}
     QPageSize__B0 => // 14
     {return String::from("B0");}
     QPageSize__B1 => // 15
     {return String::from("B1");}
     QPageSize__B10 => // 16
     {return String::from("B10");}
     QPageSize__B2 => // 17
     {return String::from("B2");}
     QPageSize__B3 => // 18
     {return String::from("B3");}
     QPageSize__B4 => // 19
     {return String::from("B4");}
     QPageSize__B6 => // 20
     {return String::from("B6");}
     QPageSize__B7 => // 21
     {return String::from("B7");}
     QPageSize__B8 => // 22
     {return String::from("B8");}
     QPageSize__B9 => // 23
     {return String::from("B9");}
     QPageSize__C5E => // 24
     {return String::from("C5E,EnvelopeC5");}
     QPageSize__Comm10E => // 25
     {return String::from("Comm10E,Envelope10");}
     QPageSize__DLE => // 26
     {return String::from("DLE,EnvelopeDL");}
     QPageSize__Folio => // 27
     {return String::from("Folio");}
     QPageSize__Ledger => // 28
     {return String::from("Ledger,AnsiB");}
     QPageSize__Tabloid => // 29
     {return String::from("Tabloid");}
     QPageSize__Custom => // 30
     {return String::from("Custom");}
     QPageSize__A10 => // 31
     {return String::from("A10");}
     QPageSize__A3Extra => // 32
     {return String::from("A3Extra");}
     QPageSize__A4Extra => // 33
     {return String::from("A4Extra");}
     QPageSize__A4Plus => // 34
     {return String::from("A4Plus");}
     QPageSize__A4Small => // 35
     {return String::from("A4Small");}
     QPageSize__A5Extra => // 36
     {return String::from("A5Extra");}
     QPageSize__B5Extra => // 37
     {return String::from("B5Extra");}
     QPageSize__JisB0 => // 38
     {return String::from("JisB0");}
     QPageSize__JisB1 => // 39
     {return String::from("JisB1");}
     QPageSize__JisB2 => // 40
     {return String::from("JisB2");}
     QPageSize__JisB3 => // 41
     {return String::from("JisB3");}
     QPageSize__JisB4 => // 42
     {return String::from("JisB4");}
     QPageSize__JisB5 => // 43
     {return String::from("JisB5");}
     QPageSize__JisB6 => // 44
     {return String::from("JisB6");}
     QPageSize__JisB7 => // 45
     {return String::from("JisB7");}
     QPageSize__JisB8 => // 46
     {return String::from("JisB8");}
     QPageSize__JisB9 => // 47
     {return String::from("JisB9");}
     QPageSize__JisB10 => // 48
     {return String::from("JisB10");}
     QPageSize__AnsiC => // 49
     {return String::from("AnsiC");}
     QPageSize__AnsiD => // 50
     {return String::from("AnsiD");}
     QPageSize__AnsiE => // 51
     {return String::from("AnsiE");}
     QPageSize__LegalExtra => // 52
     {return String::from("LegalExtra");}
     QPageSize__LetterExtra => // 53
     {return String::from("LetterExtra");}
     QPageSize__LetterPlus => // 54
     {return String::from("LetterPlus");}
     QPageSize__LetterSmall => // 55
     {return String::from("LetterSmall");}
     QPageSize__TabloidExtra => // 56
     {return String::from("TabloidExtra");}
     QPageSize__ArchA => // 57
     {return String::from("ArchA");}
     QPageSize__ArchB => // 58
     {return String::from("ArchB");}
     QPageSize__ArchC => // 59
     {return String::from("ArchC");}
     QPageSize__ArchD => // 60
     {return String::from("ArchD");}
     QPageSize__ArchE => // 61
     {return String::from("ArchE");}
     QPageSize__Imperial7x9 => // 62
     {return String::from("Imperial7x9");}
     QPageSize__Imperial8x10 => // 63
     {return String::from("Imperial8x10");}
     QPageSize__Imperial9x11 => // 64
     {return String::from("Imperial9x11");}
     QPageSize__Imperial9x12 => // 65
     {return String::from("Imperial9x12");}
     QPageSize__Imperial10x11 => // 66
     {return String::from("Imperial10x11");}
     QPageSize__Imperial10x13 => // 67
     {return String::from("Imperial10x13");}
     QPageSize__Imperial10x14 => // 68
     {return String::from("Imperial10x14");}
     QPageSize__Imperial12x11 => // 69
     {return String::from("Imperial12x11");}
     QPageSize__Imperial15x11 => // 70
     {return String::from("Imperial15x11");}
     QPageSize__ExecutiveStandard => // 71
     {return String::from("ExecutiveStandard");}
     QPageSize__Note => // 72
     {return String::from("Note");}
     QPageSize__Quarto => // 73
     {return String::from("Quarto");}
     QPageSize__Statement => // 74
     {return String::from("Statement");}
     QPageSize__SuperA => // 75
     {return String::from("SuperA");}
     QPageSize__SuperB => // 76
     {return String::from("SuperB");}
     QPageSize__Postcard => // 77
     {return String::from("Postcard");}
     QPageSize__DoublePostcard => // 78
     {return String::from("DoublePostcard");}
     QPageSize__Prc16K => // 79
     {return String::from("Prc16K");}
     QPageSize__Prc32K => // 80
     {return String::from("Prc32K");}
     QPageSize__Prc32KBig => // 81
     {return String::from("Prc32KBig");}
     QPageSize__FanFoldUS => // 82
     {return String::from("FanFoldUS");}
     QPageSize__FanFoldGerman => // 83
     {return String::from("FanFoldGerman");}
     QPageSize__FanFoldGermanLegal => // 84
     {return String::from("FanFoldGermanLegal");}
     QPageSize__EnvelopeB4 => // 85
     {return String::from("EnvelopeB4");}
     QPageSize__EnvelopeB5 => // 86
     {return String::from("EnvelopeB5");}
     QPageSize__EnvelopeB6 => // 87
     {return String::from("EnvelopeB6");}
     QPageSize__EnvelopeC0 => // 88
     {return String::from("EnvelopeC0");}
     QPageSize__EnvelopeC1 => // 89
     {return String::from("EnvelopeC1");}
     QPageSize__EnvelopeC2 => // 90
     {return String::from("EnvelopeC2");}
     QPageSize__EnvelopeC3 => // 91
     {return String::from("EnvelopeC3");}
     QPageSize__EnvelopeC4 => // 92
     {return String::from("EnvelopeC4");}
     QPageSize__EnvelopeC6 => // 93
     {return String::from("EnvelopeC6");}
     QPageSize__EnvelopeC65 => // 94
     {return String::from("EnvelopeC65");}
     QPageSize__EnvelopeC7 => // 95
     {return String::from("EnvelopeC7");}
     QPageSize__Envelope9 => // 96
     {return String::from("Envelope9");}
     QPageSize__Envelope11 => // 97
     {return String::from("Envelope11");}
     QPageSize__Envelope12 => // 98
     {return String::from("Envelope12");}
     QPageSize__Envelope14 => // 99
     {return String::from("Envelope14");}
     QPageSize__EnvelopeMonarch => // 100
     {return String::from("EnvelopeMonarch");}
     QPageSize__EnvelopePersonal => // 101
     {return String::from("EnvelopePersonal");}
     QPageSize__EnvelopeChou3 => // 102
     {return String::from("EnvelopeChou3");}
     QPageSize__EnvelopeChou4 => // 103
     {return String::from("EnvelopeChou4");}
     QPageSize__EnvelopeInvite => // 104
     {return String::from("EnvelopeInvite");}
     QPageSize__EnvelopeItalian => // 105
     {return String::from("EnvelopeItalian");}
     QPageSize__EnvelopeKaku2 => // 106
     {return String::from("EnvelopeKaku2");}
     QPageSize__EnvelopeKaku3 => // 107
     {return String::from("EnvelopeKaku3");}
     QPageSize__EnvelopePrc1 => // 108
     {return String::from("EnvelopePrc1");}
     QPageSize__EnvelopePrc2 => // 109
     {return String::from("EnvelopePrc2");}
     QPageSize__EnvelopePrc3 => // 110
     {return String::from("EnvelopePrc3");}
     QPageSize__EnvelopePrc4 => // 111
     {return String::from("EnvelopePrc4");}
     QPageSize__EnvelopePrc5 => // 112
     {return String::from("EnvelopePrc5");}
     QPageSize__EnvelopePrc6 => // 113
     {return String::from("EnvelopePrc6");}
     QPageSize__EnvelopePrc7 => // 114
     {return String::from("EnvelopePrc7");}
     QPageSize__EnvelopePrc8 => // 115
     {return String::from("EnvelopePrc8");}
     QPageSize__EnvelopePrc9 => // 116
     {return String::from("EnvelopePrc9");}
     QPageSize__EnvelopePrc10 => // 117
     {return String::from("EnvelopePrc10");}
     QPageSize__EnvelopeYou4 => // 118
     {return String::from("EnvelopeYou4,LastPageSize,NPageSize,NPaperSize");}
    // QPageSize__LastPageSize => // 118
    // {return String::from("");}
    // QPageSize__NPageSize => // 118
    // {return String::from("");}
    // QPageSize__NPaperSize => // 118
    // {return String::from("");}
    // QPageSize__AnsiA => // 2
    // {return String::from("");}
    // QPageSize__AnsiB => // 28
    // {return String::from("");}
    // QPageSize__EnvelopeC5 => // 24
    // {return String::from("");}
    // QPageSize__EnvelopeDL => // 26
    // {return String::from("");}
    // QPageSize__Envelope10 => // 25
    // {return String::from("");}
  _ => {return format!("{}", val);}
}
}
pub fn QPageSize_PageSizeIdItemName_s(val: i32) ->String {
  //var nilthis *QPageSize
  //return nilthis.PageSizeIdItemName(val);
  return QPageSize_PageSizeIdItemName(val);
}


/*
This enum type is used to specify the measurement unit for page sizes.


*/
pub type QPageSize__Unit = i32;
//  
pub const QPageSize__Millimeter :QPageSize__Unit = 0;
// 
pub const QPageSize__Point :QPageSize__Unit = 1;
//  
pub const QPageSize__Inch :QPageSize__Unit = 2;
// 
pub const QPageSize__Pica :QPageSize__Unit = 3;
// 
pub const QPageSize__Didot :QPageSize__Unit = 4;
// 
pub const QPageSize__Cicero :QPageSize__Unit = 5;
pub fn QPageSize_UnitItemName(val: i32) ->String {
  match val {
     QPageSize__Millimeter => // 0
     {return String::from("Millimeter");}
     QPageSize__Point => // 1
     {return String::from("Point");}
     QPageSize__Inch => // 2
     {return String::from("Inch");}
     QPageSize__Pica => // 3
     {return String::from("Pica");}
     QPageSize__Didot => // 4
     {return String::from("Didot");}
     QPageSize__Cicero => // 5
     {return String::from("Cicero");}
  _ => {return format!("{}", val);}
}
}
pub fn QPageSize_UnitItemName_s(val: i32) ->String {
  //var nilthis *QPageSize
  //return nilthis.UnitItemName(val);
  return QPageSize_UnitItemName(val);
}


/*

*/
pub type QPageSize__SizeMatchPolicy = i32;
// Match to a standard page size if within the margin of tolerance.
pub const QPageSize__FuzzyMatch :QPageSize__SizeMatchPolicy = 0;
// Match to a standard page size if within the margin of tolerance regardless of orientation.
pub const QPageSize__FuzzyOrientationMatch :QPageSize__SizeMatchPolicy = 1;
// Only match to a standard page size if the sizes match exactly.
pub const QPageSize__ExactMatch :QPageSize__SizeMatchPolicy = 2;
pub fn QPageSize_SizeMatchPolicyItemName(val: i32) ->String {
  match val {
     QPageSize__FuzzyMatch => // 0
     {return String::from("FuzzyMatch");}
     QPageSize__FuzzyOrientationMatch => // 1
     {return String::from("FuzzyOrientationMatch");}
     QPageSize__ExactMatch => // 2
     {return String::from("ExactMatch");}
  _ => {return format!("{}", val);}
}
}
pub fn QPageSize_SizeMatchPolicyItemName_s(val: i32) ->String {
  //var nilthis *QPageSize
  //return nilthis.SizeMatchPolicyItemName(val);
  return QPageSize_SizeMatchPolicyItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
