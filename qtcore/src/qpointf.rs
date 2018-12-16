

// mod ::core::QPointF
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
// extern C begin: 17
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
#[derive(Default)] // class sizeof(QPointF)=16
pub struct QPointF {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPointF_ITF interface {
//    QPointF_PTR() *QPointF
//}
//func (ptr *QPointF) QPointF_PTR() *QPointF { return ptr }

impl /*struct*/ QPointF {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPointF {
    return QPointF{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPointF {
//  type Target = QPointFBASE;
//
//  fn deref(&self) -> &QPointFBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPointFBASE> for QPointF {
//  fn as_ref(& self) -> & QPointFBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qpoint.h:222
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QPointF()

/*

*/
// QPointF() ctx.fn_proto_cpp
impl /*struct*/ QPointF {
  pub fn QPointF_0<T: QPointF_QPointF_0>(value: T) -> QPointF {
    let rsthis = value.QPointF_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPointF_QPointF_0 {
  fn QPointF_0(self) -> QPointF;
}
// QPointF() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPointF_QPointF_0 for () {
  fn QPointF_0(self) -> QPointF {
    // unsafe{_ZN7QPointFC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QPointFC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPointF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpoint.h:223
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QPointF(const QPoint &)

/*

*/
// QPointF(const QPoint &) ctx.fn_proto_cpp
impl /*struct*/ QPointF {
  pub fn QPointF_1<T: QPointF_QPointF_1>(value: T) -> QPointF {
    let rsthis = value.QPointF_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPointF_QPointF_1 {
  fn QPointF_1(self) -> QPointF;
}
// QPointF(const QPoint &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPointF_QPointF_1 for (usize) {
  fn QPointF_1(self) -> QPointF {
    // unsafe{_ZN7QPointFC2ERK6QPoint()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QPointFC2ERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPointF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpoint.h:224
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QPointF(qreal, qreal)

/*

*/
// QPointF(qreal, qreal) ctx.fn_proto_cpp
impl /*struct*/ QPointF {
  pub fn QPointF_2<T: QPointF_QPointF_2>(value: T) -> QPointF {
    let rsthis = value.QPointF_2();
    return rsthis;
    // return 1;
  }
}

pub trait QPointF_QPointF_2 {
  fn QPointF_2(self) -> QPointF;
}
// QPointF(qreal, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPointF_QPointF_2 for (f64,f64) {
  fn QPointF_2(self) -> QPointF {
    // unsafe{_ZN7QPointFC2Edd()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QPointFC2Edd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPointF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpoint.h:226
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal manhattanLength() const

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
impl /*struct*/ QPointF {
  pub fn manhattanLength_0<RetType, T: QPointF_manhattanLength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.manhattanLength_0(self);
    // return 1;
  }
}
pub trait QPointF_manhattanLength_0<RetType> {
  fn manhattanLength_0(self , rsthis: & QPointF) -> RetType;
}
impl<'a> /*trait*/ QPointF_manhattanLength_0<f64> for () {
  fn manhattanLength_0(self , rsthis: & QPointF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPointF15manhattanLengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:228
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if both the x and y coordinates are set to 0, otherwise returns false.
*/
impl /*struct*/ QPointF {
  pub fn isNull_0<RetType, T: QPointF_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QPointF_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QPointF) -> RetType;
}
impl<'a> /*trait*/ QPointF_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QPointF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPointF6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:230
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal x() const

/*
Returns the x coordinate of this point.

See also setX() and rx().
*/
impl /*struct*/ QPointF {
  pub fn x_0<RetType, T: QPointF_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QPointF_x_0<RetType> {
  fn x_0(self , rsthis: & QPointF) -> RetType;
}
impl<'a> /*trait*/ QPointF_x_0<f64> for () {
  fn x_0(self , rsthis: & QPointF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPointF1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:231
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal y() const

/*
Returns the y coordinate of this point.

See also setY() and ry().
*/
impl /*struct*/ QPointF {
  pub fn y_0<RetType, T: QPointF_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QPointF_y_0<RetType> {
  fn y_0(self , rsthis: & QPointF) -> RetType;
}
impl<'a> /*trait*/ QPointF_y_0<f64> for () {
  fn y_0(self , rsthis: & QPointF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPointF1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:232
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setX(qreal)

/*
Sets the x coordinate of this point to the given x coordinate.

See also x() and setY().
*/
impl /*struct*/ QPointF {
  pub fn setX_0<RetType, T: QPointF_setX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setX_0(self);
    // return 1;
  }
}
pub trait QPointF_setX_0<RetType> {
  fn setX_0(self , rsthis: & QPointF) -> RetType;
}
impl<'a> /*trait*/ QPointF_setX_0<(/*void*/)> for (f64) {
  fn setX_0(self , rsthis: & QPointF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN7QPointF4setXEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpoint.h:233
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setY(qreal)

/*
Sets the y coordinate of this point to the given y coordinate.

See also y() and setX().
*/
impl /*struct*/ QPointF {
  pub fn setY_0<RetType, T: QPointF_setY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setY_0(self);
    // return 1;
  }
}
pub trait QPointF_setY_0<RetType> {
  fn setY_0(self , rsthis: & QPointF) -> RetType;
}
impl<'a> /*trait*/ QPointF_setY_0<(/*void*/)> for (f64) {
  fn setY_0(self , rsthis: & QPointF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN7QPointF4setYEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpoint.h:235
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal & rx()

/*
Returns a reference to the x coordinate of this point.

Using a reference makes it possible to directly manipulate x. For example:


  QPoint p(1, 2);
  p.rx()--;   // p becomes (0, 2)



See also x() and setX().
*/
impl /*struct*/ QPointF {
  pub fn rx_0<RetType, T: QPointF_rx_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rx_0(self);
    // return 1;
  }
}
pub trait QPointF_rx_0<RetType> {
  fn rx_0(self , rsthis: & QPointF) -> RetType;
}
impl<'a> /*trait*/ QPointF_rx_0<usize> for () {
  fn rx_0(self , rsthis: & QPointF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPointF2rxEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:236
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal & ry()

/*
Returns a reference to the y coordinate of this point.

Using a reference makes it possible to directly manipulate y. For example:


  QPoint p(1, 2);
  p.ry()++;   // p becomes (1, 3)



See also y() and setY().
*/
impl /*struct*/ QPointF {
  pub fn ry_0<RetType, T: QPointF_ry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ry_0(self);
    // return 1;
  }
}
pub trait QPointF_ry_0<RetType> {
  fn ry_0(self , rsthis: & QPointF) -> RetType;
}
impl<'a> /*trait*/ QPointF_ry_0<usize> for () {
  fn ry_0(self , rsthis: & QPointF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPointF2ryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:238
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF & operator+=(const QPointF &)

/*

*/
impl /*struct*/ QPointF {
  pub fn operator_add_equal_0<RetType, T: QPointF_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QPointF_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QPointF) -> RetType;
}
impl<'a> /*trait*/ QPointF_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QPointF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPointFpLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:239
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF & operator-=(const QPointF &)

/*

*/
impl /*struct*/ QPointF {
  pub fn operator_minus_equal_0<RetType, T: QPointF_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QPointF_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QPointF) -> RetType;
}
impl<'a> /*trait*/ QPointF_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QPointF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPointFmIERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:240
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF & operator*=(qreal)

/*

*/
impl /*struct*/ QPointF {
  pub fn operator_mul_equal_0<RetType, T: QPointF_operator_mul_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_0(self);
    // return 1;
  }
}
pub trait QPointF_operator_mul_equal_0<RetType> {
  fn operator_mul_equal_0(self , rsthis: & QPointF) -> RetType;
}
impl<'a> /*trait*/ QPointF_operator_mul_equal_0<usize> for (f64) {
  fn operator_mul_equal_0(self , rsthis: & QPointF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPointFmLEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:241
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF & operator/=(qreal)

/*

*/
impl /*struct*/ QPointF {
  pub fn operator_div_equal_0<RetType, T: QPointF_operator_div_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_0(self);
    // return 1;
  }
}
pub trait QPointF_operator_div_equal_0<RetType> {
  fn operator_div_equal_0(self , rsthis: & QPointF) -> RetType;
}
impl<'a> /*trait*/ QPointF_operator_div_equal_0<usize> for (f64) {
  fn operator_div_equal_0(self , rsthis: & QPointF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPointFdVEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:243
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] qreal dotProduct(const QPointF &, const QPointF &)

/*
QPoint p( 3, 7);
  QPoint q(-1, 4);
  int lengthSquared = QPoint::dotProduct(p, q);   // lengthSquared becomes 25



Returns the dot product of p1 and p2.

This function was introduced in  Qt 5.1.
*/
impl /*struct*/ QPointF {
  pub fn dotProduct_0<RetType, T: QPointF_dotProduct_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.dotProduct_0();
    // return 1;
  }
}
pub trait QPointF_dotProduct_0<RetType> {
  fn dotProduct_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPointF_dotProduct_0<f64> for (usize,usize) {
  fn dotProduct_0(self ) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPointF10dotProductERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpoint.h:256
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint toPoint() const

/*

*/
impl /*struct*/ QPointF {
  pub fn toPoint_0<RetType, T: QPointF_toPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPoint_0(self);
    // return 1;
  }
}
pub trait QPointF_toPoint_0<RetType> {
  fn toPoint_0(self , rsthis: & QPointF) -> RetType;
}
impl<'a> /*trait*/ QPointF_toPoint_0<usize> for () {
  fn toPoint_0(self , rsthis: & QPointF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPointF7toPointEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQPointF(this :*mut QPointF) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN7QPointFD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
