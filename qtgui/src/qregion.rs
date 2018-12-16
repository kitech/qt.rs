

// mod ::gui::QRegion
// package qtgui
// /usr/include/qt/QtGui/qregion.h
// #include <qregion.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 109
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
#[derive(Default)] // class sizeof(QRegion)=8
pub struct QRegion {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRegion_ITF interface {
//    QRegion_PTR() *QRegion
//}
//func (ptr *QRegion) QRegion_PTR() *QRegion { return ptr }

impl /*struct*/ QRegion {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRegion {
    return QRegion{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRegion {
//  type Target = QRegionBASE;
//
//  fn deref(&self) -> &QRegionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRegionBASE> for QRegion {
//  fn as_ref(& self) -> & QRegionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qregion.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QRegion()

/*
Constructs an empty region.

See also isEmpty().
*/
// QRegion() ctx.fn_proto_cpp
impl /*struct*/ QRegion {
  pub fn QRegion_0<T: QRegion_QRegion_0>(value: T) -> QRegion {
    let rsthis = value.QRegion_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRegion_QRegion_0 {
  fn QRegion_0(self) -> QRegion;
}
// QRegion() ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegion_QRegion_0 for () {
  fn QRegion_0(self) -> QRegion {
    // unsafe{_ZN7QRegionC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QRegionC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qregion.h:68
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QRegion(int, int, int, int, QRegion::RegionType)

/*
Constructs an empty region.

See also isEmpty().
*/
// QRegion(int, int, int, int, QRegion::RegionType) ctx.fn_proto_cpp
impl /*struct*/ QRegion {
  pub fn QRegion_1<T: QRegion_QRegion_1>(value: T) -> QRegion {
    let rsthis = value.QRegion_1();
    return rsthis;
    // return 1;
  }
}

pub trait QRegion_QRegion_1 {
  fn QRegion_1(self) -> QRegion;
}
// QRegion(int, int, int, int, QRegion::RegionType) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegion_QRegion_1 for (i32,i32,i32,i32,i32) {
  fn QRegion_1(self) -> QRegion {
    // unsafe{_ZN7QRegionC2EiiiiNS_10RegionTypeE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QRegionC2EiiiiNS_10RegionTypeE", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qregion.h:69
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QRegion(const QRect &, QRegion::RegionType)

/*
Constructs an empty region.

See also isEmpty().
*/
// QRegion(const QRect &, QRegion::RegionType) ctx.fn_proto_cpp
impl /*struct*/ QRegion {
  pub fn QRegion_2<T: QRegion_QRegion_2>(value: T) -> QRegion {
    let rsthis = value.QRegion_2();
    return rsthis;
    // return 1;
  }
}

pub trait QRegion_QRegion_2 {
  fn QRegion_2(self) -> QRegion;
}
// QRegion(const QRect &, QRegion::RegionType) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegion_QRegion_2 for (usize,i32) {
  fn QRegion_2(self) -> QRegion {
    // unsafe{_ZN7QRegionC2ERK5QRectNS_10RegionTypeE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QRegionC2ERK5QRectNS_10RegionTypeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qregion.h:70
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QRegion(const QPolygon &, Qt::FillRule)

/*
Constructs an empty region.

See also isEmpty().
*/
// QRegion(const QPolygon &, Qt::FillRule) ctx.fn_proto_cpp
impl /*struct*/ QRegion {
  pub fn QRegion_3<T: QRegion_QRegion_3>(value: T) -> QRegion {
    let rsthis = value.QRegion_3();
    return rsthis;
    // return 1;
  }
}

pub trait QRegion_QRegion_3 {
  fn QRegion_3(self) -> QRegion;
}
// QRegion(const QPolygon &, Qt::FillRule) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegion_QRegion_3 for (usize,i32) {
  fn QRegion_3(self) -> QRegion {
    // unsafe{_ZN7QRegionC2ERK8QPolygonN2Qt8FillRuleE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QRegionC2ERK8QPolygonN2Qt8FillRuleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qregion.h:74
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QRegion(const QBitmap &)

/*
Constructs an empty region.

See also isEmpty().
*/
// QRegion(const QBitmap &) ctx.fn_proto_cpp
impl /*struct*/ QRegion {
  pub fn QRegion_4<T: QRegion_QRegion_4>(value: T) -> QRegion {
    let rsthis = value.QRegion_4();
    return rsthis;
    // return 1;
  }
}

pub trait QRegion_QRegion_4 {
  fn QRegion_4(self) -> QRegion;
}
// QRegion(const QBitmap &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegion_QRegion_4 for (usize) {
  fn QRegion_4(self) -> QRegion {
    // unsafe{_ZN7QRegionC2ERK7QBitmap()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QRegionC2ERK7QBitmap", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qregion.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QRegion()

/*

*/
pub fn DeleteQRegion(this :*mut QRegion) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QRegionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qregion.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion & operator=(const QRegion &)

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_equal_0<RetType, T: QRegion_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QRegion_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegionaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:78
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QRegion & operator=(QRegion &&)

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_equal_1<RetType, T: QRegion_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QRegion_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegionaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QRegion &)

/*
Swaps region other with this region. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QRegion {
  pub fn swap_0<RetType, T: QRegion_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QRegion_swap_0<RetType> {
  fn swap_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QRegion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QRegion4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qregion.h:82
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the region is empty; otherwise returns false. An empty region is a region that contains no points.

Example:


  QRegion r1(10, 10, 20, 20);
  r1.isEmpty();               // false

  QRegion r3;
  r3.isEmpty();               // true

  QRegion r2(40, 40, 20, 20);
  r3 = r1.intersected(r2);    // r3: intersection of r1 and r2
  r3.isEmpty();               // true

  r3 = r1.united(r2);         // r3: union of r1 and r2
  r3.isEmpty();               // false
*/
impl /*struct*/ QRegion {
  pub fn isEmpty_0<RetType, T: QRegion_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QRegion_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QRegion) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:83
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the region is empty; otherwise returns false. An empty region is a region that contains no points. This function is the same as isEmpty

This function was introduced in  Qt 5.0.

See also isEmpty().
*/
impl /*struct*/ QRegion {
  pub fn isNull_0<RetType, T: QRegion_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QRegion_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QRegion) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion::const_iterator begin() const

/*
Returns a const_iterator pointing to the beginning of the range of rectangles that make up this range, in the order in which rects() returns them.

This function was introduced in  Qt 5.8.

See also rbegin(), cbegin(), and end().
*/
impl /*struct*/ QRegion {
  pub fn begin_0<RetType, T: QRegion_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QRegion_begin_0<RetType> {
  fn begin_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:89
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QRegion::const_iterator cbegin() const

/*
Same as begin().

This function was introduced in  Qt 5.8.
*/
impl /*struct*/ QRegion {
  pub fn cbegin_0<RetType, T: QRegion_cbegin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cbegin_0(self);
    // return 1;
  }
}
pub trait QRegion_cbegin_0<RetType> {
  fn cbegin_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_cbegin_0<usize> for () {
  fn cbegin_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion6cbeginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:90
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion::const_iterator end() const

/*
Returns a const_iterator pointing to one past the end of the range of rectangles that make up this range, in the order in which rects() returns them.

This function was introduced in  Qt 5.8.

See also rend(), cend(), and begin().
*/
impl /*struct*/ QRegion {
  pub fn end_0<RetType, T: QRegion_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QRegion_end_0<RetType> {
  fn end_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_end_0<usize> for () {
  fn end_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:91
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QRegion::const_iterator cend() const

/*
Same as end().

This function was introduced in  Qt 5.8.
*/
impl /*struct*/ QRegion {
  pub fn cend_0<RetType, T: QRegion_cend_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cend_0(self);
    // return 1;
  }
}
pub trait QRegion_cend_0<RetType> {
  fn cend_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_cend_0<usize> for () {
  fn cend_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion4cendEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:97
// index:0
// Public Visibility=Default Availability=Available
// [1] bool contains(const QPoint &) const

/*
Returns true if the region contains the point p; otherwise returns false.
*/
impl /*struct*/ QRegion {
  pub fn contains_0<RetType, T: QRegion_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QRegion_contains_0<RetType> {
  fn contains_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_contains_0<bool> for (usize) {
  fn contains_0(self , rsthis: & QRegion) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion8containsERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:98
// index:1
// Public Visibility=Default Availability=Available
// [1] bool contains(const QRect &) const

/*
Returns true if the region contains the point p; otherwise returns false.
*/
impl /*struct*/ QRegion {
  pub fn contains_1<RetType, T: QRegion_contains_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_1(self);
    // return 1;
  }
}
pub trait QRegion_contains_1<RetType> {
  fn contains_1(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_contains_1<bool> for (usize) {
  fn contains_1(self , rsthis: & QRegion) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion8containsERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void translate(int, int)

/*
Translates (moves) the region dx along the X axis and dy along the Y axis.
*/
impl /*struct*/ QRegion {
  pub fn translate_0<RetType, T: QRegion_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QRegion_translate_0<RetType> {
  fn translate_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_translate_0<(/*void*/)> for (i32,i32) {
  fn translate_0(self , rsthis: & QRegion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QRegion9translateEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qregion.h:101
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void translate(const QPoint &)

/*
Translates (moves) the region dx along the X axis and dy along the Y axis.
*/
impl /*struct*/ QRegion {
  pub fn translate_1<RetType, T: QRegion_translate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_1(self);
    // return 1;
  }
}
pub trait QRegion_translate_1<RetType> {
  fn translate_1(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_translate_1<(/*void*/)> for (usize) {
  fn translate_1(self , rsthis: & QRegion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QRegion9translateERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qregion.h:102
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion translated(int, int) const

/*
Returns a copy of the region that is translated dx along the x axis and dy along the y axis, relative to the current position. Positive values move the region to the right and down.

This function was introduced in  Qt 4.1.

See also translate().
*/
impl /*struct*/ QRegion {
  pub fn translated_0<RetType, T: QRegion_translated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_0(self);
    // return 1;
  }
}
pub trait QRegion_translated_0<RetType> {
  fn translated_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_translated_0<usize> for (i32,i32) {
  fn translated_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion10translatedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:103
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QRegion translated(const QPoint &) const

/*
Returns a copy of the region that is translated dx along the x axis and dy along the y axis, relative to the current position. Positive values move the region to the right and down.

This function was introduced in  Qt 4.1.

See also translate().
*/
impl /*struct*/ QRegion {
  pub fn translated_1<RetType, T: QRegion_translated_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_1(self);
    // return 1;
  }
}
pub trait QRegion_translated_1<RetType> {
  fn translated_1(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_translated_1<usize> for (usize) {
  fn translated_1(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion10translatedERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion united(const QRegion &) const

/*
Returns a region which is the union of this region and r.



The figure shows the union of two elliptical regions.

This function was introduced in  Qt 4.2.

See also intersected(), subtracted(), and xored().
*/
impl /*struct*/ QRegion {
  pub fn united_0<RetType, T: QRegion_united_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.united_0(self);
    // return 1;
  }
}
pub trait QRegion_united_0<RetType> {
  fn united_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_united_0<usize> for (usize) {
  fn united_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion6unitedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:106
// index:1
// Public Visibility=Default Availability=Available
// [8] QRegion united(const QRect &) const

/*
Returns a region which is the union of this region and r.



The figure shows the union of two elliptical regions.

This function was introduced in  Qt 4.2.

See also intersected(), subtracted(), and xored().
*/
impl /*struct*/ QRegion {
  pub fn united_1<RetType, T: QRegion_united_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.united_1(self);
    // return 1;
  }
}
pub trait QRegion_united_1<RetType> {
  fn united_1(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_united_1<usize> for (usize) {
  fn united_1(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion6unitedERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:107
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion intersected(const QRegion &) const

/*
Returns a region which is the intersection of this region and r.



The figure shows the intersection of two elliptical regions.

This function was introduced in  Qt 4.2.

See also subtracted(), united(), and xored().
*/
impl /*struct*/ QRegion {
  pub fn intersected_0<RetType, T: QRegion_intersected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersected_0(self);
    // return 1;
  }
}
pub trait QRegion_intersected_0<RetType> {
  fn intersected_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_intersected_0<usize> for (usize) {
  fn intersected_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion11intersectedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:108
// index:1
// Public Visibility=Default Availability=Available
// [8] QRegion intersected(const QRect &) const

/*
Returns a region which is the intersection of this region and r.



The figure shows the intersection of two elliptical regions.

This function was introduced in  Qt 4.2.

See also subtracted(), united(), and xored().
*/
impl /*struct*/ QRegion {
  pub fn intersected_1<RetType, T: QRegion_intersected_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersected_1(self);
    // return 1;
  }
}
pub trait QRegion_intersected_1<RetType> {
  fn intersected_1(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_intersected_1<usize> for (usize) {
  fn intersected_1(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion11intersectedERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:109
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion subtracted(const QRegion &) const

/*
Returns a region which is r subtracted from this region.



The figure shows the result when the ellipse on the right is subtracted from the ellipse on the left (left - right).

This function was introduced in  Qt 4.2.

See also intersected(), united(), and xored().
*/
impl /*struct*/ QRegion {
  pub fn subtracted_0<RetType, T: QRegion_subtracted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subtracted_0(self);
    // return 1;
  }
}
pub trait QRegion_subtracted_0<RetType> {
  fn subtracted_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_subtracted_0<usize> for (usize) {
  fn subtracted_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion10subtractedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:110
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion xored(const QRegion &) const

/*
Returns a region which is the exclusive or (XOR) of this region and r.



The figure shows the exclusive or of two elliptical regions.

This function was introduced in  Qt 4.2.

See also intersected(), united(), and subtracted().
*/
impl /*struct*/ QRegion {
  pub fn xored_0<RetType, T: QRegion_xored_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.xored_0(self);
    // return 1;
  }
}
pub trait QRegion_xored_0<RetType> {
  fn xored_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_xored_0<usize> for (usize) {
  fn xored_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion5xoredERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:121
// index:0
// Public Visibility=Default Availability=Available
// [1] bool intersects(const QRegion &) const

/*
Returns true if this region intersects with region, otherwise returns false.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QRegion {
  pub fn intersects_0<RetType, T: QRegion_intersects_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersects_0(self);
    // return 1;
  }
}
pub trait QRegion_intersects_0<RetType> {
  fn intersects_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_intersects_0<bool> for (usize) {
  fn intersects_0(self , rsthis: & QRegion) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion10intersectsERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:122
// index:1
// Public Visibility=Default Availability=Available
// [1] bool intersects(const QRect &) const

/*
Returns true if this region intersects with region, otherwise returns false.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QRegion {
  pub fn intersects_1<RetType, T: QRegion_intersects_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersects_1(self);
    // return 1;
  }
}
pub trait QRegion_intersects_1<RetType> {
  fn intersects_1(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_intersects_1<bool> for (usize) {
  fn intersects_1(self , rsthis: & QRegion) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion10intersectsERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:124
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect boundingRect() const

/*
Returns the bounding rectangle of this region. An empty region gives a rectangle that is QRect::isNull().
*/
impl /*struct*/ QRegion {
  pub fn boundingRect_0<RetType, T: QRegion_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QRegion_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_boundingRect_0<usize> for () {
  fn boundingRect_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion12boundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRects(const QRect *, int)

/*
Sets the region using the array of rectangles specified by rects and number. The rectangles must be optimally Y-X sorted and follow these restrictions:


The rectangles must not intersect.
All rectangles with a given top coordinate must have the same height.
No two rectangles may abut horizontally (they should be combined into a single wider rectangle in that case).
The rectangles must be sorted in ascending order, with Y as the major sort key and X as the minor sort key.


See also rects().
*/
impl /*struct*/ QRegion {
  pub fn setRects_0<RetType, T: QRegion_setRects_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRects_0(self);
    // return 1;
  }
}
pub trait QRegion_setRects_0<RetType> {
  fn setRects_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_setRects_0<(/*void*/)> for (usize,i32) {
  fn setRects_0(self , rsthis: & QRegion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QRegion8setRectsEPK5QRecti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qregion.h:127
// index:0
// Public Visibility=Default Availability=Available
// [4] int rectCount() const

/*
Returns the number of rectangles that will be returned in rects().

This function was introduced in  Qt 4.6.
*/
impl /*struct*/ QRegion {
  pub fn rectCount_0<RetType, T: QRegion_rectCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rectCount_0(self);
    // return 1;
  }
}
pub trait QRegion_rectCount_0<RetType> {
  fn rectCount_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_rectCount_0<i32> for () {
  fn rectCount_0(self , rsthis: & QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegion9rectCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:138
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion operator|(const QRegion &) const

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_or_0<RetType, T: QRegion_operator_or_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_or_0(self);
    // return 1;
  }
}
pub trait QRegion_operator_or_0<RetType> {
  fn operator_or_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_or_0<usize> for (usize) {
  fn operator_or_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegionorERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:139
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion operator+(const QRegion &) const

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_add_0<RetType, T: QRegion_operator_add_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_0(self);
    // return 1;
  }
}
pub trait QRegion_operator_add_0<RetType> {
  fn operator_add_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_add_0<usize> for (usize) {
  fn operator_add_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegionplERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:140
// index:1
// Public Visibility=Default Availability=Available
// [8] QRegion operator+(const QRect &) const

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_add_1<RetType, T: QRegion_operator_add_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_1(self);
    // return 1;
  }
}
pub trait QRegion_operator_add_1<RetType> {
  fn operator_add_1(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_add_1<usize> for (usize) {
  fn operator_add_1(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegionplERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:141
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion operator&(const QRegion &) const

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_and_0<RetType, T: QRegion_operator_and_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_and_0(self);
    // return 1;
  }
}
pub trait QRegion_operator_and_0<RetType> {
  fn operator_and_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_and_0<usize> for (usize) {
  fn operator_and_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegionanERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:142
// index:1
// Public Visibility=Default Availability=Available
// [8] QRegion operator&(const QRect &) const

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_and_1<RetType, T: QRegion_operator_and_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_and_1(self);
    // return 1;
  }
}
pub trait QRegion_operator_and_1<RetType> {
  fn operator_and_1(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_and_1<usize> for (usize) {
  fn operator_and_1(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegionanERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:143
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion operator-(const QRegion &) const

/*
Applies the subtracted() function to this region and r. r1-r2 is equivalent to r1.subtracted(r2).

See also subtracted().
*/
impl /*struct*/ QRegion {
  pub fn operator_minus_0<RetType, T: QRegion_operator_minus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_0(self);
    // return 1;
  }
}
pub trait QRegion_operator_minus_0<RetType> {
  fn operator_minus_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_minus_0<usize> for (usize) {
  fn operator_minus_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegionmiERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:144
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion operator^(const QRegion &) const

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_caret_0<RetType, T: QRegion_operator_caret_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_caret_0(self);
    // return 1;
  }
}
pub trait QRegion_operator_caret_0<RetType> {
  fn operator_caret_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_caret_0<usize> for (usize) {
  fn operator_caret_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegioneoERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:146
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion & operator|=(const QRegion &)

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_or_equal_0<RetType, T: QRegion_operator_or_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_or_equal_0(self);
    // return 1;
  }
}
pub trait QRegion_operator_or_equal_0<RetType> {
  fn operator_or_equal_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_or_equal_0<usize> for (usize) {
  fn operator_or_equal_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegionoRERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:147
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion & operator+=(const QRegion &)

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_add_equal_0<RetType, T: QRegion_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QRegion_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegionpLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:148
// index:1
// Public Visibility=Default Availability=Available
// [8] QRegion & operator+=(const QRect &)

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_add_equal_1<RetType, T: QRegion_operator_add_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_1(self);
    // return 1;
  }
}
pub trait QRegion_operator_add_equal_1<RetType> {
  fn operator_add_equal_1(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_add_equal_1<usize> for (usize) {
  fn operator_add_equal_1(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegionpLERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:149
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion & operator&=(const QRegion &)

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_and_equal_0<RetType, T: QRegion_operator_and_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_and_equal_0(self);
    // return 1;
  }
}
pub trait QRegion_operator_and_equal_0<RetType> {
  fn operator_and_equal_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_and_equal_0<usize> for (usize) {
  fn operator_and_equal_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegionaNERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:150
// index:1
// Public Visibility=Default Availability=Available
// [8] QRegion & operator&=(const QRect &)

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_and_equal_1<RetType, T: QRegion_operator_and_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_and_equal_1(self);
    // return 1;
  }
}
pub trait QRegion_operator_and_equal_1<RetType> {
  fn operator_and_equal_1(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_and_equal_1<usize> for (usize) {
  fn operator_and_equal_1(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegionaNERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:151
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion & operator-=(const QRegion &)

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_minus_equal_0<RetType, T: QRegion_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QRegion_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegionmIERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:152
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion & operator^=(const QRegion &)

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_caret_equal_0<RetType, T: QRegion_operator_caret_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_caret_equal_0(self);
    // return 1;
  }
}
pub trait QRegion_operator_caret_equal_0<RetType> {
  fn operator_caret_equal_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_caret_equal_0<usize> for (usize) {
  fn operator_caret_equal_0(self , rsthis: & QRegion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegioneOERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:154
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QRegion &) const

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_equal_equal_0<RetType, T: QRegion_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QRegion_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QRegion) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegioneqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qregion.h:155
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QRegion &) const

/*

*/
impl /*struct*/ QRegion {
  pub fn operator_not_equal_0<RetType, T: QRegion_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QRegion_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QRegion) -> RetType;
}
impl<'a> /*trait*/ QRegion_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QRegion) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegionneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
Specifies the shape of the region to be created.


*/
pub type QRegion__RegionType = i32;
// the region covers the entire rectangle.
pub const QRegion__Rectangle :QRegion__RegionType = 0;
// the region is an ellipse inside the rectangle.
pub const QRegion__Ellipse :QRegion__RegionType = 1;
pub fn QRegion_RegionTypeItemName(val: i32) ->String {
  match val {
     QRegion__Rectangle => // 0
     {return String::from("Rectangle");}
     QRegion__Ellipse => // 1
     {return String::from("Ellipse");}
  _ => {return format!("{}", val);}
}
}
pub fn QRegion_RegionTypeItemName_s(val: i32) ->String {
  //var nilthis *QRegion
  //return nilthis.RegionTypeItemName(val);
  return QRegion_RegionTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
