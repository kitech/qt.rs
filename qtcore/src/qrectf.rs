

// mod ::core::QRectF
// package qtcore
// /usr/include/qt/QtCore/qrect.h
// #include <qrect.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 72
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
#[derive(Default)] // class sizeof(QRectF)=32
pub struct QRectF {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRectF_ITF interface {
//    QRectF_PTR() *QRectF
//}
//func (ptr *QRectF) QRectF_PTR() *QRectF { return ptr }

impl /*struct*/ QRectF {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRectF {
    return QRectF{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRectF {
//  type Target = QRectFBASE;
//
//  fn deref(&self) -> &QRectFBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRectFBASE> for QRectF {
//  fn as_ref(& self) -> & QRectFBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qrect.h:514
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QRectF()

/*

*/
// QRectF() ctx.fn_proto_cpp
impl /*struct*/ QRectF {
  pub fn QRectF_0<T: QRectF_QRectF_0>(value: T) -> QRectF {
    let rsthis = value.QRectF_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRectF_QRectF_0 {
  fn QRectF_0(self) -> QRectF;
}
// QRectF() ctx.fn_proto_cpp
impl<'a> /*trait*/ QRectF_QRectF_0 for () {
  fn QRectF_0(self) -> QRectF {
    // unsafe{_ZN6QRectFC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QRectFC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRectF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:515
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QRectF(const QPointF &, const QSizeF &)

/*

*/
// QRectF(const QPointF &, const QSizeF &) ctx.fn_proto_cpp
impl /*struct*/ QRectF {
  pub fn QRectF_1<T: QRectF_QRectF_1>(value: T) -> QRectF {
    let rsthis = value.QRectF_1();
    return rsthis;
    // return 1;
  }
}

pub trait QRectF_QRectF_1 {
  fn QRectF_1(self) -> QRectF;
}
// QRectF(const QPointF &, const QSizeF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRectF_QRectF_1 for (usize,usize) {
  fn QRectF_1(self) -> QRectF {
    // unsafe{_ZN6QRectFC2ERK7QPointFRK6QSizeF()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QRectFC2ERK7QPointFRK6QSizeF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRectF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:516
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QRectF(const QPointF &, const QPointF &)

/*

*/
// QRectF(const QPointF &, const QPointF &) ctx.fn_proto_cpp
impl /*struct*/ QRectF {
  pub fn QRectF_2<T: QRectF_QRectF_2>(value: T) -> QRectF {
    let rsthis = value.QRectF_2();
    return rsthis;
    // return 1;
  }
}

pub trait QRectF_QRectF_2 {
  fn QRectF_2(self) -> QRectF;
}
// QRectF(const QPointF &, const QPointF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRectF_QRectF_2 for (usize,usize) {
  fn QRectF_2(self) -> QRectF {
    // unsafe{_ZN6QRectFC2ERK7QPointFS2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QRectFC2ERK7QPointFS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRectF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:517
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void QRectF(qreal, qreal, qreal, qreal)

/*

*/
// QRectF(qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl /*struct*/ QRectF {
  pub fn QRectF_3<T: QRectF_QRectF_3>(value: T) -> QRectF {
    let rsthis = value.QRectF_3();
    return rsthis;
    // return 1;
  }
}

pub trait QRectF_QRectF_3 {
  fn QRectF_3(self) -> QRectF;
}
// QRectF(qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRectF_QRectF_3 for (f64,f64,f64,f64) {
  fn QRectF_3(self) -> QRectF {
    // unsafe{_ZN6QRectFC2Edddd()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QRectFC2Edddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRectF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:518
// index:4
// Public inline Visibility=Default Availability=Available
// [-2] void QRectF(const QRect &)

/*

*/
// QRectF(const QRect &) ctx.fn_proto_cpp
impl /*struct*/ QRectF {
  pub fn QRectF_4<T: QRectF_QRectF_4>(value: T) -> QRectF {
    let rsthis = value.QRectF_4();
    return rsthis;
    // return 1;
  }
}

pub trait QRectF_QRectF_4 {
  fn QRectF_4(self) -> QRectF;
}
// QRectF(const QRect &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRectF_QRectF_4 for (usize) {
  fn QRectF_4(self) -> QRectF {
    // unsafe{_ZN6QRectFC2ERK5QRect()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QRectFC2ERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRectF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:520
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the rectangle is a null rectangle, otherwise returns false.

A null rectangle has both the width and the height set to 0 (i.e., right() == left() - 1 and bottom() == top() - 1). A null rectangle is also empty, and hence is not valid.

See also isEmpty() and isValid().
*/
impl /*struct*/ QRectF {
  pub fn isNull_0<RetType, T: QRectF_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QRectF_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QRectF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:521
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the rectangle is empty, otherwise returns false.

An empty rectangle has a left() > right() or top() > bottom(). An empty rectangle is not valid (i.e., isEmpty() == !isValid()).

Use the normalized() function to retrieve a rectangle where the corners are swapped.

See also isNull(), isValid(), and normalized().
*/
impl /*struct*/ QRectF {
  pub fn isEmpty_0<RetType, T: QRectF_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QRectF_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QRectF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:522
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the rectangle is valid, otherwise returns false.

A valid rectangle has a left() <= right() and top() <= bottom(). Note that non-trivial operations like intersections are not defined for invalid rectangles. A valid rectangle is not empty (i.e., isValid() == !isEmpty()).

See also isNull(), isEmpty(), and normalized().
*/
impl /*struct*/ QRectF {
  pub fn isValid_0<RetType, T: QRectF_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QRectF_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QRectF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:523
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF normalized() const

/*
Returns a normalized rectangle; i.e., a rectangle that has a non-negative width and height.

If width() < 0 the function swaps the left and right corners, and it swaps the top and bottom corners if height() < 0.

See also isValid() and isEmpty().
*/
impl /*struct*/ QRectF {
  pub fn normalized_0<RetType, T: QRectF_normalized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalized_0(self);
    // return 1;
  }
}
pub trait QRectF_normalized_0<RetType> {
  fn normalized_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_normalized_0<usize> for () {
  fn normalized_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF10normalizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:525
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal left() const

/*
Returns the x-coordinate of the rectangle's left edge. Equivalent to x().

See also setLeft(), topLeft(), and bottomLeft().
*/
impl /*struct*/ QRectF {
  pub fn left_0<RetType, T: QRectF_left_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.left_0(self);
    // return 1;
  }
}
pub trait QRectF_left_0<RetType> {
  fn left_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_left_0<f64> for () {
  fn left_0(self , rsthis: & QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF4leftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:526
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal top() const

/*
Returns the y-coordinate of the rectangle's top edge. Equivalent to y().

See also setTop(), topLeft(), and topRight().
*/
impl /*struct*/ QRectF {
  pub fn top_0<RetType, T: QRectF_top_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.top_0(self);
    // return 1;
  }
}
pub trait QRectF_top_0<RetType> {
  fn top_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_top_0<f64> for () {
  fn top_0(self , rsthis: & QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF3topEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:527
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal right() const

/*
Returns the x-coordinate of the rectangle's right edge.

Note that for historical reasons this function returns left() + width() - 1; use x() + width() to retrieve the true x-coordinate.

See also setRight(), topRight(), and bottomRight().
*/
impl /*struct*/ QRectF {
  pub fn right_0<RetType, T: QRectF_right_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.right_0(self);
    // return 1;
  }
}
pub trait QRectF_right_0<RetType> {
  fn right_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_right_0<f64> for () {
  fn right_0(self , rsthis: & QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF5rightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:528
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal bottom() const

/*
Returns the y-coordinate of the rectangle's bottom edge.

Note that for historical reasons this function returns top() + height() - 1; use y() + height() to retrieve the true y-coordinate.

See also setBottom(), bottomLeft(), and bottomRight().
*/
impl /*struct*/ QRectF {
  pub fn bottom_0<RetType, T: QRectF_bottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottom_0(self);
    // return 1;
  }
}
pub trait QRectF_bottom_0<RetType> {
  fn bottom_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_bottom_0<f64> for () {
  fn bottom_0(self , rsthis: & QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF6bottomEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:530
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal x() const

/*
Returns the x-coordinate of the rectangle's left edge. Equivalent to left().

See also setX(), y(), and topLeft().
*/
impl /*struct*/ QRectF {
  pub fn x_0<RetType, T: QRectF_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QRectF_x_0<RetType> {
  fn x_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_x_0<f64> for () {
  fn x_0(self , rsthis: & QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:531
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal y() const

/*
Returns the y-coordinate of the rectangle's top edge. Equivalent to top().

See also setY(), x(), and topLeft().
*/
impl /*struct*/ QRectF {
  pub fn y_0<RetType, T: QRectF_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QRectF_y_0<RetType> {
  fn y_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_y_0<f64> for () {
  fn y_0(self , rsthis: & QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:532
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setLeft(qreal)

/*
Sets the left edge of the rectangle to the given x coordinate. May change the width, but will never change the right edge of the rectangle.

Equivalent to setX().

See also left() and moveLeft().
*/
impl /*struct*/ QRectF {
  pub fn setLeft_0<RetType, T: QRectF_setLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLeft_0(self);
    // return 1;
  }
}
pub trait QRectF_setLeft_0<RetType> {
  fn setLeft_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setLeft_0<(/*void*/)> for (f64) {
  fn setLeft_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF7setLeftEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:533
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTop(qreal)

/*
Sets the top edge of the rectangle to the given y coordinate. May change the height, but will never change the bottom edge of the rectangle.

Equivalent to setY().

See also top() and moveTop().
*/
impl /*struct*/ QRectF {
  pub fn setTop_0<RetType, T: QRectF_setTop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTop_0(self);
    // return 1;
  }
}
pub trait QRectF_setTop_0<RetType> {
  fn setTop_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setTop_0<(/*void*/)> for (f64) {
  fn setTop_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF6setTopEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:534
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setRight(qreal)

/*
Sets the right edge of the rectangle to the given x coordinate. May change the width, but will never change the left edge of the rectangle.

See also right() and moveRight().
*/
impl /*struct*/ QRectF {
  pub fn setRight_0<RetType, T: QRectF_setRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRight_0(self);
    // return 1;
  }
}
pub trait QRectF_setRight_0<RetType> {
  fn setRight_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setRight_0<(/*void*/)> for (f64) {
  fn setRight_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF8setRightEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:535
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBottom(qreal)

/*
Sets the bottom edge of the rectangle to the given y coordinate. May change the height, but will never change the top edge of the rectangle.

See also bottom() and moveBottom().
*/
impl /*struct*/ QRectF {
  pub fn setBottom_0<RetType, T: QRectF_setBottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottom_0(self);
    // return 1;
  }
}
pub trait QRectF_setBottom_0<RetType> {
  fn setBottom_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setBottom_0<(/*void*/)> for (f64) {
  fn setBottom_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF9setBottomEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:536
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setX(qreal)

/*
Sets the left edge of the rectangle to the given x coordinate. May change the width, but will never change the right edge of the rectangle.

Equivalent to setLeft().

See also x(), setY(), and setTopLeft().
*/
impl /*struct*/ QRectF {
  pub fn setX_0<RetType, T: QRectF_setX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setX_0(self);
    // return 1;
  }
}
pub trait QRectF_setX_0<RetType> {
  fn setX_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setX_0<(/*void*/)> for (f64) {
  fn setX_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF4setXEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:537
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setY(qreal)

/*
Sets the top edge of the rectangle to the given y coordinate. May change the height, but will never change the bottom edge of the rectangle.

Equivalent to setTop().

See also y(), setX(), and setTopLeft().
*/
impl /*struct*/ QRectF {
  pub fn setY_0<RetType, T: QRectF_setY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setY_0(self);
    // return 1;
  }
}
pub trait QRectF_setY_0<RetType> {
  fn setY_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setY_0<(/*void*/)> for (f64) {
  fn setY_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF4setYEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:539
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF topLeft() const

/*
Returns the position of the rectangle's top-left corner.

See also setTopLeft(), top(), and left().
*/
impl /*struct*/ QRectF {
  pub fn topLeft_0<RetType, T: QRectF_topLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topLeft_0(self);
    // return 1;
  }
}
pub trait QRectF_topLeft_0<RetType> {
  fn topLeft_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_topLeft_0<usize> for () {
  fn topLeft_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF7topLeftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:540
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF bottomRight() const

/*
Returns the position of the rectangle's bottom-right corner.

Note that for historical reasons this function returns QPoint(left() + width() -1, top() + height() - 1).

See also setBottomRight(), bottom(), and right().
*/
impl /*struct*/ QRectF {
  pub fn bottomRight_0<RetType, T: QRectF_bottomRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottomRight_0(self);
    // return 1;
  }
}
pub trait QRectF_bottomRight_0<RetType> {
  fn bottomRight_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_bottomRight_0<usize> for () {
  fn bottomRight_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF11bottomRightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:541
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF topRight() const

/*
Returns the position of the rectangle's top-right corner.

Note that for historical reasons this function returns QPoint(left() + width() -1, top()).

See also setTopRight(), top(), and right().
*/
impl /*struct*/ QRectF {
  pub fn topRight_0<RetType, T: QRectF_topRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topRight_0(self);
    // return 1;
  }
}
pub trait QRectF_topRight_0<RetType> {
  fn topRight_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_topRight_0<usize> for () {
  fn topRight_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF8topRightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:542
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF bottomLeft() const

/*
Returns the position of the rectangle's bottom-left corner. Note that for historical reasons this function returns QPoint(left(), top() + height() - 1).

See also setBottomLeft(), bottom(), and left().
*/
impl /*struct*/ QRectF {
  pub fn bottomLeft_0<RetType, T: QRectF_bottomLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottomLeft_0(self);
    // return 1;
  }
}
pub trait QRectF_bottomLeft_0<RetType> {
  fn bottomLeft_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_bottomLeft_0<usize> for () {
  fn bottomLeft_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF10bottomLeftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:543
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF center() const

/*
Returns the center point of the rectangle.

See also moveCenter().
*/
impl /*struct*/ QRectF {
  pub fn center_0<RetType, T: QRectF_center_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.center_0(self);
    // return 1;
  }
}
pub trait QRectF_center_0<RetType> {
  fn center_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_center_0<usize> for () {
  fn center_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF6centerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:545
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTopLeft(const QPointF &)

/*
Set the top-left corner of the rectangle to the given position. May change the size, but will never change the bottom-right corner of the rectangle.

See also topLeft() and moveTopLeft().
*/
impl /*struct*/ QRectF {
  pub fn setTopLeft_0<RetType, T: QRectF_setTopLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTopLeft_0(self);
    // return 1;
  }
}
pub trait QRectF_setTopLeft_0<RetType> {
  fn setTopLeft_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setTopLeft_0<(/*void*/)> for (usize) {
  fn setTopLeft_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF10setTopLeftERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:546
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBottomRight(const QPointF &)

/*
Set the bottom-right corner of the rectangle to the given position. May change the size, but will never change the top-left corner of the rectangle.

See also bottomRight() and moveBottomRight().
*/
impl /*struct*/ QRectF {
  pub fn setBottomRight_0<RetType, T: QRectF_setBottomRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottomRight_0(self);
    // return 1;
  }
}
pub trait QRectF_setBottomRight_0<RetType> {
  fn setBottomRight_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setBottomRight_0<(/*void*/)> for (usize) {
  fn setBottomRight_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF14setBottomRightERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:547
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTopRight(const QPointF &)

/*
Set the top-right corner of the rectangle to the given position. May change the size, but will never change the bottom-left corner of the rectangle.

See also topRight() and moveTopRight().
*/
impl /*struct*/ QRectF {
  pub fn setTopRight_0<RetType, T: QRectF_setTopRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTopRight_0(self);
    // return 1;
  }
}
pub trait QRectF_setTopRight_0<RetType> {
  fn setTopRight_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setTopRight_0<(/*void*/)> for (usize) {
  fn setTopRight_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF11setTopRightERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:548
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBottomLeft(const QPointF &)

/*
Set the bottom-left corner of the rectangle to the given position. May change the size, but will never change the top-right corner of the rectangle.

See also bottomLeft() and moveBottomLeft().
*/
impl /*struct*/ QRectF {
  pub fn setBottomLeft_0<RetType, T: QRectF_setBottomLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottomLeft_0(self);
    // return 1;
  }
}
pub trait QRectF_setBottomLeft_0<RetType> {
  fn setBottomLeft_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setBottomLeft_0<(/*void*/)> for (usize) {
  fn setBottomLeft_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF13setBottomLeftERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:550
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveLeft(qreal)

/*
Moves the rectangle horizontally, leaving the rectangle's left edge at the given x coordinate. The rectangle's size is unchanged.

See also left(), setLeft(), and moveRight().
*/
impl /*struct*/ QRectF {
  pub fn moveLeft_0<RetType, T: QRectF_moveLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveLeft_0(self);
    // return 1;
  }
}
pub trait QRectF_moveLeft_0<RetType> {
  fn moveLeft_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_moveLeft_0<(/*void*/)> for (f64) {
  fn moveLeft_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF8moveLeftEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:551
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveTop(qreal)

/*
Moves the rectangle vertically, leaving the rectangle's top edge at the given y coordinate. The rectangle's size is unchanged.

See also top(), setTop(), and moveBottom().
*/
impl /*struct*/ QRectF {
  pub fn moveTop_0<RetType, T: QRectF_moveTop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveTop_0(self);
    // return 1;
  }
}
pub trait QRectF_moveTop_0<RetType> {
  fn moveTop_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_moveTop_0<(/*void*/)> for (f64) {
  fn moveTop_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF7moveTopEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:552
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveRight(qreal)

/*
Moves the rectangle horizontally, leaving the rectangle's right edge at the given x coordinate. The rectangle's size is unchanged.

See also right(), setRight(), and moveLeft().
*/
impl /*struct*/ QRectF {
  pub fn moveRight_0<RetType, T: QRectF_moveRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveRight_0(self);
    // return 1;
  }
}
pub trait QRectF_moveRight_0<RetType> {
  fn moveRight_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_moveRight_0<(/*void*/)> for (f64) {
  fn moveRight_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF9moveRightEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:553
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveBottom(qreal)

/*
Moves the rectangle vertically, leaving the rectangle's bottom edge at the given y coordinate. The rectangle's size is unchanged.

See also bottom(), setBottom(), and moveTop().
*/
impl /*struct*/ QRectF {
  pub fn moveBottom_0<RetType, T: QRectF_moveBottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveBottom_0(self);
    // return 1;
  }
}
pub trait QRectF_moveBottom_0<RetType> {
  fn moveBottom_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_moveBottom_0<(/*void*/)> for (f64) {
  fn moveBottom_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF10moveBottomEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:554
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveTopLeft(const QPointF &)

/*
Moves the rectangle, leaving the top-left corner at the given position. The rectangle's size is unchanged.

See also setTopLeft(), moveTop(), and moveLeft().
*/
impl /*struct*/ QRectF {
  pub fn moveTopLeft_0<RetType, T: QRectF_moveTopLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveTopLeft_0(self);
    // return 1;
  }
}
pub trait QRectF_moveTopLeft_0<RetType> {
  fn moveTopLeft_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_moveTopLeft_0<(/*void*/)> for (usize) {
  fn moveTopLeft_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF11moveTopLeftERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:555
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveBottomRight(const QPointF &)

/*
Moves the rectangle, leaving the bottom-right corner at the given position. The rectangle's size is unchanged.

See also setBottomRight(), moveRight(), and moveBottom().
*/
impl /*struct*/ QRectF {
  pub fn moveBottomRight_0<RetType, T: QRectF_moveBottomRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveBottomRight_0(self);
    // return 1;
  }
}
pub trait QRectF_moveBottomRight_0<RetType> {
  fn moveBottomRight_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_moveBottomRight_0<(/*void*/)> for (usize) {
  fn moveBottomRight_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF15moveBottomRightERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:556
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveTopRight(const QPointF &)

/*
Moves the rectangle, leaving the top-right corner at the given position. The rectangle's size is unchanged.

See also setTopRight(), moveTop(), and moveRight().
*/
impl /*struct*/ QRectF {
  pub fn moveTopRight_0<RetType, T: QRectF_moveTopRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveTopRight_0(self);
    // return 1;
  }
}
pub trait QRectF_moveTopRight_0<RetType> {
  fn moveTopRight_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_moveTopRight_0<(/*void*/)> for (usize) {
  fn moveTopRight_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF12moveTopRightERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:557
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveBottomLeft(const QPointF &)

/*
Moves the rectangle, leaving the bottom-left corner at the given position. The rectangle's size is unchanged.

See also setBottomLeft(), moveBottom(), and moveLeft().
*/
impl /*struct*/ QRectF {
  pub fn moveBottomLeft_0<RetType, T: QRectF_moveBottomLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveBottomLeft_0(self);
    // return 1;
  }
}
pub trait QRectF_moveBottomLeft_0<RetType> {
  fn moveBottomLeft_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_moveBottomLeft_0<(/*void*/)> for (usize) {
  fn moveBottomLeft_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF14moveBottomLeftERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:558
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveCenter(const QPointF &)

/*
Moves the rectangle, leaving the center point at the given position. The rectangle's size is unchanged.

See also center().
*/
impl /*struct*/ QRectF {
  pub fn moveCenter_0<RetType, T: QRectF_moveCenter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveCenter_0(self);
    // return 1;
  }
}
pub trait QRectF_moveCenter_0<RetType> {
  fn moveCenter_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_moveCenter_0<(/*void*/)> for (usize) {
  fn moveCenter_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF10moveCenterERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:560
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void translate(qreal, qreal)

/*
Moves the rectangle dx along the x axis and dy along the y axis, relative to the current position. Positive values move the rectangle to the right and down.

See also moveTopLeft(), moveTo(), and translated().
*/
impl /*struct*/ QRectF {
  pub fn translate_0<RetType, T: QRectF_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QRectF_translate_0<RetType> {
  fn translate_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_translate_0<(/*void*/)> for (f64,f64) {
  fn translate_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF9translateEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:561
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void translate(const QPointF &)

/*
Moves the rectangle dx along the x axis and dy along the y axis, relative to the current position. Positive values move the rectangle to the right and down.

See also moveTopLeft(), moveTo(), and translated().
*/
impl /*struct*/ QRectF {
  pub fn translate_1<RetType, T: QRectF_translate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_1(self);
    // return 1;
  }
}
pub trait QRectF_translate_1<RetType> {
  fn translate_1(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_translate_1<(/*void*/)> for (usize) {
  fn translate_1(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF9translateERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:563
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QRectF translated(qreal, qreal) const

/*
Returns a copy of the rectangle that is translated dx along the x axis and dy along the y axis, relative to the current position. Positive values move the rectangle to the right and down.

See also translate().
*/
impl /*struct*/ QRectF {
  pub fn translated_0<RetType, T: QRectF_translated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_0(self);
    // return 1;
  }
}
pub trait QRectF_translated_0<RetType> {
  fn translated_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_translated_0<usize> for (f64,f64) {
  fn translated_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF10translatedEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:564
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QRectF translated(const QPointF &) const

/*
Returns a copy of the rectangle that is translated dx along the x axis and dy along the y axis, relative to the current position. Positive values move the rectangle to the right and down.

See also translate().
*/
impl /*struct*/ QRectF {
  pub fn translated_1<RetType, T: QRectF_translated_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_1(self);
    // return 1;
  }
}
pub trait QRectF_translated_1<RetType> {
  fn translated_1(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_translated_1<usize> for (usize) {
  fn translated_1(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF10translatedERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:566
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QRectF transposed() const

/*
Returns a copy of the rectangle that has its width and height exchanged:


  QRect r = {15, 51, 42, 24};
  r = r.transposed(); // r == {15, 51, 24, 42}



This function was introduced in  Qt 5.7.

See also QSize::transposed().
*/
impl /*struct*/ QRectF {
  pub fn transposed_0<RetType, T: QRectF_transposed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transposed_0(self);
    // return 1;
  }
}
pub trait QRectF_transposed_0<RetType> {
  fn transposed_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_transposed_0<usize> for () {
  fn transposed_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF10transposedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:568
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveTo(qreal, qreal)

/*
Moves the rectangle, leaving the top-left corner at the given position (x, y). The rectangle's size is unchanged.

See also translate() and moveTopLeft().
*/
impl /*struct*/ QRectF {
  pub fn moveTo_0<RetType, T: QRectF_moveTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveTo_0(self);
    // return 1;
  }
}
pub trait QRectF_moveTo_0<RetType> {
  fn moveTo_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_moveTo_0<(/*void*/)> for (f64,f64) {
  fn moveTo_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF6moveToEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:569
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void moveTo(const QPointF &)

/*
Moves the rectangle, leaving the top-left corner at the given position (x, y). The rectangle's size is unchanged.

See also translate() and moveTopLeft().
*/
impl /*struct*/ QRectF {
  pub fn moveTo_1<RetType, T: QRectF_moveTo_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveTo_1(self);
    // return 1;
  }
}
pub trait QRectF_moveTo_1<RetType> {
  fn moveTo_1(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_moveTo_1<(/*void*/)> for (usize) {
  fn moveTo_1(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF6moveToERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:571
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setRect(qreal, qreal, qreal, qreal)

/*
Sets the coordinates of the rectangle's top-left corner to (x, y), and its size to the given width and height.

See also getRect() and setCoords().
*/
impl /*struct*/ QRectF {
  pub fn setRect_0<RetType, T: QRectF_setRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRect_0(self);
    // return 1;
  }
}
pub trait QRectF_setRect_0<RetType> {
  fn setRect_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setRect_0<(/*void*/)> for (f64,f64,f64,f64) {
  fn setRect_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF7setRectEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:572
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void getRect(qreal *, qreal *, qreal *, qreal *) const

/*
Extracts the position of the rectangle's top-left corner to *x and *y, and its dimensions to *width and *height.

See also setRect() and getCoords().
*/
impl /*struct*/ QRectF {
  pub fn getRect_0<RetType, T: QRectF_getRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getRect_0(self);
    // return 1;
  }
}
pub trait QRectF_getRect_0<RetType> {
  fn getRect_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_getRect_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getRect_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK6QRectF7getRectEPdS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:574
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setCoords(qreal, qreal, qreal, qreal)

/*
Sets the coordinates of the rectangle's top-left corner to (x1, y1), and the coordinates of its bottom-right corner to (x2, y2).

See also getCoords() and setRect().
*/
impl /*struct*/ QRectF {
  pub fn setCoords_0<RetType, T: QRectF_setCoords_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCoords_0(self);
    // return 1;
  }
}
pub trait QRectF_setCoords_0<RetType> {
  fn setCoords_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setCoords_0<(/*void*/)> for (f64,f64,f64,f64) {
  fn setCoords_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF9setCoordsEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:575
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void getCoords(qreal *, qreal *, qreal *, qreal *) const

/*
Extracts the position of the rectangle's top-left corner to *x1 and *y1, and the position of the bottom-right corner to *x2 and *y2.

See also setCoords() and getRect().
*/
impl /*struct*/ QRectF {
  pub fn getCoords_0<RetType, T: QRectF_getCoords_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getCoords_0(self);
    // return 1;
  }
}
pub trait QRectF_getCoords_0<RetType> {
  fn getCoords_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_getCoords_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getCoords_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK6QRectF9getCoordsEPdS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:577
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void adjust(qreal, qreal, qreal, qreal)

/*
Adds dx1, dy1, dx2 and dy2 respectively to the existing coordinates of the rectangle.

See also adjusted() and setRect().
*/
impl /*struct*/ QRectF {
  pub fn adjust_0<RetType, T: QRectF_adjust_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.adjust_0(self);
    // return 1;
  }
}
pub trait QRectF_adjust_0<RetType> {
  fn adjust_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_adjust_0<(/*void*/)> for (f64,f64,f64,f64) {
  fn adjust_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF6adjustEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:578
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QRectF adjusted(qreal, qreal, qreal, qreal) const

/*
Returns a new rectangle with dx1, dy1, dx2 and dy2 added respectively to the existing coordinates of this rectangle.

See also adjust().
*/
impl /*struct*/ QRectF {
  pub fn adjusted_0<RetType, T: QRectF_adjusted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.adjusted_0(self);
    // return 1;
  }
}
pub trait QRectF_adjusted_0<RetType> {
  fn adjusted_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_adjusted_0<usize> for (f64,f64,f64,f64) {
  fn adjusted_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF8adjustedEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:580
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QSizeF size() const

/*
Returns the size of the rectangle.

See also setSize(), width(), and height().
*/
impl /*struct*/ QRectF {
  pub fn size_0<RetType, T: QRectF_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QRectF_size_0<RetType> {
  fn size_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_size_0<usize> for () {
  fn size_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:581
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal width() const

/*
Returns the width of the rectangle.

See also setWidth(), height(), and size().
*/
impl /*struct*/ QRectF {
  pub fn width_0<RetType, T: QRectF_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QRectF_width_0<RetType> {
  fn width_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_width_0<f64> for () {
  fn width_0(self , rsthis: & QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:582
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal height() const

/*
Returns the height of the rectangle.

See also setHeight(), width(), and size().
*/
impl /*struct*/ QRectF {
  pub fn height_0<RetType, T: QRectF_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QRectF_height_0<RetType> {
  fn height_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_height_0<f64> for () {
  fn height_0(self , rsthis: & QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:583
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setWidth(qreal)

/*
Sets the width of the rectangle to the given width. The right edge is changed, but not the left one.

See also width() and setSize().
*/
impl /*struct*/ QRectF {
  pub fn setWidth_0<RetType, T: QRectF_setWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidth_0(self);
    // return 1;
  }
}
pub trait QRectF_setWidth_0<RetType> {
  fn setWidth_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setWidth_0<(/*void*/)> for (f64) {
  fn setWidth_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF8setWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:584
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setHeight(qreal)

/*
Sets the height of the rectangle to the given height. The bottom edge is changed, but not the top one.

See also height() and setSize().
*/
impl /*struct*/ QRectF {
  pub fn setHeight_0<RetType, T: QRectF_setHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeight_0(self);
    // return 1;
  }
}
pub trait QRectF_setHeight_0<RetType> {
  fn setHeight_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setHeight_0<(/*void*/)> for (f64) {
  fn setHeight_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF9setHeightEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:585
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setSize(const QSizeF &)

/*
Sets the size of the rectangle to the given size. The top-left corner is not moved.

See also size(), setWidth(), and setHeight().
*/
impl /*struct*/ QRectF {
  pub fn setSize_0<RetType, T: QRectF_setSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSize_0(self);
    // return 1;
  }
}
pub trait QRectF_setSize_0<RetType> {
  fn setSize_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_setSize_0<(/*void*/)> for (usize) {
  fn setSize_0(self , rsthis: & QRectF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QRectF7setSizeERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:587
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF operator|(const QRectF &) const

/*

*/
impl /*struct*/ QRectF {
  pub fn operator_or_0<RetType, T: QRectF_operator_or_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_or_0(self);
    // return 1;
  }
}
pub trait QRectF_operator_or_0<RetType> {
  fn operator_or_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_operator_or_0<usize> for (usize) {
  fn operator_or_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectForERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:588
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF operator&(const QRectF &) const

/*

*/
impl /*struct*/ QRectF {
  pub fn operator_and_0<RetType, T: QRectF_operator_and_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_and_0(self);
    // return 1;
  }
}
pub trait QRectF_operator_and_0<RetType> {
  fn operator_and_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_operator_and_0<usize> for (usize) {
  fn operator_and_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectFanERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:589
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QRectF & operator|=(const QRectF &)

/*

*/
impl /*struct*/ QRectF {
  pub fn operator_or_equal_0<RetType, T: QRectF_operator_or_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_or_equal_0(self);
    // return 1;
  }
}
pub trait QRectF_operator_or_equal_0<RetType> {
  fn operator_or_equal_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_operator_or_equal_0<usize> for (usize) {
  fn operator_or_equal_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QRectFoRERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:590
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QRectF & operator&=(const QRectF &)

/*

*/
impl /*struct*/ QRectF {
  pub fn operator_and_equal_0<RetType, T: QRectF_operator_and_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_and_equal_0(self);
    // return 1;
  }
}
pub trait QRectF_operator_and_equal_0<RetType> {
  fn operator_and_equal_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_operator_and_equal_0<usize> for (usize) {
  fn operator_and_equal_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QRectFaNERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:592
// index:0
// Public Visibility=Default Availability=Available
// [1] bool contains(const QRectF &) const

/*
Returns true if the given point is inside or on the edge of the rectangle, otherwise returns false. If proper is true, this function only returns true if the given point is inside the rectangle (i.e., not on the edge).

See also intersects().
*/
impl /*struct*/ QRectF {
  pub fn contains_0<RetType, T: QRectF_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QRectF_contains_0<RetType> {
  fn contains_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_contains_0<bool> for (usize) {
  fn contains_0(self , rsthis: & QRectF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF8containsERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:593
// index:1
// Public Visibility=Default Availability=Available
// [1] bool contains(const QPointF &) const

/*
Returns true if the given point is inside or on the edge of the rectangle, otherwise returns false. If proper is true, this function only returns true if the given point is inside the rectangle (i.e., not on the edge).

See also intersects().
*/
impl /*struct*/ QRectF {
  pub fn contains_1<RetType, T: QRectF_contains_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_1(self);
    // return 1;
  }
}
pub trait QRectF_contains_1<RetType> {
  fn contains_1(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_contains_1<bool> for (usize) {
  fn contains_1(self , rsthis: & QRectF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF8containsERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:594
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool contains(qreal, qreal) const

/*
Returns true if the given point is inside or on the edge of the rectangle, otherwise returns false. If proper is true, this function only returns true if the given point is inside the rectangle (i.e., not on the edge).

See also intersects().
*/
impl /*struct*/ QRectF {
  pub fn contains_2<RetType, T: QRectF_contains_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_2(self);
    // return 1;
  }
}
pub trait QRectF_contains_2<RetType> {
  fn contains_2(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_contains_2<bool> for (f64,f64) {
  fn contains_2(self , rsthis: & QRectF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF8containsEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:595
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QRectF united(const QRectF &) const

/*
Returns the bounding rectangle of this rectangle and the given rectangle.



This function was introduced in  Qt 4.2.

See also intersected().
*/
impl /*struct*/ QRectF {
  pub fn united_0<RetType, T: QRectF_united_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.united_0(self);
    // return 1;
  }
}
pub trait QRectF_united_0<RetType> {
  fn united_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_united_0<usize> for (usize) {
  fn united_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF6unitedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:596
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QRectF intersected(const QRectF &) const

/*
Returns the intersection of this rectangle and the given rectangle. Note that r.intersected(s) is equivalent to r & s.



This function was introduced in  Qt 4.2.

See also intersects(), united(), and operator&=().
*/
impl /*struct*/ QRectF {
  pub fn intersected_0<RetType, T: QRectF_intersected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersected_0(self);
    // return 1;
  }
}
pub trait QRectF_intersected_0<RetType> {
  fn intersected_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_intersected_0<usize> for (usize) {
  fn intersected_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF11intersectedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:597
// index:0
// Public Visibility=Default Availability=Available
// [1] bool intersects(const QRectF &) const

/*
Returns true if this rectangle intersects with the given rectangle (i.e., there is at least one pixel that is within both rectangles), otherwise returns false.

The intersection rectangle can be retrieved using the intersected() function.

See also contains().
*/
impl /*struct*/ QRectF {
  pub fn intersects_0<RetType, T: QRectF_intersects_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersects_0(self);
    // return 1;
  }
}
pub trait QRectF_intersects_0<RetType> {
  fn intersects_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_intersects_0<bool> for (usize) {
  fn intersects_0(self , rsthis: & QRectF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF10intersectsERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:599
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QRectF marginsAdded(const QMarginsF &) const

/*
Returns a rectangle grown by the margins.

This function was introduced in  Qt 5.1.

See also operator+=(), marginsRemoved(), and operator-=().
*/
impl /*struct*/ QRectF {
  pub fn marginsAdded_0<RetType, T: QRectF_marginsAdded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.marginsAdded_0(self);
    // return 1;
  }
}
pub trait QRectF_marginsAdded_0<RetType> {
  fn marginsAdded_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_marginsAdded_0<usize> for (usize) {
  fn marginsAdded_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF12marginsAddedERK9QMarginsF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:600
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QRectF marginsRemoved(const QMarginsF &) const

/*
Removes the margins from the rectangle, shrinking it.

This function was introduced in  Qt 5.1.

See also marginsAdded(), operator+=(), and operator-=().
*/
impl /*struct*/ QRectF {
  pub fn marginsRemoved_0<RetType, T: QRectF_marginsRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.marginsRemoved_0(self);
    // return 1;
  }
}
pub trait QRectF_marginsRemoved_0<RetType> {
  fn marginsRemoved_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_marginsRemoved_0<usize> for (usize) {
  fn marginsRemoved_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF14marginsRemovedERK9QMarginsF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:601
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QRectF & operator+=(const QMarginsF &)

/*

*/
impl /*struct*/ QRectF {
  pub fn operator_add_equal_0<RetType, T: QRectF_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QRectF_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QRectFpLERK9QMarginsF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:602
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QRectF & operator-=(const QMarginsF &)

/*

*/
impl /*struct*/ QRectF {
  pub fn operator_minus_equal_0<RetType, T: QRectF_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QRectF_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QRectFmIERK9QMarginsF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:612
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect toRect() const

/*

*/
impl /*struct*/ QRectF {
  pub fn toRect_0<RetType, T: QRectF_toRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toRect_0(self);
    // return 1;
  }
}
pub trait QRectF_toRect_0<RetType> {
  fn toRect_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_toRect_0<usize> for () {
  fn toRect_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF6toRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:613
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect toAlignedRect() const

/*

*/
impl /*struct*/ QRectF {
  pub fn toAlignedRect_0<RetType, T: QRectF_toAlignedRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toAlignedRect_0(self);
    // return 1;
  }
}
pub trait QRectF_toAlignedRect_0<RetType> {
  fn toAlignedRect_0(self , rsthis: & QRectF) -> RetType;
}
impl<'a> /*trait*/ QRectF_toAlignedRect_0<usize> for () {
  fn toAlignedRect_0(self , rsthis: & QRectF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QRectF13toAlignedRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQRectF(this :*mut QRectF) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN6QRectFD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
