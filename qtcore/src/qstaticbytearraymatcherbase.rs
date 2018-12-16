

// mod ::core::QStaticByteArrayMatcherBase
// package qtcore
// /usr/include/qt/QtCore/qbytearraymatcher.h
// #include <qbytearraymatcher.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 9
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

// int indexOfIn(const char *, uint, const char *, int, int)
// func (this *QStaticByteArrayMatcherBase) InheritIndexOfIn(f func(needle string, nlen uint, haystack string, hlen int, from int) int) {
//  qtrt.SetAllInheritCallback(this, "indexOfIn", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QStaticByteArrayMatcherBase)=256
pub struct QStaticByteArrayMatcherBase {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStaticByteArrayMatcherBase_ITF interface {
//    QStaticByteArrayMatcherBase_PTR() *QStaticByteArrayMatcherBase
//}
//func (ptr *QStaticByteArrayMatcherBase) QStaticByteArrayMatcherBase_PTR() *QStaticByteArrayMatcherBase { return ptr }

impl /*struct*/ QStaticByteArrayMatcherBase {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStaticByteArrayMatcherBase {
    return QStaticByteArrayMatcherBase{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStaticByteArrayMatcherBase {
//  type Target = QStaticByteArrayMatcherBaseBASE;
//
//  fn deref(&self) -> &QStaticByteArrayMatcherBaseBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStaticByteArrayMatcherBaseBASE> for QStaticByteArrayMatcherBase {
//  fn as_ref(& self) -> & QStaticByteArrayMatcherBaseBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qbytearraymatcher.h:93
// index:0
// Protected inline Visibility=Default Availability=Available
// [-2] void QStaticByteArrayMatcherBase(const char *, uint)

/*

*/
// QStaticByteArrayMatcherBase(const char *, uint) ctx.fn_proto_cpp
impl /*struct*/ QStaticByteArrayMatcherBase {
  pub fn QStaticByteArrayMatcherBase_0<T: QStaticByteArrayMatcherBase_QStaticByteArrayMatcherBase_0>(value: T) -> QStaticByteArrayMatcherBase {
    let rsthis = value.QStaticByteArrayMatcherBase_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStaticByteArrayMatcherBase_QStaticByteArrayMatcherBase_0 {
  fn QStaticByteArrayMatcherBase_0(self) -> QStaticByteArrayMatcherBase;
}
// QStaticByteArrayMatcherBase(const char *, uint) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStaticByteArrayMatcherBase_QStaticByteArrayMatcherBase_0 for (usize,u32) {
  fn QStaticByteArrayMatcherBase_0(self) -> QStaticByteArrayMatcherBase {
    // unsafe{_ZN27QStaticByteArrayMatcherBaseC2EPKcj()};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const u32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN27QStaticByteArrayMatcherBaseC2EPKcj", 2,qtrt::FFITY_POINTER,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStaticByteArrayMatcherBase{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearraymatcher.h:98
// index:0
// Protected Visibility=Default Availability=Available
// [4] int indexOfIn(const char *, uint, const char *, int, int) const

/*

*/
impl /*struct*/ QStaticByteArrayMatcherBase {
  pub fn indexOfIn_0<RetType, T: QStaticByteArrayMatcherBase_indexOfIn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfIn_0(self);
    // return 1;
  }
}
pub trait QStaticByteArrayMatcherBase_indexOfIn_0<RetType> {
  fn indexOfIn_0(self , rsthis: & QStaticByteArrayMatcherBase) -> RetType;
}
impl<'a> /*trait*/ QStaticByteArrayMatcherBase_indexOfIn_0<i32> for (usize,u32,usize,i32,i32) {
  fn indexOfIn_0(self , rsthis: & QStaticByteArrayMatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const u32 as usize;
    let arg2 = (self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QStaticByteArrayMatcherBase9indexOfInEPKcjS1_ii", 5,qtrt::FFITY_POINTER,qtrt::FFITY_UINT32,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQStaticByteArrayMatcherBase(this :*mut QStaticByteArrayMatcherBase) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN27QStaticByteArrayMatcherBaseD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
