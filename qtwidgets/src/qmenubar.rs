

// mod ::widgets::QMenuBar
// package qtwidgets
// /usr/include/qt/QtWidgets/qmenubar.h
// #include <qmenubar.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 66
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

// void changeEvent(QEvent *)
// func (this *QMenuBar) InheritChangeEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QMenuBar) InheritKeyPressEvent(f func(arg0 *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QMenuBar) InheritMouseReleaseEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QMenuBar) InheritMousePressEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QMenuBar) InheritMouseMoveEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void leaveEvent(QEvent *)
// func (this *QMenuBar) InheritLeaveEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "leaveEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QMenuBar) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QMenuBar) InheritResizeEvent(f func(arg0 *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void actionEvent(QActionEvent *)
// func (this *QMenuBar) InheritActionEvent(f func(arg0 *qtgui.QActionEvent/*777 QActionEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "actionEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QMenuBar) InheritFocusOutEvent(f func(arg0 *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QMenuBar) InheritFocusInEvent(f func(arg0 *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QMenuBar) InheritTimerEvent(f func(arg0 *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QMenuBar) InheritEventFilter(f func(arg0 *qtcore.QObject/*777 QObject **/, arg1 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// bool event(QEvent *)
// func (this *QMenuBar) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void initStyleOption(QStyleOptionMenuItem *, const QAction *)
// func (this *QMenuBar) InheritInitStyleOption(f func(option *QStyleOptionMenuItem/*777 QStyleOptionMenuItem **/, action *QAction/*777 const QAction **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QMenuBar)=48
pub struct QMenuBar {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMenuBar_ITF interface {
//    QWidget_ITF
//    QMenuBar_PTR() *QMenuBar
//}
//func (ptr *QMenuBar) QMenuBar_PTR() *QMenuBar { return ptr }

impl /*struct*/ QMenuBar {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMenuBar {
    return QMenuBar{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMenuBar {
//  type Target = QMenuBarBASE;
//
//  fn deref(&self) -> &QMenuBarBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMenuBarBASE> for QMenuBar {
//  fn as_ref(& self) -> & QMenuBarBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qmenubar.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QMenuBar {
  pub fn metaObject_0<RetType, T: QMenuBar_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QMenuBar_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMenuBar10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMenuBar(QWidget *)

/*
Constructs a menu bar with parent parent.
*/
// QMenuBar(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QMenuBar {
  pub fn QMenuBar_0<T: QMenuBar_QMenuBar_0>(value: T) -> QMenuBar {
    let rsthis = value.QMenuBar_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMenuBar_QMenuBar_0 {
  fn QMenuBar_0(self) -> QMenuBar;
}
// QMenuBar(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMenuBar_QMenuBar_0 for (usize) {
  fn QMenuBar_0(self) -> QMenuBar {
    // unsafe{_ZN8QMenuBarC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QMenuBarC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMenuBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QMenuBar()

/*

*/
pub fn DeleteQMenuBar(this :*mut QMenuBar) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QMenuBarD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qmenubar.h:67
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(const QString &)

/*
This is an overloaded function.

This convenience function creates a new action with text. The function adds the newly created action to the menu's list of actions, and returns it.

See also QWidget::addAction() and QWidget::actions().
*/
impl /*struct*/ QMenuBar {
  pub fn addAction_0<RetType, T: QMenuBar_addAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_0(self);
    // return 1;
  }
}
pub trait QMenuBar_addAction_0<RetType> {
  fn addAction_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_addAction_0<usize> for (usize) {
  fn addAction_0(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMenuBar9addActionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:68
// index:1
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(const QString &, const QObject *, const char *)

/*
This is an overloaded function.

This convenience function creates a new action with text. The function adds the newly created action to the menu's list of actions, and returns it.

See also QWidget::addAction() and QWidget::actions().
*/
impl /*struct*/ QMenuBar {
  pub fn addAction_1<RetType, T: QMenuBar_addAction_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_1(self);
    // return 1;
  }
}
pub trait QMenuBar_addAction_1<RetType> {
  fn addAction_1(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_addAction_1<usize> for (usize,usize,usize) {
  fn addAction_1(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMenuBar9addActionERK7QStringPK7QObjectPKc", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:70
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * addMenu(QMenu *)

/*
Appends menu to the menu bar. Returns the menu's menuAction().

Note: The returned QAction object can be used to hide the corresponding menu.

See also QWidget::addAction() and QMenu::menuAction().
*/
impl /*struct*/ QMenuBar {
  pub fn addMenu_0<RetType, T: QMenuBar_addMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addMenu_0(self);
    // return 1;
  }
}
pub trait QMenuBar_addMenu_0<RetType> {
  fn addMenu_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_addMenu_0<usize> for (usize) {
  fn addMenu_0(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMenuBar7addMenuEP5QMenu", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:71
// index:1
// Public Visibility=Default Availability=Available
// [8] QMenu * addMenu(const QString &)

/*
Appends menu to the menu bar. Returns the menu's menuAction().

Note: The returned QAction object can be used to hide the corresponding menu.

See also QWidget::addAction() and QMenu::menuAction().
*/
impl /*struct*/ QMenuBar {
  pub fn addMenu_1<RetType, T: QMenuBar_addMenu_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addMenu_1(self);
    // return 1;
  }
}
pub trait QMenuBar_addMenu_1<RetType> {
  fn addMenu_1(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_addMenu_1<usize> for (usize) {
  fn addMenu_1(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMenuBar7addMenuERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:72
// index:2
// Public Visibility=Default Availability=Available
// [8] QMenu * addMenu(const QIcon &, const QString &)

/*
Appends menu to the menu bar. Returns the menu's menuAction().

Note: The returned QAction object can be used to hide the corresponding menu.

See also QWidget::addAction() and QMenu::menuAction().
*/
impl /*struct*/ QMenuBar {
  pub fn addMenu_2<RetType, T: QMenuBar_addMenu_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addMenu_2(self);
    // return 1;
  }
}
pub trait QMenuBar_addMenu_2<RetType> {
  fn addMenu_2(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_addMenu_2<usize> for (usize,usize) {
  fn addMenu_2(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMenuBar7addMenuERK5QIconRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:75
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * addSeparator()

/*
Appends a separator to the menu.
*/
impl /*struct*/ QMenuBar {
  pub fn addSeparator_0<RetType, T: QMenuBar_addSeparator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addSeparator_0(self);
    // return 1;
  }
}
pub trait QMenuBar_addSeparator_0<RetType> {
  fn addSeparator_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_addSeparator_0<usize> for () {
  fn addSeparator_0(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMenuBar12addSeparatorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * insertSeparator(QAction *)

/*
This convenience function creates a new separator action, i.e. an action with QAction::isSeparator() returning true. The function inserts the newly created action into this menu bar's list of actions before action before and returns it.

See also QWidget::insertAction() and addSeparator().
*/
impl /*struct*/ QMenuBar {
  pub fn insertSeparator_0<RetType, T: QMenuBar_insertSeparator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertSeparator_0(self);
    // return 1;
  }
}
pub trait QMenuBar_insertSeparator_0<RetType> {
  fn insertSeparator_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_insertSeparator_0<usize> for (usize) {
  fn insertSeparator_0(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMenuBar15insertSeparatorEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * insertMenu(QAction *, QMenu *)

/*
This convenience function inserts menu before action before and returns the menus menuAction().

See also QWidget::insertAction() and addMenu().
*/
impl /*struct*/ QMenuBar {
  pub fn insertMenu_0<RetType, T: QMenuBar_insertMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertMenu_0(self);
    // return 1;
  }
}
pub trait QMenuBar_insertMenu_0<RetType> {
  fn insertMenu_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_insertMenu_0<usize> for (usize,usize) {
  fn insertMenu_0(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMenuBar10insertMenuEP7QActionP5QMenu", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Removes all the actions from the menu bar.

Note: On macOS, menu items that have been merged to the system menu bar are not removed by this function. One way to handle this would be to remove the extra actions yourself. You can set the menu role on the different menus, so that you know ahead of time which menu items get merged and which do not. Then decide what to recreate or remove yourself.

See also removeAction().
*/
impl /*struct*/ QMenuBar {
  pub fn clear_0<RetType, T: QMenuBar_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QMenuBar_clear_0<RetType> {
  fn clear_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QMenuBar5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:82
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * activeAction() const

/*
Returns the QAction that is currently highlighted. A null pointer will be returned if no action is currently selected.

See also setActiveAction().
*/
impl /*struct*/ QMenuBar {
  pub fn activeAction_0<RetType, T: QMenuBar_activeAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activeAction_0(self);
    // return 1;
  }
}
pub trait QMenuBar_activeAction_0<RetType> {
  fn activeAction_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_activeAction_0<usize> for () {
  fn activeAction_0(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMenuBar12activeActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setActiveAction(QAction *)

/*
Sets the currently highlighted action to act.

This function was introduced in  Qt 4.1.

See also activeAction().
*/
impl /*struct*/ QMenuBar {
  pub fn setActiveAction_0<RetType, T: QMenuBar_setActiveAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setActiveAction_0(self);
    // return 1;
  }
}
pub trait QMenuBar_setActiveAction_0<RetType> {
  fn setActiveAction_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_setActiveAction_0<(/*void*/)> for (usize) {
  fn setActiveAction_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar15setActiveActionEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultUp(bool)

/*

*/
impl /*struct*/ QMenuBar {
  pub fn setDefaultUp_0<RetType, T: QMenuBar_setDefaultUp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultUp_0(self);
    // return 1;
  }
}
pub trait QMenuBar_setDefaultUp_0<RetType> {
  fn setDefaultUp_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_setDefaultUp_0<(/*void*/)> for (bool) {
  fn setDefaultUp_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar12setDefaultUpEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:86
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDefaultUp() const

/*

*/
impl /*struct*/ QMenuBar {
  pub fn isDefaultUp_0<RetType, T: QMenuBar_isDefaultUp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDefaultUp_0(self);
    // return 1;
  }
}
pub trait QMenuBar_isDefaultUp_0<RetType> {
  fn isDefaultUp_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_isDefaultUp_0<bool> for () {
  fn isDefaultUp_0(self , rsthis: & QMenuBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMenuBar11isDefaultUpEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QMenuBar {
  pub fn sizeHint_0<RetType, T: QMenuBar_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QMenuBar_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMenuBar8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:89
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QMenuBar {
  pub fn minimumSizeHint_0<RetType, T: QMenuBar_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QMenuBar_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMenuBar15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:90
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int heightForWidth(int) const

/*
Reimplemented from QWidget::heightForWidth().
*/
impl /*struct*/ QMenuBar {
  pub fn heightForWidth_0<RetType, T: QMenuBar_heightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth_0(self);
    // return 1;
  }
}
pub trait QMenuBar_heightForWidth_0<RetType> {
  fn heightForWidth_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_heightForWidth_0<i32> for (i32) {
  fn heightForWidth_0(self , rsthis: & QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMenuBar14heightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:92
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect actionGeometry(QAction *) const

/*
Returns the geometry of action act as a QRect.

See also actionAt().
*/
impl /*struct*/ QMenuBar {
  pub fn actionGeometry_0<RetType, T: QMenuBar_actionGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionGeometry_0(self);
    // return 1;
  }
}
pub trait QMenuBar_actionGeometry_0<RetType> {
  fn actionGeometry_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_actionGeometry_0<usize> for (usize) {
  fn actionGeometry_0(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMenuBar14actionGeometryEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:93
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * actionAt(const QPoint &) const

/*
Returns the QAction at pt. Returns 0 if there is no action at pt or if the location has a separator.

See also addAction() and addSeparator().
*/
impl /*struct*/ QMenuBar {
  pub fn actionAt_0<RetType, T: QMenuBar_actionAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionAt_0(self);
    // return 1;
  }
}
pub trait QMenuBar_actionAt_0<RetType> {
  fn actionAt_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_actionAt_0<usize> for (usize) {
  fn actionAt_0(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMenuBar8actionAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCornerWidget(QWidget *, Qt::Corner)

/*
This sets the given widget to be shown directly on the left of the first menu item, or on the right of the last menu item, depending on corner.

The menu bar takes ownership of widget, reparenting it into the menu bar. However, if the corner already contains a widget, this previous widget will no longer be managed and will still be a visible child of the menu bar.

Note: Using a corner other than Qt::TopRightCorner or Qt::TopLeftCorner will result in a warning.

See also cornerWidget().
*/
impl /*struct*/ QMenuBar {
  pub fn setCornerWidget_0<RetType, T: QMenuBar_setCornerWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCornerWidget_0(self);
    // return 1;
  }
}
pub trait QMenuBar_setCornerWidget_0<RetType> {
  fn setCornerWidget_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_setCornerWidget_0<(/*void*/)> for (usize,i32) {
  fn setCornerWidget_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar15setCornerWidgetEP7QWidgetN2Qt6CornerE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:96
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * cornerWidget(Qt::Corner) const

/*
Returns the widget on the left of the first or on the right of the last menu item, depending on corner.

Note: Using a corner other than Qt::TopRightCorner or Qt::TopLeftCorner will result in a warning.

See also setCornerWidget().
*/
impl /*struct*/ QMenuBar {
  pub fn cornerWidget_0<RetType, T: QMenuBar_cornerWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cornerWidget_0(self);
    // return 1;
  }
}
pub trait QMenuBar_cornerWidget_0<RetType> {
  fn cornerWidget_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_cornerWidget_0<usize> for (i32) {
  fn cornerWidget_0(self , rsthis: & QMenuBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMenuBar12cornerWidgetEN2Qt6CornerE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:102
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNativeMenuBar() const

/*

*/
impl /*struct*/ QMenuBar {
  pub fn isNativeMenuBar_0<RetType, T: QMenuBar_isNativeMenuBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNativeMenuBar_0(self);
    // return 1;
  }
}
pub trait QMenuBar_isNativeMenuBar_0<RetType> {
  fn isNativeMenuBar_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_isNativeMenuBar_0<bool> for () {
  fn isNativeMenuBar_0(self , rsthis: & QMenuBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMenuBar15isNativeMenuBarEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNativeMenuBar(bool)

/*

*/
impl /*struct*/ QMenuBar {
  pub fn setNativeMenuBar_0<RetType, T: QMenuBar_setNativeMenuBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNativeMenuBar_0(self);
    // return 1;
  }
}
pub trait QMenuBar_setNativeMenuBar_0<RetType> {
  fn setNativeMenuBar_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_setNativeMenuBar_0<(/*void*/)> for (bool) {
  fn setNativeMenuBar_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar16setNativeMenuBarEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:106
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*
Reimplemented from QWidget::setVisible().
*/
impl /*struct*/ QMenuBar {
  pub fn setVisible_0<RetType, T: QMenuBar_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QMenuBar_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void triggered(QAction *)

/*
This signal is emitted when an action in a menu belonging to this menubar is triggered as a result of a mouse click; action is the action that caused the signal to be emitted.

Note: QMenuBar has to have ownership of the QMenu in order this signal to work.

Normally, you connect each menu action to a single slot using QAction::triggered(), but sometimes you will want to connect several items to a single slot (most often if the user selects from an array). This signal is useful in such cases.

See also hovered() and QAction::triggered().
*/
impl /*struct*/ QMenuBar {
  pub fn triggered_0<RetType, T: QMenuBar_triggered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.triggered_0(self);
    // return 1;
  }
}
pub trait QMenuBar_triggered_0<RetType> {
  fn triggered_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_triggered_0<(/*void*/)> for (usize) {
  fn triggered_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar9triggeredEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void hovered(QAction *)

/*
This signal is emitted when a menu action is highlighted; action is the action that caused the event to be sent.

Often this is used to update status information.

See also triggered() and QAction::hovered().
*/
impl /*struct*/ QMenuBar {
  pub fn hovered_0<RetType, T: QMenuBar_hovered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hovered_0(self);
    // return 1;
  }
}
pub trait QMenuBar_hovered_0<RetType> {
  fn hovered_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_hovered_0<(/*void*/)> for (usize) {
  fn hovered_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar7hoveredEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:113
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QMenuBar {
  pub fn changeEvent_0<RetType, T: QMenuBar_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QMenuBar_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:114
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QMenuBar {
  pub fn keyPressEvent_0<RetType, T: QMenuBar_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QMenuBar_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:115
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QMenuBar {
  pub fn mouseReleaseEvent_0<RetType, T: QMenuBar_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QMenuBar_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:116
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QMenuBar {
  pub fn mousePressEvent_0<RetType, T: QMenuBar_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QMenuBar_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:117
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QMenuBar {
  pub fn mouseMoveEvent_0<RetType, T: QMenuBar_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QMenuBar_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:118
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void leaveEvent(QEvent *)

/*
Reimplemented from QWidget::leaveEvent().
*/
impl /*struct*/ QMenuBar {
  pub fn leaveEvent_0<RetType, T: QMenuBar_leaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leaveEvent_0(self);
    // return 1;
  }
}
pub trait QMenuBar_leaveEvent_0<RetType> {
  fn leaveEvent_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_leaveEvent_0<(/*void*/)> for (usize) {
  fn leaveEvent_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar10leaveEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:119
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QMenuBar {
  pub fn paintEvent_0<RetType, T: QMenuBar_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QMenuBar_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:120
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QMenuBar {
  pub fn resizeEvent_0<RetType, T: QMenuBar_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QMenuBar_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:121
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void actionEvent(QActionEvent *)

/*
Reimplemented from QWidget::actionEvent().
*/
impl /*struct*/ QMenuBar {
  pub fn actionEvent_0<RetType, T: QMenuBar_actionEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionEvent_0(self);
    // return 1;
  }
}
pub trait QMenuBar_actionEvent_0<RetType> {
  fn actionEvent_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_actionEvent_0<(/*void*/)> for (usize) {
  fn actionEvent_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar11actionEventEP12QActionEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:122
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusOutEvent().
*/
impl /*struct*/ QMenuBar {
  pub fn focusOutEvent_0<RetType, T: QMenuBar_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QMenuBar_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:123
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().
*/
impl /*struct*/ QMenuBar {
  pub fn focusInEvent_0<RetType, T: QMenuBar_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QMenuBar_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:124
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QMenuBar {
  pub fn timerEvent_0<RetType, T: QMenuBar_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QMenuBar_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QMenuBar10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:125
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().
*/
impl /*struct*/ QMenuBar {
  pub fn eventFilter_0<RetType, T: QMenuBar_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QMenuBar_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QMenuBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMenuBar11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:126
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QMenuBar {
  pub fn event_0<RetType, T: QMenuBar_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QMenuBar_event_0<RetType> {
  fn event_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QMenuBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMenuBar5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenubar.h:127
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionMenuItem *, const QAction *) const

/*
Initialize option with the values from the menu bar and information from action. This method is useful for subclasses when they need a QStyleOptionMenuItem, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom() and QMenu::initStyleOption().
*/
impl /*struct*/ QMenuBar {
  pub fn initStyleOption_0<RetType, T: QMenuBar_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QMenuBar_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QMenuBar) -> RetType;
}
impl<'a> /*trait*/ QMenuBar_initStyleOption_0<(/*void*/)> for (usize,usize) {
  fn initStyleOption_0(self , rsthis: & QMenuBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK8QMenuBar15initStyleOptionEP20QStyleOptionMenuItemPK7QAction", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
