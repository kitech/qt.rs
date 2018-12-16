

// mod ::widgets::QStackedWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qstackedwidget.h
// #include <qstackedwidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 25
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
// func (this *QStackedWidget) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QStackedWidget)=48
pub struct QStackedWidget {
  qbase: QFrame,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStackedWidget_ITF interface {
//    QFrame_ITF
//    QStackedWidget_PTR() *QStackedWidget
//}
//func (ptr *QStackedWidget) QStackedWidget_PTR() *QStackedWidget { return ptr }

impl /*struct*/ QStackedWidget {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStackedWidget {
    return QStackedWidget{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStackedWidget {
//  type Target = QStackedWidgetBASE;
//
//  fn deref(&self) -> &QStackedWidgetBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStackedWidgetBASE> for QStackedWidget {
//  fn as_ref(& self) -> & QStackedWidgetBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstackedwidget.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QStackedWidget {
  pub fn metaObject_0<RetType, T: QStackedWidget_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QStackedWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedWidget10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStackedWidget(QWidget *)

/*
Constructs a QStackedWidget with the given parent.

See also addWidget() and insertWidget().
*/
// QStackedWidget(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QStackedWidget {
  pub fn QStackedWidget_0<T: QStackedWidget_QStackedWidget_0>(value: T) -> QStackedWidget {
    let rsthis = value.QStackedWidget_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStackedWidget_QStackedWidget_0 {
  fn QStackedWidget_0(self) -> QStackedWidget;
}
// QStackedWidget(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStackedWidget_QStackedWidget_0 for (usize) {
  fn QStackedWidget_0(self) -> QStackedWidget {
    // unsafe{_ZN14QStackedWidgetC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QStackedWidgetC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStackedWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QStackedWidget()

/*

*/
pub fn DeleteQStackedWidget(this :*mut QStackedWidget) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QStackedWidgetD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qstackedwidget.h:62
// index:0
// Public Visibility=Default Availability=Available
// [4] int addWidget(QWidget *)

/*
Appends the given widget to the QStackedWidget and returns the index position. Ownership of widget is passed on to the QStackedWidget.

If the QStackedWidget is empty before this function is called, widget becomes the current widget.

See also insertWidget(), removeWidget(), and setCurrentWidget().
*/
impl /*struct*/ QStackedWidget {
  pub fn addWidget_0<RetType, T: QStackedWidget_addWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addWidget_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_addWidget_0<RetType> {
  fn addWidget_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_addWidget_0<i32> for (usize) {
  fn addWidget_0(self , rsthis: & QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStackedWidget9addWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:63
// index:0
// Public Visibility=Default Availability=Available
// [4] int insertWidget(int, QWidget *)

/*
Inserts the given widget at the given index in the QStackedWidget. Ownership of widget is passed on to the QStackedWidget. If index is out of range, the widget is appended (in which case it is the actual index of the widget that is returned).

If the QStackedWidget was empty before this function is called, the given widget becomes the current widget.

Inserting a new widget at an index less than or equal to the current index will increment the current index, but keep the current widget.

See also addWidget(), removeWidget(), and setCurrentWidget().
*/
impl /*struct*/ QStackedWidget {
  pub fn insertWidget_0<RetType, T: QStackedWidget_insertWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertWidget_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_insertWidget_0<RetType> {
  fn insertWidget_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_insertWidget_0<i32> for (i32,usize) {
  fn insertWidget_0(self , rsthis: & QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStackedWidget12insertWidgetEiP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeWidget(QWidget *)

/*
Removes widget from the QStackedWidget. i.e., widget is not deleted but simply removed from the stacked layout, causing it to be hidden.

Note: Parent object and parent widget of widget will remain the QStackedWidget. If the application wants to reuse the removed widget, then it is recommended to re-parent it.

See also addWidget(), insertWidget(), and currentWidget().
*/
impl /*struct*/ QStackedWidget {
  pub fn removeWidget_0<RetType, T: QStackedWidget_removeWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeWidget_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_removeWidget_0<RetType> {
  fn removeWidget_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_removeWidget_0<(/*void*/)> for (usize) {
  fn removeWidget_0(self , rsthis: & QStackedWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QStackedWidget12removeWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * currentWidget() const

/*
Returns the current widget, or 0 if there are no child widgets.

See also currentIndex() and setCurrentWidget().
*/
impl /*struct*/ QStackedWidget {
  pub fn currentWidget_0<RetType, T: QStackedWidget_currentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentWidget_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_currentWidget_0<RetType> {
  fn currentWidget_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_currentWidget_0<usize> for () {
  fn currentWidget_0(self , rsthis: & QStackedWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedWidget13currentWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:67
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentIndex() const

/*

*/
impl /*struct*/ QStackedWidget {
  pub fn currentIndex_0<RetType, T: QStackedWidget_currentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIndex_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_currentIndex_0<RetType> {
  fn currentIndex_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_currentIndex_0<i32> for () {
  fn currentIndex_0(self , rsthis: & QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedWidget12currentIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:69
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOf(QWidget *) const

/*
Returns the index of the given widget, or -1 if the given widget is not a child of the QStackedWidget.

See also currentIndex() and widget().
*/
impl /*struct*/ QStackedWidget {
  pub fn indexOf_0<RetType, T: QStackedWidget_indexOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_indexOf_0<RetType> {
  fn indexOf_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_indexOf_0<i32> for (usize) {
  fn indexOf_0(self , rsthis: & QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedWidget7indexOfEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:70
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widget(int) const

/*
Returns the widget at the given index, or 0 if there is no such widget.

See also currentWidget() and indexOf().
*/
impl /*struct*/ QStackedWidget {
  pub fn widget_0<RetType, T: QStackedWidget_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_widget_0<RetType> {
  fn widget_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_widget_0<usize> for (i32) {
  fn widget_0(self , rsthis: & QStackedWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedWidget6widgetEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:71
// index:0
// Public Visibility=Default Availability=Available
// [4] int count() const

/*

*/
impl /*struct*/ QStackedWidget {
  pub fn count_0<RetType, T: QStackedWidget_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_count_0<RetType> {
  fn count_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_count_0<i32> for () {
  fn count_0(self , rsthis: & QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedWidget5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentIndex(int)

/*

*/
impl /*struct*/ QStackedWidget {
  pub fn setCurrentIndex_0<RetType, T: QStackedWidget_setCurrentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_setCurrentIndex_0<RetType> {
  fn setCurrentIndex_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_setCurrentIndex_0<(/*void*/)> for (i32) {
  fn setCurrentIndex_0(self , rsthis: & QStackedWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QStackedWidget15setCurrentIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentWidget(QWidget *)

/*
Sets the current widget to be the specified widget. The new current widget must already be contained in this stacked widget.

See also currentWidget() and setCurrentIndex().
*/
impl /*struct*/ QStackedWidget {
  pub fn setCurrentWidget_0<RetType, T: QStackedWidget_setCurrentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentWidget_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_setCurrentWidget_0<RetType> {
  fn setCurrentWidget_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_setCurrentWidget_0<(/*void*/)> for (usize) {
  fn setCurrentWidget_0(self , rsthis: & QStackedWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QStackedWidget16setCurrentWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentChanged(int)

/*
This signal is emitted whenever the current widget changes.

The parameter holds the index of the new current widget, or -1 if there isn't a new one (for example, if there are no widgets in the QStackedWidget).

Note: Notifier signal for property currentIndex. 

See also currentWidget() and setCurrentWidget().
*/
impl /*struct*/ QStackedWidget {
  pub fn currentChanged_0<RetType, T: QStackedWidget_currentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentChanged_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_currentChanged_0<RetType> {
  fn currentChanged_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_currentChanged_0<(/*void*/)> for (i32) {
  fn currentChanged_0(self , rsthis: & QStackedWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QStackedWidget14currentChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void widgetRemoved(int)

/*
This signal is emitted whenever a widget is removed. The widget's index is passed as parameter.

See also removeWidget().
*/
impl /*struct*/ QStackedWidget {
  pub fn widgetRemoved_0<RetType, T: QStackedWidget_widgetRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widgetRemoved_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_widgetRemoved_0<RetType> {
  fn widgetRemoved_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_widgetRemoved_0<(/*void*/)> for (i32) {
  fn widgetRemoved_0(self , rsthis: & QStackedWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QStackedWidget13widgetRemovedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedwidget.h:82
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QStackedWidget {
  pub fn event_0<RetType, T: QStackedWidget_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QStackedWidget_event_0<RetType> {
  fn event_0(self , rsthis: & QStackedWidget) -> RetType;
}
impl<'a> /*trait*/ QStackedWidget_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QStackedWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStackedWidget5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
