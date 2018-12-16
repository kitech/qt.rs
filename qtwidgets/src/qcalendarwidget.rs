

// mod ::widgets::QCalendarWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qcalendarwidget.h
// #include <qcalendarwidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 20
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
// func (this *QCalendarWidget) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QCalendarWidget) InheritEventFilter(f func(watched *qtcore.QObject/*777 QObject **/, event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QCalendarWidget) InheritMousePressEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QCalendarWidget) InheritResizeEvent(f func(event *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QCalendarWidget) InheritKeyPressEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void paintCell(QPainter *, const QRect &, const QDate &)
// func (this *QCalendarWidget) InheritPaintCell(f func(painter *qtgui.QPainter/*777 QPainter **/, rect *qtcore.QRect, date *qtcore.QDate) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintCell", f)
// }

// void updateCell(const QDate &)
// func (this *QCalendarWidget) InheritUpdateCell(f func(date *qtcore.QDate) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateCell", f)
// }

// void updateCells()
// func (this *QCalendarWidget) InheritUpdateCells(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateCells", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QCalendarWidget)=48
pub struct QCalendarWidget {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCalendarWidget_ITF interface {
//    QWidget_ITF
//    QCalendarWidget_PTR() *QCalendarWidget
//}
//func (ptr *QCalendarWidget) QCalendarWidget_PTR() *QCalendarWidget { return ptr }

impl /*struct*/ QCalendarWidget {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCalendarWidget {
    return QCalendarWidget{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCalendarWidget {
//  type Target = QCalendarWidgetBASE;
//
//  fn deref(&self) -> &QCalendarWidgetBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCalendarWidgetBASE> for QCalendarWidget {
//  fn as_ref(& self) -> & QCalendarWidgetBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qcalendarwidget.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn metaObject_0<RetType, T: QCalendarWidget_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QCalendarWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QCalendarWidget(QWidget *)

/*
Constructs a calendar widget with the given parent.

The widget is initialized with the current month and year, and the currently selected date is today.

See also setCurrentPage().
*/
// QCalendarWidget(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QCalendarWidget {
  pub fn QCalendarWidget_0<T: QCalendarWidget_QCalendarWidget_0>(value: T) -> QCalendarWidget {
    let rsthis = value.QCalendarWidget_0();
    return rsthis;
    // return 1;
  }
}

pub trait QCalendarWidget_QCalendarWidget_0 {
  fn QCalendarWidget_0(self) -> QCalendarWidget;
}
// QCalendarWidget(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCalendarWidget_QCalendarWidget_0 for (usize) {
  fn QCalendarWidget_0(self) -> QCalendarWidget {
    // unsafe{_ZN15QCalendarWidgetC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QCalendarWidgetC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCalendarWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:93
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QCalendarWidget()

/*

*/
pub fn DeleteQCalendarWidget(this :*mut QCalendarWidget) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QCalendarWidgetD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qcalendarwidget.h:95
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QCalendarWidget {
  pub fn sizeHint_0<RetType, T: QCalendarWidget_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QCalendarWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:96
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QCalendarWidget {
  pub fn minimumSizeHint_0<RetType, T: QCalendarWidget_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QCalendarWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:98
// index:0
// Public Visibility=Default Availability=Available
// [8] QDate selectedDate() const

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn selectedDate_0<RetType, T: QCalendarWidget_selectedDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedDate_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_selectedDate_0<RetType> {
  fn selectedDate_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_selectedDate_0<usize> for () {
  fn selectedDate_0(self , rsthis: & QCalendarWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget12selectedDateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:100
// index:0
// Public Visibility=Default Availability=Available
// [4] int yearShown() const

/*
Returns the year of the currently displayed month. Months are numbered from 1 to 12.

See also monthShown() and setCurrentPage().
*/
impl /*struct*/ QCalendarWidget {
  pub fn yearShown_0<RetType, T: QCalendarWidget_yearShown_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.yearShown_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_yearShown_0<RetType> {
  fn yearShown_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_yearShown_0<i32> for () {
  fn yearShown_0(self , rsthis: & QCalendarWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget9yearShownEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] int monthShown() const

/*
Returns the currently displayed month. Months are numbered from 1 to 12.

See also yearShown() and setCurrentPage().
*/
impl /*struct*/ QCalendarWidget {
  pub fn monthShown_0<RetType, T: QCalendarWidget_monthShown_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.monthShown_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_monthShown_0<RetType> {
  fn monthShown_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_monthShown_0<i32> for () {
  fn monthShown_0(self , rsthis: & QCalendarWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget10monthShownEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] QDate minimumDate() const

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn minimumDate_0<RetType, T: QCalendarWidget_minimumDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumDate_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_minimumDate_0<RetType> {
  fn minimumDate_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_minimumDate_0<usize> for () {
  fn minimumDate_0(self , rsthis: & QCalendarWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget11minimumDateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumDate(const QDate &)

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn setMinimumDate_0<RetType, T: QCalendarWidget_setMinimumDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumDate_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setMinimumDate_0<RetType> {
  fn setMinimumDate_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setMinimumDate_0<(/*void*/)> for (usize) {
  fn setMinimumDate_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget14setMinimumDateERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:106
// index:0
// Public Visibility=Default Availability=Available
// [8] QDate maximumDate() const

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn maximumDate_0<RetType, T: QCalendarWidget_maximumDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumDate_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_maximumDate_0<RetType> {
  fn maximumDate_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_maximumDate_0<usize> for () {
  fn maximumDate_0(self , rsthis: & QCalendarWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget11maximumDateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumDate(const QDate &)

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn setMaximumDate_0<RetType, T: QCalendarWidget_setMaximumDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumDate_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setMaximumDate_0<RetType> {
  fn setMaximumDate_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setMaximumDate_0<(/*void*/)> for (usize) {
  fn setMaximumDate_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget14setMaximumDateERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:109
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::DayOfWeek firstDayOfWeek() const

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn firstDayOfWeek_0<RetType, T: QCalendarWidget_firstDayOfWeek_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.firstDayOfWeek_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_firstDayOfWeek_0<RetType> {
  fn firstDayOfWeek_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_firstDayOfWeek_0<i32> for () {
  fn firstDayOfWeek_0(self , rsthis: & QCalendarWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget14firstDayOfWeekEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFirstDayOfWeek(Qt::DayOfWeek)

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn setFirstDayOfWeek_0<RetType, T: QCalendarWidget_setFirstDayOfWeek_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFirstDayOfWeek_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setFirstDayOfWeek_0<RetType> {
  fn setFirstDayOfWeek_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setFirstDayOfWeek_0<(/*void*/)> for (i32) {
  fn setFirstDayOfWeek_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget17setFirstDayOfWeekEN2Qt9DayOfWeekE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:112
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNavigationBarVisible() const

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn isNavigationBarVisible_0<RetType, T: QCalendarWidget_isNavigationBarVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNavigationBarVisible_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_isNavigationBarVisible_0<RetType> {
  fn isNavigationBarVisible_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_isNavigationBarVisible_0<bool> for () {
  fn isNavigationBarVisible_0(self , rsthis: & QCalendarWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget22isNavigationBarVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:113
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isGridVisible() const

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn isGridVisible_0<RetType, T: QCalendarWidget_isGridVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isGridVisible_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_isGridVisible_0<RetType> {
  fn isGridVisible_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_isGridVisible_0<bool> for () {
  fn isGridVisible_0(self , rsthis: & QCalendarWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget13isGridVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:115
// index:0
// Public Visibility=Default Availability=Available
// [4] QCalendarWidget::SelectionMode selectionMode() const

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn selectionMode_0<RetType, T: QCalendarWidget_selectionMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionMode_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_selectionMode_0<RetType> {
  fn selectionMode_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_selectionMode_0<i32> for () {
  fn selectionMode_0(self , rsthis: & QCalendarWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget13selectionModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSelectionMode(QCalendarWidget::SelectionMode)

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn setSelectionMode_0<RetType, T: QCalendarWidget_setSelectionMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionMode_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setSelectionMode_0<RetType> {
  fn setSelectionMode_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setSelectionMode_0<(/*void*/)> for (i32) {
  fn setSelectionMode_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget16setSelectionModeENS_13SelectionModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:118
// index:0
// Public Visibility=Default Availability=Available
// [4] QCalendarWidget::HorizontalHeaderFormat horizontalHeaderFormat() const

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn horizontalHeaderFormat_0<RetType, T: QCalendarWidget_horizontalHeaderFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalHeaderFormat_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_horizontalHeaderFormat_0<RetType> {
  fn horizontalHeaderFormat_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_horizontalHeaderFormat_0<i32> for () {
  fn horizontalHeaderFormat_0(self , rsthis: & QCalendarWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget22horizontalHeaderFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHorizontalHeaderFormat(QCalendarWidget::HorizontalHeaderFormat)

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn setHorizontalHeaderFormat_0<RetType, T: QCalendarWidget_setHorizontalHeaderFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalHeaderFormat_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setHorizontalHeaderFormat_0<RetType> {
  fn setHorizontalHeaderFormat_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setHorizontalHeaderFormat_0<(/*void*/)> for (i32) {
  fn setHorizontalHeaderFormat_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget25setHorizontalHeaderFormatENS_22HorizontalHeaderFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:121
// index:0
// Public Visibility=Default Availability=Available
// [4] QCalendarWidget::VerticalHeaderFormat verticalHeaderFormat() const

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn verticalHeaderFormat_0<RetType, T: QCalendarWidget_verticalHeaderFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalHeaderFormat_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_verticalHeaderFormat_0<RetType> {
  fn verticalHeaderFormat_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_verticalHeaderFormat_0<i32> for () {
  fn verticalHeaderFormat_0(self , rsthis: & QCalendarWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget20verticalHeaderFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:122
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalHeaderFormat(QCalendarWidget::VerticalHeaderFormat)

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn setVerticalHeaderFormat_0<RetType, T: QCalendarWidget_setVerticalHeaderFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalHeaderFormat_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setVerticalHeaderFormat_0<RetType> {
  fn setVerticalHeaderFormat_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setVerticalHeaderFormat_0<(/*void*/)> for (i32) {
  fn setVerticalHeaderFormat_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget23setVerticalHeaderFormatENS_20VerticalHeaderFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:124
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextCharFormat headerTextFormat() const

/*
Returns the text char format for rendering the header.

See also setHeaderTextFormat().
*/
impl /*struct*/ QCalendarWidget {
  pub fn headerTextFormat_0<RetType, T: QCalendarWidget_headerTextFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.headerTextFormat_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_headerTextFormat_0<RetType> {
  fn headerTextFormat_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_headerTextFormat_0<usize> for () {
  fn headerTextFormat_0(self , rsthis: & QCalendarWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget16headerTextFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:125
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHeaderTextFormat(const QTextCharFormat &)

/*
Sets the text char format for rendering the header to format. If you also set a weekday text format, this format's foreground and background color will take precedence over the header's format. The other formatting information will still be decided by the header's format.

See also headerTextFormat().
*/
impl /*struct*/ QCalendarWidget {
  pub fn setHeaderTextFormat_0<RetType, T: QCalendarWidget_setHeaderTextFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeaderTextFormat_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setHeaderTextFormat_0<RetType> {
  fn setHeaderTextFormat_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setHeaderTextFormat_0<(/*void*/)> for (usize) {
  fn setHeaderTextFormat_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget19setHeaderTextFormatERK15QTextCharFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:127
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextCharFormat weekdayTextFormat(Qt::DayOfWeek) const

/*
Returns the text char format for rendering of day in the week dayOfWeek.

See also setWeekdayTextFormat() and headerTextFormat().
*/
impl /*struct*/ QCalendarWidget {
  pub fn weekdayTextFormat_0<RetType, T: QCalendarWidget_weekdayTextFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.weekdayTextFormat_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_weekdayTextFormat_0<RetType> {
  fn weekdayTextFormat_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_weekdayTextFormat_0<usize> for (i32) {
  fn weekdayTextFormat_0(self , rsthis: & QCalendarWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget17weekdayTextFormatEN2Qt9DayOfWeekE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:128
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWeekdayTextFormat(Qt::DayOfWeek, const QTextCharFormat &)

/*
Sets the text char format for rendering of day in the week dayOfWeek to format. The format will take precedence over the header format in case of foreground and background color. Other text formatting information is taken from the headers format.

See also weekdayTextFormat() and setHeaderTextFormat().
*/
impl /*struct*/ QCalendarWidget {
  pub fn setWeekdayTextFormat_0<RetType, T: QCalendarWidget_setWeekdayTextFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWeekdayTextFormat_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setWeekdayTextFormat_0<RetType> {
  fn setWeekdayTextFormat_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setWeekdayTextFormat_0<(/*void*/)> for (i32,usize) {
  fn setWeekdayTextFormat_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget20setWeekdayTextFormatEN2Qt9DayOfWeekERK15QTextCharFormat", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:131
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextCharFormat dateTextFormat(const QDate &) const

/*
Returns a QMap from QDate to QTextCharFormat showing all dates that use a special format that alters their rendering.

See also setDateTextFormat().
*/
impl /*struct*/ QCalendarWidget {
  pub fn dateTextFormat_0<RetType, T: QCalendarWidget_dateTextFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dateTextFormat_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_dateTextFormat_0<RetType> {
  fn dateTextFormat_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_dateTextFormat_0<usize> for (usize) {
  fn dateTextFormat_0(self , rsthis: & QCalendarWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget14dateTextFormatERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDateTextFormat(const QDate &, const QTextCharFormat &)

/*
Sets the format used to render the given date to that specified by format.

If date is null, all date formats are cleared.

See also dateTextFormat().
*/
impl /*struct*/ QCalendarWidget {
  pub fn setDateTextFormat_0<RetType, T: QCalendarWidget_setDateTextFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDateTextFormat_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setDateTextFormat_0<RetType> {
  fn setDateTextFormat_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setDateTextFormat_0<(/*void*/)> for (usize,usize) {
  fn setDateTextFormat_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget17setDateTextFormatERK5QDateRK15QTextCharFormat", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:134
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDateEditEnabled() const

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn isDateEditEnabled_0<RetType, T: QCalendarWidget_isDateEditEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDateEditEnabled_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_isDateEditEnabled_0<RetType> {
  fn isDateEditEnabled_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_isDateEditEnabled_0<bool> for () {
  fn isDateEditEnabled_0(self , rsthis: & QCalendarWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget17isDateEditEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDateEditEnabled(bool)

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn setDateEditEnabled_0<RetType, T: QCalendarWidget_setDateEditEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDateEditEnabled_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setDateEditEnabled_0<RetType> {
  fn setDateEditEnabled_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setDateEditEnabled_0<(/*void*/)> for (bool) {
  fn setDateEditEnabled_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget18setDateEditEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:137
// index:0
// Public Visibility=Default Availability=Available
// [4] int dateEditAcceptDelay() const

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn dateEditAcceptDelay_0<RetType, T: QCalendarWidget_dateEditAcceptDelay_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dateEditAcceptDelay_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_dateEditAcceptDelay_0<RetType> {
  fn dateEditAcceptDelay_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_dateEditAcceptDelay_0<i32> for () {
  fn dateEditAcceptDelay_0(self , rsthis: & QCalendarWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget19dateEditAcceptDelayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:138
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDateEditAcceptDelay(int)

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn setDateEditAcceptDelay_0<RetType, T: QCalendarWidget_setDateEditAcceptDelay_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDateEditAcceptDelay_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setDateEditAcceptDelay_0<RetType> {
  fn setDateEditAcceptDelay_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setDateEditAcceptDelay_0<(/*void*/)> for (i32) {
  fn setDateEditAcceptDelay_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget22setDateEditAcceptDelayEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:141
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QCalendarWidget {
  pub fn event_0<RetType, T: QCalendarWidget_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_event_0<RetType> {
  fn event_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QCalendarWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QCalendarWidget5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:142
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().
*/
impl /*struct*/ QCalendarWidget {
  pub fn eventFilter_0<RetType, T: QCalendarWidget_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QCalendarWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QCalendarWidget11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:143
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QCalendarWidget {
  pub fn mousePressEvent_0<RetType, T: QCalendarWidget_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:144
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QCalendarWidget {
  pub fn resizeEvent_0<RetType, T: QCalendarWidget_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:145
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QCalendarWidget {
  pub fn keyPressEvent_0<RetType, T: QCalendarWidget_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:147
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintCell(QPainter *, const QRect &, const QDate &) const

/*
Paints the cell specified by the given date, using the given painter and rect.
*/
impl /*struct*/ QCalendarWidget {
  pub fn paintCell_0<RetType, T: QCalendarWidget_paintCell_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintCell_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_paintCell_0<RetType> {
  fn paintCell_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_paintCell_0<(/*void*/)> for (usize,usize,usize) {
  fn paintCell_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK15QCalendarWidget9paintCellEP8QPainterRK5QRectRK5QDate", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:148
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void updateCell(const QDate &)

/*
Updates the cell specified by the given date unless updates are disabled or the cell is hidden.

This function was introduced in  Qt 4.4.

See also updateCells(), yearShown(), and monthShown().
*/
impl /*struct*/ QCalendarWidget {
  pub fn updateCell_0<RetType, T: QCalendarWidget_updateCell_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateCell_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_updateCell_0<RetType> {
  fn updateCell_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_updateCell_0<(/*void*/)> for (usize) {
  fn updateCell_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget10updateCellERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:149
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void updateCells()

/*
Updates all visible cells unless updates are disabled.

This function was introduced in  Qt 4.4.

See also updateCell().
*/
impl /*struct*/ QCalendarWidget {
  pub fn updateCells_0<RetType, T: QCalendarWidget_updateCells_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateCells_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_updateCells_0<RetType> {
  fn updateCells_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_updateCells_0<(/*void*/)> for () {
  fn updateCells_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget11updateCellsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSelectedDate(const QDate &)

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn setSelectedDate_0<RetType, T: QCalendarWidget_setSelectedDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectedDate_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setSelectedDate_0<RetType> {
  fn setSelectedDate_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setSelectedDate_0<(/*void*/)> for (usize) {
  fn setSelectedDate_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget15setSelectedDateERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:153
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDateRange(const QDate &, const QDate &)

/*
Defines a date range by setting the minimumDate and maximumDate properties.

The date range restricts the user selection, i.e. the user can only select dates within the specified date range. Note that


  QCalendarWidget *calendar;

  calendar->setDateRange(min, max);



is analogous to


  QCalendarWidget *calendar;

  calendar->setMinimumDate(min);
  calendar->setMaximumDate(max);



If either the min or max parameters are not valid QDate objects, this function does nothing.

See also setMinimumDate() and setMaximumDate().
*/
impl /*struct*/ QCalendarWidget {
  pub fn setDateRange_0<RetType, T: QCalendarWidget_setDateRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDateRange_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setDateRange_0<RetType> {
  fn setDateRange_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setDateRange_0<(/*void*/)> for (usize,usize) {
  fn setDateRange_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget12setDateRangeERK5QDateS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:154
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentPage(int, int)

/*
Displays the given month of the given year without changing the selected date. Use the setSelectedDate() function to alter the selected date.

The currently displayed month and year can be retrieved using the monthShown() and yearShown() functions respectively.

See also yearShown(), monthShown(), showPreviousMonth(), showNextMonth(), showPreviousYear(), and showNextYear().
*/
impl /*struct*/ QCalendarWidget {
  pub fn setCurrentPage_0<RetType, T: QCalendarWidget_setCurrentPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentPage_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setCurrentPage_0<RetType> {
  fn setCurrentPage_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setCurrentPage_0<(/*void*/)> for (i32,i32) {
  fn setCurrentPage_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget14setCurrentPageEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGridVisible(bool)

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn setGridVisible_0<RetType, T: QCalendarWidget_setGridVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGridVisible_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setGridVisible_0<RetType> {
  fn setGridVisible_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setGridVisible_0<(/*void*/)> for (bool) {
  fn setGridVisible_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget14setGridVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNavigationBarVisible(bool)

/*

*/
impl /*struct*/ QCalendarWidget {
  pub fn setNavigationBarVisible_0<RetType, T: QCalendarWidget_setNavigationBarVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNavigationBarVisible_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_setNavigationBarVisible_0<RetType> {
  fn setNavigationBarVisible_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_setNavigationBarVisible_0<(/*void*/)> for (bool) {
  fn setNavigationBarVisible_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget23setNavigationBarVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showNextMonth()

/*
Shows the next month relative to the currently displayed month. Note that the selected date is not changed.

See also showPreviousMonth(), setCurrentPage(), and setSelectedDate().
*/
impl /*struct*/ QCalendarWidget {
  pub fn showNextMonth_0<RetType, T: QCalendarWidget_showNextMonth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showNextMonth_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_showNextMonth_0<RetType> {
  fn showNextMonth_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_showNextMonth_0<(/*void*/)> for () {
  fn showNextMonth_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget13showNextMonthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showPreviousMonth()

/*
Shows the previous month relative to the currently displayed month. Note that the selected date is not changed.

See also showNextMonth(), setCurrentPage(), and setSelectedDate().
*/
impl /*struct*/ QCalendarWidget {
  pub fn showPreviousMonth_0<RetType, T: QCalendarWidget_showPreviousMonth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showPreviousMonth_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_showPreviousMonth_0<RetType> {
  fn showPreviousMonth_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_showPreviousMonth_0<(/*void*/)> for () {
  fn showPreviousMonth_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget17showPreviousMonthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:159
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showNextYear()

/*
Shows the currently displayed month in the next year relative to the currently displayed year. Note that the selected date is not changed.

See also showPreviousYear(), setCurrentPage(), and setSelectedDate().
*/
impl /*struct*/ QCalendarWidget {
  pub fn showNextYear_0<RetType, T: QCalendarWidget_showNextYear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showNextYear_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_showNextYear_0<RetType> {
  fn showNextYear_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_showNextYear_0<(/*void*/)> for () {
  fn showNextYear_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget12showNextYearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:160
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showPreviousYear()

/*
Shows the currently displayed month in the previous year relative to the currently displayed year. Note that the selected date is not changed.

See also showNextYear(), setCurrentPage(), and setSelectedDate().
*/
impl /*struct*/ QCalendarWidget {
  pub fn showPreviousYear_0<RetType, T: QCalendarWidget_showPreviousYear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showPreviousYear_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_showPreviousYear_0<RetType> {
  fn showPreviousYear_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_showPreviousYear_0<(/*void*/)> for () {
  fn showPreviousYear_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget16showPreviousYearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:161
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showSelectedDate()

/*
Shows the month of the selected date.

See also selectedDate() and setCurrentPage().
*/
impl /*struct*/ QCalendarWidget {
  pub fn showSelectedDate_0<RetType, T: QCalendarWidget_showSelectedDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showSelectedDate_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_showSelectedDate_0<RetType> {
  fn showSelectedDate_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_showSelectedDate_0<(/*void*/)> for () {
  fn showSelectedDate_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget16showSelectedDateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:162
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showToday()

/*
Shows the month of the today's date.

See also selectedDate() and setCurrentPage().
*/
impl /*struct*/ QCalendarWidget {
  pub fn showToday_0<RetType, T: QCalendarWidget_showToday_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showToday_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_showToday_0<RetType> {
  fn showToday_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_showToday_0<(/*void*/)> for () {
  fn showToday_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget9showTodayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:165
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectionChanged()

/*
This signal is emitted when the currently selected date is changed.

The currently selected date can be changed by the user using the mouse or keyboard, or by the programmer using setSelectedDate().

See also selectedDate().
*/
impl /*struct*/ QCalendarWidget {
  pub fn selectionChanged_0<RetType, T: QCalendarWidget_selectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_selectionChanged_0<RetType> {
  fn selectionChanged_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_selectionChanged_0<(/*void*/)> for () {
  fn selectionChanged_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget16selectionChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:166
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clicked(const QDate &)

/*
This signal is emitted when a mouse button is clicked. The date the mouse was clicked on is specified by date. The signal is only emitted when clicked on a valid date, e.g., dates are not outside the minimumDate() and maximumDate(). If the selection mode is NoSelection, this signal will not be emitted.
*/
impl /*struct*/ QCalendarWidget {
  pub fn clicked_0<RetType, T: QCalendarWidget_clicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clicked_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_clicked_0<RetType> {
  fn clicked_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_clicked_0<(/*void*/)> for (usize) {
  fn clicked_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget7clickedERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:167
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activated(const QDate &)

/*
This signal is emitted whenever the user presses the Return or Enter key or double-clicks a date in the calendar widget.
*/
impl /*struct*/ QCalendarWidget {
  pub fn activated_0<RetType, T: QCalendarWidget_activated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activated_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_activated_0<RetType> {
  fn activated_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_activated_0<(/*void*/)> for (usize) {
  fn activated_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget9activatedERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcalendarwidget.h:168
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentPageChanged(int, int)

/*
This signal is emitted when the currently shown month is changed. The new year and month are passed as parameters.

See also setCurrentPage().
*/
impl /*struct*/ QCalendarWidget {
  pub fn currentPageChanged_0<RetType, T: QCalendarWidget_currentPageChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentPageChanged_0(self);
    // return 1;
  }
}
pub trait QCalendarWidget_currentPageChanged_0<RetType> {
  fn currentPageChanged_0(self , rsthis: & QCalendarWidget) -> RetType;
}
impl<'a> /*trait*/ QCalendarWidget_currentPageChanged_0<(/*void*/)> for (i32,i32) {
  fn currentPageChanged_0(self , rsthis: & QCalendarWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QCalendarWidget18currentPageChangedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum type defines the various formats the horizontal header can display.



See also horizontalHeaderFormat() and VerticalHeaderFormat.

*/
pub type QCalendarWidget__HorizontalHeaderFormat = i32;
// The header is hidden.
pub const QCalendarWidget__NoHorizontalHeader :QCalendarWidget__HorizontalHeaderFormat = 0;
// The header displays a single letter abbreviation for day names (e.g. M for Monday).
pub const QCalendarWidget__SingleLetterDayNames :QCalendarWidget__HorizontalHeaderFormat = 1;
// The header displays a short abbreviation for day names (e.g. Mon for Monday).
pub const QCalendarWidget__ShortDayNames :QCalendarWidget__HorizontalHeaderFormat = 2;
// The header displays complete day names (e.g. Monday).
pub const QCalendarWidget__LongDayNames :QCalendarWidget__HorizontalHeaderFormat = 3;
pub fn QCalendarWidget_HorizontalHeaderFormatItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QCalendarWidget", val);
}
pub fn QCalendarWidget_HorizontalHeaderFormatItemName_s(val: i32) ->String {
  //var nilthis *QCalendarWidget
  //return nilthis.HorizontalHeaderFormatItemName(val);
  return QCalendarWidget_HorizontalHeaderFormatItemName(val);
}


/*
This enum type defines the various formats the vertical header can display.



See also verticalHeaderFormat() and HorizontalHeaderFormat.

*/
pub type QCalendarWidget__VerticalHeaderFormat = i32;
// The header is hidden.
pub const QCalendarWidget__NoVerticalHeader :QCalendarWidget__VerticalHeaderFormat = 0;
// The header displays ISO week numbers as described by QDate::weekNumber().
pub const QCalendarWidget__ISOWeekNumbers :QCalendarWidget__VerticalHeaderFormat = 1;
pub fn QCalendarWidget_VerticalHeaderFormatItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QCalendarWidget", val);
}
pub fn QCalendarWidget_VerticalHeaderFormatItemName_s(val: i32) ->String {
  //var nilthis *QCalendarWidget
  //return nilthis.VerticalHeaderFormatItemName(val);
  return QCalendarWidget_VerticalHeaderFormatItemName(val);
}


/*
This enum describes the types of selection offered to the user for selecting dates in the calendar.



See also selectionMode.

*/
pub type QCalendarWidget__SelectionMode = i32;
// Dates cannot be selected.
pub const QCalendarWidget__NoSelection :QCalendarWidget__SelectionMode = 0;
// Single dates can be selected.
pub const QCalendarWidget__SingleSelection :QCalendarWidget__SelectionMode = 1;
pub fn QCalendarWidget_SelectionModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QCalendarWidget", val);
}
pub fn QCalendarWidget_SelectionModeItemName_s(val: i32) ->String {
  //var nilthis *QCalendarWidget
  //return nilthis.SelectionModeItemName(val);
  return QCalendarWidget_SelectionModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
