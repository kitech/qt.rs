

// mod ::gui::QAccessibleEditableTextInterface
// package qtgui
// /usr/include/qt/QtGui/qaccessible.h
// #include <qaccessible.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 17
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
#[derive(Default)] // class sizeof(QAccessibleEditableTextInterface)=8
pub struct QAccessibleEditableTextInterface {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleEditableTextInterface_ITF interface {
//    QAccessibleEditableTextInterface_PTR() *QAccessibleEditableTextInterface
//}
//func (ptr *QAccessibleEditableTextInterface) QAccessibleEditableTextInterface_PTR() *QAccessibleEditableTextInterface { return ptr }

impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleEditableTextInterface {
    return QAccessibleEditableTextInterface{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleEditableTextInterface {
//  type Target = QAccessibleEditableTextInterfaceBASE;
//
//  fn deref(&self) -> &QAccessibleEditableTextInterfaceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleEditableTextInterfaceBASE> for QAccessibleEditableTextInterface {
//  fn as_ref(& self) -> & QAccessibleEditableTextInterfaceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:556
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleEditableTextInterface()

/*

*/
pub fn DeleteQAccessibleEditableTextInterface(this :*mut QAccessibleEditableTextInterface) {
    // let rv = qtrt::InvokeQtFunc6("_ZN32QAccessibleEditableTextInterfaceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:558
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void deleteText(int, int)

/*

*/
impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn deleteText_0<RetType, T: QAccessibleEditableTextInterface_deleteText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.deleteText_0(self);
    // return 1;
  }
}
pub trait QAccessibleEditableTextInterface_deleteText_0<RetType> {
  fn deleteText_0(self , rsthis: & QAccessibleEditableTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleEditableTextInterface_deleteText_0<(/*void*/)> for (i32,i32) {
  fn deleteText_0(self , rsthis: & QAccessibleEditableTextInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN32QAccessibleEditableTextInterface10deleteTextEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:559
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void insertText(int, const QString &)

/*

*/
impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn insertText_0<RetType, T: QAccessibleEditableTextInterface_insertText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertText_0(self);
    // return 1;
  }
}
pub trait QAccessibleEditableTextInterface_insertText_0<RetType> {
  fn insertText_0(self , rsthis: & QAccessibleEditableTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleEditableTextInterface_insertText_0<(/*void*/)> for (i32,usize) {
  fn insertText_0(self , rsthis: & QAccessibleEditableTextInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN32QAccessibleEditableTextInterface10insertTextEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:560
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void replaceText(int, int, const QString &)

/*

*/
impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn replaceText_0<RetType, T: QAccessibleEditableTextInterface_replaceText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replaceText_0(self);
    // return 1;
  }
}
pub trait QAccessibleEditableTextInterface_replaceText_0<RetType> {
  fn replaceText_0(self , rsthis: & QAccessibleEditableTextInterface) -> RetType;
}
impl<'a> /*trait*/ QAccessibleEditableTextInterface_replaceText_0<(/*void*/)> for (i32,i32,usize) {
  fn replaceText_0(self , rsthis: & QAccessibleEditableTextInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN32QAccessibleEditableTextInterface11replaceTextEiiRK7QString", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
