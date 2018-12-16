

// mod ::gui::QImage
// package qtgui
// /usr/include/qt/QtGui/qimage.h
// #include <qimage.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 24
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

// int metric(QPaintDevice::PaintDeviceMetric)
// func (this *QImage) InheritMetric(f func(metric int) int) {
//  qtrt.SetAllInheritCallback(this, "metric", f)
// }

// QImage mirrored_helper(bool, bool)
// func (this *QImage) InheritMirrored_helper(f func(horizontal bool, vertical bool) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "mirrored_helper", f)
// }

// QImage rgbSwapped_helper()
// func (this *QImage) InheritRgbSwapped_helper(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "rgbSwapped_helper", f)
// }

// void mirrored_inplace(bool, bool)
// func (this *QImage) InheritMirrored_inplace(f func(horizontal bool, vertical bool) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mirrored_inplace", f)
// }

// void rgbSwapped_inplace()
// func (this *QImage) InheritRgbSwapped_inplace(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "rgbSwapped_inplace", f)
// }

// QImage convertToFormat_helper(QImage::Format, Qt::ImageConversionFlags)
// func (this *QImage) InheritConvertToFormat_helper(f func(format int, flags int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "convertToFormat_helper", f)
// }

// bool convertToFormat_inplace(QImage::Format, Qt::ImageConversionFlags)
// func (this *QImage) InheritConvertToFormat_inplace(f func(format int, flags int) bool) {
//  qtrt.SetAllInheritCallback(this, "convertToFormat_inplace", f)
// }

