

// mod ::gui::QPixmap
// package qtgui
// /usr/include/qt/QtGui/qpixmap.h
// #include <qpixmap.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 112
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
// func (this *QPixmap) InheritMetric(f func(arg0 int) int) {
//  qtrt.SetAllInheritCallback(this, "metric", f)
// }

// QPixmap fromImageInPlace(QImage &, Qt::ImageConversionFlags)
// func (this *QPixmap) InheritFromImageInPlace(f func(image *QImage, flags int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "fromImageInPlace", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QPixmap)=32
pub struct QPixmap {
  qbase: QPaintDevice,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPixmap_ITF interface {
//    QPaintDevice_ITF
//    QPixmap_PTR() *QPixmap
//}
//func (ptr *QPixmap) QPixmap_PTR() *QPixmap { return ptr }

impl /*struct*/ QPixmap {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPixmap {
    return QPixmap{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPixmap {
//  type Target = QPixmapBASE;
//
//  fn deref(&self) -> &QPixmapBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPixmapBASE> for QPixmap {
//  fn as_ref(& self) -> & QPixmapBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpixmap.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPixmap()

/*
Constructs a null pixmap.

See also isNull().
*/
// QPixmap() ctx.fn_proto_cpp
impl /*struct*/ QPixmap {
  pub fn QPixmap_0<T: QPixmap_QPixmap_0>(value: T) -> QPixmap {
    let rsthis = value.QPixmap_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPixmap_QPixmap_0 {
  fn QPixmap_0(self) -> QPixmap;
}
// QPixmap() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPixmap_QPixmap_0 for () {
  fn QPixmap_0(self) -> QPixmap {
    // unsafe{_ZN7QPixmapC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QPixmapC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPixmap{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:66
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPixmap(int, int)

/*
Constructs a null pixmap.

See also isNull().
*/
// QPixmap(int, int) ctx.fn_proto_cpp
impl /*struct*/ QPixmap {
  pub fn QPixmap_1<T: QPixmap_QPixmap_1>(value: T) -> QPixmap {
    let rsthis = value.QPixmap_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPixmap_QPixmap_1 {
  fn QPixmap_1(self) -> QPixmap;
}
// QPixmap(int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPixmap_QPixmap_1 for (i32,i32) {
  fn QPixmap_1(self) -> QPixmap {
    // unsafe{_ZN7QPixmapC2Eii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QPixmapC2Eii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPixmap{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:67
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QPixmap(const QSize &)

/*
Constructs a null pixmap.

See also isNull().
*/
// QPixmap(const QSize &) ctx.fn_proto_cpp
impl /*struct*/ QPixmap {
  pub fn QPixmap_2<T: QPixmap_QPixmap_2>(value: T) -> QPixmap {
    let rsthis = value.QPixmap_2();
    return rsthis;
    // return 1;
  }
}

pub trait QPixmap_QPixmap_2 {
  fn QPixmap_2(self) -> QPixmap;
}
// QPixmap(const QSize &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPixmap_QPixmap_2 for (usize) {
  fn QPixmap_2(self) -> QPixmap {
    // unsafe{_ZN7QPixmapC2ERK5QSize()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QPixmapC2ERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPixmap{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:68
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QPixmap(const QString &, const char *, Qt::ImageConversionFlags)

/*
Constructs a null pixmap.

See also isNull().
*/
// QPixmap(const QString &, const char *, Qt::ImageConversionFlags) ctx.fn_proto_cpp
impl /*struct*/ QPixmap {
  pub fn QPixmap_3<T: QPixmap_QPixmap_3>(value: T) -> QPixmap {
    let rsthis = value.QPixmap_3();
    return rsthis;
    // return 1;
  }
}

pub trait QPixmap_QPixmap_3 {
  fn QPixmap_3(self) -> QPixmap;
}
// QPixmap(const QString &, const char *, Qt::ImageConversionFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPixmap_QPixmap_3 for (usize,usize,i32) {
  fn QPixmap_3(self) -> QPixmap {
    // unsafe{_ZN7QPixmapC2ERK7QStringPKc6QFlagsIN2Qt19ImageConversionFlagEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QPixmapC2ERK7QStringPKc6QFlagsIN2Qt19ImageConversionFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPixmap{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:70
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QPixmap(const char *const *)

/*
Constructs a null pixmap.

See also isNull().
*/
// QPixmap(const char *const *) ctx.fn_proto_cpp
impl /*struct*/ QPixmap {
  pub fn QPixmap_4<T: QPixmap_QPixmap_4>(value: T) -> QPixmap {
    let rsthis = value.QPixmap_4();
    return rsthis;
    // return 1;
  }
}

pub trait QPixmap_QPixmap_4 {
  fn QPixmap_4(self) -> QPixmap;
}
// QPixmap(const char *const *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPixmap_QPixmap_4 for (usize) {
  fn QPixmap_4(self) -> QPixmap {
    // unsafe{_ZN7QPixmapC2EPKPKc()};
    let arg0 = (&self) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QPixmapC2EPKPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPixmap{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:73
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPixmap()

/*

*/
pub fn DeleteQPixmap(this :*mut QPixmap) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QPixmapD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpixmap.h:75
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap & operator=(const QPixmap &)

/*

*/
impl /*struct*/ QPixmap {
  pub fn operator_equal_0<RetType, T: QPixmap_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QPixmap_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmapaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:77
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QPixmap & operator=(QPixmap &&)

/*

*/
impl /*struct*/ QPixmap {
  pub fn operator_equal_1<RetType, T: QPixmap_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QPixmap_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmapaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:80
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QPixmap &)

/*
Swaps pixmap other with this pixmap. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QPixmap {
  pub fn swap_0<RetType, T: QPixmap_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QPixmap_swap_0<RetType> {
  fn swap_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QPixmap) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QPixmap4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:85
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if this is a null pixmap; otherwise returns false.

A null pixmap has zero width, zero height and no contents. You cannot draw in a null pixmap.
*/
impl /*struct*/ QPixmap {
  pub fn isNull_0<RetType, T: QPixmap_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QPixmap_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QPixmap) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:86
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int devType() const

/*

*/
impl /*struct*/ QPixmap {
  pub fn devType_0<RetType, T: QPixmap_devType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.devType_0(self);
    // return 1;
  }
}
pub trait QPixmap_devType_0<RetType> {
  fn devType_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_devType_0<i32> for () {
  fn devType_0(self , rsthis: & QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap7devTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:88
// index:0
// Public Visibility=Default Availability=Available
// [4] int width() const

/*
Returns the width of the pixmap.

See also size() and Pixmap Information.
*/
impl /*struct*/ QPixmap {
  pub fn width_0<RetType, T: QPixmap_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QPixmap_width_0<RetType> {
  fn width_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_width_0<i32> for () {
  fn width_0(self , rsthis: & QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:89
// index:0
// Public Visibility=Default Availability=Available
// [4] int height() const

/*
Returns the height of the pixmap.

See also size() and Pixmap Information.
*/
impl /*struct*/ QPixmap {
  pub fn height_0<RetType, T: QPixmap_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QPixmap_height_0<RetType> {
  fn height_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_height_0<i32> for () {
  fn height_0(self , rsthis: & QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:90
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize size() const

/*
Returns the size of the pixmap.

See also width(), height(), and Pixmap Information.
*/
impl /*struct*/ QPixmap {
  pub fn size_0<RetType, T: QPixmap_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QPixmap_size_0<RetType> {
  fn size_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_size_0<usize> for () {
  fn size_0(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:91
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect rect() const

/*
Returns the pixmap's enclosing rectangle.

See also Pixmap Information.
*/
impl /*struct*/ QPixmap {
  pub fn rect_0<RetType, T: QPixmap_rect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rect_0(self);
    // return 1;
  }
}
pub trait QPixmap_rect_0<RetType> {
  fn rect_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_rect_0<usize> for () {
  fn rect_0(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap4rectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:92
// index:0
// Public Visibility=Default Availability=Available
// [4] int depth() const

/*
Returns the depth of the pixmap.

The pixmap depth is also called bits per pixel (bpp) or bit planes of a pixmap. A null pixmap has depth 0.

See also defaultDepth() and Pixmap Information.
*/
impl /*struct*/ QPixmap {
  pub fn depth_0<RetType, T: QPixmap_depth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.depth_0(self);
    // return 1;
  }
}
pub trait QPixmap_depth_0<RetType> {
  fn depth_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_depth_0<i32> for () {
  fn depth_0(self , rsthis: & QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap5depthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:94
// index:0
// Public static Visibility=Default Availability=Available
// [4] int defaultDepth()

/*
Returns the default pixmap depth used by the application.

On all platforms the depth of the primary screen will be returned.

See also depth(), QColormap::depth(), and Pixmap Information.
*/
impl /*struct*/ QPixmap {
  pub fn defaultDepth_0<RetType, T: QPixmap_defaultDepth_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultDepth_0();
    // return 1;
  }
}
pub trait QPixmap_defaultDepth_0<RetType> {
  fn defaultDepth_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmap_defaultDepth_0<i32> for () {
  fn defaultDepth_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap12defaultDepthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void fill(const QColor &)

/*
Fills the pixmap with the given color.

The effect of this function is undefined when the pixmap is being painted on.

See also Pixmap Transformations.
*/
impl /*struct*/ QPixmap {
  pub fn fill_0<RetType, T: QPixmap_fill_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fill_0(self);
    // return 1;
  }
}
pub trait QPixmap_fill_0<RetType> {
  fn fill_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_fill_0<(/*void*/)> for (usize) {
  fn fill_0(self , rsthis: & QPixmap) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QPixmap4fillERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:97
// index:1
// Public Visibility=Default Availability=Available
// [-2] void fill(const QPaintDevice *, const QPoint &)

/*
Fills the pixmap with the given color.

The effect of this function is undefined when the pixmap is being painted on.

See also Pixmap Transformations.
*/
impl /*struct*/ QPixmap {
  pub fn fill_1<RetType, T: QPixmap_fill_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fill_1(self);
    // return 1;
  }
}
pub trait QPixmap_fill_1<RetType> {
  fn fill_1(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_fill_1<(/*void*/)> for (usize,usize) {
  fn fill_1(self , rsthis: & QPixmap) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QPixmap4fillEPK12QPaintDeviceRK6QPoint", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:98
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void fill(const QPaintDevice *, int, int)

/*
Fills the pixmap with the given color.

The effect of this function is undefined when the pixmap is being painted on.

See also Pixmap Transformations.
*/
impl /*struct*/ QPixmap {
  pub fn fill_2<RetType, T: QPixmap_fill_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fill_2(self);
    // return 1;
  }
}
pub trait QPixmap_fill_2<RetType> {
  fn fill_2(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_fill_2<(/*void*/)> for (usize,i32,i32) {
  fn fill_2(self , rsthis: & QPixmap) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QPixmap4fillEPK12QPaintDeviceii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:100
// index:0
// Public Visibility=Default Availability=Available
// [32] QBitmap mask() const

/*
Extracts a bitmap mask from the pixmap's alpha channel.

Warning: This is potentially an expensive operation. The mask of the pixmap is extracted dynamically from the pixeldata.

See also setMask() and Pixmap Information.
*/
impl /*struct*/ QPixmap {
  pub fn mask_0<RetType, T: QPixmap_mask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mask_0(self);
    // return 1;
  }
}
pub trait QPixmap_mask_0<RetType> {
  fn mask_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_mask_0<usize> for () {
  fn mask_0(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap4maskEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMask(const QBitmap &)

/*
Sets a mask bitmap.

This function merges the mask with the pixmap's alpha channel. A pixel value of 1 on the mask means the pixmap's pixel is unchanged; a value of 0 means the pixel is transparent. The mask must have the same size as this pixmap.

Setting a null mask resets the mask, leaving the previously transparent pixels black. The effect of this function is undefined when the pixmap is being painted on.

Warning: This is potentially an expensive operation.

See also mask(), Pixmap Transformations, and QBitmap.
*/
impl /*struct*/ QPixmap {
  pub fn setMask_0<RetType, T: QPixmap_setMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMask_0(self);
    // return 1;
  }
}
pub trait QPixmap_setMask_0<RetType> {
  fn setMask_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_setMask_0<(/*void*/)> for (usize) {
  fn setMask_0(self , rsthis: & QPixmap) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QPixmap7setMaskERK7QBitmap", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal devicePixelRatio() const

/*
Returns the device pixel ratio for the pixmap. This is the ratio between device pixels and device independent pixels.

Use this function when calculating layout geometry based on the pixmap size: QSize layoutSize = image.size() / image.devicePixelRatio()

The default value is 1.0.

See also setDevicePixelRatio() and QImageReader.
*/
impl /*struct*/ QPixmap {
  pub fn devicePixelRatio_0<RetType, T: QPixmap_devicePixelRatio_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio_0(self);
    // return 1;
  }
}
pub trait QPixmap_devicePixelRatio_0<RetType> {
  fn devicePixelRatio_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_devicePixelRatio_0<f64> for () {
  fn devicePixelRatio_0(self , rsthis: & QPixmap) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap16devicePixelRatioEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDevicePixelRatio(qreal)

/*
Sets the device pixel ratio for the pixmap. This is the ratio between image pixels and device-independent pixels.

The default scaleFactor is 1.0. Setting it to something else has two effects:

QPainters that are opened on the pixmap will be scaled. For example, painting on a 200x200 image if with a ratio of 2.0 will result in effective (device-independent) painting bounds of 100x100.

Code paths in Qt that calculate layout geometry based on the pixmap size will take the ratio into account: QSize layoutSize = pixmap.size() / pixmap.devicePixelRatio() The net effect of this is that the pixmap is displayed as high-DPI pixmap rather than a large pixmap (see Drawing High Resolution Versions of Pixmaps and Images).

See also devicePixelRatio().
*/
impl /*struct*/ QPixmap {
  pub fn setDevicePixelRatio_0<RetType, T: QPixmap_setDevicePixelRatio_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDevicePixelRatio_0(self);
    // return 1;
  }
}
pub trait QPixmap_setDevicePixelRatio_0<RetType> {
  fn setDevicePixelRatio_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_setDevicePixelRatio_0<(/*void*/)> for (f64) {
  fn setDevicePixelRatio_0(self , rsthis: & QPixmap) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN7QPixmap19setDevicePixelRatioEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:106
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasAlpha() const

/*
Returns true if this pixmap has an alpha channel, or has a mask, otherwise returns false.

See also hasAlphaChannel() and mask().
*/
impl /*struct*/ QPixmap {
  pub fn hasAlpha_0<RetType, T: QPixmap_hasAlpha_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasAlpha_0(self);
    // return 1;
  }
}
pub trait QPixmap_hasAlpha_0<RetType> {
  fn hasAlpha_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_hasAlpha_0<bool> for () {
  fn hasAlpha_0(self , rsthis: & QPixmap) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap8hasAlphaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:107
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasAlphaChannel() const

/*
Returns true if the pixmap has a format that respects the alpha channel, otherwise returns false.

See also hasAlpha().
*/
impl /*struct*/ QPixmap {
  pub fn hasAlphaChannel_0<RetType, T: QPixmap_hasAlphaChannel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasAlphaChannel_0(self);
    // return 1;
  }
}
pub trait QPixmap_hasAlphaChannel_0<RetType> {
  fn hasAlphaChannel_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_hasAlphaChannel_0<bool> for () {
  fn hasAlphaChannel_0(self , rsthis: & QPixmap) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap15hasAlphaChannelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:110
// index:0
// Public Visibility=Default Availability=Available
// [32] QBitmap createHeuristicMask(bool) const

/*
Creates and returns a heuristic mask for this pixmap.

The function works by selecting a color from one of the corners and then chipping away pixels of that color, starting at all the edges. If clipTight is true (the default) the mask is just large enough to cover the pixels; otherwise, the mask is larger than the data pixels.

The mask may not be perfect but it should be reasonable, so you can do things such as the following:


  QPixmap myPixmap;
  myPixmap.setMask(myPixmap.createHeuristicMask());



This function is slow because it involves converting to/from a QImage, and non-trivial computations.

See also QImage::createHeuristicMask() and createMaskFromColor().
*/
impl /*struct*/ QPixmap {
  pub fn createHeuristicMask_0<RetType, T: QPixmap_createHeuristicMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createHeuristicMask_0(self);
    // return 1;
  }
}
pub trait QPixmap_createHeuristicMask_0<RetType> {
  fn createHeuristicMask_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_createHeuristicMask_0<usize> for (bool) {
  fn createHeuristicMask_0(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap19createHeuristicMaskEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:112
// index:0
// Public Visibility=Default Availability=Available
// [32] QBitmap createMaskFromColor(const QColor &, Qt::MaskMode) const

/*
Creates and returns a mask for this pixmap based on the given maskColor. If the mode is Qt::MaskInColor, all pixels matching the maskColor will be transparent. If mode is Qt::MaskOutColor, all pixels matching the maskColor will be opaque.

This function is slow because it involves converting to/from a QImage.

See also createHeuristicMask() and QImage::createMaskFromColor().
*/
impl /*struct*/ QPixmap {
  pub fn createMaskFromColor_0<RetType, T: QPixmap_createMaskFromColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createMaskFromColor_0(self);
    // return 1;
  }
}
pub trait QPixmap_createMaskFromColor_0<RetType> {
  fn createMaskFromColor_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_createMaskFromColor_0<usize> for (usize,i32) {
  fn createMaskFromColor_0(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap19createMaskFromColorERK6QColorN2Qt8MaskModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:114
// index:0
// Public static Visibility=Default Availability=Available
// [32] QPixmap grabWindow(WId, int, int, int, int)

/*

*/
impl /*struct*/ QPixmap {
  pub fn grabWindow_0<RetType, T: QPixmap_grabWindow_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.grabWindow_0();
    // return 1;
  }
}
pub trait QPixmap_grabWindow_0<RetType> {
  fn grabWindow_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmap_grabWindow_0<usize> for (u64,i32,i32,i32,i32) {
  fn grabWindow_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap10grabWindowEyiiii", 5,qtrt::FFITY_UINT64,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:115
// index:0
// Public static Visibility=Default Availability=Available
// [32] QPixmap grabWidget(QObject *, const QRect &)

/*

*/
impl /*struct*/ QPixmap {
  pub fn grabWidget_0<RetType, T: QPixmap_grabWidget_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.grabWidget_0();
    // return 1;
  }
}
pub trait QPixmap_grabWidget_0<RetType> {
  fn grabWidget_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmap_grabWidget_0<usize> for (usize,usize) {
  fn grabWidget_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap10grabWidgetEP7QObjectRK5QRect", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:116
// index:1
// Public static inline Visibility=Default Availability=Available
// [32] QPixmap grabWidget(QObject *, int, int, int, int)

/*

*/
impl /*struct*/ QPixmap {
  pub fn grabWidget_1<RetType, T: QPixmap_grabWidget_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.grabWidget_1();
    // return 1;
  }
}
pub trait QPixmap_grabWidget_1<RetType> {
  fn grabWidget_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmap_grabWidget_1<usize> for (usize,i32,i32,i32,i32) {
  fn grabWidget_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap10grabWidgetEP7QObjectiiii", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:119
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QPixmap scaled(int, int, Qt::AspectRatioMode, Qt::TransformationMode) const

/*
Scales the pixmap to the given size, using the aspect ratio and transformation modes specified by aspectRatioMode and transformMode.




If aspectRatioMode is Qt::IgnoreAspectRatio, the pixmap is scaled to size.
If aspectRatioMode is Qt::KeepAspectRatio, the pixmap is scaled to a rectangle as large as possible inside size, preserving the aspect ratio.
If aspectRatioMode is Qt::KeepAspectRatioByExpanding, the pixmap is scaled to a rectangle as small as possible outside size, preserving the aspect ratio.


If the given size is empty, this function returns a null pixmap.

In some cases it can be more beneficial to draw the pixmap to a painter with a scale set rather than scaling the pixmap. This is the case when the painter is for instance based on OpenGL or when the scale factor changes rapidly.

See also isNull() and Pixmap Transformations.
*/
impl /*struct*/ QPixmap {
  pub fn scaled_0<RetType, T: QPixmap_scaled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaled_0(self);
    // return 1;
  }
}
pub trait QPixmap_scaled_0<RetType> {
  fn scaled_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_scaled_0<usize> for (i32,i32,i32,i32) {
  fn scaled_0(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap6scaledEiiN2Qt15AspectRatioModeENS0_18TransformationModeE", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:122
// index:1
// Public Visibility=Default Availability=Available
// [32] QPixmap scaled(const QSize &, Qt::AspectRatioMode, Qt::TransformationMode) const

/*
Scales the pixmap to the given size, using the aspect ratio and transformation modes specified by aspectRatioMode and transformMode.




If aspectRatioMode is Qt::IgnoreAspectRatio, the pixmap is scaled to size.
If aspectRatioMode is Qt::KeepAspectRatio, the pixmap is scaled to a rectangle as large as possible inside size, preserving the aspect ratio.
If aspectRatioMode is Qt::KeepAspectRatioByExpanding, the pixmap is scaled to a rectangle as small as possible outside size, preserving the aspect ratio.


If the given size is empty, this function returns a null pixmap.

In some cases it can be more beneficial to draw the pixmap to a painter with a scale set rather than scaling the pixmap. This is the case when the painter is for instance based on OpenGL or when the scale factor changes rapidly.

See also isNull() and Pixmap Transformations.
*/
impl /*struct*/ QPixmap {
  pub fn scaled_1<RetType, T: QPixmap_scaled_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaled_1(self);
    // return 1;
  }
}
pub trait QPixmap_scaled_1<RetType> {
  fn scaled_1(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_scaled_1<usize> for (usize,i32,i32) {
  fn scaled_1(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap6scaledERK5QSizeN2Qt15AspectRatioModeENS3_18TransformationModeE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:124
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap scaledToWidth(int, Qt::TransformationMode) const

/*
Returns a scaled copy of the image. The returned image is scaled to the given width using the specified transformation mode. The height of the pixmap is automatically calculated so that the aspect ratio of the pixmap is preserved.

If width is 0 or negative, a null pixmap is returned.

See also isNull() and Pixmap Transformations.
*/
impl /*struct*/ QPixmap {
  pub fn scaledToWidth_0<RetType, T: QPixmap_scaledToWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaledToWidth_0(self);
    // return 1;
  }
}
pub trait QPixmap_scaledToWidth_0<RetType> {
  fn scaledToWidth_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_scaledToWidth_0<usize> for (i32,i32) {
  fn scaledToWidth_0(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap13scaledToWidthEiN2Qt18TransformationModeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:125
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap scaledToHeight(int, Qt::TransformationMode) const

/*
Returns a scaled copy of the image. The returned image is scaled to the given height using the specified transformation mode. The width of the pixmap is automatically calculated so that the aspect ratio of the pixmap is preserved.

If height is 0 or negative, a null pixmap is returned.

See also isNull() and Pixmap Transformations.
*/
impl /*struct*/ QPixmap {
  pub fn scaledToHeight_0<RetType, T: QPixmap_scaledToHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaledToHeight_0(self);
    // return 1;
  }
}
pub trait QPixmap_scaledToHeight_0<RetType> {
  fn scaledToHeight_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_scaledToHeight_0<usize> for (i32,i32) {
  fn scaledToHeight_0(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap14scaledToHeightEiN2Qt18TransformationModeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:126
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap transformed(const QMatrix &, Qt::TransformationMode) const

/*
Returns a copy of the pixmap that is transformed using the given transformation transform and transformation mode. The original pixmap is not changed.

The transformation transform is internally adjusted to compensate for unwanted translation; i.e. the pixmap produced is the smallest pixmap that contains all the transformed points of the original pixmap. Use the trueMatrix() function to retrieve the actual matrix used for transforming the pixmap.

This function is slow because it involves transformation to a QImage, non-trivial computations and a transformation back to a QPixmap.

See also trueMatrix() and Pixmap Transformations.
*/
impl /*struct*/ QPixmap {
  pub fn transformed_0<RetType, T: QPixmap_transformed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transformed_0(self);
    // return 1;
  }
}
pub trait QPixmap_transformed_0<RetType> {
  fn transformed_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_transformed_0<usize> for (usize,i32) {
  fn transformed_0(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap11transformedERK7QMatrixN2Qt18TransformationModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:128
// index:1
// Public Visibility=Default Availability=Available
// [32] QPixmap transformed(const QTransform &, Qt::TransformationMode) const

/*
Returns a copy of the pixmap that is transformed using the given transformation transform and transformation mode. The original pixmap is not changed.

The transformation transform is internally adjusted to compensate for unwanted translation; i.e. the pixmap produced is the smallest pixmap that contains all the transformed points of the original pixmap. Use the trueMatrix() function to retrieve the actual matrix used for transforming the pixmap.

This function is slow because it involves transformation to a QImage, non-trivial computations and a transformation back to a QPixmap.

See also trueMatrix() and Pixmap Transformations.
*/
impl /*struct*/ QPixmap {
  pub fn transformed_1<RetType, T: QPixmap_transformed_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transformed_1(self);
    // return 1;
  }
}
pub trait QPixmap_transformed_1<RetType> {
  fn transformed_1(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_transformed_1<usize> for (usize,i32) {
  fn transformed_1(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap11transformedERK10QTransformN2Qt18TransformationModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:127
// index:0
// Public static Visibility=Default Availability=Available
// [48] QMatrix trueMatrix(const QMatrix &, int, int)

/*
Returns the actual matrix used for transforming a pixmap with the given width, height and matrix.

When transforming a pixmap using the transformed() function, the transformation matrix is internally adjusted to compensate for unwanted translation, i.e. transformed() returns the smallest pixmap containing all transformed points of the original pixmap. This function returns the modified matrix, which maps points correctly from the original pixmap into the new pixmap.

See also transformed() and Pixmap Transformations.
*/
impl /*struct*/ QPixmap {
  pub fn trueMatrix_0<RetType, T: QPixmap_trueMatrix_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.trueMatrix_0();
    // return 1;
  }
}
pub trait QPixmap_trueMatrix_0<RetType> {
  fn trueMatrix_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmap_trueMatrix_0<usize> for (usize,i32,i32) {
  fn trueMatrix_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap10trueMatrixERK7QMatrixii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:129
// index:1
// Public static Visibility=Default Availability=Available
// [88] QTransform trueMatrix(const QTransform &, int, int)

/*
Returns the actual matrix used for transforming a pixmap with the given width, height and matrix.

When transforming a pixmap using the transformed() function, the transformation matrix is internally adjusted to compensate for unwanted translation, i.e. transformed() returns the smallest pixmap containing all transformed points of the original pixmap. This function returns the modified matrix, which maps points correctly from the original pixmap into the new pixmap.

See also transformed() and Pixmap Transformations.
*/
impl /*struct*/ QPixmap {
  pub fn trueMatrix_1<RetType, T: QPixmap_trueMatrix_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.trueMatrix_1();
    // return 1;
  }
}
pub trait QPixmap_trueMatrix_1<RetType> {
  fn trueMatrix_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmap_trueMatrix_1<usize> for (usize,i32,i32) {
  fn trueMatrix_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap10trueMatrixERK10QTransformii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:131
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage toImage() const

/*
Converts the pixmap to a QImage. Returns a null image if the conversion fails.

If the pixmap has 1-bit depth, the returned image will also be 1 bit deep. Images with more bits will be returned in a format closely represents the underlying system. Usually this will be QImage::Format_ARGB32_Premultiplied for pixmaps with an alpha and QImage::Format_RGB32 or QImage::Format_RGB16 for pixmaps without alpha.

Note that for the moment, alpha masks on monochrome images are ignored.

See also fromImage() and Image Formats.
*/
impl /*struct*/ QPixmap {
  pub fn toImage_0<RetType, T: QPixmap_toImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toImage_0(self);
    // return 1;
  }
}
pub trait QPixmap_toImage_0<RetType> {
  fn toImage_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_toImage_0<usize> for () {
  fn toImage_0(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap7toImageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:132
// index:0
// Public static Visibility=Default Availability=Available
// [32] QPixmap fromImage(const QImage &, Qt::ImageConversionFlags)

/*
Converts the given image to a pixmap using the specified flags to control the conversion. The flags argument is a bitwise-OR of the Qt::ImageConversionFlags. Passing 0 for flags sets all the default options.

In case of monochrome and 8-bit images, the image is first converted to a 32-bit pixmap and then filled with the colors in the color table. If this is too expensive an operation, you can use QBitmap::fromImage() instead.

See also fromImageReader(), toImage(), and Pixmap Conversion.
*/
impl /*struct*/ QPixmap {
  pub fn fromImage_0<RetType, T: QPixmap_fromImage_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromImage_0();
    // return 1;
  }
}
pub trait QPixmap_fromImage_0<RetType> {
  fn fromImage_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmap_fromImage_0<usize> for (usize,i32) {
  fn fromImage_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap9fromImageERK6QImage6QFlagsIN2Qt19ImageConversionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:135
// index:1
// Public static inline Visibility=Default Availability=Available
// [32] QPixmap fromImage(QImage &&, Qt::ImageConversionFlags)

/*
Converts the given image to a pixmap using the specified flags to control the conversion. The flags argument is a bitwise-OR of the Qt::ImageConversionFlags. Passing 0 for flags sets all the default options.

In case of monochrome and 8-bit images, the image is first converted to a 32-bit pixmap and then filled with the colors in the color table. If this is too expensive an operation, you can use QBitmap::fromImage() instead.

See also fromImageReader(), toImage(), and Pixmap Conversion.
*/
impl /*struct*/ QPixmap {
  pub fn fromImage_1<RetType, T: QPixmap_fromImage_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromImage_1();
    // return 1;
  }
}
pub trait QPixmap_fromImage_1<RetType> {
  fn fromImage_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmap_fromImage_1<usize> for (usize,i32) {
  fn fromImage_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap9fromImageEO6QImage6QFlagsIN2Qt19ImageConversionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:133
// index:0
// Public static Visibility=Default Availability=Available
// [32] QPixmap fromImageReader(QImageReader *, Qt::ImageConversionFlags)

/*
Create a QPixmap from an image read directly from an imageReader. The flags argument is a bitwise-OR of the Qt::ImageConversionFlags. Passing 0 for flags sets all the default options.

On some systems, reading an image directly to QPixmap can use less memory than reading a QImage to convert it to QPixmap.

See also fromImage(), toImage(), and Pixmap Conversion.
*/
impl /*struct*/ QPixmap {
  pub fn fromImageReader_0<RetType, T: QPixmap_fromImageReader_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromImageReader_0();
    // return 1;
  }
}
pub trait QPixmap_fromImageReader_0<RetType> {
  fn fromImageReader_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmap_fromImageReader_0<usize> for (usize,i32) {
  fn fromImageReader_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap15fromImageReaderEP12QImageReader6QFlagsIN2Qt19ImageConversionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:141
// index:0
// Public Visibility=Default Availability=Available
// [1] bool load(const QString &, const char *, Qt::ImageConversionFlags)

/*
Loads a pixmap from the file with the given fileName. Returns true if the pixmap was successfully loaded; otherwise invalidates the pixmap and returns false.

The loader attempts to read the pixmap using the specified format. If the format is not specified (which is the default), the loader probes the file for a header to guess the file format.

The file name can either refer to an actual file on disk or to one of the application's embedded resources. See the Resource System overview for details on how to embed pixmaps and other resource files in the application's executable.

If the data needs to be modified to fit in a lower-resolution result (e.g. converting from 32-bit to 8-bit), use the flags to control the conversion.

Note that QPixmaps are automatically added to the QPixmapCache when loaded from a file; the key used is internal and can not be acquired.

See also loadFromData() and Reading and Writing Image Files.
*/
impl /*struct*/ QPixmap {
  pub fn load_0<RetType, T: QPixmap_load_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.load_0(self);
    // return 1;
  }
}
pub trait QPixmap_load_0<RetType> {
  fn load_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_load_0<bool> for (usize,usize,i32) {
  fn load_0(self , rsthis: & QPixmap) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap4loadERK7QStringPKc6QFlagsIN2Qt19ImageConversionFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:142
// index:0
// Public Visibility=Default Availability=Available
// [1] bool loadFromData(const uchar *, uint, const char *, Qt::ImageConversionFlags)

/*
Loads a pixmap from the len first bytes of the given binary data. Returns true if the pixmap was loaded successfully; otherwise invalidates the pixmap and returns false.

The loader attempts to read the pixmap using the specified format. If the format is not specified (which is the default), the loader probes the file for a header to guess the file format.

If the data needs to be modified to fit in a lower-resolution result (e.g. converting from 32-bit to 8-bit), use the flags to control the conversion.

See also load() and Reading and Writing Image Files.
*/
impl /*struct*/ QPixmap {
  pub fn loadFromData_0<RetType, T: QPixmap_loadFromData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loadFromData_0(self);
    // return 1;
  }
}
pub trait QPixmap_loadFromData_0<RetType> {
  fn loadFromData_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_loadFromData_0<bool> for (usize,u32,usize,i32) {
  fn loadFromData_0(self , rsthis: & QPixmap) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const u32 as usize;
    let arg2 = (self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap12loadFromDataEPKhjPKc6QFlagsIN2Qt19ImageConversionFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_UINT32,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:143
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool loadFromData(const QByteArray &, const char *, Qt::ImageConversionFlags)

/*
Loads a pixmap from the len first bytes of the given binary data. Returns true if the pixmap was loaded successfully; otherwise invalidates the pixmap and returns false.

The loader attempts to read the pixmap using the specified format. If the format is not specified (which is the default), the loader probes the file for a header to guess the file format.

If the data needs to be modified to fit in a lower-resolution result (e.g. converting from 32-bit to 8-bit), use the flags to control the conversion.

See also load() and Reading and Writing Image Files.
*/
impl /*struct*/ QPixmap {
  pub fn loadFromData_1<RetType, T: QPixmap_loadFromData_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loadFromData_1(self);
    // return 1;
  }
}
pub trait QPixmap_loadFromData_1<RetType> {
  fn loadFromData_1(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_loadFromData_1<bool> for (usize,usize,i32) {
  fn loadFromData_1(self , rsthis: & QPixmap) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap12loadFromDataERK10QByteArrayPKc6QFlagsIN2Qt19ImageConversionFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:144
// index:0
// Public Visibility=Default Availability=Available
// [1] bool save(const QString &, const char *, int) const

/*
Saves the pixmap to the file with the given fileName using the specified image file format and quality factor. Returns true if successful; otherwise returns false.

The quality factor must be in the range [0,100] or -1. Specify 0 to obtain small compressed files, 100 for large uncompressed files, and -1 to use the default settings.

If format is 0, an image format will be chosen from fileName's suffix.

See also Reading and Writing Image Files.
*/
impl /*struct*/ QPixmap {
  pub fn save_0<RetType, T: QPixmap_save_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.save_0(self);
    // return 1;
  }
}
pub trait QPixmap_save_0<RetType> {
  fn save_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_save_0<bool> for (usize,usize,i32) {
  fn save_0(self , rsthis: & QPixmap) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap4saveERK7QStringPKci", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:145
// index:1
// Public Visibility=Default Availability=Available
// [1] bool save(QIODevice *, const char *, int) const

/*
Saves the pixmap to the file with the given fileName using the specified image file format and quality factor. Returns true if successful; otherwise returns false.

The quality factor must be in the range [0,100] or -1. Specify 0 to obtain small compressed files, 100 for large uncompressed files, and -1 to use the default settings.

If format is 0, an image format will be chosen from fileName's suffix.

See also Reading and Writing Image Files.
*/
impl /*struct*/ QPixmap {
  pub fn save_1<RetType, T: QPixmap_save_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.save_1(self);
    // return 1;
  }
}
pub trait QPixmap_save_1<RetType> {
  fn save_1(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_save_1<bool> for (usize,usize,i32) {
  fn save_1(self , rsthis: & QPixmap) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap4saveEP9QIODevicePKci", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:147
// index:0
// Public Visibility=Default Availability=Available
// [1] bool convertFromImage(const QImage &, Qt::ImageConversionFlags)

/*
Replaces this pixmap's data with the given image using the specified flags to control the conversion. The flags argument is a bitwise-OR of the Qt::ImageConversionFlags. Passing 0 for flags sets all the default options. Returns true if the result is that this pixmap is not null.

Note: this function was part of Qt 3 support in Qt 4.6 and earlier. It has been promoted to official API status in 4.7 to support updating the pixmap's image without creating a new QPixmap as fromImage() would.

This function was introduced in  Qt 4.7.

See also fromImage().
*/
impl /*struct*/ QPixmap {
  pub fn convertFromImage_0<RetType, T: QPixmap_convertFromImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.convertFromImage_0(self);
    // return 1;
  }
}
pub trait QPixmap_convertFromImage_0<RetType> {
  fn convertFromImage_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_convertFromImage_0<bool> for (usize,i32) {
  fn convertFromImage_0(self , rsthis: & QPixmap) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap16convertFromImageERK6QImage6QFlagsIN2Qt19ImageConversionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:149
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QPixmap copy(int, int, int, int) const

/*
Returns a deep copy of the subset of the pixmap that is specified by the given rectangle. For more information on deep copies, see the Implicit Data Sharing documentation.

If the given rectangle is empty, the whole image is copied.

See also operator=(), QPixmap(), and Pixmap Transformations.
*/
impl /*struct*/ QPixmap {
  pub fn copy_0<RetType, T: QPixmap_copy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.copy_0(self);
    // return 1;
  }
}
pub trait QPixmap_copy_0<RetType> {
  fn copy_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_copy_0<usize> for (i32,i32,i32,i32) {
  fn copy_0(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap4copyEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:150
// index:1
// Public Visibility=Default Availability=Available
// [32] QPixmap copy(const QRect &) const

/*
Returns a deep copy of the subset of the pixmap that is specified by the given rectangle. For more information on deep copies, see the Implicit Data Sharing documentation.

If the given rectangle is empty, the whole image is copied.

See also operator=(), QPixmap(), and Pixmap Transformations.
*/
impl /*struct*/ QPixmap {
  pub fn copy_1<RetType, T: QPixmap_copy_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.copy_1(self);
    // return 1;
  }
}
pub trait QPixmap_copy_1<RetType> {
  fn copy_1(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_copy_1<usize> for (usize) {
  fn copy_1(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap4copyERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:152
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void scroll(int, int, int, int, int, int, QRegion *)

/*
This convenience function is equivalent to calling QPixmap::scroll(dx, dy, QRect(x, y, width, height), exposed).

This function was introduced in  Qt 4.6.

See also QWidget::scroll() and QGraphicsItem::scroll().
*/
impl /*struct*/ QPixmap {
  pub fn scroll_0<RetType, T: QPixmap_scroll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scroll_0(self);
    // return 1;
  }
}
pub trait QPixmap_scroll_0<RetType> {
  fn scroll_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_scroll_0<(/*void*/)> for (i32,i32,i32,i32,i32,i32,usize) {
  fn scroll_0(self , rsthis: & QPixmap) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QPixmap6scrollEiiiiiiP7QRegion", 7,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:153
// index:1
// Public Visibility=Default Availability=Available
// [-2] void scroll(int, int, const QRect &, QRegion *)

/*
This convenience function is equivalent to calling QPixmap::scroll(dx, dy, QRect(x, y, width, height), exposed).

This function was introduced in  Qt 4.6.

See also QWidget::scroll() and QGraphicsItem::scroll().
*/
impl /*struct*/ QPixmap {
  pub fn scroll_1<RetType, T: QPixmap_scroll_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scroll_1(self);
    // return 1;
  }
}
pub trait QPixmap_scroll_1<RetType> {
  fn scroll_1(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_scroll_1<(/*void*/)> for (i32,i32,usize,usize) {
  fn scroll_1(self , rsthis: & QPixmap) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QPixmap6scrollEiiRK5QRectP7QRegion", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:158
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 cacheKey() const

/*
Returns a number that identifies this QPixmap. Distinct QPixmap objects can only have the same cache key if they refer to the same contents.

The cacheKey() will change when the pixmap is altered.
*/
impl /*struct*/ QPixmap {
  pub fn cacheKey_0<RetType, T: QPixmap_cacheKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cacheKey_0(self);
    // return 1;
  }
}
pub trait QPixmap_cacheKey_0<RetType> {
  fn cacheKey_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_cacheKey_0<i64> for () {
  fn cacheKey_0(self , rsthis: & QPixmap) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap8cacheKeyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:160
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDetached() const

/*

*/
impl /*struct*/ QPixmap {
  pub fn isDetached_0<RetType, T: QPixmap_isDetached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDetached_0(self);
    // return 1;
  }
}
pub trait QPixmap_isDetached_0<RetType> {
  fn isDetached_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_isDetached_0<bool> for () {
  fn isDetached_0(self , rsthis: & QPixmap) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap10isDetachedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:161
// index:0
// Public Visibility=Default Availability=Available
// [-2] void detach()

/*
Detaches the pixmap from shared pixmap data.

A pixmap is automatically detached by Qt whenever its contents are about to change. This is done in almost all QPixmap member functions that modify the pixmap (fill(), fromImage(), load(), etc.), and in QPainter::begin() on a pixmap.

There are two exceptions in which detach() must be called explicitly, that is when calling the handle() or the x11PictureHandle() function (only available on X11). Otherwise, any modifications done using system calls, will be performed on the shared data.

The detach() function returns immediately if there is just a single reference or if the pixmap has not been initialized yet.
*/
impl /*struct*/ QPixmap {
  pub fn detach_0<RetType, T: QPixmap_detach_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detach_0(self);
    // return 1;
  }
}
pub trait QPixmap_detach_0<RetType> {
  fn detach_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_detach_0<(/*void*/)> for () {
  fn detach_0(self , rsthis: & QPixmap) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QPixmap6detachEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:163
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isQBitmap() const

/*
Returns true if this is a QBitmap; otherwise returns false.
*/
impl /*struct*/ QPixmap {
  pub fn isQBitmap_0<RetType, T: QPixmap_isQBitmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isQBitmap_0(self);
    // return 1;
  }
}
pub trait QPixmap_isQBitmap_0<RetType> {
  fn isQBitmap_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_isQBitmap_0<bool> for () {
  fn isQBitmap_0(self , rsthis: & QPixmap) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap9isQBitmapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:165
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QPaintEngine * paintEngine() const

/*

*/
impl /*struct*/ QPixmap {
  pub fn paintEngine_0<RetType, T: QPixmap_paintEngine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEngine_0(self);
    // return 1;
  }
}
pub trait QPixmap_paintEngine_0<RetType> {
  fn paintEngine_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_paintEngine_0<usize> for () {
  fn paintEngine_0(self , rsthis: & QPixmap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap11paintEngineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:167
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!() const

/*

*/
impl /*struct*/ QPixmap {
  pub fn operator_not_0<RetType, T: QPixmap_operator_not_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_0(self);
    // return 1;
  }
}
pub trait QPixmap_operator_not_0<RetType> {
  fn operator_not_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_operator_not_0<bool> for () {
  fn operator_not_0(self , rsthis: & QPixmap) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmapntEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:175
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int metric(QPaintDevice::PaintDeviceMetric) const

/*

*/
impl /*struct*/ QPixmap {
  pub fn metric_0<RetType, T: QPixmap_metric_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metric_0(self);
    // return 1;
  }
}
pub trait QPixmap_metric_0<RetType> {
  fn metric_0(self , rsthis: & QPixmap) -> RetType;
}
impl<'a> /*trait*/ QPixmap_metric_0<i32> for (i32) {
  fn metric_0(self , rsthis: & QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QPixmap6metricEN12QPaintDevice17PaintDeviceMetricE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmap.h:176
// index:0
// Protected static Visibility=Default Availability=Available
// [32] QPixmap fromImageInPlace(QImage &, Qt::ImageConversionFlags)

/*

*/
impl /*struct*/ QPixmap {
  pub fn fromImageInPlace_0<RetType, T: QPixmap_fromImageInPlace_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromImageInPlace_0();
    // return 1;
  }
}
pub trait QPixmap_fromImageInPlace_0<RetType> {
  fn fromImageInPlace_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmap_fromImageInPlace_0<usize> for (usize,i32) {
  fn fromImageInPlace_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QPixmap16fromImageInPlaceER6QImage6QFlagsIN2Qt19ImageConversionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
