

// mod ::widgets::QMenu
// package qtwidgets
// /usr/include/qt/QtWidgets/qmenu.h
// #include <qmenu.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 43
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

// int columnCount()
// func (this *QMenu) InheritColumnCount(f func() int) {
//  qtrt.SetAllInheritCallback(this, "columnCount", f)
// }

// void changeEvent(QEvent *)
// func (this *QMenu) InheritChangeEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QMenu) InheritKeyPressEvent(f func(arg0 *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QMenu) InheritMouseReleaseEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QMenu) InheritMousePressEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QMenu) InheritMouseMoveEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void wheelEvent(QWheelEvent *)
// func (this *QMenu) InheritWheelEvent(f func(arg0 *qtgui.QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void enterEvent(QEvent *)
// func (this *QMenu) InheritEnterEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "enterEvent", f)
// }

// void leaveEvent(QEvent *)
// func (this *QMenu) InheritLeaveEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "leaveEvent", f)
// }

// void hideEvent(QHideEvent *)
// func (this *QMenu) InheritHideEvent(f func(arg0 *qtgui.QHideEvent/*777 QHideEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hideEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QMenu) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void actionEvent(QActionEvent *)
// func (this *QMenu) InheritActionEvent(f func(arg0 *qtgui.QActionEvent/*777 QActionEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "actionEvent", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QMenu) InheritTimerEvent(f func(arg0 *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// bool event(QEvent *)
// func (this *QMenu) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// bool focusNextPrevChild(bool)
// func (this *QMenu) InheritFocusNextPrevChild(f func(next bool) bool) {
//  qtrt.SetAllInheritCallback(this, "focusNextPrevChild", f)
// }

