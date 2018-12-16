

// mod ::gui::QPolygonF
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
// extern C begin: 25
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
#[derive(Default)] // class sizeof(QPolygonF)=8
pub struct QPolygonF {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPolygonF_ITF interface {
//    QPolygonF_PTR() *QPolygonF
//}
//func (ptr *QPolygonF) QPolygonF_PTR() *QPolygonF { return ptr }

impl /*struct*/ QPolygonF {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPolygonF {
    return QPolygonF{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPolygonF {
//  type Target = QPolygonFBASE;
//
//  fn deref(&self) -> &QPolygonFBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPolygonFBASE> for QPolygonF {
//  fn as_ref(& self) -> & QPolygonFBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpolygon.h:144
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QPolygonF()

/*

*/
// QPolygonF() ctx.fn_proto_cpp
impl /*struct*/ QPolygonF {
  pub fn QPolygonF_0<T: QPolygonF_QPolygonF_0>(value: T) -> QPolygonF {
    let rsthis = value.QPolygonF_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygonF_QPolygonF_0 {
  fn QPolygonF_0(self) -> QPolygonF;
}
// QPolygonF() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPolygonF_QPolygonF_0 for () {
  fn QPolygonF_0(self) -> QPolygonF {
    // unsafe{_ZN9QPolygonFC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QPolygonFC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPolygonF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:146
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QPolygonF(int)

/*

*/
// QPolygonF(int) ctx.fn_proto_cpp
impl /*struct*/ QPolygonF {
  pub fn QPolygonF_1<T: QPolygonF_QPolygonF_1>(value: T) -> QPolygonF {
    let rsthis = value.QPolygonF_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygonF_QPolygonF_1 {
  fn QPolygonF_1(self) -> QPolygonF;
}
// QPolygonF(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPolygonF_QPolygonF_1 for (i32) {
  fn QPolygonF_1(self) -> QPolygonF {
    // unsafe{_ZN9QPolygonFC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QPolygonFC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPolygonF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:151
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QPolygonF(const QRectF &)

/*

*/
// QPolygonF(const QRectF &) ctx.fn_proto_cpp
impl /*struct*/ QPolygonF {
  pub fn QPolygonF_2<T: QPolygonF_QPolygonF_2>(value: T) -> QPolygonF {
    let rsthis = value.QPolygonF_2();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygonF_QPolygonF_2 {
  fn QPolygonF_2(self) -> QPolygonF;
}
// QPolygonF(const QRectF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPolygonF_QPolygonF_2 for (usize) {
  fn QPolygonF_2(self) -> QPolygonF {
    // unsafe{_ZN9QPolygonFC2ERK6QRectF()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QPolygonFC2ERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPolygonF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:152
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QPolygonF(const QPolygon &)

/*

*/
// QPolygonF(const QPolygon &) ctx.fn_proto_cpp
impl /*struct*/ QPolygonF {
  pub fn QPolygonF_3<T: QPolygonF_QPolygonF_3>(value: T) -> QPolygonF {
    let rsthis = value.QPolygonF_3();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygonF_QPolygonF_3 {
  fn QPolygonF_3(self) -> QPolygonF;
}
// QPolygonF(const QPolygon &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPolygonF_QPolygonF_3 for (usize) {
  fn QPolygonF_3(self) -> QPolygonF {
    // unsafe{_ZN9QPolygonFC2ERK8QPolygon()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QPolygonFC2ERK8QPolygon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPolygonF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:145
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~QPolygonF()

/*

*/
pub fn DeleteQPolygonF(this :*mut QPolygonF) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QPolygonFD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpolygon.h:156
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPolygonF & operator=(QPolygonF &&)

/*

*/
impl /*struct*/ QPolygonF {
  pub fn operator_equal_0<RetType, T: QPolygonF_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QPolygonF_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QPolygonF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPolygonFaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:158
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QPolygonF & operator=(const QPolygonF &)

/*

*/
impl /*struct*/ QPolygonF {
  pub fn operator_equal_1<RetType, T: QPolygonF_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QPolygonF_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QPolygonF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QPolygonFaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:159
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QPolygonF &)

/*
Swaps polygon other with this polygon. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QPolygonF {
  pub fn swap_0<RetType, T: QPolygonF_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QPolygonF_swap_0<RetType> {
  fn swap_0(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QPolygonF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QPolygonF4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:163
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void translate(qreal, qreal)

/*
Translates all points in the polygon by (dx, dy).

See also translated().
*/
impl /*struct*/ QPolygonF {
  pub fn translate_0<RetType, T: QPolygonF_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QPolygonF_translate_0<RetType> {
  fn translate_0(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_translate_0<(/*void*/)> for (f64,f64) {
  fn translate_0(self , rsthis: & QPolygonF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QPolygonF9translateEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:164
// index:1
// Public Visibility=Default Availability=Available
// [-2] void translate(const QPointF &)

/*
Translates all points in the polygon by (dx, dy).

See also translated().
*/
impl /*struct*/ QPolygonF {
  pub fn translate_1<RetType, T: QPolygonF_translate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_1(self);
    // return 1;
  }
}
pub trait QPolygonF_translate_1<RetType> {
  fn translate_1(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_translate_1<(/*void*/)> for (usize) {
  fn translate_1(self , rsthis: & QPolygonF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QPolygonF9translateERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:166
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPolygonF translated(qreal, qreal) const

/*
Returns a copy of the polygon that is translated by (dx, dy).

This function was introduced in  Qt 4.6.

See also translate().
*/
impl /*struct*/ QPolygonF {
  pub fn translated_0<RetType, T: QPolygonF_translated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_0(self);
    // return 1;
  }
}
pub trait QPolygonF_translated_0<RetType> {
  fn translated_0(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_translated_0<usize> for (f64,f64) {
  fn translated_0(self , rsthis: & QPolygonF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPolygonF10translatedEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:167
// index:1
// Public Visibility=Default Availability=Available
// [8] QPolygonF translated(const QPointF &) const

/*
Returns a copy of the polygon that is translated by (dx, dy).

This function was introduced in  Qt 4.6.

See also translate().
*/
impl /*struct*/ QPolygonF {
  pub fn translated_1<RetType, T: QPolygonF_translated_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_1(self);
    // return 1;
  }
}
pub trait QPolygonF_translated_1<RetType> {
  fn translated_1(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_translated_1<usize> for (usize) {
  fn translated_1(self , rsthis: & QPolygonF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPolygonF10translatedERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:169
// index:0
// Public Visibility=Default Availability=Available
// [8] QPolygon toPolygon() const

/*

*/
impl /*struct*/ QPolygonF {
  pub fn toPolygon_0<RetType, T: QPolygonF_toPolygon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPolygon_0(self);
    // return 1;
  }
}
pub trait QPolygonF_toPolygon_0<RetType> {
  fn toPolygon_0(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_toPolygon_0<usize> for () {
  fn toPolygon_0(self , rsthis: & QPolygonF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPolygonF9toPolygonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:171
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isClosed() const

/*

*/
impl /*struct*/ QPolygonF {
  pub fn isClosed_0<RetType, T: QPolygonF_isClosed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isClosed_0(self);
    // return 1;
  }
}
pub trait QPolygonF_isClosed_0<RetType> {
  fn isClosed_0(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_isClosed_0<bool> for () {
  fn isClosed_0(self , rsthis: & QPolygonF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPolygonF8isClosedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:173
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF boundingRect() const

/*
Returns the bounding rectangle of the polygon, or QRect(0, 0, 0, 0) if the polygon is empty.

See also QVector::isEmpty().
*/
impl /*struct*/ QPolygonF {
  pub fn boundingRect_0<RetType, T: QPolygonF_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QPolygonF_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_boundingRect_0<usize> for () {
  fn boundingRect_0(self , rsthis: & QPolygonF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPolygonF12boundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:175
// index:0
// Public Visibility=Default Availability=Available
// [1] bool containsPoint(const QPointF &, Qt::FillRule) const

/*
Returns true if the given point is inside the polygon according to the specified fillRule; otherwise returns false.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QPolygonF {
  pub fn containsPoint_0<RetType, T: QPolygonF_containsPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.containsPoint_0(self);
    // return 1;
  }
}
pub trait QPolygonF_containsPoint_0<RetType> {
  fn containsPoint_0(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_containsPoint_0<bool> for (usize,i32) {
  fn containsPoint_0(self , rsthis: & QPolygonF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPolygonF13containsPointERK7QPointFN2Qt8FillRuleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:177
// index:0
// Public Visibility=Default Availability=Available
// [8] QPolygonF united(const QPolygonF &) const

/*
Returns a polygon which is the union of this polygon and r.

Set operations on polygons, will treat the polygons as areas, and implicitly close the polygon.

This function was introduced in  Qt 4.3.

See also intersected() and subtracted().
*/
impl /*struct*/ QPolygonF {
  pub fn united_0<RetType, T: QPolygonF_united_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.united_0(self);
    // return 1;
  }
}
pub trait QPolygonF_united_0<RetType> {
  fn united_0(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_united_0<usize> for (usize) {
  fn united_0(self , rsthis: & QPolygonF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPolygonF6unitedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:178
// index:0
// Public Visibility=Default Availability=Available
// [8] QPolygonF intersected(const QPolygonF &) const

/*
Returns a polygon which is the intersection of this polygon and r.

Set operations on polygons will treat the polygons as areas. Non-closed polygons will be treated as implicitly closed.

This function was introduced in  Qt 4.3.

See also intersects().
*/
impl /*struct*/ QPolygonF {
  pub fn intersected_0<RetType, T: QPolygonF_intersected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersected_0(self);
    // return 1;
  }
}
pub trait QPolygonF_intersected_0<RetType> {
  fn intersected_0(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_intersected_0<usize> for (usize) {
  fn intersected_0(self , rsthis: & QPolygonF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPolygonF11intersectedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:179
// index:0
// Public Visibility=Default Availability=Available
// [8] QPolygonF subtracted(const QPolygonF &) const

/*
Returns a polygon which is r subtracted from this polygon.

Set operations on polygons will treat the polygons as areas. Non-closed polygons will be treated as implicitly closed.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QPolygonF {
  pub fn subtracted_0<RetType, T: QPolygonF_subtracted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subtracted_0(self);
    // return 1;
  }
}
pub trait QPolygonF_subtracted_0<RetType> {
  fn subtracted_0(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_subtracted_0<usize> for (usize) {
  fn subtracted_0(self , rsthis: & QPolygonF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPolygonF10subtractedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpolygon.h:181
// index:0
// Public Visibility=Default Availability=Available
// [1] bool intersects(const QPolygonF &) const

/*
Returns true if the current polygon intersects at any point the given polygon p. Also returns true if the current polygon contains or is contained by any part of p.

Set operations on polygons will treat the polygons as areas. Non-closed polygons will be treated as implicitly closed.

This function was introduced in  Qt 5.10.

See also intersected().
*/
impl /*struct*/ QPolygonF {
  pub fn intersects_0<RetType, T: QPolygonF_intersects_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersects_0(self);
    // return 1;
  }
}
pub trait QPolygonF_intersects_0<RetType> {
  fn intersects_0(self , rsthis: & QPolygonF) -> RetType;
}
impl<'a> /*trait*/ QPolygonF_intersects_0<bool> for (usize) {
  fn intersects_0(self , rsthis: & QPolygonF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QPolygonF10intersectsERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
