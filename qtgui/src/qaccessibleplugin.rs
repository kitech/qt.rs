

// mod ::gui::QAccessiblePlugin
// package qtgui
// /usr/include/qt/QtGui/qaccessibleplugin.h
// #include <qaccessibleplugin.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 10
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
#[derive(Default)] // class sizeof(QAccessiblePlugin)=16
pub struct QAccessiblePlugin {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessiblePlugin_ITF interface {
//    qtcore.QObject_ITF
//    QAccessiblePlugin_PTR() *QAccessiblePlugin
//}
//func (ptr *QAccessiblePlugin) QAccessiblePlugin_PTR() *QAccessiblePlugin { return ptr }

impl /*struct*/ QAccessiblePlugin {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessiblePlugin {
    return QAccessiblePlugin{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessiblePlugin {
//  type Target = QAccessiblePluginBASE;
//
//  fn deref(&self) -> &QAccessiblePluginBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessiblePluginBASE> for QAccessiblePlugin {
//  fn as_ref(& self) -> & QAccessiblePluginBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessibleplugin.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAccessiblePlugin {
  pub fn metaObject_0<RetType, T: QAccessiblePlugin_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAccessiblePlugin_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAccessiblePlugin) -> RetType;
}
impl<'a> /*trait*/ QAccessiblePlugin_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAccessiblePlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QAccessiblePlugin10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessibleplugin.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAccessiblePlugin(QObject *)

/*
Constructs an accessibility plugin with the given parent. This is invoked automatically by the plugin loader.
*/
// QAccessiblePlugin(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QAccessiblePlugin {
  pub fn QAccessiblePlugin_0<T: QAccessiblePlugin_QAccessiblePlugin_0>(value: T) -> QAccessiblePlugin {
    let rsthis = value.QAccessiblePlugin_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessiblePlugin_QAccessiblePlugin_0 {
  fn QAccessiblePlugin_0(self) -> QAccessiblePlugin;
}
// QAccessiblePlugin(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessiblePlugin_QAccessiblePlugin_0 for (usize) {
  fn QAccessiblePlugin_0(self) -> QAccessiblePlugin {
    // unsafe{_ZN17QAccessiblePluginC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QAccessiblePluginC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessiblePlugin{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessibleplugin.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessiblePlugin()

/*

*/
pub fn DeleteQAccessiblePlugin(this :*mut QAccessiblePlugin) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QAccessiblePluginD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessibleplugin.h:66
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * create(const QString &, QObject *)

/*
Creates and returns a QAccessibleInterface implementation for the class key and the object object. Keys are case sensitive.
*/
impl /*struct*/ QAccessiblePlugin {
  pub fn create_0<RetType, T: QAccessiblePlugin_create_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.create_0(self);
    // return 1;
  }
}
pub trait QAccessiblePlugin_create_0<RetType> {
  fn create_0(self , rsthis: & QAccessiblePlugin) -> RetType;
}
impl<'a> /*trait*/ QAccessiblePlugin_create_0<usize> for (usize,usize) {
  fn create_0(self , rsthis: & QAccessiblePlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QAccessiblePlugin6createERK7QStringP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
