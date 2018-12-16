

// mod ::core::QByteRef
// package qtcore
// /usr/include/qt/QtCore/qbytearray.h
// #include <qbytearray.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 147
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
#[derive(Default)] // class sizeof(QByteRef)=16
pub struct QByteRef {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QByteRef_ITF interface {
//    QByteRef_PTR() *QByteRef
//}
//func (ptr *QByteRef) QByteRef_PTR() *QByteRef { return ptr }

impl /*struct*/ QByteRef {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QByteRef {
    return QByteRef{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QByteRef {
//  type Target = QByteRefBASE;
//
//  fn deref(&self) -> &QByteRefBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QByteRefBASE> for QByteRef {
//  fn as_ref(& self) -> & QByteRefBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qbytearray.h:535
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QByteRef & operator=(char)

/*

*/
impl /*struct*/ QByteRef {
  pub fn operator_equal_0<RetType, T: QByteRef_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QByteRef_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QByteRef) -> RetType;
}
impl<'a> /*trait*/ QByteRef_operator_equal_0<usize> for (i8) {
  fn operator_equal_0(self , rsthis: & QByteRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QByteRefaSEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:538
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QByteRef & operator=(const QByteRef &)

/*

*/
impl /*struct*/ QByteRef {
  pub fn operator_equal_1<RetType, T: QByteRef_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QByteRef_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QByteRef) -> RetType;
}
impl<'a> /*trait*/ QByteRef_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QByteRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QByteRefaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:541
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(char) const

/*

*/
impl /*struct*/ QByteRef {
  pub fn operator_equal_equal_0<RetType, T: QByteRef_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QByteRef_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QByteRef) -> RetType;
}
impl<'a> /*trait*/ QByteRef_operator_equal_equal_0<bool> for (i8) {
  fn operator_equal_equal_0(self , rsthis: & QByteRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QByteRefeqEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:543
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(char) const

/*

*/
impl /*struct*/ QByteRef {
  pub fn operator_not_equal_0<RetType, T: QByteRef_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QByteRef_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QByteRef) -> RetType;
}
impl<'a> /*trait*/ QByteRef_operator_not_equal_0<bool> for (i8) {
  fn operator_not_equal_0(self , rsthis: & QByteRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QByteRefneEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:545
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>(char) const

/*

*/
impl /*struct*/ QByteRef {
  pub fn operator_greater_than_0<RetType, T: QByteRef_operator_greater_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_0(self);
    // return 1;
  }
}
pub trait QByteRef_operator_greater_than_0<RetType> {
  fn operator_greater_than_0(self , rsthis: & QByteRef) -> RetType;
}
impl<'a> /*trait*/ QByteRef_operator_greater_than_0<bool> for (i8) {
  fn operator_greater_than_0(self , rsthis: & QByteRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QByteRefgtEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:547
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(char) const

/*

*/
impl /*struct*/ QByteRef {
  pub fn operator_greater_than_equal_0<RetType, T: QByteRef_operator_greater_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_0(self);
    // return 1;
  }
}
pub trait QByteRef_operator_greater_than_equal_0<RetType> {
  fn operator_greater_than_equal_0(self , rsthis: & QByteRef) -> RetType;
}
impl<'a> /*trait*/ QByteRef_operator_greater_than_equal_0<bool> for (i8) {
  fn operator_greater_than_equal_0(self , rsthis: & QByteRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QByteRefgeEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:549
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(char) const

/*

*/
impl /*struct*/ QByteRef {
  pub fn operator_less_than_0<RetType, T: QByteRef_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QByteRef_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QByteRef) -> RetType;
}
impl<'a> /*trait*/ QByteRef_operator_less_than_0<bool> for (i8) {
  fn operator_less_than_0(self , rsthis: & QByteRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QByteRefltEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:551
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(char) const

/*

*/
impl /*struct*/ QByteRef {
  pub fn operator_less_than_equal_0<RetType, T: QByteRef_operator_less_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_0(self);
    // return 1;
  }
}
pub trait QByteRef_operator_less_than_equal_0<RetType> {
  fn operator_less_than_equal_0(self , rsthis: & QByteRef) -> RetType;
}
impl<'a> /*trait*/ QByteRef_operator_less_than_equal_0<bool> for (i8) {
  fn operator_less_than_equal_0(self , rsthis: & QByteRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QByteRefleEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQByteRef(this :*mut QByteRef) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN8QByteRefD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
