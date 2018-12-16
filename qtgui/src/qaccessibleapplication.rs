

// mod ::gui::QAccessibleApplication
// package qtgui
// /usr/include/qt/QtGui/qaccessibleobject.h
// #include <qaccessibleobject.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 4
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QAccessibleApplication)=16
pub struct QAccessibleApplication {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleApplication_ITF interface {
//    QAccessibleApplication_PTR() *QAccessibleApplication
//}
//func (ptr *QAccessibleApplication) QAccessibleApplication_PTR() *QAccessibleApplication { return ptr }

impl /*struct*/ QAccessibleApplication {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleApplication {
    return QAccessibleApplication{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleApplication {
//  type Target = QAccessibleApplicationBASE;
//
//  fn deref(&self) -> &QAccessibleApplicationBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleApplicationBASE> for QAccessibleApplication {
//  fn as_ref(& self) -> & QAccessibleApplicationBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessibleobject.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAccessibleApplication()

/*

*/
// QAccessibleApplication() ctx.fn_proto_cpp
impl /*struct*/ QAccessibleApplication {
  pub fn QAccessibleApplication_0<T: QAccessibleApplication_QAccessibleApplication_0>(value: T) -> QAccessibleApplication {
    let rsthis = value.QAccessibleApplication_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleApplication_QAccessibleApplication_0 {
  fn QAccessibleApplication_0(self) -> QAccessibleApplication;
}
// QAccessibleApplication() ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleApplication_QAccessibleApplication_0 for () {
  fn QAccessibleApplication_0(self) -> QAccessibleApplication {
    // unsafe{_ZN22QAccessibleApplicationC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QAccessibleApplicationC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleApplication{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessibleobject.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QWindow * window() const

/*

*/
impl /*struct*/ QAccessibleApplication {
  pub fn window_0<RetType, T: QAccessibleApplication_window_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.window_0(self);
    // return 1;
  }
}
pub trait QAccessibleApplication_window_0<RetType> {
  fn window_0(self , rsthis: & QAccessibleApplication) -> RetType;
}
impl<'a> /*trait*/ QAccessibleApplication_window_0<usize> for () {
  fn window_0(self , rsthis: & QAccessibleApplication) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QAccessibleApplication6windowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessibleobject.h:82
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int childCount() const

/*

*/
impl /*struct*/ QAccessibleApplication {
  pub fn childCount_0<RetType, T: QAccessibleApplication_childCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childCount_0(self);
    // return 1;
  }
}
pub trait QAccessibleApplication_childCount_0<RetType> {
  fn childCount_0(self , rsthis: & QAccessibleApplication) -> RetType;
}
impl<'a> /*trait*/ QAccessibleApplication_childCount_0<i32> for () {
  fn childCount_0(self , rsthis: & QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QAccessibleApplication10childCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessibleobject.h:83
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int indexOfChild(const QAccessibleInterface *) const

/*

*/
impl /*struct*/ QAccessibleApplication {
  pub fn indexOfChild_0<RetType, T: QAccessibleApplication_indexOfChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfChild_0(self);
    // return 1;
  }
}
pub trait QAccessibleApplication_indexOfChild_0<RetType> {
  fn indexOfChild_0(self , rsthis: & QAccessibleApplication) -> RetType;
}
impl<'a> /*trait*/ QAccessibleApplication_indexOfChild_0<i32> for (usize) {
  fn indexOfChild_0(self , rsthis: & QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QAccessibleApplication12indexOfChildEPK20QAccessibleInterface", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessibleobject.h:84
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * focusChild() const

/*

*/
impl /*struct*/ QAccessibleApplication {
  pub fn focusChild_0<RetType, T: QAccessibleApplication_focusChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusChild_0(self);
    // return 1;
  }
}
pub trait QAccessibleApplication_focusChild_0<RetType> {
  fn focusChild_0(self , rsthis: & QAccessibleApplication) -> RetType;
}
impl<'a> /*trait*/ QAccessibleApplication_focusChild_0<usize> for () {
  fn focusChild_0(self , rsthis: & QAccessibleApplication) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QAccessibleApplication10focusChildEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessibleobject.h:87
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * parent() const

/*

*/
impl /*struct*/ QAccessibleApplication {
  pub fn parent_0<RetType, T: QAccessibleApplication_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QAccessibleApplication_parent_0<RetType> {
  fn parent_0(self , rsthis: & QAccessibleApplication) -> RetType;
}
impl<'a> /*trait*/ QAccessibleApplication_parent_0<usize> for () {
  fn parent_0(self , rsthis: & QAccessibleApplication) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QAccessibleApplication6parentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessibleobject.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * child(int) const

/*

*/
impl /*struct*/ QAccessibleApplication {
  pub fn child_0<RetType, T: QAccessibleApplication_child_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.child_0(self);
    // return 1;
  }
}
pub trait QAccessibleApplication_child_0<RetType> {
  fn child_0(self , rsthis: & QAccessibleApplication) -> RetType;
}
impl<'a> /*trait*/ QAccessibleApplication_child_0<usize> for (i32) {
  fn child_0(self , rsthis: & QAccessibleApplication) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QAccessibleApplication5childEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessibleobject.h:91
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString text(QAccessible::Text) const

/*

*/
impl /*struct*/ QAccessibleApplication {
  pub fn text_0<RetType, T: QAccessibleApplication_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QAccessibleApplication_text_0<RetType> {
  fn text_0(self , rsthis: & QAccessibleApplication) -> RetType;
}
impl<'a> /*trait*/ QAccessibleApplication_text_0<usize> for (i32) {
  fn text_0(self , rsthis: & QAccessibleApplication) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QAccessibleApplication4textEN11QAccessible4TextE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessibleobject.h:92
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QAccessible::Role role() const

/*

*/
impl /*struct*/ QAccessibleApplication {
  pub fn role_0<RetType, T: QAccessibleApplication_role_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.role_0(self);
    // return 1;
  }
}
pub trait QAccessibleApplication_role_0<RetType> {
  fn role_0(self , rsthis: & QAccessibleApplication) -> RetType;
}
impl<'a> /*trait*/ QAccessibleApplication_role_0<i32> for () {
  fn role_0(self , rsthis: & QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QAccessibleApplication4roleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessibleobject.h:93
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QAccessible::State state() const

/*

*/
impl /*struct*/ QAccessibleApplication {
  pub fn state_0<RetType, T: QAccessibleApplication_state_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.state_0(self);
    // return 1;
  }
}
pub trait QAccessibleApplication_state_0<RetType> {
  fn state_0(self , rsthis: & QAccessibleApplication) -> RetType;
}
impl<'a> /*trait*/ QAccessibleApplication_state_0<i32> for () {
  fn state_0(self , rsthis: & QAccessibleApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QAccessibleApplication5stateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQAccessibleApplication(this :*mut QAccessibleApplication) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN22QAccessibleApplicationD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
