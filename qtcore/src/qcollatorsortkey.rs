

// mod ::core::QCollatorSortKey
// package qtcore
// /usr/include/qt/QtCore/qcollator.h
// #include <qcollator.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 109
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
#[derive(Default)] // class sizeof(QCollatorSortKey)=8
pub struct QCollatorSortKey {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCollatorSortKey_ITF interface {
//    QCollatorSortKey_PTR() *QCollatorSortKey
//}
//func (ptr *QCollatorSortKey) QCollatorSortKey_PTR() *QCollatorSortKey { return ptr }

impl /*struct*/ QCollatorSortKey {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCollatorSortKey {
    return QCollatorSortKey{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCollatorSortKey {
//  type Target = QCollatorSortKeyBASE;
//
//  fn deref(&self) -> &QCollatorSortKeyBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCollatorSortKeyBASE> for QCollatorSortKey {
//  fn as_ref(& self) -> & QCollatorSortKeyBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qcollator.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QCollatorSortKey()

/*

*/
pub fn DeleteQCollatorSortKey(this :*mut QCollatorSortKey) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QCollatorSortKeyD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qcollator.h:59
// index:0
// Public Visibility=Default Availability=Available
// [8] QCollatorSortKey & operator=(const QCollatorSortKey &)

/*

*/
impl /*struct*/ QCollatorSortKey {
  pub fn operator_equal_0<RetType, T: QCollatorSortKey_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QCollatorSortKey_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QCollatorSortKey) -> RetType;
}
impl<'a> /*trait*/ QCollatorSortKey_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QCollatorSortKey) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCollatorSortKeyaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcollator.h:61
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QCollatorSortKey & operator=(QCollatorSortKey &&)

/*

*/
impl /*struct*/ QCollatorSortKey {
  pub fn operator_equal_1<RetType, T: QCollatorSortKey_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QCollatorSortKey_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QCollatorSortKey) -> RetType;
}
impl<'a> /*trait*/ QCollatorSortKey_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QCollatorSortKey) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QCollatorSortKeyaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcollator.h:64
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QCollatorSortKey &)

/*
Swaps this collator with other. This function is very fast and never fails.
*/
impl /*struct*/ QCollatorSortKey {
  pub fn swap_0<RetType, T: QCollatorSortKey_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QCollatorSortKey_swap_0<RetType> {
  fn swap_0(self , rsthis: & QCollatorSortKey) -> RetType;
}
impl<'a> /*trait*/ QCollatorSortKey_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QCollatorSortKey) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QCollatorSortKey4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcollator.h:67
// index:0
// Public Visibility=Default Availability=Available
// [4] int compare(const QCollatorSortKey &) const

/*
Compares s1 with s2. Returns an integer less than, equal to, or greater than zero depending on whether s1 is smaller, equal or larger than s2.
*/
impl /*struct*/ QCollatorSortKey {
  pub fn compare_0<RetType, T: QCollatorSortKey_compare_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compare_0(self);
    // return 1;
  }
}
pub trait QCollatorSortKey_compare_0<RetType> {
  fn compare_0(self , rsthis: & QCollatorSortKey) -> RetType;
}
impl<'a> /*trait*/ QCollatorSortKey_compare_0<i32> for (usize) {
  fn compare_0(self , rsthis: & QCollatorSortKey) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QCollatorSortKey7compareERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
