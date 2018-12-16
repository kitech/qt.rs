

// mod ::widgets::QCommonStyle
// package qtwidgets
// /usr/include/qt/QtWidgets/qcommonstyle.h
// #include <qcommonstyle.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 12
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



/*

*/
#[derive(Default)] // class sizeof(QCommonStyle)=16
pub struct QCommonStyle {
  qbase: QStyle,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCommonStyle_ITF interface {
//    QStyle_ITF
//    QCommonStyle_PTR() *QCommonStyle
//}
//func (ptr *QCommonStyle) QCommonStyle_PTR() *QCommonStyle { return ptr }

impl /*struct*/ QCommonStyle {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCommonStyle {
    return QCommonStyle{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCommonStyle {
//  type Target = QCommonStyleBASE;
//
//  fn deref(&self) -> &QCommonStyleBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCommonStyleBASE> for QCommonStyle {
//  fn as_ref(& self) -> & QCommonStyleBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qcommonstyle.h:52
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QCommonStyle {
  pub fn metaObject_0<RetType, T: QCommonStyle_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QCommonStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QCommonStyle10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:55
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QCommonStyle()

/*
Constructs a QCommonStyle.
*/
// QCommonStyle() ctx.fn_proto_cpp
impl /*struct*/ QCommonStyle {
  pub fn QCommonStyle_0<T: QCommonStyle_QCommonStyle_0>(value: T) -> QCommonStyle {
    let rsthis = value.QCommonStyle_0();
    return rsthis;
    // return 1;
  }
}

pub trait QCommonStyle_QCommonStyle_0 {
  fn QCommonStyle_0(self) -> QCommonStyle;
}
// QCommonStyle() ctx.fn_proto_cpp
impl<'a> /*trait*/ QCommonStyle_QCommonStyle_0 for () {
  fn QCommonStyle_0(self) -> QCommonStyle {
    // unsafe{_ZN12QCommonStyleC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QCommonStyleC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCommonStyle{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QCommonStyle()

/*

*/
pub fn DeleteQCommonStyle(this :*mut QCommonStyle) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QCommonStyleD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qcommonstyle.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawPrimitive(QStyle::PrimitiveElement, const QStyleOption *, QPainter *, const QWidget *) const

/*
Reimplemented from QStyle::drawPrimitive().
*/
impl /*struct*/ QCommonStyle {
  pub fn drawPrimitive_0<RetType, T: QCommonStyle_drawPrimitive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPrimitive_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_drawPrimitive_0<RetType> {
  fn drawPrimitive_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_drawPrimitive_0<(/*void*/)> for (i32,usize,usize,usize) {
  fn drawPrimitive_0(self , rsthis: & QCommonStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK12QCommonStyle13drawPrimitiveEN6QStyle16PrimitiveElementEPK12QStyleOptionP8QPainterPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawControl(QStyle::ControlElement, const QStyleOption *, QPainter *, const QWidget *) const

/*
Reimplemented from QStyle::drawControl().
*/
impl /*struct*/ QCommonStyle {
  pub fn drawControl_0<RetType, T: QCommonStyle_drawControl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawControl_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_drawControl_0<RetType> {
  fn drawControl_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_drawControl_0<(/*void*/)> for (i32,usize,usize,usize) {
  fn drawControl_0(self , rsthis: & QCommonStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK12QCommonStyle11drawControlEN6QStyle14ControlElementEPK12QStyleOptionP8QPainterPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect subElementRect(QStyle::SubElement, const QStyleOption *, const QWidget *) const

/*
Reimplemented from QStyle::subElementRect().
*/
impl /*struct*/ QCommonStyle {
  pub fn subElementRect_0<RetType, T: QCommonStyle_subElementRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subElementRect_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_subElementRect_0<RetType> {
  fn subElementRect_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_subElementRect_0<usize> for (i32,usize,usize) {
  fn subElementRect_0(self , rsthis: & QCommonStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QCommonStyle14subElementRectEN6QStyle10SubElementEPK12QStyleOptionPK7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawComplexControl(QStyle::ComplexControl, const QStyleOptionComplex *, QPainter *, const QWidget *) const

/*
Reimplemented from QStyle::drawComplexControl().
*/
impl /*struct*/ QCommonStyle {
  pub fn drawComplexControl_0<RetType, T: QCommonStyle_drawComplexControl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawComplexControl_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_drawComplexControl_0<RetType> {
  fn drawComplexControl_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_drawComplexControl_0<(/*void*/)> for (i32,usize,usize,usize) {
  fn drawComplexControl_0(self , rsthis: & QCommonStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK12QCommonStyle18drawComplexControlEN6QStyle14ComplexControlEPK19QStyleOptionComplexP8QPainterPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QStyle::SubControl hitTestComplexControl(QStyle::ComplexControl, const QStyleOptionComplex *, const QPoint &, const QWidget *) const

/*
Reimplemented from QStyle::hitTestComplexControl().
*/
impl /*struct*/ QCommonStyle {
  pub fn hitTestComplexControl_0<RetType, T: QCommonStyle_hitTestComplexControl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hitTestComplexControl_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_hitTestComplexControl_0<RetType> {
  fn hitTestComplexControl_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_hitTestComplexControl_0<i32> for (i32,usize,usize,usize) {
  fn hitTestComplexControl_0(self , rsthis: & QCommonStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QCommonStyle21hitTestComplexControlEN6QStyle14ComplexControlEPK19QStyleOptionComplexRK6QPointPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect subControlRect(QStyle::ComplexControl, const QStyleOptionComplex *, QStyle::SubControl, const QWidget *) const

/*
Reimplemented from QStyle::subControlRect().
*/
impl /*struct*/ QCommonStyle {
  pub fn subControlRect_0<RetType, T: QCommonStyle_subControlRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subControlRect_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_subControlRect_0<RetType> {
  fn subControlRect_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_subControlRect_0<usize> for (i32,usize,i32,usize) {
  fn subControlRect_0(self , rsthis: & QCommonStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QCommonStyle14subControlRectEN6QStyle14ComplexControlEPK19QStyleOptionComplexNS0_10SubControlEPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeFromContents(QStyle::ContentsType, const QStyleOption *, const QSize &, const QWidget *) const

/*
Reimplemented from QStyle::sizeFromContents().
*/
impl /*struct*/ QCommonStyle {
  pub fn sizeFromContents_0<RetType, T: QCommonStyle_sizeFromContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeFromContents_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_sizeFromContents_0<RetType> {
  fn sizeFromContents_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_sizeFromContents_0<usize> for (i32,usize,usize,usize) {
  fn sizeFromContents_0(self , rsthis: & QCommonStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QCommonStyle16sizeFromContentsEN6QStyle12ContentsTypeEPK12QStyleOptionRK5QSizePK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int pixelMetric(QStyle::PixelMetric, const QStyleOption *, const QWidget *) const

/*
Reimplemented from QStyle::pixelMetric().
*/
impl /*struct*/ QCommonStyle {
  pub fn pixelMetric_0<RetType, T: QCommonStyle_pixelMetric_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixelMetric_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_pixelMetric_0<RetType> {
  fn pixelMetric_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_pixelMetric_0<i32> for (i32,usize,usize) {
  fn pixelMetric_0(self , rsthis: & QCommonStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QCommonStyle11pixelMetricEN6QStyle11PixelMetricEPK12QStyleOptionPK7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int styleHint(QStyle::StyleHint, const QStyleOption *, const QWidget *, QStyleHintReturn *) const

/*
Reimplemented from QStyle::styleHint().
*/
impl /*struct*/ QCommonStyle {
  pub fn styleHint_0<RetType, T: QCommonStyle_styleHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styleHint_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_styleHint_0<RetType> {
  fn styleHint_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_styleHint_0<i32> for (i32,usize,usize,usize) {
  fn styleHint_0(self , rsthis: & QCommonStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QCommonStyle9styleHintEN6QStyle9StyleHintEPK12QStyleOptionPK7QWidgetP16QStyleHintReturn", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:77
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QIcon standardIcon(QStyle::StandardPixmap, const QStyleOption *, const QWidget *) const

/*

*/
impl /*struct*/ QCommonStyle {
  pub fn standardIcon_0<RetType, T: QCommonStyle_standardIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standardIcon_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_standardIcon_0<RetType> {
  fn standardIcon_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_standardIcon_0<usize> for (i32,usize,usize) {
  fn standardIcon_0(self , rsthis: & QCommonStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QCommonStyle12standardIconEN6QStyle14StandardPixmapEPK12QStyleOptionPK7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [32] QPixmap standardPixmap(QStyle::StandardPixmap, const QStyleOption *, const QWidget *) const

/*
Reimplemented from QStyle::standardPixmap().
*/
impl /*struct*/ QCommonStyle {
  pub fn standardPixmap_0<RetType, T: QCommonStyle_standardPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standardPixmap_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_standardPixmap_0<RetType> {
  fn standardPixmap_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_standardPixmap_0<usize> for (i32,usize,usize) {
  fn standardPixmap_0(self , rsthis: & QCommonStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QCommonStyle14standardPixmapEN6QStyle14StandardPixmapEPK12QStyleOptionPK7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:82
// index:0
// Public virtual Visibility=Default Availability=Available
// [32] QPixmap generatedIconPixmap(QIcon::Mode, const QPixmap &, const QStyleOption *) const

/*
Reimplemented from QStyle::generatedIconPixmap().
*/
impl /*struct*/ QCommonStyle {
  pub fn generatedIconPixmap_0<RetType, T: QCommonStyle_generatedIconPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.generatedIconPixmap_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_generatedIconPixmap_0<RetType> {
  fn generatedIconPixmap_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_generatedIconPixmap_0<usize> for (i32,usize,usize) {
  fn generatedIconPixmap_0(self , rsthis: & QCommonStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QCommonStyle19generatedIconPixmapEN5QIcon4ModeERK7QPixmapPK12QStyleOption", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:84
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int layoutSpacing(QSizePolicy::ControlType, QSizePolicy::ControlType, Qt::Orientation, const QStyleOption *, const QWidget *) const

/*
Reimplemented from QStyle::layoutSpacing().
*/
impl /*struct*/ QCommonStyle {
  pub fn layoutSpacing_0<RetType, T: QCommonStyle_layoutSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layoutSpacing_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_layoutSpacing_0<RetType> {
  fn layoutSpacing_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_layoutSpacing_0<i32> for (i32,i32,i32,usize,usize) {
  fn layoutSpacing_0(self , rsthis: & QCommonStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QCommonStyle13layoutSpacingEN11QSizePolicy11ControlTypeES1_N2Qt11OrientationEPK12QStyleOptionPK7QWidget", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void polish(QPalette &)

/*
Reimplemented from QStyle::polish().
*/
impl /*struct*/ QCommonStyle {
  pub fn polish_0<RetType, T: QCommonStyle_polish_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.polish_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_polish_0<RetType> {
  fn polish_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_polish_0<(/*void*/)> for (usize) {
  fn polish_0(self , rsthis: & QCommonStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QCommonStyle6polishER8QPalette", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:89
// index:1
// Public virtual Visibility=Default Availability=Available
// [-2] void polish(QApplication *)

/*
Reimplemented from QStyle::polish().
*/
impl /*struct*/ QCommonStyle {
  pub fn polish_1<RetType, T: QCommonStyle_polish_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.polish_1(self);
    // return 1;
  }
}
pub trait QCommonStyle_polish_1<RetType> {
  fn polish_1(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_polish_1<(/*void*/)> for (usize) {
  fn polish_1(self , rsthis: & QCommonStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QCommonStyle6polishEP12QApplication", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:90
// index:2
// Public virtual Visibility=Default Availability=Available
// [-2] void polish(QWidget *)

/*
Reimplemented from QStyle::polish().
*/
impl /*struct*/ QCommonStyle {
  pub fn polish_2<RetType, T: QCommonStyle_polish_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.polish_2(self);
    // return 1;
  }
}
pub trait QCommonStyle_polish_2<RetType> {
  fn polish_2(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_polish_2<(/*void*/)> for (usize) {
  fn polish_2(self , rsthis: & QCommonStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QCommonStyle6polishEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:91
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void unpolish(QWidget *)

/*
Reimplemented from QStyle::unpolish().
*/
impl /*struct*/ QCommonStyle {
  pub fn unpolish_0<RetType, T: QCommonStyle_unpolish_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unpolish_0(self);
    // return 1;
  }
}
pub trait QCommonStyle_unpolish_0<RetType> {
  fn unpolish_0(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_unpolish_0<(/*void*/)> for (usize) {
  fn unpolish_0(self , rsthis: & QCommonStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QCommonStyle8unpolishEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcommonstyle.h:92
// index:1
// Public virtual Visibility=Default Availability=Available
// [-2] void unpolish(QApplication *)

/*
Reimplemented from QStyle::unpolish().
*/
impl /*struct*/ QCommonStyle {
  pub fn unpolish_1<RetType, T: QCommonStyle_unpolish_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unpolish_1(self);
    // return 1;
  }
}
pub trait QCommonStyle_unpolish_1<RetType> {
  fn unpolish_1(self , rsthis: & QCommonStyle) -> RetType;
}
impl<'a> /*trait*/ QCommonStyle_unpolish_1<(/*void*/)> for (usize) {
  fn unpolish_1(self , rsthis: & QCommonStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QCommonStyle8unpolishEP12QApplication", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
