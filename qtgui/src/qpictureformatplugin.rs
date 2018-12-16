

// mod ::gui::QPictureFormatPlugin
// package qtgui
// /usr/include/qt/QtGui/qpictureformatplugin.h
// #include <qpictureformatplugin.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 26
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
#[derive(Default)] // class sizeof(QPictureFormatPlugin)=16
pub struct QPictureFormatPlugin {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPictureFormatPlugin_ITF interface {
//    qtcore.QObject_ITF
//    QPictureFormatPlugin_PTR() *QPictureFormatPlugin
//}
//func (ptr *QPictureFormatPlugin) QPictureFormatPlugin_PTR() *QPictureFormatPlugin { return ptr }

impl /*struct*/ QPictureFormatPlugin {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPictureFormatPlugin {
    return QPictureFormatPlugin{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPictureFormatPlugin {
//  type Target = QPictureFormatPluginBASE;
//
//  fn deref(&self) -> &QPictureFormatPluginBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPictureFormatPluginBASE> for QPictureFormatPlugin {
//  fn as_ref(& self) -> & QPictureFormatPluginBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpictureformatplugin.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QPictureFormatPlugin {
  pub fn metaObject_0<RetType, T: QPictureFormatPlugin_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QPictureFormatPlugin_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QPictureFormatPlugin) -> RetType;
}
impl<'a> /*trait*/ QPictureFormatPlugin_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QPictureFormatPlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QPictureFormatPlugin10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpictureformatplugin.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPictureFormatPlugin(QObject *)

/*
Constructs an picture format plugin with the given parent. This is invoked automatically by the moc generated code that exports the plugin.
*/
// QPictureFormatPlugin(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QPictureFormatPlugin {
  pub fn QPictureFormatPlugin_0<T: QPictureFormatPlugin_QPictureFormatPlugin_0>(value: T) -> QPictureFormatPlugin {
    let rsthis = value.QPictureFormatPlugin_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPictureFormatPlugin_QPictureFormatPlugin_0 {
  fn QPictureFormatPlugin_0(self) -> QPictureFormatPlugin;
}
// QPictureFormatPlugin(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPictureFormatPlugin_QPictureFormatPlugin_0 for (usize) {
  fn QPictureFormatPlugin_0(self) -> QPictureFormatPlugin {
    // unsafe{_ZN20QPictureFormatPluginC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QPictureFormatPluginC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPictureFormatPlugin{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpictureformatplugin.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPictureFormatPlugin()

/*

*/
pub fn DeleteQPictureFormatPlugin(this :*mut QPictureFormatPlugin) {
    // let rv = qtrt::InvokeQtFunc6("_ZN20QPictureFormatPluginD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpictureformatplugin.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool loadPicture(const QString &, const QString &, QPicture *)

/*
Loads the picture stored in the file called fileName, with the given format, into *picture. Returns true on success; otherwise returns false.

See also savePicture().
*/
impl /*struct*/ QPictureFormatPlugin {
  pub fn loadPicture_0<RetType, T: QPictureFormatPlugin_loadPicture_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loadPicture_0(self);
    // return 1;
  }
}
pub trait QPictureFormatPlugin_loadPicture_0<RetType> {
  fn loadPicture_0(self , rsthis: & QPictureFormatPlugin) -> RetType;
}
impl<'a> /*trait*/ QPictureFormatPlugin_loadPicture_0<bool> for (usize,usize,usize) {
  fn loadPicture_0(self , rsthis: & QPictureFormatPlugin) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QPictureFormatPlugin11loadPictureERK7QStringS2_P8QPicture", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpictureformatplugin.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool savePicture(const QString &, const QString &, const QPicture &)

/*
Saves the given picture into the file called fileName, using the specified format. Returns true on success; otherwise returns false.

See also loadPicture().
*/
impl /*struct*/ QPictureFormatPlugin {
  pub fn savePicture_0<RetType, T: QPictureFormatPlugin_savePicture_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.savePicture_0(self);
    // return 1;
  }
}
pub trait QPictureFormatPlugin_savePicture_0<RetType> {
  fn savePicture_0(self , rsthis: & QPictureFormatPlugin) -> RetType;
}
impl<'a> /*trait*/ QPictureFormatPlugin_savePicture_0<bool> for (usize,usize,usize) {
  fn savePicture_0(self , rsthis: & QPictureFormatPlugin) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QPictureFormatPlugin11savePictureERK7QStringS2_RK8QPicture", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpictureformatplugin.h:68
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool installIOHandler(const QString &)

/*
Installs a QPictureIO picture I/O handler for the picture format format. Returns true on success.
*/
impl /*struct*/ QPictureFormatPlugin {
  pub fn installIOHandler_0<RetType, T: QPictureFormatPlugin_installIOHandler_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.installIOHandler_0(self);
    // return 1;
  }
}
pub trait QPictureFormatPlugin_installIOHandler_0<RetType> {
  fn installIOHandler_0(self , rsthis: & QPictureFormatPlugin) -> RetType;
}
impl<'a> /*trait*/ QPictureFormatPlugin_installIOHandler_0<bool> for (usize) {
  fn installIOHandler_0(self , rsthis: & QPictureFormatPlugin) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QPictureFormatPlugin16installIOHandlerERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
