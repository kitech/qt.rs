

// mod ::core::QStaticPlugin
// package qtcore
// /usr/include/qt/QtCore/qplugin.h
// #include <qplugin.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QStaticPlugin)=16
pub struct QStaticPlugin {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStaticPlugin_ITF interface {
//    QStaticPlugin_PTR() *QStaticPlugin
//}
//func (ptr *QStaticPlugin) QStaticPlugin_PTR() *QStaticPlugin { return ptr }

impl /*struct*/ QStaticPlugin {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStaticPlugin {
    return QStaticPlugin{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStaticPlugin {
//  type Target = QStaticPluginBASE;
//
//  fn deref(&self) -> &QStaticPluginBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStaticPluginBASE> for QStaticPlugin {
//  fn as_ref(& self) -> & QStaticPluginBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qplugin.h:74
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonObject metaData() const

/*

*/
impl /*struct*/ QStaticPlugin {
  pub fn metaData_0<RetType, T: QStaticPlugin_metaData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaData_0(self);
    // return 1;
  }
}
pub trait QStaticPlugin_metaData_0<RetType> {
  fn metaData_0(self , rsthis: & QStaticPlugin) -> RetType;
}
impl<'a> /*trait*/ QStaticPlugin_metaData_0<usize> for () {
  fn metaData_0(self , rsthis: & QStaticPlugin) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStaticPlugin8metaDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQStaticPlugin(this :*mut QStaticPlugin) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN13QStaticPluginD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
