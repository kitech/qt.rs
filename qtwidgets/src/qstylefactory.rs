

// mod ::widgets::QStyleFactory
// package qtwidgets
// /usr/include/qt/QtWidgets/qstylefactory.h
// #include <qstylefactory.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 15
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
#[derive(Default)] // class sizeof(QStyleFactory)=1
pub struct QStyleFactory {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleFactory_ITF interface {
//    QStyleFactory_PTR() *QStyleFactory
//}
//func (ptr *QStyleFactory) QStyleFactory_PTR() *QStyleFactory { return ptr }

impl /*struct*/ QStyleFactory {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleFactory {
    return QStyleFactory{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleFactory {
//  type Target = QStyleFactoryBASE;
//
//  fn deref(&self) -> &QStyleFactoryBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleFactoryBASE> for QStyleFactory {
//  fn as_ref(& self) -> & QStyleFactoryBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstylefactory.h:54
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList keys()

/*
Returns the list of valid keys, i.e. the keys this factory can create styles for.

See also create().
*/
impl /*struct*/ QStyleFactory {
  pub fn keys_0<RetType, T: QStyleFactory_keys_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.keys_0();
    // return 1;
  }
}
pub trait QStyleFactory_keys_0<RetType> {
  fn keys_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStyleFactory_keys_0<usize> for () {
  fn keys_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QStyleFactory4keysEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstylefactory.h:55
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStyle * create(const QString &)

/*
Creates and returns a QStyle object that matches the given key, or returns 0 if no matching style is found.

Both built-in styles and styles from style plugins are queried for a matching style.

Note: The keys used are case insensitive.

See also keys().
*/
impl /*struct*/ QStyleFactory {
  pub fn create_0<RetType, T: QStyleFactory_create_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.create_0();
    // return 1;
  }
}
pub trait QStyleFactory_create_0<RetType> {
  fn create_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStyleFactory_create_0<usize> for (usize) {
  fn create_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QStyleFactory6createERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQStyleFactory(this :*mut QStyleFactory) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN13QStyleFactoryD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
