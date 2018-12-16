

// mod ::widgets::QToolBar
// package qtwidgets
// /usr/include/qt/QtWidgets/qtoolbar.h
// #include <qtoolbar.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 38
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

// void actionEvent(QActionEvent *)
// func (this *QToolBar) InheritActionEvent(f func(event *qtgui.QActionEvent/*777 QActionEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "actionEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QToolBar) InheritChangeEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QToolBar) InheritPaintEvent(f func(event *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// bool event(QEvent *)
// func (this *QToolBar) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void initStyleOption(QStyleOptionToolBar *)
// func (this *QToolBar) InheritInitStyleOption(f func(option *QStyleOptionToolBar/*777 QStyleOptionToolBar **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QToolBar)=48
pub struct QToolBar {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QToolBar_ITF interface {
//    QWidget_ITF
//    QToolBar_PTR() *QToolBar
//}
//func (ptr *QToolBar) QToolBar_PTR() *QToolBar { return ptr }

impl /*struct*/ QToolBar {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QToolBar {
    return QToolBar{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QToolBar {
//  type Target = QToolBarBASE;
//
//  fn deref(&self) -> &QToolBarBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QToolBarBASE> for QToolBar {
//  fn as_ref(& self) -> & QToolBarBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtoolbar.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QToolBar {
  pub fn metaObject_0<RetType, T: QToolBar_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QToolBar_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QToolBar(const QString &, QWidget *)

/*
Constructs a QToolBar with the given parent.

The given window title identifies the toolbar and is shown in the context menu provided by QMainWindow.

See also setWindowTitle().
*/
// QToolBar(const QString &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QToolBar {
  pub fn QToolBar_0<T: QToolBar_QToolBar_0>(value: T) -> QToolBar {
    let rsthis = value.QToolBar_0();
    return rsthis;
    // return 1;
  }
}

pub trait QToolBar_QToolBar_0 {
  fn QToolBar_0(self) -> QToolBar;
}
// QToolBar(const QString &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QToolBar_QToolBar_0 for (usize,usize) {
  fn QToolBar_0(self) -> QToolBar {
    // unsafe{_ZN8QToolBarC2ERK7QStringP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QToolBarC2ERK7QStringP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QToolBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:80
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QToolBar(QWidget *)

/*
Constructs a QToolBar with the given parent.

The given window title identifies the toolbar and is shown in the context menu provided by QMainWindow.

See also setWindowTitle().
*/
// QToolBar(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QToolBar {
  pub fn QToolBar_1<T: QToolBar_QToolBar_1>(value: T) -> QToolBar {
    let rsthis = value.QToolBar_1();
    return rsthis;
    // return 1;
  }
}

pub trait QToolBar_QToolBar_1 {
  fn QToolBar_1(self) -> QToolBar;
}
// QToolBar(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QToolBar_QToolBar_1 for (usize) {
  fn QToolBar_1(self) -> QToolBar {
    // unsafe{_ZN8QToolBarC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QToolBarC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QToolBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:81
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QToolBar()

/*

*/
pub fn DeleteQToolBar(this :*mut QToolBar) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QToolBarD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtoolbar.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMovable(bool)

/*

*/
impl /*struct*/ QToolBar {
  pub fn setMovable_0<RetType, T: QToolBar_setMovable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMovable_0(self);
    // return 1;
  }
}
pub trait QToolBar_setMovable_0<RetType> {
  fn setMovable_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_setMovable_0<(/*void*/)> for (bool) {
  fn setMovable_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar10setMovableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:84
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isMovable() const

/*

*/
impl /*struct*/ QToolBar {
  pub fn isMovable_0<RetType, T: QToolBar_isMovable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isMovable_0(self);
    // return 1;
  }
}
pub trait QToolBar_isMovable_0<RetType> {
  fn isMovable_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_isMovable_0<bool> for () {
  fn isMovable_0(self , rsthis: & QToolBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar9isMovableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAllowedAreas(Qt::ToolBarAreas)

/*

*/
impl /*struct*/ QToolBar {
  pub fn setAllowedAreas_0<RetType, T: QToolBar_setAllowedAreas_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAllowedAreas_0(self);
    // return 1;
  }
}
pub trait QToolBar_setAllowedAreas_0<RetType> {
  fn setAllowedAreas_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_setAllowedAreas_0<(/*void*/)> for (i32) {
  fn setAllowedAreas_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar15setAllowedAreasE6QFlagsIN2Qt11ToolBarAreaEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:87
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ToolBarAreas allowedAreas() const

/*

*/
impl /*struct*/ QToolBar {
  pub fn allowedAreas_0<RetType, T: QToolBar_allowedAreas_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.allowedAreas_0(self);
    // return 1;
  }
}
pub trait QToolBar_allowedAreas_0<RetType> {
  fn allowedAreas_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_allowedAreas_0<i32> for () {
  fn allowedAreas_0(self , rsthis: & QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar12allowedAreasEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:89
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isAreaAllowed(Qt::ToolBarArea) const

/*
Returns true if this toolbar is dockable in the given area; otherwise returns false.
*/
impl /*struct*/ QToolBar {
  pub fn isAreaAllowed_0<RetType, T: QToolBar_isAreaAllowed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAreaAllowed_0(self);
    // return 1;
  }
}
pub trait QToolBar_isAreaAllowed_0<RetType> {
  fn isAreaAllowed_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_isAreaAllowed_0<bool> for (i32) {
  fn isAreaAllowed_0(self , rsthis: & QToolBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar13isAreaAllowedEN2Qt11ToolBarAreaE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrientation(Qt::Orientation)

/*

*/
impl /*struct*/ QToolBar {
  pub fn setOrientation_0<RetType, T: QToolBar_setOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrientation_0(self);
    // return 1;
  }
}
pub trait QToolBar_setOrientation_0<RetType> {
  fn setOrientation_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_setOrientation_0<(/*void*/)> for (i32) {
  fn setOrientation_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar14setOrientationEN2Qt11OrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:93
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Orientation orientation() const

/*

*/
impl /*struct*/ QToolBar {
  pub fn orientation_0<RetType, T: QToolBar_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QToolBar_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Removes all actions from the toolbar.

See also removeAction().
*/
impl /*struct*/ QToolBar {
  pub fn clear_0<RetType, T: QToolBar_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QToolBar_clear_0<RetType> {
  fn clear_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QToolBar5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:98
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(const QString &)

/*
This is an overloaded function.

Creates a new action with the given text. This action is added to the end of the toolbar.
*/
impl /*struct*/ QToolBar {
  pub fn addAction_0<RetType, T: QToolBar_addAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_0(self);
    // return 1;
  }
}
pub trait QToolBar_addAction_0<RetType> {
  fn addAction_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_addAction_0<usize> for (usize) {
  fn addAction_0(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBar9addActionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:99
// index:1
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(const QIcon &, const QString &)

/*
This is an overloaded function.

Creates a new action with the given text. This action is added to the end of the toolbar.
*/
impl /*struct*/ QToolBar {
  pub fn addAction_1<RetType, T: QToolBar_addAction_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_1(self);
    // return 1;
  }
}
pub trait QToolBar_addAction_1<RetType> {
  fn addAction_1(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_addAction_1<usize> for (usize,usize) {
  fn addAction_1(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBar9addActionERK5QIconRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:100
// index:2
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(const QString &, const QObject *, const char *)

/*
This is an overloaded function.

Creates a new action with the given text. This action is added to the end of the toolbar.
*/
impl /*struct*/ QToolBar {
  pub fn addAction_2<RetType, T: QToolBar_addAction_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_2(self);
    // return 1;
  }
}
pub trait QToolBar_addAction_2<RetType> {
  fn addAction_2(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_addAction_2<usize> for (usize,usize,usize) {
  fn addAction_2(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBar9addActionERK7QStringPK7QObjectPKc", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:101
// index:3
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(const QIcon &, const QString &, const QObject *, const char *)

/*
This is an overloaded function.

Creates a new action with the given text. This action is added to the end of the toolbar.
*/
impl /*struct*/ QToolBar {
  pub fn addAction_3<RetType, T: QToolBar_addAction_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_3(self);
    // return 1;
  }
}
pub trait QToolBar_addAction_3<RetType> {
  fn addAction_3(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_addAction_3<usize> for (usize,usize,usize,usize) {
  fn addAction_3(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (self.3) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBar9addActionERK5QIconRK7QStringPK7QObjectPKc", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:155
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * addSeparator()

/*
Adds a separator to the end of the toolbar.

See also insertSeparator().
*/
impl /*struct*/ QToolBar {
  pub fn addSeparator_0<RetType, T: QToolBar_addSeparator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addSeparator_0(self);
    // return 1;
  }
}
pub trait QToolBar_addSeparator_0<RetType> {
  fn addSeparator_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_addSeparator_0<usize> for () {
  fn addSeparator_0(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBar12addSeparatorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:156
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * insertSeparator(QAction *)

/*
Inserts a separator into the toolbar in front of the toolbar item associated with the before action.

See also addSeparator().
*/
impl /*struct*/ QToolBar {
  pub fn insertSeparator_0<RetType, T: QToolBar_insertSeparator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertSeparator_0(self);
    // return 1;
  }
}
pub trait QToolBar_insertSeparator_0<RetType> {
  fn insertSeparator_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_insertSeparator_0<usize> for (usize) {
  fn insertSeparator_0(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBar15insertSeparatorEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:158
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * addWidget(QWidget *)

/*
Adds the given widget to the toolbar as the toolbar's last item.

The toolbar takes ownership of widget.

If you add a QToolButton with this method, the toolbar's Qt::ToolButtonStyle will not be respected.

Note: You should use QAction::setVisible() to change the visibility of the widget. Using QWidget::setVisible(), QWidget::show() and QWidget::hide() does not work.

See also insertWidget().
*/
impl /*struct*/ QToolBar {
  pub fn addWidget_0<RetType, T: QToolBar_addWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addWidget_0(self);
    // return 1;
  }
}
pub trait QToolBar_addWidget_0<RetType> {
  fn addWidget_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_addWidget_0<usize> for (usize) {
  fn addWidget_0(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBar9addWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:159
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * insertWidget(QAction *, QWidget *)

/*
Inserts the given widget in front of the toolbar item associated with the before action.

Note: You should use QAction::setVisible() to change the visibility of the widget. Using QWidget::setVisible(), QWidget::show() and QWidget::hide() does not work.

See also addWidget().
*/
impl /*struct*/ QToolBar {
  pub fn insertWidget_0<RetType, T: QToolBar_insertWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertWidget_0(self);
    // return 1;
  }
}
pub trait QToolBar_insertWidget_0<RetType> {
  fn insertWidget_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_insertWidget_0<usize> for (usize,usize) {
  fn insertWidget_0(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBar12insertWidgetEP7QActionP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:161
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect actionGeometry(QAction *) const

/*

*/
impl /*struct*/ QToolBar {
  pub fn actionGeometry_0<RetType, T: QToolBar_actionGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionGeometry_0(self);
    // return 1;
  }
}
pub trait QToolBar_actionGeometry_0<RetType> {
  fn actionGeometry_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_actionGeometry_0<usize> for (usize) {
  fn actionGeometry_0(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar14actionGeometryEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:162
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * actionAt(const QPoint &) const

/*
Returns the action at point p. This function returns zero if no action was found.

See also QWidget::childAt().
*/
impl /*struct*/ QToolBar {
  pub fn actionAt_0<RetType, T: QToolBar_actionAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionAt_0(self);
    // return 1;
  }
}
pub trait QToolBar_actionAt_0<RetType> {
  fn actionAt_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_actionAt_0<usize> for (usize) {
  fn actionAt_0(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar8actionAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:163
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QAction * actionAt(int, int) const

/*
Returns the action at point p. This function returns zero if no action was found.

See also QWidget::childAt().
*/
impl /*struct*/ QToolBar {
  pub fn actionAt_1<RetType, T: QToolBar_actionAt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionAt_1(self);
    // return 1;
  }
}
pub trait QToolBar_actionAt_1<RetType> {
  fn actionAt_1(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_actionAt_1<usize> for (i32,i32) {
  fn actionAt_1(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar8actionAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:165
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * toggleViewAction() const

/*
Returns a checkable action that can be used to show or hide this toolbar.

The action's text is set to the toolbar's window title.

See also QAction::text and QWidget::windowTitle.
*/
impl /*struct*/ QToolBar {
  pub fn toggleViewAction_0<RetType, T: QToolBar_toggleViewAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toggleViewAction_0(self);
    // return 1;
  }
}
pub trait QToolBar_toggleViewAction_0<RetType> {
  fn toggleViewAction_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_toggleViewAction_0<usize> for () {
  fn toggleViewAction_0(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar16toggleViewActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:167
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize iconSize() const

/*

*/
impl /*struct*/ QToolBar {
  pub fn iconSize_0<RetType, T: QToolBar_iconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconSize_0(self);
    // return 1;
  }
}
pub trait QToolBar_iconSize_0<RetType> {
  fn iconSize_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_iconSize_0<usize> for () {
  fn iconSize_0(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar8iconSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:168
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ToolButtonStyle toolButtonStyle() const

/*

*/
impl /*struct*/ QToolBar {
  pub fn toolButtonStyle_0<RetType, T: QToolBar_toolButtonStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolButtonStyle_0(self);
    // return 1;
  }
}
pub trait QToolBar_toolButtonStyle_0<RetType> {
  fn toolButtonStyle_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_toolButtonStyle_0<i32> for () {
  fn toolButtonStyle_0(self , rsthis: & QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar15toolButtonStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:170
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widgetForAction(QAction *) const

/*
Returns the widget associated with the specified action.

This function was introduced in  Qt 4.2.

See also addWidget().
*/
impl /*struct*/ QToolBar {
  pub fn widgetForAction_0<RetType, T: QToolBar_widgetForAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widgetForAction_0(self);
    // return 1;
  }
}
pub trait QToolBar_widgetForAction_0<RetType> {
  fn widgetForAction_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_widgetForAction_0<usize> for (usize) {
  fn widgetForAction_0(self , rsthis: & QToolBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar15widgetForActionEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:172
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFloatable() const

/*

*/
impl /*struct*/ QToolBar {
  pub fn isFloatable_0<RetType, T: QToolBar_isFloatable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFloatable_0(self);
    // return 1;
  }
}
pub trait QToolBar_isFloatable_0<RetType> {
  fn isFloatable_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_isFloatable_0<bool> for () {
  fn isFloatable_0(self , rsthis: & QToolBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar11isFloatableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:173
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFloatable(bool)

/*

*/
impl /*struct*/ QToolBar {
  pub fn setFloatable_0<RetType, T: QToolBar_setFloatable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFloatable_0(self);
    // return 1;
  }
}
pub trait QToolBar_setFloatable_0<RetType> {
  fn setFloatable_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_setFloatable_0<(/*void*/)> for (bool) {
  fn setFloatable_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar12setFloatableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:174
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFloating() const

/*

*/
impl /*struct*/ QToolBar {
  pub fn isFloating_0<RetType, T: QToolBar_isFloating_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFloating_0(self);
    // return 1;
  }
}
pub trait QToolBar_isFloating_0<RetType> {
  fn isFloating_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_isFloating_0<bool> for () {
  fn isFloating_0(self , rsthis: & QToolBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QToolBar10isFloatingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:177
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIconSize(const QSize &)

/*

*/
impl /*struct*/ QToolBar {
  pub fn setIconSize_0<RetType, T: QToolBar_setIconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIconSize_0(self);
    // return 1;
  }
}
pub trait QToolBar_setIconSize_0<RetType> {
  fn setIconSize_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_setIconSize_0<(/*void*/)> for (usize) {
  fn setIconSize_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar11setIconSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setToolButtonStyle(Qt::ToolButtonStyle)

/*

*/
impl /*struct*/ QToolBar {
  pub fn setToolButtonStyle_0<RetType, T: QToolBar_setToolButtonStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolButtonStyle_0(self);
    // return 1;
  }
}
pub trait QToolBar_setToolButtonStyle_0<RetType> {
  fn setToolButtonStyle_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_setToolButtonStyle_0<(/*void*/)> for (i32) {
  fn setToolButtonStyle_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar18setToolButtonStyleEN2Qt15ToolButtonStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:181
// index:0
// Public Visibility=Default Availability=Available
// [-2] void actionTriggered(QAction *)

/*
This signal is emitted when an action in this toolbar is triggered. This happens when the action's tool button is pressed, or when the action is triggered in some other way outside the toolbar. The parameter holds the triggered action.
*/
impl /*struct*/ QToolBar {
  pub fn actionTriggered_0<RetType, T: QToolBar_actionTriggered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionTriggered_0(self);
    // return 1;
  }
}
pub trait QToolBar_actionTriggered_0<RetType> {
  fn actionTriggered_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_actionTriggered_0<(/*void*/)> for (usize) {
  fn actionTriggered_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar15actionTriggeredEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:182
// index:0
// Public Visibility=Default Availability=Available
// [-2] void movableChanged(bool)

/*
This signal is emitted when the toolbar becomes movable or fixed. If the toolbar can be moved, movable is true; otherwise it is false.

Note: Notifier signal for property movable. 

See also movable.
*/
impl /*struct*/ QToolBar {
  pub fn movableChanged_0<RetType, T: QToolBar_movableChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.movableChanged_0(self);
    // return 1;
  }
}
pub trait QToolBar_movableChanged_0<RetType> {
  fn movableChanged_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_movableChanged_0<(/*void*/)> for (bool) {
  fn movableChanged_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar14movableChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:183
// index:0
// Public Visibility=Default Availability=Available
// [-2] void allowedAreasChanged(Qt::ToolBarAreas)

/*
This signal is emitted when the collection of allowed areas for the toolbar is changed. The new areas in which the toolbar can be positioned are specified by allowedAreas.

Note: Notifier signal for property allowedAreas. 

See also allowedAreas.
*/
impl /*struct*/ QToolBar {
  pub fn allowedAreasChanged_0<RetType, T: QToolBar_allowedAreasChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.allowedAreasChanged_0(self);
    // return 1;
  }
}
pub trait QToolBar_allowedAreasChanged_0<RetType> {
  fn allowedAreasChanged_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_allowedAreasChanged_0<(/*void*/)> for (i32) {
  fn allowedAreasChanged_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar19allowedAreasChangedE6QFlagsIN2Qt11ToolBarAreaEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:184
// index:0
// Public Visibility=Default Availability=Available
// [-2] void orientationChanged(Qt::Orientation)

/*
This signal is emitted when the orientation of the toolbar changes. The orientation parameter holds the toolbar's new orientation.

Note: Notifier signal for property orientation. 

See also orientation.
*/
impl /*struct*/ QToolBar {
  pub fn orientationChanged_0<RetType, T: QToolBar_orientationChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientationChanged_0(self);
    // return 1;
  }
}
pub trait QToolBar_orientationChanged_0<RetType> {
  fn orientationChanged_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_orientationChanged_0<(/*void*/)> for (i32) {
  fn orientationChanged_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar18orientationChangedEN2Qt11OrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:185
// index:0
// Public Visibility=Default Availability=Available
// [-2] void iconSizeChanged(const QSize &)

/*
This signal is emitted when the icon size is changed. The iconSize parameter holds the toolbar's new icon size.

Note: Notifier signal for property iconSize. 

See also iconSize and QMainWindow::iconSize.
*/
impl /*struct*/ QToolBar {
  pub fn iconSizeChanged_0<RetType, T: QToolBar_iconSizeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconSizeChanged_0(self);
    // return 1;
  }
}
pub trait QToolBar_iconSizeChanged_0<RetType> {
  fn iconSizeChanged_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_iconSizeChanged_0<(/*void*/)> for (usize) {
  fn iconSizeChanged_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar15iconSizeChangedERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:186
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toolButtonStyleChanged(Qt::ToolButtonStyle)

/*
This signal is emitted when the tool button style is changed. The toolButtonStyle parameter holds the toolbar's new tool button style.

Note: Notifier signal for property toolButtonStyle. 

See also toolButtonStyle and QMainWindow::toolButtonStyle.
*/
impl /*struct*/ QToolBar {
  pub fn toolButtonStyleChanged_0<RetType, T: QToolBar_toolButtonStyleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolButtonStyleChanged_0(self);
    // return 1;
  }
}
pub trait QToolBar_toolButtonStyleChanged_0<RetType> {
  fn toolButtonStyleChanged_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_toolButtonStyleChanged_0<(/*void*/)> for (i32) {
  fn toolButtonStyleChanged_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar22toolButtonStyleChangedEN2Qt15ToolButtonStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void topLevelChanged(bool)

/*
This signal is emitted when the floating property changes. The topLevel parameter is true if the toolbar is now floating; otherwise it is false.

This function was introduced in  Qt 4.6.

See also isWindow().
*/
impl /*struct*/ QToolBar {
  pub fn topLevelChanged_0<RetType, T: QToolBar_topLevelChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topLevelChanged_0(self);
    // return 1;
  }
}
pub trait QToolBar_topLevelChanged_0<RetType> {
  fn topLevelChanged_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_topLevelChanged_0<(/*void*/)> for (bool) {
  fn topLevelChanged_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar15topLevelChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:188
// index:0
// Public Visibility=Default Availability=Available
// [-2] void visibilityChanged(bool)

/*
This signal is emitted when the toolbar becomes visible (or invisible). This happens when the widget is hidden or shown.

This function was introduced in  Qt 4.7.
*/
impl /*struct*/ QToolBar {
  pub fn visibilityChanged_0<RetType, T: QToolBar_visibilityChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visibilityChanged_0(self);
    // return 1;
  }
}
pub trait QToolBar_visibilityChanged_0<RetType> {
  fn visibilityChanged_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_visibilityChanged_0<(/*void*/)> for (bool) {
  fn visibilityChanged_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar17visibilityChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:191
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void actionEvent(QActionEvent *)

/*
Reimplemented from QWidget::actionEvent().
*/
impl /*struct*/ QToolBar {
  pub fn actionEvent_0<RetType, T: QToolBar_actionEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionEvent_0(self);
    // return 1;
  }
}
pub trait QToolBar_actionEvent_0<RetType> {
  fn actionEvent_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_actionEvent_0<(/*void*/)> for (usize) {
  fn actionEvent_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar11actionEventEP12QActionEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:192
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QToolBar {
  pub fn changeEvent_0<RetType, T: QToolBar_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QToolBar_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:193
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QToolBar {
  pub fn paintEvent_0<RetType, T: QToolBar_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QToolBar_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolBar10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:194
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QToolBar {
  pub fn event_0<RetType, T: QToolBar_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QToolBar_event_0<RetType> {
  fn event_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QToolBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolBar5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbar.h:195
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionToolBar *) const

/*

*/
impl /*struct*/ QToolBar {
  pub fn initStyleOption_0<RetType, T: QToolBar_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QToolBar_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QToolBar) -> RetType;
}
impl<'a> /*trait*/ QToolBar_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QToolBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK8QToolBar15initStyleOptionEP19QStyleOptionToolBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