// void initStyleOption(QStyleOptionMenuItem *, const QAction *)
// func (this *QMenu) InheritInitStyleOption(f func(option *QStyleOptionMenuItem/*777 QStyleOptionMenuItem **/, action *QAction/*777 const QAction **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QMenu)=48
pub struct QMenu {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMenu_ITF interface {
//    QWidget_ITF
//    QMenu_PTR() *QMenu
//}
//func (ptr *QMenu) QMenu_PTR() *QMenu { return ptr }

impl /*struct*/ QMenu {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMenu {
    return QMenu{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMenu {
//  type Target = QMenuBASE;
//
//  fn deref(&self) -> &QMenuBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMenuBASE> for QMenu {
//  fn as_ref(& self) -> & QMenuBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qmenu.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QMenu {
  pub fn metaObject_0<RetType, T: QMenu_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QMenu_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMenu(QWidget *)

/*
Constructs a menu with parent parent.

Although a popup menu is always a top-level widget, if a parent is passed the popup menu will be deleted when that parent is destroyed (as with any other QObject).
*/
// QMenu(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QMenu {
  pub fn QMenu_0<T: QMenu_QMenu_0>(value: T) -> QMenu {
    let rsthis = value.QMenu_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMenu_QMenu_0 {
  fn QMenu_0(self) -> QMenu;
}
// QMenu(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMenu_QMenu_0 for (usize) {
  fn QMenu_0(self) -> QMenu {
    // unsafe{_ZN5QMenuC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QMenuC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMenu{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:75
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QMenu(const QString &, QWidget *)

/*
Constructs a menu with parent parent.

Although a popup menu is always a top-level widget, if a parent is passed the popup menu will be deleted when that parent is destroyed (as with any other QObject).
*/
// QMenu(const QString &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QMenu {
  pub fn QMenu_1<T: QMenu_QMenu_1>(value: T) -> QMenu {
    let rsthis = value.QMenu_1();
    return rsthis;
    // return 1;
  }
}

pub trait QMenu_QMenu_1 {
  fn QMenu_1(self) -> QMenu;
}
// QMenu(const QString &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMenu_QMenu_1 for (usize,usize) {
  fn QMenu_1(self) -> QMenu {
    // unsafe{_ZN5QMenuC2ERK7QStringP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QMenuC2ERK7QStringP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMenu{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QMenu()

/*

*/
pub fn DeleteQMenu(this :*mut QMenu) {
    // let rv = qtrt::InvokeQtFunc6("_ZN5QMenuD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qmenu.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(const QString &)

/*
This is an overloaded function.

This convenience function creates a new action with text. The function adds the newly created action to the menu's list of actions, and returns it.

QMenu takes ownership of the returned QAction.

See also QWidget::addAction().
*/
impl /*struct*/ QMenu {
  pub fn addAction_0<RetType, T: QMenu_addAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_0(self);
    // return 1;
  }
}
pub trait QMenu_addAction_0<RetType> {
  fn addAction_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_addAction_0<usize> for (usize) {
  fn addAction_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu9addActionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:80
// index:1
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(const QIcon &, const QString &)

/*
This is an overloaded function.

This convenience function creates a new action with text. The function adds the newly created action to the menu's list of actions, and returns it.

QMenu takes ownership of the returned QAction.

See also QWidget::addAction().
*/
impl /*struct*/ QMenu {
  pub fn addAction_1<RetType, T: QMenu_addAction_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_1(self);
    // return 1;
  }
}
pub trait QMenu_addAction_1<RetType> {
  fn addAction_1(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_addAction_1<usize> for (usize,usize) {
  fn addAction_1(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu9addActionERK5QIconRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:81
// index:2
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(const QString &, const QObject *, const char *, const QKeySequence &)

/*
This is an overloaded function.

This convenience function creates a new action with text. The function adds the newly created action to the menu's list of actions, and returns it.

QMenu takes ownership of the returned QAction.

See also QWidget::addAction().
*/
impl /*struct*/ QMenu {
  pub fn addAction_2<RetType, T: QMenu_addAction_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_2(self);
    // return 1;
  }
}
pub trait QMenu_addAction_2<RetType> {
  fn addAction_2(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_addAction_2<usize> for (usize,usize,usize,usize) {
  fn addAction_2(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (self.2) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu9addActionERK7QStringPK7QObjectPKcRK12QKeySequence", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:82
// index:3
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(const QIcon &, const QString &, const QObject *, const char *, const QKeySequence &)

/*
This is an overloaded function.

This convenience function creates a new action with text. The function adds the newly created action to the menu's list of actions, and returns it.

QMenu takes ownership of the returned QAction.

See also QWidget::addAction().
*/
impl /*struct*/ QMenu {
  pub fn addAction_3<RetType, T: QMenu_addAction_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_3(self);
    // return 1;
  }
}
pub trait QMenu_addAction_3<RetType> {
  fn addAction_3(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_addAction_3<usize> for (usize,usize,usize,usize,usize) {
  fn addAction_3(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (self.3) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu9addActionERK5QIconRK7QStringPK7QObjectPKcRK12QKeySequence", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:156
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * addMenu(QMenu *)

/*
This convenience function adds menu as a submenu to this menu. It returns menu's menuAction(). This menu does not take ownership of menu.

See also QWidget::addAction() and QMenu::menuAction().
*/
impl /*struct*/ QMenu {
  pub fn addMenu_0<RetType, T: QMenu_addMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addMenu_0(self);
    // return 1;
  }
}
pub trait QMenu_addMenu_0<RetType> {
  fn addMenu_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_addMenu_0<usize> for (usize) {
  fn addMenu_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu7addMenuEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:157
// index:1
// Public Visibility=Default Availability=Available
// [8] QMenu * addMenu(const QString &)

/*
This convenience function adds menu as a submenu to this menu. It returns menu's menuAction(). This menu does not take ownership of menu.

See also QWidget::addAction() and QMenu::menuAction().
*/
impl /*struct*/ QMenu {
  pub fn addMenu_1<RetType, T: QMenu_addMenu_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addMenu_1(self);
    // return 1;
  }
}
pub trait QMenu_addMenu_1<RetType> {
  fn addMenu_1(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_addMenu_1<usize> for (usize) {
  fn addMenu_1(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu7addMenuERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:158
// index:2
// Public Visibility=Default Availability=Available
// [8] QMenu * addMenu(const QIcon &, const QString &)

/*
This convenience function adds menu as a submenu to this menu. It returns menu's menuAction(). This menu does not take ownership of menu.

See also QWidget::addAction() and QMenu::menuAction().
*/
impl /*struct*/ QMenu {
  pub fn addMenu_2<RetType, T: QMenu_addMenu_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addMenu_2(self);
    // return 1;
  }
}
pub trait QMenu_addMenu_2<RetType> {
  fn addMenu_2(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_addMenu_2<usize> for (usize,usize) {
  fn addMenu_2(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu7addMenuERK5QIconRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:160
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * addSeparator()

/*
This convenience function creates a new separator action, i.e. an action with QAction::isSeparator() returning true, and adds the new action to this menu's list of actions. It returns the newly created action.

QMenu takes ownership of the returned QAction.

See also QWidget::addAction().
*/
impl /*struct*/ QMenu {
  pub fn addSeparator_0<RetType, T: QMenu_addSeparator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addSeparator_0(self);
    // return 1;
  }
}
pub trait QMenu_addSeparator_0<RetType> {
  fn addSeparator_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_addSeparator_0<usize> for () {
  fn addSeparator_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu12addSeparatorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:162
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * addSection(const QString &)

/*
This convenience function creates a new section action, i.e. an action with QAction::isSeparator() returning true but also having text hint, and adds the new action to this menu's list of actions. It returns the newly created action.

The rendering of the hint is style and platform dependent. Widget styles can use the text information in the rendering for sections, or can choose to ignore it and render sections like simple separators.

QMenu takes ownership of the returned QAction.

This function was introduced in  Qt 5.1.

See also QWidget::addAction().
*/
impl /*struct*/ QMenu {
  pub fn addSection_0<RetType, T: QMenu_addSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addSection_0(self);
    // return 1;
  }
}
pub trait QMenu_addSection_0<RetType> {
  fn addSection_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_addSection_0<usize> for (usize) {
  fn addSection_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu10addSectionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:163
// index:1
// Public Visibility=Default Availability=Available
// [8] QAction * addSection(const QIcon &, const QString &)

/*
This convenience function creates a new section action, i.e. an action with QAction::isSeparator() returning true but also having text hint, and adds the new action to this menu's list of actions. It returns the newly created action.

The rendering of the hint is style and platform dependent. Widget styles can use the text information in the rendering for sections, or can choose to ignore it and render sections like simple separators.

QMenu takes ownership of the returned QAction.

This function was introduced in  Qt 5.1.

See also QWidget::addAction().
*/
impl /*struct*/ QMenu {
  pub fn addSection_1<RetType, T: QMenu_addSection_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addSection_1(self);
    // return 1;
  }
}
pub trait QMenu_addSection_1<RetType> {
  fn addSection_1(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_addSection_1<usize> for (usize,usize) {
  fn addSection_1(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu10addSectionERK5QIconRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:165
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * insertMenu(QAction *, QMenu *)

/*
This convenience function inserts menu before action before and returns the menus menuAction().

See also QWidget::insertAction() and addMenu().
*/
impl /*struct*/ QMenu {
  pub fn insertMenu_0<RetType, T: QMenu_insertMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertMenu_0(self);
    // return 1;
  }
}
pub trait QMenu_insertMenu_0<RetType> {
  fn insertMenu_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_insertMenu_0<usize> for (usize,usize) {
  fn insertMenu_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu10insertMenuEP7QActionPS_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:166
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * insertSeparator(QAction *)

/*
This convenience function creates a new separator action, i.e. an action with QAction::isSeparator() returning true. The function inserts the newly created action into this menu's list of actions before action before and returns it.

QMenu takes ownership of the returned QAction.

See also QWidget::insertAction() and addSeparator().
*/
impl /*struct*/ QMenu {
  pub fn insertSeparator_0<RetType, T: QMenu_insertSeparator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertSeparator_0(self);
    // return 1;
  }
}
pub trait QMenu_insertSeparator_0<RetType> {
  fn insertSeparator_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_insertSeparator_0<usize> for (usize) {
  fn insertSeparator_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu15insertSeparatorEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:167
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * insertSection(QAction *, const QString &)

/*
This convenience function creates a new title action, i.e. an action with QAction::isSeparator() returning true but also having text hint. The function inserts the newly created action into this menu's list of actions before action before and returns it.

The rendering of the hint is style and platform dependent. Widget styles can use the text information in the rendering for sections, or can choose to ignore it and render sections like simple separators.

QMenu takes ownership of the returned QAction.

This function was introduced in  Qt 5.1.

See also QWidget::insertAction() and addSection().
*/
impl /*struct*/ QMenu {
  pub fn insertSection_0<RetType, T: QMenu_insertSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertSection_0(self);
    // return 1;
  }
}
pub trait QMenu_insertSection_0<RetType> {
  fn insertSection_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_insertSection_0<usize> for (usize,usize) {
  fn insertSection_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu13insertSectionEP7QActionRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:168
// index:1
// Public Visibility=Default Availability=Available
// [8] QAction * insertSection(QAction *, const QIcon &, const QString &)

/*
This convenience function creates a new title action, i.e. an action with QAction::isSeparator() returning true but also having text hint. The function inserts the newly created action into this menu's list of actions before action before and returns it.

The rendering of the hint is style and platform dependent. Widget styles can use the text information in the rendering for sections, or can choose to ignore it and render sections like simple separators.

QMenu takes ownership of the returned QAction.

This function was introduced in  Qt 5.1.

See also QWidget::insertAction() and addSection().
*/
impl /*struct*/ QMenu {
  pub fn insertSection_1<RetType, T: QMenu_insertSection_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertSection_1(self);
    // return 1;
  }
}
pub trait QMenu_insertSection_1<RetType> {
  fn insertSection_1(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_insertSection_1<usize> for (usize,usize,usize) {
  fn insertSection_1(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu13insertSectionEP7QActionRK5QIconRK7QString", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:170
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if there are no visible actions inserted into the menu, false otherwise.

This function was introduced in  Qt 4.2.

See also QWidget::actions().
*/
impl /*struct*/ QMenu {
  pub fn isEmpty_0<RetType, T: QMenu_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QMenu_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QMenu) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:171
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Removes all the menu's actions. Actions owned by the menu and not shown in any other widget are deleted.

See also removeAction().
*/
impl /*struct*/ QMenu {
  pub fn clear_0<RetType, T: QMenu_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QMenu_clear_0<RetType> {
  fn clear_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN5QMenu5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:173
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTearOffEnabled(bool)

/*

*/
impl /*struct*/ QMenu {
  pub fn setTearOffEnabled_0<RetType, T: QMenu_setTearOffEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTearOffEnabled_0(self);
    // return 1;
  }
}
pub trait QMenu_setTearOffEnabled_0<RetType> {
  fn setTearOffEnabled_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_setTearOffEnabled_0<(/*void*/)> for (bool) {
  fn setTearOffEnabled_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu17setTearOffEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:174
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isTearOffEnabled() const

/*

*/
impl /*struct*/ QMenu {
  pub fn isTearOffEnabled_0<RetType, T: QMenu_isTearOffEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTearOffEnabled_0(self);
    // return 1;
  }
}
pub trait QMenu_isTearOffEnabled_0<RetType> {
  fn isTearOffEnabled_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_isTearOffEnabled_0<bool> for () {
  fn isTearOffEnabled_0(self , rsthis: & QMenu) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu16isTearOffEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:176
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isTearOffMenuVisible() const

/*
When a menu is torn off a second menu is shown to display the menu contents in a new window. When the menu is in this mode and the menu is visible returns true; otherwise false.

See also showTearOffMenu(), hideTearOffMenu(), and isTearOffEnabled().
*/
impl /*struct*/ QMenu {
  pub fn isTearOffMenuVisible_0<RetType, T: QMenu_isTearOffMenuVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTearOffMenuVisible_0(self);
    // return 1;
  }
}
pub trait QMenu_isTearOffMenuVisible_0<RetType> {
  fn isTearOffMenuVisible_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_isTearOffMenuVisible_0<bool> for () {
  fn isTearOffMenuVisible_0(self , rsthis: & QMenu) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu20isTearOffMenuVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:177
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showTearOffMenu()

/*
This function will forcibly show the torn off menu making it appear on the user's desktop at the specified global position pos.

This function was introduced in  Qt 5.7.

See also hideTearOffMenu(), isTearOffMenuVisible(), and isTearOffEnabled().
*/
impl /*struct*/ QMenu {
  pub fn showTearOffMenu_0<RetType, T: QMenu_showTearOffMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showTearOffMenu_0(self);
    // return 1;
  }
}
pub trait QMenu_showTearOffMenu_0<RetType> {
  fn showTearOffMenu_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_showTearOffMenu_0<(/*void*/)> for () {
  fn showTearOffMenu_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN5QMenu15showTearOffMenuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:178
// index:1
// Public Visibility=Default Availability=Available
// [-2] void showTearOffMenu(const QPoint &)

/*
This function will forcibly show the torn off menu making it appear on the user's desktop at the specified global position pos.

This function was introduced in  Qt 5.7.

See also hideTearOffMenu(), isTearOffMenuVisible(), and isTearOffEnabled().
*/
impl /*struct*/ QMenu {
  pub fn showTearOffMenu_1<RetType, T: QMenu_showTearOffMenu_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showTearOffMenu_1(self);
    // return 1;
  }
}
pub trait QMenu_showTearOffMenu_1<RetType> {
  fn showTearOffMenu_1(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_showTearOffMenu_1<(/*void*/)> for (usize) {
  fn showTearOffMenu_1(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu15showTearOffMenuERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:179
// index:0
// Public Visibility=Default Availability=Available
// [-2] void hideTearOffMenu()

/*
This function will forcibly hide the torn off menu making it disappear from the user's desktop.

See also showTearOffMenu(), isTearOffMenuVisible(), and isTearOffEnabled().
*/
impl /*struct*/ QMenu {
  pub fn hideTearOffMenu_0<RetType, T: QMenu_hideTearOffMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideTearOffMenu_0(self);
    // return 1;
  }
}
pub trait QMenu_hideTearOffMenu_0<RetType> {
  fn hideTearOffMenu_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_hideTearOffMenu_0<(/*void*/)> for () {
  fn hideTearOffMenu_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN5QMenu15hideTearOffMenuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:181
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultAction(QAction *)

/*
This sets the default action to act. The default action may have a visual cue, depending on the current QStyle. A default action usually indicates what will happen by default when a drop occurs.

See also defaultAction().
*/
impl /*struct*/ QMenu {
  pub fn setDefaultAction_0<RetType, T: QMenu_setDefaultAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultAction_0(self);
    // return 1;
  }
}
pub trait QMenu_setDefaultAction_0<RetType> {
  fn setDefaultAction_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_setDefaultAction_0<(/*void*/)> for (usize) {
  fn setDefaultAction_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu16setDefaultActionEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:182
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * defaultAction() const

/*
Returns the current default action.

See also setDefaultAction().
*/
impl /*struct*/ QMenu {
  pub fn defaultAction_0<RetType, T: QMenu_defaultAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultAction_0(self);
    // return 1;
  }
}
pub trait QMenu_defaultAction_0<RetType> {
  fn defaultAction_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_defaultAction_0<usize> for () {
  fn defaultAction_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu13defaultActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:184
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setActiveAction(QAction *)

/*
Sets the currently highlighted action to act.

See also activeAction().
*/
impl /*struct*/ QMenu {
  pub fn setActiveAction_0<RetType, T: QMenu_setActiveAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setActiveAction_0(self);
    // return 1;
  }
}
pub trait QMenu_setActiveAction_0<RetType> {
  fn setActiveAction_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_setActiveAction_0<(/*void*/)> for (usize) {
  fn setActiveAction_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu15setActiveActionEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:185
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * activeAction() const

/*
Returns the currently highlighted action, or 0 if no action is currently highlighted.

See also setActiveAction().
*/
impl /*struct*/ QMenu {
  pub fn activeAction_0<RetType, T: QMenu_activeAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activeAction_0(self);
    // return 1;
  }
}
pub trait QMenu_activeAction_0<RetType> {
  fn activeAction_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_activeAction_0<usize> for () {
  fn activeAction_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu12activeActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void popup(const QPoint &, QAction *)

/*
Displays the menu so that the action atAction will be at the specified global position p. To translate a widget's local coordinates into global coordinates, use QWidget::mapToGlobal().

When positioning a menu with exec() or popup(), bear in mind that you cannot rely on the menu's current size(). For performance reasons, the menu adapts its size only when necessary, so in many cases, the size before and after the show is different. Instead, use sizeHint() which calculates the proper size depending on the menu's current contents.

See also QWidget::mapToGlobal() and exec().
*/
impl /*struct*/ QMenu {
  pub fn popup_0<RetType, T: QMenu_popup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.popup_0(self);
    // return 1;
  }
}
pub trait QMenu_popup_0<RetType> {
  fn popup_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_popup_0<(/*void*/)> for (usize,usize) {
  fn popup_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu5popupERK6QPointP7QAction", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:188
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * exec()

/*
Executes this menu synchronously.

This is equivalent to exec(pos()).

This returns the triggered QAction in either the popup menu or one of its submenus, or 0 if no item was triggered (normally because the user pressed Esc).

In most situations you'll want to specify the position yourself, for example, the current mouse position:


  exec(QCursor::pos());



or aligned to a widget:


  exec(somewidget.mapToGlobal(QPoint(0,0)));



or in reaction to a QMouseEvent *e:


  exec(e->globalPos());
*/
impl /*struct*/ QMenu {
  pub fn exec_0<RetType, T: QMenu_exec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exec_0(self);
    // return 1;
  }
}
pub trait QMenu_exec_0<RetType> {
  fn exec_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_exec_0<usize> for () {
  fn exec_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu4execEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:189
// index:1
// Public Visibility=Default Availability=Available
// [8] QAction * exec(const QPoint &, QAction *)

/*
Executes this menu synchronously.

This is equivalent to exec(pos()).

This returns the triggered QAction in either the popup menu or one of its submenus, or 0 if no item was triggered (normally because the user pressed Esc).

In most situations you'll want to specify the position yourself, for example, the current mouse position:


  exec(QCursor::pos());



or aligned to a widget:


  exec(somewidget.mapToGlobal(QPoint(0,0)));



or in reaction to a QMouseEvent *e:


  exec(e->globalPos());
*/
impl /*struct*/ QMenu {
  pub fn exec_1<RetType, T: QMenu_exec_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exec_1(self);
    // return 1;
  }
}
pub trait QMenu_exec_1<RetType> {
  fn exec_1(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_exec_1<usize> for (usize,usize) {
  fn exec_1(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu4execERK6QPointP7QAction", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:197
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QMenu {
  pub fn sizeHint_0<RetType, T: QMenu_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QMenu_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:199
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect actionGeometry(QAction *) const

/*
Returns the geometry of action act.
*/
impl /*struct*/ QMenu {
  pub fn actionGeometry_0<RetType, T: QMenu_actionGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionGeometry_0(self);
    // return 1;
  }
}
pub trait QMenu_actionGeometry_0<RetType> {
  fn actionGeometry_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_actionGeometry_0<usize> for (usize) {
  fn actionGeometry_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu14actionGeometryEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:200
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * actionAt(const QPoint &) const

/*
Returns the item at pt; returns 0 if there is no item there.
*/
impl /*struct*/ QMenu {
  pub fn actionAt_0<RetType, T: QMenu_actionAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionAt_0(self);
    // return 1;
  }
}
pub trait QMenu_actionAt_0<RetType> {
  fn actionAt_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_actionAt_0<usize> for (usize) {
  fn actionAt_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu8actionAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:202
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * menuAction() const

/*
Returns the action associated with this menu.
*/
impl /*struct*/ QMenu {
  pub fn menuAction_0<RetType, T: QMenu_menuAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.menuAction_0(self);
    // return 1;
  }
}
pub trait QMenu_menuAction_0<RetType> {
  fn menuAction_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_menuAction_0<usize> for () {
  fn menuAction_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu10menuActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:204
// index:0
// Public Visibility=Default Availability=Available
// [8] QString title() const

/*

*/
impl /*struct*/ QMenu {
  pub fn title_0<RetType, T: QMenu_title_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.title_0(self);
    // return 1;
  }
}
pub trait QMenu_title_0<RetType> {
  fn title_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_title_0<usize> for () {
  fn title_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu5titleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:205
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTitle(const QString &)

/*

*/
impl /*struct*/ QMenu {
  pub fn setTitle_0<RetType, T: QMenu_setTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTitle_0(self);
    // return 1;
  }
}
pub trait QMenu_setTitle_0<RetType> {
  fn setTitle_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_setTitle_0<(/*void*/)> for (usize) {
  fn setTitle_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu8setTitleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:207
// index:0
// Public Visibility=Default Availability=Available
// [8] QIcon icon() const

/*

*/
impl /*struct*/ QMenu {
  pub fn icon_0<RetType, T: QMenu_icon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.icon_0(self);
    // return 1;
  }
}
pub trait QMenu_icon_0<RetType> {
  fn icon_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_icon_0<usize> for () {
  fn icon_0(self , rsthis: & QMenu) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu4iconEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:208
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIcon(const QIcon &)

/*

*/
impl /*struct*/ QMenu {
  pub fn setIcon_0<RetType, T: QMenu_setIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIcon_0(self);
    // return 1;
  }
}
pub trait QMenu_setIcon_0<RetType> {
  fn setIcon_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_setIcon_0<(/*void*/)> for (usize) {
  fn setIcon_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu7setIconERK5QIcon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:210
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNoReplayFor(QWidget *)

/*

*/
impl /*struct*/ QMenu {
  pub fn setNoReplayFor_0<RetType, T: QMenu_setNoReplayFor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNoReplayFor_0(self);
    // return 1;
  }
}
pub trait QMenu_setNoReplayFor_0<RetType> {
  fn setNoReplayFor_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_setNoReplayFor_0<(/*void*/)> for (usize) {
  fn setNoReplayFor_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu14setNoReplayForEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:219
// index:0
// Public Visibility=Default Availability=Available
// [1] bool separatorsCollapsible() const

/*

*/
impl /*struct*/ QMenu {
  pub fn separatorsCollapsible_0<RetType, T: QMenu_separatorsCollapsible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.separatorsCollapsible_0(self);
    // return 1;
  }
}
pub trait QMenu_separatorsCollapsible_0<RetType> {
  fn separatorsCollapsible_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_separatorsCollapsible_0<bool> for () {
  fn separatorsCollapsible_0(self , rsthis: & QMenu) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu21separatorsCollapsibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:220
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSeparatorsCollapsible(bool)

/*

*/
impl /*struct*/ QMenu {
  pub fn setSeparatorsCollapsible_0<RetType, T: QMenu_setSeparatorsCollapsible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSeparatorsCollapsible_0(self);
    // return 1;
  }
}
pub trait QMenu_setSeparatorsCollapsible_0<RetType> {
  fn setSeparatorsCollapsible_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_setSeparatorsCollapsible_0<(/*void*/)> for (bool) {
  fn setSeparatorsCollapsible_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu24setSeparatorsCollapsibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:222
// index:0
// Public Visibility=Default Availability=Available
// [1] bool toolTipsVisible() const

/*

*/
impl /*struct*/ QMenu {
  pub fn toolTipsVisible_0<RetType, T: QMenu_toolTipsVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolTipsVisible_0(self);
    // return 1;
  }
}
pub trait QMenu_toolTipsVisible_0<RetType> {
  fn toolTipsVisible_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_toolTipsVisible_0<bool> for () {
  fn toolTipsVisible_0(self , rsthis: & QMenu) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu15toolTipsVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:223
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setToolTipsVisible(bool)

/*

*/
impl /*struct*/ QMenu {
  pub fn setToolTipsVisible_0<RetType, T: QMenu_setToolTipsVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolTipsVisible_0(self);
    // return 1;
  }
}
pub trait QMenu_setToolTipsVisible_0<RetType> {
  fn setToolTipsVisible_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_setToolTipsVisible_0<(/*void*/)> for (bool) {
  fn setToolTipsVisible_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu18setToolTipsVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:226
// index:0
// Public Visibility=Default Availability=Available
// [-2] void aboutToShow()

/*
This signal is emitted just before the menu is shown to the user.

See also aboutToHide() and show().
*/
impl /*struct*/ QMenu {
  pub fn aboutToShow_0<RetType, T: QMenu_aboutToShow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.aboutToShow_0(self);
    // return 1;
  }
}
pub trait QMenu_aboutToShow_0<RetType> {
  fn aboutToShow_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_aboutToShow_0<(/*void*/)> for () {
  fn aboutToShow_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN5QMenu11aboutToShowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:227
// index:0
// Public Visibility=Default Availability=Available
// [-2] void aboutToHide()

/*
This signal is emitted just before the menu is hidden from the user.

This function was introduced in  Qt 4.2.

See also aboutToShow() and hide().
*/
impl /*struct*/ QMenu {
  pub fn aboutToHide_0<RetType, T: QMenu_aboutToHide_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.aboutToHide_0(self);
    // return 1;
  }
}
pub trait QMenu_aboutToHide_0<RetType> {
  fn aboutToHide_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_aboutToHide_0<(/*void*/)> for () {
  fn aboutToHide_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN5QMenu11aboutToHideEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:228
// index:0
// Public Visibility=Default Availability=Available
// [-2] void triggered(QAction *)

/*
This signal is emitted when an action in this menu is triggered.

action is the action that caused the signal to be emitted.

Normally, you connect each menu action's triggered() signal to its own custom slot, but sometimes you will want to connect several actions to a single slot, for example, when you have a group of closely related actions, such as "left justify", "center", "right justify".

Note: This signal is emitted for the main parent menu in a hierarchy. Hence, only the parent menu needs to be connected to a slot; sub-menus need not be connected.

See also hovered() and QAction::triggered().
*/
impl /*struct*/ QMenu {
  pub fn triggered_0<RetType, T: QMenu_triggered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.triggered_0(self);
    // return 1;
  }
}
pub trait QMenu_triggered_0<RetType> {
  fn triggered_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_triggered_0<(/*void*/)> for (usize) {
  fn triggered_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu9triggeredEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:229
// index:0
// Public Visibility=Default Availability=Available
// [-2] void hovered(QAction *)

/*
This signal is emitted when a menu action is highlighted; action is the action that caused the signal to be emitted.

Often this is used to update status information.

See also triggered() and QAction::hovered().
*/
impl /*struct*/ QMenu {
  pub fn hovered_0<RetType, T: QMenu_hovered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hovered_0(self);
    // return 1;
  }
}
pub trait QMenu_hovered_0<RetType> {
  fn hovered_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_hovered_0<(/*void*/)> for (usize) {
  fn hovered_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu7hoveredEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:232
// index:0
// Protected Visibility=Default Availability=Available
// [4] int columnCount() const

/*
If a menu does not fit on the screen it lays itself out so that it does fit. It is style dependent what layout means (for example, on Windows it will use multiple columns).

This functions returns the number of columns necessary.
*/
impl /*struct*/ QMenu {
  pub fn columnCount_0<RetType, T: QMenu_columnCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnCount_0(self);
    // return 1;
  }
}
pub trait QMenu_columnCount_0<RetType> {
  fn columnCount_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_columnCount_0<i32> for () {
  fn columnCount_0(self , rsthis: & QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QMenu11columnCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:234
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QMenu {
  pub fn changeEvent_0<RetType, T: QMenu_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QMenu_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:235
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QMenu {
  pub fn keyPressEvent_0<RetType, T: QMenu_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QMenu_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:236
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QMenu {
  pub fn mouseReleaseEvent_0<RetType, T: QMenu_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QMenu_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:237
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QMenu {
  pub fn mousePressEvent_0<RetType, T: QMenu_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QMenu_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:238
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QMenu {
  pub fn mouseMoveEvent_0<RetType, T: QMenu_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QMenu_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:240
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
Reimplemented from QWidget::wheelEvent().
*/
impl /*struct*/ QMenu {
  pub fn wheelEvent_0<RetType, T: QMenu_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QMenu_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:242
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void enterEvent(QEvent *)

/*
Reimplemented from QWidget::enterEvent().
*/
impl /*struct*/ QMenu {
  pub fn enterEvent_0<RetType, T: QMenu_enterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.enterEvent_0(self);
    // return 1;
  }
}
pub trait QMenu_enterEvent_0<RetType> {
  fn enterEvent_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_enterEvent_0<(/*void*/)> for (usize) {
  fn enterEvent_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu10enterEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:243
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void leaveEvent(QEvent *)

/*
Reimplemented from QWidget::leaveEvent().
*/
impl /*struct*/ QMenu {
  pub fn leaveEvent_0<RetType, T: QMenu_leaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leaveEvent_0(self);
    // return 1;
  }
}
pub trait QMenu_leaveEvent_0<RetType> {
  fn leaveEvent_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_leaveEvent_0<(/*void*/)> for (usize) {
  fn leaveEvent_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu10leaveEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:244
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hideEvent(QHideEvent *)

/*
Reimplemented from QWidget::hideEvent().
*/
impl /*struct*/ QMenu {
  pub fn hideEvent_0<RetType, T: QMenu_hideEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideEvent_0(self);
    // return 1;
  }
}
pub trait QMenu_hideEvent_0<RetType> {
  fn hideEvent_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_hideEvent_0<(/*void*/)> for (usize) {
  fn hideEvent_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu9hideEventEP10QHideEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:245
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QMenu {
  pub fn paintEvent_0<RetType, T: QMenu_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QMenu_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:246
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void actionEvent(QActionEvent *)

/*
Reimplemented from QWidget::actionEvent().
*/
impl /*struct*/ QMenu {
  pub fn actionEvent_0<RetType, T: QMenu_actionEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionEvent_0(self);
    // return 1;
  }
}
pub trait QMenu_actionEvent_0<RetType> {
  fn actionEvent_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_actionEvent_0<(/*void*/)> for (usize) {
  fn actionEvent_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu11actionEventEP12QActionEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:247
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QMenu {
  pub fn timerEvent_0<RetType, T: QMenu_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QMenu_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QMenu10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:248
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QMenu {
  pub fn event_0<RetType, T: QMenu_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QMenu_event_0<RetType> {
  fn event_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QMenu) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:249
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool focusNextPrevChild(bool)

/*
Reimplemented from QWidget::focusNextPrevChild().
*/
impl /*struct*/ QMenu {
  pub fn focusNextPrevChild_0<RetType, T: QMenu_focusNextPrevChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild_0(self);
    // return 1;
  }
}
pub trait QMenu_focusNextPrevChild_0<RetType> {
  fn focusNextPrevChild_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_focusNextPrevChild_0<bool> for (bool) {
  fn focusNextPrevChild_0(self , rsthis: & QMenu) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QMenu18focusNextPrevChildEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmenu.h:250
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionMenuItem *, const QAction *) const

/*
Initialize option with the values from this menu and information from action. This method is useful for subclasses when they need a QStyleOptionMenuItem, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom() and QMenuBar::initStyleOption().
*/
impl /*struct*/ QMenu {
  pub fn initStyleOption_0<RetType, T: QMenu_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QMenu_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QMenu) -> RetType;
}
impl<'a> /*trait*/ QMenu_initStyleOption_0<(/*void*/)> for (usize,usize) {
  fn initStyleOption_0(self , rsthis: & QMenu) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK5QMenu15initStyleOptionEP20QStyleOptionMenuItemPK7QAction", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
