

// mod ::core::QXmlStreamEntityResolver
// package qtcore
// /usr/include/qt/QtCore/qxmlstream.h
// #include <qxmlstream.h>
// #include <QtCore>

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
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QXmlStreamEntityResolver)=8
pub struct QXmlStreamEntityResolver {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QXmlStreamEntityResolver_ITF interface {
//    QXmlStreamEntityResolver_PTR() *QXmlStreamEntityResolver
//}
//func (ptr *QXmlStreamEntityResolver) QXmlStreamEntityResolver_PTR() *QXmlStreamEntityResolver { return ptr }

impl /*struct*/ QXmlStreamEntityResolver {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QXmlStreamEntityResolver {
    return QXmlStreamEntityResolver{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QXmlStreamEntityResolver {
//  type Target = QXmlStreamEntityResolverBASE;
//
//  fn deref(&self) -> &QXmlStreamEntityResolverBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QXmlStreamEntityResolverBASE> for QXmlStreamEntityResolver {
//  fn as_ref(& self) -> & QXmlStreamEntityResolverBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qxmlstream.h:336
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QXmlStreamEntityResolver()

/*

*/
pub fn DeleteQXmlStreamEntityResolver(this :*mut QXmlStreamEntityResolver) {
    // let rv = qtrt::InvokeQtFunc6("_ZN24QXmlStreamEntityResolverD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qxmlstream.h:337
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString resolveEntity(const QString &, const QString &)

/*

*/
impl /*struct*/ QXmlStreamEntityResolver {
  pub fn resolveEntity_0<RetType, T: QXmlStreamEntityResolver_resolveEntity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolveEntity_0(self);
    // return 1;
  }
}
pub trait QXmlStreamEntityResolver_resolveEntity_0<RetType> {
  fn resolveEntity_0(self , rsthis: & QXmlStreamEntityResolver) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamEntityResolver_resolveEntity_0<usize> for (usize,usize) {
  fn resolveEntity_0(self , rsthis: & QXmlStreamEntityResolver) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN24QXmlStreamEntityResolver13resolveEntityERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:338
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString resolveUndeclaredEntity(const QString &)

/*

*/
impl /*struct*/ QXmlStreamEntityResolver {
  pub fn resolveUndeclaredEntity_0<RetType, T: QXmlStreamEntityResolver_resolveUndeclaredEntity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolveUndeclaredEntity_0(self);
    // return 1;
  }
}
pub trait QXmlStreamEntityResolver_resolveUndeclaredEntity_0<RetType> {
  fn resolveUndeclaredEntity_0(self , rsthis: & QXmlStreamEntityResolver) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamEntityResolver_resolveUndeclaredEntity_0<usize> for (usize) {
  fn resolveUndeclaredEntity_0(self , rsthis: & QXmlStreamEntityResolver) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN24QXmlStreamEntityResolver23resolveUndeclaredEntityERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
