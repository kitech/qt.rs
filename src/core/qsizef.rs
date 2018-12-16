

// mod ::core::QSizeF
// package qtcore
// /usr/include/qt/QtCore/qsize.h
// #include <qsize.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 23
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
#[derive(Default)] // class sizeof(QSizeF)=16
pub struct QSizeF {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSizeF_ITF interface {
//    QSizeF_PTR() *QSizeF
//}
//func (ptr *QSizeF) QSizeF_PTR() *QSizeF { return ptr }

impl /*struct*/ QSizeF {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSizeF {
    return QSizeF{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSizeF {
//  type Target = QSizeFBASE;
//
//  fn deref(&self) -> &QSizeFBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSizeFBASE> for QSizeF {
//  fn as_ref(& self) -> & QSizeFBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsize.h:218
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QSizeF()

/*

*/
// QSizeF() ctx.fn_proto_cpp
impl /*struct*/ QSizeF {
  pub fn QSizeF_0<T: QSizeF_QSizeF_0>(value: T) -> QSizeF {
    let rsthis = value.QSizeF_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSizeF_QSizeF_0 {
  fn QSizeF_0(self) -> QSizeF;
}
// QSizeF() ctx.fn_proto_cpp
impl<'a> /*trait*/ QSizeF_QSizeF_0 for () {
  fn QSizeF_0(self) -> QSizeF {
    // unsafe{_ZN6QSizeFC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QSizeFC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSizeF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:219
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QSizeF(const QSize &)

/*

*/
// QSizeF(const QSize &) ctx.fn_proto_cpp
impl /*struct*/ QSizeF {
  pub fn QSizeF_1<T: QSizeF_QSizeF_1>(value: T) -> QSizeF {
    let rsthis = value.QSizeF_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSizeF_QSizeF_1 {
  fn QSizeF_1(self) -> QSizeF;
}
// QSizeF(const QSize &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSizeF_QSizeF_1 for (usize) {
  fn QSizeF_1(self) -> QSizeF {
    // unsafe{_ZN6QSizeFC2ERK5QSize()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QSizeFC2ERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSizeF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:220
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QSizeF(qreal, qreal)

/*

*/
// QSizeF(qreal, qreal) ctx.fn_proto_cpp
impl /*struct*/ QSizeF {
  pub fn QSizeF_2<T: QSizeF_QSizeF_2>(value: T) -> QSizeF {
    let rsthis = value.QSizeF_2();
    return rsthis;
    // return 1;
  }
}

pub trait QSizeF_QSizeF_2 {
  fn QSizeF_2(self) -> QSizeF;
}
// QSizeF(qreal, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSizeF_QSizeF_2 for (f64,f64) {
  fn QSizeF_2(self) -> QSizeF {
    // unsafe{_ZN6QSizeFC2Edd()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QSizeFC2Edd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSizeF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:222
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if both the width and height is 0; otherwise returns false.

See also isValid() and isEmpty().
*/
impl /*struct*/ QSizeF {
  pub fn isNull_0<RetType, T: QSizeF_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QSizeF_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QSizeF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QSizeF6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:223
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if either of the width and height is less than or equal to 0; otherwise returns false.

See also isNull() and isValid().
*/
impl /*struct*/ QSizeF {
  pub fn isEmpty_0<RetType, T: QSizeF_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QSizeF_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QSizeF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QSizeF7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:224
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if both the width and height is equal to or greater than 0; otherwise returns false.

See also isNull() and isEmpty().
*/
impl /*struct*/ QSizeF {
  pub fn isValid_0<RetType, T: QSizeF_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QSizeF_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QSizeF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QSizeF7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:226
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal width() const

/*
Returns the width.

See also height() and setWidth().
*/
impl /*struct*/ QSizeF {
  pub fn width_0<RetType, T: QSizeF_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QSizeF_width_0<RetType> {
  fn width_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_width_0<f64> for () {
  fn width_0(self , rsthis: & QSizeF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QSizeF5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:227
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal height() const

/*
Returns the height.

See also width() and setHeight().
*/
impl /*struct*/ QSizeF {
  pub fn height_0<RetType, T: QSizeF_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QSizeF_height_0<RetType> {
  fn height_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_height_0<f64> for () {
  fn height_0(self , rsthis: & QSizeF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QSizeF6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:228
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setWidth(qreal)

/*
Sets the width to the given width.

See also rwidth(), width(), and setHeight().
*/
impl /*struct*/ QSizeF {
  pub fn setWidth_0<RetType, T: QSizeF_setWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidth_0(self);
    // return 1;
  }
}
pub trait QSizeF_setWidth_0<RetType> {
  fn setWidth_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_setWidth_0<(/*void*/)> for (f64) {
  fn setWidth_0(self , rsthis: & QSizeF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QSizeF8setWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:229
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setHeight(qreal)

/*
Sets the height to the given height.

See also rheight(), height(), and setWidth().
*/
impl /*struct*/ QSizeF {
  pub fn setHeight_0<RetType, T: QSizeF_setHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeight_0(self);
    // return 1;
  }
}
pub trait QSizeF_setHeight_0<RetType> {
  fn setHeight_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_setHeight_0<(/*void*/)> for (f64) {
  fn setHeight_0(self , rsthis: & QSizeF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QSizeF9setHeightEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:230
// index:0
// Public Visibility=Default Availability=Available
// [-2] void transpose()

/*
Swaps the width and height values.

See also setWidth(), setHeight(), and transposed().
*/
impl /*struct*/ QSizeF {
  pub fn transpose_0<RetType, T: QSizeF_transpose_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transpose_0(self);
    // return 1;
  }
}
pub trait QSizeF_transpose_0<RetType> {
  fn transpose_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_transpose_0<(/*void*/)> for () {
  fn transpose_0(self , rsthis: & QSizeF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QSizeF9transposeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:231
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QSizeF transposed() const

/*
Returns a QSize with width and height swapped.

This function was introduced in  Qt 5.0.

See also transpose().
*/
impl /*struct*/ QSizeF {
  pub fn transposed_0<RetType, T: QSizeF_transposed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transposed_0(self);
    // return 1;
  }
}
pub trait QSizeF_transposed_0<RetType> {
  fn transposed_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_transposed_0<usize> for () {
  fn transposed_0(self , rsthis: & QSizeF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QSizeF10transposedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:233
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void scale(qreal, qreal, Qt::AspectRatioMode)

/*
Scales the size to a rectangle with the given width and height, according to the specified mode:


If mode is Qt::IgnoreAspectRatio, the size is set to (width, height).
If mode is Qt::KeepAspectRatio, the current size is scaled to a rectangle as large as possible inside (width, height), preserving the aspect ratio.
If mode is Qt::KeepAspectRatioByExpanding, the current size is scaled to a rectangle as small as possible outside (width, height), preserving the aspect ratio.


Example:


  QSize t1(10, 12);
  t1.scale(60, 60, Qt::IgnoreAspectRatio);
  // t1 is (60, 60)

  QSize t2(10, 12);
  t2.scale(60, 60, Qt::KeepAspectRatio);
  // t2 is (50, 60)

  QSize t3(10, 12);
  t3.scale(60, 60, Qt::KeepAspectRatioByExpanding);
  // t3 is (60, 72)



See also setWidth(), setHeight(), and scaled().
*/
impl /*struct*/ QSizeF {
  pub fn scale_0<RetType, T: QSizeF_scale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scale_0(self);
    // return 1;
  }
}
pub trait QSizeF_scale_0<RetType> {
  fn scale_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_scale_0<(/*void*/)> for (f64,f64,i32) {
  fn scale_0(self , rsthis: & QSizeF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QSizeF5scaleEddN2Qt15AspectRatioModeE", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:234
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void scale(const QSizeF &, Qt::AspectRatioMode)

/*
Scales the size to a rectangle with the given width and height, according to the specified mode:


If mode is Qt::IgnoreAspectRatio, the size is set to (width, height).
If mode is Qt::KeepAspectRatio, the current size is scaled to a rectangle as large as possible inside (width, height), preserving the aspect ratio.
If mode is Qt::KeepAspectRatioByExpanding, the current size is scaled to a rectangle as small as possible outside (width, height), preserving the aspect ratio.


Example:


  QSize t1(10, 12);
  t1.scale(60, 60, Qt::IgnoreAspectRatio);
  // t1 is (60, 60)

  QSize t2(10, 12);
  t2.scale(60, 60, Qt::KeepAspectRatio);
  // t2 is (50, 60)

  QSize t3(10, 12);
  t3.scale(60, 60, Qt::KeepAspectRatioByExpanding);
  // t3 is (60, 72)



See also setWidth(), setHeight(), and scaled().
*/
impl /*struct*/ QSizeF {
  pub fn scale_1<RetType, T: QSizeF_scale_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scale_1(self);
    // return 1;
  }
}
pub trait QSizeF_scale_1<RetType> {
  fn scale_1(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_scale_1<(/*void*/)> for (usize,i32) {
  fn scale_1(self , rsthis: & QSizeF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QSizeF5scaleERKS_N2Qt15AspectRatioModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:235
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF scaled(qreal, qreal, Qt::AspectRatioMode) const

/*
Return a size scaled to a rectangle with the given width and height, according to the specified mode.

This function was introduced in  Qt 5.0.

See also scale().
*/
impl /*struct*/ QSizeF {
  pub fn scaled_0<RetType, T: QSizeF_scaled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaled_0(self);
    // return 1;
  }
}
pub trait QSizeF_scaled_0<RetType> {
  fn scaled_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_scaled_0<usize> for (f64,f64,i32) {
  fn scaled_0(self , rsthis: & QSizeF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QSizeF6scaledEddN2Qt15AspectRatioModeE", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:236
// index:1
// Public Visibility=Default Availability=Available
// [16] QSizeF scaled(const QSizeF &, Qt::AspectRatioMode) const

/*
Return a size scaled to a rectangle with the given width and height, according to the specified mode.

This function was introduced in  Qt 5.0.

See also scale().
*/
impl /*struct*/ QSizeF {
  pub fn scaled_1<RetType, T: QSizeF_scaled_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaled_1(self);
    // return 1;
  }
}
pub trait QSizeF_scaled_1<RetType> {
  fn scaled_1(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_scaled_1<usize> for (usize,i32) {
  fn scaled_1(self , rsthis: & QSizeF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QSizeF6scaledERKS_N2Qt15AspectRatioModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:238
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QSizeF expandedTo(const QSizeF &) const

/*
Returns a size holding the maximum width and height of this size and the given otherSize.

See also boundedTo() and scale().
*/
impl /*struct*/ QSizeF {
  pub fn expandedTo_0<RetType, T: QSizeF_expandedTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandedTo_0(self);
    // return 1;
  }
}
pub trait QSizeF_expandedTo_0<RetType> {
  fn expandedTo_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_expandedTo_0<usize> for (usize) {
  fn expandedTo_0(self , rsthis: & QSizeF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QSizeF10expandedToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:239
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QSizeF boundedTo(const QSizeF &) const

/*
Returns a size holding the minimum width and height of this size and the given otherSize.

See also expandedTo() and scale().
*/
impl /*struct*/ QSizeF {
  pub fn boundedTo_0<RetType, T: QSizeF_boundedTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundedTo_0(self);
    // return 1;
  }
}
pub trait QSizeF_boundedTo_0<RetType> {
  fn boundedTo_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_boundedTo_0<usize> for (usize) {
  fn boundedTo_0(self , rsthis: & QSizeF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QSizeF9boundedToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:241
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal & rwidth()

/*
Returns a reference to the width.

Using a reference makes it possible to manipulate the width directly. For example:


  QSize size(100, 10);
  size.rwidth() += 20;

  // size becomes (120,10)



See also rheight() and setWidth().
*/
impl /*struct*/ QSizeF {
  pub fn rwidth_0<RetType, T: QSizeF_rwidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rwidth_0(self);
    // return 1;
  }
}
pub trait QSizeF_rwidth_0<RetType> {
  fn rwidth_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_rwidth_0<usize> for () {
  fn rwidth_0(self , rsthis: & QSizeF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QSizeF6rwidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:242
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal & rheight()

/*
Returns a reference to the height.

Using a reference makes it possible to manipulate the height directly. For example:


  QSize size(100, 10);
  size.rheight() += 5;

  // size becomes (100,15)



See also rwidth() and setHeight().
*/
impl /*struct*/ QSizeF {
  pub fn rheight_0<RetType, T: QSizeF_rheight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rheight_0(self);
    // return 1;
  }
}
pub trait QSizeF_rheight_0<RetType> {
  fn rheight_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_rheight_0<usize> for () {
  fn rheight_0(self , rsthis: & QSizeF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QSizeF7rheightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:244
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QSizeF & operator+=(const QSizeF &)

/*

*/
impl /*struct*/ QSizeF {
  pub fn operator_add_equal_0<RetType, T: QSizeF_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QSizeF_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QSizeF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QSizeFpLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:245
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QSizeF & operator-=(const QSizeF &)

/*

*/
impl /*struct*/ QSizeF {
  pub fn operator_minus_equal_0<RetType, T: QSizeF_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QSizeF_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QSizeF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QSizeFmIERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:246
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QSizeF & operator*=(qreal)

/*

*/
impl /*struct*/ QSizeF {
  pub fn operator_mul_equal_0<RetType, T: QSizeF_operator_mul_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_0(self);
    // return 1;
  }
}
pub trait QSizeF_operator_mul_equal_0<RetType> {
  fn operator_mul_equal_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_operator_mul_equal_0<usize> for (f64) {
  fn operator_mul_equal_0(self , rsthis: & QSizeF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QSizeFmLEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:247
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QSizeF & operator/=(qreal)

/*

*/
impl /*struct*/ QSizeF {
  pub fn operator_div_equal_0<RetType, T: QSizeF_operator_div_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_0(self);
    // return 1;
  }
}
pub trait QSizeF_operator_div_equal_0<RetType> {
  fn operator_div_equal_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_operator_div_equal_0<usize> for (f64) {
  fn operator_div_equal_0(self , rsthis: & QSizeF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QSizeFdVEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:257
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSize toSize() const

/*

*/
impl /*struct*/ QSizeF {
  pub fn toSize_0<RetType, T: QSizeF_toSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toSize_0(self);
    // return 1;
  }
}
pub trait QSizeF_toSize_0<RetType> {
  fn toSize_0(self , rsthis: & QSizeF) -> RetType;
}
impl<'a> /*trait*/ QSizeF_toSize_0<usize> for () {
  fn toSize_0(self , rsthis: & QSizeF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QSizeF6toSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQSizeF(this :*mut QSizeF) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN6QSizeFD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
