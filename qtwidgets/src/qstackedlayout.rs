

// mod ::widgets::QStackedLayout
// package qtwidgets
// /usr/include/qt/QtWidgets/qstackedlayout.h
// #include <qstackedlayout.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 16
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
#[derive(Default)] // class sizeof(QStackedLayout)=32
pub struct QStackedLayout {
  qbase: QLayout,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStackedLayout_ITF interface {
//    QLayout_ITF
//    QStackedLayout_PTR() *QStackedLayout
//}
//func (ptr *QStackedLayout) QStackedLayout_PTR() *QStackedLayout { return ptr }

impl /*struct*/ QStackedLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStackedLayout {
    return QStackedLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStackedLayout {
//  type Target = QStackedLayoutBASE;
//
//  fn deref(&self) -> &QStackedLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStackedLayoutBASE> for QStackedLayout {
//  fn as_ref(& self) -> & QStackedLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstackedlayout.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QStackedLayout {
  pub fn metaObject_0<RetType, T: QStackedLayout_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QStackedLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedLayout10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStackedLayout()

/*
Constructs a QStackedLayout with no parent.

This QStackedLayout must be installed on a widget later on to become effective.

See also addWidget() and insertWidget().
*/
// QStackedLayout() ctx.fn_proto_cpp
impl /*struct*/ QStackedLayout {
  pub fn QStackedLayout_0<T: QStackedLayout_QStackedLayout_0>(value: T) -> QStackedLayout {
    let rsthis = value.QStackedLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStackedLayout_QStackedLayout_0 {
  fn QStackedLayout_0(self) -> QStackedLayout;
}
// QStackedLayout() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStackedLayout_QStackedLayout_0 for () {
  fn QStackedLayout_0(self) -> QStackedLayout {
    // unsafe{_ZN14QStackedLayoutC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QStackedLayoutC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStackedLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:67
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QStackedLayout(QWidget *)

/*
Constructs a QStackedLayout with no parent.

This QStackedLayout must be installed on a widget later on to become effective.

See also addWidget() and insertWidget().
*/
// QStackedLayout(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QStackedLayout {
  pub fn QStackedLayout_1<T: QStackedLayout_QStackedLayout_1>(value: T) -> QStackedLayout {
    let rsthis = value.QStackedLayout_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStackedLayout_QStackedLayout_1 {
  fn QStackedLayout_1(self) -> QStackedLayout;
}
// QStackedLayout(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStackedLayout_QStackedLayout_1 for (usize) {
  fn QStackedLayout_1(self) -> QStackedLayout {
    // unsafe{_ZN14QStackedLayoutC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QStackedLayoutC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStackedLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:68
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QStackedLayout(QLayout *)

/*
Constructs a QStackedLayout with no parent.

This QStackedLayout must be installed on a widget later on to become effective.

See also addWidget() and insertWidget().
*/
// QStackedLayout(QLayout *) ctx.fn_proto_cpp
impl /*struct*/ QStackedLayout {
  pub fn QStackedLayout_2<T: QStackedLayout_QStackedLayout_2>(value: T) -> QStackedLayout {
    let rsthis = value.QStackedLayout_2();
    return rsthis;
    // return 1;
  }
}

pub trait QStackedLayout_QStackedLayout_2 {
  fn QStackedLayout_2(self) -> QStackedLayout;
}
// QStackedLayout(QLayout *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStackedLayout_QStackedLayout_2 for (usize) {
  fn QStackedLayout_2(self) -> QStackedLayout {
    // unsafe{_ZN14QStackedLayoutC2EP7QLayout()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QStackedLayoutC2EP7QLayout", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStackedLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QStackedLayout()

/*

*/
pub fn DeleteQStackedLayout(this :*mut QStackedLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QStackedLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qstackedlayout.h:71
// index:0
// Public Visibility=Default Availability=Available
// [4] int addWidget(QWidget *)

/*
Adds the given widget to the end of this layout and returns the index position of the widget.

If the QStackedLayout is empty before this function is called, the given widget becomes the current widget.

See also insertWidget(), removeWidget(), and setCurrentWidget().
*/
impl /*struct*/ QStackedLayout {
  pub fn addWidget_0<RetType, T: QStackedLayout_addWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addWidget_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_addWidget_0<RetType> {
  fn addWidget_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_addWidget_0<i32> for (usize) {
  fn addWidget_0(self , rsthis: & QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStackedLayout9addWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:72
// index:0
// Public Visibility=Default Availability=Available
// [4] int insertWidget(int, QWidget *)

/*
Inserts the given widget at the given index in this QStackedLayout. If index is out of range, the widget is appended (in which case it is the actual index of the widget that is returned).

If the QStackedLayout is empty before this function is called, the given widget becomes the current widget.

Inserting a new widget at an index less than or equal to the current index will increment the current index, but keep the current widget.

See also addWidget(), removeWidget(), and setCurrentWidget().
*/
impl /*struct*/ QStackedLayout {
  pub fn insertWidget_0<RetType, T: QStackedLayout_insertWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertWidget_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_insertWidget_0<RetType> {
  fn insertWidget_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_insertWidget_0<i32> for (i32,usize) {
  fn insertWidget_0(self , rsthis: & QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStackedLayout12insertWidgetEiP7QWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * currentWidget() const

/*
Returns the current widget, or 0 if there are no widgets in this layout.

See also currentIndex() and setCurrentWidget().
*/
impl /*struct*/ QStackedLayout {
  pub fn currentWidget_0<RetType, T: QStackedLayout_currentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentWidget_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_currentWidget_0<RetType> {
  fn currentWidget_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_currentWidget_0<usize> for () {
  fn currentWidget_0(self , rsthis: & QStackedLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedLayout13currentWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:75
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentIndex() const

/*

*/
impl /*struct*/ QStackedLayout {
  pub fn currentIndex_0<RetType, T: QStackedLayout_currentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIndex_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_currentIndex_0<RetType> {
  fn currentIndex_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_currentIndex_0<i32> for () {
  fn currentIndex_0(self , rsthis: & QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedLayout12currentIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:77
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widget(int) const

/*
Returns the widget at the given index, or 0 if there is no widget at the given position.

See also currentWidget() and indexOf().
*/
impl /*struct*/ QStackedLayout {
  pub fn widget_0<RetType, T: QStackedLayout_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_widget_0<RetType> {
  fn widget_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_widget_0<usize> for (i32) {
  fn widget_0(self , rsthis: & QStackedLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedLayout6widgetEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:78
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int count() const

/*

*/
impl /*struct*/ QStackedLayout {
  pub fn count_0<RetType, T: QStackedLayout_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_count_0<RetType> {
  fn count_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_count_0<i32> for () {
  fn count_0(self , rsthis: & QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedLayout5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:80
// index:0
// Public Visibility=Default Availability=Available
// [4] QStackedLayout::StackingMode stackingMode() const

/*

*/
impl /*struct*/ QStackedLayout {
  pub fn stackingMode_0<RetType, T: QStackedLayout_stackingMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stackingMode_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_stackingMode_0<RetType> {
  fn stackingMode_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_stackingMode_0<i32> for () {
  fn stackingMode_0(self , rsthis: & QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedLayout12stackingModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStackingMode(QStackedLayout::StackingMode)

/*

*/
impl /*struct*/ QStackedLayout {
  pub fn setStackingMode_0<RetType, T: QStackedLayout_setStackingMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStackingMode_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_setStackingMode_0<RetType> {
  fn setStackingMode_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_setStackingMode_0<(/*void*/)> for (i32) {
  fn setStackingMode_0(self , rsthis: & QStackedLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QStackedLayout15setStackingModeENS_12StackingModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:84
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void addItem(QLayoutItem *)

/*
Reimplemented from QLayout::addItem().
*/
impl /*struct*/ QStackedLayout {
  pub fn addItem_0<RetType, T: QStackedLayout_addItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_addItem_0<RetType> {
  fn addItem_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_addItem_0<(/*void*/)> for (usize) {
  fn addItem_0(self , rsthis: & QStackedLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QStackedLayout7addItemEP11QLayoutItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:85
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QLayoutItem::sizeHint().
*/
impl /*struct*/ QStackedLayout {
  pub fn sizeHint_0<RetType, T: QStackedLayout_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QStackedLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedLayout8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:86
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSize() const

/*
Reimplemented from QLayoutItem::minimumSize().
*/
impl /*struct*/ QStackedLayout {
  pub fn minimumSize_0<RetType, T: QStackedLayout_minimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSize_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_minimumSize_0<RetType> {
  fn minimumSize_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_minimumSize_0<usize> for () {
  fn minimumSize_0(self , rsthis: & QStackedLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedLayout11minimumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:87
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QLayoutItem * itemAt(int) const

/*
Reimplemented from QLayout::itemAt().
*/
impl /*struct*/ QStackedLayout {
  pub fn itemAt_0<RetType, T: QStackedLayout_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_itemAt_0<usize> for (i32) {
  fn itemAt_0(self , rsthis: & QStackedLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedLayout6itemAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QLayoutItem * takeAt(int)

/*
Reimplemented from QLayout::takeAt().
*/
impl /*struct*/ QStackedLayout {
  pub fn takeAt_0<RetType, T: QStackedLayout_takeAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeAt_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_takeAt_0<RetType> {
  fn takeAt_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_takeAt_0<usize> for (i32) {
  fn takeAt_0(self , rsthis: & QStackedLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStackedLayout6takeAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:89
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setGeometry(const QRect &)

/*
Reimplemented from QLayoutItem::setGeometry().
*/
impl /*struct*/ QStackedLayout {
  pub fn setGeometry_0<RetType, T: QStackedLayout_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_setGeometry_0<(/*void*/)> for (usize) {
  fn setGeometry_0(self , rsthis: & QStackedLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QStackedLayout11setGeometryERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:90
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasHeightForWidth() const

/*
Reimplemented from QLayoutItem::hasHeightForWidth().
*/
impl /*struct*/ QStackedLayout {
  pub fn hasHeightForWidth_0<RetType, T: QStackedLayout_hasHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_hasHeightForWidth_0<RetType> {
  fn hasHeightForWidth_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_hasHeightForWidth_0<bool> for () {
  fn hasHeightForWidth_0(self , rsthis: & QStackedLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedLayout17hasHeightForWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:91
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int heightForWidth(int) const

/*
Reimplemented from QLayoutItem::heightForWidth().
*/
impl /*struct*/ QStackedLayout {
  pub fn heightForWidth_0<RetType, T: QStackedLayout_heightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_heightForWidth_0<RetType> {
  fn heightForWidth_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_heightForWidth_0<i32> for (i32) {
  fn heightForWidth_0(self , rsthis: & QStackedLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStackedLayout14heightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void widgetRemoved(int)

/*
This signal is emitted whenever a widget is removed from the layout. The widget's index is passed as parameter.

See also removeWidget().
*/
impl /*struct*/ QStackedLayout {
  pub fn widgetRemoved_0<RetType, T: QStackedLayout_widgetRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widgetRemoved_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_widgetRemoved_0<RetType> {
  fn widgetRemoved_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_widgetRemoved_0<(/*void*/)> for (i32) {
  fn widgetRemoved_0(self , rsthis: & QStackedLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QStackedLayout13widgetRemovedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentChanged(int)

/*
This signal is emitted whenever the current widget in the layout changes. The index specifies the index of the new current widget, or -1 if there isn't a new one (for example, if there are no widgets in the QStackedLayout)

Note: Notifier signal for property currentIndex. 

See also currentWidget() and setCurrentWidget().
*/
impl /*struct*/ QStackedLayout {
  pub fn currentChanged_0<RetType, T: QStackedLayout_currentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentChanged_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_currentChanged_0<RetType> {
  fn currentChanged_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_currentChanged_0<(/*void*/)> for (i32) {
  fn currentChanged_0(self , rsthis: & QStackedLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QStackedLayout14currentChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentIndex(int)

/*

*/
impl /*struct*/ QStackedLayout {
  pub fn setCurrentIndex_0<RetType, T: QStackedLayout_setCurrentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_setCurrentIndex_0<RetType> {
  fn setCurrentIndex_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_setCurrentIndex_0<(/*void*/)> for (i32) {
  fn setCurrentIndex_0(self , rsthis: & QStackedLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QStackedLayout15setCurrentIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstackedlayout.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentWidget(QWidget *)

/*
Sets the current widget to be the specified widget. The new current widget must already be contained in this stacked layout.

See also setCurrentIndex() and currentWidget().
*/
impl /*struct*/ QStackedLayout {
  pub fn setCurrentWidget_0<RetType, T: QStackedLayout_setCurrentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentWidget_0(self);
    // return 1;
  }
}
pub trait QStackedLayout_setCurrentWidget_0<RetType> {
  fn setCurrentWidget_0(self , rsthis: & QStackedLayout) -> RetType;
}
impl<'a> /*trait*/ QStackedLayout_setCurrentWidget_0<(/*void*/)> for (usize) {
  fn setCurrentWidget_0(self , rsthis: & QStackedLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QStackedLayout16setCurrentWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum specifies how the layout handles its child widgets regarding their visibility.



This enum was introduced or modified in  Qt 4.4.

*/
pub type QStackedLayout__StackingMode = i32;
// Only the current widget is visible. This is the default.
pub const QStackedLayout__StackOne :QStackedLayout__StackingMode = 0;
// All widgets are visible. The current widget is merely raised.
pub const QStackedLayout__StackAll :QStackedLayout__StackingMode = 1;
pub fn QStackedLayout_StackingModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStackedLayout", val);
}
pub fn QStackedLayout_StackingModeItemName_s(val: i32) ->String {
  //var nilthis *QStackedLayout
  //return nilthis.StackingModeItemName(val);
  return QStackedLayout_StackingModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
