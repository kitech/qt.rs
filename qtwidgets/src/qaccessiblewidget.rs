

// mod ::widgets::QAccessibleWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qaccessiblewidget.h
// #include <qaccessiblewidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 138
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

// QWidget * widget()
// func (this *QAccessibleWidget) InheritWidget(f func() unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "widget", f)
// }

// QObject * parentObject()
// func (this *QAccessibleWidget) InheritParentObject(f func() unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "parentObject", f)
// }

// void addControllingSignal(const QString &)
// func (this *QAccessibleWidget) InheritAddControllingSignal(f func(signal string) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "addControllingSignal", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAccessibleWidget)=32
pub struct QAccessibleWidget {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleWidget_ITF interface {
//    QAccessibleWidget_PTR() *QAccessibleWidget
//}
//func (ptr *QAccessibleWidget) QAccessibleWidget_PTR() *QAccessibleWidget { return ptr }

impl /*struct*/ QAccessibleWidget {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleWidget {
    return QAccessibleWidget{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleWidget {
//  type Target = QAccessibleWidgetBASE;
//
//  fn deref(&self) -> &QAccessibleWidgetBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleWidgetBASE> for QAccessibleWidget {
//  fn as_ref(& self) -> & QAccessibleWidgetBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qaccessiblewidget.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAccessibleWidget(QWidget *, QAccessible::Role, const QString &)

/*
Creates a QAccessibleWidget object for widget w. role and name are optional parameters that set the object's role and name properties.
*/
// QAccessibleWidget(QWidget *, QAccessible::Role, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleWidget {
  pub fn QAccessibleWidget_0<T: QAccessibleWidget_QAccessibleWidget_0>(value: T) -> QAccessibleWidget {
    let rsthis = value.QAccessibleWidget_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleWidget_QAccessibleWidget_0 {
  fn QAccessibleWidget_0(self) -> QAccessibleWidget;
}
// QAccessibleWidget(QWidget *, QAccessible::Role, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleWidget_QAccessibleWidget_0 for (usize,i32,usize) {
  fn QAccessibleWidget_0(self) -> QAccessibleWidget {
    // unsafe{_ZN17QAccessibleWidgetC2EP7QWidgetN11QAccessible4RoleERK7QString()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QAccessibleWidgetC2EP7QWidgetN11QAccessible4RoleERK7QString", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Reimplemented from QAccessibleInterface::isValid().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn isValid_0<RetType, T: QAccessibleWidget_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QAccessibleWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QWindow * window() const

/*
Reimplemented from QAccessibleInterface::window().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn window_0<RetType, T: QAccessibleWidget_window_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.window_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_window_0<RetType> {
  fn window_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_window_0<usize> for () {
  fn window_0(self , rsthis: & QAccessibleWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget6windowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int childCount() const

/*
Reimplemented from QAccessibleInterface::childCount().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn childCount_0<RetType, T: QAccessibleWidget_childCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childCount_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_childCount_0<RetType> {
  fn childCount_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_childCount_0<i32> for () {
  fn childCount_0(self , rsthis: & QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget10childCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int indexOfChild(const QAccessibleInterface *) const

/*
Reimplemented from QAccessibleInterface::indexOfChild().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn indexOfChild_0<RetType, T: QAccessibleWidget_indexOfChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfChild_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_indexOfChild_0<RetType> {
  fn indexOfChild_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_indexOfChild_0<i32> for (usize) {
  fn indexOfChild_0(self , rsthis: & QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget12indexOfChildEPK20QAccessibleInterface", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * focusChild() const

/*
Reimplemented from QAccessibleInterface::focusChild().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn focusChild_0<RetType, T: QAccessibleWidget_focusChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusChild_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_focusChild_0<RetType> {
  fn focusChild_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_focusChild_0<usize> for () {
  fn focusChild_0(self , rsthis: & QAccessibleWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget10focusChildEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect rect() const

/*
Reimplemented from QAccessibleInterface::rect().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn rect_0<RetType, T: QAccessibleWidget_rect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rect_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_rect_0<RetType> {
  fn rect_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_rect_0<usize> for () {
  fn rect_0(self , rsthis: & QAccessibleWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget4rectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * parent() const

/*
Reimplemented from QAccessibleInterface::parent().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn parent_0<RetType, T: QAccessibleWidget_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_parent_0<RetType> {
  fn parent_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_parent_0<usize> for () {
  fn parent_0(self , rsthis: & QAccessibleWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget6parentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:68
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * child(int) const

/*
Reimplemented from QAccessibleInterface::child().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn child_0<RetType, T: QAccessibleWidget_child_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.child_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_child_0<RetType> {
  fn child_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_child_0<usize> for (i32) {
  fn child_0(self , rsthis: & QAccessibleWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget5childEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:70
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString text(QAccessible::Text) const

/*
Reimplemented from QAccessibleInterface::text().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn text_0<RetType, T: QAccessibleWidget_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_text_0<RetType> {
  fn text_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_text_0<usize> for (i32) {
  fn text_0(self , rsthis: & QAccessibleWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget4textEN11QAccessible4TextE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QAccessible::Role role() const

/*
Reimplemented from QAccessibleInterface::role().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn role_0<RetType, T: QAccessibleWidget_role_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.role_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_role_0<RetType> {
  fn role_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_role_0<i32> for () {
  fn role_0(self , rsthis: & QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget4roleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QAccessible::State state() const

/*
Reimplemented from QAccessibleInterface::state().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn state_0<RetType, T: QAccessibleWidget_state_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.state_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_state_0<RetType> {
  fn state_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_state_0<i32> for () {
  fn state_0(self , rsthis: & QAccessibleWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget5stateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QColor foregroundColor() const

/*
Reimplemented from QAccessibleInterface::foregroundColor().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn foregroundColor_0<RetType, T: QAccessibleWidget_foregroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.foregroundColor_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_foregroundColor_0<RetType> {
  fn foregroundColor_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_foregroundColor_0<usize> for () {
  fn foregroundColor_0(self , rsthis: & QAccessibleWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget15foregroundColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:75
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QColor backgroundColor() const

/*
Reimplemented from QAccessibleInterface::backgroundColor().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn backgroundColor_0<RetType, T: QAccessibleWidget_backgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backgroundColor_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_backgroundColor_0<RetType> {
  fn backgroundColor_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_backgroundColor_0<usize> for () {
  fn backgroundColor_0(self , rsthis: & QAccessibleWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget15backgroundColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:77
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] void * interface_cast(QAccessible::InterfaceType)

/*
Reimplemented from QAccessibleInterface::interface_cast().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn interface_cast_0<RetType, T: QAccessibleWidget_interface_cast_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.interface_cast_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_interface_cast_0<RetType> {
  fn interface_cast_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_interface_cast_0<usize> for (i32) {
  fn interface_cast_0(self , rsthis: & QAccessibleWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QAccessibleWidget14interface_castEN11QAccessible13InterfaceTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QStringList actionNames() const

/*
Reimplemented from QAccessibleActionInterface::actionNames().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn actionNames_0<RetType, T: QAccessibleWidget_actionNames_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionNames_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_actionNames_0<RetType> {
  fn actionNames_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_actionNames_0<usize> for () {
  fn actionNames_0(self , rsthis: & QAccessibleWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget11actionNamesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:81
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void doAction(const QString &)

/*
Reimplemented from QAccessibleActionInterface::doAction().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn doAction_0<RetType, T: QAccessibleWidget_doAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doAction_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_doAction_0<RetType> {
  fn doAction_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_doAction_0<(/*void*/)> for (usize) {
  fn doAction_0(self , rsthis: & QAccessibleWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAccessibleWidget8doActionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:82
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QStringList keyBindingsForAction(const QString &) const

/*
Reimplemented from QAccessibleActionInterface::keyBindingsForAction().
*/
impl /*struct*/ QAccessibleWidget {
  pub fn keyBindingsForAction_0<RetType, T: QAccessibleWidget_keyBindingsForAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyBindingsForAction_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_keyBindingsForAction_0<RetType> {
  fn keyBindingsForAction_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_keyBindingsForAction_0<usize> for (usize) {
  fn keyBindingsForAction_0(self , rsthis: & QAccessibleWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget20keyBindingsForActionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:84
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleWidget()

/*

*/
pub fn DeleteQAccessibleWidget(this :*mut QAccessibleWidget) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QAccessibleWidgetD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qaccessiblewidget.h:85
// index:0
// Protected Visibility=Default Availability=Available
// [8] QWidget * widget() const

/*
Returns the associated widget.
*/
impl /*struct*/ QAccessibleWidget {
  pub fn widget_0<RetType, T: QAccessibleWidget_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_widget_0<RetType> {
  fn widget_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_widget_0<usize> for () {
  fn widget_0(self , rsthis: & QAccessibleWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget6widgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:86
// index:0
// Protected Visibility=Default Availability=Available
// [8] QObject * parentObject() const

/*
Returns the associated widget's parent object, which is either the parent widget, or qApp for top-level widgets.
*/
impl /*struct*/ QAccessibleWidget {
  pub fn parentObject_0<RetType, T: QAccessibleWidget_parentObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parentObject_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_parentObject_0<RetType> {
  fn parentObject_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_parentObject_0<usize> for () {
  fn parentObject_0(self , rsthis: & QAccessibleWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessibleWidget12parentObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaccessiblewidget.h:88
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void addControllingSignal(const QString &)

/*
Registers signal as a controlling signal.

An object is a Controller to any other object connected to a controlling signal.
*/
impl /*struct*/ QAccessibleWidget {
  pub fn addControllingSignal_0<RetType, T: QAccessibleWidget_addControllingSignal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addControllingSignal_0(self);
    // return 1;
  }
}
pub trait QAccessibleWidget_addControllingSignal_0<RetType> {
  fn addControllingSignal_0(self , rsthis: & QAccessibleWidget) -> RetType;
}
impl<'a> /*trait*/ QAccessibleWidget_addControllingSignal_0<(/*void*/)> for (usize) {
  fn addControllingSignal_0(self , rsthis: & QAccessibleWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAccessibleWidget20addControllingSignalERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
