

// mod ::gui::QIconEnginePlugin
// package qtgui
// /usr/include/qt/QtGui/qiconengineplugin.h
// #include <qiconengineplugin.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 16
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
#[derive(Default)] // class sizeof(QIconEnginePlugin)=16
pub struct QIconEnginePlugin {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QIconEnginePlugin_ITF interface {
//    qtcore.QObject_ITF
//    QIconEnginePlugin_PTR() *QIconEnginePlugin
//}
//func (ptr *QIconEnginePlugin) QIconEnginePlugin_PTR() *QIconEnginePlugin { return ptr }

impl /*struct*/ QIconEnginePlugin {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QIconEnginePlugin {
    return QIconEnginePlugin{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QIconEnginePlugin {
//  type Target = QIconEnginePluginBASE;
//
//  fn deref(&self) -> &QIconEnginePluginBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QIconEnginePluginBASE> for QIconEnginePlugin {
//  fn as_ref(& self) -> & QIconEnginePluginBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qiconengineplugin.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QIconEnginePlugin {
  pub fn metaObject_0<RetType, T: QIconEnginePlugin_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QIconEnginePlugin_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QIconEnginePlugin) -> RetType;
}
impl<'a> /*trait*/ QIconEnginePlugin_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QIconEnginePlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QIconEnginePlugin10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qiconengineplugin.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QIconEnginePlugin(QObject *)

/*
Constructs a icon engine plugin with the given parent. This is invoked automatically by the plugin loader.
*/
// QIconEnginePlugin(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QIconEnginePlugin {
  pub fn QIconEnginePlugin_0<T: QIconEnginePlugin_QIconEnginePlugin_0>(value: T) -> QIconEnginePlugin {
    let rsthis = value.QIconEnginePlugin_0();
    return rsthis;
    // return 1;
  }
}

pub trait QIconEnginePlugin_QIconEnginePlugin_0 {
  fn QIconEnginePlugin_0(self) -> QIconEnginePlugin;
}
// QIconEnginePlugin(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QIconEnginePlugin_QIconEnginePlugin_0 for (usize) {
  fn QIconEnginePlugin_0(self) -> QIconEnginePlugin {
    // unsafe{_ZN17QIconEnginePluginC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QIconEnginePluginC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QIconEnginePlugin{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qiconengineplugin.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QIconEnginePlugin()

/*

*/
pub fn DeleteQIconEnginePlugin(this :*mut QIconEnginePlugin) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QIconEnginePluginD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qiconengineplugin.h:61
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QIconEngine * create(const QString &)

/*
Creates and returns a QIconEngine object for the icon with the given filename.
*/
impl /*struct*/ QIconEnginePlugin {
  pub fn create_0<RetType, T: QIconEnginePlugin_create_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.create_0(self);
    // return 1;
  }
}
pub trait QIconEnginePlugin_create_0<RetType> {
  fn create_0(self , rsthis: & QIconEnginePlugin) -> RetType;
}
impl<'a> /*trait*/ QIconEnginePlugin_create_0<usize> for (usize) {
  fn create_0(self , rsthis: & QIconEnginePlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QIconEnginePlugin6createERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
