

// mod ::widgets::QStylePainter
// package qtwidgets
// /usr/include/qt/QtWidgets/qstylepainter.h
// #include <qstylepainter.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 2
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
#[derive(Default)] // class sizeof(QStylePainter)=24
pub struct QStylePainter {
  qbase: QPainter,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStylePainter_ITF interface {
//    qtgui.QPainter_ITF
//    QStylePainter_PTR() *QStylePainter
//}
//func (ptr *QStylePainter) QStylePainter_PTR() *QStylePainter { return ptr }

impl /*struct*/ QStylePainter {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStylePainter {
    return QStylePainter{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStylePainter {
//  type Target = QStylePainterBASE;
//
//  fn deref(&self) -> &QStylePainterBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStylePainterBASE> for QStylePainter {
//  fn as_ref(& self) -> & QStylePainterBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstylepainter.h:54
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QStylePainter()

/*
Constructs a QStylePainter.
*/
// QStylePainter() ctx.fn_proto_cpp
impl /*struct*/ QStylePainter {
  pub fn QStylePainter_0<T: QStylePainter_QStylePainter_0>(value: T) -> QStylePainter {
    let rsthis = value.QStylePainter_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStylePainter_QStylePainter_0 {
  fn QStylePainter_0(self) -> QStylePainter;
}
// QStylePainter() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStylePainter_QStylePainter_0 for () {
  fn QStylePainter_0(self) -> QStylePainter {
    // unsafe{_ZN13QStylePainterC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QStylePainterC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStylePainter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstylepainter.h:55
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QStylePainter(QWidget *)

/*
Constructs a QStylePainter.
*/
// QStylePainter(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QStylePainter {
  pub fn QStylePainter_1<T: QStylePainter_QStylePainter_1>(value: T) -> QStylePainter {
    let rsthis = value.QStylePainter_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStylePainter_QStylePainter_1 {
  fn QStylePainter_1(self) -> QStylePainter;
}
// QStylePainter(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStylePainter_QStylePainter_1 for (usize) {
  fn QStylePainter_1(self) -> QStylePainter {
    // unsafe{_ZN13QStylePainterC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QStylePainterC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStylePainter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstylepainter.h:56
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QStylePainter(QPaintDevice *, QWidget *)

/*
Constructs a QStylePainter.
*/
// QStylePainter(QPaintDevice *, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QStylePainter {
  pub fn QStylePainter_2<T: QStylePainter_QStylePainter_2>(value: T) -> QStylePainter {
    let rsthis = value.QStylePainter_2();
    return rsthis;
    // return 1;
  }
}

pub trait QStylePainter_QStylePainter_2 {
  fn QStylePainter_2(self) -> QStylePainter;
}
// QStylePainter(QPaintDevice *, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStylePainter_QStylePainter_2 for (usize,usize) {
  fn QStylePainter_2(self) -> QStylePainter {
    // unsafe{_ZN13QStylePainterC2EP12QPaintDeviceP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QStylePainterC2EP12QPaintDeviceP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStylePainter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstylepainter.h:57
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool begin(QWidget *)

/*
Begin painting operations on the specified widget. Returns true if the painter is ready to use; otherwise returns false.

This is automatically called by the constructor that takes a QWidget.
*/
impl /*struct*/ QStylePainter {
  pub fn begin_0<RetType, T: QStylePainter_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QStylePainter_begin_0<RetType> {
  fn begin_0(self , rsthis: & QStylePainter) -> RetType;
}
impl<'a> /*trait*/ QStylePainter_begin_0<bool> for (usize) {
  fn begin_0(self , rsthis: & QStylePainter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QStylePainter5beginEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstylepainter.h:58
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool begin(QPaintDevice *, QWidget *)

/*
Begin painting operations on the specified widget. Returns true if the painter is ready to use; otherwise returns false.

This is automatically called by the constructor that takes a QWidget.
*/
impl /*struct*/ QStylePainter {
  pub fn begin_1<RetType, T: QStylePainter_begin_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_1(self);
    // return 1;
  }
}
pub trait QStylePainter_begin_1<RetType> {
  fn begin_1(self , rsthis: & QStylePainter) -> RetType;
}
impl<'a> /*trait*/ QStylePainter_begin_1<bool> for (usize,usize) {
  fn begin_1(self , rsthis: & QStylePainter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QStylePainter5beginEP12QPaintDeviceP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstylepainter.h:64
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void drawPrimitive(QStyle::PrimitiveElement, const QStyleOption &)

/*
Use the widget's style to draw a primitive element pe specified by QStyleOption option.

See also QStyle::drawPrimitive().
*/
impl /*struct*/ QStylePainter {
  pub fn drawPrimitive_0<RetType, T: QStylePainter_drawPrimitive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPrimitive_0(self);
    // return 1;
  }
}
pub trait QStylePainter_drawPrimitive_0<RetType> {
  fn drawPrimitive_0(self , rsthis: & QStylePainter) -> RetType;
}
impl<'a> /*trait*/ QStylePainter_drawPrimitive_0<(/*void*/)> for (i32,usize) {
  fn drawPrimitive_0(self , rsthis: & QStylePainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStylePainter13drawPrimitiveEN6QStyle16PrimitiveElementERK12QStyleOption", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstylepainter.h:65
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void drawControl(QStyle::ControlElement, const QStyleOption &)

/*
Use the widget's style to draw a control element ce specified by QStyleOption option.

See also QStyle::drawControl().
*/
impl /*struct*/ QStylePainter {
  pub fn drawControl_0<RetType, T: QStylePainter_drawControl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawControl_0(self);
    // return 1;
  }
}
pub trait QStylePainter_drawControl_0<RetType> {
  fn drawControl_0(self , rsthis: & QStylePainter) -> RetType;
}
impl<'a> /*trait*/ QStylePainter_drawControl_0<(/*void*/)> for (i32,usize) {
  fn drawControl_0(self , rsthis: & QStylePainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStylePainter11drawControlEN6QStyle14ControlElementERK12QStyleOption", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstylepainter.h:66
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void drawComplexControl(QStyle::ComplexControl, const QStyleOptionComplex &)

/*
Use the widget's style to draw a complex control cc specified by the QStyleOptionComplex option.

See also QStyle::drawComplexControl().
*/
impl /*struct*/ QStylePainter {
  pub fn drawComplexControl_0<RetType, T: QStylePainter_drawComplexControl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawComplexControl_0(self);
    // return 1;
  }
}
pub trait QStylePainter_drawComplexControl_0<RetType> {
  fn drawComplexControl_0(self , rsthis: & QStylePainter) -> RetType;
}
impl<'a> /*trait*/ QStylePainter_drawComplexControl_0<(/*void*/)> for (i32,usize) {
  fn drawComplexControl_0(self , rsthis: & QStylePainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStylePainter18drawComplexControlEN6QStyle14ComplexControlERK19QStyleOptionComplex", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstylepainter.h:67
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void drawItemText(const QRect &, int, const QPalette &, bool, const QString &, QPalette::ColorRole)

/*
Draws the text in rectangle rect and palette pal. The text is aligned and wrapped according to flags.

The pen color is specified with textRole. The enabled bool indicates whether or not the item is enabled; when reimplementing this bool should influence how the item is drawn.

See also QStyle::drawItemText() and Qt::Alignment.
*/
impl /*struct*/ QStylePainter {
  pub fn drawItemText_0<RetType, T: QStylePainter_drawItemText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawItemText_0(self);
    // return 1;
  }
}
pub trait QStylePainter_drawItemText_0<RetType> {
  fn drawItemText_0(self , rsthis: & QStylePainter) -> RetType;
}
impl<'a> /*trait*/ QStylePainter_drawItemText_0<(/*void*/)> for (usize,i32,usize,bool,usize,i32) {
  fn drawItemText_0(self , rsthis: & QStylePainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const bool as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStylePainter12drawItemTextERK5QRectiRK8QPalettebRK7QStringNS3_9ColorRoleE", 6,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstylepainter.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void drawItemPixmap(const QRect &, int, const QPixmap &)

/*
Draws the pixmap in rectangle rect. The pixmap is aligned according to flags.

See also QStyle::drawItemPixmap() and Qt::Alignment.
*/
impl /*struct*/ QStylePainter {
  pub fn drawItemPixmap_0<RetType, T: QStylePainter_drawItemPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawItemPixmap_0(self);
    // return 1;
  }
}
pub trait QStylePainter_drawItemPixmap_0<RetType> {
  fn drawItemPixmap_0(self , rsthis: & QStylePainter) -> RetType;
}
impl<'a> /*trait*/ QStylePainter_drawItemPixmap_0<(/*void*/)> for (usize,i32,usize) {
  fn drawItemPixmap_0(self , rsthis: & QStylePainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStylePainter14drawItemPixmapERK5QRectiRK7QPixmap", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstylepainter.h:70
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStyle * style() const

/*
Return the current style used by the QStylePainter.
*/
impl /*struct*/ QStylePainter {
  pub fn style_0<RetType, T: QStylePainter_style_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.style_0(self);
    // return 1;
  }
}
pub trait QStylePainter_style_0<RetType> {
  fn style_0(self , rsthis: & QStylePainter) -> RetType;
}
impl<'a> /*trait*/ QStylePainter_style_0<usize> for () {
  fn style_0(self , rsthis: & QStylePainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStylePainter5styleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQStylePainter(this :*mut QStylePainter) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN13QStylePainterD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
