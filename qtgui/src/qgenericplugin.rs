

// mod ::gui::QGenericPlugin
// package qtgui
// /usr/include/qt/QtGui/qgenericplugin.h
// #include <qgenericplugin.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 37
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
#[derive(Default)] // class sizeof(QGenericPlugin)=16
pub struct QGenericPlugin {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGenericPlugin_ITF interface {
//    qtcore.QObject_ITF
//    QGenericPlugin_PTR() *QGenericPlugin
//}
//func (ptr *QGenericPlugin) QGenericPlugin_PTR() *QGenericPlugin { return ptr }

impl /*struct*/ QGenericPlugin {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGenericPlugin {
    return QGenericPlugin{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGenericPlugin {
//  type Target = QGenericPluginBASE;
//
//  fn deref(&self) -> &QGenericPluginBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGenericPluginBASE> for QGenericPlugin {
//  fn as_ref(& self) -> & QGenericPluginBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qgenericplugin.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGenericPlugin {
  pub fn metaObject_0<RetType, T: QGenericPlugin_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGenericPlugin_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGenericPlugin) -> RetType;
}
impl<'a> /*trait*/ QGenericPlugin_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGenericPlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGenericPlugin10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qgenericplugin.h:55
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGenericPlugin(QObject *)

/*
Constructs a plugin with the given parent.

Note that this constructor is invoked automatically by the moc generated code that exports the plugin, so there is no need for calling it explicitly.
*/
// QGenericPlugin(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGenericPlugin {
  pub fn QGenericPlugin_0<T: QGenericPlugin_QGenericPlugin_0>(value: T) -> QGenericPlugin {
    let rsthis = value.QGenericPlugin_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGenericPlugin_QGenericPlugin_0 {
  fn QGenericPlugin_0(self) -> QGenericPlugin;
}
// QGenericPlugin(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGenericPlugin_QGenericPlugin_0 for (usize) {
  fn QGenericPlugin_0(self) -> QGenericPlugin {
    // unsafe{_ZN14QGenericPluginC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QGenericPluginC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGenericPlugin{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qgenericplugin.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGenericPlugin()

/*

*/
pub fn DeleteQGenericPlugin(this :*mut QGenericPlugin) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QGenericPluginD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qgenericplugin.h:58
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QObject * create(const QString &, const QString &)

/*
Implement this function to create a driver matching the type specified by the given key and specification parameters. Note that keys are case-insensitive.
*/
impl /*struct*/ QGenericPlugin {
  pub fn create_0<RetType, T: QGenericPlugin_create_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.create_0(self);
    // return 1;
  }
}
pub trait QGenericPlugin_create_0<RetType> {
  fn create_0(self , rsthis: & QGenericPlugin) -> RetType;
}
impl<'a> /*trait*/ QGenericPlugin_create_0<usize> for (usize,usize) {
  fn create_0(self , rsthis: & QGenericPlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGenericPlugin6createERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
