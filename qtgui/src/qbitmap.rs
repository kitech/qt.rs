

// mod ::gui::QBitmap
// package qtgui
// /usr/include/qt/QtGui/qbitmap.h
// #include <qbitmap.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 13
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
#[derive(Default)] // class sizeof(QBitmap)=32
pub struct QBitmap {
  qbase: QPixmap,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QBitmap_ITF interface {
//    QPixmap_ITF
//    QBitmap_PTR() *QBitmap
//}
//func (ptr *QBitmap) QBitmap_PTR() *QBitmap { return ptr }

impl /*struct*/ QBitmap {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QBitmap {
    return QBitmap{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QBitmap {
//  type Target = QBitmapBASE;
//
//  fn deref(&self) -> &QBitmapBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QBitmapBASE> for QBitmap {
//  fn as_ref(& self) -> & QBitmapBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qbitmap.h:54
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QBitmap()

/*
Constructs a null bitmap.

See also QPixmap::isNull().
*/
// QBitmap() ctx.fn_proto_cpp
impl /*struct*/ QBitmap {
  pub fn QBitmap_0<T: QBitmap_QBitmap_0>(value: T) -> QBitmap {
    let rsthis = value.QBitmap_0();
    return rsthis;
    // return 1;
  }
}

pub trait QBitmap_QBitmap_0 {
  fn QBitmap_0(self) -> QBitmap;
}
// QBitmap() ctx.fn_proto_cpp
impl<'a> /*trait*/ QBitmap_QBitmap_0 for () {
  fn QBitmap_0(self) -> QBitmap {
    // unsafe{_ZN7QBitmapC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QBitmapC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBitmap{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbitmap.h:55
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QBitmap(const QPixmap &)

/*
Constructs a null bitmap.

See also QPixmap::isNull().
*/
// QBitmap(const QPixmap &) ctx.fn_proto_cpp
impl /*struct*/ QBitmap {
  pub fn QBitmap_1<T: QBitmap_QBitmap_1>(value: T) -> QBitmap {
    let rsthis = value.QBitmap_1();
    return rsthis;
    // return 1;
  }
}

pub trait QBitmap_QBitmap_1 {
  fn QBitmap_1(self) -> QBitmap;
}
// QBitmap(const QPixmap &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBitmap_QBitmap_1 for (usize) {
  fn QBitmap_1(self) -> QBitmap {
    // unsafe{_ZN7QBitmapC2ERK7QPixmap()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QBitmapC2ERK7QPixmap", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBitmap{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbitmap.h:56
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QBitmap(int, int)

/*
Constructs a null bitmap.

See also QPixmap::isNull().
*/
// QBitmap(int, int) ctx.fn_proto_cpp
impl /*struct*/ QBitmap {
  pub fn QBitmap_2<T: QBitmap_QBitmap_2>(value: T) -> QBitmap {
    let rsthis = value.QBitmap_2();
    return rsthis;
    // return 1;
  }
}

pub trait QBitmap_QBitmap_2 {
  fn QBitmap_2(self) -> QBitmap;
}
// QBitmap(int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBitmap_QBitmap_2 for (i32,i32) {
  fn QBitmap_2(self) -> QBitmap {
    // unsafe{_ZN7QBitmapC2Eii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QBitmapC2Eii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBitmap{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbitmap.h:57
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QBitmap(const QSize &)

/*
Constructs a null bitmap.

See also QPixmap::isNull().
*/
// QBitmap(const QSize &) ctx.fn_proto_cpp
impl /*struct*/ QBitmap {
  pub fn QBitmap_3<T: QBitmap_QBitmap_3>(value: T) -> QBitmap {
    let rsthis = value.QBitmap_3();
    return rsthis;
    // return 1;
  }
}

pub trait QBitmap_QBitmap_3 {
  fn QBitmap_3(self) -> QBitmap;
}
// QBitmap(const QSize &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBitmap_QBitmap_3 for (usize) {
  fn QBitmap_3(self) -> QBitmap {
    // unsafe{_ZN7QBitmapC2ERK5QSize()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QBitmapC2ERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBitmap{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbitmap.h:58
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QBitmap(const QString &, const char *)

/*
Constructs a null bitmap.

See also QPixmap::isNull().
*/
// QBitmap(const QString &, const char *) ctx.fn_proto_cpp
impl /*struct*/ QBitmap {
  pub fn QBitmap_4<T: QBitmap_QBitmap_4>(value: T) -> QBitmap {
    let rsthis = value.QBitmap_4();
    return rsthis;
    // return 1;
  }
}

pub trait QBitmap_QBitmap_4 {
  fn QBitmap_4(self) -> QBitmap;
}
// QBitmap(const QString &, const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBitmap_QBitmap_4 for (usize,usize) {
  fn QBitmap_4(self) -> QBitmap {
    // unsafe{_ZN7QBitmapC2ERK7QStringPKc()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QBitmapC2ERK7QStringPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBitmap{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbitmap.h:63
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QBitmap & operator=(const QBitmap &)

/*

*/
impl /*struct*/ QBitmap {
  pub fn operator_equal_0<RetType, T: QBitmap_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QBitmap_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QBitmap) -> RetType;
}
impl<'a> /*trait*/ QBitmap_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QBitmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QBitmapaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbitmap.h:64
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QBitmap & operator=(QBitmap &&)

/*

*/
impl /*struct*/ QBitmap {
  pub fn operator_equal_1<RetType, T: QBitmap_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QBitmap_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QBitmap) -> RetType;
}
impl<'a> /*trait*/ QBitmap_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QBitmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QBitmapaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbitmap.h:68
// index:2
// Public Visibility=Default Availability=Available
// [32] QBitmap & operator=(const QPixmap &)

/*

*/
impl /*struct*/ QBitmap {
  pub fn operator_equal_2<RetType, T: QBitmap_operator_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_2(self);
    // return 1;
  }
}
pub trait QBitmap_operator_equal_2<RetType> {
  fn operator_equal_2(self , rsthis: & QBitmap) -> RetType;
}
impl<'a> /*trait*/ QBitmap_operator_equal_2<usize> for (usize) {
  fn operator_equal_2(self , rsthis: & QBitmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QBitmapaSERK7QPixmap", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbitmap.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QBitmap()

/*

*/
pub fn DeleteQBitmap(this :*mut QBitmap) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QBitmapD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qbitmap.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QBitmap &)

/*
Swaps bitmap other with this bitmap. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QBitmap {
  pub fn swap_0<RetType, T: QBitmap_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QBitmap_swap_0<RetType> {
  fn swap_0(self , rsthis: & QBitmap) -> RetType;
}
impl<'a> /*trait*/ QBitmap_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QBitmap) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QBitmap4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbitmap.h:72
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the bitmap, setting all its bits to Qt::color0.
*/
impl /*struct*/ QBitmap {
  pub fn clear_0<RetType, T: QBitmap_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QBitmap_clear_0<RetType> {
  fn clear_0(self , rsthis: & QBitmap) -> RetType;
}
impl<'a> /*trait*/ QBitmap_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QBitmap) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QBitmap5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbitmap.h:74
// index:0
// Public static Visibility=Default Availability=Available
// [32] QBitmap fromImage(const QImage &, Qt::ImageConversionFlags)

/*
Returns a copy of the given image converted to a bitmap using the specified image conversion flags.

See also fromData().
*/
impl /*struct*/ QBitmap {
  pub fn fromImage_0<RetType, T: QBitmap_fromImage_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromImage_0();
    // return 1;
  }
}
pub trait QBitmap_fromImage_0<RetType> {
  fn fromImage_0(self ) -> RetType;
}
impl<'a> /*trait*/ QBitmap_fromImage_0<usize> for (usize,i32) {
  fn fromImage_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QBitmap9fromImageERK6QImage6QFlagsIN2Qt19ImageConversionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbitmap.h:75
// index:0
// Public static Visibility=Default Availability=Available
// [32] QBitmap fromData(const QSize &, const uchar *, QImage::Format)

/*
Constructs a bitmap with the given size, and sets the contents to the bits supplied.

The bitmap data has to be byte aligned and provided in in the bit order specified by monoFormat. The mono format must be either QImage::Format_Mono or QImage::Format_MonoLSB. Use QImage::Format_Mono to specify data on the XBM format.

See also fromImage().
*/
impl /*struct*/ QBitmap {
  pub fn fromData_0<RetType, T: QBitmap_fromData_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromData_0();
    // return 1;
  }
}
pub trait QBitmap_fromData_0<RetType> {
  fn fromData_0(self ) -> RetType;
}
impl<'a> /*trait*/ QBitmap_fromData_0<usize> for (usize,usize,i32) {
  fn fromData_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QBitmap8fromDataERK5QSizePKhN6QImage6FormatE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbitmap.h:78
// index:0
// Public Visibility=Default Availability=Available
// [32] QBitmap transformed(const QMatrix &) const

/*
Returns a copy of this bitmap, transformed according to the given matrix.

See also QPixmap::transformed().
*/
impl /*struct*/ QBitmap {
  pub fn transformed_0<RetType, T: QBitmap_transformed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transformed_0(self);
    // return 1;
  }
}
pub trait QBitmap_transformed_0<RetType> {
  fn transformed_0(self , rsthis: & QBitmap) -> RetType;
}
impl<'a> /*trait*/ QBitmap_transformed_0<usize> for (usize) {
  fn transformed_0(self , rsthis: & QBitmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QBitmap11transformedERK7QMatrix", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbitmap.h:79
// index:1
// Public Visibility=Default Availability=Available
// [32] QBitmap transformed(const QTransform &) const

/*
Returns a copy of this bitmap, transformed according to the given matrix.

See also QPixmap::transformed().
*/
impl /*struct*/ QBitmap {
  pub fn transformed_1<RetType, T: QBitmap_transformed_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transformed_1(self);
    // return 1;
  }
}
pub trait QBitmap_transformed_1<RetType> {
  fn transformed_1(self , rsthis: & QBitmap) -> RetType;
}
impl<'a> /*trait*/ QBitmap_transformed_1<usize> for (usize) {
  fn transformed_1(self , rsthis: & QBitmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QBitmap11transformedERK10QTransform", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
