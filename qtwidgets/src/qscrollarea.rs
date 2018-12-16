

// mod ::widgets::QScrollArea
// package qtwidgets
// /usr/include/qt/QtWidgets/qscrollarea.h
// #include <qscrollarea.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 14
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
// func (this *QScrollArea) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QScrollArea) InheritEventFilter(f func(arg0 *qtcore.QObject/*777 QObject **/, arg1 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QScrollArea) InheritResizeEvent(f func(arg0 *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void scrollContentsBy(int, int)
// func (this *QScrollArea) InheritScrollContentsBy(f func(dx int, dy int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scrollContentsBy", f)
// }

// QSize viewportSizeHint()
// func (this *QScrollArea) InheritViewportSizeHint(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "viewportSizeHint", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QScrollArea)=48
pub struct QScrollArea {
  qbase: QAbstractScrollArea,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QScrollArea_ITF interface {
//    QAbstractScrollArea_ITF
//    QScrollArea_PTR() *QScrollArea
//}
//func (ptr *QScrollArea) QScrollArea_PTR() *QScrollArea { return ptr }

impl /*struct*/ QScrollArea {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QScrollArea {
    return QScrollArea{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QScrollArea {
//  type Target = QScrollAreaBASE;
//
//  fn deref(&self) -> &QScrollAreaBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QScrollAreaBASE> for QScrollArea {
//  fn as_ref(& self) -> & QScrollAreaBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qscrollarea.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QScrollArea {
  pub fn metaObject_0<RetType, T: QScrollArea_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QScrollArea_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QScrollArea10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QScrollArea(QWidget *)

/*
Constructs an empty scroll area with the given parent.

See also setWidget().
*/
// QScrollArea(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QScrollArea {
  pub fn QScrollArea_0<T: QScrollArea_QScrollArea_0>(value: T) -> QScrollArea {
    let rsthis = value.QScrollArea_0();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollArea_QScrollArea_0 {
  fn QScrollArea_0(self) -> QScrollArea;
}
// QScrollArea(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QScrollArea_QScrollArea_0 for (usize) {
  fn QScrollArea_0(self) -> QScrollArea {
    // unsafe{_ZN11QScrollAreaC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QScrollAreaC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QScrollArea{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QScrollArea()

/*

*/
pub fn DeleteQScrollArea(this :*mut QScrollArea) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QScrollAreaD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qscrollarea.h:62
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widget() const

/*
Returns the scroll area's widget, or 0 if there is none.

See also setWidget().
*/
impl /*struct*/ QScrollArea {
  pub fn widget_0<RetType, T: QScrollArea_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QScrollArea_widget_0<RetType> {
  fn widget_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_widget_0<usize> for () {
  fn widget_0(self , rsthis: & QScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QScrollArea6widgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidget(QWidget *)

/*
Sets the scroll area's widget.

The widget becomes a child of the scroll area, and will be destroyed when the scroll area is deleted or when a new widget is set.

The widget's autoFillBackground property will be set to true.

If the scroll area is visible when the widget is added, you must show() it explicitly.

Note that You must add the layout of widget before you call this function; if you add it later, the widget will not be visible - regardless of when you show() the scroll area. In this case, you can also not show() the widget later.

See also widget().
*/
impl /*struct*/ QScrollArea {
  pub fn setWidget_0<RetType, T: QScrollArea_setWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidget_0(self);
    // return 1;
  }
}
pub trait QScrollArea_setWidget_0<RetType> {
  fn setWidget_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_setWidget_0<(/*void*/)> for (usize) {
  fn setWidget_0(self , rsthis: & QScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QScrollArea9setWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:64
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * takeWidget()

/*
Removes the scroll area's widget, and passes ownership of the widget to the caller.

See also widget().
*/
impl /*struct*/ QScrollArea {
  pub fn takeWidget_0<RetType, T: QScrollArea_takeWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeWidget_0(self);
    // return 1;
  }
}
pub trait QScrollArea_takeWidget_0<RetType> {
  fn takeWidget_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_takeWidget_0<usize> for () {
  fn takeWidget_0(self , rsthis: & QScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QScrollArea10takeWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:66
// index:0
// Public Visibility=Default Availability=Available
// [1] bool widgetResizable() const

/*

*/
impl /*struct*/ QScrollArea {
  pub fn widgetResizable_0<RetType, T: QScrollArea_widgetResizable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widgetResizable_0(self);
    // return 1;
  }
}
pub trait QScrollArea_widgetResizable_0<RetType> {
  fn widgetResizable_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_widgetResizable_0<bool> for () {
  fn widgetResizable_0(self , rsthis: & QScrollArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QScrollArea15widgetResizableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidgetResizable(bool)

/*

*/
impl /*struct*/ QScrollArea {
  pub fn setWidgetResizable_0<RetType, T: QScrollArea_setWidgetResizable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidgetResizable_0(self);
    // return 1;
  }
}
pub trait QScrollArea_setWidgetResizable_0<RetType> {
  fn setWidgetResizable_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_setWidgetResizable_0<(/*void*/)> for (bool) {
  fn setWidgetResizable_0(self , rsthis: & QScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QScrollArea18setWidgetResizableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QScrollArea {
  pub fn sizeHint_0<RetType, T: QScrollArea_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QScrollArea_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QScrollArea8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool focusNextPrevChild(bool)

/*
Reimplemented from QWidget::focusNextPrevChild().
*/
impl /*struct*/ QScrollArea {
  pub fn focusNextPrevChild_0<RetType, T: QScrollArea_focusNextPrevChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild_0(self);
    // return 1;
  }
}
pub trait QScrollArea_focusNextPrevChild_0<RetType> {
  fn focusNextPrevChild_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_focusNextPrevChild_0<bool> for (bool) {
  fn focusNextPrevChild_0(self , rsthis: & QScrollArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QScrollArea18focusNextPrevChildEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:73
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment alignment() const

/*

*/
impl /*struct*/ QScrollArea {
  pub fn alignment_0<RetType, T: QScrollArea_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QScrollArea_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_alignment_0<i32> for () {
  fn alignment_0(self , rsthis: & QScrollArea) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QScrollArea9alignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlignment(Qt::Alignment)

/*

*/
impl /*struct*/ QScrollArea {
  pub fn setAlignment_0<RetType, T: QScrollArea_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QScrollArea_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_setAlignment_0<(/*void*/)> for (i32) {
  fn setAlignment_0(self , rsthis: & QScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QScrollArea12setAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ensureVisible(int, int, int, int)

/*
Scrolls the contents of the scroll area so that the point (x, y) is visible inside the region of the viewport with margins specified in pixels by xmargin and ymargin. If the specified point cannot be reached, the contents are scrolled to the nearest valid position. The default value for both margins is 50 pixels.
*/
impl /*struct*/ QScrollArea {
  pub fn ensureVisible_0<RetType, T: QScrollArea_ensureVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ensureVisible_0(self);
    // return 1;
  }
}
pub trait QScrollArea_ensureVisible_0<RetType> {
  fn ensureVisible_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_ensureVisible_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn ensureVisible_0(self , rsthis: & QScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QScrollArea13ensureVisibleEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ensureWidgetVisible(QWidget *, int, int)

/*
Scrolls the contents of the scroll area so that the childWidget of QScrollArea::widget() is visible inside the viewport with margins specified in pixels by xmargin and ymargin. If the specified point cannot be reached, the contents are scrolled to the nearest valid position. The default value for both margins is 50 pixels.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QScrollArea {
  pub fn ensureWidgetVisible_0<RetType, T: QScrollArea_ensureWidgetVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ensureWidgetVisible_0(self);
    // return 1;
  }
}
pub trait QScrollArea_ensureWidgetVisible_0<RetType> {
  fn ensureWidgetVisible_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_ensureWidgetVisible_0<(/*void*/)> for (usize,i32,i32) {
  fn ensureWidgetVisible_0(self , rsthis: & QScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QScrollArea19ensureWidgetVisibleEP7QWidgetii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:81
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QScrollArea {
  pub fn event_0<RetType, T: QScrollArea_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QScrollArea_event_0<RetType> {
  fn event_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QScrollArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QScrollArea5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:82
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().
*/
impl /*struct*/ QScrollArea {
  pub fn eventFilter_0<RetType, T: QScrollArea_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QScrollArea_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QScrollArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QScrollArea11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:83
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QScrollArea {
  pub fn resizeEvent_0<RetType, T: QScrollArea_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QScrollArea_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QScrollArea11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:84
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void scrollContentsBy(int, int)

/*
Reimplemented from QAbstractScrollArea::scrollContentsBy().
*/
impl /*struct*/ QScrollArea {
  pub fn scrollContentsBy_0<RetType, T: QScrollArea_scrollContentsBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollContentsBy_0(self);
    // return 1;
  }
}
pub trait QScrollArea_scrollContentsBy_0<RetType> {
  fn scrollContentsBy_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_scrollContentsBy_0<(/*void*/)> for (i32,i32) {
  fn scrollContentsBy_0(self , rsthis: & QScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QScrollArea16scrollContentsByEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollarea.h:86
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QSize viewportSizeHint() const

/*
Reimplemented from QAbstractScrollArea::viewportSizeHint().
*/
impl /*struct*/ QScrollArea {
  pub fn viewportSizeHint_0<RetType, T: QScrollArea_viewportSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportSizeHint_0(self);
    // return 1;
  }
}
pub trait QScrollArea_viewportSizeHint_0<RetType> {
  fn viewportSizeHint_0(self , rsthis: & QScrollArea) -> RetType;
}
impl<'a> /*trait*/ QScrollArea_viewportSizeHint_0<usize> for () {
  fn viewportSizeHint_0(self , rsthis: & QScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QScrollArea16viewportSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
