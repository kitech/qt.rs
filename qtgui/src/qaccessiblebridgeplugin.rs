

// mod ::gui::QAccessibleBridgePlugin
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
// extern C begin: 3
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
#[derive(Default)] // class sizeof(QAccessibleBridgePlugin)=16
pub struct QAccessibleBridgePlugin {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleBridgePlugin_ITF interface {
//    qtcore.QObject_ITF
//    QAccessibleBridgePlugin_PTR() *QAccessibleBridgePlugin
//}
//func (ptr *QAccessibleBridgePlugin) QAccessibleBridgePlugin_PTR() *QAccessibleBridgePlugin { return ptr }

impl /*struct*/ QAccessibleBridgePlugin {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleBridgePlugin {
    return QAccessibleBridgePlugin{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleBridgePlugin {
//  type Target = QAccessibleBridgePluginBASE;
//
//  fn deref(&self) -> &QAccessibleBridgePluginBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleBridgePluginBASE> for QAccessibleBridgePlugin {
//  fn as_ref(& self) -> & QAccessibleBridgePluginBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessiblebridge.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn metaObject_0<RetType, T: QAccessibleBridgePlugin_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAccessibleBridgePlugin_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAccessibleBridgePlugin) -> RetType;
}
impl<'a> /*trait*/ QAccessibleBridgePlugin_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAccessibleBridgePlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QAccessibleBridgePlugin10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessiblebridge.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAccessibleBridgePlugin(QObject *)

/*

*/
// QAccessibleBridgePlugin(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn QAccessibleBridgePlugin_0<T: QAccessibleBridgePlugin_QAccessibleBridgePlugin_0>(value: T) -> QAccessibleBridgePlugin {
    let rsthis = value.QAccessibleBridgePlugin_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleBridgePlugin_QAccessibleBridgePlugin_0 {
  fn QAccessibleBridgePlugin_0(self) -> QAccessibleBridgePlugin;
}
// QAccessibleBridgePlugin(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleBridgePlugin_QAccessibleBridgePlugin_0 for (usize) {
  fn QAccessibleBridgePlugin_0(self) -> QAccessibleBridgePlugin {
    // unsafe{_ZN23QAccessibleBridgePluginC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QAccessibleBridgePluginC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleBridgePlugin{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessiblebridge.h:70
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleBridgePlugin()

/*

*/
pub fn DeleteQAccessibleBridgePlugin(this :*mut QAccessibleBridgePlugin) {
    // let rv = qtrt::InvokeQtFunc6("_ZN23QAccessibleBridgePluginD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessiblebridge.h:72
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QAccessibleBridge * create(const QString &)

/*

*/
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn create_0<RetType, T: QAccessibleBridgePlugin_create_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.create_0(self);
    // return 1;
  }
}
pub trait QAccessibleBridgePlugin_create_0<RetType> {
  fn create_0(self , rsthis: & QAccessibleBridgePlugin) -> RetType;
}
impl<'a> /*trait*/ QAccessibleBridgePlugin_create_0<usize> for (usize) {
  fn create_0(self , rsthis: & QAccessibleBridgePlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QAccessibleBridgePlugin6createERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
