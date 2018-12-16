

// mod ::core::QSize
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
// extern C begin: 5
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
#[derive(Default)] // class sizeof(QSize)=8
pub struct QSize {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSize_ITF interface {
//    QSize_PTR() *QSize
//}
//func (ptr *QSize) QSize_PTR() *QSize { return ptr }

impl /*struct*/ QSize {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSize {
    return QSize{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSize {
//  type Target = QSizeBASE;
//
//  fn deref(&self) -> &QSizeBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSizeBASE> for QSize {
//  fn as_ref(& self) -> & QSizeBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsize.h:55
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QSize()

/*
Constructs a size with an invalid width and height (i.e., isValid() returns false).

See also isValid().
*/
// QSize() ctx.fn_proto_cpp
impl /*struct*/ QSize {
  pub fn QSize_0<T: QSize_QSize_0>(value: T) -> QSize {
    let rsthis = value.QSize_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSize_QSize_0 {
  fn QSize_0(self) -> QSize;
}
// QSize() ctx.fn_proto_cpp
impl<'a> /*trait*/ QSize_QSize_0 for () {
  fn QSize_0(self) -> QSize {
    // unsafe{_ZN5QSizeC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QSizeC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSize{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:56
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QSize(int, int)

/*
Constructs a size with an invalid width and height (i.e., isValid() returns false).

See also isValid().
*/
// QSize(int, int) ctx.fn_proto_cpp
impl /*struct*/ QSize {
  pub fn QSize_1<T: QSize_QSize_1>(value: T) -> QSize {
    let rsthis = value.QSize_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSize_QSize_1 {
  fn QSize_1(self) -> QSize;
}
// QSize(int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSize_QSize_1 for (i32,i32) {
  fn QSize_1(self) -> QSize {
    // unsafe{_ZN5QSizeC2Eii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QSizeC2Eii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSize{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:58
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if both the width and height is 0; otherwise returns false.

See also isValid() and isEmpty().
*/
impl /*struct*/ QSize {
  pub fn isNull_0<RetType, T: QSize_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QSize_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QSize) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QSize6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:59
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if either of the width and height is less than or equal to 0; otherwise returns false.

See also isNull() and isValid().
*/
impl /*struct*/ QSize {
  pub fn isEmpty_0<RetType, T: QSize_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QSize_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QSize) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QSize7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:60
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if both the width and height is equal to or greater than 0; otherwise returns false.

See also isNull() and isEmpty().
*/
impl /*struct*/ QSize {
  pub fn isValid_0<RetType, T: QSize_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QSize_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QSize) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QSize7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:62
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int width() const

/*
Returns the width.

See also height() and setWidth().
*/
impl /*struct*/ QSize {
  pub fn width_0<RetType, T: QSize_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QSize_width_0<RetType> {
  fn width_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_width_0<i32> for () {
  fn width_0(self , rsthis: & QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QSize5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:63
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int height() const

/*
Returns the height.

See also width() and setHeight().
*/
impl /*struct*/ QSize {
  pub fn height_0<RetType, T: QSize_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QSize_height_0<RetType> {
  fn height_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_height_0<i32> for () {
  fn height_0(self , rsthis: & QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QSize6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:64
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setWidth(int)

/*
Sets the width to the given width.

See also rwidth(), width(), and setHeight().
*/
impl /*struct*/ QSize {
  pub fn setWidth_0<RetType, T: QSize_setWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidth_0(self);
    // return 1;
  }
}
pub trait QSize_setWidth_0<RetType> {
  fn setWidth_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_setWidth_0<(/*void*/)> for (i32) {
  fn setWidth_0(self , rsthis: & QSize) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QSize8setWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:65
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setHeight(int)

/*
Sets the height to the given height.

See also rheight(), height(), and setWidth().
*/
impl /*struct*/ QSize {
  pub fn setHeight_0<RetType, T: QSize_setHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeight_0(self);
    // return 1;
  }
}
pub trait QSize_setHeight_0<RetType> {
  fn setHeight_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_setHeight_0<(/*void*/)> for (i32) {
  fn setHeight_0(self , rsthis: & QSize) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QSize9setHeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void transpose()

/*
Swaps the width and height values.

See also setWidth(), setHeight(), and transposed().
*/
impl /*struct*/ QSize {
  pub fn transpose_0<RetType, T: QSize_transpose_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transpose_0(self);
    // return 1;
  }
}
pub trait QSize_transpose_0<RetType> {
  fn transpose_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_transpose_0<(/*void*/)> for () {
  fn transpose_0(self , rsthis: & QSize) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN5QSize9transposeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:67
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSize transposed() const

/*
Returns a QSize with width and height swapped.

This function was introduced in  Qt 5.0.

See also transpose().
*/
impl /*struct*/ QSize {
  pub fn transposed_0<RetType, T: QSize_transposed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transposed_0(self);
    // return 1;
  }
}
pub trait QSize_transposed_0<RetType> {
  fn transposed_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_transposed_0<usize> for () {
  fn transposed_0(self , rsthis: & QSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QSize10transposedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void scale(int, int, Qt::AspectRatioMode)

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
impl /*struct*/ QSize {
  pub fn scale_0<RetType, T: QSize_scale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scale_0(self);
    // return 1;
  }
}
pub trait QSize_scale_0<RetType> {
  fn scale_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_scale_0<(/*void*/)> for (i32,i32,i32) {
  fn scale_0(self , rsthis: & QSize) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QSize5scaleEiiN2Qt15AspectRatioModeE", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:70
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void scale(const QSize &, Qt::AspectRatioMode)

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
impl /*struct*/ QSize {
  pub fn scale_1<RetType, T: QSize_scale_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scale_1(self);
    // return 1;
  }
}
pub trait QSize_scale_1<RetType> {
  fn scale_1(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_scale_1<(/*void*/)> for (usize,i32) {
  fn scale_1(self , rsthis: & QSize) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QSize5scaleERKS_N2Qt15AspectRatioModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsize.h:71
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize scaled(int, int, Qt::AspectRatioMode) const

/*
Return a size scaled to a rectangle with the given width and height, according to the specified mode.

This function was introduced in  Qt 5.0.

See also scale().
*/
impl /*struct*/ QSize {
  pub fn scaled_0<RetType, T: QSize_scaled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaled_0(self);
    // return 1;
  }
}
pub trait QSize_scaled_0<RetType> {
  fn scaled_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_scaled_0<usize> for (i32,i32,i32) {
  fn scaled_0(self , rsthis: & QSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QSize6scaledEiiN2Qt15AspectRatioModeE", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:72
// index:1
// Public Visibility=Default Availability=Available
// [8] QSize scaled(const QSize &, Qt::AspectRatioMode) const

/*
Return a size scaled to a rectangle with the given width and height, according to the specified mode.

This function was introduced in  Qt 5.0.

See also scale().
*/
impl /*struct*/ QSize {
  pub fn scaled_1<RetType, T: QSize_scaled_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaled_1(self);
    // return 1;
  }
}
pub trait QSize_scaled_1<RetType> {
  fn scaled_1(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_scaled_1<usize> for (usize,i32) {
  fn scaled_1(self , rsthis: & QSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QSize6scaledERKS_N2Qt15AspectRatioModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:74
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSize expandedTo(const QSize &) const

/*
Returns a size holding the maximum width and height of this size and the given otherSize.

See also boundedTo() and scale().
*/
impl /*struct*/ QSize {
  pub fn expandedTo_0<RetType, T: QSize_expandedTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expandedTo_0(self);
    // return 1;
  }
}
pub trait QSize_expandedTo_0<RetType> {
  fn expandedTo_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_expandedTo_0<usize> for (usize) {
  fn expandedTo_0(self , rsthis: & QSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QSize10expandedToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:75
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSize boundedTo(const QSize &) const

/*
Returns a size holding the minimum width and height of this size and the given otherSize.

See also expandedTo() and scale().
*/
impl /*struct*/ QSize {
  pub fn boundedTo_0<RetType, T: QSize_boundedTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundedTo_0(self);
    // return 1;
  }
}
pub trait QSize_boundedTo_0<RetType> {
  fn boundedTo_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_boundedTo_0<usize> for (usize) {
  fn boundedTo_0(self , rsthis: & QSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QSize9boundedToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:77
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int & rwidth()

/*
Returns a reference to the width.

Using a reference makes it possible to manipulate the width directly. For example:


  QSize size(100, 10);
  size.rwidth() += 20;

  // size becomes (120,10)



See also rheight() and setWidth().
*/
impl /*struct*/ QSize {
  pub fn rwidth_0<RetType, T: QSize_rwidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rwidth_0(self);
    // return 1;
  }
}
pub trait QSize_rwidth_0<RetType> {
  fn rwidth_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_rwidth_0<usize> for () {
  fn rwidth_0(self , rsthis: & QSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QSize6rwidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int & rheight()

/*
Returns a reference to the height.

Using a reference makes it possible to manipulate the height directly. For example:


  QSize size(100, 10);
  size.rheight() += 5;

  // size becomes (100,15)



See also rwidth() and setHeight().
*/
impl /*struct*/ QSize {
  pub fn rheight_0<RetType, T: QSize_rheight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rheight_0(self);
    // return 1;
  }
}
pub trait QSize_rheight_0<RetType> {
  fn rheight_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_rheight_0<usize> for () {
  fn rheight_0(self , rsthis: & QSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QSize7rheightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:80
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSize & operator+=(const QSize &)

/*

*/
impl /*struct*/ QSize {
  pub fn operator_add_equal_0<RetType, T: QSize_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QSize_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QSizepLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSize & operator-=(const QSize &)

/*

*/
impl /*struct*/ QSize {
  pub fn operator_minus_equal_0<RetType, T: QSize_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QSize_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QSizemIERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:82
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSize & operator*=(qreal)

/*

*/
impl /*struct*/ QSize {
  pub fn operator_mul_equal_0<RetType, T: QSize_operator_mul_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_0(self);
    // return 1;
  }
}
pub trait QSize_operator_mul_equal_0<RetType> {
  fn operator_mul_equal_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_operator_mul_equal_0<usize> for (f64) {
  fn operator_mul_equal_0(self , rsthis: & QSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QSizemLEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsize.h:83
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSize & operator/=(qreal)

/*

*/
impl /*struct*/ QSize {
  pub fn operator_div_equal_0<RetType, T: QSize_operator_div_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_0(self);
    // return 1;
  }
}
pub trait QSize_operator_div_equal_0<RetType> {
  fn operator_div_equal_0(self , rsthis: & QSize) -> RetType;
}
impl<'a> /*trait*/ QSize_operator_div_equal_0<usize> for (f64) {
  fn operator_div_equal_0(self , rsthis: & QSize) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QSizedVEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQSize(this :*mut QSize) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN5QSizeD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
