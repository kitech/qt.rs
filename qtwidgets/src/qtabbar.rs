

// mod ::widgets::QTabBar
// package qtwidgets
// /usr/include/qt/QtWidgets/qtabbar.h
// #include <qtabbar.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 34
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

// QSize tabSizeHint(int)
// func (this *QTabBar) InheritTabSizeHint(f func(index int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "tabSizeHint", f)
// }

// QSize minimumTabSizeHint(int)
// func (this *QTabBar) InheritMinimumTabSizeHint(f func(index int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "minimumTabSizeHint", f)
// }

// void tabInserted(int)
// func (this *QTabBar) InheritTabInserted(f func(index int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "tabInserted", f)
// }

// void tabRemoved(int)
// func (this *QTabBar) InheritTabRemoved(f func(index int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "tabRemoved", f)
// }

// void tabLayoutChange()
// func (this *QTabBar) InheritTabLayoutChange(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "tabLayoutChange", f)
// }

// bool event(QEvent *)
// func (this *QTabBar) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QTabBar) InheritResizeEvent(f func(arg0 *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QTabBar) InheritShowEvent(f func(arg0 *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void hideEvent(QHideEvent *)
// func (this *QTabBar) InheritHideEvent(f func(arg0 *qtgui.QHideEvent/*777 QHideEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hideEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QTabBar) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QTabBar) InheritMousePressEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QTabBar) InheritMouseMoveEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QTabBar) InheritMouseReleaseEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void wheelEvent(QWheelEvent *)
// func (this *QTabBar) InheritWheelEvent(f func(event *qtgui.QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QTabBar) InheritKeyPressEvent(f func(arg0 *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QTabBar) InheritChangeEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QTabBar) InheritTimerEvent(f func(event *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// void initStyleOption(QStyleOptionTab *, int)
// func (this *QTabBar) InheritInitStyleOption(f func(option *QStyleOptionTab/*777 QStyleOptionTab **/, tabIndex int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTabBar)=48
pub struct QTabBar {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTabBar_ITF interface {
//    QWidget_ITF
//    QTabBar_PTR() *QTabBar
//}
//func (ptr *QTabBar) QTabBar_PTR() *QTabBar { return ptr }

impl /*struct*/ QTabBar {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTabBar {
    return QTabBar{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTabBar {
//  type Target = QTabBarBASE;
//
//  fn deref(&self) -> &QTabBarBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTabBarBASE> for QTabBar {
//  fn as_ref(& self) -> & QTabBarBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtabbar.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn metaObject_0<RetType, T: QTabBar_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTabBar_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTabBar(QWidget *)

/*
Creates a new tab bar with the given parent.
*/
// QTabBar(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QTabBar {
  pub fn QTabBar_0<T: QTabBar_QTabBar_0>(value: T) -> QTabBar {
    let rsthis = value.QTabBar_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTabBar_QTabBar_0 {
  fn QTabBar_0(self) -> QTabBar;
}
// QTabBar(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTabBar_QTabBar_0 for (usize) {
  fn QTabBar_0(self) -> QTabBar {
    // unsafe{_ZN7QTabBarC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QTabBarC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTabBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:75
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTabBar()

/*

*/
pub fn DeleteQTabBar(this :*mut QTabBar) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QTabBarD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtabbar.h:93
// index:0
// Public Visibility=Default Availability=Available
// [4] QTabBar::Shape shape() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn shape_0<RetType, T: QTabBar_shape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shape_0(self);
    // return 1;
  }
}
pub trait QTabBar_shape_0<RetType> {
  fn shape_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_shape_0<i32> for () {
  fn shape_0(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar5shapeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setShape(QTabBar::Shape)

/*

*/
impl /*struct*/ QTabBar {
  pub fn setShape_0<RetType, T: QTabBar_setShape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setShape_0(self);
    // return 1;
  }
}
pub trait QTabBar_setShape_0<RetType> {
  fn setShape_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setShape_0<(/*void*/)> for (i32) {
  fn setShape_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar8setShapeENS_5ShapeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:96
// index:0
// Public Visibility=Default Availability=Available
// [4] int addTab(const QString &)

/*
Adds a new tab with text text. Returns the new tab's index.
*/
impl /*struct*/ QTabBar {
  pub fn addTab_0<RetType, T: QTabBar_addTab_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addTab_0(self);
    // return 1;
  }
}
pub trait QTabBar_addTab_0<RetType> {
  fn addTab_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_addTab_0<i32> for (usize) {
  fn addTab_0(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QTabBar6addTabERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:97
// index:1
// Public Visibility=Default Availability=Available
// [4] int addTab(const QIcon &, const QString &)

/*
Adds a new tab with text text. Returns the new tab's index.
*/
impl /*struct*/ QTabBar {
  pub fn addTab_1<RetType, T: QTabBar_addTab_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addTab_1(self);
    // return 1;
  }
}
pub trait QTabBar_addTab_1<RetType> {
  fn addTab_1(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_addTab_1<i32> for (usize,usize) {
  fn addTab_1(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QTabBar6addTabERK5QIconRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] int insertTab(int, const QString &)

/*
Inserts a new tab with text text at position index. If index is out of range, the new tab is appened. Returns the new tab's index.
*/
impl /*struct*/ QTabBar {
  pub fn insertTab_0<RetType, T: QTabBar_insertTab_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertTab_0(self);
    // return 1;
  }
}
pub trait QTabBar_insertTab_0<RetType> {
  fn insertTab_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_insertTab_0<i32> for (i32,usize) {
  fn insertTab_0(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QTabBar9insertTabEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:100
// index:1
// Public Visibility=Default Availability=Available
// [4] int insertTab(int, const QIcon &, const QString &)

/*
Inserts a new tab with text text at position index. If index is out of range, the new tab is appened. Returns the new tab's index.
*/
impl /*struct*/ QTabBar {
  pub fn insertTab_1<RetType, T: QTabBar_insertTab_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertTab_1(self);
    // return 1;
  }
}
pub trait QTabBar_insertTab_1<RetType> {
  fn insertTab_1(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_insertTab_1<i32> for (i32,usize,usize) {
  fn insertTab_1(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QTabBar9insertTabEiRK5QIconRK7QString", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeTab(int)

/*
Removes the tab at position index.

See also SelectionBehavior.
*/
impl /*struct*/ QTabBar {
  pub fn removeTab_0<RetType, T: QTabBar_removeTab_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeTab_0(self);
    // return 1;
  }
}
pub trait QTabBar_removeTab_0<RetType> {
  fn removeTab_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_removeTab_0<(/*void*/)> for (i32) {
  fn removeTab_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar9removeTabEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void moveTab(int, int)

/*
Moves the item at index position from to index position to.

This function was introduced in  Qt 4.5.

See also tabMoved() and tabLayoutChange().
*/
impl /*struct*/ QTabBar {
  pub fn moveTab_0<RetType, T: QTabBar_moveTab_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveTab_0(self);
    // return 1;
  }
}
pub trait QTabBar_moveTab_0<RetType> {
  fn moveTab_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_moveTab_0<(/*void*/)> for (i32,i32) {
  fn moveTab_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar7moveTabEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:105
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isTabEnabled(int) const

/*
Returns true if the tab at position index is enabled; otherwise returns false.
*/
impl /*struct*/ QTabBar {
  pub fn isTabEnabled_0<RetType, T: QTabBar_isTabEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTabEnabled_0(self);
    // return 1;
  }
}
pub trait QTabBar_isTabEnabled_0<RetType> {
  fn isTabEnabled_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_isTabEnabled_0<bool> for (i32) {
  fn isTabEnabled_0(self , rsthis: & QTabBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar12isTabEnabledEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabEnabled(int, bool)

/*
If enabled is true then the tab at position index is enabled; otherwise the item at position index is disabled.

See also isTabEnabled().
*/
impl /*struct*/ QTabBar {
  pub fn setTabEnabled_0<RetType, T: QTabBar_setTabEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabEnabled_0(self);
    // return 1;
  }
}
pub trait QTabBar_setTabEnabled_0<RetType> {
  fn setTabEnabled_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setTabEnabled_0<(/*void*/)> for (i32,bool) {
  fn setTabEnabled_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar13setTabEnabledEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:108
// index:0
// Public Visibility=Default Availability=Available
// [8] QString tabText(int) const

/*
Returns the text of the tab at position index, or an empty string if index is out of range.

See also setTabText().
*/
impl /*struct*/ QTabBar {
  pub fn tabText_0<RetType, T: QTabBar_tabText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabText_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabText_0<RetType> {
  fn tabText_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabText_0<usize> for (i32) {
  fn tabText_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar7tabTextEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabText(int, const QString &)

/*
Sets the text of the tab at position index to text.

See also tabText().
*/
impl /*struct*/ QTabBar {
  pub fn setTabText_0<RetType, T: QTabBar_setTabText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabText_0(self);
    // return 1;
  }
}
pub trait QTabBar_setTabText_0<RetType> {
  fn setTabText_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setTabText_0<(/*void*/)> for (i32,usize) {
  fn setTabText_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar10setTabTextEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:111
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor tabTextColor(int) const

/*
Returns the text color of the tab with the given index, or a invalid color if index is out of range.

See also setTabTextColor().
*/
impl /*struct*/ QTabBar {
  pub fn tabTextColor_0<RetType, T: QTabBar_tabTextColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabTextColor_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabTextColor_0<RetType> {
  fn tabTextColor_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabTextColor_0<usize> for (i32) {
  fn tabTextColor_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar12tabTextColorEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabTextColor(int, const QColor &)

/*
Sets the color of the text in the tab with the given index to the specified color.

If an invalid color is specified, the tab will use the QTabBar foreground role instead.

See also tabTextColor().
*/
impl /*struct*/ QTabBar {
  pub fn setTabTextColor_0<RetType, T: QTabBar_setTabTextColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabTextColor_0(self);
    // return 1;
  }
}
pub trait QTabBar_setTabTextColor_0<RetType> {
  fn setTabTextColor_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setTabTextColor_0<(/*void*/)> for (i32,usize) {
  fn setTabTextColor_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar15setTabTextColorEiRK6QColor", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:114
// index:0
// Public Visibility=Default Availability=Available
// [8] QIcon tabIcon(int) const

/*
Returns the icon of the tab at position index, or a null icon if index is out of range.

See also setTabIcon().
*/
impl /*struct*/ QTabBar {
  pub fn tabIcon_0<RetType, T: QTabBar_tabIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabIcon_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabIcon_0<RetType> {
  fn tabIcon_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabIcon_0<usize> for (i32) {
  fn tabIcon_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar7tabIconEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabIcon(int, const QIcon &)

/*
Sets the icon of the tab at position index to icon.

See also tabIcon().
*/
impl /*struct*/ QTabBar {
  pub fn setTabIcon_0<RetType, T: QTabBar_setTabIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabIcon_0(self);
    // return 1;
  }
}
pub trait QTabBar_setTabIcon_0<RetType> {
  fn setTabIcon_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setTabIcon_0<(/*void*/)> for (i32,usize) {
  fn setTabIcon_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar10setTabIconEiRK5QIcon", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:117
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TextElideMode elideMode() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn elideMode_0<RetType, T: QTabBar_elideMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.elideMode_0(self);
    // return 1;
  }
}
pub trait QTabBar_elideMode_0<RetType> {
  fn elideMode_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_elideMode_0<i32> for () {
  fn elideMode_0(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar9elideModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setElideMode(Qt::TextElideMode)

/*

*/
impl /*struct*/ QTabBar {
  pub fn setElideMode_0<RetType, T: QTabBar_setElideMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setElideMode_0(self);
    // return 1;
  }
}
pub trait QTabBar_setElideMode_0<RetType> {
  fn setElideMode_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setElideMode_0<(/*void*/)> for (i32) {
  fn setElideMode_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar12setElideModeEN2Qt13TextElideModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabToolTip(int, const QString &)

/*
Sets the tool tip of the tab at position index to tip.

See also tabToolTip().
*/
impl /*struct*/ QTabBar {
  pub fn setTabToolTip_0<RetType, T: QTabBar_setTabToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabToolTip_0(self);
    // return 1;
  }
}
pub trait QTabBar_setTabToolTip_0<RetType> {
  fn setTabToolTip_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setTabToolTip_0<(/*void*/)> for (i32,usize) {
  fn setTabToolTip_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar13setTabToolTipEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:122
// index:0
// Public Visibility=Default Availability=Available
// [8] QString tabToolTip(int) const

/*
Returns the tool tip of the tab at position index, or an empty string if index is out of range.

See also setTabToolTip().
*/
impl /*struct*/ QTabBar {
  pub fn tabToolTip_0<RetType, T: QTabBar_tabToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabToolTip_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabToolTip_0<RetType> {
  fn tabToolTip_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabToolTip_0<usize> for (i32) {
  fn tabToolTip_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar10tabToolTipEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabWhatsThis(int, const QString &)

/*
Sets the What's This help text of the tab at position index to text.

This function was introduced in  Qt 4.1.

See also tabWhatsThis().
*/
impl /*struct*/ QTabBar {
  pub fn setTabWhatsThis_0<RetType, T: QTabBar_setTabWhatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabWhatsThis_0(self);
    // return 1;
  }
}
pub trait QTabBar_setTabWhatsThis_0<RetType> {
  fn setTabWhatsThis_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setTabWhatsThis_0<(/*void*/)> for (i32,usize) {
  fn setTabWhatsThis_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar15setTabWhatsThisEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:127
// index:0
// Public Visibility=Default Availability=Available
// [8] QString tabWhatsThis(int) const

/*
Returns the What's This help text of the tab at position index, or an empty string if index is out of range.

This function was introduced in  Qt 4.1.

See also setTabWhatsThis().
*/
impl /*struct*/ QTabBar {
  pub fn tabWhatsThis_0<RetType, T: QTabBar_tabWhatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabWhatsThis_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabWhatsThis_0<RetType> {
  fn tabWhatsThis_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabWhatsThis_0<usize> for (i32) {
  fn tabWhatsThis_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar12tabWhatsThisEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabData(int, const QVariant &)

/*
Sets the data of the tab at position index to data.

See also tabData().
*/
impl /*struct*/ QTabBar {
  pub fn setTabData_0<RetType, T: QTabBar_setTabData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabData_0(self);
    // return 1;
  }
}
pub trait QTabBar_setTabData_0<RetType> {
  fn setTabData_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setTabData_0<(/*void*/)> for (i32,usize) {
  fn setTabData_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar10setTabDataEiRK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:131
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant tabData(int) const

/*
Returns the data of the tab at position index, or a null variant if index is out of range.

See also setTabData().
*/
impl /*struct*/ QTabBar {
  pub fn tabData_0<RetType, T: QTabBar_tabData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabData_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabData_0<RetType> {
  fn tabData_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabData_0<usize> for (i32) {
  fn tabData_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar7tabDataEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:133
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect tabRect(int) const

/*
Returns the visual rectangle of the tab at position index, or a null rectangle if index is out of range.
*/
impl /*struct*/ QTabBar {
  pub fn tabRect_0<RetType, T: QTabBar_tabRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabRect_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabRect_0<RetType> {
  fn tabRect_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabRect_0<usize> for (i32) {
  fn tabRect_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar7tabRectEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:134
// index:0
// Public Visibility=Default Availability=Available
// [4] int tabAt(const QPoint &) const

/*
Returns the index of the tab that covers position or -1 if no tab covers position;

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QTabBar {
  pub fn tabAt_0<RetType, T: QTabBar_tabAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabAt_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabAt_0<RetType> {
  fn tabAt_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabAt_0<i32> for (usize) {
  fn tabAt_0(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar5tabAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:136
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentIndex() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn currentIndex_0<RetType, T: QTabBar_currentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIndex_0(self);
    // return 1;
  }
}
pub trait QTabBar_currentIndex_0<RetType> {
  fn currentIndex_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_currentIndex_0<i32> for () {
  fn currentIndex_0(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar12currentIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:137
// index:0
// Public Visibility=Default Availability=Available
// [4] int count() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn count_0<RetType, T: QTabBar_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QTabBar_count_0<RetType> {
  fn count_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_count_0<i32> for () {
  fn count_0(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:139
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QTabBar {
  pub fn sizeHint_0<RetType, T: QTabBar_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QTabBar_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:140
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QTabBar {
  pub fn minimumSizeHint_0<RetType, T: QTabBar_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QTabBar_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:142
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDrawBase(bool)

/*

*/
impl /*struct*/ QTabBar {
  pub fn setDrawBase_0<RetType, T: QTabBar_setDrawBase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDrawBase_0(self);
    // return 1;
  }
}
pub trait QTabBar_setDrawBase_0<RetType> {
  fn setDrawBase_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setDrawBase_0<(/*void*/)> for (bool) {
  fn setDrawBase_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar11setDrawBaseEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:143
// index:0
// Public Visibility=Default Availability=Available
// [1] bool drawBase() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn drawBase_0<RetType, T: QTabBar_drawBase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawBase_0(self);
    // return 1;
  }
}
pub trait QTabBar_drawBase_0<RetType> {
  fn drawBase_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_drawBase_0<bool> for () {
  fn drawBase_0(self , rsthis: & QTabBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar8drawBaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:145
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize iconSize() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn iconSize_0<RetType, T: QTabBar_iconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconSize_0(self);
    // return 1;
  }
}
pub trait QTabBar_iconSize_0<RetType> {
  fn iconSize_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_iconSize_0<usize> for () {
  fn iconSize_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar8iconSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIconSize(const QSize &)

/*

*/
impl /*struct*/ QTabBar {
  pub fn setIconSize_0<RetType, T: QTabBar_setIconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIconSize_0(self);
    // return 1;
  }
}
pub trait QTabBar_setIconSize_0<RetType> {
  fn setIconSize_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setIconSize_0<(/*void*/)> for (usize) {
  fn setIconSize_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar11setIconSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:148
// index:0
// Public Visibility=Default Availability=Available
// [1] bool usesScrollButtons() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn usesScrollButtons_0<RetType, T: QTabBar_usesScrollButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.usesScrollButtons_0(self);
    // return 1;
  }
}
pub trait QTabBar_usesScrollButtons_0<RetType> {
  fn usesScrollButtons_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_usesScrollButtons_0<bool> for () {
  fn usesScrollButtons_0(self , rsthis: & QTabBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar17usesScrollButtonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:149
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUsesScrollButtons(bool)

/*

*/
impl /*struct*/ QTabBar {
  pub fn setUsesScrollButtons_0<RetType, T: QTabBar_setUsesScrollButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUsesScrollButtons_0(self);
    // return 1;
  }
}
pub trait QTabBar_setUsesScrollButtons_0<RetType> {
  fn setUsesScrollButtons_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setUsesScrollButtons_0<(/*void*/)> for (bool) {
  fn setUsesScrollButtons_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar20setUsesScrollButtonsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:151
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tabsClosable() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn tabsClosable_0<RetType, T: QTabBar_tabsClosable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabsClosable_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabsClosable_0<RetType> {
  fn tabsClosable_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabsClosable_0<bool> for () {
  fn tabsClosable_0(self , rsthis: & QTabBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar12tabsClosableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabsClosable(bool)

/*

*/
impl /*struct*/ QTabBar {
  pub fn setTabsClosable_0<RetType, T: QTabBar_setTabsClosable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabsClosable_0(self);
    // return 1;
  }
}
pub trait QTabBar_setTabsClosable_0<RetType> {
  fn setTabsClosable_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setTabsClosable_0<(/*void*/)> for (bool) {
  fn setTabsClosable_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar15setTabsClosableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:154
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabButton(int, QTabBar::ButtonPosition, QWidget *)

/*
Sets widget on the tab index. The widget is placed on the left or right hand side depending upon the position.

Any previously set widget in position is hidden.

The tab bar will take ownership of the widget and so all widgets set here will be deleted by the tab bar when it is destroyed unless you separately reparent the widget after setting some other widget (or 0).

This function was introduced in  Qt 4.5.

See also tabButton() and tabsClosable().
*/
impl /*struct*/ QTabBar {
  pub fn setTabButton_0<RetType, T: QTabBar_setTabButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabButton_0(self);
    // return 1;
  }
}
pub trait QTabBar_setTabButton_0<RetType> {
  fn setTabButton_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setTabButton_0<(/*void*/)> for (i32,i32,usize) {
  fn setTabButton_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar12setTabButtonEiNS_14ButtonPositionEP7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:155
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * tabButton(int, QTabBar::ButtonPosition) const

/*
Returns the widget set a tab index and position or 0 if one is not set.

See also setTabButton().
*/
impl /*struct*/ QTabBar {
  pub fn tabButton_0<RetType, T: QTabBar_tabButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabButton_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabButton_0<RetType> {
  fn tabButton_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabButton_0<usize> for (i32,i32) {
  fn tabButton_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar9tabButtonEiNS_14ButtonPositionE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:157
// index:0
// Public Visibility=Default Availability=Available
// [4] QTabBar::SelectionBehavior selectionBehaviorOnRemove() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn selectionBehaviorOnRemove_0<RetType, T: QTabBar_selectionBehaviorOnRemove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionBehaviorOnRemove_0(self);
    // return 1;
  }
}
pub trait QTabBar_selectionBehaviorOnRemove_0<RetType> {
  fn selectionBehaviorOnRemove_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_selectionBehaviorOnRemove_0<i32> for () {
  fn selectionBehaviorOnRemove_0(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar25selectionBehaviorOnRemoveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSelectionBehaviorOnRemove(QTabBar::SelectionBehavior)

/*

*/
impl /*struct*/ QTabBar {
  pub fn setSelectionBehaviorOnRemove_0<RetType, T: QTabBar_setSelectionBehaviorOnRemove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionBehaviorOnRemove_0(self);
    // return 1;
  }
}
pub trait QTabBar_setSelectionBehaviorOnRemove_0<RetType> {
  fn setSelectionBehaviorOnRemove_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setSelectionBehaviorOnRemove_0<(/*void*/)> for (i32) {
  fn setSelectionBehaviorOnRemove_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar28setSelectionBehaviorOnRemoveENS_17SelectionBehaviorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:160
// index:0
// Public Visibility=Default Availability=Available
// [1] bool expanding() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn expanding_0<RetType, T: QTabBar_expanding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expanding_0(self);
    // return 1;
  }
}
pub trait QTabBar_expanding_0<RetType> {
  fn expanding_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_expanding_0<bool> for () {
  fn expanding_0(self , rsthis: & QTabBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar9expandingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:161
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setExpanding(bool)

/*

*/
impl /*struct*/ QTabBar {
  pub fn setExpanding_0<RetType, T: QTabBar_setExpanding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExpanding_0(self);
    // return 1;
  }
}
pub trait QTabBar_setExpanding_0<RetType> {
  fn setExpanding_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setExpanding_0<(/*void*/)> for (bool) {
  fn setExpanding_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar12setExpandingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:163
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isMovable() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn isMovable_0<RetType, T: QTabBar_isMovable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isMovable_0(self);
    // return 1;
  }
}
pub trait QTabBar_isMovable_0<RetType> {
  fn isMovable_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_isMovable_0<bool> for () {
  fn isMovable_0(self , rsthis: & QTabBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar9isMovableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:164
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMovable(bool)

/*

*/
impl /*struct*/ QTabBar {
  pub fn setMovable_0<RetType, T: QTabBar_setMovable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMovable_0(self);
    // return 1;
  }
}
pub trait QTabBar_setMovable_0<RetType> {
  fn setMovable_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setMovable_0<(/*void*/)> for (bool) {
  fn setMovable_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar10setMovableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:166
// index:0
// Public Visibility=Default Availability=Available
// [1] bool documentMode() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn documentMode_0<RetType, T: QTabBar_documentMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentMode_0(self);
    // return 1;
  }
}
pub trait QTabBar_documentMode_0<RetType> {
  fn documentMode_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_documentMode_0<bool> for () {
  fn documentMode_0(self , rsthis: & QTabBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar12documentModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:167
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDocumentMode(bool)

/*

*/
impl /*struct*/ QTabBar {
  pub fn setDocumentMode_0<RetType, T: QTabBar_setDocumentMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDocumentMode_0(self);
    // return 1;
  }
}
pub trait QTabBar_setDocumentMode_0<RetType> {
  fn setDocumentMode_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setDocumentMode_0<(/*void*/)> for (bool) {
  fn setDocumentMode_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar15setDocumentModeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:169
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoHide() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn autoHide_0<RetType, T: QTabBar_autoHide_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoHide_0(self);
    // return 1;
  }
}
pub trait QTabBar_autoHide_0<RetType> {
  fn autoHide_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_autoHide_0<bool> for () {
  fn autoHide_0(self , rsthis: & QTabBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar8autoHideEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:170
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoHide(bool)

/*

*/
impl /*struct*/ QTabBar {
  pub fn setAutoHide_0<RetType, T: QTabBar_setAutoHide_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoHide_0(self);
    // return 1;
  }
}
pub trait QTabBar_setAutoHide_0<RetType> {
  fn setAutoHide_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setAutoHide_0<(/*void*/)> for (bool) {
  fn setAutoHide_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar11setAutoHideEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:172
// index:0
// Public Visibility=Default Availability=Available
// [1] bool changeCurrentOnDrag() const

/*

*/
impl /*struct*/ QTabBar {
  pub fn changeCurrentOnDrag_0<RetType, T: QTabBar_changeCurrentOnDrag_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeCurrentOnDrag_0(self);
    // return 1;
  }
}
pub trait QTabBar_changeCurrentOnDrag_0<RetType> {
  fn changeCurrentOnDrag_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_changeCurrentOnDrag_0<bool> for () {
  fn changeCurrentOnDrag_0(self , rsthis: & QTabBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar19changeCurrentOnDragEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:173
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setChangeCurrentOnDrag(bool)

/*

*/
impl /*struct*/ QTabBar {
  pub fn setChangeCurrentOnDrag_0<RetType, T: QTabBar_setChangeCurrentOnDrag_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setChangeCurrentOnDrag_0(self);
    // return 1;
  }
}
pub trait QTabBar_setChangeCurrentOnDrag_0<RetType> {
  fn setChangeCurrentOnDrag_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setChangeCurrentOnDrag_0<(/*void*/)> for (bool) {
  fn setChangeCurrentOnDrag_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar22setChangeCurrentOnDragEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:176
// index:0
// Public Visibility=Default Availability=Available
// [8] QString accessibleTabName(int) const

/*
Returns the accessibleName of the tab at position index, or an empty string if index is out of range.

See also setAccessibleTabName().
*/
impl /*struct*/ QTabBar {
  pub fn accessibleTabName_0<RetType, T: QTabBar_accessibleTabName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accessibleTabName_0(self);
    // return 1;
  }
}
pub trait QTabBar_accessibleTabName_0<RetType> {
  fn accessibleTabName_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_accessibleTabName_0<usize> for (i32) {
  fn accessibleTabName_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar17accessibleTabNameEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:177
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAccessibleTabName(int, const QString &)

/*
Sets the accessibleName of the tab at position index to name.

See also accessibleTabName().
*/
impl /*struct*/ QTabBar {
  pub fn setAccessibleTabName_0<RetType, T: QTabBar_setAccessibleTabName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAccessibleTabName_0(self);
    // return 1;
  }
}
pub trait QTabBar_setAccessibleTabName_0<RetType> {
  fn setAccessibleTabName_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setAccessibleTabName_0<(/*void*/)> for (i32,usize) {
  fn setAccessibleTabName_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar20setAccessibleTabNameEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:181
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentIndex(int)

/*

*/
impl /*struct*/ QTabBar {
  pub fn setCurrentIndex_0<RetType, T: QTabBar_setCurrentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex_0(self);
    // return 1;
  }
}
pub trait QTabBar_setCurrentIndex_0<RetType> {
  fn setCurrentIndex_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_setCurrentIndex_0<(/*void*/)> for (i32) {
  fn setCurrentIndex_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar15setCurrentIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:184
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentChanged(int)

/*
This signal is emitted when the tab bar's current tab changes. The new current has the given index, or -1 if there isn't a new one (for example, if there are no tab in the QTabBar)

Note: Notifier signal for property currentIndex.
*/
impl /*struct*/ QTabBar {
  pub fn currentChanged_0<RetType, T: QTabBar_currentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentChanged_0(self);
    // return 1;
  }
}
pub trait QTabBar_currentChanged_0<RetType> {
  fn currentChanged_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_currentChanged_0<(/*void*/)> for (i32) {
  fn currentChanged_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar14currentChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:185
// index:0
// Public Visibility=Default Availability=Available
// [-2] void tabCloseRequested(int)

/*
This signal is emitted when the close button on a tab is clicked. The index is the index that should be removed.

This function was introduced in  Qt 4.5.

See also setTabsClosable().
*/
impl /*struct*/ QTabBar {
  pub fn tabCloseRequested_0<RetType, T: QTabBar_tabCloseRequested_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabCloseRequested_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabCloseRequested_0<RetType> {
  fn tabCloseRequested_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabCloseRequested_0<(/*void*/)> for (i32) {
  fn tabCloseRequested_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar17tabCloseRequestedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:186
// index:0
// Public Visibility=Default Availability=Available
// [-2] void tabMoved(int, int)

/*
This signal is emitted when the tab has moved the tab at index position from to index position to.

note: QTabWidget will automatically move the page when this signal is emitted from its tab bar.

This function was introduced in  Qt 4.5.

See also moveTab().
*/
impl /*struct*/ QTabBar {
  pub fn tabMoved_0<RetType, T: QTabBar_tabMoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabMoved_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabMoved_0<RetType> {
  fn tabMoved_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabMoved_0<(/*void*/)> for (i32,i32) {
  fn tabMoved_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar8tabMovedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void tabBarClicked(int)

/*
This signal is emitted when user clicks on a tab at an index.

index is the index of a clicked tab, or -1 if no tab is under the cursor.

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QTabBar {
  pub fn tabBarClicked_0<RetType, T: QTabBar_tabBarClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabBarClicked_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabBarClicked_0<RetType> {
  fn tabBarClicked_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabBarClicked_0<(/*void*/)> for (i32) {
  fn tabBarClicked_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar13tabBarClickedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:188
// index:0
// Public Visibility=Default Availability=Available
// [-2] void tabBarDoubleClicked(int)

/*
This signal is emitted when the user double clicks on a tab at index.

index refers to the tab clicked, or -1 if no tab is under the cursor.

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QTabBar {
  pub fn tabBarDoubleClicked_0<RetType, T: QTabBar_tabBarDoubleClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabBarDoubleClicked_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabBarDoubleClicked_0<RetType> {
  fn tabBarDoubleClicked_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabBarDoubleClicked_0<(/*void*/)> for (i32) {
  fn tabBarDoubleClicked_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar19tabBarDoubleClickedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:191
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QSize tabSizeHint(int) const

/*
Returns the size hint for the tab at position index.
*/
impl /*struct*/ QTabBar {
  pub fn tabSizeHint_0<RetType, T: QTabBar_tabSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabSizeHint_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabSizeHint_0<RetType> {
  fn tabSizeHint_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabSizeHint_0<usize> for (i32) {
  fn tabSizeHint_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar11tabSizeHintEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:192
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QSize minimumTabSizeHint(int) const

/*
Returns the minimum tab size hint for the tab at position index.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QTabBar {
  pub fn minimumTabSizeHint_0<RetType, T: QTabBar_minimumTabSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumTabSizeHint_0(self);
    // return 1;
  }
}
pub trait QTabBar_minimumTabSizeHint_0<RetType> {
  fn minimumTabSizeHint_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_minimumTabSizeHint_0<usize> for (i32) {
  fn minimumTabSizeHint_0(self , rsthis: & QTabBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QTabBar18minimumTabSizeHintEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:193
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void tabInserted(int)

/*
This virtual handler is called after a new tab was added or inserted at position index.

See also tabRemoved().
*/
impl /*struct*/ QTabBar {
  pub fn tabInserted_0<RetType, T: QTabBar_tabInserted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabInserted_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabInserted_0<RetType> {
  fn tabInserted_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabInserted_0<(/*void*/)> for (i32) {
  fn tabInserted_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar11tabInsertedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:194
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void tabRemoved(int)

/*
This virtual handler is called after a tab was removed from position index.

See also tabInserted().
*/
impl /*struct*/ QTabBar {
  pub fn tabRemoved_0<RetType, T: QTabBar_tabRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabRemoved_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabRemoved_0<RetType> {
  fn tabRemoved_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabRemoved_0<(/*void*/)> for (i32) {
  fn tabRemoved_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar10tabRemovedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:195
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void tabLayoutChange()

/*
This virtual handler is called whenever the tab layout changes.

See also tabRect().
*/
impl /*struct*/ QTabBar {
  pub fn tabLayoutChange_0<RetType, T: QTabBar_tabLayoutChange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabLayoutChange_0(self);
    // return 1;
  }
}
pub trait QTabBar_tabLayoutChange_0<RetType> {
  fn tabLayoutChange_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_tabLayoutChange_0<(/*void*/)> for () {
  fn tabLayoutChange_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QTabBar15tabLayoutChangeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:197
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QTabBar {
  pub fn event_0<RetType, T: QTabBar_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QTabBar_event_0<RetType> {
  fn event_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QTabBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QTabBar5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:198
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QTabBar {
  pub fn resizeEvent_0<RetType, T: QTabBar_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QTabBar_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:199
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QTabBar {
  pub fn showEvent_0<RetType, T: QTabBar_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QTabBar_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:200
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hideEvent(QHideEvent *)

/*
Reimplemented from QWidget::hideEvent().
*/
impl /*struct*/ QTabBar {
  pub fn hideEvent_0<RetType, T: QTabBar_hideEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideEvent_0(self);
    // return 1;
  }
}
pub trait QTabBar_hideEvent_0<RetType> {
  fn hideEvent_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_hideEvent_0<(/*void*/)> for (usize) {
  fn hideEvent_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar9hideEventEP10QHideEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:201
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QTabBar {
  pub fn paintEvent_0<RetType, T: QTabBar_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QTabBar_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:202
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QTabBar {
  pub fn mousePressEvent_0<RetType, T: QTabBar_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QTabBar_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:203
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QTabBar {
  pub fn mouseMoveEvent_0<RetType, T: QTabBar_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QTabBar_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:204
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QTabBar {
  pub fn mouseReleaseEvent_0<RetType, T: QTabBar_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QTabBar_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:206
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
Reimplemented from QWidget::wheelEvent().
*/
impl /*struct*/ QTabBar {
  pub fn wheelEvent_0<RetType, T: QTabBar_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QTabBar_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:208
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QTabBar {
  pub fn keyPressEvent_0<RetType, T: QTabBar_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QTabBar_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:209
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QTabBar {
  pub fn changeEvent_0<RetType, T: QTabBar_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QTabBar_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:210
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QTabBar {
  pub fn timerEvent_0<RetType, T: QTabBar_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QTabBar_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QTabBar10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtabbar.h:211
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionTab *, int) const

/*
Initialize option with the values from the tab at tabIndex. This method is useful for subclasses when they need a QStyleOptionTab, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom() and QTabWidget::initStyleOption().
*/
impl /*struct*/ QTabBar {
  pub fn initStyleOption_0<RetType, T: QTabBar_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QTabBar_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QTabBar) -> RetType;
}
impl<'a> /*trait*/ QTabBar_initStyleOption_0<(/*void*/)> for (usize,i32) {
  fn initStyleOption_0(self , rsthis: & QTabBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZNK7QTabBar15initStyleOptionEP15QStyleOptionTabi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum type lists the built-in shapes supported by QTabBar. Treat these as hints as some styles may not render some of the shapes. However, position should be honored.


*/
pub type QTabBar__Shape = i32;
// The normal rounded look above the pages
pub const QTabBar__RoundedNorth :QTabBar__Shape = 0;
// The normal rounded look below the pages
pub const QTabBar__RoundedSouth :QTabBar__Shape = 1;
// The normal rounded look on the left side of the pages
pub const QTabBar__RoundedWest :QTabBar__Shape = 2;
// The normal rounded look on the right side the pages
pub const QTabBar__RoundedEast :QTabBar__Shape = 3;
// Triangular tabs above the pages.
pub const QTabBar__TriangularNorth :QTabBar__Shape = 4;
// Triangular tabs similar to those used in the Excel spreadsheet, for example
pub const QTabBar__TriangularSouth :QTabBar__Shape = 5;
// Triangular tabs on the left of the pages.
pub const QTabBar__TriangularWest :QTabBar__Shape = 6;
// Triangular tabs on the right of the pages.
pub const QTabBar__TriangularEast :QTabBar__Shape = 7;
pub fn QTabBar_ShapeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTabBar", val);
}
pub fn QTabBar_ShapeItemName_s(val: i32) ->String {
  //var nilthis *QTabBar
  //return nilthis.ShapeItemName(val);
  return QTabBar_ShapeItemName(val);
}


/*
This enum type lists the location of the widget on a tab.



This enum was introduced or modified in  Qt 4.5.

*/
pub type QTabBar__ButtonPosition = i32;
// Left side of the tab.
pub const QTabBar__LeftSide :QTabBar__ButtonPosition = 0;
// Right side of the tab.
pub const QTabBar__RightSide :QTabBar__ButtonPosition = 1;
pub fn QTabBar_ButtonPositionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTabBar", val);
}
pub fn QTabBar_ButtonPositionItemName_s(val: i32) ->String {
  //var nilthis *QTabBar
  //return nilthis.ButtonPositionItemName(val);
  return QTabBar_ButtonPositionItemName(val);
}


/*
This enum type lists the behavior of QTabBar when a tab is removed and the tab being removed is also the current tab.



This enum was introduced or modified in  Qt 4.5.

*/
pub type QTabBar__SelectionBehavior = i32;
// Select the tab to the left of the one being removed.
pub const QTabBar__SelectLeftTab :QTabBar__SelectionBehavior = 0;
// Select the tab to the right of the one being removed.
pub const QTabBar__SelectRightTab :QTabBar__SelectionBehavior = 1;
// Select the previously selected tab.
pub const QTabBar__SelectPreviousTab :QTabBar__SelectionBehavior = 2;
pub fn QTabBar_SelectionBehaviorItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTabBar", val);
}
pub fn QTabBar_SelectionBehaviorItemName_s(val: i32) ->String {
  //var nilthis *QTabBar
  //return nilthis.SelectionBehaviorItemName(val);
  return QTabBar_SelectionBehaviorItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
