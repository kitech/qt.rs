

// mod ::widgets::QRubberBand
// package qtwidgets
// /usr/include/qt/QtWidgets/qrubberband.h
// #include <qrubberband.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 65
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
// func (this *QRubberBand) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QRubberBand) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QRubberBand) InheritChangeEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QRubberBand) InheritShowEvent(f func(arg0 *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QRubberBand) InheritResizeEvent(f func(arg0 *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void moveEvent(QMoveEvent *)
// func (this *QRubberBand) InheritMoveEvent(f func(arg0 *qtgui.QMoveEvent/*777 QMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "moveEvent", f)
// }

// void initStyleOption(QStyleOptionRubberBand *)
// func (this *QRubberBand) InheritInitStyleOption(f func(option *QStyleOptionRubberBand/*777 QStyleOptionRubberBand **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QRubberBand)=48
pub struct QRubberBand {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRubberBand_ITF interface {
//    QWidget_ITF
//    QRubberBand_PTR() *QRubberBand
//}
//func (ptr *QRubberBand) QRubberBand_PTR() *QRubberBand { return ptr }

impl /*struct*/ QRubberBand {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRubberBand {
    return QRubberBand{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRubberBand {
//  type Target = QRubberBandBASE;
//
//  fn deref(&self) -> &QRubberBandBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRubberBandBASE> for QRubberBand {
//  fn as_ref(& self) -> & QRubberBandBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qrubberband.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QRubberBand {
  pub fn metaObject_0<RetType, T: QRubberBand_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QRubberBand_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QRubberBand) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QRubberBand10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QRubberBand(QRubberBand::Shape, QWidget *)

/*
Constructs a rubber band of shape s, with parent p.

By default a rectangular rubber band (s is Rectangle) will use a mask, so that a small border of the rectangle is all that is visible. Some styles (e.g., native macOS) will change this and call QWidget::setWindowOpacity() to make a semi-transparent filled selection rectangle.
*/
// QRubberBand(QRubberBand::Shape, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QRubberBand {
  pub fn QRubberBand_0<T: QRubberBand_QRubberBand_0>(value: T) -> QRubberBand {
    let rsthis = value.QRubberBand_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRubberBand_QRubberBand_0 {
  fn QRubberBand_0(self) -> QRubberBand;
}
// QRubberBand(QRubberBand::Shape, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRubberBand_QRubberBand_0 for (i32,usize) {
  fn QRubberBand_0(self) -> QRubberBand {
    // unsafe{_ZN11QRubberBandC2ENS_5ShapeEP7QWidget()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QRubberBandC2ENS_5ShapeEP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRubberBand{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QRubberBand()

/*

*/
pub fn DeleteQRubberBand(this :*mut QRubberBand) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QRubberBandD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qrubberband.h:62
// index:0
// Public Visibility=Default Availability=Available
// [4] QRubberBand::Shape shape() const

/*
Returns the shape of this rubber band. The shape can only be set upon construction.
*/
impl /*struct*/ QRubberBand {
  pub fn shape_0<RetType, T: QRubberBand_shape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shape_0(self);
    // return 1;
  }
}
pub trait QRubberBand_shape_0<RetType> {
  fn shape_0(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_shape_0<i32> for () {
  fn shape_0(self , rsthis: & QRubberBand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QRubberBand5shapeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGeometry(const QRect &)

/*
Sets the geometry of the rubber band to rect, specified in the coordinate system of its parent widget.

See also QWidget::geometry.
*/
impl /*struct*/ QRubberBand {
  pub fn setGeometry_0<RetType, T: QRubberBand_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QRubberBand_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QRubberBand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QRubberBand11setGeometryERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:66
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setGeometry(int, int, int, int)

/*
Sets the geometry of the rubber band to rect, specified in the coordinate system of its parent widget.

See also QWidget::geometry.
*/
impl /*struct*/ QRubberBand {
  pub fn setGeometry_1<RetType, T: QRubberBand_setGeometry_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_1(self);
    // return 1;
  }
}
pub trait QRubberBand_setGeometry_1<RetType> {
  fn setGeometry_1(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_setGeometry_1<(/*void*/)> for (i32,i32,i32,i32) {
  fn setGeometry_1(self , rsthis: & QRubberBand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QRubberBand11setGeometryEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:67
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void move(int, int)

/*
Moves the rubberband to point (x, y).

See also resize().
*/
impl /*struct*/ QRubberBand {
  pub fn move__0<RetType, T: QRubberBand_move__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.move__0(self);
    // return 1;
  }
}
pub trait QRubberBand_move__0<RetType> {
  fn move__0(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_move__0<(/*void*/)> for (i32,i32) {
  fn move__0(self , rsthis: & QRubberBand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QRubberBand4moveEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:68
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void move(const QPoint &)

/*
Moves the rubberband to point (x, y).

See also resize().
*/
impl /*struct*/ QRubberBand {
  pub fn move__1<RetType, T: QRubberBand_move__1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.move__1(self);
    // return 1;
  }
}
pub trait QRubberBand_move__1<RetType> {
  fn move__1(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_move__1<(/*void*/)> for (usize) {
  fn move__1(self , rsthis: & QRubberBand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QRubberBand4moveERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:70
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void resize(int, int)

/*
Resizes the rubberband so that its width is width, and its height is height.

See also move().
*/
impl /*struct*/ QRubberBand {
  pub fn resize_0<RetType, T: QRubberBand_resize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_0(self);
    // return 1;
  }
}
pub trait QRubberBand_resize_0<RetType> {
  fn resize_0(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_resize_0<(/*void*/)> for (i32,i32) {
  fn resize_0(self , rsthis: & QRubberBand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QRubberBand6resizeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:72
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void resize(const QSize &)

/*
Resizes the rubberband so that its width is width, and its height is height.

See also move().
*/
impl /*struct*/ QRubberBand {
  pub fn resize_1<RetType, T: QRubberBand_resize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_1(self);
    // return 1;
  }
}
pub trait QRubberBand_resize_1<RetType> {
  fn resize_1(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_resize_1<(/*void*/)> for (usize) {
  fn resize_1(self , rsthis: & QRubberBand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QRubberBand6resizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:76
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QRubberBand {
  pub fn event_0<RetType, T: QRubberBand_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QRubberBand_event_0<RetType> {
  fn event_0(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QRubberBand) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QRubberBand5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:77
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QRubberBand {
  pub fn paintEvent_0<RetType, T: QRubberBand_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QRubberBand_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QRubberBand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QRubberBand10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:78
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QRubberBand {
  pub fn changeEvent_0<RetType, T: QRubberBand_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QRubberBand_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QRubberBand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QRubberBand11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:79
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QRubberBand {
  pub fn showEvent_0<RetType, T: QRubberBand_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QRubberBand_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QRubberBand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QRubberBand9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:80
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QRubberBand {
  pub fn resizeEvent_0<RetType, T: QRubberBand_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QRubberBand_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QRubberBand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QRubberBand11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:81
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void moveEvent(QMoveEvent *)

/*
Reimplemented from QWidget::moveEvent().
*/
impl /*struct*/ QRubberBand {
  pub fn moveEvent_0<RetType, T: QRubberBand_moveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveEvent_0(self);
    // return 1;
  }
}
pub trait QRubberBand_moveEvent_0<RetType> {
  fn moveEvent_0(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_moveEvent_0<(/*void*/)> for (usize) {
  fn moveEvent_0(self , rsthis: & QRubberBand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QRubberBand9moveEventEP10QMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qrubberband.h:82
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionRubberBand *) const

/*
Initialize option with the values from this QRubberBand. This method is useful for subclasses when they need a QStyleOptionRubberBand, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QRubberBand {
  pub fn initStyleOption_0<RetType, T: QRubberBand_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QRubberBand_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QRubberBand) -> RetType;
}
impl<'a> /*trait*/ QRubberBand_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QRubberBand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QRubberBand15initStyleOptionEP22QStyleOptionRubberBand", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum specifies what shape a QRubberBand should have. This is a drawing hint that is passed down to the style system, and can be interpreted by each QStyle.


*/
pub type QRubberBand__Shape = i32;
// A QRubberBand can represent a vertical or horizontal line. Geometry is still given in rect() and the line will fill the given geometry on most styles.
pub const QRubberBand__Line :QRubberBand__Shape = 0;
// A QRubberBand can represent a rectangle. Some styles will interpret this as a filled (often semi-transparent) rectangle, or a rectangular outline.
pub const QRubberBand__Rectangle :QRubberBand__Shape = 1;
pub fn QRubberBand_ShapeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QRubberBand", val);
}
pub fn QRubberBand_ShapeItemName_s(val: i32) ->String {
  //var nilthis *QRubberBand
  //return nilthis.ShapeItemName(val);
  return QRubberBand_ShapeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
