

// mod ::gui::QGenericPluginFactory
// package qtgui
// /usr/include/qt/QtGui/qgenericpluginfactory.h
// #include <qgenericpluginfactory.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 4
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
#[derive(Default)] // class sizeof(QGenericPluginFactory)=1
pub struct QGenericPluginFactory {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGenericPluginFactory_ITF interface {
//    QGenericPluginFactory_PTR() *QGenericPluginFactory
//}
//func (ptr *QGenericPluginFactory) QGenericPluginFactory_PTR() *QGenericPluginFactory { return ptr }

impl /*struct*/ QGenericPluginFactory {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGenericPluginFactory {
    return QGenericPluginFactory{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGenericPluginFactory {
//  type Target = QGenericPluginFactoryBASE;
//
//  fn deref(&self) -> &QGenericPluginFactoryBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGenericPluginFactoryBASE> for QGenericPluginFactory {
//  fn as_ref(& self) -> & QGenericPluginFactoryBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qgenericpluginfactory.h:55
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList keys()

/*
Returns the list of valid keys, i.e. the available mouse drivers.

See also create().
*/
impl /*struct*/ QGenericPluginFactory {
  pub fn keys_0<RetType, T: QGenericPluginFactory_keys_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.keys_0();
    // return 1;
  }
}
pub trait QGenericPluginFactory_keys_0<RetType> {
  fn keys_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGenericPluginFactory_keys_0<usize> for () {
  fn keys_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QGenericPluginFactory4keysEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qgenericpluginfactory.h:56
// index:0
// Public static Visibility=Default Availability=Available
// [8] QObject * create(const QString &, const QString &)

/*
Creates the driver specified by key, using the given specification.

Note that the keys are case-insensitive.

See also keys().
*/
impl /*struct*/ QGenericPluginFactory {
  pub fn create_0<RetType, T: QGenericPluginFactory_create_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.create_0();
    // return 1;
  }
}
pub trait QGenericPluginFactory_create_0<RetType> {
  fn create_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGenericPluginFactory_create_0<usize> for (usize,usize) {
  fn create_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QGenericPluginFactory6createERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQGenericPluginFactory(this :*mut QGenericPluginFactory) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN21QGenericPluginFactoryD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
