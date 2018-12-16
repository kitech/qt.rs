

// mod ::gui::QPolygon
// package qtgui
// /usr/include/qt/QtGui/qpolygon.h
// #include <qpolygon.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 77
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
#[derive(Default)] // class sizeof(QPolygon)=8
pub struct QPolygon {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPolygon_ITF interface {
//    QPolygon_PTR() *QPolygon
//}
//func (ptr *QPolygon) QPolygon_PTR() *QPolygon { return ptr }

impl /*struct*/ QPolygon {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPolygon {
    return QPolygon{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPolygon {
//  type Target = QPolygonBASE;
//
//  fn deref(&self) -> &QPolygonBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPolygonBASE> for QPolygon {
//  fn as_ref(& self) -> & QPolygonBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpolygon.h:59
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QPolygon()

/*
Constructs a polygon with no points.

See also QVector::isEmpty().
*/
// QPolygon() ctx.fn_proto_cpp
impl /*struct*/ QPolygon {
  pub fn QPolygon_0<T: QPolygon_QPolygon_0>(value: T) -> QPolygon {
    let rsthis = value.QPolygon_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygon_QPolygon_0 {
  fn QPolygon_0(self) -> QPolygon;
}
// QPolygon() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPolygon_QPolygon_0 for () {
  fn QPolygon_0(self) -> QPolygon {
    // unsafe{_ZN8QPolygonC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QPolygonC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPolygon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:61
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QPolygon(int)

/*
Constructs a polygon with no points.

See also QVector::isEmpty().
*/
// QPolygon(int) ctx.fn_proto_cpp
impl /*struct*/ QPolygon {
  pub fn QPolygon_1<T: QPolygon_QPolygon_1>(value: T) -> QPolygon {
    let rsthis = value.QPolygon_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygon_QPolygon_1 {
  fn QPolygon_1(self) -> QPolygon;
}
// QPolygon(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPolygon_QPolygon_1 for (i32) {
  fn QPolygon_1(self) -> QPolygon {
    // unsafe{_ZN8QPolygonC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QPolygonC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPolygon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:66
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QPolygon(const QRect &, bool)

/*
Constructs a polygon with no points.

See also QVector::isEmpty().
*/
// QPolygon(const QRect &, bool) ctx.fn_proto_cpp
impl /*struct*/ QPolygon {
  pub fn QPolygon_2<T: QPolygon_QPolygon_2>(value: T) -> QPolygon {
    let rsthis = value.QPolygon_2();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygon_QPolygon_2 {
  fn QPolygon_2(self) -> QPolygon;
}
// QPolygon(const QRect &, bool) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPolygon_QPolygon_2 for (usize,bool) {
  fn QPolygon_2(self) -> QPolygon {
    // unsafe{_ZN8QPolygonC2ERK5QRectb()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QPolygonC2ERK5QRectb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPolygon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:67
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QPolygon(int, const int *)

/*
Constructs a polygon with no points.

See also QVector::isEmpty().
*/
// QPolygon(int, const int *) ctx.fn_proto_cpp
impl /*struct*/ QPolygon {
  pub fn QPolygon_3<T: QPolygon_QPolygon_3>(value: T) -> QPolygon {
    let rsthis = value.QPolygon_3();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygon_QPolygon_3 {
  fn QPolygon_3(self) -> QPolygon;
}
// QPolygon(int, const int *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPolygon_QPolygon_3 for (i32,usize) {
  fn QPolygon_3(self) -> QPolygon {
    // unsafe{_ZN8QPolygonC2EiPKi()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QPolygonC2EiPKi", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPolygon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:60
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~QPolygon()

/*

*/
pub fn DeleteQPolygon(this :*mut QPolygon) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QPolygonD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpolygon.h:71
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPolygon & operator=(QPolygon &&)

/*

*/
impl /*struct*/ QPolygon {
  pub fn operator_equal_0<RetType, T: QPolygon_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QPolygon_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QPolygon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QPolygonaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:73
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QPolygon & operator=(const QPolygon &)

/*

*/
impl /*struct*/ QPolygon {
  pub fn operator_equal_1<RetType, T: QPolygon_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QPolygon_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QPolygon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QPolygonaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:74
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QPolygon &)

/*
Swaps polygon other with this polygon. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QPolygon {
  pub fn swap_0<RetType, T: QPolygon_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QPolygon_swap_0<RetType> {
  fn swap_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QPolygon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPolygon4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void translate(int, int)

/*
Translates all points in the polygon by (dx, dy).

See also translated().
*/
impl /*struct*/ QPolygon {
  pub fn translate_0<RetType, T: QPolygon_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QPolygon_translate_0<RetType> {
  fn translate_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_translate_0<(/*void*/)> for (i32,i32) {
  fn translate_0(self , rsthis: & QPolygon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPolygon9translateEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:79
// index:1
// Public Visibility=Default Availability=Available
// [-2] void translate(const QPoint &)

/*
Translates all points in the polygon by (dx, dy).

See also translated().
*/
impl /*struct*/ QPolygon {
  pub fn translate_1<RetType, T: QPolygon_translate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_1(self);
    // return 1;
  }
}
pub trait QPolygon_translate_1<RetType> {
  fn translate_1(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_translate_1<(/*void*/)> for (usize) {
  fn translate_1(self , rsthis: & QPolygon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPolygon9translateERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:81
// index:0
// Public Visibility=Default Availability=Available
// [8] QPolygon translated(int, int) const

/*
Returns a copy of the polygon that is translated by (dx, dy).

This function was introduced in  Qt 4.6.

See also translate().
*/
impl /*struct*/ QPolygon {
  pub fn translated_0<RetType, T: QPolygon_translated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_0(self);
    // return 1;
  }
}
pub trait QPolygon_translated_0<RetType> {
  fn translated_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_translated_0<usize> for (i32,i32) {
  fn translated_0(self , rsthis: & QPolygon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPolygon10translatedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:82
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QPolygon translated(const QPoint &) const

/*
Returns a copy of the polygon that is translated by (dx, dy).

This function was introduced in  Qt 4.6.

See also translate().
*/
impl /*struct*/ QPolygon {
  pub fn translated_1<RetType, T: QPolygon_translated_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_1(self);
    // return 1;
  }
}
pub trait QPolygon_translated_1<RetType> {
  fn translated_1(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_translated_1<usize> for (usize) {
  fn translated_1(self , rsthis: & QPolygon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPolygon10translatedERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:84
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect boundingRect() const

/*
Returns the bounding rectangle of the polygon, or QRect(0, 0, 0, 0) if the polygon is empty.

See also QVector::isEmpty().
*/
impl /*struct*/ QPolygon {
  pub fn boundingRect_0<RetType, T: QPolygon_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QPolygon_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_boundingRect_0<usize> for () {
  fn boundingRect_0(self , rsthis: & QPolygon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPolygon12boundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void point(int, int *, int *) const

/*
Extracts the coordinates of the point at the given index to *x and *y (if they are valid pointers).

See also setPoint().
*/
impl /*struct*/ QPolygon {
  pub fn point_0<RetType, T: QPolygon_point_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.point_0(self);
    // return 1;
  }
}
pub trait QPolygon_point_0<RetType> {
  fn point_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_point_0<(/*void*/)> for (i32,usize,usize) {
  fn point_0(self , rsthis: & QPolygon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK8QPolygon5pointEiPiS0_", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:87
// index:1
// Public Visibility=Default Availability=Available
// [8] QPoint point(int) const

/*
Extracts the coordinates of the point at the given index to *x and *y (if they are valid pointers).

See also setPoint().
*/
impl /*struct*/ QPolygon {
  pub fn point_1<RetType, T: QPolygon_point_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.point_1(self);
    // return 1;
  }
}
pub trait QPolygon_point_1<RetType> {
  fn point_1(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_point_1<usize> for (i32) {
  fn point_1(self , rsthis: & QPolygon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPolygon5pointEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPoint(int, int, int)

/*
Sets the point at the given index to the point specified by (x, y).

See also point(), putPoints(), and setPoints().
*/
impl /*struct*/ QPolygon {
  pub fn setPoint_0<RetType, T: QPolygon_setPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPoint_0(self);
    // return 1;
  }
}
pub trait QPolygon_setPoint_0<RetType> {
  fn setPoint_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_setPoint_0<(/*void*/)> for (i32,i32,i32) {
  fn setPoint_0(self , rsthis: & QPolygon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPolygon8setPointEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:89
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setPoint(int, const QPoint &)

/*
Sets the point at the given index to the point specified by (x, y).

See also point(), putPoints(), and setPoints().
*/
impl /*struct*/ QPolygon {
  pub fn setPoint_1<RetType, T: QPolygon_setPoint_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPoint_1(self);
    // return 1;
  }
}
pub trait QPolygon_setPoint_1<RetType> {
  fn setPoint_1(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_setPoint_1<(/*void*/)> for (i32,usize) {
  fn setPoint_1(self , rsthis: & QPolygon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPolygon8setPointEiRK6QPoint", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPoints(int, const int *)

/*
Resizes the polygon to nPoints and populates it with the given points.

The example code creates a polygon with two points (10, 20) and (30, 40):


          static const int points[] = { 10, 20, 30, 40 };
          QPolygon polygon;
          polygon.setPoints(2, points);



See also setPoint() and putPoints().
*/
impl /*struct*/ QPolygon {
  pub fn setPoints_0<RetType, T: QPolygon_setPoints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPoints_0(self);
    // return 1;
  }
}
pub trait QPolygon_setPoints_0<RetType> {
  fn setPoints_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_setPoints_0<(/*void*/)> for (i32,usize) {
  fn setPoints_0(self , rsthis: & QPolygon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPolygon9setPointsEiPKi", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void putPoints(int, int, const int *)

/*
Copies nPoints points from the variable argument list into this polygon from the given index.

The points are given as a sequence of integers, starting with firstx then firsty, and so on. The polygon is resized if index+nPoints exceeds its current size.

The example code creates a polygon with three points (4,5), (6,7) and (8,9), by expanding the polygon from 1 to 3 points:


          QPolygon polygon(1);
          polygon[0] = QPoint(4, 5);
          polygon.putPoints(1, 2, 6,7, 8,9);



The following code has the same result, but here the putPoints() function overwrites rather than extends:


          QPolygon polygon(3);
          polygon.putPoints(0, 3, 4,5, 0,0, 8,9);
          polygon.putPoints(1, 1, 6,7);



See also setPoints().
*/
impl /*struct*/ QPolygon {
  pub fn putPoints_0<RetType, T: QPolygon_putPoints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.putPoints_0(self);
    // return 1;
  }
}
pub trait QPolygon_putPoints_0<RetType> {
  fn putPoints_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_putPoints_0<(/*void*/)> for (i32,i32,usize) {
  fn putPoints_0(self , rsthis: & QPolygon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPolygon9putPointsEiiPKi", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:94
// index:1
// Public Visibility=Default Availability=Available
// [-2] void putPoints(int, int, const QPolygon &, int)

/*
Copies nPoints points from the variable argument list into this polygon from the given index.

The points are given as a sequence of integers, starting with firstx then firsty, and so on. The polygon is resized if index+nPoints exceeds its current size.

The example code creates a polygon with three points (4,5), (6,7) and (8,9), by expanding the polygon from 1 to 3 points:


          QPolygon polygon(1);
          polygon[0] = QPoint(4, 5);
          polygon.putPoints(1, 2, 6,7, 8,9);



The following code has the same result, but here the putPoints() function overwrites rather than extends:


          QPolygon polygon(3);
          polygon.putPoints(0, 3, 4,5, 0,0, 8,9);
          polygon.putPoints(1, 1, 6,7);



See also setPoints().
*/
impl /*struct*/ QPolygon {
  pub fn putPoints_1<RetType, T: QPolygon_putPoints_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.putPoints_1(self);
    // return 1;
  }
}
pub trait QPolygon_putPoints_1<RetType> {
  fn putPoints_1(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_putPoints_1<(/*void*/)> for (i32,i32,usize,i32) {
  fn putPoints_1(self , rsthis: & QPolygon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPolygon9putPointsEiiRKS_i", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:96
// index:0
// Public Visibility=Default Availability=Available
// [1] bool containsPoint(const QPoint &, Qt::FillRule) const

/*
Returns true if the given point is inside the polygon according to the specified fillRule; otherwise returns false.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QPolygon {
  pub fn containsPoint_0<RetType, T: QPolygon_containsPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.containsPoint_0(self);
    // return 1;
  }
}
pub trait QPolygon_containsPoint_0<RetType> {
  fn containsPoint_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_containsPoint_0<bool> for (usize,i32) {
  fn containsPoint_0(self , rsthis: & QPolygon) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPolygon13containsPointERK6QPointN2Qt8FillRuleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:98
// index:0
// Public Visibility=Default Availability=Available
// [8] QPolygon united(const QPolygon &) const

/*
Returns a polygon which is the union of this polygon and r.

Set operations on polygons, will treat the polygons as areas, and implicitly close the polygon.

This function was introduced in  Qt 4.3.

See also intersected() and subtracted().
*/
impl /*struct*/ QPolygon {
  pub fn united_0<RetType, T: QPolygon_united_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.united_0(self);
    // return 1;
  }
}
pub trait QPolygon_united_0<RetType> {
  fn united_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_united_0<usize> for (usize) {
  fn united_0(self , rsthis: & QPolygon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPolygon6unitedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:99
// index:0
// Public Visibility=Default Availability=Available
// [8] QPolygon intersected(const QPolygon &) const

/*
Returns a polygon which is the intersection of this polygon and r.

Set operations on polygons will treat the polygons as areas. Non-closed polygons will be treated as implicitly closed.

This function was introduced in  Qt 4.3.

See also intersects().
*/
impl /*struct*/ QPolygon {
  pub fn intersected_0<RetType, T: QPolygon_intersected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersected_0(self);
    // return 1;
  }
}
pub trait QPolygon_intersected_0<RetType> {
  fn intersected_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_intersected_0<usize> for (usize) {
  fn intersected_0(self , rsthis: & QPolygon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPolygon11intersectedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:100
// index:0
// Public Visibility=Default Availability=Available
// [8] QPolygon subtracted(const QPolygon &) const

/*
Returns a polygon which is r subtracted from this polygon.

Set operations on polygons will treat the polygons as areas. Non-closed polygons will be treated as implicitly closed.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QPolygon {
  pub fn subtracted_0<RetType, T: QPolygon_subtracted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subtracted_0(self);
    // return 1;
  }
}
pub trait QPolygon_subtracted_0<RetType> {
  fn subtracted_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_subtracted_0<usize> for (usize) {
  fn subtracted_0(self , rsthis: & QPolygon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPolygon10subtractedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:102
// index:0
// Public Visibility=Default Availability=Available
// [1] bool intersects(const QPolygon &) const

/*
Returns true if the current polygon intersects at any point the given polygon p. Also returns true if the current polygon contains or is contained by any part of p.

Set operations on polygons will treat the polygons as areas. Non-closed polygons will be treated as implicitly closed.

This function was introduced in  Qt 5.10.

See also intersected().
*/
impl /*struct*/ QPolygon {
  pub fn intersects_0<RetType, T: QPolygon_intersects_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersects_0(self);
    // return 1;
  }
}
pub trait QPolygon_intersects_0<RetType> {
  fn intersects_0(self , rsthis: & QPolygon) -> RetType;
}
impl<'a> /*trait*/ QPolygon_intersects_0<bool> for (usize) {
  fn intersects_0(self , rsthis: & QPolygon) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPolygon10intersectsERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
