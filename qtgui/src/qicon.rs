

// mod ::gui::QIcon
// package qtgui
// /usr/include/qt/QtGui/qicon.h
// #include <qicon.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 7
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
#[derive(Default)] // class sizeof(QIcon)=8
pub struct QIcon {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QIcon_ITF interface {
//    QIcon_PTR() *QIcon
//}
//func (ptr *QIcon) QIcon_PTR() *QIcon { return ptr }

impl /*struct*/ QIcon {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QIcon {
    return QIcon{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QIcon {
//  type Target = QIconBASE;
//
//  fn deref(&self) -> &QIconBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QIconBASE> for QIcon {
//  fn as_ref(& self) -> & QIconBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qicon.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QIcon()

/*
Constructs a null icon.
*/
// QIcon() ctx.fn_proto_cpp
impl /*struct*/ QIcon {
  pub fn QIcon_0<T: QIcon_QIcon_0>(value: T) -> QIcon {
    let rsthis = value.QIcon_0();
    return rsthis;
    // return 1;
  }
}

pub trait QIcon_QIcon_0 {
  fn QIcon_0(self) -> QIcon;
}
// QIcon() ctx.fn_proto_cpp
impl<'a> /*trait*/ QIcon_QIcon_0 for () {
  fn QIcon_0(self) -> QIcon {
    // unsafe{_ZN5QIconC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QIconC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QIcon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qicon.h:61
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QIcon(const QPixmap &)

/*
Constructs a null icon.
*/
// QIcon(const QPixmap &) ctx.fn_proto_cpp
impl /*struct*/ QIcon {
  pub fn QIcon_1<T: QIcon_QIcon_1>(value: T) -> QIcon {
    let rsthis = value.QIcon_1();
    return rsthis;
    // return 1;
  }
}

pub trait QIcon_QIcon_1 {
  fn QIcon_1(self) -> QIcon;
}
// QIcon(const QPixmap &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QIcon_QIcon_1 for (usize) {
  fn QIcon_1(self) -> QIcon {
    // unsafe{_ZN5QIconC2ERK7QPixmap()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QIconC2ERK7QPixmap", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QIcon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qicon.h:68
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QIcon(const QString &)

/*
Constructs a null icon.
*/
// QIcon(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QIcon {
  pub fn QIcon_2<T: QIcon_QIcon_2>(value: T) -> QIcon {
    let rsthis = value.QIcon_2();
    return rsthis;
    // return 1;
  }
}

pub trait QIcon_QIcon_2 {
  fn QIcon_2(self) -> QIcon;
}
// QIcon(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QIcon_QIcon_2 for (usize) {
  fn QIcon_2(self) -> QIcon {
    // unsafe{_ZN5QIconC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QIconC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QIcon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qicon.h:69
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QIcon(QIconEngine *)

/*
Constructs a null icon.
*/
// QIcon(QIconEngine *) ctx.fn_proto_cpp
impl /*struct*/ QIcon {
  pub fn QIcon_3<T: QIcon_QIcon_3>(value: T) -> QIcon {
    let rsthis = value.QIcon_3();
    return rsthis;
    // return 1;
  }
}

pub trait QIcon_QIcon_3 {
  fn QIcon_3(self) -> QIcon;
}
// QIcon(QIconEngine *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QIcon_QIcon_3 for (usize) {
  fn QIcon_3(self) -> QIcon {
    // unsafe{_ZN5QIconC2EP11QIconEngine()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QIconC2EP11QIconEngine", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QIcon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qicon.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QIcon()

/*

*/
pub fn DeleteQIcon(this :*mut QIcon) {
    // let rv = qtrt::InvokeQtFunc6("_ZN5QIconD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qicon.h:71
// index:0
// Public Visibility=Default Availability=Available
// [8] QIcon & operator=(const QIcon &)

/*

*/
impl /*struct*/ QIcon {
  pub fn operator_equal_0<RetType, T: QIcon_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QIcon_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QIconaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:73
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QIcon & operator=(QIcon &&)

/*

*/
impl /*struct*/ QIcon {
  pub fn operator_equal_1<RetType, T: QIcon_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QIcon_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QIconaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:76
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QIcon &)

/*
Swaps icon other with this icon. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QIcon {
  pub fn swap_0<RetType, T: QIcon_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QIcon_swap_0<RetType> {
  fn swap_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QIcon4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qicon.h:81
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap pixmap(const QSize &, QIcon::Mode, QIcon::State) const

/*
Returns a pixmap with the requested size, mode, and state, generating one if necessary. The pixmap might be smaller than requested, but never larger.

Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this function to return pixmaps that are larger than the requested size. Such images will have a devicePixelRatio larger than 1.

See also actualSize() and paint().
*/
impl /*struct*/ QIcon {
  pub fn pixmap_0<RetType, T: QIcon_pixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixmap_0(self);
    // return 1;
  }
}
pub trait QIcon_pixmap_0<RetType> {
  fn pixmap_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_pixmap_0<usize> for (usize,i32,i32) {
  fn pixmap_0(self , rsthis: & QIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QIcon6pixmapERK5QSizeNS_4ModeENS_5StateE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:82
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QPixmap pixmap(int, int, QIcon::Mode, QIcon::State) const

/*
Returns a pixmap with the requested size, mode, and state, generating one if necessary. The pixmap might be smaller than requested, but never larger.

Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this function to return pixmaps that are larger than the requested size. Such images will have a devicePixelRatio larger than 1.

See also actualSize() and paint().
*/
impl /*struct*/ QIcon {
  pub fn pixmap_1<RetType, T: QIcon_pixmap_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixmap_1(self);
    // return 1;
  }
}
pub trait QIcon_pixmap_1<RetType> {
  fn pixmap_1(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_pixmap_1<usize> for (i32,i32,i32,i32) {
  fn pixmap_1(self , rsthis: & QIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QIcon6pixmapEiiNS_4ModeENS_5StateE", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:84
// index:2
// Public inline Visibility=Default Availability=Available
// [32] QPixmap pixmap(int, QIcon::Mode, QIcon::State) const

/*
Returns a pixmap with the requested size, mode, and state, generating one if necessary. The pixmap might be smaller than requested, but never larger.

Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this function to return pixmaps that are larger than the requested size. Such images will have a devicePixelRatio larger than 1.

See also actualSize() and paint().
*/
impl /*struct*/ QIcon {
  pub fn pixmap_2<RetType, T: QIcon_pixmap_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixmap_2(self);
    // return 1;
  }
}
pub trait QIcon_pixmap_2<RetType> {
  fn pixmap_2(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_pixmap_2<usize> for (i32,i32,i32) {
  fn pixmap_2(self , rsthis: & QIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QIcon6pixmapEiNS_4ModeENS_5StateE", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:86
// index:3
// Public Visibility=Default Availability=Available
// [32] QPixmap pixmap(QWindow *, const QSize &, QIcon::Mode, QIcon::State) const

/*
Returns a pixmap with the requested size, mode, and state, generating one if necessary. The pixmap might be smaller than requested, but never larger.

Setting the Qt::AA_UseHighDpiPixmaps application attribute enables this function to return pixmaps that are larger than the requested size. Such images will have a devicePixelRatio larger than 1.

See also actualSize() and paint().
*/
impl /*struct*/ QIcon {
  pub fn pixmap_3<RetType, T: QIcon_pixmap_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixmap_3(self);
    // return 1;
  }
}
pub trait QIcon_pixmap_3<RetType> {
  fn pixmap_3(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_pixmap_3<usize> for (usize,usize,i32,i32) {
  fn pixmap_3(self , rsthis: & QIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QIcon6pixmapEP7QWindowRK5QSizeNS_4ModeENS_5StateE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize actualSize(const QSize &, QIcon::Mode, QIcon::State) const

/*
Returns the actual size of the icon for the requested size, mode, and state. The result might be smaller than requested, but never larger. The returned size is in device-independent pixels (This is relevant for high-dpi pixmaps.)

See also pixmap() and paint().
*/
impl /*struct*/ QIcon {
  pub fn actualSize_0<RetType, T: QIcon_actualSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actualSize_0(self);
    // return 1;
  }
}
pub trait QIcon_actualSize_0<RetType> {
  fn actualSize_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_actualSize_0<usize> for (usize,i32,i32) {
  fn actualSize_0(self , rsthis: & QIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QIcon10actualSizeERK5QSizeNS_4ModeENS_5StateE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:89
// index:1
// Public Visibility=Default Availability=Available
// [8] QSize actualSize(QWindow *, const QSize &, QIcon::Mode, QIcon::State) const

/*
Returns the actual size of the icon for the requested size, mode, and state. The result might be smaller than requested, but never larger. The returned size is in device-independent pixels (This is relevant for high-dpi pixmaps.)

See also pixmap() and paint().
*/
impl /*struct*/ QIcon {
  pub fn actualSize_1<RetType, T: QIcon_actualSize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actualSize_1(self);
    // return 1;
  }
}
pub trait QIcon_actualSize_1<RetType> {
  fn actualSize_1(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_actualSize_1<usize> for (usize,usize,i32,i32) {
  fn actualSize_1(self , rsthis: & QIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QIcon10actualSizeEP7QWindowRK5QSizeNS_4ModeENS_5StateE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:91
// index:0
// Public Visibility=Default Availability=Available
// [8] QString name() const

/*
Returns the name used to create the icon, if available.

Depending on the way the icon was created, it may have an associated name. This is the case for icons created with fromTheme() or icons using a QIconEngine which supports the QIconEngine::IconNameHook.

This function was introduced in  Qt 4.7.

See also fromTheme() and QIconEngine.
*/
impl /*struct*/ QIcon {
  pub fn name_0<RetType, T: QIcon_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QIcon_name_0<RetType> {
  fn name_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_name_0<usize> for () {
  fn name_0(self , rsthis: & QIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QIcon4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void paint(QPainter *, const QRect &, Qt::Alignment, QIcon::Mode, QIcon::State) const

/*
Uses the painter to paint the icon with specified alignment, required mode, and state into the rectangle rect.

See also actualSize() and pixmap().
*/
impl /*struct*/ QIcon {
  pub fn paint_0<RetType, T: QIcon_paint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paint_0(self);
    // return 1;
  }
}
pub trait QIcon_paint_0<RetType> {
  fn paint_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_paint_0<(/*void*/)> for (usize,usize,i32,i32,i32) {
  fn paint_0(self , rsthis: & QIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZNK5QIcon5paintEP8QPainterRK5QRect6QFlagsIN2Qt13AlignmentFlagEENS_4ModeENS_5StateE", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qicon.h:94
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void paint(QPainter *, int, int, int, int, Qt::Alignment, QIcon::Mode, QIcon::State) const

/*
Uses the painter to paint the icon with specified alignment, required mode, and state into the rectangle rect.

See also actualSize() and pixmap().
*/
impl /*struct*/ QIcon {
  pub fn paint_1<RetType, T: QIcon_paint_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paint_1(self);
    // return 1;
  }
}
pub trait QIcon_paint_1<RetType> {
  fn paint_1(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_paint_1<(/*void*/)> for (usize,i32,i32,i32,i32,i32,i32,i32) {
  fn paint_1(self , rsthis: & QIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZNK5QIcon5paintEP8QPainteriiii6QFlagsIN2Qt13AlignmentFlagEENS_4ModeENS_5StateE", 8,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qicon.h:97
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the icon is empty; otherwise returns false.

An icon is empty if it has neither a pixmap nor a filename.

Note: Even a non-null icon might not be able to create valid pixmaps, eg. if the file does not exist or cannot be read.
*/
impl /*struct*/ QIcon {
  pub fn isNull_0<RetType, T: QIcon_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QIcon_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QIcon) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QIcon6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:98
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDetached() const

/*

*/
impl /*struct*/ QIcon {
  pub fn isDetached_0<RetType, T: QIcon_isDetached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDetached_0(self);
    // return 1;
  }
}
pub trait QIcon_isDetached_0<RetType> {
  fn isDetached_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_isDetached_0<bool> for () {
  fn isDetached_0(self , rsthis: & QIcon) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QIcon10isDetachedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void detach()

/*

*/
impl /*struct*/ QIcon {
  pub fn detach_0<RetType, T: QIcon_detach_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detach_0(self);
    // return 1;
  }
}
pub trait QIcon_detach_0<RetType> {
  fn detach_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_detach_0<(/*void*/)> for () {
  fn detach_0(self , rsthis: & QIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN5QIcon6detachEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qicon.h:104
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 cacheKey() const

/*
Returns a number that identifies the contents of this QIcon object. Distinct QIcon objects can have the same key if they refer to the same contents.

The cacheKey() will change when the icon is altered via addPixmap() or addFile().

Cache keys are mostly useful in conjunction with caching.

This function was introduced in  Qt 4.3.

See also QPixmap::cacheKey().
*/
impl /*struct*/ QIcon {
  pub fn cacheKey_0<RetType, T: QIcon_cacheKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cacheKey_0(self);
    // return 1;
  }
}
pub trait QIcon_cacheKey_0<RetType> {
  fn cacheKey_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_cacheKey_0<i64> for () {
  fn cacheKey_0(self , rsthis: & QIcon) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QIcon8cacheKeyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addPixmap(const QPixmap &, QIcon::Mode, QIcon::State)

/*
Adds pixmap to the icon, as a specialization for mode and state.

Custom icon engines are free to ignore additionally added pixmaps.

See also addFile().
*/
impl /*struct*/ QIcon {
  pub fn addPixmap_0<RetType, T: QIcon_addPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addPixmap_0(self);
    // return 1;
  }
}
pub trait QIcon_addPixmap_0<RetType> {
  fn addPixmap_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_addPixmap_0<(/*void*/)> for (usize,i32,i32) {
  fn addPixmap_0(self , rsthis: & QIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QIcon9addPixmapERK7QPixmapNS_4ModeENS_5StateE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qicon.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addFile(const QString &, const QSize &, QIcon::Mode, QIcon::State)

/*
Adds an image from the file with the given fileName to the icon, as a specialization for size, mode and state. The file will be loaded on demand. Note: custom icon engines are free to ignore additionally added pixmaps.

If fileName contains a relative path (e.g. the filename only) the relevant file must be found relative to the runtime working directory.

The file name can refer to an actual file on disk or to one of the application's embedded resources. See the Resource System overview for details on how to embed images and other resource files in the application's executable.

Use the QImageReader::supportedImageFormats() and QImageWriter::supportedImageFormats() functions to retrieve a complete list of the supported file formats.

If a high resolution version of the image exists (identified by the suffix @2x on the base name), it is automatically loaded and added with the device pixel ratio set to a value of 2. This can be disabled by setting the environment variable QT_HIGHDPI_DISABLE_2X_IMAGE_LOADING (see QImageReader).

Note: When you add a non-empty filename to a QIcon, the icon becomes non-null, even if the file doesn't exist or points to a corrupt file.

See also addPixmap() and QPixmap::devicePixelRatio().
*/
impl /*struct*/ QIcon {
  pub fn addFile_0<RetType, T: QIcon_addFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addFile_0(self);
    // return 1;
  }
}
pub trait QIcon_addFile_0<RetType> {
  fn addFile_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_addFile_0<(/*void*/)> for (usize,usize,i32,i32) {
  fn addFile_0(self , rsthis: & QIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QIcon7addFileERK7QStringRK5QSizeNS_4ModeENS_5StateE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qicon.h:109
// index:0
// Public Visibility=Default Availability=Available
// [8] QList<QSize> availableSizes(QIcon::Mode, QIcon::State) const

/*
Returns a list of available icon sizes for the specified mode and state.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QIcon {
  pub fn availableSizes_0<RetType, T: QIcon_availableSizes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.availableSizes_0(self);
    // return 1;
  }
}
pub trait QIcon_availableSizes_0<RetType> {
  fn availableSizes_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_availableSizes_0<usize> for (i32,i32) {
  fn availableSizes_0(self , rsthis: & QIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QIcon14availableSizesENS_4ModeENS_5StateE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIsMask(bool)

/*
Indicate that this icon is a mask image(boolean isMask), and hence can potentially be modified based on where it's displayed.

This function was introduced in  Qt 5.6.

See also isMask().
*/
impl /*struct*/ QIcon {
  pub fn setIsMask_0<RetType, T: QIcon_setIsMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIsMask_0(self);
    // return 1;
  }
}
pub trait QIcon_setIsMask_0<RetType> {
  fn setIsMask_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_setIsMask_0<(/*void*/)> for (bool) {
  fn setIsMask_0(self , rsthis: & QIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QIcon9setIsMaskEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qicon.h:112
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isMask() const

/*
Returns true if this icon has been marked as a mask image. Certain platforms render mask icons differently (for example, menu icons on macOS).

This function was introduced in  Qt 5.6.

See also setIsMask().
*/
impl /*struct*/ QIcon {
  pub fn isMask_0<RetType, T: QIcon_isMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isMask_0(self);
    // return 1;
  }
}
pub trait QIcon_isMask_0<RetType> {
  fn isMask_0(self , rsthis: & QIcon) -> RetType;
}
impl<'a> /*trait*/ QIcon_isMask_0<bool> for () {
  fn isMask_0(self , rsthis: & QIcon) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QIcon6isMaskEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:114
// index:0
// Public static Visibility=Default Availability=Available
// [8] QIcon fromTheme(const QString &)

/*
Returns the QIcon corresponding to name in the current icon theme.

The latest version of the freedesktop icon specification and naming specification can be obtained here:


http://standards.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html
http://standards.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html


To fetch an icon from the current icon theme:


      QIcon undoicon = QIcon::fromTheme("edit-undo");



Note: By default, only X11 will support themed icons. In order to use themed icons on Mac and Windows, you will have to bundle a compliant theme in one of your themeSearchPaths() and set the appropriate themeName().

Note: Qt will make use of GTK's icon-theme.cache if present to speed up the lookup. These caches can be generated using gtk-update-icon-cache: https://developer.gnome.org/gtk3/stable/gtk-update-icon-cache.html.

This function was introduced in  Qt 4.6.

See also themeName(), setThemeName(), and themeSearchPaths().
*/
impl /*struct*/ QIcon {
  pub fn fromTheme_0<RetType, T: QIcon_fromTheme_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromTheme_0();
    // return 1;
  }
}
pub trait QIcon_fromTheme_0<RetType> {
  fn fromTheme_0(self ) -> RetType;
}
impl<'a> /*trait*/ QIcon_fromTheme_0<usize> for (usize) {
  fn fromTheme_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QIcon9fromThemeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:115
// index:1
// Public static Visibility=Default Availability=Available
// [8] QIcon fromTheme(const QString &, const QIcon &)

/*
Returns the QIcon corresponding to name in the current icon theme.

The latest version of the freedesktop icon specification and naming specification can be obtained here:


http://standards.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html
http://standards.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html


To fetch an icon from the current icon theme:


      QIcon undoicon = QIcon::fromTheme("edit-undo");



Note: By default, only X11 will support themed icons. In order to use themed icons on Mac and Windows, you will have to bundle a compliant theme in one of your themeSearchPaths() and set the appropriate themeName().

Note: Qt will make use of GTK's icon-theme.cache if present to speed up the lookup. These caches can be generated using gtk-update-icon-cache: https://developer.gnome.org/gtk3/stable/gtk-update-icon-cache.html.

This function was introduced in  Qt 4.6.

See also themeName(), setThemeName(), and themeSearchPaths().
*/
impl /*struct*/ QIcon {
  pub fn fromTheme_1<RetType, T: QIcon_fromTheme_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromTheme_1();
    // return 1;
  }
}
pub trait QIcon_fromTheme_1<RetType> {
  fn fromTheme_1(self ) -> RetType;
}
impl<'a> /*trait*/ QIcon_fromTheme_1<usize> for (usize,usize) {
  fn fromTheme_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QIcon9fromThemeERK7QStringRKS_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:116
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool hasThemeIcon(const QString &)

/*
Returns true if there is an icon available for name in the current icon theme, otherwise returns false.

This function was introduced in  Qt 4.6.

See also themeSearchPaths(), fromTheme(), and setThemeName().
*/
impl /*struct*/ QIcon {
  pub fn hasThemeIcon_0<RetType, T: QIcon_hasThemeIcon_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasThemeIcon_0();
    // return 1;
  }
}
pub trait QIcon_hasThemeIcon_0<RetType> {
  fn hasThemeIcon_0(self ) -> RetType;
}
impl<'a> /*trait*/ QIcon_hasThemeIcon_0<bool> for (usize) {
  fn hasThemeIcon_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QIcon12hasThemeIconERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:118
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList themeSearchPaths()

/*
Returns the search paths for icon themes.

The default value will depend on the platform:

On X11, the search path will use the XDG_DATA_DIRS environment variable if available.

By default all platforms will have the resource directory :\icons as a fallback. You can use "rcc -project" to generate a resource file from your icon theme.

This function was introduced in  Qt 4.6.

See also setThemeSearchPaths(), fromTheme(), and setThemeName().
*/
impl /*struct*/ QIcon {
  pub fn themeSearchPaths_0<RetType, T: QIcon_themeSearchPaths_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.themeSearchPaths_0();
    // return 1;
  }
}
pub trait QIcon_themeSearchPaths_0<RetType> {
  fn themeSearchPaths_0(self ) -> RetType;
}
impl<'a> /*trait*/ QIcon_themeSearchPaths_0<usize> for () {
  fn themeSearchPaths_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QIcon16themeSearchPathsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:119
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setThemeSearchPaths(const QStringList &)

/*
Sets the search paths for icon themes to paths.

This function was introduced in  Qt 4.6.

See also themeSearchPaths(), fromTheme(), and setThemeName().
*/
impl /*struct*/ QIcon {
  pub fn setThemeSearchPaths_0<RetType, T: QIcon_setThemeSearchPaths_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setThemeSearchPaths_0();
    // return 1;
  }
}
pub trait QIcon_setThemeSearchPaths_0<RetType> {
  fn setThemeSearchPaths_0(self ) -> RetType;
}
impl<'a> /*trait*/ QIcon_setThemeSearchPaths_0<(/*void*/)> for (usize) {
  fn setThemeSearchPaths_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QIcon19setThemeSearchPathsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qicon.h:121
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString themeName()

/*
Returns the name of the current icon theme.

On X11, the current icon theme depends on your desktop settings. On other platforms it is not set by default.

This function was introduced in  Qt 4.6.

See also setThemeName(), themeSearchPaths(), fromTheme(), and hasThemeIcon().
*/
impl /*struct*/ QIcon {
  pub fn themeName_0<RetType, T: QIcon_themeName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.themeName_0();
    // return 1;
  }
}
pub trait QIcon_themeName_0<RetType> {
  fn themeName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QIcon_themeName_0<usize> for () {
  fn themeName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QIcon9themeNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qicon.h:122
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setThemeName(const QString &)

/*
Sets the current icon theme to name.

The name should correspond to a directory name in the themeSearchPath() containing an index.theme file describing it's contents.

This function was introduced in  Qt 4.6.

See also themeSearchPaths() and themeName().
*/
impl /*struct*/ QIcon {
  pub fn setThemeName_0<RetType, T: QIcon_setThemeName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setThemeName_0();
    // return 1;
  }
}
pub trait QIcon_setThemeName_0<RetType> {
  fn setThemeName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QIcon_setThemeName_0<(/*void*/)> for (usize) {
  fn setThemeName_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QIcon12setThemeNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum type describes the mode for which a pixmap is intended to be used. The currently defined modes are:


*/
pub type QIcon__Mode = i32;
// Display the pixmap when the user is not interacting with the icon, but the functionality represented by the icon is available.
pub const QIcon__Normal :QIcon__Mode = 0;
// Display the pixmap when the functionality represented by the icon is not available.
pub const QIcon__Disabled :QIcon__Mode = 1;
// Display the pixmap when the functionality represented by the icon is available and the user is interacting with the icon, for example, moving the mouse over it or clicking it.
pub const QIcon__Active :QIcon__Mode = 2;
// Display the pixmap when the item represented by the icon is selected.
pub const QIcon__Selected :QIcon__Mode = 3;
pub fn QIcon_ModeItemName(val: i32) ->String {
  match val {
     QIcon__Normal => // 0
     {return String::from("Normal");}
     QIcon__Disabled => // 1
     {return String::from("Disabled");}
     QIcon__Active => // 2
     {return String::from("Active");}
     QIcon__Selected => // 3
     {return String::from("Selected");}
  _ => {return format!("{}", val);}
}
}
pub fn QIcon_ModeItemName_s(val: i32) ->String {
  //var nilthis *QIcon
  //return nilthis.ModeItemName(val);
  return QIcon_ModeItemName(val);
}


/*
This enum describes the state for which a pixmap is intended to be used. The state can be:


*/
pub type QIcon__State = i32;
// Display the pixmap when the widget is in an "on" state
pub const QIcon__On :QIcon__State = 0;
// Display the pixmap when the widget is in an "off" state
pub const QIcon__Off :QIcon__State = 1;
pub fn QIcon_StateItemName(val: i32) ->String {
  match val {
     QIcon__On => // 0
     {return String::from("On");}
     QIcon__Off => // 1
     {return String::from("Off");}
  _ => {return format!("{}", val);}
}
}
pub fn QIcon_StateItemName_s(val: i32) ->String {
  //var nilthis *QIcon
  //return nilthis.StateItemName(val);
  return QIcon_StateItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
