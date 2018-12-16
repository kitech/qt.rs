

// mod ::gui::QBrush
// package qtgui
// /usr/include/qt/QtGui/qbrush.h
// #include <qbrush.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 61
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
#[derive(Default)] // class sizeof(QBrush)=8
pub struct QBrush {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QBrush_ITF interface {
//    QBrush_PTR() *QBrush
//}
//func (ptr *QBrush) QBrush_PTR() *QBrush { return ptr }

impl /*struct*/ QBrush {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QBrush {
    return QBrush{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QBrush {
//  type Target = QBrushBASE;
//
//  fn deref(&self) -> &QBrushBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QBrushBASE> for QBrush {
//  fn as_ref(& self) -> & QBrushBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qbrush.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QBrush()

/*
Constructs a default black brush with the style Qt::NoBrush (i.e. this brush will not fill shapes).
*/
// QBrush() ctx.fn_proto_cpp
impl /*struct*/ QBrush {
  pub fn QBrush_0<T: QBrush_QBrush_0>(value: T) -> QBrush {
    let rsthis = value.QBrush_0();
    return rsthis;
    // return 1;
  }
}

pub trait QBrush_QBrush_0 {
  fn QBrush_0(self) -> QBrush;
}
// QBrush() ctx.fn_proto_cpp
impl<'a> /*trait*/ QBrush_QBrush_0 for () {
  fn QBrush_0(self) -> QBrush {
    // unsafe{_ZN6QBrushC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QBrushC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:67
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QBrush(Qt::BrushStyle)

/*
Constructs a default black brush with the style Qt::NoBrush (i.e. this brush will not fill shapes).
*/
// QBrush(Qt::BrushStyle) ctx.fn_proto_cpp
impl /*struct*/ QBrush {
  pub fn QBrush_1<T: QBrush_QBrush_1>(value: T) -> QBrush {
    let rsthis = value.QBrush_1();
    return rsthis;
    // return 1;
  }
}

pub trait QBrush_QBrush_1 {
  fn QBrush_1(self) -> QBrush;
}
// QBrush(Qt::BrushStyle) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBrush_QBrush_1 for (i32) {
  fn QBrush_1(self) -> QBrush {
    // unsafe{_ZN6QBrushC2EN2Qt10BrushStyleE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QBrushC2EN2Qt10BrushStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:68
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QBrush(const QColor &, Qt::BrushStyle)

/*
Constructs a default black brush with the style Qt::NoBrush (i.e. this brush will not fill shapes).
*/
// QBrush(const QColor &, Qt::BrushStyle) ctx.fn_proto_cpp
impl /*struct*/ QBrush {
  pub fn QBrush_2<T: QBrush_QBrush_2>(value: T) -> QBrush {
    let rsthis = value.QBrush_2();
    return rsthis;
    // return 1;
  }
}

pub trait QBrush_QBrush_2 {
  fn QBrush_2(self) -> QBrush;
}
// QBrush(const QColor &, Qt::BrushStyle) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBrush_QBrush_2 for (usize,i32) {
  fn QBrush_2(self) -> QBrush {
    // unsafe{_ZN6QBrushC2ERK6QColorN2Qt10BrushStyleE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QBrushC2ERK6QColorN2Qt10BrushStyleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:69
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QBrush(Qt::GlobalColor, Qt::BrushStyle)

/*
Constructs a default black brush with the style Qt::NoBrush (i.e. this brush will not fill shapes).
*/
// QBrush(Qt::GlobalColor, Qt::BrushStyle) ctx.fn_proto_cpp
impl /*struct*/ QBrush {
  pub fn QBrush_3<T: QBrush_QBrush_3>(value: T) -> QBrush {
    let rsthis = value.QBrush_3();
    return rsthis;
    // return 1;
  }
}

pub trait QBrush_QBrush_3 {
  fn QBrush_3(self) -> QBrush;
}
// QBrush(Qt::GlobalColor, Qt::BrushStyle) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBrush_QBrush_3 for (i32,i32) {
  fn QBrush_3(self) -> QBrush {
    // unsafe{_ZN6QBrushC2EN2Qt11GlobalColorENS0_10BrushStyleE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QBrushC2EN2Qt11GlobalColorENS0_10BrushStyleE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:71
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QBrush(const QColor &, const QPixmap &)

/*
Constructs a default black brush with the style Qt::NoBrush (i.e. this brush will not fill shapes).
*/
// QBrush(const QColor &, const QPixmap &) ctx.fn_proto_cpp
impl /*struct*/ QBrush {
  pub fn QBrush_4<T: QBrush_QBrush_4>(value: T) -> QBrush {
    let rsthis = value.QBrush_4();
    return rsthis;
    // return 1;
  }
}

pub trait QBrush_QBrush_4 {
  fn QBrush_4(self) -> QBrush;
}
// QBrush(const QColor &, const QPixmap &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBrush_QBrush_4 for (usize,usize) {
  fn QBrush_4(self) -> QBrush {
    // unsafe{_ZN6QBrushC2ERK6QColorRK7QPixmap()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QBrushC2ERK6QColorRK7QPixmap", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:72
// index:5
// Public Visibility=Default Availability=Available
// [-2] void QBrush(Qt::GlobalColor, const QPixmap &)

/*
Constructs a default black brush with the style Qt::NoBrush (i.e. this brush will not fill shapes).
*/
// QBrush(Qt::GlobalColor, const QPixmap &) ctx.fn_proto_cpp
impl /*struct*/ QBrush {
  pub fn QBrush_5<T: QBrush_QBrush_5>(value: T) -> QBrush {
    let rsthis = value.QBrush_5();
    return rsthis;
    // return 1;
  }
}

pub trait QBrush_QBrush_5 {
  fn QBrush_5(self) -> QBrush;
}
// QBrush(Qt::GlobalColor, const QPixmap &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBrush_QBrush_5 for (i32,usize) {
  fn QBrush_5(self) -> QBrush {
    // unsafe{_ZN6QBrushC2EN2Qt11GlobalColorERK7QPixmap()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QBrushC2EN2Qt11GlobalColorERK7QPixmap", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:73
// index:6
// Public Visibility=Default Availability=Available
// [-2] void QBrush(const QPixmap &)

/*
Constructs a default black brush with the style Qt::NoBrush (i.e. this brush will not fill shapes).
*/
// QBrush(const QPixmap &) ctx.fn_proto_cpp
impl /*struct*/ QBrush {
  pub fn QBrush_6<T: QBrush_QBrush_6>(value: T) -> QBrush {
    let rsthis = value.QBrush_6();
    return rsthis;
    // return 1;
  }
}

pub trait QBrush_QBrush_6 {
  fn QBrush_6(self) -> QBrush;
}
// QBrush(const QPixmap &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBrush_QBrush_6 for (usize) {
  fn QBrush_6(self) -> QBrush {
    // unsafe{_ZN6QBrushC2ERK7QPixmap()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QBrushC2ERK7QPixmap", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:74
// index:7
// Public Visibility=Default Availability=Available
// [-2] void QBrush(const QImage &)

/*
Constructs a default black brush with the style Qt::NoBrush (i.e. this brush will not fill shapes).
*/
// QBrush(const QImage &) ctx.fn_proto_cpp
impl /*struct*/ QBrush {
  pub fn QBrush_7<T: QBrush_QBrush_7>(value: T) -> QBrush {
    let rsthis = value.QBrush_7();
    return rsthis;
    // return 1;
  }
}

pub trait QBrush_QBrush_7 {
  fn QBrush_7(self) -> QBrush;
}
// QBrush(const QImage &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBrush_QBrush_7 for (usize) {
  fn QBrush_7(self) -> QBrush {
    // unsafe{_ZN6QBrushC2ERK6QImage()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QBrushC2ERK6QImage", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:78
// index:8
// Public Visibility=Default Availability=Available
// [-2] void QBrush(const QGradient &)

/*
Constructs a default black brush with the style Qt::NoBrush (i.e. this brush will not fill shapes).
*/
// QBrush(const QGradient &) ctx.fn_proto_cpp
impl /*struct*/ QBrush {
  pub fn QBrush_8<T: QBrush_QBrush_8>(value: T) -> QBrush {
    let rsthis = value.QBrush_8();
    return rsthis;
    // return 1;
  }
}

pub trait QBrush_QBrush_8 {
  fn QBrush_8(self) -> QBrush;
}
// QBrush(const QGradient &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBrush_QBrush_8 for (usize) {
  fn QBrush_8(self) -> QBrush {
    // unsafe{_ZN6QBrushC2ERK9QGradient()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QBrushC2ERK9QGradient", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QBrush()

/*

*/
pub fn DeleteQBrush(this :*mut QBrush) {
    // let rv = qtrt::InvokeQtFunc6("_ZN6QBrushD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qbrush.h:81
// index:0
// Public Visibility=Default Availability=Available
// [8] QBrush & operator=(const QBrush &)

/*

*/
impl /*struct*/ QBrush {
  pub fn operator_equal_0<RetType, T: QBrush_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QBrush_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QBrush) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QBrushaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:83
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QBrush & operator=(QBrush &&)

/*

*/
impl /*struct*/ QBrush {
  pub fn operator_equal_1<RetType, T: QBrush_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QBrush_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QBrush) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QBrushaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:86
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QBrush &)

/*
Swaps brush other with this brush. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QBrush {
  pub fn swap_0<RetType, T: QBrush_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QBrush_swap_0<RetType> {
  fn swap_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QBrush) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QBrush4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:91
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::BrushStyle style() const

/*
Returns the brush style.

See also setStyle().
*/
impl /*struct*/ QBrush {
  pub fn style_0<RetType, T: QBrush_style_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.style_0(self);
    // return 1;
  }
}
pub trait QBrush_style_0<RetType> {
  fn style_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_style_0<i32> for () {
  fn style_0(self , rsthis: & QBrush) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QBrush5styleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStyle(Qt::BrushStyle)

/*
Sets the brush style to style.

See also style().
*/
impl /*struct*/ QBrush {
  pub fn setStyle_0<RetType, T: QBrush_setStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStyle_0(self);
    // return 1;
  }
}
pub trait QBrush_setStyle_0<RetType> {
  fn setStyle_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_setStyle_0<(/*void*/)> for (i32) {
  fn setStyle_0(self , rsthis: & QBrush) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QBrush8setStyleEN2Qt10BrushStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:94
// index:0
// Public inline Visibility=Default Availability=Available
// [48] const QMatrix & matrix() const

/*
Returns the current transformation matrix for the brush.

This function was introduced in  Qt 4.2.

See also setMatrix().
*/
impl /*struct*/ QBrush {
  pub fn matrix_0<RetType, T: QBrush_matrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.matrix_0(self);
    // return 1;
  }
}
pub trait QBrush_matrix_0<RetType> {
  fn matrix_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_matrix_0<usize> for () {
  fn matrix_0(self , rsthis: & QBrush) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QBrush6matrixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMatrix(const QMatrix &)

/*
Sets matrix as an explicit transformation matrix on the current brush. The brush transformation matrix is merged with QPainter transformation matrix to produce the final result.

This function was introduced in  Qt 4.2.

See also matrix().
*/
impl /*struct*/ QBrush {
  pub fn setMatrix_0<RetType, T: QBrush_setMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMatrix_0(self);
    // return 1;
  }
}
pub trait QBrush_setMatrix_0<RetType> {
  fn setMatrix_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_setMatrix_0<(/*void*/)> for (usize) {
  fn setMatrix_0(self , rsthis: & QBrush) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QBrush9setMatrixERK7QMatrix", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:97
// index:0
// Public inline Visibility=Default Availability=Available
// [88] QTransform transform() const

/*
Returns the current transformation matrix for the brush.

This function was introduced in  Qt 4.3.

See also setTransform().
*/
impl /*struct*/ QBrush {
  pub fn transform_0<RetType, T: QBrush_transform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transform_0(self);
    // return 1;
  }
}
pub trait QBrush_transform_0<RetType> {
  fn transform_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_transform_0<usize> for () {
  fn transform_0(self , rsthis: & QBrush) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QBrush9transformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTransform(const QTransform &)

/*
Sets matrix as an explicit transformation matrix on the current brush. The brush transformation matrix is merged with QPainter transformation matrix to produce the final result.

This function was introduced in  Qt 4.3.

See also transform().
*/
impl /*struct*/ QBrush {
  pub fn setTransform_0<RetType, T: QBrush_setTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTransform_0(self);
    // return 1;
  }
}
pub trait QBrush_setTransform_0<RetType> {
  fn setTransform_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_setTransform_0<(/*void*/)> for (usize) {
  fn setTransform_0(self , rsthis: & QBrush) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QBrush12setTransformERK10QTransform", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:100
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap texture() const

/*
Returns the custom brush pattern, or a null pixmap if no custom brush pattern has been set.

See also setTexture().
*/
impl /*struct*/ QBrush {
  pub fn texture_0<RetType, T: QBrush_texture_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.texture_0(self);
    // return 1;
  }
}
pub trait QBrush_texture_0<RetType> {
  fn texture_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_texture_0<usize> for () {
  fn texture_0(self , rsthis: & QBrush) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QBrush7textureEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTexture(const QPixmap &)

/*
Sets the brush pixmap to pixmap. The style is set to Qt::TexturePattern.

The current brush color will only have an effect for monochrome pixmaps, i.e. for QPixmap::depth() == 1 (QBitmaps).

See also texture().
*/
impl /*struct*/ QBrush {
  pub fn setTexture_0<RetType, T: QBrush_setTexture_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTexture_0(self);
    // return 1;
  }
}
pub trait QBrush_setTexture_0<RetType> {
  fn setTexture_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_setTexture_0<(/*void*/)> for (usize) {
  fn setTexture_0(self , rsthis: & QBrush) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QBrush10setTextureERK7QPixmap", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:103
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage textureImage() const

/*
Returns the custom brush pattern, or a null image if no custom brush pattern has been set.

If the texture was set as a QPixmap it will be converted to a QImage.

This function was introduced in  Qt 4.2.

See also setTextureImage().
*/
impl /*struct*/ QBrush {
  pub fn textureImage_0<RetType, T: QBrush_textureImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textureImage_0(self);
    // return 1;
  }
}
pub trait QBrush_textureImage_0<RetType> {
  fn textureImage_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_textureImage_0<usize> for () {
  fn textureImage_0(self , rsthis: & QBrush) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QBrush12textureImageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextureImage(const QImage &)

/*
Sets the brush image to image. The style is set to Qt::TexturePattern.

Note the current brush color will not have any affect on monochrome images, as opposed to calling setTexture() with a QBitmap. If you want to change the color of monochrome image brushes, either convert the image to QBitmap with QBitmap::fromImage() and set the resulting QBitmap as a texture, or change the entries in the color table for the image.

This function was introduced in  Qt 4.2.

See also textureImage() and setTexture().
*/
impl /*struct*/ QBrush {
  pub fn setTextureImage_0<RetType, T: QBrush_setTextureImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextureImage_0(self);
    // return 1;
  }
}
pub trait QBrush_setTextureImage_0<RetType> {
  fn setTextureImage_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_setTextureImage_0<(/*void*/)> for (usize) {
  fn setTextureImage_0(self , rsthis: & QBrush) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QBrush15setTextureImageERK6QImage", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:106
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QColor & color() const

/*
Returns the brush color.

See also setColor().
*/
impl /*struct*/ QBrush {
  pub fn color_0<RetType, T: QBrush_color_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.color_0(self);
    // return 1;
  }
}
pub trait QBrush_color_0<RetType> {
  fn color_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_color_0<usize> for () {
  fn color_0(self , rsthis: & QBrush) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QBrush5colorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColor(const QColor &)

/*
Sets the brush color to the given color.

Note that calling setColor() will not make a difference if the style is a gradient. The same is the case if the style is Qt::TexturePattern style unless the current texture is a QBitmap.

See also color().
*/
impl /*struct*/ QBrush {
  pub fn setColor_0<RetType, T: QBrush_setColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColor_0(self);
    // return 1;
  }
}
pub trait QBrush_setColor_0<RetType> {
  fn setColor_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_setColor_0<(/*void*/)> for (usize) {
  fn setColor_0(self , rsthis: & QBrush) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QBrush8setColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:108
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setColor(Qt::GlobalColor)

/*
Sets the brush color to the given color.

Note that calling setColor() will not make a difference if the style is a gradient. The same is the case if the style is Qt::TexturePattern style unless the current texture is a QBitmap.

See also color().
*/
impl /*struct*/ QBrush {
  pub fn setColor_1<RetType, T: QBrush_setColor_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColor_1(self);
    // return 1;
  }
}
pub trait QBrush_setColor_1<RetType> {
  fn setColor_1(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_setColor_1<(/*void*/)> for (i32) {
  fn setColor_1(self , rsthis: & QBrush) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QBrush8setColorEN2Qt11GlobalColorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:110
// index:0
// Public Visibility=Default Availability=Available
// [8] const QGradient * gradient() const

/*
Returns the gradient describing this brush.
*/
impl /*struct*/ QBrush {
  pub fn gradient_0<RetType, T: QBrush_gradient_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.gradient_0(self);
    // return 1;
  }
}
pub trait QBrush_gradient_0<RetType> {
  fn gradient_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_gradient_0<usize> for () {
  fn gradient_0(self , rsthis: & QBrush) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QBrush8gradientEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:112
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isOpaque() const

/*
Returns true if the brush is fully opaque otherwise false. A brush is considered opaque if:


The alpha component of the color() is 255.
Its texture() does not have an alpha channel and is not a QBitmap.
The colors in the gradient() all have an alpha component that is 255.
It is an extended radial gradient.
*/
impl /*struct*/ QBrush {
  pub fn isOpaque_0<RetType, T: QBrush_isOpaque_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isOpaque_0(self);
    // return 1;
  }
}
pub trait QBrush_isOpaque_0<RetType> {
  fn isOpaque_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_isOpaque_0<bool> for () {
  fn isOpaque_0(self , rsthis: & QBrush) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QBrush8isOpaqueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:114
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QBrush &) const

/*

*/
impl /*struct*/ QBrush {
  pub fn operator_equal_equal_0<RetType, T: QBrush_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QBrush_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QBrush) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QBrusheqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:115
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QBrush &) const

/*

*/
impl /*struct*/ QBrush {
  pub fn operator_not_equal_0<RetType, T: QBrush_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QBrush_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QBrush) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QBrushneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:129
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDetached() const

/*

*/
impl /*struct*/ QBrush {
  pub fn isDetached_0<RetType, T: QBrush_isDetached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDetached_0(self);
    // return 1;
  }
}
pub trait QBrush_isDetached_0<RetType> {
  fn isDetached_0(self , rsthis: & QBrush) -> RetType;
}
impl<'a> /*trait*/ QBrush_isDetached_0<bool> for () {
  fn isDetached_0(self , rsthis: & QBrush) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QBrush10isDetachedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
