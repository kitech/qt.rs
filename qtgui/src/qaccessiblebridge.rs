

// mod ::gui::QAccessibleBridge
// package qtgui
// /usr/include/qt/QtGui/qaccessiblebridge.h
// #include <qaccessiblebridge.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 13
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
#[derive(Default)] // class sizeof(QAccessibleBridge)=8
pub struct QAccessibleBridge {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleBridge_ITF interface {
//    QAccessibleBridge_PTR() *QAccessibleBridge
//}
//func (ptr *QAccessibleBridge) QAccessibleBridge_PTR() *QAccessibleBridge { return ptr }

impl /*struct*/ QAccessibleBridge {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleBridge {
    return QAccessibleBridge{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleBridge {
//  type Target = QAccessibleBridgeBASE;
//
//  fn deref(&self) -> &QAccessibleBridgeBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleBridgeBASE> for QAccessibleBridge {
//  fn as_ref(& self) -> & QAccessibleBridgeBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessiblebridge.h:58
// index:0
// Public inline virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleBridge()

/*

*/
pub fn DeleteQAccessibleBridge(this :*mut QAccessibleBridge) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QAccessibleBridgeD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessiblebridge.h:59
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void setRootObject(QAccessibleInterface *)

/*

*/
impl /*struct*/ QAccessibleBridge {
  pub fn setRootObject_0<RetType, T: QAccessibleBridge_setRootObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRootObject_0(self);
    // return 1;
  }
}
pub trait QAccessibleBridge_setRootObject_0<RetType> {
  fn setRootObject_0(self , rsthis: & QAccessibleBridge) -> RetType;
}
impl<'a> /*trait*/ QAccessibleBridge_setRootObject_0<(/*void*/)> for (usize) {
  fn setRootObject_0(self , rsthis: & QAccessibleBridge) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAccessibleBridge13setRootObjectEP20QAccessibleInterface", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessiblebridge.h:60
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void notifyAccessibilityUpdate(QAccessibleEvent *)

/*

*/
impl /*struct*/ QAccessibleBridge {
  pub fn notifyAccessibilityUpdate_0<RetType, T: QAccessibleBridge_notifyAccessibilityUpdate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notifyAccessibilityUpdate_0(self);
    // return 1;
  }
}
pub trait QAccessibleBridge_notifyAccessibilityUpdate_0<RetType> {
  fn notifyAccessibilityUpdate_0(self , rsthis: & QAccessibleBridge) -> RetType;
}
impl<'a> /*trait*/ QAccessibleBridge_notifyAccessibilityUpdate_0<(/*void*/)> for (usize) {
  fn notifyAccessibilityUpdate_0(self , rsthis: & QAccessibleBridge) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QAccessibleBridge25notifyAccessibilityUpdateEP16QAccessibleEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
