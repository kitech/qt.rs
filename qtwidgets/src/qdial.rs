

// mod ::widgets::QDial
// package qtwidgets
// /usr/include/qt/QtWidgets/qdial.h
// #include <qdial.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 21
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
// func (this *QDial) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QDial) InheritResizeEvent(f func(re *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QDial) InheritPaintEvent(f func(pe *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QDial) InheritMousePressEvent(f func(me *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QDial) InheritMouseReleaseEvent(f func(me *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QDial) InheritMouseMoveEvent(f func(me *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void sliderChange(QAbstractSlider::SliderChange)
// func (this *QDial) InheritSliderChange(f func(change int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "sliderChange", f)
// }

// void initStyleOption(QStyleOptionSlider *)
// func (this *QDial) InheritInitStyleOption(f func(option *QStyleOptionSlider/*777 QStyleOptionSlider **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QDial)=48
pub struct QDial {
  qbase: QAbstractSlider,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDial_ITF interface {
//    QAbstractSlider_ITF
//    QDial_PTR() *QDial
//}
//func (ptr *QDial) QDial_PTR() *QDial { return ptr }

impl /*struct*/ QDial {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDial {
    return QDial{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDial {
//  type Target = QDialBASE;
//
//  fn deref(&self) -> &QDialBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDialBASE> for QDial {
//  fn as_ref(& self) -> & QDialBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qdial.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QDial {
  pub fn metaObject_0<RetType, T: QDial_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QDial_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QDial) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDial10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDial(QWidget *)

/*
Constructs a dial.

The parent argument is sent to the QAbstractSlider constructor.
*/
// QDial(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QDial {
  pub fn QDial_0<T: QDial_QDial_0>(value: T) -> QDial {
    let rsthis = value.QDial_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDial_QDial_0 {
  fn QDial_0(self) -> QDial;
}
// QDial(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDial_QDial_0 for (usize) {
  fn QDial_0(self) -> QDial {
    // unsafe{_ZN5QDialC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QDialC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDial{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDial()

/*

*/
pub fn DeleteQDial(this :*mut QDial) {
    // let rv = qtrt::InvokeQtFunc6("_ZN5QDialD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qdial.h:68
// index:0
// Public Visibility=Default Availability=Available
// [1] bool wrapping() const

/*

*/
impl /*struct*/ QDial {
  pub fn wrapping_0<RetType, T: QDial_wrapping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wrapping_0(self);
    // return 1;
  }
}
pub trait QDial_wrapping_0<RetType> {
  fn wrapping_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_wrapping_0<bool> for () {
  fn wrapping_0(self , rsthis: & QDial) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDial8wrappingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:70
// index:0
// Public Visibility=Default Availability=Available
// [4] int notchSize() const

/*

*/
impl /*struct*/ QDial {
  pub fn notchSize_0<RetType, T: QDial_notchSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notchSize_0(self);
    // return 1;
  }
}
pub trait QDial_notchSize_0<RetType> {
  fn notchSize_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_notchSize_0<i32> for () {
  fn notchSize_0(self , rsthis: & QDial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDial9notchSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNotchTarget(double)

/*

*/
impl /*struct*/ QDial {
  pub fn setNotchTarget_0<RetType, T: QDial_setNotchTarget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNotchTarget_0(self);
    // return 1;
  }
}
pub trait QDial_setNotchTarget_0<RetType> {
  fn setNotchTarget_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_setNotchTarget_0<(/*void*/)> for (f64) {
  fn setNotchTarget_0(self , rsthis: & QDial) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN5QDial14setNotchTargetEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:73
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal notchTarget() const

/*

*/
impl /*struct*/ QDial {
  pub fn notchTarget_0<RetType, T: QDial_notchTarget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notchTarget_0(self);
    // return 1;
  }
}
pub trait QDial_notchTarget_0<RetType> {
  fn notchTarget_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_notchTarget_0<f64> for () {
  fn notchTarget_0(self , rsthis: & QDial) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDial11notchTargetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:74
// index:0
// Public Visibility=Default Availability=Available
// [1] bool notchesVisible() const

/*

*/
impl /*struct*/ QDial {
  pub fn notchesVisible_0<RetType, T: QDial_notchesVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notchesVisible_0(self);
    // return 1;
  }
}
pub trait QDial_notchesVisible_0<RetType> {
  fn notchesVisible_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_notchesVisible_0<bool> for () {
  fn notchesVisible_0(self , rsthis: & QDial) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDial14notchesVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QDial {
  pub fn sizeHint_0<RetType, T: QDial_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QDial_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QDial) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDial8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:77
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QDial {
  pub fn minimumSizeHint_0<RetType, T: QDial_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QDial_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QDial) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDial15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNotchesVisible(bool)

/*

*/
impl /*struct*/ QDial {
  pub fn setNotchesVisible_0<RetType, T: QDial_setNotchesVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNotchesVisible_0(self);
    // return 1;
  }
}
pub trait QDial_setNotchesVisible_0<RetType> {
  fn setNotchesVisible_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_setNotchesVisible_0<(/*void*/)> for (bool) {
  fn setNotchesVisible_0(self , rsthis: & QDial) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QDial17setNotchesVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWrapping(bool)

/*

*/
impl /*struct*/ QDial {
  pub fn setWrapping_0<RetType, T: QDial_setWrapping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWrapping_0(self);
    // return 1;
  }
}
pub trait QDial_setWrapping_0<RetType> {
  fn setWrapping_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_setWrapping_0<(/*void*/)> for (bool) {
  fn setWrapping_0(self , rsthis: & QDial) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QDial11setWrappingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:84
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QDial {
  pub fn event_0<RetType, T: QDial_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QDial_event_0<RetType> {
  fn event_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QDial) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDial5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:85
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QDial {
  pub fn resizeEvent_0<RetType, T: QDial_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QDial_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QDial) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QDial11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:86
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QDial {
  pub fn paintEvent_0<RetType, T: QDial_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QDial_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QDial) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QDial10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:88
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QDial {
  pub fn mousePressEvent_0<RetType, T: QDial_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QDial_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QDial) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QDial15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:89
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QDial {
  pub fn mouseReleaseEvent_0<RetType, T: QDial_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QDial_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QDial) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QDial17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:90
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QDial {
  pub fn mouseMoveEvent_0<RetType, T: QDial_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QDial_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QDial) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QDial14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:92
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void sliderChange(QAbstractSlider::SliderChange)

/*
Reimplemented from QAbstractSlider::sliderChange().
*/
impl /*struct*/ QDial {
  pub fn sliderChange_0<RetType, T: QDial_sliderChange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sliderChange_0(self);
    // return 1;
  }
}
pub trait QDial_sliderChange_0<RetType> {
  fn sliderChange_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_sliderChange_0<(/*void*/)> for (i32) {
  fn sliderChange_0(self , rsthis: & QDial) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QDial12sliderChangeEN15QAbstractSlider12SliderChangeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdial.h:93
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionSlider *) const

/*
Initialize option with the values from this QDial. This method is useful for subclasses when they need a QStyleOptionSlider, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QDial {
  pub fn initStyleOption_0<RetType, T: QDial_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QDial_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QDial) -> RetType;
}
impl<'a> /*trait*/ QDial_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QDial) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK5QDial15initStyleOptionEP18QStyleOptionSlider", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
