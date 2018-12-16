

// mod ::core::QGenericArgument
// package qtcore
// /usr/include/qt/QtCore/qobjectdefs.h
// #include <qobjectdefs.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 1
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QGenericArgument)=16
pub struct QGenericArgument {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGenericArgument_ITF interface {
//    QGenericArgument_PTR() *QGenericArgument
//}
//func (ptr *QGenericArgument) QGenericArgument_PTR() *QGenericArgument { return ptr }

impl /*struct*/ QGenericArgument {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGenericArgument {
    return QGenericArgument{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGenericArgument {
//  type Target = QGenericArgumentBASE;
//
//  fn deref(&self) -> &QGenericArgumentBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGenericArgumentBASE> for QGenericArgument {
//  fn as_ref(& self) -> & QGenericArgumentBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qobjectdefs.h:297
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QGenericArgument(const char *, const void *)

/*

*/
// QGenericArgument(const char *, const void *) ctx.fn_proto_cpp
impl /*struct*/ QGenericArgument {
  pub fn QGenericArgument_0<T: QGenericArgument_QGenericArgument_0>(value: T) -> QGenericArgument {
    let rsthis = value.QGenericArgument_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGenericArgument_QGenericArgument_0 {
  fn QGenericArgument_0(self) -> QGenericArgument;
}
// QGenericArgument(const char *, const void *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGenericArgument_QGenericArgument_0 for (usize,usize) {
  fn QGenericArgument_0(self) -> QGenericArgument {
    // unsafe{_ZN16QGenericArgumentC2EPKcPKv()};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QGenericArgumentC2EPKcPKv", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGenericArgument{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:299
// index:0
// Public inline Visibility=Default Availability=Available
// [8] void * data() const

/*

*/
impl /*struct*/ QGenericArgument {
  pub fn data_0<RetType, T: QGenericArgument_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QGenericArgument_data_0<RetType> {
  fn data_0(self , rsthis: & QGenericArgument) -> RetType;
}
impl<'a> /*trait*/ QGenericArgument_data_0<usize> for () {
  fn data_0(self , rsthis: & QGenericArgument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QGenericArgument4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:300
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const char * name() const

/*

*/
impl /*struct*/ QGenericArgument {
  pub fn name_0<RetType, T: QGenericArgument_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QGenericArgument_name_0<RetType> {
  fn name_0(self , rsthis: & QGenericArgument) -> RetType;
}
impl<'a> /*trait*/ QGenericArgument_name_0<usize> for () {
  fn name_0(self , rsthis: & QGenericArgument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QGenericArgument4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQGenericArgument(this :*mut QGenericArgument) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN16QGenericArgumentD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
