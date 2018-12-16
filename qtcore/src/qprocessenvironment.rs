

// mod ::core::QProcessEnvironment
// package qtcore
// /usr/include/qt/QtCore/qprocess.h
// #include <qprocess.h>
// #include <QtCore>

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
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QProcessEnvironment)=8
pub struct QProcessEnvironment {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QProcessEnvironment_ITF interface {
//    QProcessEnvironment_PTR() *QProcessEnvironment
//}
//func (ptr *QProcessEnvironment) QProcessEnvironment_PTR() *QProcessEnvironment { return ptr }

impl /*struct*/ QProcessEnvironment {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QProcessEnvironment {
    return QProcessEnvironment{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QProcessEnvironment {
//  type Target = QProcessEnvironmentBASE;
//
//  fn deref(&self) -> &QProcessEnvironmentBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QProcessEnvironmentBASE> for QProcessEnvironment {
//  fn as_ref(& self) -> & QProcessEnvironmentBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qprocess.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QProcessEnvironment()

/*

*/
// QProcessEnvironment() ctx.fn_proto_cpp
impl /*struct*/ QProcessEnvironment {
  pub fn QProcessEnvironment_0<T: QProcessEnvironment_QProcessEnvironment_0>(value: T) -> QProcessEnvironment {
    let rsthis = value.QProcessEnvironment_0();
    return rsthis;
    // return 1;
  }
}

pub trait QProcessEnvironment_QProcessEnvironment_0 {
  fn QProcessEnvironment_0(self) -> QProcessEnvironment;
}
// QProcessEnvironment() ctx.fn_proto_cpp
impl<'a> /*trait*/ QProcessEnvironment_QProcessEnvironment_0 for () {
  fn QProcessEnvironment_0(self) -> QProcessEnvironment {
    // unsafe{_ZN19QProcessEnvironmentC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QProcessEnvironmentC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QProcessEnvironment{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QProcessEnvironment()

/*

*/
pub fn DeleteQProcessEnvironment(this :*mut QProcessEnvironment) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QProcessEnvironmentD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qprocess.h:74
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QProcessEnvironment & operator=(QProcessEnvironment &&)

/*

*/
impl /*struct*/ QProcessEnvironment {
  pub fn operator_equal_0<RetType, T: QProcessEnvironment_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QProcessEnvironment_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QProcessEnvironment) -> RetType;
}
impl<'a> /*trait*/ QProcessEnvironment_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QProcessEnvironment) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QProcessEnvironmentaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:76
// index:1
// Public Visibility=Default Availability=Available
// [8] QProcessEnvironment & operator=(const QProcessEnvironment &)

/*

*/
impl /*struct*/ QProcessEnvironment {
  pub fn operator_equal_1<RetType, T: QProcessEnvironment_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QProcessEnvironment_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QProcessEnvironment) -> RetType;
}
impl<'a> /*trait*/ QProcessEnvironment_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QProcessEnvironment) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QProcessEnvironmentaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QProcessEnvironment &)

/*

*/
impl /*struct*/ QProcessEnvironment {
  pub fn swap_0<RetType, T: QProcessEnvironment_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QProcessEnvironment_swap_0<RetType> {
  fn swap_0(self , rsthis: & QProcessEnvironment) -> RetType;
}
impl<'a> /*trait*/ QProcessEnvironment_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QProcessEnvironment) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QProcessEnvironment4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:80
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QProcessEnvironment &) const

/*

*/
impl /*struct*/ QProcessEnvironment {
  pub fn operator_equal_equal_0<RetType, T: QProcessEnvironment_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QProcessEnvironment_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QProcessEnvironment) -> RetType;
}
impl<'a> /*trait*/ QProcessEnvironment_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QProcessEnvironment) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QProcessEnvironmenteqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QProcessEnvironment &) const

/*

*/
impl /*struct*/ QProcessEnvironment {
  pub fn operator_not_equal_0<RetType, T: QProcessEnvironment_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QProcessEnvironment_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QProcessEnvironment) -> RetType;
}
impl<'a> /*trait*/ QProcessEnvironment_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QProcessEnvironment) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QProcessEnvironmentneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:84
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*

*/
impl /*struct*/ QProcessEnvironment {
  pub fn isEmpty_0<RetType, T: QProcessEnvironment_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QProcessEnvironment_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QProcessEnvironment) -> RetType;
}
impl<'a> /*trait*/ QProcessEnvironment_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QProcessEnvironment) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QProcessEnvironment7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*

*/
impl /*struct*/ QProcessEnvironment {
  pub fn clear_0<RetType, T: QProcessEnvironment_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QProcessEnvironment_clear_0<RetType> {
  fn clear_0(self , rsthis: & QProcessEnvironment) -> RetType;
}
impl<'a> /*trait*/ QProcessEnvironment_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QProcessEnvironment) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN19QProcessEnvironment5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:87
// index:0
// Public Visibility=Default Availability=Available
// [1] bool contains(const QString &) const

/*

*/
impl /*struct*/ QProcessEnvironment {
  pub fn contains_0<RetType, T: QProcessEnvironment_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QProcessEnvironment_contains_0<RetType> {
  fn contains_0(self , rsthis: & QProcessEnvironment) -> RetType;
}
impl<'a> /*trait*/ QProcessEnvironment_contains_0<bool> for (usize) {
  fn contains_0(self , rsthis: & QProcessEnvironment) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QProcessEnvironment8containsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void remove(const QString &)

/*

*/
impl /*struct*/ QProcessEnvironment {
  pub fn remove_0<RetType, T: QProcessEnvironment_remove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_0(self);
    // return 1;
  }
}
pub trait QProcessEnvironment_remove_0<RetType> {
  fn remove_0(self , rsthis: & QProcessEnvironment) -> RetType;
}
impl<'a> /*trait*/ QProcessEnvironment_remove_0<(/*void*/)> for (usize) {
  fn remove_0(self , rsthis: & QProcessEnvironment) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QProcessEnvironment6removeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qprocess.h:90
// index:0
// Public Visibility=Default Availability=Available
// [8] QString value(const QString &, const QString &) const

/*

*/
impl /*struct*/ QProcessEnvironment {
  pub fn value_0<RetType, T: QProcessEnvironment_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QProcessEnvironment_value_0<RetType> {
  fn value_0(self , rsthis: & QProcessEnvironment) -> RetType;
}
impl<'a> /*trait*/ QProcessEnvironment_value_0<usize> for (usize,usize) {
  fn value_0(self , rsthis: & QProcessEnvironment) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QProcessEnvironment5valueERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:92
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList toStringList() const

/*

*/
impl /*struct*/ QProcessEnvironment {
  pub fn toStringList_0<RetType, T: QProcessEnvironment_toStringList_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toStringList_0(self);
    // return 1;
  }
}
pub trait QProcessEnvironment_toStringList_0<RetType> {
  fn toStringList_0(self , rsthis: & QProcessEnvironment) -> RetType;
}
impl<'a> /*trait*/ QProcessEnvironment_toStringList_0<usize> for () {
  fn toStringList_0(self , rsthis: & QProcessEnvironment) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QProcessEnvironment12toStringListEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:94
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList keys() const

/*

*/
impl /*struct*/ QProcessEnvironment {
  pub fn keys_0<RetType, T: QProcessEnvironment_keys_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keys_0(self);
    // return 1;
  }
}
pub trait QProcessEnvironment_keys_0<RetType> {
  fn keys_0(self , rsthis: & QProcessEnvironment) -> RetType;
}
impl<'a> /*trait*/ QProcessEnvironment_keys_0<usize> for () {
  fn keys_0(self , rsthis: & QProcessEnvironment) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QProcessEnvironment4keysEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qprocess.h:98
// index:0
// Public static Visibility=Default Availability=Available
// [8] QProcessEnvironment systemEnvironment()

/*
Returns the environment of the calling process as a list of key=value pairs. Example:


  QStringList environment = QProcess::systemEnvironment();
  // environment = {"PATH=/usr/bin:/usr/local/bin",
  //                "USER=greg", "HOME=/home/greg"}



This function does not cache the system environment. Therefore, it's possible to obtain an updated version of the environment if low-level C library functions like setenv or putenv have been called.

However, note that repeated calls to this function will recreate the list of environment variables, which is a non-trivial operation.

Note: For new code, it is recommended to use QProcessEnvironment::systemEnvironment()

This function was introduced in  Qt 4.1.

See also QProcessEnvironment::systemEnvironment() and setProcessEnvironment().
*/
impl /*struct*/ QProcessEnvironment {
  pub fn systemEnvironment_0<RetType, T: QProcessEnvironment_systemEnvironment_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.systemEnvironment_0();
    // return 1;
  }
}
pub trait QProcessEnvironment_systemEnvironment_0<RetType> {
  fn systemEnvironment_0(self ) -> RetType;
}
impl<'a> /*trait*/ QProcessEnvironment_systemEnvironment_0<usize> for () {
  fn systemEnvironment_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QProcessEnvironment17systemEnvironmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
