

// mod ::core::QRect
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
// extern C begin: 25
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
#[derive(Default)] // class sizeof(QRect)=16
pub struct QRect {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRect_ITF interface {
//    QRect_PTR() *QRect
//}
//func (ptr *QRect) QRect_PTR() *QRect { return ptr }

impl /*struct*/ QRect {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRect {
    return QRect{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRect {
//  type Target = QRectBASE;
//
//  fn deref(&self) -> &QRectBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRectBASE> for QRect {
//  fn as_ref(& self) -> & QRectBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qrect.h:60
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QRect()

/*
Constructs a null rectangle.

See also isNull().
*/
// QRect() ctx.fn_proto_cpp
impl /*struct*/ QRect {
  pub fn QRect_0<T: QRect_QRect_0>(value: T) -> QRect {
    let rsthis = value.QRect_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRect_QRect_0 {
  fn QRect_0(self) -> QRect;
}
// QRect() ctx.fn_proto_cpp
impl<'a> /*trait*/ QRect_QRect_0 for () {
  fn QRect_0(self) -> QRect {
    // unsafe{_ZN5QRectC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QRectC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRect{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:61
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QRect(const QPoint &, const QPoint &)

/*
Constructs a null rectangle.

See also isNull().
*/
// QRect(const QPoint &, const QPoint &) ctx.fn_proto_cpp
impl /*struct*/ QRect {
  pub fn QRect_1<T: QRect_QRect_1>(value: T) -> QRect {
    let rsthis = value.QRect_1();
    return rsthis;
    // return 1;
  }
}

pub trait QRect_QRect_1 {
  fn QRect_1(self) -> QRect;
}
// QRect(const QPoint &, const QPoint &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRect_QRect_1 for (usize,usize) {
  fn QRect_1(self) -> QRect {
    // unsafe{_ZN5QRectC2ERK6QPointS2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QRectC2ERK6QPointS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRect{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:62
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QRect(const QPoint &, const QSize &)

/*
Constructs a null rectangle.

See also isNull().
*/
// QRect(const QPoint &, const QSize &) ctx.fn_proto_cpp
impl /*struct*/ QRect {
  pub fn QRect_2<T: QRect_QRect_2>(value: T) -> QRect {
    let rsthis = value.QRect_2();
    return rsthis;
    // return 1;
  }
}

pub trait QRect_QRect_2 {
  fn QRect_2(self) -> QRect;
}
// QRect(const QPoint &, const QSize &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRect_QRect_2 for (usize,usize) {
  fn QRect_2(self) -> QRect {
    // unsafe{_ZN5QRectC2ERK6QPointRK5QSize()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QRectC2ERK6QPointRK5QSize", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRect{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:63
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void QRect(int, int, int, int)

/*
Constructs a null rectangle.

See also isNull().
*/
// QRect(int, int, int, int) ctx.fn_proto_cpp
impl /*struct*/ QRect {
  pub fn QRect_3<T: QRect_QRect_3>(value: T) -> QRect {
    let rsthis = value.QRect_3();
    return rsthis;
    // return 1;
  }
}

pub trait QRect_QRect_3 {
  fn QRect_3(self) -> QRect;
}
// QRect(int, int, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRect_QRect_3 for (i32,i32,i32,i32) {
  fn QRect_3(self) -> QRect {
    // unsafe{_ZN5QRectC2Eiiii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QRectC2Eiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRect{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:65
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the rectangle is a null rectangle, otherwise returns false.

A null rectangle has both the width and the height set to 0 (i.e., right() == left() - 1 and bottom() == top() - 1). A null rectangle is also empty, and hence is not valid.

See also isEmpty() and isValid().
*/
impl /*struct*/ QRect {
  pub fn isNull_0<RetType, T: QRect_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QRect_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QRect) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:66
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the rectangle is empty, otherwise returns false.

An empty rectangle has a left() > right() or top() > bottom(). An empty rectangle is not valid (i.e., isEmpty() == !isValid()).

Use the normalized() function to retrieve a rectangle where the corners are swapped.

See also isNull(), isValid(), and normalized().
*/
impl /*struct*/ QRect {
  pub fn isEmpty_0<RetType, T: QRect_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QRect_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QRect) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:67
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the rectangle is valid, otherwise returns false.

A valid rectangle has a left() <= right() and top() <= bottom(). Note that non-trivial operations like intersections are not defined for invalid rectangles. A valid rectangle is not empty (i.e., isValid() == !isEmpty()).

See also isNull(), isEmpty(), and normalized().
*/
impl /*struct*/ QRect {
  pub fn isValid_0<RetType, T: QRect_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QRect_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QRect) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int left() const

/*
Returns the x-coordinate of the rectangle's left edge. Equivalent to x().

See also setLeft(), topLeft(), and bottomLeft().
*/
impl /*struct*/ QRect {
  pub fn left_0<RetType, T: QRect_left_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.left_0(self);
    // return 1;
  }
}
pub trait QRect_left_0<RetType> {
  fn left_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_left_0<i32> for () {
  fn left_0(self , rsthis: & QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect4leftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:70
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int top() const

/*
Returns the y-coordinate of the rectangle's top edge. Equivalent to y().

See also setTop(), topLeft(), and topRight().
*/
impl /*struct*/ QRect {
  pub fn top_0<RetType, T: QRect_top_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.top_0(self);
    // return 1;
  }
}
pub trait QRect_top_0<RetType> {
  fn top_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_top_0<i32> for () {
  fn top_0(self , rsthis: & QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect3topEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:71
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int right() const

/*
Returns the x-coordinate of the rectangle's right edge.

Note that for historical reasons this function returns left() + width() - 1; use x() + width() to retrieve the true x-coordinate.

See also setRight(), topRight(), and bottomRight().
*/
impl /*struct*/ QRect {
  pub fn right_0<RetType, T: QRect_right_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.right_0(self);
    // return 1;
  }
}
pub trait QRect_right_0<RetType> {
  fn right_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_right_0<i32> for () {
  fn right_0(self , rsthis: & QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect5rightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:72
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int bottom() const

/*
Returns the y-coordinate of the rectangle's bottom edge.

Note that for historical reasons this function returns top() + height() - 1; use y() + height() to retrieve the true y-coordinate.

See also setBottom(), bottomLeft(), and bottomRight().
*/
impl /*struct*/ QRect {
  pub fn bottom_0<RetType, T: QRect_bottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottom_0(self);
    // return 1;
  }
}
pub trait QRect_bottom_0<RetType> {
  fn bottom_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_bottom_0<i32> for () {
  fn bottom_0(self , rsthis: & QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect6bottomEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:73
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect normalized() const

/*
Returns a normalized rectangle; i.e., a rectangle that has a non-negative width and height.

If width() < 0 the function swaps the left and right corners, and it swaps the top and bottom corners if height() < 0.

See also isValid() and isEmpty().
*/
impl /*struct*/ QRect {
  pub fn normalized_0<RetType, T: QRect_normalized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalized_0(self);
    // return 1;
  }
}
pub trait QRect_normalized_0<RetType> {
  fn normalized_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_normalized_0<usize> for () {
  fn normalized_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect10normalizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:75
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int x() const

/*
Returns the x-coordinate of the rectangle's left edge. Equivalent to left().

See also setX(), y(), and topLeft().
*/
impl /*struct*/ QRect {
  pub fn x_0<RetType, T: QRect_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QRect_x_0<RetType> {
  fn x_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_x_0<i32> for () {
  fn x_0(self , rsthis: & QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:76
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int y() const

/*
Returns the y-coordinate of the rectangle's top edge. Equivalent to top().

See also setY(), x(), and topLeft().
*/
impl /*struct*/ QRect {
  pub fn y_0<RetType, T: QRect_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QRect_y_0<RetType> {
  fn y_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_y_0<i32> for () {
  fn y_0(self , rsthis: & QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:77
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setLeft(int)

/*
Sets the left edge of the rectangle to the given x coordinate. May change the width, but will never change the right edge of the rectangle.

Equivalent to setX().

See also left() and moveLeft().
*/
impl /*struct*/ QRect {
  pub fn setLeft_0<RetType, T: QRect_setLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLeft_0(self);
    // return 1;
  }
}
pub trait QRect_setLeft_0<RetType> {
  fn setLeft_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setLeft_0<(/*void*/)> for (i32) {
  fn setLeft_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect7setLeftEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTop(int)

/*
Sets the top edge of the rectangle to the given y coordinate. May change the height, but will never change the bottom edge of the rectangle.

Equivalent to setY().

See also top() and moveTop().
*/
impl /*struct*/ QRect {
  pub fn setTop_0<RetType, T: QRect_setTop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTop_0(self);
    // return 1;
  }
}
pub trait QRect_setTop_0<RetType> {
  fn setTop_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setTop_0<(/*void*/)> for (i32) {
  fn setTop_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect6setTopEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:79
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setRight(int)

/*
Sets the right edge of the rectangle to the given x coordinate. May change the width, but will never change the left edge of the rectangle.

See also right() and moveRight().
*/
impl /*struct*/ QRect {
  pub fn setRight_0<RetType, T: QRect_setRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRight_0(self);
    // return 1;
  }
}
pub trait QRect_setRight_0<RetType> {
  fn setRight_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setRight_0<(/*void*/)> for (i32) {
  fn setRight_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect8setRightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:80
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBottom(int)

/*
Sets the bottom edge of the rectangle to the given y coordinate. May change the height, but will never change the top edge of the rectangle.

See also bottom() and moveBottom().
*/
impl /*struct*/ QRect {
  pub fn setBottom_0<RetType, T: QRect_setBottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottom_0(self);
    // return 1;
  }
}
pub trait QRect_setBottom_0<RetType> {
  fn setBottom_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setBottom_0<(/*void*/)> for (i32) {
  fn setBottom_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect9setBottomEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setX(int)

/*
Sets the left edge of the rectangle to the given x coordinate. May change the width, but will never change the right edge of the rectangle.

Equivalent to setLeft().

See also x(), setY(), and setTopLeft().
*/
impl /*struct*/ QRect {
  pub fn setX_0<RetType, T: QRect_setX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setX_0(self);
    // return 1;
  }
}
pub trait QRect_setX_0<RetType> {
  fn setX_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setX_0<(/*void*/)> for (i32) {
  fn setX_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect4setXEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:82
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setY(int)

/*
Sets the top edge of the rectangle to the given y coordinate. May change the height, but will never change the bottom edge of the rectangle.

Equivalent to setTop().

See also y(), setX(), and setTopLeft().
*/
impl /*struct*/ QRect {
  pub fn setY_0<RetType, T: QRect_setY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setY_0(self);
    // return 1;
  }
}
pub trait QRect_setY_0<RetType> {
  fn setY_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setY_0<(/*void*/)> for (i32) {
  fn setY_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect4setYEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:84
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTopLeft(const QPoint &)

/*
Set the top-left corner of the rectangle to the given position. May change the size, but will never change the bottom-right corner of the rectangle.

See also topLeft() and moveTopLeft().
*/
impl /*struct*/ QRect {
  pub fn setTopLeft_0<RetType, T: QRect_setTopLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTopLeft_0(self);
    // return 1;
  }
}
pub trait QRect_setTopLeft_0<RetType> {
  fn setTopLeft_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setTopLeft_0<(/*void*/)> for (usize) {
  fn setTopLeft_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect10setTopLeftERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:85
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBottomRight(const QPoint &)

/*
Set the bottom-right corner of the rectangle to the given position. May change the size, but will never change the top-left corner of the rectangle.

See also bottomRight() and moveBottomRight().
*/
impl /*struct*/ QRect {
  pub fn setBottomRight_0<RetType, T: QRect_setBottomRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottomRight_0(self);
    // return 1;
  }
}
pub trait QRect_setBottomRight_0<RetType> {
  fn setBottomRight_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setBottomRight_0<(/*void*/)> for (usize) {
  fn setBottomRight_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect14setBottomRightERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:86
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTopRight(const QPoint &)

/*
Set the top-right corner of the rectangle to the given position. May change the size, but will never change the bottom-left corner of the rectangle.

See also topRight() and moveTopRight().
*/
impl /*struct*/ QRect {
  pub fn setTopRight_0<RetType, T: QRect_setTopRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTopRight_0(self);
    // return 1;
  }
}
pub trait QRect_setTopRight_0<RetType> {
  fn setTopRight_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setTopRight_0<(/*void*/)> for (usize) {
  fn setTopRight_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect11setTopRightERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:87
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBottomLeft(const QPoint &)

/*
Set the bottom-left corner of the rectangle to the given position. May change the size, but will never change the top-right corner of the rectangle.

See also bottomLeft() and moveBottomLeft().
*/
impl /*struct*/ QRect {
  pub fn setBottomLeft_0<RetType, T: QRect_setBottomLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottomLeft_0(self);
    // return 1;
  }
}
pub trait QRect_setBottomLeft_0<RetType> {
  fn setBottomLeft_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setBottomLeft_0<(/*void*/)> for (usize) {
  fn setBottomLeft_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect13setBottomLeftERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:89
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint topLeft() const

/*
Returns the position of the rectangle's top-left corner.

See also setTopLeft(), top(), and left().
*/
impl /*struct*/ QRect {
  pub fn topLeft_0<RetType, T: QRect_topLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topLeft_0(self);
    // return 1;
  }
}
pub trait QRect_topLeft_0<RetType> {
  fn topLeft_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_topLeft_0<usize> for () {
  fn topLeft_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect7topLeftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:90
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint bottomRight() const

/*
Returns the position of the rectangle's bottom-right corner.

Note that for historical reasons this function returns QPoint(left() + width() -1, top() + height() - 1).

See also setBottomRight(), bottom(), and right().
*/
impl /*struct*/ QRect {
  pub fn bottomRight_0<RetType, T: QRect_bottomRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottomRight_0(self);
    // return 1;
  }
}
pub trait QRect_bottomRight_0<RetType> {
  fn bottomRight_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_bottomRight_0<usize> for () {
  fn bottomRight_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect11bottomRightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:91
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint topRight() const

/*
Returns the position of the rectangle's top-right corner.

Note that for historical reasons this function returns QPoint(left() + width() -1, top()).

See also setTopRight(), top(), and right().
*/
impl /*struct*/ QRect {
  pub fn topRight_0<RetType, T: QRect_topRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topRight_0(self);
    // return 1;
  }
}
pub trait QRect_topRight_0<RetType> {
  fn topRight_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_topRight_0<usize> for () {
  fn topRight_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect8topRightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:92
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint bottomLeft() const

/*
Returns the position of the rectangle's bottom-left corner. Note that for historical reasons this function returns QPoint(left(), top() + height() - 1).

See also setBottomLeft(), bottom(), and left().
*/
impl /*struct*/ QRect {
  pub fn bottomLeft_0<RetType, T: QRect_bottomLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottomLeft_0(self);
    // return 1;
  }
}
pub trait QRect_bottomLeft_0<RetType> {
  fn bottomLeft_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_bottomLeft_0<usize> for () {
  fn bottomLeft_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect10bottomLeftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:93
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint center() const

/*
Returns the center point of the rectangle.

See also moveCenter().
*/
impl /*struct*/ QRect {
  pub fn center_0<RetType, T: QRect_center_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.center_0(self);
    // return 1;
  }
}
pub trait QRect_center_0<RetType> {
  fn center_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_center_0<usize> for () {
  fn center_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect6centerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:95
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveLeft(int)

/*
Moves the rectangle horizontally, leaving the rectangle's left edge at the given x coordinate. The rectangle's size is unchanged.

See also left(), setLeft(), and moveRight().
*/
impl /*struct*/ QRect {
  pub fn moveLeft_0<RetType, T: QRect_moveLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveLeft_0(self);
    // return 1;
  }
}
pub trait QRect_moveLeft_0<RetType> {
  fn moveLeft_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_moveLeft_0<(/*void*/)> for (i32) {
  fn moveLeft_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect8moveLeftEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:96
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveTop(int)

/*
Moves the rectangle vertically, leaving the rectangle's top edge at the given y coordinate. The rectangle's size is unchanged.

See also top(), setTop(), and moveBottom().
*/
impl /*struct*/ QRect {
  pub fn moveTop_0<RetType, T: QRect_moveTop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveTop_0(self);
    // return 1;
  }
}
pub trait QRect_moveTop_0<RetType> {
  fn moveTop_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_moveTop_0<(/*void*/)> for (i32) {
  fn moveTop_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect7moveTopEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:97
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveRight(int)

/*
Moves the rectangle horizontally, leaving the rectangle's right edge at the given x coordinate. The rectangle's size is unchanged.

See also right(), setRight(), and moveLeft().
*/
impl /*struct*/ QRect {
  pub fn moveRight_0<RetType, T: QRect_moveRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveRight_0(self);
    // return 1;
  }
}
pub trait QRect_moveRight_0<RetType> {
  fn moveRight_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_moveRight_0<(/*void*/)> for (i32) {
  fn moveRight_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect9moveRightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:98
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveBottom(int)

/*
Moves the rectangle vertically, leaving the rectangle's bottom edge at the given y coordinate. The rectangle's size is unchanged.

See also bottom(), setBottom(), and moveTop().
*/
impl /*struct*/ QRect {
  pub fn moveBottom_0<RetType, T: QRect_moveBottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveBottom_0(self);
    // return 1;
  }
}
pub trait QRect_moveBottom_0<RetType> {
  fn moveBottom_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_moveBottom_0<(/*void*/)> for (i32) {
  fn moveBottom_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect10moveBottomEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:99
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveTopLeft(const QPoint &)

/*
Moves the rectangle, leaving the top-left corner at the given position. The rectangle's size is unchanged.

See also setTopLeft(), moveTop(), and moveLeft().
*/
impl /*struct*/ QRect {
  pub fn moveTopLeft_0<RetType, T: QRect_moveTopLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveTopLeft_0(self);
    // return 1;
  }
}
pub trait QRect_moveTopLeft_0<RetType> {
  fn moveTopLeft_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_moveTopLeft_0<(/*void*/)> for (usize) {
  fn moveTopLeft_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect11moveTopLeftERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:100
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveBottomRight(const QPoint &)

/*
Moves the rectangle, leaving the bottom-right corner at the given position. The rectangle's size is unchanged.

See also setBottomRight(), moveRight(), and moveBottom().
*/
impl /*struct*/ QRect {
  pub fn moveBottomRight_0<RetType, T: QRect_moveBottomRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveBottomRight_0(self);
    // return 1;
  }
}
pub trait QRect_moveBottomRight_0<RetType> {
  fn moveBottomRight_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_moveBottomRight_0<(/*void*/)> for (usize) {
  fn moveBottomRight_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect15moveBottomRightERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:101
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveTopRight(const QPoint &)

/*
Moves the rectangle, leaving the top-right corner at the given position. The rectangle's size is unchanged.

See also setTopRight(), moveTop(), and moveRight().
*/
impl /*struct*/ QRect {
  pub fn moveTopRight_0<RetType, T: QRect_moveTopRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveTopRight_0(self);
    // return 1;
  }
}
pub trait QRect_moveTopRight_0<RetType> {
  fn moveTopRight_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_moveTopRight_0<(/*void*/)> for (usize) {
  fn moveTopRight_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect12moveTopRightERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:102
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveBottomLeft(const QPoint &)

/*
Moves the rectangle, leaving the bottom-left corner at the given position. The rectangle's size is unchanged.

See also setBottomLeft(), moveBottom(), and moveLeft().
*/
impl /*struct*/ QRect {
  pub fn moveBottomLeft_0<RetType, T: QRect_moveBottomLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveBottomLeft_0(self);
    // return 1;
  }
}
pub trait QRect_moveBottomLeft_0<RetType> {
  fn moveBottomLeft_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_moveBottomLeft_0<(/*void*/)> for (usize) {
  fn moveBottomLeft_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect14moveBottomLeftERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:103
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveCenter(const QPoint &)

/*
Moves the rectangle, leaving the center point at the given position. The rectangle's size is unchanged.

See also center().
*/
impl /*struct*/ QRect {
  pub fn moveCenter_0<RetType, T: QRect_moveCenter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveCenter_0(self);
    // return 1;
  }
}
pub trait QRect_moveCenter_0<RetType> {
  fn moveCenter_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_moveCenter_0<(/*void*/)> for (usize) {
  fn moveCenter_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect10moveCenterERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:105
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void translate(int, int)

/*
Moves the rectangle dx along the x axis and dy along the y axis, relative to the current position. Positive values move the rectangle to the right and down.

See also moveTopLeft(), moveTo(), and translated().
*/
impl /*struct*/ QRect {
  pub fn translate_0<RetType, T: QRect_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QRect_translate_0<RetType> {
  fn translate_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_translate_0<(/*void*/)> for (i32,i32) {
  fn translate_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect9translateEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:106
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void translate(const QPoint &)

/*
Moves the rectangle dx along the x axis and dy along the y axis, relative to the current position. Positive values move the rectangle to the right and down.

See also moveTopLeft(), moveTo(), and translated().
*/
impl /*struct*/ QRect {
  pub fn translate_1<RetType, T: QRect_translate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_1(self);
    // return 1;
  }
}
pub trait QRect_translate_1<RetType> {
  fn translate_1(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_translate_1<(/*void*/)> for (usize) {
  fn translate_1(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect9translateERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:107
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect translated(int, int) const

/*
Returns a copy of the rectangle that is translated dx along the x axis and dy along the y axis, relative to the current position. Positive values move the rectangle to the right and down.

See also translate().
*/
impl /*struct*/ QRect {
  pub fn translated_0<RetType, T: QRect_translated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_0(self);
    // return 1;
  }
}
pub trait QRect_translated_0<RetType> {
  fn translated_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_translated_0<usize> for (i32,i32) {
  fn translated_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect10translatedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:108
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QRect translated(const QPoint &) const

/*
Returns a copy of the rectangle that is translated dx along the x axis and dy along the y axis, relative to the current position. Positive values move the rectangle to the right and down.

See also translate().
*/
impl /*struct*/ QRect {
  pub fn translated_1<RetType, T: QRect_translated_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_1(self);
    // return 1;
  }
}
pub trait QRect_translated_1<RetType> {
  fn translated_1(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_translated_1<usize> for (usize) {
  fn translated_1(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect10translatedERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:109
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect transposed() const

/*
Returns a copy of the rectangle that has its width and height exchanged:


  QRect r = {15, 51, 42, 24};
  r = r.transposed(); // r == {15, 51, 24, 42}



This function was introduced in  Qt 5.7.

See also QSize::transposed().
*/
impl /*struct*/ QRect {
  pub fn transposed_0<RetType, T: QRect_transposed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transposed_0(self);
    // return 1;
  }
}
pub trait QRect_transposed_0<RetType> {
  fn transposed_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_transposed_0<usize> for () {
  fn transposed_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect10transposedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:111
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void moveTo(int, int)

/*
Moves the rectangle, leaving the top-left corner at the given position (x, y). The rectangle's size is unchanged.

See also translate() and moveTopLeft().
*/
impl /*struct*/ QRect {
  pub fn moveTo_0<RetType, T: QRect_moveTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveTo_0(self);
    // return 1;
  }
}
pub trait QRect_moveTo_0<RetType> {
  fn moveTo_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_moveTo_0<(/*void*/)> for (i32,i32) {
  fn moveTo_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect6moveToEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:112
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void moveTo(const QPoint &)

/*
Moves the rectangle, leaving the top-left corner at the given position (x, y). The rectangle's size is unchanged.

See also translate() and moveTopLeft().
*/
impl /*struct*/ QRect {
  pub fn moveTo_1<RetType, T: QRect_moveTo_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveTo_1(self);
    // return 1;
  }
}
pub trait QRect_moveTo_1<RetType> {
  fn moveTo_1(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_moveTo_1<(/*void*/)> for (usize) {
  fn moveTo_1(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect6moveToERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:114
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setRect(int, int, int, int)

/*
Sets the coordinates of the rectangle's top-left corner to (x, y), and its size to the given width and height.

See also getRect() and setCoords().
*/
impl /*struct*/ QRect {
  pub fn setRect_0<RetType, T: QRect_setRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRect_0(self);
    // return 1;
  }
}
pub trait QRect_setRect_0<RetType> {
  fn setRect_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setRect_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn setRect_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect7setRectEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:115
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void getRect(int *, int *, int *, int *) const

/*
Extracts the position of the rectangle's top-left corner to *x and *y, and its dimensions to *width and *height.

See also setRect() and getCoords().
*/
impl /*struct*/ QRect {
  pub fn getRect_0<RetType, T: QRect_getRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getRect_0(self);
    // return 1;
  }
}
pub trait QRect_getRect_0<RetType> {
  fn getRect_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_getRect_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getRect_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK5QRect7getRectEPiS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:117
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setCoords(int, int, int, int)

/*
Sets the coordinates of the rectangle's top-left corner to (x1, y1), and the coordinates of its bottom-right corner to (x2, y2).

See also getCoords() and setRect().
*/
impl /*struct*/ QRect {
  pub fn setCoords_0<RetType, T: QRect_setCoords_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCoords_0(self);
    // return 1;
  }
}
pub trait QRect_setCoords_0<RetType> {
  fn setCoords_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setCoords_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn setCoords_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect9setCoordsEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:118
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void getCoords(int *, int *, int *, int *) const

/*
Extracts the position of the rectangle's top-left corner to *x1 and *y1, and the position of the bottom-right corner to *x2 and *y2.

See also setCoords() and getRect().
*/
impl /*struct*/ QRect {
  pub fn getCoords_0<RetType, T: QRect_getCoords_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getCoords_0(self);
    // return 1;
  }
}
pub trait QRect_getCoords_0<RetType> {
  fn getCoords_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_getCoords_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getCoords_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK5QRect9getCoordsEPiS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:120
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void adjust(int, int, int, int)

/*
Adds dx1, dy1, dx2 and dy2 respectively to the existing coordinates of the rectangle.

See also adjusted() and setRect().
*/
impl /*struct*/ QRect {
  pub fn adjust_0<RetType, T: QRect_adjust_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.adjust_0(self);
    // return 1;
  }
}
pub trait QRect_adjust_0<RetType> {
  fn adjust_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_adjust_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn adjust_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect6adjustEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:121
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect adjusted(int, int, int, int) const

/*
Returns a new rectangle with dx1, dy1, dx2 and dy2 added respectively to the existing coordinates of this rectangle.

See also adjust().
*/
impl /*struct*/ QRect {
  pub fn adjusted_0<RetType, T: QRect_adjusted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.adjusted_0(self);
    // return 1;
  }
}
pub trait QRect_adjusted_0<RetType> {
  fn adjusted_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_adjusted_0<usize> for (i32,i32,i32,i32) {
  fn adjusted_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect8adjustedEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:123
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSize size() const

/*
Returns the size of the rectangle.

See also setSize(), width(), and height().
*/
impl /*struct*/ QRect {
  pub fn size_0<RetType, T: QRect_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QRect_size_0<RetType> {
  fn size_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_size_0<usize> for () {
  fn size_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:124
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int width() const

/*
Returns the width of the rectangle.

See also setWidth(), height(), and size().
*/
impl /*struct*/ QRect {
  pub fn width_0<RetType, T: QRect_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QRect_width_0<RetType> {
  fn width_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_width_0<i32> for () {
  fn width_0(self , rsthis: & QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:125
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int height() const

/*
Returns the height of the rectangle.

See also setHeight(), width(), and size().
*/
impl /*struct*/ QRect {
  pub fn height_0<RetType, T: QRect_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QRect_height_0<RetType> {
  fn height_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_height_0<i32> for () {
  fn height_0(self , rsthis: & QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:126
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setWidth(int)

/*
Sets the width of the rectangle to the given width. The right edge is changed, but not the left one.

See also width() and setSize().
*/
impl /*struct*/ QRect {
  pub fn setWidth_0<RetType, T: QRect_setWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidth_0(self);
    // return 1;
  }
}
pub trait QRect_setWidth_0<RetType> {
  fn setWidth_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setWidth_0<(/*void*/)> for (i32) {
  fn setWidth_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect8setWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:127
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setHeight(int)

/*
Sets the height of the rectangle to the given height. The bottom edge is changed, but not the top one.

See also height() and setSize().
*/
impl /*struct*/ QRect {
  pub fn setHeight_0<RetType, T: QRect_setHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeight_0(self);
    // return 1;
  }
}
pub trait QRect_setHeight_0<RetType> {
  fn setHeight_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setHeight_0<(/*void*/)> for (i32) {
  fn setHeight_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect9setHeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:128
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setSize(const QSize &)

/*
Sets the size of the rectangle to the given size. The top-left corner is not moved.

See also size(), setWidth(), and setHeight().
*/
impl /*struct*/ QRect {
  pub fn setSize_0<RetType, T: QRect_setSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSize_0(self);
    // return 1;
  }
}
pub trait QRect_setSize_0<RetType> {
  fn setSize_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_setSize_0<(/*void*/)> for (usize) {
  fn setSize_0(self , rsthis: & QRect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QRect7setSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrect.h:130
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect operator|(const QRect &) const

/*

*/
impl /*struct*/ QRect {
  pub fn operator_or_0<RetType, T: QRect_operator_or_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_or_0(self);
    // return 1;
  }
}
pub trait QRect_operator_or_0<RetType> {
  fn operator_or_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_operator_or_0<usize> for (usize) {
  fn operator_or_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRectorERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:131
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect operator&(const QRect &) const

/*

*/
impl /*struct*/ QRect {
  pub fn operator_and_0<RetType, T: QRect_operator_and_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_and_0(self);
    // return 1;
  }
}
pub trait QRect_operator_and_0<RetType> {
  fn operator_and_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_operator_and_0<usize> for (usize) {
  fn operator_and_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRectanERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:132
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect & operator|=(const QRect &)

/*

*/
impl /*struct*/ QRect {
  pub fn operator_or_equal_0<RetType, T: QRect_operator_or_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_or_equal_0(self);
    // return 1;
  }
}
pub trait QRect_operator_or_equal_0<RetType> {
  fn operator_or_equal_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_operator_or_equal_0<usize> for (usize) {
  fn operator_or_equal_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QRectoRERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:133
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect & operator&=(const QRect &)

/*

*/
impl /*struct*/ QRect {
  pub fn operator_and_equal_0<RetType, T: QRect_operator_and_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_and_equal_0(self);
    // return 1;
  }
}
pub trait QRect_operator_and_equal_0<RetType> {
  fn operator_and_equal_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_operator_and_equal_0<usize> for (usize) {
  fn operator_and_equal_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QRectaNERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:135
// index:0
// Public Visibility=Default Availability=Available
// [1] bool contains(const QRect &, bool) const

/*
Returns true if the given point is inside or on the edge of the rectangle, otherwise returns false. If proper is true, this function only returns true if the given point is inside the rectangle (i.e., not on the edge).

See also intersects().
*/
impl /*struct*/ QRect {
  pub fn contains_0<RetType, T: QRect_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QRect_contains_0<RetType> {
  fn contains_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_contains_0<bool> for (usize,bool) {
  fn contains_0(self , rsthis: & QRect) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect8containsERKS_b", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:136
// index:1
// Public Visibility=Default Availability=Available
// [1] bool contains(const QPoint &, bool) const

/*
Returns true if the given point is inside or on the edge of the rectangle, otherwise returns false. If proper is true, this function only returns true if the given point is inside the rectangle (i.e., not on the edge).

See also intersects().
*/
impl /*struct*/ QRect {
  pub fn contains_1<RetType, T: QRect_contains_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_1(self);
    // return 1;
  }
}
pub trait QRect_contains_1<RetType> {
  fn contains_1(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_contains_1<bool> for (usize,bool) {
  fn contains_1(self , rsthis: & QRect) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect8containsERK6QPointb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:137
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool contains(int, int) const

/*
Returns true if the given point is inside or on the edge of the rectangle, otherwise returns false. If proper is true, this function only returns true if the given point is inside the rectangle (i.e., not on the edge).

See also intersects().
*/
impl /*struct*/ QRect {
  pub fn contains_2<RetType, T: QRect_contains_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_2(self);
    // return 1;
  }
}
pub trait QRect_contains_2<RetType> {
  fn contains_2(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_contains_2<bool> for (i32,i32) {
  fn contains_2(self , rsthis: & QRect) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect8containsEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:138
// index:3
// Public inline Visibility=Default Availability=Available
// [1] bool contains(int, int, bool) const

/*
Returns true if the given point is inside or on the edge of the rectangle, otherwise returns false. If proper is true, this function only returns true if the given point is inside the rectangle (i.e., not on the edge).

See also intersects().
*/
impl /*struct*/ QRect {
  pub fn contains_3<RetType, T: QRect_contains_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_3(self);
    // return 1;
  }
}
pub trait QRect_contains_3<RetType> {
  fn contains_3(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_contains_3<bool> for (i32,i32,bool) {
  fn contains_3(self , rsthis: & QRect) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect8containsEiib", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:139
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect united(const QRect &) const

/*
Returns the bounding rectangle of this rectangle and the given rectangle.



This function was introduced in  Qt 4.2.

See also intersected().
*/
impl /*struct*/ QRect {
  pub fn united_0<RetType, T: QRect_united_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.united_0(self);
    // return 1;
  }
}
pub trait QRect_united_0<RetType> {
  fn united_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_united_0<usize> for (usize) {
  fn united_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect6unitedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:140
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect intersected(const QRect &) const

/*
Returns the intersection of this rectangle and the given rectangle. Note that r.intersected(s) is equivalent to r & s.



This function was introduced in  Qt 4.2.

See also intersects(), united(), and operator&=().
*/
impl /*struct*/ QRect {
  pub fn intersected_0<RetType, T: QRect_intersected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersected_0(self);
    // return 1;
  }
}
pub trait QRect_intersected_0<RetType> {
  fn intersected_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_intersected_0<usize> for (usize) {
  fn intersected_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect11intersectedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:141
// index:0
// Public Visibility=Default Availability=Available
// [1] bool intersects(const QRect &) const

/*
Returns true if this rectangle intersects with the given rectangle (i.e., there is at least one pixel that is within both rectangles), otherwise returns false.

The intersection rectangle can be retrieved using the intersected() function.

See also contains().
*/
impl /*struct*/ QRect {
  pub fn intersects_0<RetType, T: QRect_intersects_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersects_0(self);
    // return 1;
  }
}
pub trait QRect_intersects_0<RetType> {
  fn intersects_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_intersects_0<bool> for (usize) {
  fn intersects_0(self , rsthis: & QRect) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect10intersectsERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:143
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect marginsAdded(const QMargins &) const

/*
Returns a rectangle grown by the margins.

This function was introduced in  Qt 5.1.

See also operator+=(), marginsRemoved(), and operator-=().
*/
impl /*struct*/ QRect {
  pub fn marginsAdded_0<RetType, T: QRect_marginsAdded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.marginsAdded_0(self);
    // return 1;
  }
}
pub trait QRect_marginsAdded_0<RetType> {
  fn marginsAdded_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_marginsAdded_0<usize> for (usize) {
  fn marginsAdded_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect12marginsAddedERK8QMargins", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:144
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect marginsRemoved(const QMargins &) const

/*
Removes the margins from the rectangle, shrinking it.

This function was introduced in  Qt 5.1.

See also marginsAdded(), operator+=(), and operator-=().
*/
impl /*struct*/ QRect {
  pub fn marginsRemoved_0<RetType, T: QRect_marginsRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.marginsRemoved_0(self);
    // return 1;
  }
}
pub trait QRect_marginsRemoved_0<RetType> {
  fn marginsRemoved_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_marginsRemoved_0<usize> for (usize) {
  fn marginsRemoved_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QRect14marginsRemovedERK8QMargins", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:145
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect & operator+=(const QMargins &)

/*

*/
impl /*struct*/ QRect {
  pub fn operator_add_equal_0<RetType, T: QRect_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QRect_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QRectpLERK8QMargins", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrect.h:146
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect & operator-=(const QMargins &)

/*

*/
impl /*struct*/ QRect {
  pub fn operator_minus_equal_0<RetType, T: QRect_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QRect_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QRect) -> RetType;
}
impl<'a> /*trait*/ QRect_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QRect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QRectmIERK8QMargins", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQRect(this :*mut QRect) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN5QRectD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