// QImage smoothScaled(int, int)
// func (this *QImage) InheritSmoothScaled(f func(w int, h int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "smoothScaled", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QImage)=32
pub struct QImage {
  qbase: QPaintDevice,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QImage_ITF interface {
//    QPaintDevice_ITF
//    QImage_PTR() *QImage
//}
//func (ptr *QImage) QImage_PTR() *QImage { return ptr }

impl /*struct*/ QImage {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QImage {
    return QImage{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QImage {
//  type Target = QImageBASE;
//
//  fn deref(&self) -> &QImageBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QImageBASE> for QImage {
//  fn as_ref(& self) -> & QImageBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qimage.h:136
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QImage()

/*
Constructs a null image.

See also isNull().
*/
// QImage() ctx.fn_proto_cpp
impl /*struct*/ QImage {
  pub fn QImage_0<T: QImage_QImage_0>(value: T) -> QImage {
    let rsthis = value.QImage_0();
    return rsthis;
    // return 1;
  }
}

pub trait QImage_QImage_0 {
  fn QImage_0(self) -> QImage;
}
// QImage() ctx.fn_proto_cpp
impl<'a> /*trait*/ QImage_QImage_0 for () {
  fn QImage_0(self) -> QImage {
    // unsafe{_ZN6QImageC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QImageC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImage{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:137
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QImage(const QSize &, QImage::Format)

/*
Constructs a null image.

See also isNull().
*/
// QImage(const QSize &, QImage::Format) ctx.fn_proto_cpp
impl /*struct*/ QImage {
  pub fn QImage_1<T: QImage_QImage_1>(value: T) -> QImage {
    let rsthis = value.QImage_1();
    return rsthis;
    // return 1;
  }
}

pub trait QImage_QImage_1 {
  fn QImage_1(self) -> QImage;
}
// QImage(const QSize &, QImage::Format) ctx.fn_proto_cpp
impl<'a> /*trait*/ QImage_QImage_1 for (usize,i32) {
  fn QImage_1(self) -> QImage {
    // unsafe{_ZN6QImageC2ERK5QSizeNS_6FormatE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QImageC2ERK5QSizeNS_6FormatE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImage{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:138
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QImage(int, int, QImage::Format)

/*
Constructs a null image.

See also isNull().
*/
// QImage(int, int, QImage::Format) ctx.fn_proto_cpp
impl /*struct*/ QImage {
  pub fn QImage_2<T: QImage_QImage_2>(value: T) -> QImage {
    let rsthis = value.QImage_2();
    return rsthis;
    // return 1;
  }
}

pub trait QImage_QImage_2 {
  fn QImage_2(self) -> QImage;
}
// QImage(int, int, QImage::Format) ctx.fn_proto_cpp
impl<'a> /*trait*/ QImage_QImage_2 for (i32,i32,i32) {
  fn QImage_2(self) -> QImage {
    // unsafe{_ZN6QImageC2EiiNS_6FormatE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QImageC2EiiNS_6FormatE", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImage{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:139
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QImage(uchar *, int, int, QImage::Format, QImageCleanupFunction, void *)

/*
Constructs a null image.

See also isNull().
*/
// QImage(uchar *, int, int, QImage::Format, QImageCleanupFunction, void *) ctx.fn_proto_cpp
impl /*struct*/ QImage {
  pub fn QImage_3<T: QImage_QImage_3>(value: T) -> QImage {
    let rsthis = value.QImage_3();
    return rsthis;
    // return 1;
  }
}

pub trait QImage_QImage_3 {
  fn QImage_3(self) -> QImage;
}
// QImage(uchar *, int, int, QImage::Format, QImageCleanupFunction, void *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QImage_QImage_3 for (usize,i32,i32,i32,usize,usize) {
  fn QImage_3(self) -> QImage {
    // unsafe{_ZN6QImageC2EPhiiNS_6FormatEPFvPvES2_()};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const usize as usize;
    let arg5 = (&self.5) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QImageC2EPhiiNS_6FormatEPFvPvES2_", 6,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImage{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:140
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QImage(const uchar *, int, int, QImage::Format, QImageCleanupFunction, void *)

/*
Constructs a null image.

See also isNull().
*/
// QImage(const uchar *, int, int, QImage::Format, QImageCleanupFunction, void *) ctx.fn_proto_cpp
impl /*struct*/ QImage {
  pub fn QImage_4<T: QImage_QImage_4>(value: T) -> QImage {
    let rsthis = value.QImage_4();
    return rsthis;
    // return 1;
  }
}

pub trait QImage_QImage_4 {
  fn QImage_4(self) -> QImage;
}
// QImage(const uchar *, int, int, QImage::Format, QImageCleanupFunction, void *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QImage_QImage_4 for (usize,i32,i32,i32,usize,usize) {
  fn QImage_4(self) -> QImage {
    // unsafe{_ZN6QImageC2EPKhiiNS_6FormatEPFvPvES3_()};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const usize as usize;
    let arg5 = (&self.5) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QImageC2EPKhiiNS_6FormatEPFvPvES3_", 6,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImage{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:141
// index:5
// Public Visibility=Default Availability=Available
// [-2] void QImage(uchar *, int, int, int, QImage::Format, QImageCleanupFunction, void *)

/*
Constructs a null image.

See also isNull().
*/
// QImage(uchar *, int, int, int, QImage::Format, QImageCleanupFunction, void *) ctx.fn_proto_cpp
impl /*struct*/ QImage {
  pub fn QImage_5<T: QImage_QImage_5>(value: T) -> QImage {
    let rsthis = value.QImage_5();
    return rsthis;
    // return 1;
  }
}

pub trait QImage_QImage_5 {
  fn QImage_5(self) -> QImage;
}
// QImage(uchar *, int, int, int, QImage::Format, QImageCleanupFunction, void *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QImage_QImage_5 for (usize,i32,i32,i32,i32,usize,usize) {
  fn QImage_5(self) -> QImage {
    // unsafe{_ZN6QImageC2EPhiiiNS_6FormatEPFvPvES2_()};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const usize as usize;
    let arg6 = (&self.6) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QImageC2EPhiiiNS_6FormatEPFvPvES2_", 7,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    let rsthis = QImage{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:142
// index:6
// Public Visibility=Default Availability=Available
// [-2] void QImage(const uchar *, int, int, int, QImage::Format, QImageCleanupFunction, void *)

/*
Constructs a null image.

See also isNull().
*/
// QImage(const uchar *, int, int, int, QImage::Format, QImageCleanupFunction, void *) ctx.fn_proto_cpp
impl /*struct*/ QImage {
  pub fn QImage_6<T: QImage_QImage_6>(value: T) -> QImage {
    let rsthis = value.QImage_6();
    return rsthis;
    // return 1;
  }
}

pub trait QImage_QImage_6 {
  fn QImage_6(self) -> QImage;
}
// QImage(const uchar *, int, int, int, QImage::Format, QImageCleanupFunction, void *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QImage_QImage_6 for (usize,i32,i32,i32,i32,usize,usize) {
  fn QImage_6(self) -> QImage {
    // unsafe{_ZN6QImageC2EPKhiiiNS_6FormatEPFvPvES3_()};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const usize as usize;
    let arg6 = (&self.6) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QImageC2EPKhiiiNS_6FormatEPFvPvES3_", 7,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    let rsthis = QImage{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:145
// index:7
// Public Visibility=Default Availability=Available
// [-2] void QImage(const char *const *)

/*
Constructs a null image.

See also isNull().
*/
// QImage(const char *const *) ctx.fn_proto_cpp
impl /*struct*/ QImage {
  pub fn QImage_7<T: QImage_QImage_7>(value: T) -> QImage {
    let rsthis = value.QImage_7();
    return rsthis;
    // return 1;
  }
}

pub trait QImage_QImage_7 {
  fn QImage_7(self) -> QImage;
}
// QImage(const char *const *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QImage_QImage_7 for (usize) {
  fn QImage_7(self) -> QImage {
    // unsafe{_ZN6QImageC2EPKPKc()};
    let arg0 = (&self) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QImageC2EPKPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImage{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:147
// index:8
// Public Visibility=Default Availability=Available
// [-2] void QImage(const QString &, const char *)

/*
Constructs a null image.

See also isNull().
*/
// QImage(const QString &, const char *) ctx.fn_proto_cpp
impl /*struct*/ QImage {
  pub fn QImage_8<T: QImage_QImage_8>(value: T) -> QImage {
    let rsthis = value.QImage_8();
    return rsthis;
    // return 1;
  }
}

pub trait QImage_QImage_8 {
  fn QImage_8(self) -> QImage;
}
// QImage(const QString &, const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QImage_QImage_8 for (usize,usize) {
  fn QImage_8(self) -> QImage {
    // unsafe{_ZN6QImageC2ERK7QStringPKc()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QImageC2ERK7QStringPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImage{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:155
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QImage()

/*

*/
pub fn DeleteQImage(this :*mut QImage) {
    // let rv = qtrt::InvokeQtFunc6("_ZN6QImageD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qimage.h:157
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage & operator=(const QImage &)

/*

*/
impl /*struct*/ QImage {
  pub fn operator_equal_0<RetType, T: QImage_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QImage_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImageaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:159
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QImage & operator=(QImage &&)

/*

*/
impl /*struct*/ QImage {
  pub fn operator_equal_1<RetType, T: QImage_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QImage_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImageaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:162
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QImage &)

/*
Swaps image other with this image. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QImage {
  pub fn swap_0<RetType, T: QImage_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QImage_swap_0<RetType> {
  fn swap_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:165
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if it is a null image, otherwise returns false.

A null image has all parameters set to zero and no allocated data.
*/
impl /*struct*/ QImage {
  pub fn isNull_0<RetType, T: QImage_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QImage_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:167
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int devType() const

/*

*/
impl /*struct*/ QImage {
  pub fn devType_0<RetType, T: QImage_devType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.devType_0(self);
    // return 1;
  }
}
pub trait QImage_devType_0<RetType> {
  fn devType_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_devType_0<i32> for () {
  fn devType_0(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage7devTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:169
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QImage &) const

/*

*/
impl /*struct*/ QImage {
  pub fn operator_equal_equal_0<RetType, T: QImage_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QImage_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImageeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:170
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QImage &) const

/*

*/
impl /*struct*/ QImage {
  pub fn operator_not_equal_0<RetType, T: QImage_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QImage_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImageneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:172
// index:0
// Public Visibility=Default Availability=Available
// [-2] void detach()

/*

*/
impl /*struct*/ QImage {
  pub fn detach_0<RetType, T: QImage_detach_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detach_0(self);
    // return 1;
  }
}
pub trait QImage_detach_0<RetType> {
  fn detach_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_detach_0<(/*void*/)> for () {
  fn detach_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QImage6detachEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:173
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDetached() const

/*

*/
impl /*struct*/ QImage {
  pub fn isDetached_0<RetType, T: QImage_isDetached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDetached_0(self);
    // return 1;
  }
}
pub trait QImage_isDetached_0<RetType> {
  fn isDetached_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_isDetached_0<bool> for () {
  fn isDetached_0(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage10isDetachedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:175
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage copy(const QRect &) const

/*
Returns a sub-area of the image as a new image.

The returned image is copied from the position (rectangle.x(), rectangle.y()) in this image, and will always have the size of the given rectangle.

In areas beyond this image, pixels are set to 0. For 32-bit RGB images, this means black; for 32-bit ARGB images, this means transparent black; for 8-bit images, this means the color with index 0 in the color table which can be anything; for 1-bit images, this means Qt::color0.

If the given rectangle is a null rectangle the entire image is copied.

See also QImage().
*/
impl /*struct*/ QImage {
  pub fn copy_0<RetType, T: QImage_copy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.copy_0(self);
    // return 1;
  }
}
pub trait QImage_copy_0<RetType> {
  fn copy_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_copy_0<usize> for (usize) {
  fn copy_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage4copyERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:176
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QImage copy(int, int, int, int) const

/*
Returns a sub-area of the image as a new image.

The returned image is copied from the position (rectangle.x(), rectangle.y()) in this image, and will always have the size of the given rectangle.

In areas beyond this image, pixels are set to 0. For 32-bit RGB images, this means black; for 32-bit ARGB images, this means transparent black; for 8-bit images, this means the color with index 0 in the color table which can be anything; for 1-bit images, this means Qt::color0.

If the given rectangle is a null rectangle the entire image is copied.

See also QImage().
*/
impl /*struct*/ QImage {
  pub fn copy_1<RetType, T: QImage_copy_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.copy_1(self);
    // return 1;
  }
}
pub trait QImage_copy_1<RetType> {
  fn copy_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_copy_1<usize> for (i32,i32,i32,i32) {
  fn copy_1(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage4copyEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:179
// index:0
// Public Visibility=Default Availability=Available
// [4] QImage::Format format() const

/*
Returns the format of the image.

See also Image Formats.
*/
impl /*struct*/ QImage {
  pub fn format_0<RetType, T: QImage_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QImage_format_0<RetType> {
  fn format_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_format_0<i32> for () {
  fn format_0(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:182
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QImage convertToFormat(QImage::Format, Qt::ImageConversionFlags) const

/*
Returns a copy of the image in the given format.

The specified image conversion flags control how the image data is handled during the conversion process.

See also Image Formats.
*/
impl /*struct*/ QImage {
  pub fn convertToFormat_0<RetType, T: QImage_convertToFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.convertToFormat_0(self);
    // return 1;
  }
}
pub trait QImage_convertToFormat_0<RetType> {
  fn convertToFormat_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_convertToFormat_0<usize> for (i32,i32) {
  fn convertToFormat_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR6QImage15convertToFormatENS_6FormatE6QFlagsIN2Qt19ImageConversionFlagEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:184
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QImage convertToFormat(QImage::Format, Qt::ImageConversionFlags)

/*
Returns a copy of the image in the given format.

The specified image conversion flags control how the image data is handled during the conversion process.

See also Image Formats.
*/
impl /*struct*/ QImage {
  pub fn convertToFormat_1<RetType, T: QImage_convertToFormat_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.convertToFormat_1(self);
    // return 1;
  }
}
pub trait QImage_convertToFormat_1<RetType> {
  fn convertToFormat_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_convertToFormat_1<usize> for (i32,i32) {
  fn convertToFormat_1(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNO6QImage15convertToFormatENS_6FormatE6QFlagsIN2Qt19ImageConversionFlagEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:195
// index:0
// Public Visibility=Default Availability=Available
// [1] bool reinterpretAsFormat(QImage::Format)

/*
Changes the format of the image without changing the data. Only works between formats of the same depth.

Returns true if successful.

This function can be used to change images with alpha-channels to their corresponding opaque formats if the data is known to be opaque-only, or to change the format of a given image buffer before overwriting it with new data.

Warning: The function does not check if the image data is valid in the new format and will still return true if the depths are compatible. Operations on an image with invalid data are undefined.

Warning: If the image is not detached, this will cause the data to be copied.

This function was introduced in  Qt 5.9.

See also hasAlphaChannel() and convertToFormat().
*/
impl /*struct*/ QImage {
  pub fn reinterpretAsFormat_0<RetType, T: QImage_reinterpretAsFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reinterpretAsFormat_0(self);
    // return 1;
  }
}
pub trait QImage_reinterpretAsFormat_0<RetType> {
  fn reinterpretAsFormat_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_reinterpretAsFormat_0<bool> for (i32) {
  fn reinterpretAsFormat_0(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage19reinterpretAsFormatENS_6FormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:197
// index:0
// Public Visibility=Default Availability=Available
// [4] int width() const

/*
Returns the width of the image.

See also Image Information.
*/
impl /*struct*/ QImage {
  pub fn width_0<RetType, T: QImage_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QImage_width_0<RetType> {
  fn width_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_width_0<i32> for () {
  fn width_0(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:198
// index:0
// Public Visibility=Default Availability=Available
// [4] int height() const

/*
Returns the height of the image.

See also Image Information.
*/
impl /*struct*/ QImage {
  pub fn height_0<RetType, T: QImage_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QImage_height_0<RetType> {
  fn height_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_height_0<i32> for () {
  fn height_0(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:199
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize size() const

/*
Returns the size of the image, i.e. its width() and height().

See also Image Information.
*/
impl /*struct*/ QImage {
  pub fn size_0<RetType, T: QImage_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QImage_size_0<RetType> {
  fn size_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_size_0<usize> for () {
  fn size_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:200
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect rect() const

/*
Returns the enclosing rectangle (0, 0, width(), height()) of the image.

See also Image Information.
*/
impl /*struct*/ QImage {
  pub fn rect_0<RetType, T: QImage_rect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rect_0(self);
    // return 1;
  }
}
pub trait QImage_rect_0<RetType> {
  fn rect_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_rect_0<usize> for () {
  fn rect_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage4rectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:202
// index:0
// Public Visibility=Default Availability=Available
// [4] int depth() const

/*
Returns the depth of the image.

The image depth is the number of bits used to store a single pixel, also called bits per pixel (bpp).

The supported depths are 1, 8, 16, 24 and 32.

See also bitPlaneCount(), convertToFormat(), Image Formats, and Image Information.
*/
impl /*struct*/ QImage {
  pub fn depth_0<RetType, T: QImage_depth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.depth_0(self);
    // return 1;
  }
}
pub trait QImage_depth_0<RetType> {
  fn depth_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_depth_0<i32> for () {
  fn depth_0(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage5depthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:203
// index:0
// Public Visibility=Default Availability=Available
// [4] int colorCount() const

/*
Returns the size of the color table for the image.

Notice that colorCount() returns 0 for 32-bpp images because these images do not use color tables, but instead encode pixel values as ARGB quadruplets.

This function was introduced in  Qt 4.6.

See also setColorCount() and Image Information.
*/
impl /*struct*/ QImage {
  pub fn colorCount_0<RetType, T: QImage_colorCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.colorCount_0(self);
    // return 1;
  }
}
pub trait QImage_colorCount_0<RetType> {
  fn colorCount_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_colorCount_0<i32> for () {
  fn colorCount_0(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage10colorCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:204
// index:0
// Public Visibility=Default Availability=Available
// [4] int bitPlaneCount() const

/*
Returns the number of bit planes in the image.

The number of bit planes is the number of bits of color and transparency information for each pixel. This is different from (i.e. smaller than) the depth when the image format contains unused bits.

This function was introduced in  Qt 4.7.

See also depth(), format(), and Image Formats.
*/
impl /*struct*/ QImage {
  pub fn bitPlaneCount_0<RetType, T: QImage_bitPlaneCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bitPlaneCount_0(self);
    // return 1;
  }
}
pub trait QImage_bitPlaneCount_0<RetType> {
  fn bitPlaneCount_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_bitPlaneCount_0<i32> for () {
  fn bitPlaneCount_0(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage13bitPlaneCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:206
// index:0
// Public Visibility=Default Availability=Available
// [4] QRgb color(int) const

/*
Returns the color in the color table at index i. The first color is at index 0.

The colors in an image's color table are specified as ARGB quadruplets (QRgb). Use the qAlpha(), qRed(), qGreen(), and qBlue() functions to get the color value components.

See also setColor(), pixelIndex(), and Pixel Manipulation.
*/
impl /*struct*/ QImage {
  pub fn color_0<RetType, T: QImage_color_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.color_0(self);
    // return 1;
  }
}
pub trait QImage_color_0<RetType> {
  fn color_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_color_0<u32> for (i32) {
  fn color_0(self , rsthis: & QImage) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage5colorEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:207
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColor(int, QRgb)

/*
Sets the color at the given index in the color table, to the given to colorValue. The color value is an ARGB quadruplet.

If index is outside the current size of the color table, it is expanded with setColorCount().

See also color(), colorCount(), setColorTable(), and Pixel Manipulation.
*/
impl /*struct*/ QImage {
  pub fn setColor_0<RetType, T: QImage_setColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColor_0(self);
    // return 1;
  }
}
pub trait QImage_setColor_0<RetType> {
  fn setColor_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_setColor_0<(/*void*/)> for (i32,u32) {
  fn setColor_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const u32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage8setColorEij", 2,qtrt::FFITY_INT,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:208
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColorCount(int)

/*
Resizes the color table to contain colorCount entries.

If the color table is expanded, all the extra colors will be set to transparent (i.e qRgba(0, 0, 0, 0)).

When the image is used, the color table must be large enough to have entries for all the pixel/index values present in the image, otherwise the results are undefined.

This function was introduced in  Qt 4.6.

See also colorCount(), colorTable(), setColor(), and Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn setColorCount_0<RetType, T: QImage_setColorCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColorCount_0(self);
    // return 1;
  }
}
pub trait QImage_setColorCount_0<RetType> {
  fn setColorCount_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_setColorCount_0<(/*void*/)> for (i32) {
  fn setColorCount_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage13setColorCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:210
// index:0
// Public Visibility=Default Availability=Available
// [1] bool allGray() const

/*
Returns true if all the colors in the image are shades of gray (i.e. their red, green and blue components are equal); otherwise false.

Note that this function is slow for images without color table.

See also isGrayscale().
*/
impl /*struct*/ QImage {
  pub fn allGray_0<RetType, T: QImage_allGray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.allGray_0(self);
    // return 1;
  }
}
pub trait QImage_allGray_0<RetType> {
  fn allGray_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_allGray_0<bool> for () {
  fn allGray_0(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage7allGrayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:211
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isGrayscale() const

/*
For 32-bit images, this function is equivalent to allGray().

For color indexed images, this function returns true if color(i) is QRgb(i, i, i) for all indexes of the color table; otherwise returns false.

See also allGray() and Image Formats.
*/
impl /*struct*/ QImage {
  pub fn isGrayscale_0<RetType, T: QImage_isGrayscale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isGrayscale_0(self);
    // return 1;
  }
}
pub trait QImage_isGrayscale_0<RetType> {
  fn isGrayscale_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_isGrayscale_0<bool> for () {
  fn isGrayscale_0(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage11isGrayscaleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:213
// index:0
// Public Visibility=Default Availability=Available
// [8] uchar * bits()

/*
Returns a pointer to the first pixel data. This is equivalent to scanLine(0).

Note that QImage uses implicit data sharing. This function performs a deep copy of the shared pixel data, thus ensuring that this QImage is the only one using the current return value.

See also scanLine(), sizeInBytes(), and constBits().
*/
impl /*struct*/ QImage {
  pub fn bits_0<RetType, T: QImage_bits_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bits_0(self);
    // return 1;
  }
}
pub trait QImage_bits_0<RetType> {
  fn bits_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_bits_0<usize> for () {
  fn bits_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage4bitsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:214
// index:1
// Public Visibility=Default Availability=Available
// [8] const uchar * bits() const

/*
Returns a pointer to the first pixel data. This is equivalent to scanLine(0).

Note that QImage uses implicit data sharing. This function performs a deep copy of the shared pixel data, thus ensuring that this QImage is the only one using the current return value.

See also scanLine(), sizeInBytes(), and constBits().
*/
impl /*struct*/ QImage {
  pub fn bits_1<RetType, T: QImage_bits_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bits_1(self);
    // return 1;
  }
}
pub trait QImage_bits_1<RetType> {
  fn bits_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_bits_1<usize> for () {
  fn bits_1(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage4bitsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:215
// index:0
// Public Visibility=Default Availability=Available
// [8] const uchar * constBits() const

/*
Returns a pointer to the first pixel data.

Note that QImage uses implicit data sharing, but this function does not perform a deep copy of the shared pixel data, because the returned data is const.

This function was introduced in  Qt 4.7.

See also bits() and constScanLine().
*/
impl /*struct*/ QImage {
  pub fn constBits_0<RetType, T: QImage_constBits_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constBits_0(self);
    // return 1;
  }
}
pub trait QImage_constBits_0<RetType> {
  fn constBits_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_constBits_0<usize> for () {
  fn constBits_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage9constBitsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:218
// index:0
// Public Visibility=Default Availability=Available
// [4] int byteCount() const

/*

*/
impl /*struct*/ QImage {
  pub fn byteCount_0<RetType, T: QImage_byteCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.byteCount_0(self);
    // return 1;
  }
}
pub trait QImage_byteCount_0<RetType> {
  fn byteCount_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_byteCount_0<i32> for () {
  fn byteCount_0(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage9byteCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:220
// index:0
// Public Visibility=Default Availability=Available
// [8] qsizetype sizeInBytes() const

/*
Returns the image data size in bytes.

This function was introduced in  Qt 5.10.

See also byteCount(), bytesPerLine(), bits(), and Image Information.
*/
impl /*struct*/ QImage {
  pub fn sizeInBytes_0<RetType, T: QImage_sizeInBytes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeInBytes_0(self);
    // return 1;
  }
}
pub trait QImage_sizeInBytes_0<RetType> {
  fn sizeInBytes_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_sizeInBytes_0<i64> for () {
  fn sizeInBytes_0(self , rsthis: & QImage) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage11sizeInBytesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:222
// index:0
// Public Visibility=Default Availability=Available
// [8] uchar * scanLine(int)

/*
Returns a pointer to the pixel data at the scanline with index i. The first scanline is at index 0.

The scanline data is aligned on a 32-bit boundary.

Warning: If you are accessing 32-bpp image data, cast the returned pointer to QRgb* (QRgb has a 32-bit size) and use it to read/write the pixel value. You cannot use the uchar* pointer directly, because the pixel format depends on the byte order on the underlying platform. Use qRed(), qGreen(), qBlue(), and qAlpha() to access the pixels.

See also bytesPerLine(), bits(), Pixel Manipulation, and constScanLine().
*/
impl /*struct*/ QImage {
  pub fn scanLine_0<RetType, T: QImage_scanLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scanLine_0(self);
    // return 1;
  }
}
pub trait QImage_scanLine_0<RetType> {
  fn scanLine_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_scanLine_0<usize> for (i32) {
  fn scanLine_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage8scanLineEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:223
// index:1
// Public Visibility=Default Availability=Available
// [8] const uchar * scanLine(int) const

/*
Returns a pointer to the pixel data at the scanline with index i. The first scanline is at index 0.

The scanline data is aligned on a 32-bit boundary.

Warning: If you are accessing 32-bpp image data, cast the returned pointer to QRgb* (QRgb has a 32-bit size) and use it to read/write the pixel value. You cannot use the uchar* pointer directly, because the pixel format depends on the byte order on the underlying platform. Use qRed(), qGreen(), qBlue(), and qAlpha() to access the pixels.

See also bytesPerLine(), bits(), Pixel Manipulation, and constScanLine().
*/
impl /*struct*/ QImage {
  pub fn scanLine_1<RetType, T: QImage_scanLine_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scanLine_1(self);
    // return 1;
  }
}
pub trait QImage_scanLine_1<RetType> {
  fn scanLine_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_scanLine_1<usize> for (i32) {
  fn scanLine_1(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage8scanLineEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:224
// index:0
// Public Visibility=Default Availability=Available
// [8] const uchar * constScanLine(int) const

/*
Returns a pointer to the pixel data at the scanline with index i. The first scanline is at index 0.

The scanline data is aligned on a 32-bit boundary.

Note that QImage uses implicit data sharing, but this function does not perform a deep copy of the shared pixel data, because the returned data is const.

This function was introduced in  Qt 4.7.

See also scanLine() and constBits().
*/
impl /*struct*/ QImage {
  pub fn constScanLine_0<RetType, T: QImage_constScanLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constScanLine_0(self);
    // return 1;
  }
}
pub trait QImage_constScanLine_0<RetType> {
  fn constScanLine_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_constScanLine_0<usize> for (i32) {
  fn constScanLine_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage13constScanLineEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:225
// index:0
// Public Visibility=Default Availability=Available
// [4] int bytesPerLine() const

/*
Returns the number of bytes per image scanline.

This is equivalent to sizeInBytes() / height() if height() is non-zero.

See also scanLine().
*/
impl /*struct*/ QImage {
  pub fn bytesPerLine_0<RetType, T: QImage_bytesPerLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bytesPerLine_0(self);
    // return 1;
  }
}
pub trait QImage_bytesPerLine_0<RetType> {
  fn bytesPerLine_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_bytesPerLine_0<i32> for () {
  fn bytesPerLine_0(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage12bytesPerLineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:227
// index:0
// Public Visibility=Default Availability=Available
// [1] bool valid(int, int) const

/*
Returns true if pos is a valid coordinate pair within the image; otherwise returns false.

See also rect() and QRect::contains().
*/
impl /*struct*/ QImage {
  pub fn valid_0<RetType, T: QImage_valid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valid_0(self);
    // return 1;
  }
}
pub trait QImage_valid_0<RetType> {
  fn valid_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_valid_0<bool> for (i32,i32) {
  fn valid_0(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage5validEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:228
// index:1
// Public Visibility=Default Availability=Available
// [1] bool valid(const QPoint &) const

/*
Returns true if pos is a valid coordinate pair within the image; otherwise returns false.

See also rect() and QRect::contains().
*/
impl /*struct*/ QImage {
  pub fn valid_1<RetType, T: QImage_valid_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valid_1(self);
    // return 1;
  }
}
pub trait QImage_valid_1<RetType> {
  fn valid_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_valid_1<bool> for (usize) {
  fn valid_1(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage5validERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:230
// index:0
// Public Visibility=Default Availability=Available
// [4] int pixelIndex(int, int) const

/*
Returns the pixel index at the given position.

If position is not valid, or if the image is not a paletted image (depth() > 8), the results are undefined.

See also valid(), depth(), and Pixel Manipulation.
*/
impl /*struct*/ QImage {
  pub fn pixelIndex_0<RetType, T: QImage_pixelIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixelIndex_0(self);
    // return 1;
  }
}
pub trait QImage_pixelIndex_0<RetType> {
  fn pixelIndex_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_pixelIndex_0<i32> for (i32,i32) {
  fn pixelIndex_0(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage10pixelIndexEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:231
// index:1
// Public Visibility=Default Availability=Available
// [4] int pixelIndex(const QPoint &) const

/*
Returns the pixel index at the given position.

If position is not valid, or if the image is not a paletted image (depth() > 8), the results are undefined.

See also valid(), depth(), and Pixel Manipulation.
*/
impl /*struct*/ QImage {
  pub fn pixelIndex_1<RetType, T: QImage_pixelIndex_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixelIndex_1(self);
    // return 1;
  }
}
pub trait QImage_pixelIndex_1<RetType> {
  fn pixelIndex_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_pixelIndex_1<i32> for (usize) {
  fn pixelIndex_1(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage10pixelIndexERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:233
// index:0
// Public Visibility=Default Availability=Available
// [4] QRgb pixel(int, int) const

/*
Returns the color of the pixel at the given position.

If the position is not valid, the results are undefined.

Warning: This function is expensive when used for massive pixel manipulations. Use constBits() or constScanLine() when many pixels needs to be read.

See also setPixel(), valid(), constBits(), constScanLine(), and Pixel Manipulation.
*/
impl /*struct*/ QImage {
  pub fn pixel_0<RetType, T: QImage_pixel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixel_0(self);
    // return 1;
  }
}
pub trait QImage_pixel_0<RetType> {
  fn pixel_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_pixel_0<u32> for (i32,i32) {
  fn pixel_0(self , rsthis: & QImage) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage5pixelEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:234
// index:1
// Public Visibility=Default Availability=Available
// [4] QRgb pixel(const QPoint &) const

/*
Returns the color of the pixel at the given position.

If the position is not valid, the results are undefined.

Warning: This function is expensive when used for massive pixel manipulations. Use constBits() or constScanLine() when many pixels needs to be read.

See also setPixel(), valid(), constBits(), constScanLine(), and Pixel Manipulation.
*/
impl /*struct*/ QImage {
  pub fn pixel_1<RetType, T: QImage_pixel_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixel_1(self);
    // return 1;
  }
}
pub trait QImage_pixel_1<RetType> {
  fn pixel_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_pixel_1<u32> for (usize) {
  fn pixel_1(self , rsthis: & QImage) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage5pixelERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:236
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPixel(int, int, uint)

/*
Sets the pixel index or color at the given position to index_or_rgb.

If the image's format is either monochrome or paletted, the given index_or_rgb value must be an index in the image's color table, otherwise the parameter must be a QRgb value.

If position is not a valid coordinate pair in the image, or if index_or_rgb >= colorCount() in the case of monochrome and paletted images, the result is undefined.

Warning: This function is expensive due to the call of the internal detach() function called within; if performance is a concern, we recommend the use of scanLine() or bits() to access pixel data directly.

See also pixel() and Pixel Manipulation.
*/
impl /*struct*/ QImage {
  pub fn setPixel_0<RetType, T: QImage_setPixel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPixel_0(self);
    // return 1;
  }
}
pub trait QImage_setPixel_0<RetType> {
  fn setPixel_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_setPixel_0<(/*void*/)> for (i32,i32,u32) {
  fn setPixel_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const u32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage8setPixelEiij", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:237
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setPixel(const QPoint &, uint)

/*
Sets the pixel index or color at the given position to index_or_rgb.

If the image's format is either monochrome or paletted, the given index_or_rgb value must be an index in the image's color table, otherwise the parameter must be a QRgb value.

If position is not a valid coordinate pair in the image, or if index_or_rgb >= colorCount() in the case of monochrome and paletted images, the result is undefined.

Warning: This function is expensive due to the call of the internal detach() function called within; if performance is a concern, we recommend the use of scanLine() or bits() to access pixel data directly.

See also pixel() and Pixel Manipulation.
*/
impl /*struct*/ QImage {
  pub fn setPixel_1<RetType, T: QImage_setPixel_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPixel_1(self);
    // return 1;
  }
}
pub trait QImage_setPixel_1<RetType> {
  fn setPixel_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_setPixel_1<(/*void*/)> for (usize,u32) {
  fn setPixel_1(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const u32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage8setPixelERK6QPointj", 2,qtrt::FFITY_POINTER,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:239
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor pixelColor(int, int) const

/*
Returns the color of the pixel at the given position as a QColor.

If the position is not valid, an invalid QColor is returned.

Warning: This function is expensive when used for massive pixel manipulations. Use constBits() or constScanLine() when many pixels needs to be read.

This function was introduced in  Qt 5.6.

See also setPixelColor(), setPixel(), valid(), constBits(), constScanLine(), and Pixel Manipulation.
*/
impl /*struct*/ QImage {
  pub fn pixelColor_0<RetType, T: QImage_pixelColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixelColor_0(self);
    // return 1;
  }
}
pub trait QImage_pixelColor_0<RetType> {
  fn pixelColor_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_pixelColor_0<usize> for (i32,i32) {
  fn pixelColor_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage10pixelColorEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:240
// index:1
// Public Visibility=Default Availability=Available
// [16] QColor pixelColor(const QPoint &) const

/*
Returns the color of the pixel at the given position as a QColor.

If the position is not valid, an invalid QColor is returned.

Warning: This function is expensive when used for massive pixel manipulations. Use constBits() or constScanLine() when many pixels needs to be read.

This function was introduced in  Qt 5.6.

See also setPixelColor(), setPixel(), valid(), constBits(), constScanLine(), and Pixel Manipulation.
*/
impl /*struct*/ QImage {
  pub fn pixelColor_1<RetType, T: QImage_pixelColor_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixelColor_1(self);
    // return 1;
  }
}
pub trait QImage_pixelColor_1<RetType> {
  fn pixelColor_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_pixelColor_1<usize> for (usize) {
  fn pixelColor_1(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage10pixelColorERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:242
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPixelColor(int, int, const QColor &)

/*
Sets the color at the given position to color.

If position is not a valid coordinate pair in the image, or the image's format is either monochrome or paletted, the result is undefined.

Warning: This function is expensive due to the call of the internal detach() function called within; if performance is a concern, we recommend the use of scanLine() or bits() to access pixel data directly.

This function was introduced in  Qt 5.6.

See also pixelColor(), pixel(), bits(), scanLine(), and Pixel Manipulation.
*/
impl /*struct*/ QImage {
  pub fn setPixelColor_0<RetType, T: QImage_setPixelColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPixelColor_0(self);
    // return 1;
  }
}
pub trait QImage_setPixelColor_0<RetType> {
  fn setPixelColor_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_setPixelColor_0<(/*void*/)> for (i32,i32,usize) {
  fn setPixelColor_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage13setPixelColorEiiRK6QColor", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:243
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setPixelColor(const QPoint &, const QColor &)

/*
Sets the color at the given position to color.

If position is not a valid coordinate pair in the image, or the image's format is either monochrome or paletted, the result is undefined.

Warning: This function is expensive due to the call of the internal detach() function called within; if performance is a concern, we recommend the use of scanLine() or bits() to access pixel data directly.

This function was introduced in  Qt 5.6.

See also pixelColor(), pixel(), bits(), scanLine(), and Pixel Manipulation.
*/
impl /*struct*/ QImage {
  pub fn setPixelColor_1<RetType, T: QImage_setPixelColor_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPixelColor_1(self);
    // return 1;
  }
}
pub trait QImage_setPixelColor_1<RetType> {
  fn setPixelColor_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_setPixelColor_1<(/*void*/)> for (usize,usize) {
  fn setPixelColor_1(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage13setPixelColorERK6QPointRK6QColor", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:252
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal devicePixelRatio() const

/*
Returns the device pixel ratio for the image. This is the ratio between device pixels and device independent pixels.

Use this function when calculating layout geometry based on the image size: QSize layoutSize = image.size() / image.devicePixelRatio()

The default value is 1.0.

See also setDevicePixelRatio() and QImageReader.
*/
impl /*struct*/ QImage {
  pub fn devicePixelRatio_0<RetType, T: QImage_devicePixelRatio_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio_0(self);
    // return 1;
  }
}
pub trait QImage_devicePixelRatio_0<RetType> {
  fn devicePixelRatio_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_devicePixelRatio_0<f64> for () {
  fn devicePixelRatio_0(self , rsthis: & QImage) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage16devicePixelRatioEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:253
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDevicePixelRatio(qreal)

/*
Sets the device pixel ratio for the image. This is the ratio between image pixels and device-independent pixels.

The default scaleFactor is 1.0. Setting it to something else has two effects:

QPainters that are opened on the image will be scaled. For example, painting on a 200x200 image if with a ratio of 2.0 will result in effective (device-independent) painting bounds of 100x100.

Code paths in Qt that calculate layout geometry based on the image size will take the ratio into account: QSize layoutSize = image.size() / image.devicePixelRatio() The net effect of this is that the image is displayed as high-DPI image rather than a large image (see Drawing High Resolution Versions of Pixmaps and Images).

See also devicePixelRatio().
*/
impl /*struct*/ QImage {
  pub fn setDevicePixelRatio_0<RetType, T: QImage_setDevicePixelRatio_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDevicePixelRatio_0(self);
    // return 1;
  }
}
pub trait QImage_setDevicePixelRatio_0<RetType> {
  fn setDevicePixelRatio_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_setDevicePixelRatio_0<(/*void*/)> for (f64) {
  fn setDevicePixelRatio_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage19setDevicePixelRatioEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:255
// index:0
// Public Visibility=Default Availability=Available
// [-2] void fill(uint)

/*
Fills the entire image with the given pixelValue.

If the depth of this image is 1, only the lowest bit is used. If you say fill(0), fill(2), etc., the image is filled with 0s. If you say fill(1), fill(3), etc., the image is filled with 1s. If the depth is 8, the lowest 8 bits are used and if the depth is 16 the lowest 16 bits are used.

Note: QImage::pixel() returns the color of the pixel at the given coordinates while QColor::pixel() returns the pixel value of the underlying window system (essentially an index value), so normally you will want to use QImage::pixel() to use a color from an existing image or QColor::rgb() to use a specific color.

See also depth() and Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn fill_0<RetType, T: QImage_fill_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fill_0(self);
    // return 1;
  }
}
pub trait QImage_fill_0<RetType> {
  fn fill_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_fill_0<(/*void*/)> for (u32) {
  fn fill_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage4fillEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:256
// index:1
// Public Visibility=Default Availability=Available
// [-2] void fill(const QColor &)

/*
Fills the entire image with the given pixelValue.

If the depth of this image is 1, only the lowest bit is used. If you say fill(0), fill(2), etc., the image is filled with 0s. If you say fill(1), fill(3), etc., the image is filled with 1s. If the depth is 8, the lowest 8 bits are used and if the depth is 16 the lowest 16 bits are used.

Note: QImage::pixel() returns the color of the pixel at the given coordinates while QColor::pixel() returns the pixel value of the underlying window system (essentially an index value), so normally you will want to use QImage::pixel() to use a color from an existing image or QColor::rgb() to use a specific color.

See also depth() and Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn fill_1<RetType, T: QImage_fill_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fill_1(self);
    // return 1;
  }
}
pub trait QImage_fill_1<RetType> {
  fn fill_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_fill_1<(/*void*/)> for (usize) {
  fn fill_1(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage4fillERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:257
// index:2
// Public Visibility=Default Availability=Available
// [-2] void fill(Qt::GlobalColor)

/*
Fills the entire image with the given pixelValue.

If the depth of this image is 1, only the lowest bit is used. If you say fill(0), fill(2), etc., the image is filled with 0s. If you say fill(1), fill(3), etc., the image is filled with 1s. If the depth is 8, the lowest 8 bits are used and if the depth is 16 the lowest 16 bits are used.

Note: QImage::pixel() returns the color of the pixel at the given coordinates while QColor::pixel() returns the pixel value of the underlying window system (essentially an index value), so normally you will want to use QImage::pixel() to use a color from an existing image or QColor::rgb() to use a specific color.

See also depth() and Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn fill_2<RetType, T: QImage_fill_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fill_2(self);
    // return 1;
  }
}
pub trait QImage_fill_2<RetType> {
  fn fill_2(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_fill_2<(/*void*/)> for (i32) {
  fn fill_2(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage4fillEN2Qt11GlobalColorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:260
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasAlphaChannel() const

/*
Returns true if the image has a format that respects the alpha channel, otherwise returns false.

See also Image Information.
*/
impl /*struct*/ QImage {
  pub fn hasAlphaChannel_0<RetType, T: QImage_hasAlphaChannel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasAlphaChannel_0(self);
    // return 1;
  }
}
pub trait QImage_hasAlphaChannel_0<RetType> {
  fn hasAlphaChannel_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_hasAlphaChannel_0<bool> for () {
  fn hasAlphaChannel_0(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage15hasAlphaChannelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:261
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlphaChannel(const QImage &)

/*

*/
impl /*struct*/ QImage {
  pub fn setAlphaChannel_0<RetType, T: QImage_setAlphaChannel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlphaChannel_0(self);
    // return 1;
  }
}
pub trait QImage_setAlphaChannel_0<RetType> {
  fn setAlphaChannel_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_setAlphaChannel_0<(/*void*/)> for (usize) {
  fn setAlphaChannel_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage15setAlphaChannelERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:262
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage alphaChannel() const

/*

*/
impl /*struct*/ QImage {
  pub fn alphaChannel_0<RetType, T: QImage_alphaChannel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alphaChannel_0(self);
    // return 1;
  }
}
pub trait QImage_alphaChannel_0<RetType> {
  fn alphaChannel_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_alphaChannel_0<usize> for () {
  fn alphaChannel_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage12alphaChannelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:263
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage createAlphaMask(Qt::ImageConversionFlags) const

/*
Builds and returns a 1-bpp mask from the alpha buffer in this image. Returns a null image if the image's format is QImage::Format_RGB32.

The flags argument is a bitwise-OR of the Qt::ImageConversionFlags, and controls the conversion process. Passing 0 for flags sets all the default options.

The returned image has little-endian bit order (i.e. the image's format is QImage::Format_MonoLSB), which you can convert to big-endian (QImage::Format_Mono) using the convertToFormat() function.

See also createHeuristicMask() and Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn createAlphaMask_0<RetType, T: QImage_createAlphaMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createAlphaMask_0(self);
    // return 1;
  }
}
pub trait QImage_createAlphaMask_0<RetType> {
  fn createAlphaMask_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_createAlphaMask_0<usize> for (i32) {
  fn createAlphaMask_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage15createAlphaMaskE6QFlagsIN2Qt19ImageConversionFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:265
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage createHeuristicMask(bool) const

/*
Creates and returns a 1-bpp heuristic mask for this image.

The function works by selecting a color from one of the corners, then chipping away pixels of that color starting at all the edges. The four corners vote for which color is to be masked away. In case of a draw (this generally means that this function is not applicable to the image), the result is arbitrary.

The returned image has little-endian bit order (i.e. the image's format is QImage::Format_MonoLSB), which you can convert to big-endian (QImage::Format_Mono) using the convertToFormat() function.

If clipTight is true (the default) the mask is just large enough to cover the pixels; otherwise, the mask is larger than the data pixels.

Note that this function disregards the alpha buffer.

See also createAlphaMask() and Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn createHeuristicMask_0<RetType, T: QImage_createHeuristicMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createHeuristicMask_0(self);
    // return 1;
  }
}
pub trait QImage_createHeuristicMask_0<RetType> {
  fn createHeuristicMask_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_createHeuristicMask_0<usize> for (bool) {
  fn createHeuristicMask_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage19createHeuristicMaskEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:267
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage createMaskFromColor(QRgb, Qt::MaskMode) const

/*
Creates and returns a mask for this image based on the given color value. If the mode is MaskInColor (the default value), all pixels matching color will be opaque pixels in the mask. If mode is MaskOutColor, all pixels matching the given color will be transparent.

See also createAlphaMask() and createHeuristicMask().
*/
impl /*struct*/ QImage {
  pub fn createMaskFromColor_0<RetType, T: QImage_createMaskFromColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createMaskFromColor_0(self);
    // return 1;
  }
}
pub trait QImage_createMaskFromColor_0<RetType> {
  fn createMaskFromColor_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_createMaskFromColor_0<usize> for (u32,i32) {
  fn createMaskFromColor_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const u32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage19createMaskFromColorEjN2Qt8MaskModeE", 2,qtrt::FFITY_UINT32,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:269
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QImage scaled(int, int, Qt::AspectRatioMode, Qt::TransformationMode) const

/*
Returns a copy of the image scaled to a rectangle defined by the given size according to the given aspectRatioMode and transformMode.




If aspectRatioMode is Qt::IgnoreAspectRatio, the image is scaled to size.
If aspectRatioMode is Qt::KeepAspectRatio, the image is scaled to a rectangle as large as possible inside size, preserving the aspect ratio.
If aspectRatioMode is Qt::KeepAspectRatioByExpanding, the image is scaled to a rectangle as small as possible outside size, preserving the aspect ratio.


If the given size is empty, this function returns a null image.

See also isNull() and Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn scaled_0<RetType, T: QImage_scaled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaled_0(self);
    // return 1;
  }
}
pub trait QImage_scaled_0<RetType> {
  fn scaled_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_scaled_0<usize> for (i32,i32,i32,i32) {
  fn scaled_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage6scaledEiiN2Qt15AspectRatioModeENS0_18TransformationModeE", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:272
// index:1
// Public Visibility=Default Availability=Available
// [32] QImage scaled(const QSize &, Qt::AspectRatioMode, Qt::TransformationMode) const

/*
Returns a copy of the image scaled to a rectangle defined by the given size according to the given aspectRatioMode and transformMode.




If aspectRatioMode is Qt::IgnoreAspectRatio, the image is scaled to size.
If aspectRatioMode is Qt::KeepAspectRatio, the image is scaled to a rectangle as large as possible inside size, preserving the aspect ratio.
If aspectRatioMode is Qt::KeepAspectRatioByExpanding, the image is scaled to a rectangle as small as possible outside size, preserving the aspect ratio.


If the given size is empty, this function returns a null image.

See also isNull() and Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn scaled_1<RetType, T: QImage_scaled_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaled_1(self);
    // return 1;
  }
}
pub trait QImage_scaled_1<RetType> {
  fn scaled_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_scaled_1<usize> for (usize,i32,i32) {
  fn scaled_1(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage6scaledERK5QSizeN2Qt15AspectRatioModeENS3_18TransformationModeE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:274
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage scaledToWidth(int, Qt::TransformationMode) const

/*
Returns a scaled copy of the image. The returned image is scaled to the given width using the specified transformation mode.

This function automatically calculates the height of the image so that its aspect ratio is preserved.

If the given width is 0 or negative, a null image is returned.

See also Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn scaledToWidth_0<RetType, T: QImage_scaledToWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaledToWidth_0(self);
    // return 1;
  }
}
pub trait QImage_scaledToWidth_0<RetType> {
  fn scaledToWidth_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_scaledToWidth_0<usize> for (i32,i32) {
  fn scaledToWidth_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage13scaledToWidthEiN2Qt18TransformationModeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:275
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage scaledToHeight(int, Qt::TransformationMode) const

/*
Returns a scaled copy of the image. The returned image is scaled to the given height using the specified transformation mode.

This function automatically calculates the width of the image so that the ratio of the image is preserved.

If the given height is 0 or negative, a null image is returned.

See also Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn scaledToHeight_0<RetType, T: QImage_scaledToHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaledToHeight_0(self);
    // return 1;
  }
}
pub trait QImage_scaledToHeight_0<RetType> {
  fn scaledToHeight_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_scaledToHeight_0<usize> for (i32,i32) {
  fn scaledToHeight_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage14scaledToHeightEiN2Qt18TransformationModeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:276
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage transformed(const QMatrix &, Qt::TransformationMode) const

/*
Returns a copy of the image that is transformed using the given transformation matrix and transformation mode.

The transformation matrix is internally adjusted to compensate for unwanted translation; i.e. the image produced is the smallest image that contains all the transformed points of the original image. Use the trueMatrix() function to retrieve the actual matrix used for transforming an image.

See also trueMatrix() and Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn transformed_0<RetType, T: QImage_transformed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transformed_0(self);
    // return 1;
  }
}
pub trait QImage_transformed_0<RetType> {
  fn transformed_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_transformed_0<usize> for (usize,i32) {
  fn transformed_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage11transformedERK7QMatrixN2Qt18TransformationModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:278
// index:1
// Public Visibility=Default Availability=Available
// [32] QImage transformed(const QTransform &, Qt::TransformationMode) const

/*
Returns a copy of the image that is transformed using the given transformation matrix and transformation mode.

The transformation matrix is internally adjusted to compensate for unwanted translation; i.e. the image produced is the smallest image that contains all the transformed points of the original image. Use the trueMatrix() function to retrieve the actual matrix used for transforming an image.

See also trueMatrix() and Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn transformed_1<RetType, T: QImage_transformed_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transformed_1(self);
    // return 1;
  }
}
pub trait QImage_transformed_1<RetType> {
  fn transformed_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_transformed_1<usize> for (usize,i32) {
  fn transformed_1(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage11transformedERK10QTransformN2Qt18TransformationModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:277
// index:0
// Public static Visibility=Default Availability=Available
// [48] QMatrix trueMatrix(const QMatrix &, int, int)

/*
Returns the actual matrix used for transforming an image with the given width, height and matrix.

When transforming an image using the transformed() function, the transformation matrix is internally adjusted to compensate for unwanted translation, i.e. transformed() returns the smallest image containing all transformed points of the original image. This function returns the modified matrix, which maps points correctly from the original image into the new image.

See also transformed() and Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn trueMatrix_0<RetType, T: QImage_trueMatrix_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.trueMatrix_0();
    // return 1;
  }
}
pub trait QImage_trueMatrix_0<RetType> {
  fn trueMatrix_0(self ) -> RetType;
}
impl<'a> /*trait*/ QImage_trueMatrix_0<usize> for (usize,i32,i32) {
  fn trueMatrix_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage10trueMatrixERK7QMatrixii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:279
// index:1
// Public static Visibility=Default Availability=Available
// [88] QTransform trueMatrix(const QTransform &, int, int)

/*
Returns the actual matrix used for transforming an image with the given width, height and matrix.

When transforming an image using the transformed() function, the transformation matrix is internally adjusted to compensate for unwanted translation, i.e. transformed() returns the smallest image containing all transformed points of the original image. This function returns the modified matrix, which maps points correctly from the original image into the new image.

See also transformed() and Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn trueMatrix_1<RetType, T: QImage_trueMatrix_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.trueMatrix_1();
    // return 1;
  }
}
pub trait QImage_trueMatrix_1<RetType> {
  fn trueMatrix_1(self ) -> RetType;
}
impl<'a> /*trait*/ QImage_trueMatrix_1<usize> for (usize,i32,i32) {
  fn trueMatrix_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage10trueMatrixERK10QTransformii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:281
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QImage mirrored(bool, bool) const

/*
Returns a mirror of the image, mirrored in the horizontal and/or the vertical direction depending on whether horizontal and vertical are set to true or false.

Note that the original image is not changed.

See also Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn mirrored_0<RetType, T: QImage_mirrored_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mirrored_0(self);
    // return 1;
  }
}
pub trait QImage_mirrored_0<RetType> {
  fn mirrored_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_mirrored_0<usize> for (bool,bool) {
  fn mirrored_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const bool as usize;
    let arg1 = (&self.1) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR6QImage8mirroredEbb", 2,qtrt::FFITY_SINT8,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:283
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QImage && mirrored(bool, bool)

/*
Returns a mirror of the image, mirrored in the horizontal and/or the vertical direction depending on whether horizontal and vertical are set to true or false.

Note that the original image is not changed.

See also Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn mirrored_1<RetType, T: QImage_mirrored_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mirrored_1(self);
    // return 1;
  }
}
pub trait QImage_mirrored_1<RetType> {
  fn mirrored_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_mirrored_1<usize> for (bool,bool) {
  fn mirrored_1(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const bool as usize;
    let arg1 = (&self.1) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNO6QImage8mirroredEbb", 2,qtrt::FFITY_SINT8,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:285
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QImage rgbSwapped() const

/*
Returns a QImage in which the values of the red and blue components of all pixels have been swapped, effectively converting an RGB image to an BGR image.

The original QImage is not changed.

See also Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn rgbSwapped_0<RetType, T: QImage_rgbSwapped_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rgbSwapped_0(self);
    // return 1;
  }
}
pub trait QImage_rgbSwapped_0<RetType> {
  fn rgbSwapped_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_rgbSwapped_0<usize> for () {
  fn rgbSwapped_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR6QImage10rgbSwappedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:287
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QImage && rgbSwapped()

/*
Returns a QImage in which the values of the red and blue components of all pixels have been swapped, effectively converting an RGB image to an BGR image.

The original QImage is not changed.

See also Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn rgbSwapped_1<RetType, T: QImage_rgbSwapped_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rgbSwapped_1(self);
    // return 1;
  }
}
pub trait QImage_rgbSwapped_1<RetType> {
  fn rgbSwapped_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_rgbSwapped_1<usize> for () {
  fn rgbSwapped_1(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNO6QImage10rgbSwappedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:293
// index:0
// Public Visibility=Default Availability=Available
// [-2] void invertPixels(QImage::InvertMode)

/*
Inverts all pixel values in the image.

The given invert mode only have a meaning when the image's depth is 32. The default mode is InvertRgb, which leaves the alpha channel unchanged. If the mode is InvertRgba, the alpha bits are also inverted.

Inverting an 8-bit image means to replace all pixels using color index i with a pixel using color index 255 minus i. The same is the case for a 1-bit image. Note that the color table is not changed.

If the image has a premultiplied alpha channel, the image is first converted to ARGB32 to be inverted and then converted back.

See also Image Transformations.
*/
impl /*struct*/ QImage {
  pub fn invertPixels_0<RetType, T: QImage_invertPixels_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invertPixels_0(self);
    // return 1;
  }
}
pub trait QImage_invertPixels_0<RetType> {
  fn invertPixels_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_invertPixels_0<(/*void*/)> for (i32) {
  fn invertPixels_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage12invertPixelsENS_10InvertModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:296
// index:0
// Public Visibility=Default Availability=Available
// [1] bool load(QIODevice *, const char *)

/*
Loads an image from the file with the given fileName. Returns true if the image was successfully loaded; otherwise invalidates the image and returns false.

The loader attempts to read the image using the specified format, e.g., PNG or JPG. If format is not specified (which is the default), it is auto-detected based on the file's suffix and header. For details, see {QImageReader::setAutoDetectImageFormat()}{QImageReader}.

The file name can either refer to an actual file on disk or to one of the application's embedded resources. See the Resource System overview for details on how to embed images and other resource files in the application's executable.

See also Reading and Writing Image Files.
*/
impl /*struct*/ QImage {
  pub fn load_0<RetType, T: QImage_load_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.load_0(self);
    // return 1;
  }
}
pub trait QImage_load_0<RetType> {
  fn load_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_load_0<bool> for (usize,usize) {
  fn load_0(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage4loadEP9QIODevicePKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:297
// index:1
// Public Visibility=Default Availability=Available
// [1] bool load(const QString &, const char *)

/*
Loads an image from the file with the given fileName. Returns true if the image was successfully loaded; otherwise invalidates the image and returns false.

The loader attempts to read the image using the specified format, e.g., PNG or JPG. If format is not specified (which is the default), it is auto-detected based on the file's suffix and header. For details, see {QImageReader::setAutoDetectImageFormat()}{QImageReader}.

The file name can either refer to an actual file on disk or to one of the application's embedded resources. See the Resource System overview for details on how to embed images and other resource files in the application's executable.

See also Reading and Writing Image Files.
*/
impl /*struct*/ QImage {
  pub fn load_1<RetType, T: QImage_load_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.load_1(self);
    // return 1;
  }
}
pub trait QImage_load_1<RetType> {
  fn load_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_load_1<bool> for (usize,usize) {
  fn load_1(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage4loadERK7QStringPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:298
// index:0
// Public Visibility=Default Availability=Available
// [1] bool loadFromData(const uchar *, int, const char *)

/*
Loads an image from the first len bytes of the given binary data. Returns true if the image was successfully loaded; otherwise invalidates the image and returns false.

The loader attempts to read the image using the specified format, e.g., PNG or JPG. If format is not specified (which is the default), the loader probes the file for a header to guess the file format.

See also Reading and Writing Image Files.
*/
impl /*struct*/ QImage {
  pub fn loadFromData_0<RetType, T: QImage_loadFromData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loadFromData_0(self);
    // return 1;
  }
}
pub trait QImage_loadFromData_0<RetType> {
  fn loadFromData_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_loadFromData_0<bool> for (usize,i32,usize) {
  fn loadFromData_0(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage12loadFromDataEPKhiPKc", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:299
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool loadFromData(const QByteArray &, const char *)

/*
Loads an image from the first len bytes of the given binary data. Returns true if the image was successfully loaded; otherwise invalidates the image and returns false.

The loader attempts to read the image using the specified format, e.g., PNG or JPG. If format is not specified (which is the default), the loader probes the file for a header to guess the file format.

See also Reading and Writing Image Files.
*/
impl /*struct*/ QImage {
  pub fn loadFromData_1<RetType, T: QImage_loadFromData_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loadFromData_1(self);
    // return 1;
  }
}
pub trait QImage_loadFromData_1<RetType> {
  fn loadFromData_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_loadFromData_1<bool> for (usize,usize) {
  fn loadFromData_1(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage12loadFromDataERK10QByteArrayPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:302
// index:0
// Public Visibility=Default Availability=Available
// [1] bool save(const QString &, const char *, int) const

/*
Saves the image to the file with the given fileName, using the given image file format and quality factor. If format is 0, QImage will attempt to guess the format by looking at fileName's suffix.

The quality factor must be in the range 0 to 100 or -1. Specify 0 to obtain small compressed files, 100 for large uncompressed files, and -1 (the default) to use the default settings.

Returns true if the image was successfully saved; otherwise returns false.

See also Reading and Writing Image Files.
*/
impl /*struct*/ QImage {
  pub fn save_0<RetType, T: QImage_save_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.save_0(self);
    // return 1;
  }
}
pub trait QImage_save_0<RetType> {
  fn save_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_save_0<bool> for (usize,usize,i32) {
  fn save_0(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage4saveERK7QStringPKci", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:303
// index:1
// Public Visibility=Default Availability=Available
// [1] bool save(QIODevice *, const char *, int) const

/*
Saves the image to the file with the given fileName, using the given image file format and quality factor. If format is 0, QImage will attempt to guess the format by looking at fileName's suffix.

The quality factor must be in the range 0 to 100 or -1. Specify 0 to obtain small compressed files, 100 for large uncompressed files, and -1 (the default) to use the default settings.

Returns true if the image was successfully saved; otherwise returns false.

See also Reading and Writing Image Files.
*/
impl /*struct*/ QImage {
  pub fn save_1<RetType, T: QImage_save_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.save_1(self);
    // return 1;
  }
}
pub trait QImage_save_1<RetType> {
  fn save_1(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_save_1<bool> for (usize,usize,i32) {
  fn save_1(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage4saveEP9QIODevicePKci", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:305
// index:0
// Public static Visibility=Default Availability=Available
// [32] QImage fromData(const uchar *, int, const char *)

/*
Constructs a QImage from the first size bytes of the given binary data. The loader attempts to read the image using the specified format. If format is not specified (which is the default), the loader probes the data for a header to guess the file format.

If format is specified, it must be one of the values returned by QImageReader::supportedImageFormats().

If the loading of the image fails, the image returned will be a null image.

See also load(), save(), and Reading and Writing Image Files.
*/
impl /*struct*/ QImage {
  pub fn fromData_0<RetType, T: QImage_fromData_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromData_0();
    // return 1;
  }
}
pub trait QImage_fromData_0<RetType> {
  fn fromData_0(self ) -> RetType;
}
impl<'a> /*trait*/ QImage_fromData_0<usize> for (usize,i32,usize) {
  fn fromData_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage8fromDataEPKhiPKc", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:306
// index:1
// Public static inline Visibility=Default Availability=Available
// [32] QImage fromData(const QByteArray &, const char *)

/*
Constructs a QImage from the first size bytes of the given binary data. The loader attempts to read the image using the specified format. If format is not specified (which is the default), the loader probes the data for a header to guess the file format.

If format is specified, it must be one of the values returned by QImageReader::supportedImageFormats().

If the loading of the image fails, the image returned will be a null image.

See also load(), save(), and Reading and Writing Image Files.
*/
impl /*struct*/ QImage {
  pub fn fromData_1<RetType, T: QImage_fromData_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromData_1();
    // return 1;
  }
}
pub trait QImage_fromData_1<RetType> {
  fn fromData_1(self ) -> RetType;
}
impl<'a> /*trait*/ QImage_fromData_1<usize> for (usize,usize) {
  fn fromData_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage8fromDataERK10QByteArrayPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:312
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 cacheKey() const

/*
Returns a number that identifies the contents of this QImage object. Distinct QImage objects can only have the same key if they refer to the same contents.

The key will change when the image is altered.
*/
impl /*struct*/ QImage {
  pub fn cacheKey_0<RetType, T: QImage_cacheKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cacheKey_0(self);
    // return 1;
  }
}
pub trait QImage_cacheKey_0<RetType> {
  fn cacheKey_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_cacheKey_0<i64> for () {
  fn cacheKey_0(self , rsthis: & QImage) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage8cacheKeyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:314
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QPaintEngine * paintEngine() const

/*

*/
impl /*struct*/ QImage {
  pub fn paintEngine_0<RetType, T: QImage_paintEngine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEngine_0(self);
    // return 1;
  }
}
pub trait QImage_paintEngine_0<RetType> {
  fn paintEngine_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_paintEngine_0<usize> for () {
  fn paintEngine_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage11paintEngineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:317
// index:0
// Public Visibility=Default Availability=Available
// [4] int dotsPerMeterX() const

/*
Returns the number of pixels that fit horizontally in a physical meter. Together with dotsPerMeterY(), this number defines the intended scale and aspect ratio of the image.

See also setDotsPerMeterX() and Image Information.
*/
impl /*struct*/ QImage {
  pub fn dotsPerMeterX_0<RetType, T: QImage_dotsPerMeterX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dotsPerMeterX_0(self);
    // return 1;
  }
}
pub trait QImage_dotsPerMeterX_0<RetType> {
  fn dotsPerMeterX_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_dotsPerMeterX_0<i32> for () {
  fn dotsPerMeterX_0(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage13dotsPerMeterXEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:318
// index:0
// Public Visibility=Default Availability=Available
// [4] int dotsPerMeterY() const

/*
Returns the number of pixels that fit vertically in a physical meter. Together with dotsPerMeterX(), this number defines the intended scale and aspect ratio of the image.

See also setDotsPerMeterY() and Image Information.
*/
impl /*struct*/ QImage {
  pub fn dotsPerMeterY_0<RetType, T: QImage_dotsPerMeterY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dotsPerMeterY_0(self);
    // return 1;
  }
}
pub trait QImage_dotsPerMeterY_0<RetType> {
  fn dotsPerMeterY_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_dotsPerMeterY_0<i32> for () {
  fn dotsPerMeterY_0(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage13dotsPerMeterYEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:319
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDotsPerMeterX(int)

/*
Sets the number of pixels that fit horizontally in a physical meter, to x.

Together with dotsPerMeterY(), this number defines the intended scale and aspect ratio of the image, and determines the scale at which QPainter will draw graphics on the image. It does not change the scale or aspect ratio of the image when it is rendered on other paint devices.

See also dotsPerMeterX() and Image Information.
*/
impl /*struct*/ QImage {
  pub fn setDotsPerMeterX_0<RetType, T: QImage_setDotsPerMeterX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDotsPerMeterX_0(self);
    // return 1;
  }
}
pub trait QImage_setDotsPerMeterX_0<RetType> {
  fn setDotsPerMeterX_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_setDotsPerMeterX_0<(/*void*/)> for (i32) {
  fn setDotsPerMeterX_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage16setDotsPerMeterXEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:320
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDotsPerMeterY(int)

/*
Sets the number of pixels that fit vertically in a physical meter, to y.

Together with dotsPerMeterX(), this number defines the intended scale and aspect ratio of the image, and determines the scale at which QPainter will draw graphics on the image. It does not change the scale or aspect ratio of the image when it is rendered on other paint devices.

See also dotsPerMeterY() and Image Information.
*/
impl /*struct*/ QImage {
  pub fn setDotsPerMeterY_0<RetType, T: QImage_setDotsPerMeterY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDotsPerMeterY_0(self);
    // return 1;
  }
}
pub trait QImage_setDotsPerMeterY_0<RetType> {
  fn setDotsPerMeterY_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_setDotsPerMeterY_0<(/*void*/)> for (i32) {
  fn setDotsPerMeterY_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage16setDotsPerMeterYEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:321
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint offset() const

/*
Returns the number of pixels by which the image is intended to be offset by when positioning relative to other images.

See also setOffset() and Image Information.
*/
impl /*struct*/ QImage {
  pub fn offset_0<RetType, T: QImage_offset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.offset_0(self);
    // return 1;
  }
}
pub trait QImage_offset_0<RetType> {
  fn offset_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_offset_0<usize> for () {
  fn offset_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage6offsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:322
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOffset(const QPoint &)

/*
Sets the number of pixels by which the image is intended to be offset by when positioning relative to other images, to offset.

See also offset() and Image Information.
*/
impl /*struct*/ QImage {
  pub fn setOffset_0<RetType, T: QImage_setOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOffset_0(self);
    // return 1;
  }
}
pub trait QImage_setOffset_0<RetType> {
  fn setOffset_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_setOffset_0<(/*void*/)> for (usize) {
  fn setOffset_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage9setOffsetERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:324
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList textKeys() const

/*
Returns the text keys for this image.

You can use these keys with text() to list the image text for a certain key.

See also text().
*/
impl /*struct*/ QImage {
  pub fn textKeys_0<RetType, T: QImage_textKeys_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textKeys_0(self);
    // return 1;
  }
}
pub trait QImage_textKeys_0<RetType> {
  fn textKeys_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_textKeys_0<usize> for () {
  fn textKeys_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage8textKeysEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:325
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text(const QString &) const

/*
Returns the image text associated with the given key. If the specified key is an empty string, the whole image text is returned, with each key-text pair separated by a newline.

See also setText() and textKeys().
*/
impl /*struct*/ QImage {
  pub fn text_0<RetType, T: QImage_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QImage_text_0<RetType> {
  fn text_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_text_0<usize> for (usize) {
  fn text_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage4textERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:326
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &, const QString &)

/*
Sets the image text to the given text and associate it with the given key.

If you just want to store a single text block (i.e., a "comment" or just a description), you can either pass an empty key, or use a generic key like "Description".

The image text is embedded into the image data when you call save() or QImageWriter::write().

Not all image formats support embedded text. You can find out if a specific image or format supports embedding text by using QImageWriter::supportsOption(). We give an example:


      QImageWriter writer;
      writer.setFormat("png");
      if (writer.supportsOption(QImageIOHandler::Description))
          qDebug() << "Png supports embedded text";



You can use QImageWriter::supportedImageFormats() to find out which image formats are available to you.

See also text() and textKeys().
*/
impl /*struct*/ QImage {
  pub fn setText_0<RetType, T: QImage_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QImage_setText_0<RetType> {
  fn setText_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_setText_0<(/*void*/)> for (usize,usize) {
  fn setText_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage7setTextERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:328
// index:0
// Public Visibility=Default Availability=Available
// [8] QPixelFormat pixelFormat() const

/*
Returns the QImage::Format as a QPixelFormat
*/
impl /*struct*/ QImage {
  pub fn pixelFormat_0<RetType, T: QImage_pixelFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixelFormat_0(self);
    // return 1;
  }
}
pub trait QImage_pixelFormat_0<RetType> {
  fn pixelFormat_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_pixelFormat_0<usize> for () {
  fn pixelFormat_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage11pixelFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:329
// index:0
// Public static Visibility=Default Availability=Available
// [8] QPixelFormat toPixelFormat(QImage::Format)

/*
Converts format into a QPixelFormat
*/
impl /*struct*/ QImage {
  pub fn toPixelFormat_0<RetType, T: QImage_toPixelFormat_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.toPixelFormat_0();
    // return 1;
  }
}
pub trait QImage_toPixelFormat_0<RetType> {
  fn toPixelFormat_0(self ) -> RetType;
}
impl<'a> /*trait*/ QImage_toPixelFormat_0<usize> for (i32) {
  fn toPixelFormat_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage13toPixelFormatENS_6FormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:330
// index:0
// Public static Visibility=Default Availability=Available
// [4] QImage::Format toImageFormat(QPixelFormat)

/*
Converts format into a QImage::Format
*/
impl /*struct*/ QImage {
  pub fn toImageFormat_0<RetType, T: QImage_toImageFormat_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.toImageFormat_0();
    // return 1;
  }
}
pub trait QImage_toImageFormat_0<RetType> {
  fn toImageFormat_0(self ) -> RetType;
}
impl<'a> /*trait*/ QImage_toImageFormat_0<i32> for (usize) {
  fn toImageFormat_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage13toImageFormatE12QPixelFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:352
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int metric(QPaintDevice::PaintDeviceMetric) const

/*

*/
impl /*struct*/ QImage {
  pub fn metric_0<RetType, T: QImage_metric_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metric_0(self);
    // return 1;
  }
}
pub trait QImage_metric_0<RetType> {
  fn metric_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_metric_0<i32> for (i32) {
  fn metric_0(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage6metricEN12QPaintDevice17PaintDeviceMetricE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:353
// index:0
// Protected Visibility=Default Availability=Available
// [32] QImage mirrored_helper(bool, bool) const

/*

*/
impl /*struct*/ QImage {
  pub fn mirrored_helper_0<RetType, T: QImage_mirrored_helper_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mirrored_helper_0(self);
    // return 1;
  }
}
pub trait QImage_mirrored_helper_0<RetType> {
  fn mirrored_helper_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_mirrored_helper_0<usize> for (bool,bool) {
  fn mirrored_helper_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const bool as usize;
    let arg1 = (&self.1) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage15mirrored_helperEbb", 2,qtrt::FFITY_SINT8,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:354
// index:0
// Protected Visibility=Default Availability=Available
// [32] QImage rgbSwapped_helper() const

/*

*/
impl /*struct*/ QImage {
  pub fn rgbSwapped_helper_0<RetType, T: QImage_rgbSwapped_helper_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rgbSwapped_helper_0(self);
    // return 1;
  }
}
pub trait QImage_rgbSwapped_helper_0<RetType> {
  fn rgbSwapped_helper_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_rgbSwapped_helper_0<usize> for () {
  fn rgbSwapped_helper_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage17rgbSwapped_helperEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:355
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void mirrored_inplace(bool, bool)

/*

*/
impl /*struct*/ QImage {
  pub fn mirrored_inplace_0<RetType, T: QImage_mirrored_inplace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mirrored_inplace_0(self);
    // return 1;
  }
}
pub trait QImage_mirrored_inplace_0<RetType> {
  fn mirrored_inplace_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_mirrored_inplace_0<(/*void*/)> for (bool,bool) {
  fn mirrored_inplace_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const bool as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN6QImage16mirrored_inplaceEbb", 2,qtrt::FFITY_SINT8,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:356
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void rgbSwapped_inplace()

/*

*/
impl /*struct*/ QImage {
  pub fn rgbSwapped_inplace_0<RetType, T: QImage_rgbSwapped_inplace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rgbSwapped_inplace_0(self);
    // return 1;
  }
}
pub trait QImage_rgbSwapped_inplace_0<RetType> {
  fn rgbSwapped_inplace_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_rgbSwapped_inplace_0<(/*void*/)> for () {
  fn rgbSwapped_inplace_0(self , rsthis: & QImage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QImage18rgbSwapped_inplaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimage.h:357
// index:0
// Protected Visibility=Default Availability=Available
// [32] QImage convertToFormat_helper(QImage::Format, Qt::ImageConversionFlags) const

/*

*/
impl /*struct*/ QImage {
  pub fn convertToFormat_helper_0<RetType, T: QImage_convertToFormat_helper_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.convertToFormat_helper_0(self);
    // return 1;
  }
}
pub trait QImage_convertToFormat_helper_0<RetType> {
  fn convertToFormat_helper_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_convertToFormat_helper_0<usize> for (i32,i32) {
  fn convertToFormat_helper_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage22convertToFormat_helperENS_6FormatE6QFlagsIN2Qt19ImageConversionFlagEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:358
// index:0
// Protected Visibility=Default Availability=Available
// [1] bool convertToFormat_inplace(QImage::Format, Qt::ImageConversionFlags)

/*

*/
impl /*struct*/ QImage {
  pub fn convertToFormat_inplace_0<RetType, T: QImage_convertToFormat_inplace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.convertToFormat_inplace_0(self);
    // return 1;
  }
}
pub trait QImage_convertToFormat_inplace_0<RetType> {
  fn convertToFormat_inplace_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_convertToFormat_inplace_0<bool> for (i32,i32) {
  fn convertToFormat_inplace_0(self , rsthis: & QImage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QImage23convertToFormat_inplaceENS_6FormatE6QFlagsIN2Qt19ImageConversionFlagEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimage.h:359
// index:0
// Protected Visibility=Default Availability=Available
// [32] QImage smoothScaled(int, int) const

/*
Returns a smoothly scaled copy of the image. The returned image has a size of width w by height h pixels.
*/
impl /*struct*/ QImage {
  pub fn smoothScaled_0<RetType, T: QImage_smoothScaled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.smoothScaled_0(self);
    // return 1;
  }
}
pub trait QImage_smoothScaled_0<RetType> {
  fn smoothScaled_0(self , rsthis: & QImage) -> RetType;
}
impl<'a> /*trait*/ QImage_smoothScaled_0<usize> for (i32,i32) {
  fn smoothScaled_0(self , rsthis: & QImage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QImage12smoothScaledEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum type is used to describe how pixel values should be inverted in the invertPixels() function.



See also invertPixels().

*/
pub type QImage__InvertMode = i32;
// Invert only the RGB values and leave the alpha channel unchanged.
pub const QImage__InvertRgb :QImage__InvertMode = 0;
// Invert all channels, including the alpha channel.
pub const QImage__InvertRgba :QImage__InvertMode = 1;
pub fn QImage_InvertModeItemName(val: i32) ->String {
  match val {
     QImage__InvertRgb => // 0
     {return String::from("InvertRgb");}
     QImage__InvertRgba => // 1
     {return String::from("InvertRgba");}
  _ => {return format!("{}", val);}
}
}
pub fn QImage_InvertModeItemName_s(val: i32) ->String {
  //var nilthis *QImage
  //return nilthis.InvertModeItemName(val);
  return QImage_InvertModeItemName(val);
}


/*
The following image formats are available in Qt. Values from Format_ARGB8565_Premultiplied to Format_ARGB4444_Premultiplied were added in Qt 4.4. Values Format_RGBX8888, Format_RGBA8888 and Format_RGBA8888_Premultiplied were added in Qt 5.2. Values Format_BGR30, Format_A2BGR30_Premultiplied, Format_RGB30, Format_A2RGB30_Premultiplied were added in Qt 5.4. Format_Alpha8 and Format_Grayscale8 were added in Qt 5.5. See the notes after the table.





Note: Formats with more than 8 bit per color channel will only be processed by the raster engine using 8 bit per color.

See also format() and convertToFormat().

*/
pub type QImage__Format = i32;
// The image is invalid.
pub const QImage__Format_Invalid :QImage__Format = 0;
// 
pub const QImage__Format_Mono :QImage__Format = 1;
// 
pub const QImage__Format_MonoLSB :QImage__Format = 2;
// 
pub const QImage__Format_Indexed8 :QImage__Format = 3;
// 
pub const QImage__Format_RGB32 :QImage__Format = 4;
// 
pub const QImage__Format_ARGB32 :QImage__Format = 5;
// 
pub const QImage__Format_ARGB32_Premultiplied :QImage__Format = 6;
// 
pub const QImage__Format_RGB16 :QImage__Format = 7;
// 
pub const QImage__Format_ARGB8565_Premultiplied :QImage__Format = 8;
// 
pub const QImage__Format_RGB666 :QImage__Format = 9;
// 
pub const QImage__Format_ARGB6666_Premultiplied :QImage__Format = 10;
// 
pub const QImage__Format_RGB555 :QImage__Format = 11;
// 
pub const QImage__Format_ARGB8555_Premultiplied :QImage__Format = 12;
// 
pub const QImage__Format_RGB888 :QImage__Format = 13;
// 
pub const QImage__Format_RGB444 :QImage__Format = 14;
// 
pub const QImage__Format_ARGB4444_Premultiplied :QImage__Format = 15;
// 
pub const QImage__Format_RGBX8888 :QImage__Format = 16;
// 
pub const QImage__Format_RGBA8888 :QImage__Format = 17;
// 
pub const QImage__Format_RGBA8888_Premultiplied :QImage__Format = 18;
// 
pub const QImage__Format_BGR30 :QImage__Format = 19;
// 
pub const QImage__Format_A2BGR30_Premultiplied :QImage__Format = 20;
// 
pub const QImage__Format_RGB30 :QImage__Format = 21;
// 
pub const QImage__Format_A2RGB30_Premultiplied :QImage__Format = 22;
// 
pub const QImage__Format_Alpha8 :QImage__Format = 23;
// 
pub const QImage__Format_Grayscale8 :QImage__Format = 24;
// 
pub const QImage__NImageFormats :QImage__Format = 25;
pub fn QImage_FormatItemName(val: i32) ->String {
  match val {
     QImage__Format_Invalid => // 0
     {return String::from("Format_Invalid");}
     QImage__Format_Mono => // 1
     {return String::from("Format_Mono");}
     QImage__Format_MonoLSB => // 2
     {return String::from("Format_MonoLSB");}
     QImage__Format_Indexed8 => // 3
     {return String::from("Format_Indexed8");}
     QImage__Format_RGB32 => // 4
     {return String::from("Format_RGB32");}
     QImage__Format_ARGB32 => // 5
     {return String::from("Format_ARGB32");}
     QImage__Format_ARGB32_Premultiplied => // 6
     {return String::from("Format_ARGB32_Premultiplied");}
     QImage__Format_RGB16 => // 7
     {return String::from("Format_RGB16");}
     QImage__Format_ARGB8565_Premultiplied => // 8
     {return String::from("Format_ARGB8565_Premultiplied");}
     QImage__Format_RGB666 => // 9
     {return String::from("Format_RGB666");}
     QImage__Format_ARGB6666_Premultiplied => // 10
     {return String::from("Format_ARGB6666_Premultiplied");}
     QImage__Format_RGB555 => // 11
     {return String::from("Format_RGB555");}
     QImage__Format_ARGB8555_Premultiplied => // 12
     {return String::from("Format_ARGB8555_Premultiplied");}
     QImage__Format_RGB888 => // 13
     {return String::from("Format_RGB888");}
     QImage__Format_RGB444 => // 14
     {return String::from("Format_RGB444");}
     QImage__Format_ARGB4444_Premultiplied => // 15
     {return String::from("Format_ARGB4444_Premultiplied");}
     QImage__Format_RGBX8888 => // 16
     {return String::from("Format_RGBX8888");}
     QImage__Format_RGBA8888 => // 17
     {return String::from("Format_RGBA8888");}
     QImage__Format_RGBA8888_Premultiplied => // 18
     {return String::from("Format_RGBA8888_Premultiplied");}
     QImage__Format_BGR30 => // 19
     {return String::from("Format_BGR30");}
     QImage__Format_A2BGR30_Premultiplied => // 20
     {return String::from("Format_A2BGR30_Premultiplied");}
     QImage__Format_RGB30 => // 21
     {return String::from("Format_RGB30");}
     QImage__Format_A2RGB30_Premultiplied => // 22
     {return String::from("Format_A2RGB30_Premultiplied");}
     QImage__Format_Alpha8 => // 23
     {return String::from("Format_Alpha8");}
     QImage__Format_Grayscale8 => // 24
     {return String::from("Format_Grayscale8");}
     QImage__NImageFormats => // 25
     {return String::from("NImageFormats");}
  _ => {return format!("{}", val);}
}
}
pub fn QImage_FormatItemName_s(val: i32) ->String {
  //var nilthis *QImage
  //return nilthis.FormatItemName(val);
  return QImage_FormatItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
