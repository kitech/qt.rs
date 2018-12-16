

// mod ::widgets::QStylePlugin
// package qtwidgets
// /usr/include/qt/QtWidgets/qstyleplugin.h
// #include <qstyleplugin.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 11
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
#[derive(Default)] // class sizeof(QStylePlugin)=16
pub struct QStylePlugin {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStylePlugin_ITF interface {
//    qtcore.QObject_ITF
//    QStylePlugin_PTR() *QStylePlugin
//}
//func (ptr *QStylePlugin) QStylePlugin_PTR() *QStylePlugin { return ptr }

impl /*struct*/ QStylePlugin {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStylePlugin {
    return QStylePlugin{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStylePlugin {
//  type Target = QStylePluginBASE;
//
//  fn deref(&self) -> &QStylePluginBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStylePluginBASE> for QStylePlugin {
//  fn as_ref(& self) -> & QStylePluginBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleplugin.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QStylePlugin {
  pub fn metaObject_0<RetType, T: QStylePlugin_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QStylePlugin_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QStylePlugin) -> RetType;
}
impl<'a> /*trait*/ QStylePlugin_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QStylePlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStylePlugin10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyleplugin.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStylePlugin(QObject *)

/*
Constructs a style plugin with the given parent.

Note that this constructor is invoked automatically by the moc generated code that exports the plugin, so there is no need for calling it explicitly.
*/
// QStylePlugin(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QStylePlugin {
  pub fn QStylePlugin_0<T: QStylePlugin_QStylePlugin_0>(value: T) -> QStylePlugin {
    let rsthis = value.QStylePlugin_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStylePlugin_QStylePlugin_0 {
  fn QStylePlugin_0(self) -> QStylePlugin;
}
// QStylePlugin(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStylePlugin_QStylePlugin_0 for (usize) {
  fn QStylePlugin_0(self) -> QStylePlugin {
    // unsafe{_ZN12QStylePluginC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QStylePluginC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStylePlugin{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleplugin.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QStylePlugin()

/*

*/
pub fn DeleteQStylePlugin(this :*mut QStylePlugin) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QStylePluginD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qstyleplugin.h:61
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QStyle * create(const QString &)

/*
Creates and returns a QStyle object for the given style key. If a plugin cannot create a style, it should return 0 instead.

The style key is usually the class name of the required style. Note that the keys are case insensitive. For example:


  QStyle *MyStylePlugin::create(const QString &key)
  {
      QString lcKey = key.toLower();
      if (lcKey == "rocket") {
          return new RocketStyle;
      } else if (lcKey == "starbuster") {
          return new StarBusterStyle;
      }
      return 0;
  }
*/
impl /*struct*/ QStylePlugin {
  pub fn create_0<RetType, T: QStylePlugin_create_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.create_0(self);
    // return 1;
  }
}
pub trait QStylePlugin_create_0<RetType> {
  fn create_0(self , rsthis: & QStylePlugin) -> RetType;
}
impl<'a> /*trait*/ QStylePlugin_create_0<usize> for (usize) {
  fn create_0(self , rsthis: & QStylePlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QStylePlugin6createERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
