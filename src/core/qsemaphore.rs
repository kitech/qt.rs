

// mod ::core::QSemaphore
// package qtcore
// /usr/include/qt/QtCore/qsemaphore.h
// #include <qsemaphore.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 13
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
#[derive(Default)] // class sizeof(QSemaphore)=8
pub struct QSemaphore {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSemaphore_ITF interface {
//    QSemaphore_PTR() *QSemaphore
//}
//func (ptr *QSemaphore) QSemaphore_PTR() *QSemaphore { return ptr }

impl /*struct*/ QSemaphore {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSemaphore {
    return QSemaphore{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSemaphore {
//  type Target = QSemaphoreBASE;
//
//  fn deref(&self) -> &QSemaphoreBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSemaphoreBASE> for QSemaphore {
//  fn as_ref(& self) -> & QSemaphoreBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsemaphore.h:55
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSemaphore(int)

/*
Creates a new semaphore and initializes the number of resources it guards to n (by default, 0).

See also release() and available().
*/
// QSemaphore(int) ctx.fn_proto_cpp
impl /*struct*/ QSemaphore {
  pub fn QSemaphore_0<T: QSemaphore_QSemaphore_0>(value: T) -> QSemaphore {
    let rsthis = value.QSemaphore_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSemaphore_QSemaphore_0 {
  fn QSemaphore_0(self) -> QSemaphore;
}
// QSemaphore(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSemaphore_QSemaphore_0 for (i32) {
  fn QSemaphore_0(self) -> QSemaphore {
    // unsafe{_ZN10QSemaphoreC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QSemaphoreC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSemaphore{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsemaphore.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QSemaphore()

/*

*/
pub fn DeleteQSemaphore(this :*mut QSemaphore) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QSemaphoreD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qsemaphore.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void acquire(int)

/*
Tries to acquire n resources guarded by the semaphore. If n > available(), this call will block until enough resources are available.

See also release(), available(), and tryAcquire().
*/
impl /*struct*/ QSemaphore {
  pub fn acquire_0<RetType, T: QSemaphore_acquire_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.acquire_0(self);
    // return 1;
  }
}
pub trait QSemaphore_acquire_0<RetType> {
  fn acquire_0(self , rsthis: & QSemaphore) -> RetType;
}
impl<'a> /*trait*/ QSemaphore_acquire_0<(/*void*/)> for (i32) {
  fn acquire_0(self , rsthis: & QSemaphore) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QSemaphore7acquireEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsemaphore.h:59
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tryAcquire(int)

/*
Tries to acquire n resources guarded by the semaphore and returns true on success. If available() < n, this call immediately returns false without acquiring any resources.

Example:


  QSemaphore sem(5);      // sem.available() == 5
  sem.tryAcquire(250);    // sem.available() == 5, returns false
  sem.tryAcquire(3);      // sem.available() == 2, returns true



See also acquire().
*/
impl /*struct*/ QSemaphore {
  pub fn tryAcquire_0<RetType, T: QSemaphore_tryAcquire_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tryAcquire_0(self);
    // return 1;
  }
}
pub trait QSemaphore_tryAcquire_0<RetType> {
  fn tryAcquire_0(self , rsthis: & QSemaphore) -> RetType;
}
impl<'a> /*trait*/ QSemaphore_tryAcquire_0<bool> for (i32) {
  fn tryAcquire_0(self , rsthis: & QSemaphore) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QSemaphore10tryAcquireEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsemaphore.h:60
// index:1
// Public Visibility=Default Availability=Available
// [1] bool tryAcquire(int, int)

/*
Tries to acquire n resources guarded by the semaphore and returns true on success. If available() < n, this call immediately returns false without acquiring any resources.

Example:


  QSemaphore sem(5);      // sem.available() == 5
  sem.tryAcquire(250);    // sem.available() == 5, returns false
  sem.tryAcquire(3);      // sem.available() == 2, returns true



See also acquire().
*/
impl /*struct*/ QSemaphore {
  pub fn tryAcquire_1<RetType, T: QSemaphore_tryAcquire_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tryAcquire_1(self);
    // return 1;
  }
}
pub trait QSemaphore_tryAcquire_1<RetType> {
  fn tryAcquire_1(self , rsthis: & QSemaphore) -> RetType;
}
impl<'a> /*trait*/ QSemaphore_tryAcquire_1<bool> for (i32,i32) {
  fn tryAcquire_1(self , rsthis: & QSemaphore) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QSemaphore10tryAcquireEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsemaphore.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void release(int)

/*
Releases n resources guarded by the semaphore.

This function can be used to "create" resources as well. For example:


  QSemaphore sem(5);      // a semaphore that guards 5 resources
  sem.acquire(5);         // acquire all 5 resources
  sem.release(5);         // release the 5 resources
  sem.release(10);        // "create" 10 new resources



QSemaphoreReleaser is a RAII wrapper around this function.

See also acquire(), available(), and QSemaphoreReleaser.
*/
impl /*struct*/ QSemaphore {
  pub fn release_0<RetType, T: QSemaphore_release_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.release_0(self);
    // return 1;
  }
}
pub trait QSemaphore_release_0<RetType> {
  fn release_0(self , rsthis: & QSemaphore) -> RetType;
}
impl<'a> /*trait*/ QSemaphore_release_0<(/*void*/)> for (i32) {
  fn release_0(self , rsthis: & QSemaphore) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QSemaphore7releaseEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsemaphore.h:64
// index:0
// Public Visibility=Default Availability=Available
// [4] int available() const

/*
Returns the number of resources currently available to the semaphore. This number can never be negative.

See also acquire() and release().
*/
impl /*struct*/ QSemaphore {
  pub fn available_0<RetType, T: QSemaphore_available_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.available_0(self);
    // return 1;
  }
}
pub trait QSemaphore_available_0<RetType> {
  fn available_0(self , rsthis: & QSemaphore) -> RetType;
}
impl<'a> /*trait*/ QSemaphore_available_0<i32> for () {
  fn available_0(self , rsthis: & QSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QSemaphore9availableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
