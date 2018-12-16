

// mod ::widgets::QSizeGrip
// package qtwidgets
// /usr/include/qt/QtWidgets/qsizegrip.h
// #include <qsizegrip.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 19
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
// func (this *QSizeGrip) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QSizeGrip) InheritMousePressEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QSizeGrip) InheritMouseMoveEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QSizeGrip) InheritMouseReleaseEvent(f func(mouseEvent *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void moveEvent(QMoveEvent *)
// func (this *QSizeGrip) InheritMoveEvent(f func(moveEvent *qtgui.QMoveEvent/*777 QMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "moveEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QSizeGrip) InheritShowEvent(f func(showEvent *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void hideEvent(QHideEvent *)
// func (this *QSizeGrip) InheritHideEvent(f func(hideEvent *qtgui.QHideEvent/*777 QHideEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hideEvent", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QSizeGrip) InheritEventFilter(f func(arg0 *qtcore.QObject/*777 QObject **/, arg1 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// bool event(QEvent *)
// func (this *QSizeGrip) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSizeGrip)=48
pub struct QSizeGrip {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSizeGrip_ITF interface {
//    QWidget_ITF
//    QSizeGrip_PTR() *QSizeGrip
//}
//func (ptr *QSizeGrip) QSizeGrip_PTR() *QSizeGrip { return ptr }

impl /*struct*/ QSizeGrip {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSizeGrip {
    return QSizeGrip{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSizeGrip {
//  type Target = QSizeGripBASE;
//
//  fn deref(&self) -> &QSizeGripBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSizeGripBASE> for QSizeGrip {
//  fn as_ref(& self) -> & QSizeGripBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qsizegrip.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSizeGrip {
  pub fn metaObject_0<RetType, T: QSizeGrip_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSizeGrip_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSizeGrip) -> RetType;
}
impl<'a> /*trait*/ QSizeGrip_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSizeGrip) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSizeGrip10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizegrip.h:55
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSizeGrip(QWidget *)

/*
Constructs a resize corner as a child widget of the given parent.
*/
// QSizeGrip(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QSizeGrip {
  pub fn QSizeGrip_0<T: QSizeGrip_QSizeGrip_0>(value: T) -> QSizeGrip {
    let rsthis = value.QSizeGrip_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSizeGrip_QSizeGrip_0 {
  fn QSizeGrip_0(self) -> QSizeGrip;
}
// QSizeGrip(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSizeGrip_QSizeGrip_0 for (usize) {
  fn QSizeGrip_0(self) -> QSizeGrip {
    // unsafe{_ZN9QSizeGripC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QSizeGripC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSizeGrip{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizegrip.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSizeGrip()

/*

*/
pub fn DeleteQSizeGrip(this :*mut QSizeGrip) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QSizeGripD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qsizegrip.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QSizeGrip {
  pub fn sizeHint_0<RetType, T: QSizeGrip_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QSizeGrip_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QSizeGrip) -> RetType;
}
impl<'a> /*trait*/ QSizeGrip_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QSizeGrip) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSizeGrip8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizegrip.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*
Reimplemented from QWidget::setVisible().
*/
impl /*struct*/ QSizeGrip {
  pub fn setVisible_0<RetType, T: QSizeGrip_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QSizeGrip_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QSizeGrip) -> RetType;
}
impl<'a> /*trait*/ QSizeGrip_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QSizeGrip) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QSizeGrip10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizegrip.h:62
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().

Paints the resize grip.

Resize grips are usually rendered as small diagonal textured lines in the lower-right corner. The paint event is passed in the event parameter.
*/
impl /*struct*/ QSizeGrip {
  pub fn paintEvent_0<RetType, T: QSizeGrip_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QSizeGrip_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QSizeGrip) -> RetType;
}
impl<'a> /*trait*/ QSizeGrip_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QSizeGrip) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSizeGrip10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizegrip.h:63
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().

Receives the mouse press events for the widget, and primes the resize operation. The mouse press event is passed in the event parameter.
*/
impl /*struct*/ QSizeGrip {
  pub fn mousePressEvent_0<RetType, T: QSizeGrip_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QSizeGrip_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QSizeGrip) -> RetType;
}
impl<'a> /*trait*/ QSizeGrip_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QSizeGrip) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSizeGrip15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizegrip.h:64
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().

Resizes the top-level widget containing this widget. The mouse move event is passed in the event parameter.
*/
impl /*struct*/ QSizeGrip {
  pub fn mouseMoveEvent_0<RetType, T: QSizeGrip_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QSizeGrip_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QSizeGrip) -> RetType;
}
impl<'a> /*trait*/ QSizeGrip_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QSizeGrip) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSizeGrip14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizegrip.h:65
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QSizeGrip {
  pub fn mouseReleaseEvent_0<RetType, T: QSizeGrip_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QSizeGrip_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QSizeGrip) -> RetType;
}
impl<'a> /*trait*/ QSizeGrip_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QSizeGrip) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSizeGrip17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizegrip.h:66
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void moveEvent(QMoveEvent *)

/*
Reimplemented from QWidget::moveEvent().
*/
impl /*struct*/ QSizeGrip {
  pub fn moveEvent_0<RetType, T: QSizeGrip_moveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveEvent_0(self);
    // return 1;
  }
}
pub trait QSizeGrip_moveEvent_0<RetType> {
  fn moveEvent_0(self , rsthis: & QSizeGrip) -> RetType;
}
impl<'a> /*trait*/ QSizeGrip_moveEvent_0<(/*void*/)> for (usize) {
  fn moveEvent_0(self , rsthis: & QSizeGrip) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSizeGrip9moveEventEP10QMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizegrip.h:67
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QSizeGrip {
  pub fn showEvent_0<RetType, T: QSizeGrip_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QSizeGrip_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QSizeGrip) -> RetType;
}
impl<'a> /*trait*/ QSizeGrip_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QSizeGrip) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSizeGrip9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizegrip.h:68
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hideEvent(QHideEvent *)

/*
Reimplemented from QWidget::hideEvent().
*/
impl /*struct*/ QSizeGrip {
  pub fn hideEvent_0<RetType, T: QSizeGrip_hideEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideEvent_0(self);
    // return 1;
  }
}
pub trait QSizeGrip_hideEvent_0<RetType> {
  fn hideEvent_0(self , rsthis: & QSizeGrip) -> RetType;
}
impl<'a> /*trait*/ QSizeGrip_hideEvent_0<(/*void*/)> for (usize) {
  fn hideEvent_0(self , rsthis: & QSizeGrip) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSizeGrip9hideEventEP10QHideEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsizegrip.h:69
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().
*/
impl /*struct*/ QSizeGrip {
  pub fn eventFilter_0<RetType, T: QSizeGrip_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QSizeGrip_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QSizeGrip) -> RetType;
}
impl<'a> /*trait*/ QSizeGrip_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QSizeGrip) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QSizeGrip11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsizegrip.h:70
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QSizeGrip {
  pub fn event_0<RetType, T: QSizeGrip_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QSizeGrip_event_0<RetType> {
  fn event_0(self , rsthis: & QSizeGrip) -> RetType;
}
impl<'a> /*trait*/ QSizeGrip_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QSizeGrip) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QSizeGrip5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
