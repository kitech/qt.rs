

// mod ::widgets::QFocusFrame
// package qtwidgets
// /usr/include/qt/QtWidgets/qfocusframe.h
// #include <qfocusframe.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 53
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
// func (this *QFocusFrame) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QFocusFrame) InheritEventFilter(f func(arg0 *qtcore.QObject/*777 QObject **/, arg1 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QFocusFrame) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void initStyleOption(QStyleOption *)
// func (this *QFocusFrame) InheritInitStyleOption(f func(option *QStyleOption/*777 QStyleOption **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QFocusFrame)=48
pub struct QFocusFrame {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFocusFrame_ITF interface {
//    QWidget_ITF
//    QFocusFrame_PTR() *QFocusFrame
//}
//func (ptr *QFocusFrame) QFocusFrame_PTR() *QFocusFrame { return ptr }

impl /*struct*/ QFocusFrame {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFocusFrame {
    return QFocusFrame{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFocusFrame {
//  type Target = QFocusFrameBASE;
//
//  fn deref(&self) -> &QFocusFrameBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFocusFrameBASE> for QFocusFrame {
//  fn as_ref(& self) -> & QFocusFrameBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qfocusframe.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QFocusFrame {
  pub fn metaObject_0<RetType, T: QFocusFrame_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QFocusFrame_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QFocusFrame) -> RetType;
}
impl<'a> /*trait*/ QFocusFrame_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QFocusFrame) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFocusFrame10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfocusframe.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFocusFrame(QWidget *)

/*
Constructs a QFocusFrame.

The focus frame will not monitor parent for updates but rather can be placed manually or by using QFocusFrame::setWidget. A QFocusFrame sets Qt::WA_NoChildEventsForParent attribute; as a result the parent will not receive a QEvent::ChildAdded event, this will make it possible to manually set the geometry of the QFocusFrame inside of a QSplitter or other child event monitoring widget.

See also QFocusFrame::setWidget().
*/
// QFocusFrame(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QFocusFrame {
  pub fn QFocusFrame_0<T: QFocusFrame_QFocusFrame_0>(value: T) -> QFocusFrame {
    let rsthis = value.QFocusFrame_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFocusFrame_QFocusFrame_0 {
  fn QFocusFrame_0(self) -> QFocusFrame;
}
// QFocusFrame(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFocusFrame_QFocusFrame_0 for (usize) {
  fn QFocusFrame_0(self) -> QFocusFrame {
    // unsafe{_ZN11QFocusFrameC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QFocusFrameC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFocusFrame{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfocusframe.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFocusFrame()

/*

*/
pub fn DeleteQFocusFrame(this :*mut QFocusFrame) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QFocusFrameD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qfocusframe.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidget(QWidget *)

/*
QFocusFrame will track changes to widget and resize itself automatically. If the monitored widget's parent changes, QFocusFrame will follow the widget and place itself around the widget automatically. If the monitored widget is deleted, QFocusFrame will set it to zero.

See also QFocusFrame::widget().
*/
impl /*struct*/ QFocusFrame {
  pub fn setWidget_0<RetType, T: QFocusFrame_setWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidget_0(self);
    // return 1;
  }
}
pub trait QFocusFrame_setWidget_0<RetType> {
  fn setWidget_0(self , rsthis: & QFocusFrame) -> RetType;
}
impl<'a> /*trait*/ QFocusFrame_setWidget_0<(/*void*/)> for (usize) {
  fn setWidget_0(self , rsthis: & QFocusFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFocusFrame9setWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfocusframe.h:60
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widget() const

/*
Returns the currently monitored widget for automatically resize and update.

See also QFocusFrame::setWidget().
*/
impl /*struct*/ QFocusFrame {
  pub fn widget_0<RetType, T: QFocusFrame_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QFocusFrame_widget_0<RetType> {
  fn widget_0(self , rsthis: & QFocusFrame) -> RetType;
}
impl<'a> /*trait*/ QFocusFrame_widget_0<usize> for () {
  fn widget_0(self , rsthis: & QFocusFrame) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFocusFrame6widgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfocusframe.h:63
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QFocusFrame {
  pub fn event_0<RetType, T: QFocusFrame_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QFocusFrame_event_0<RetType> {
  fn event_0(self , rsthis: & QFocusFrame) -> RetType;
}
impl<'a> /*trait*/ QFocusFrame_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QFocusFrame) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFocusFrame5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfocusframe.h:65
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().
*/
impl /*struct*/ QFocusFrame {
  pub fn eventFilter_0<RetType, T: QFocusFrame_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QFocusFrame_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QFocusFrame) -> RetType;
}
impl<'a> /*trait*/ QFocusFrame_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QFocusFrame) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFocusFrame11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfocusframe.h:66
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QFocusFrame {
  pub fn paintEvent_0<RetType, T: QFocusFrame_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QFocusFrame_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QFocusFrame) -> RetType;
}
impl<'a> /*trait*/ QFocusFrame_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QFocusFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFocusFrame10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfocusframe.h:67
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOption *) const

/*
Initialize option with the values from this QFocusFrame. This method is useful for subclasses when they need a QStyleOption, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QFocusFrame {
  pub fn initStyleOption_0<RetType, T: QFocusFrame_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QFocusFrame_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QFocusFrame) -> RetType;
}
impl<'a> /*trait*/ QFocusFrame_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QFocusFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QFocusFrame15initStyleOptionEP12QStyleOption", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
