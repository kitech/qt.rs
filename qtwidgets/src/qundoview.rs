

// mod ::widgets::QUndoView
// package qtwidgets
// /usr/include/qt/QtWidgets/qundoview.h
// #include <qundoview.h>
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



/*

*/
#[derive(Default)] // class sizeof(QUndoView)=48
pub struct QUndoView {
  qbase: QListView,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QUndoView_ITF interface {
//    QListView_ITF
//    QUndoView_PTR() *QUndoView
//}
//func (ptr *QUndoView) QUndoView_PTR() *QUndoView { return ptr }

impl /*struct*/ QUndoView {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QUndoView {
    return QUndoView{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QUndoView {
//  type Target = QUndoViewBASE;
//
//  fn deref(&self) -> &QUndoViewBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QUndoViewBASE> for QUndoView {
//  fn as_ref(& self) -> & QUndoViewBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qundoview.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QUndoView {
  pub fn metaObject_0<RetType, T: QUndoView_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QUndoView_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QUndoView) -> RetType;
}
impl<'a> /*trait*/ QUndoView_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QUndoView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QUndoView10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundoview.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QUndoView(QWidget *)

/*
Constructs a new view with parent parent.
*/
// QUndoView(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QUndoView {
  pub fn QUndoView_0<T: QUndoView_QUndoView_0>(value: T) -> QUndoView {
    let rsthis = value.QUndoView_0();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoView_QUndoView_0 {
  fn QUndoView_0(self) -> QUndoView;
}
// QUndoView(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUndoView_QUndoView_0 for (usize) {
  fn QUndoView_0(self) -> QUndoView {
    // unsafe{_ZN9QUndoViewC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QUndoViewC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUndoView{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundoview.h:66
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QUndoView(QUndoStack *, QWidget *)

/*
Constructs a new view with parent parent.
*/
// QUndoView(QUndoStack *, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QUndoView {
  pub fn QUndoView_1<T: QUndoView_QUndoView_1>(value: T) -> QUndoView {
    let rsthis = value.QUndoView_1();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoView_QUndoView_1 {
  fn QUndoView_1(self) -> QUndoView;
}
// QUndoView(QUndoStack *, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUndoView_QUndoView_1 for (usize,usize) {
  fn QUndoView_1(self) -> QUndoView {
    // unsafe{_ZN9QUndoViewC2EP10QUndoStackP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QUndoViewC2EP10QUndoStackP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUndoView{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundoview.h:68
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QUndoView(QUndoGroup *, QWidget *)

/*
Constructs a new view with parent parent.
*/
// QUndoView(QUndoGroup *, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QUndoView {
  pub fn QUndoView_2<T: QUndoView_QUndoView_2>(value: T) -> QUndoView {
    let rsthis = value.QUndoView_2();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoView_QUndoView_2 {
  fn QUndoView_2(self) -> QUndoView;
}
// QUndoView(QUndoGroup *, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUndoView_QUndoView_2 for (usize,usize) {
  fn QUndoView_2(self) -> QUndoView {
    // unsafe{_ZN9QUndoViewC2EP10QUndoGroupP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QUndoViewC2EP10QUndoGroupP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUndoView{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundoview.h:70
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QUndoView()

/*

*/
pub fn DeleteQUndoView(this :*mut QUndoView) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QUndoViewD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qundoview.h:72
// index:0
// Public Visibility=Default Availability=Available
// [8] QUndoStack * stack() const

/*
Returns the stack currently displayed by this view. If the view is looking at a QUndoGroup, this the group's active stack.

See also setStack() and setGroup().
*/
impl /*struct*/ QUndoView {
  pub fn stack_0<RetType, T: QUndoView_stack_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stack_0(self);
    // return 1;
  }
}
pub trait QUndoView_stack_0<RetType> {
  fn stack_0(self , rsthis: & QUndoView) -> RetType;
}
impl<'a> /*trait*/ QUndoView_stack_0<usize> for () {
  fn stack_0(self , rsthis: & QUndoView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QUndoView5stackEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundoview.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QUndoGroup * group() const

/*
Returns the group displayed by this view.

If the view is not looking at group, this function returns 0.

See also setGroup() and setStack().
*/
impl /*struct*/ QUndoView {
  pub fn group_0<RetType, T: QUndoView_group_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.group_0(self);
    // return 1;
  }
}
pub trait QUndoView_group_0<RetType> {
  fn group_0(self , rsthis: & QUndoView) -> RetType;
}
impl<'a> /*trait*/ QUndoView_group_0<usize> for () {
  fn group_0(self , rsthis: & QUndoView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QUndoView5groupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundoview.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEmptyLabel(const QString &)

/*

*/
impl /*struct*/ QUndoView {
  pub fn setEmptyLabel_0<RetType, T: QUndoView_setEmptyLabel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEmptyLabel_0(self);
    // return 1;
  }
}
pub trait QUndoView_setEmptyLabel_0<RetType> {
  fn setEmptyLabel_0(self , rsthis: & QUndoView) -> RetType;
}
impl<'a> /*trait*/ QUndoView_setEmptyLabel_0<(/*void*/)> for (usize) {
  fn setEmptyLabel_0(self , rsthis: & QUndoView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QUndoView13setEmptyLabelERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundoview.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QString emptyLabel() const

/*

*/
impl /*struct*/ QUndoView {
  pub fn emptyLabel_0<RetType, T: QUndoView_emptyLabel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.emptyLabel_0(self);
    // return 1;
  }
}
pub trait QUndoView_emptyLabel_0<RetType> {
  fn emptyLabel_0(self , rsthis: & QUndoView) -> RetType;
}
impl<'a> /*trait*/ QUndoView_emptyLabel_0<usize> for () {
  fn emptyLabel_0(self , rsthis: & QUndoView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QUndoView10emptyLabelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundoview.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCleanIcon(const QIcon &)

/*

*/
impl /*struct*/ QUndoView {
  pub fn setCleanIcon_0<RetType, T: QUndoView_setCleanIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCleanIcon_0(self);
    // return 1;
  }
}
pub trait QUndoView_setCleanIcon_0<RetType> {
  fn setCleanIcon_0(self , rsthis: & QUndoView) -> RetType;
}
impl<'a> /*trait*/ QUndoView_setCleanIcon_0<(/*void*/)> for (usize) {
  fn setCleanIcon_0(self , rsthis: & QUndoView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QUndoView12setCleanIconERK5QIcon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundoview.h:81
// index:0
// Public Visibility=Default Availability=Available
// [8] QIcon cleanIcon() const

/*

*/
impl /*struct*/ QUndoView {
  pub fn cleanIcon_0<RetType, T: QUndoView_cleanIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cleanIcon_0(self);
    // return 1;
  }
}
pub trait QUndoView_cleanIcon_0<RetType> {
  fn cleanIcon_0(self , rsthis: & QUndoView) -> RetType;
}
impl<'a> /*trait*/ QUndoView_cleanIcon_0<usize> for () {
  fn cleanIcon_0(self , rsthis: & QUndoView) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QUndoView9cleanIconEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundoview.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStack(QUndoStack *)

/*
Sets the stack displayed by this view to stack. If stack is 0, the view will be empty.

If the view was previously looking at a QUndoGroup, the group is set to 0.

See also stack() and setGroup().
*/
impl /*struct*/ QUndoView {
  pub fn setStack_0<RetType, T: QUndoView_setStack_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStack_0(self);
    // return 1;
  }
}
pub trait QUndoView_setStack_0<RetType> {
  fn setStack_0(self , rsthis: & QUndoView) -> RetType;
}
impl<'a> /*trait*/ QUndoView_setStack_0<(/*void*/)> for (usize) {
  fn setStack_0(self , rsthis: & QUndoView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QUndoView8setStackEP10QUndoStack", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundoview.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGroup(QUndoGroup *)

/*
Sets the group displayed by this view to group. If group is 0, the view will be empty.

The view will update itself autmiatically whenever the active stack of the group changes.

See also group() and setStack().
*/
impl /*struct*/ QUndoView {
  pub fn setGroup_0<RetType, T: QUndoView_setGroup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGroup_0(self);
    // return 1;
  }
}
pub trait QUndoView_setGroup_0<RetType> {
  fn setGroup_0(self , rsthis: & QUndoView) -> RetType;
}
impl<'a> /*trait*/ QUndoView_setGroup_0<(/*void*/)> for (usize) {
  fn setGroup_0(self , rsthis: & QUndoView) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QUndoView8setGroupEP10QUndoGroup", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
