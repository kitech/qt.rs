

// mod ::widgets::QProxyStyle
// package qtwidgets
// /usr/include/qt/QtWidgets/qproxystyle.h
// #include <qproxystyle.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 34
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// bool event(QEvent *)
// func (this *QProxyStyle) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QProxyStyle)=16
pub struct QProxyStyle {
  qbase: QCommonStyle,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QProxyStyle_ITF interface {
//    QCommonStyle_ITF
//    QProxyStyle_PTR() *QProxyStyle
//}
//func (ptr *QProxyStyle) QProxyStyle_PTR() *QProxyStyle { return ptr }

impl /*struct*/ QProxyStyle {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QProxyStyle {
    return QProxyStyle{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QProxyStyle {
//  type Target = QProxyStyleBASE;
//
//  fn deref(&self) -> &QProxyStyleBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QProxyStyleBASE> for QProxyStyle {
//  fn as_ref(& self) -> & QProxyStyleBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qproxystyle.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QProxyStyle {
  pub fn metaObject_0<RetType, T: QProxyStyle_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QProxyStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QProxyStyle(QStyle *)

/*
Constructs a QProxyStyle object for overriding behavior in the specified style, or in the default native style if style is not specified.

Ownership of style is transferred to QProxyStyle.
*/
// QProxyStyle(QStyle *) ctx.fn_proto_cpp
impl /*struct*/ QProxyStyle {
  pub fn QProxyStyle_0<T: QProxyStyle_QProxyStyle_0>(value: T) -> QProxyStyle {
    let rsthis = value.QProxyStyle_0();
    return rsthis;
    // return 1;
  }
}

pub trait QProxyStyle_QProxyStyle_0 {
  fn QProxyStyle_0(self) -> QProxyStyle;
}
// QProxyStyle(QStyle *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QProxyStyle_QProxyStyle_0 for (usize) {
  fn QProxyStyle_0(self) -> QProxyStyle {
    // unsafe{_ZN11QProxyStyleC2EP6QStyle()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QProxyStyleC2EP6QStyle", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QProxyStyle{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:58
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QProxyStyle(const QString &)

/*
Constructs a QProxyStyle object for overriding behavior in the specified style, or in the default native style if style is not specified.

Ownership of style is transferred to QProxyStyle.
*/
// QProxyStyle(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QProxyStyle {
  pub fn QProxyStyle_1<T: QProxyStyle_QProxyStyle_1>(value: T) -> QProxyStyle {
    let rsthis = value.QProxyStyle_1();
    return rsthis;
    // return 1;
  }
}

pub trait QProxyStyle_QProxyStyle_1 {
  fn QProxyStyle_1(self) -> QProxyStyle;
}
// QProxyStyle(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QProxyStyle_QProxyStyle_1 for (usize) {
  fn QProxyStyle_1(self) -> QProxyStyle {
    // unsafe{_ZN11QProxyStyleC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QProxyStyleC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QProxyStyle{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QProxyStyle()

/*

*/
pub fn DeleteQProxyStyle(this :*mut QProxyStyle) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QProxyStyleD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qproxystyle.h:61
// index:0
// Public Visibility=Default Availability=Available
// [8] QStyle * baseStyle() const

/*
Returns the proxy base style object. If no base style is set on the proxy style, QProxyStyle will create an instance of the application style instead.

See also setBaseStyle() and QStyle.
*/
impl /*struct*/ QProxyStyle {
  pub fn baseStyle_0<RetType, T: QProxyStyle_baseStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.baseStyle_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_baseStyle_0<RetType> {
  fn baseStyle_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_baseStyle_0<usize> for () {
  fn baseStyle_0(self , rsthis: & QProxyStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle9baseStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBaseStyle(QStyle *)

/*
Sets the base style that should be proxied.

Ownership of style is transferred to QProxyStyle.

If style is zero, a desktop-dependant style will be assigned automatically.

See also baseStyle().
*/
impl /*struct*/ QProxyStyle {
  pub fn setBaseStyle_0<RetType, T: QProxyStyle_setBaseStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBaseStyle_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_setBaseStyle_0<RetType> {
  fn setBaseStyle_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_setBaseStyle_0<(/*void*/)> for (usize) {
  fn setBaseStyle_0(self , rsthis: & QProxyStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QProxyStyle12setBaseStyleEP6QStyle", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawPrimitive(QStyle::PrimitiveElement, const QStyleOption *, QPainter *, const QWidget *) const

/*
Reimplemented from QStyle::drawPrimitive().
*/
impl /*struct*/ QProxyStyle {
  pub fn drawPrimitive_0<RetType, T: QProxyStyle_drawPrimitive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPrimitive_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_drawPrimitive_0<RetType> {
  fn drawPrimitive_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_drawPrimitive_0<(/*void*/)> for (i32,usize,usize,usize) {
  fn drawPrimitive_0(self , rsthis: & QProxyStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QProxyStyle13drawPrimitiveEN6QStyle16PrimitiveElementEPK12QStyleOptionP8QPainterPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawControl(QStyle::ControlElement, const QStyleOption *, QPainter *, const QWidget *) const

/*
Reimplemented from QStyle::drawControl().
*/
impl /*struct*/ QProxyStyle {
  pub fn drawControl_0<RetType, T: QProxyStyle_drawControl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawControl_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_drawControl_0<RetType> {
  fn drawControl_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_drawControl_0<(/*void*/)> for (i32,usize,usize,usize) {
  fn drawControl_0(self , rsthis: & QProxyStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QProxyStyle11drawControlEN6QStyle14ControlElementEPK12QStyleOptionP8QPainterPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawComplexControl(QStyle::ComplexControl, const QStyleOptionComplex *, QPainter *, const QWidget *) const

/*
Reimplemented from QStyle::drawComplexControl().
*/
impl /*struct*/ QProxyStyle {
  pub fn drawComplexControl_0<RetType, T: QProxyStyle_drawComplexControl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawComplexControl_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_drawComplexControl_0<RetType> {
  fn drawComplexControl_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_drawComplexControl_0<(/*void*/)> for (i32,usize,usize,usize) {
  fn drawComplexControl_0(self , rsthis: & QProxyStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QProxyStyle18drawComplexControlEN6QStyle14ComplexControlEPK19QStyleOptionComplexP8QPainterPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawItemText(QPainter *, const QRect &, int, const QPalette &, bool, const QString &, QPalette::ColorRole) const

/*
Reimplemented from QStyle::drawItemText().
*/
impl /*struct*/ QProxyStyle {
  pub fn drawItemText_0<RetType, T: QProxyStyle_drawItemText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawItemText_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_drawItemText_0<RetType> {
  fn drawItemText_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_drawItemText_0<(/*void*/)> for (usize,usize,i32,usize,bool,usize,i32) {
  fn drawItemText_0(self , rsthis: & QProxyStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const bool as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZNK11QProxyStyle12drawItemTextEP8QPainterRK5QRectiRK8QPalettebRK7QStringNS5_9ColorRoleE", 7,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawItemPixmap(QPainter *, const QRect &, int, const QPixmap &) const

/*
Reimplemented from QStyle::drawItemPixmap().
*/
impl /*struct*/ QProxyStyle {
  pub fn drawItemPixmap_0<RetType, T: QProxyStyle_drawItemPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawItemPixmap_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_drawItemPixmap_0<RetType> {
  fn drawItemPixmap_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_drawItemPixmap_0<(/*void*/)> for (usize,usize,i32,usize) {
  fn drawItemPixmap_0(self , rsthis: & QProxyStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QProxyStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeFromContents(QStyle::ContentsType, const QStyleOption *, const QSize &, const QWidget *) const

/*
Reimplemented from QStyle::sizeFromContents().
*/
impl /*struct*/ QProxyStyle {
  pub fn sizeFromContents_0<RetType, T: QProxyStyle_sizeFromContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeFromContents_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_sizeFromContents_0<RetType> {
  fn sizeFromContents_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_sizeFromContents_0<usize> for (i32,usize,usize,usize) {
  fn sizeFromContents_0(self , rsthis: & QProxyStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle16sizeFromContentsEN6QStyle12ContentsTypeEPK12QStyleOptionRK5QSizePK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:73
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect subElementRect(QStyle::SubElement, const QStyleOption *, const QWidget *) const

/*
Reimplemented from QStyle::subElementRect().
*/
impl /*struct*/ QProxyStyle {
  pub fn subElementRect_0<RetType, T: QProxyStyle_subElementRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subElementRect_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_subElementRect_0<RetType> {
  fn subElementRect_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_subElementRect_0<usize> for (i32,usize,usize) {
  fn subElementRect_0(self , rsthis: & QProxyStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle14subElementRectEN6QStyle10SubElementEPK12QStyleOptionPK7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect subControlRect(QStyle::ComplexControl, const QStyleOptionComplex *, QStyle::SubControl, const QWidget *) const

/*
Reimplemented from QStyle::subControlRect().
*/
impl /*struct*/ QProxyStyle {
  pub fn subControlRect_0<RetType, T: QProxyStyle_subControlRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subControlRect_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_subControlRect_0<RetType> {
  fn subControlRect_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_subControlRect_0<usize> for (i32,usize,i32,usize) {
  fn subControlRect_0(self , rsthis: & QProxyStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle14subControlRectEN6QStyle14ComplexControlEPK19QStyleOptionComplexNS0_10SubControlEPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:75
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect itemTextRect(const QFontMetrics &, const QRect &, int, bool, const QString &) const

/*
Reimplemented from QStyle::itemTextRect().
*/
impl /*struct*/ QProxyStyle {
  pub fn itemTextRect_0<RetType, T: QProxyStyle_itemTextRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemTextRect_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_itemTextRect_0<RetType> {
  fn itemTextRect_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_itemTextRect_0<usize> for (usize,usize,i32,bool,usize) {
  fn itemTextRect_0(self , rsthis: & QProxyStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const bool as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_SINT8,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect itemPixmapRect(const QRect &, int, const QPixmap &) const

/*
Reimplemented from QStyle::itemPixmapRect().
*/
impl /*struct*/ QProxyStyle {
  pub fn itemPixmapRect_0<RetType, T: QProxyStyle_itemPixmapRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemPixmapRect_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_itemPixmapRect_0<RetType> {
  fn itemPixmapRect_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_itemPixmapRect_0<usize> for (usize,i32,usize) {
  fn itemPixmapRect_0(self , rsthis: & QProxyStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle14itemPixmapRectERK5QRectiRK7QPixmap", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:78
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QStyle::SubControl hitTestComplexControl(QStyle::ComplexControl, const QStyleOptionComplex *, const QPoint &, const QWidget *) const

/*
Reimplemented from QStyle::hitTestComplexControl().
*/
impl /*struct*/ QProxyStyle {
  pub fn hitTestComplexControl_0<RetType, T: QProxyStyle_hitTestComplexControl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hitTestComplexControl_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_hitTestComplexControl_0<RetType> {
  fn hitTestComplexControl_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_hitTestComplexControl_0<i32> for (i32,usize,usize,usize) {
  fn hitTestComplexControl_0(self , rsthis: & QProxyStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle21hitTestComplexControlEN6QStyle14ComplexControlEPK19QStyleOptionComplexRK6QPointPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int styleHint(QStyle::StyleHint, const QStyleOption *, const QWidget *, QStyleHintReturn *) const

/*
Reimplemented from QStyle::styleHint().
*/
impl /*struct*/ QProxyStyle {
  pub fn styleHint_0<RetType, T: QProxyStyle_styleHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styleHint_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_styleHint_0<RetType> {
  fn styleHint_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_styleHint_0<i32> for (i32,usize,usize,usize) {
  fn styleHint_0(self , rsthis: & QProxyStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle9styleHintEN6QStyle9StyleHintEPK12QStyleOptionPK7QWidgetP16QStyleHintReturn", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int pixelMetric(QStyle::PixelMetric, const QStyleOption *, const QWidget *) const

/*
Reimplemented from QStyle::pixelMetric().
*/
impl /*struct*/ QProxyStyle {
  pub fn pixelMetric_0<RetType, T: QProxyStyle_pixelMetric_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixelMetric_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_pixelMetric_0<RetType> {
  fn pixelMetric_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_pixelMetric_0<i32> for (i32,usize,usize) {
  fn pixelMetric_0(self , rsthis: & QProxyStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle11pixelMetricEN6QStyle11PixelMetricEPK12QStyleOptionPK7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:81
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int layoutSpacing(QSizePolicy::ControlType, QSizePolicy::ControlType, Qt::Orientation, const QStyleOption *, const QWidget *) const

/*
Reimplemented from QStyle::layoutSpacing().

This slot is called by layoutSpacing() to determine the spacing that should be used between control1 and control2 in a layout. orientation specifies whether the controls are laid out side by side or stacked vertically. The option parameter can be used to pass extra information about the parent widget. The widget parameter is optional and can also be used if option is 0.

The default implementation returns -1.

See also combinedLayoutSpacing().
*/
impl /*struct*/ QProxyStyle {
  pub fn layoutSpacing_0<RetType, T: QProxyStyle_layoutSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layoutSpacing_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_layoutSpacing_0<RetType> {
  fn layoutSpacing_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_layoutSpacing_0<i32> for (i32,i32,i32,usize,usize) {
  fn layoutSpacing_0(self , rsthis: & QProxyStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle13layoutSpacingEN11QSizePolicy11ControlTypeES1_N2Qt11OrientationEPK12QStyleOptionPK7QWidget", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:84
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QIcon standardIcon(QStyle::StandardPixmap, const QStyleOption *, const QWidget *) const

/*
Reimplemented from QStyle::standardIcon().

Returns an icon for the given standardIcon.

Reimplement this slot to provide your own icons in a QStyle subclass. The option argument can be used to pass extra information required to find the appropriate icon. The widget argument is optional and can also be used to help find the icon.
*/
impl /*struct*/ QProxyStyle {
  pub fn standardIcon_0<RetType, T: QProxyStyle_standardIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standardIcon_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_standardIcon_0<RetType> {
  fn standardIcon_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_standardIcon_0<usize> for (i32,usize,usize) {
  fn standardIcon_0(self , rsthis: & QProxyStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle12standardIconEN6QStyle14StandardPixmapEPK12QStyleOptionPK7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:85
// index:0
// Public virtual Visibility=Default Availability=Available
// [32] QPixmap standardPixmap(QStyle::StandardPixmap, const QStyleOption *, const QWidget *) const

/*
Reimplemented from QStyle::standardPixmap().
*/
impl /*struct*/ QProxyStyle {
  pub fn standardPixmap_0<RetType, T: QProxyStyle_standardPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standardPixmap_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_standardPixmap_0<RetType> {
  fn standardPixmap_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_standardPixmap_0<usize> for (i32,usize,usize) {
  fn standardPixmap_0(self , rsthis: & QProxyStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle14standardPixmapEN6QStyle14StandardPixmapEPK12QStyleOptionPK7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:86
// index:0
// Public virtual Visibility=Default Availability=Available
// [32] QPixmap generatedIconPixmap(QIcon::Mode, const QPixmap &, const QStyleOption *) const

/*
Reimplemented from QStyle::generatedIconPixmap().
*/
impl /*struct*/ QProxyStyle {
  pub fn generatedIconPixmap_0<RetType, T: QProxyStyle_generatedIconPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.generatedIconPixmap_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_generatedIconPixmap_0<RetType> {
  fn generatedIconPixmap_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_generatedIconPixmap_0<usize> for (i32,usize,usize) {
  fn generatedIconPixmap_0(self , rsthis: & QProxyStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle19generatedIconPixmapEN5QIcon4ModeERK7QPixmapPK12QStyleOption", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:87
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QPalette standardPalette() const

/*
Reimplemented from QStyle::standardPalette().
*/
impl /*struct*/ QProxyStyle {
  pub fn standardPalette_0<RetType, T: QProxyStyle_standardPalette_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standardPalette_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_standardPalette_0<RetType> {
  fn standardPalette_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_standardPalette_0<usize> for () {
  fn standardPalette_0(self , rsthis: & QProxyStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QProxyStyle15standardPaletteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:89
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void polish(QWidget *)

/*
Reimplemented from QStyle::polish().
*/
impl /*struct*/ QProxyStyle {
  pub fn polish_0<RetType, T: QProxyStyle_polish_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.polish_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_polish_0<RetType> {
  fn polish_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_polish_0<(/*void*/)> for (usize) {
  fn polish_0(self , rsthis: & QProxyStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QProxyStyle6polishEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:90
// index:1
// Public virtual Visibility=Default Availability=Available
// [-2] void polish(QPalette &)

/*
Reimplemented from QStyle::polish().
*/
impl /*struct*/ QProxyStyle {
  pub fn polish_1<RetType, T: QProxyStyle_polish_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.polish_1(self);
    // return 1;
  }
}
pub trait QProxyStyle_polish_1<RetType> {
  fn polish_1(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_polish_1<(/*void*/)> for (usize) {
  fn polish_1(self , rsthis: & QProxyStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QProxyStyle6polishER8QPalette", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:91
// index:2
// Public virtual Visibility=Default Availability=Available
// [-2] void polish(QApplication *)

/*
Reimplemented from QStyle::polish().
*/
impl /*struct*/ QProxyStyle {
  pub fn polish_2<RetType, T: QProxyStyle_polish_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.polish_2(self);
    // return 1;
  }
}
pub trait QProxyStyle_polish_2<RetType> {
  fn polish_2(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_polish_2<(/*void*/)> for (usize) {
  fn polish_2(self , rsthis: & QProxyStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QProxyStyle6polishEP12QApplication", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:93
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void unpolish(QWidget *)

/*
Reimplemented from QStyle::unpolish().
*/
impl /*struct*/ QProxyStyle {
  pub fn unpolish_0<RetType, T: QProxyStyle_unpolish_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unpolish_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_unpolish_0<RetType> {
  fn unpolish_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_unpolish_0<(/*void*/)> for (usize) {
  fn unpolish_0(self , rsthis: & QProxyStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QProxyStyle8unpolishEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:94
// index:1
// Public virtual Visibility=Default Availability=Available
// [-2] void unpolish(QApplication *)

/*
Reimplemented from QStyle::unpolish().
*/
impl /*struct*/ QProxyStyle {
  pub fn unpolish_1<RetType, T: QProxyStyle_unpolish_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unpolish_1(self);
    // return 1;
  }
}
pub trait QProxyStyle_unpolish_1<RetType> {
  fn unpolish_1(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_unpolish_1<(/*void*/)> for (usize) {
  fn unpolish_1(self , rsthis: & QProxyStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QProxyStyle8unpolishEP12QApplication", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qproxystyle.h:97
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QProxyStyle {
  pub fn event_0<RetType, T: QProxyStyle_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QProxyStyle_event_0<RetType> {
  fn event_0(self , rsthis: & QProxyStyle) -> RetType;
}
impl<'a> /*trait*/ QProxyStyle_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QProxyStyle) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QProxyStyle5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
