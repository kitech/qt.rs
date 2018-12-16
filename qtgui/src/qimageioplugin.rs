

// mod ::gui::QImageIOPlugin
// package qtgui
// /usr/include/qt/QtGui/qimageiohandler.h
// #include <qimageiohandler.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 21
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
#[derive(Default)] // class sizeof(QImageIOPlugin)=16
pub struct QImageIOPlugin {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QImageIOPlugin_ITF interface {
//    qtcore.QObject_ITF
//    QImageIOPlugin_PTR() *QImageIOPlugin
//}
//func (ptr *QImageIOPlugin) QImageIOPlugin_PTR() *QImageIOPlugin { return ptr }

impl /*struct*/ QImageIOPlugin {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QImageIOPlugin {
    return QImageIOPlugin{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QImageIOPlugin {
//  type Target = QImageIOPluginBASE;
//
//  fn deref(&self) -> &QImageIOPluginBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QImageIOPluginBASE> for QImageIOPlugin {
//  fn as_ref(& self) -> & QImageIOPluginBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qimageiohandler.h:141
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QImageIOPlugin {
  pub fn metaObject_0<RetType, T: QImageIOPlugin_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QImageIOPlugin_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QImageIOPlugin) -> RetType;
}
impl<'a> /*trait*/ QImageIOPlugin_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QImageIOPlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QImageIOPlugin10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QImageIOPlugin(QObject *)

/*

*/
// QImageIOPlugin(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QImageIOPlugin {
  pub fn QImageIOPlugin_0<T: QImageIOPlugin_QImageIOPlugin_0>(value: T) -> QImageIOPlugin {
    let rsthis = value.QImageIOPlugin_0();
    return rsthis;
    // return 1;
  }
}

pub trait QImageIOPlugin_QImageIOPlugin_0 {
  fn QImageIOPlugin_0(self) -> QImageIOPlugin;
}
// QImageIOPlugin(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QImageIOPlugin_QImageIOPlugin_0 for (usize) {
  fn QImageIOPlugin_0(self) -> QImageIOPlugin {
    // unsafe{_ZN14QImageIOPluginC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QImageIOPluginC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImageIOPlugin{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:144
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QImageIOPlugin()

/*

*/
pub fn DeleteQImageIOPlugin(this :*mut QImageIOPlugin) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QImageIOPluginD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qimageiohandler.h:153
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] QImageIOPlugin::Capabilities capabilities(QIODevice *, const QByteArray &) const

/*

*/
impl /*struct*/ QImageIOPlugin {
  pub fn capabilities_0<RetType, T: QImageIOPlugin_capabilities_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capabilities_0(self);
    // return 1;
  }
}
pub trait QImageIOPlugin_capabilities_0<RetType> {
  fn capabilities_0(self , rsthis: & QImageIOPlugin) -> RetType;
}
impl<'a> /*trait*/ QImageIOPlugin_capabilities_0<i32> for (usize,usize) {
  fn capabilities_0(self , rsthis: & QImageIOPlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QImageIOPlugin12capabilitiesEP9QIODeviceRK10QByteArray", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:154
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QImageIOHandler * create(QIODevice *, const QByteArray &) const

/*

*/
impl /*struct*/ QImageIOPlugin {
  pub fn create_0<RetType, T: QImageIOPlugin_create_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.create_0(self);
    // return 1;
  }
}
pub trait QImageIOPlugin_create_0<RetType> {
  fn create_0(self , rsthis: & QImageIOPlugin) -> RetType;
}
impl<'a> /*trait*/ QImageIOPlugin_create_0<usize> for (usize,usize) {
  fn create_0(self , rsthis: & QImageIOPlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QImageIOPlugin6createEP9QIODeviceRK10QByteArray", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*


*/
pub type QImageIOPlugin__Capability = i32;
// 
pub const QImageIOPlugin__CanRead :QImageIOPlugin__Capability = 1;
// 
pub const QImageIOPlugin__CanWrite :QImageIOPlugin__Capability = 2;
// 
pub const QImageIOPlugin__CanReadIncremental :QImageIOPlugin__Capability = 4;
pub fn QImageIOPlugin_CapabilityItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QImageIOPlugin", val);
}
pub fn QImageIOPlugin_CapabilityItemName_s(val: i32) ->String {
  //var nilthis *QImageIOPlugin
  //return nilthis.CapabilityItemName(val);
  return QImageIOPlugin_CapabilityItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
