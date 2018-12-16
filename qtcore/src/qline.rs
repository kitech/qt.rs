

// mod ::core::QLine
// package qtcore
// /usr/include/qt/QtCore/qline.h
// #include <qline.h>
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
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QLine)=16
pub struct QLine {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLine_ITF interface {
//    QLine_PTR() *QLine
//}
//func (ptr *QLine) QLine_PTR() *QLine { return ptr }

impl /*struct*/ QLine {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLine {
    return QLine{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLine {
//  type Target = QLineBASE;
//
//  fn deref(&self) -> &QLineBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLineBASE> for QLine {
//  fn as_ref(& self) -> & QLineBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qline.h:55
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QLine()

/*
Constructs a null line.
*/
// QLine() ctx.fn_proto_cpp
impl /*struct*/ QLine {
  pub fn QLine_0<T: QLine_QLine_0>(value: T) -> QLine {
    let rsthis = value.QLine_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLine_QLine_0 {
  fn QLine_0(self) -> QLine;
}
// QLine() ctx.fn_proto_cpp
impl<'a> /*trait*/ QLine_QLine_0 for () {
  fn QLine_0(self) -> QLine {
    // unsafe{_ZN5QLineC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QLineC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLine{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:56
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QLine(const QPoint &, const QPoint &)

/*
Constructs a null line.
*/
// QLine(const QPoint &, const QPoint &) ctx.fn_proto_cpp
impl /*struct*/ QLine {
  pub fn QLine_1<T: QLine_QLine_1>(value: T) -> QLine {
    let rsthis = value.QLine_1();
    return rsthis;
    // return 1;
  }
}

pub trait QLine_QLine_1 {
  fn QLine_1(self) -> QLine;
}
// QLine(const QPoint &, const QPoint &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLine_QLine_1 for (usize,usize) {
  fn QLine_1(self) -> QLine {
    // unsafe{_ZN5QLineC2ERK6QPointS2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QLineC2ERK6QPointS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLine{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:57
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QLine(int, int, int, int)

/*
Constructs a null line.
*/
// QLine(int, int, int, int) ctx.fn_proto_cpp
impl /*struct*/ QLine {
  pub fn QLine_2<T: QLine_QLine_2>(value: T) -> QLine {
    let rsthis = value.QLine_2();
    return rsthis;
    // return 1;
  }
}

pub trait QLine_QLine_2 {
  fn QLine_2(self) -> QLine;
}
// QLine(int, int, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLine_QLine_2 for (i32,i32,i32,i32) {
  fn QLine_2(self) -> QLine {
    // unsafe{_ZN5QLineC2Eiiii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QLineC2Eiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLine{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:59
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the line is not set up with valid start and end point; otherwise returns false.
*/
impl /*struct*/ QLine {
  pub fn isNull_0<RetType, T: QLine_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QLine_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QLine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLine6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:61
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint p1() const

/*
Returns the line's start point.

See also setP1(), x1(), y1(), and p2().
*/
impl /*struct*/ QLine {
  pub fn p1_0<RetType, T: QLine_p1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.p1_0(self);
    // return 1;
  }
}
pub trait QLine_p1_0<RetType> {
  fn p1_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_p1_0<usize> for () {
  fn p1_0(self , rsthis: & QLine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLine2p1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:62
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint p2() const

/*
Returns the line's end point.

See also setP2(), x2(), y2(), and p1().
*/
impl /*struct*/ QLine {
  pub fn p2_0<RetType, T: QLine_p2_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.p2_0(self);
    // return 1;
  }
}
pub trait QLine_p2_0<RetType> {
  fn p2_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_p2_0<usize> for () {
  fn p2_0(self , rsthis: & QLine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLine2p2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:64
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int x1() const

/*
Returns the x-coordinate of the line's start point.

See also p1().
*/
impl /*struct*/ QLine {
  pub fn x1_0<RetType, T: QLine_x1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x1_0(self);
    // return 1;
  }
}
pub trait QLine_x1_0<RetType> {
  fn x1_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_x1_0<i32> for () {
  fn x1_0(self , rsthis: & QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLine2x1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:65
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int y1() const

/*
Returns the y-coordinate of the line's start point.

See also p1().
*/
impl /*struct*/ QLine {
  pub fn y1_0<RetType, T: QLine_y1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y1_0(self);
    // return 1;
  }
}
pub trait QLine_y1_0<RetType> {
  fn y1_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_y1_0<i32> for () {
  fn y1_0(self , rsthis: & QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLine2y1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:67
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int x2() const

/*
Returns the x-coordinate of the line's end point.

See also p2().
*/
impl /*struct*/ QLine {
  pub fn x2_0<RetType, T: QLine_x2_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x2_0(self);
    // return 1;
  }
}
pub trait QLine_x2_0<RetType> {
  fn x2_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_x2_0<i32> for () {
  fn x2_0(self , rsthis: & QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLine2x2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:68
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int y2() const

/*
Returns the y-coordinate of the line's end point.

See also p2().
*/
impl /*struct*/ QLine {
  pub fn y2_0<RetType, T: QLine_y2_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y2_0(self);
    // return 1;
  }
}
pub trait QLine_y2_0<RetType> {
  fn y2_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_y2_0<i32> for () {
  fn y2_0(self , rsthis: & QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLine2y2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:70
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int dx() const

/*
Returns the horizontal component of the line's vector.

See also dy().
*/
impl /*struct*/ QLine {
  pub fn dx_0<RetType, T: QLine_dx_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dx_0(self);
    // return 1;
  }
}
pub trait QLine_dx_0<RetType> {
  fn dx_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_dx_0<i32> for () {
  fn dx_0(self , rsthis: & QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLine2dxEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:71
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int dy() const

/*
Returns the vertical component of the line's vector.

See also dx().
*/
impl /*struct*/ QLine {
  pub fn dy_0<RetType, T: QLine_dy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dy_0(self);
    // return 1;
  }
}
pub trait QLine_dy_0<RetType> {
  fn dy_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_dy_0<i32> for () {
  fn dy_0(self , rsthis: & QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLine2dyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:73
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void translate(const QPoint &)

/*
Translates this line by the given offset.
*/
impl /*struct*/ QLine {
  pub fn translate_0<RetType, T: QLine_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QLine_translate_0<RetType> {
  fn translate_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_translate_0<(/*void*/)> for (usize) {
  fn translate_0(self , rsthis: & QLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QLine9translateERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:74
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void translate(int, int)

/*
Translates this line by the given offset.
*/
impl /*struct*/ QLine {
  pub fn translate_1<RetType, T: QLine_translate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_1(self);
    // return 1;
  }
}
pub trait QLine_translate_1<RetType> {
  fn translate_1(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_translate_1<(/*void*/)> for (i32,i32) {
  fn translate_1(self , rsthis: & QLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QLine9translateEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:76
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QLine translated(const QPoint &) const

/*
Returns this line translated by the given offset.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QLine {
  pub fn translated_0<RetType, T: QLine_translated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_0(self);
    // return 1;
  }
}
pub trait QLine_translated_0<RetType> {
  fn translated_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_translated_0<usize> for (usize) {
  fn translated_0(self , rsthis: & QLine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLine10translatedERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:77
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QLine translated(int, int) const

/*
Returns this line translated by the given offset.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QLine {
  pub fn translated_1<RetType, T: QLine_translated_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_1(self);
    // return 1;
  }
}
pub trait QLine_translated_1<RetType> {
  fn translated_1(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_translated_1<usize> for (i32,i32) {
  fn translated_1(self , rsthis: & QLine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLine10translatedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:79
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint center() const

/*
Returns the center point of this line. This is equivalent to (p1() + p2()) / 2, except it will never overflow.

This function was introduced in  Qt 5.8.
*/
impl /*struct*/ QLine {
  pub fn center_0<RetType, T: QLine_center_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.center_0(self);
    // return 1;
  }
}
pub trait QLine_center_0<RetType> {
  fn center_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_center_0<usize> for () {
  fn center_0(self , rsthis: & QLine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLine6centerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setP1(const QPoint &)

/*
Sets the starting point of this line to p1.

This function was introduced in  Qt 4.4.

See also setP2() and p1().
*/
impl /*struct*/ QLine {
  pub fn setP1_0<RetType, T: QLine_setP1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setP1_0(self);
    // return 1;
  }
}
pub trait QLine_setP1_0<RetType> {
  fn setP1_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_setP1_0<(/*void*/)> for (usize) {
  fn setP1_0(self , rsthis: & QLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QLine5setP1ERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:82
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setP2(const QPoint &)

/*
Sets the end point of this line to p2.

This function was introduced in  Qt 4.4.

See also setP1() and p2().
*/
impl /*struct*/ QLine {
  pub fn setP2_0<RetType, T: QLine_setP2_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setP2_0(self);
    // return 1;
  }
}
pub trait QLine_setP2_0<RetType> {
  fn setP2_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_setP2_0<(/*void*/)> for (usize) {
  fn setP2_0(self , rsthis: & QLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QLine5setP2ERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:83
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setPoints(const QPoint &, const QPoint &)

/*
Sets the start point of this line to p1 and the end point of this line to p2.

This function was introduced in  Qt 4.4.

See also setP1(), setP2(), p1(), and p2().
*/
impl /*struct*/ QLine {
  pub fn setPoints_0<RetType, T: QLine_setPoints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPoints_0(self);
    // return 1;
  }
}
pub trait QLine_setPoints_0<RetType> {
  fn setPoints_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_setPoints_0<(/*void*/)> for (usize,usize) {
  fn setPoints_0(self , rsthis: & QLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QLine9setPointsERK6QPointS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:84
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setLine(int, int, int, int)

/*
Sets this line to the start in x1, y1 and end in x2, y2.

This function was introduced in  Qt 4.4.

See also setP1(), setP2(), p1(), and p2().
*/
impl /*struct*/ QLine {
  pub fn setLine_0<RetType, T: QLine_setLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLine_0(self);
    // return 1;
  }
}
pub trait QLine_setLine_0<RetType> {
  fn setLine_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_setLine_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn setLine_0(self , rsthis: & QLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QLine7setLineEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:86
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QLine &) const

/*

*/
impl /*struct*/ QLine {
  pub fn operator_equal_equal_0<RetType, T: QLine_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QLine_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QLine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLineeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:87
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QLine &) const

/*

*/
impl /*struct*/ QLine {
  pub fn operator_not_equal_0<RetType, T: QLine_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QLine_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QLine) -> RetType;
}
impl<'a> /*trait*/ QLine_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QLine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QLineneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQLine(this :*mut QLine) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN5QLineD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
