

// mod ::widgets::QSlider
// package qtwidgets
// /usr/include/qt/QtWidgets/qslider.h
// #include <qslider.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 41
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

// void paintEvent(QPaintEvent *)
// func (this *QSlider) InheritPaintEvent(f func(ev *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QSlider) InheritMousePressEvent(f func(ev *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QSlider) InheritMouseReleaseEvent(f func(ev *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QSlider) InheritMouseMoveEvent(f func(ev *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void initStyleOption(QStyleOptionSlider *)
// func (this *QSlider) InheritInitStyleOption(f func(option *QStyleOptionSlider/*777 QStyleOptionSlider **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSlider)=48
pub struct QSlider {
  qbase: QAbstractSlider,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSlider_ITF interface {
//    QAbstractSlider_ITF
//    QSlider_PTR() *QSlider
//}
//func (ptr *QSlider) QSlider_PTR() *QSlider { return ptr }

impl /*struct*/ QSlider {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSlider {
    return QSlider{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSlider {
//  type Target = QSliderBASE;
//
//  fn deref(&self) -> &QSliderBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSliderBASE> for QSlider {
//  fn as_ref(& self) -> & QSliderBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qslider.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSlider {
  pub fn metaObject_0<RetType, T: QSlider_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSlider_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSlider) -> RetType;
}
impl<'a> /*trait*/ QSlider_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSlider) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QSlider10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSlider(QWidget *)

/*
Constructs a vertical slider with the given parent.
*/
// QSlider(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QSlider {
  pub fn QSlider_0<T: QSlider_QSlider_0>(value: T) -> QSlider {
    let rsthis = value.QSlider_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSlider_QSlider_0 {
  fn QSlider_0(self) -> QSlider;
}
// QSlider(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSlider_QSlider_0 for (usize) {
  fn QSlider_0(self) -> QSlider {
    // unsafe{_ZN7QSliderC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QSliderC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSlider{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:72
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QSlider(Qt::Orientation, QWidget *)

/*
Constructs a vertical slider with the given parent.
*/
// QSlider(Qt::Orientation, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QSlider {
  pub fn QSlider_1<T: QSlider_QSlider_1>(value: T) -> QSlider {
    let rsthis = value.QSlider_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSlider_QSlider_1 {
  fn QSlider_1(self) -> QSlider;
}
// QSlider(Qt::Orientation, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSlider_QSlider_1 for (i32,usize) {
  fn QSlider_1(self) -> QSlider {
    // unsafe{_ZN7QSliderC2EN2Qt11OrientationEP7QWidget()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QSliderC2EN2Qt11OrientationEP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSlider{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSlider()

/*

*/
pub fn DeleteQSlider(this :*mut QSlider) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QSliderD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qslider.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QSlider {
  pub fn sizeHint_0<RetType, T: QSlider_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QSlider_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QSlider) -> RetType;
}
impl<'a> /*trait*/ QSlider_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QSlider) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QSlider8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:77
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QSlider {
  pub fn minimumSizeHint_0<RetType, T: QSlider_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QSlider_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QSlider) -> RetType;
}
impl<'a> /*trait*/ QSlider_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QSlider) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QSlider15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTickPosition(QSlider::TickPosition)

/*

*/
impl /*struct*/ QSlider {
  pub fn setTickPosition_0<RetType, T: QSlider_setTickPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTickPosition_0(self);
    // return 1;
  }
}
pub trait QSlider_setTickPosition_0<RetType> {
  fn setTickPosition_0(self , rsthis: & QSlider) -> RetType;
}
impl<'a> /*trait*/ QSlider_setTickPosition_0<(/*void*/)> for (i32) {
  fn setTickPosition_0(self , rsthis: & QSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QSlider15setTickPositionENS_12TickPositionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:80
// index:0
// Public Visibility=Default Availability=Available
// [4] QSlider::TickPosition tickPosition() const

/*

*/
impl /*struct*/ QSlider {
  pub fn tickPosition_0<RetType, T: QSlider_tickPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tickPosition_0(self);
    // return 1;
  }
}
pub trait QSlider_tickPosition_0<RetType> {
  fn tickPosition_0(self , rsthis: & QSlider) -> RetType;
}
impl<'a> /*trait*/ QSlider_tickPosition_0<i32> for () {
  fn tickPosition_0(self , rsthis: & QSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QSlider12tickPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTickInterval(int)

/*

*/
impl /*struct*/ QSlider {
  pub fn setTickInterval_0<RetType, T: QSlider_setTickInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTickInterval_0(self);
    // return 1;
  }
}
pub trait QSlider_setTickInterval_0<RetType> {
  fn setTickInterval_0(self , rsthis: & QSlider) -> RetType;
}
impl<'a> /*trait*/ QSlider_setTickInterval_0<(/*void*/)> for (i32) {
  fn setTickInterval_0(self , rsthis: & QSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QSlider15setTickIntervalEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:83
// index:0
// Public Visibility=Default Availability=Available
// [4] int tickInterval() const

/*

*/
impl /*struct*/ QSlider {
  pub fn tickInterval_0<RetType, T: QSlider_tickInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tickInterval_0(self);
    // return 1;
  }
}
pub trait QSlider_tickInterval_0<RetType> {
  fn tickInterval_0(self , rsthis: & QSlider) -> RetType;
}
impl<'a> /*trait*/ QSlider_tickInterval_0<i32> for () {
  fn tickInterval_0(self , rsthis: & QSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QSlider12tickIntervalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:85
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QSlider {
  pub fn event_0<RetType, T: QSlider_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QSlider_event_0<RetType> {
  fn event_0(self , rsthis: & QSlider) -> RetType;
}
impl<'a> /*trait*/ QSlider_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QSlider) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QSlider5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:88
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QSlider {
  pub fn paintEvent_0<RetType, T: QSlider_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QSlider_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QSlider) -> RetType;
}
impl<'a> /*trait*/ QSlider_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QSlider10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:89
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QSlider {
  pub fn mousePressEvent_0<RetType, T: QSlider_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QSlider_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QSlider) -> RetType;
}
impl<'a> /*trait*/ QSlider_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QSlider15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:90
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QSlider {
  pub fn mouseReleaseEvent_0<RetType, T: QSlider_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QSlider_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QSlider) -> RetType;
}
impl<'a> /*trait*/ QSlider_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QSlider17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:91
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QSlider {
  pub fn mouseMoveEvent_0<RetType, T: QSlider_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QSlider_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QSlider) -> RetType;
}
impl<'a> /*trait*/ QSlider_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QSlider14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qslider.h:92
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionSlider *) const

/*
Initialize option with the values from this QSlider. This method is useful for subclasses when they need a QStyleOptionSlider, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QSlider {
  pub fn initStyleOption_0<RetType, T: QSlider_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QSlider_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QSlider) -> RetType;
}
impl<'a> /*trait*/ QSlider_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QSlider) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK7QSlider15initStyleOptionEP18QStyleOptionSlider", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum specifies where the tick marks are to be drawn relative to the slider's groove and the handle the user moves.

QSlider::TicksLeftTicksAboveDraw tick marks to the left of the (vertical) slider
QSlider::TicksRightTicksBelowDraw tick marks to the right of the (vertical) slider

*/
pub type QSlider__TickPosition = i32;
// Do not draw any tick marks.
pub const QSlider__NoTicks :QSlider__TickPosition = 0;
// Draw tick marks above the (horizontal) slider
pub const QSlider__TicksAbove :QSlider__TickPosition = 1;
// 
pub const QSlider__TicksLeft :QSlider__TickPosition = 1;
// Draw tick marks below the (horizontal) slider
pub const QSlider__TicksBelow :QSlider__TickPosition = 2;
// 
pub const QSlider__TicksRight :QSlider__TickPosition = 2;
// Draw tick marks on both sides of the groove.
pub const QSlider__TicksBothSides :QSlider__TickPosition = 3;
pub fn QSlider_TickPositionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QSlider", val);
}
pub fn QSlider_TickPositionItemName_s(val: i32) ->String {
  //var nilthis *QSlider
  //return nilthis.TickPositionItemName(val);
  return QSlider_TickPositionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
