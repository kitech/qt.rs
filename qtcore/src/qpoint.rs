

// mod ::core::QPoint
// package qtcore
// /usr/include/qt/QtCore/qpoint.h
// #include <qpoint.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 19
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
#[derive(Default)] // class sizeof(QPoint)=8
pub struct QPoint {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPoint_ITF interface {
//    QPoint_PTR() *QPoint
//}
//func (ptr *QPoint) QPoint_PTR() *QPoint { return ptr }

impl /*struct*/ QPoint {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPoint {
    return QPoint{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPoint {
//  type Target = QPointBASE;
//
//  fn deref(&self) -> &QPointBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPointBASE> for QPoint {
//  fn as_ref(& self) -> & QPointBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qpoint.h:55
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QPoint()

/*
Constructs a null point, i.e. with coordinates (0, 0)

See also isNull().
*/
// QPoint() ctx.fn_proto_cpp
impl /*struct*/ QPoint {
  pub fn QPoint_0<T: QPoint_QPoint_0>(value: T) -> QPoint {
    let rsthis = value.QPoint_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPoint_QPoint_0 {
  fn QPoint_0(self) -> QPoint;
}
// QPoint() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPoint_QPoint_0 for () {
  fn QPoint_0(self) -> QPoint {
    // unsafe{_ZN6QPointC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QPointC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPoint{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpoint.h:56
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QPoint(int, int)

/*
Constructs a null point, i.e. with coordinates (0, 0)

See also isNull().
*/
// QPoint(int, int) ctx.fn_proto_cpp
impl /*struct*/ QPoint {
  pub fn QPoint_1<T: QPoint_QPoint_1>(value: T) -> QPoint {
    let rsthis = value.QPoint_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPoint_QPoint_1 {
  fn QPoint_1(self) -> QPoint;
}
// QPoint(int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPoint_QPoint_1 for (i32,i32) {
  fn QPoint_1(self) -> QPoint {
    // unsafe{_ZN6QPointC2Eii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QPointC2Eii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPoint{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpoint.h:58
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if both the x and y coordinates are set to 0, otherwise returns false.
*/
impl /*struct*/ QPoint {
  pub fn isNull_0<RetType, T: QPoint_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QPoint_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QPoint) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QPoint6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:60
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int x() const

/*
Returns the x coordinate of this point.

See also setX() and rx().
*/
impl /*struct*/ QPoint {
  pub fn x_0<RetType, T: QPoint_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QPoint_x_0<RetType> {
  fn x_0(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_x_0<i32> for () {
  fn x_0(self , rsthis: & QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QPoint1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:61
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int y() const

/*
Returns the y coordinate of this point.

See also setY() and ry().
*/
impl /*struct*/ QPoint {
  pub fn y_0<RetType, T: QPoint_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QPoint_y_0<RetType> {
  fn y_0(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_y_0<i32> for () {
  fn y_0(self , rsthis: & QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QPoint1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:62
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setX(int)

/*
Sets the x coordinate of this point to the given x coordinate.

See also x() and setY().
*/
impl /*struct*/ QPoint {
  pub fn setX_0<RetType, T: QPoint_setX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setX_0(self);
    // return 1;
  }
}
pub trait QPoint_setX_0<RetType> {
  fn setX_0(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_setX_0<(/*void*/)> for (i32) {
  fn setX_0(self , rsthis: & QPoint) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QPoint4setXEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpoint.h:63
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setY(int)

/*
Sets the y coordinate of this point to the given y coordinate.

See also y() and setX().
*/
impl /*struct*/ QPoint {
  pub fn setY_0<RetType, T: QPoint_setY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setY_0(self);
    // return 1;
  }
}
pub trait QPoint_setY_0<RetType> {
  fn setY_0(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_setY_0<(/*void*/)> for (i32) {
  fn setY_0(self , rsthis: & QPoint) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QPoint4setYEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpoint.h:65
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int manhattanLength() const

/*
Returns the sum of the absolute values of x() and y(), traditionally known as the "Manhattan length" of the vector from the origin to the point. For example:


  QPoint oldPosition;

  MyWidget::mouseMoveEvent(QMouseEvent *event)
  {
      QPoint point = event->pos() - oldPosition;
      if (point.manhattanLength() > 3)
          // the mouse has moved more than 3 pixels since the oldPosition
  }



This is a useful, and quick to calculate, approximation to the true length:


  double trueLength = std::sqrt(std::pow(x(), 2) + std::pow(y(), 2));



The tradition of "Manhattan length" arises because such distances apply to travelers who can only travel on a rectangular grid, like the streets of Manhattan.
*/
impl /*struct*/ QPoint {
  pub fn manhattanLength_0<RetType, T: QPoint_manhattanLength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.manhattanLength_0(self);
    // return 1;
  }
}
pub trait QPoint_manhattanLength_0<RetType> {
  fn manhattanLength_0(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_manhattanLength_0<i32> for () {
  fn manhattanLength_0(self , rsthis: & QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QPoint15manhattanLengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:67
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int & rx()

/*
Returns a reference to the x coordinate of this point.

Using a reference makes it possible to directly manipulate x. For example:


  QPoint p(1, 2);
  p.rx()--;   // p becomes (0, 2)



See also x() and setX().
*/
impl /*struct*/ QPoint {
  pub fn rx_0<RetType, T: QPoint_rx_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rx_0(self);
    // return 1;
  }
}
pub trait QPoint_rx_0<RetType> {
  fn rx_0(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_rx_0<usize> for () {
  fn rx_0(self , rsthis: & QPoint) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QPoint2rxEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:68
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int & ry()

/*
Returns a reference to the y coordinate of this point.

Using a reference makes it possible to directly manipulate y. For example:


  QPoint p(1, 2);
  p.ry()++;   // p becomes (1, 3)



See also y() and setY().
*/
impl /*struct*/ QPoint {
  pub fn ry_0<RetType, T: QPoint_ry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ry_0(self);
    // return 1;
  }
}
pub trait QPoint_ry_0<RetType> {
  fn ry_0(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_ry_0<usize> for () {
  fn ry_0(self , rsthis: & QPoint) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QPoint2ryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:70
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint & operator+=(const QPoint &)

/*

*/
impl /*struct*/ QPoint {
  pub fn operator_add_equal_0<RetType, T: QPoint_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QPoint_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QPoint) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QPointpLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:71
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint & operator-=(const QPoint &)

/*

*/
impl /*struct*/ QPoint {
  pub fn operator_minus_equal_0<RetType, T: QPoint_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QPoint_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QPoint) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QPointmIERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:73
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint & operator*=(float)

/*

*/
impl /*struct*/ QPoint {
  pub fn operator_mul_equal_0<RetType, T: QPoint_operator_mul_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_0(self);
    // return 1;
  }
}
pub trait QPoint_operator_mul_equal_0<RetType> {
  fn operator_mul_equal_0(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_operator_mul_equal_0<usize> for (f32) {
  fn operator_mul_equal_0(self , rsthis: & QPoint) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QPointmLEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:74
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QPoint & operator*=(double)

/*

*/
impl /*struct*/ QPoint {
  pub fn operator_mul_equal_1<RetType, T: QPoint_operator_mul_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_1(self);
    // return 1;
  }
}
pub trait QPoint_operator_mul_equal_1<RetType> {
  fn operator_mul_equal_1(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_operator_mul_equal_1<usize> for (f64) {
  fn operator_mul_equal_1(self , rsthis: & QPoint) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QPointmLEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:75
// index:2
// Public inline Visibility=Default Availability=Available
// [8] QPoint & operator*=(int)

/*

*/
impl /*struct*/ QPoint {
  pub fn operator_mul_equal_2<RetType, T: QPoint_operator_mul_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_2(self);
    // return 1;
  }
}
pub trait QPoint_operator_mul_equal_2<RetType> {
  fn operator_mul_equal_2(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_operator_mul_equal_2<usize> for (i32) {
  fn operator_mul_equal_2(self , rsthis: & QPoint) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QPointmLEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:77
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint & operator/=(qreal)

/*

*/
impl /*struct*/ QPoint {
  pub fn operator_div_equal_0<RetType, T: QPoint_operator_div_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_0(self);
    // return 1;
  }
}
pub trait QPoint_operator_div_equal_0<RetType> {
  fn operator_div_equal_0(self , rsthis: & QPoint) -> RetType;
}
impl<'a> /*trait*/ QPoint_operator_div_equal_0<usize> for (f64) {
  fn operator_div_equal_0(self , rsthis: & QPoint) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QPointdVEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:79
// index:0
// Public static inline Visibility=Default Availability=Available
// [4] int dotProduct(const QPoint &, const QPoint &)

/*
QPoint p( 3, 7);
  QPoint q(-1, 4);
  int lengthSquared = QPoint::dotProduct(p, q);   // lengthSquared becomes 25



Returns the dot product of p1 and p2.

This function was introduced in  Qt 5.1.
*/
impl /*struct*/ QPoint {
  pub fn dotProduct_0<RetType, T: QPoint_dotProduct_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.dotProduct_0();
    // return 1;
  }
}
pub trait QPoint_dotProduct_0<RetType> {
  fn dotProduct_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPoint_dotProduct_0<i32> for (usize,usize) {
  fn dotProduct_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QPoint10dotProductERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQPoint(this :*mut QPoint) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN6QPointD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
