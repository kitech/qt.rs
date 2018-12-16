

// mod ::core::QSemaphoreReleaser
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
// extern C begin: 7
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
#[derive(Default)] // class sizeof(QSemaphoreReleaser)=16
pub struct QSemaphoreReleaser {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSemaphoreReleaser_ITF interface {
//    QSemaphoreReleaser_PTR() *QSemaphoreReleaser
//}
//func (ptr *QSemaphoreReleaser) QSemaphoreReleaser_PTR() *QSemaphoreReleaser { return ptr }

impl /*struct*/ QSemaphoreReleaser {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSemaphoreReleaser {
    return QSemaphoreReleaser{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSemaphoreReleaser {
//  type Target = QSemaphoreReleaserBASE;
//
//  fn deref(&self) -> &QSemaphoreReleaserBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSemaphoreReleaserBASE> for QSemaphoreReleaser {
//  fn as_ref(& self) -> & QSemaphoreReleaserBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsemaphore.h:75
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QSemaphoreReleaser()

/*

*/
// QSemaphoreReleaser() ctx.fn_proto_cpp
impl /*struct*/ QSemaphoreReleaser {
  pub fn QSemaphoreReleaser_0<T: QSemaphoreReleaser_QSemaphoreReleaser_0>(value: T) -> QSemaphoreReleaser {
    let rsthis = value.QSemaphoreReleaser_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSemaphoreReleaser_QSemaphoreReleaser_0 {
  fn QSemaphoreReleaser_0(self) -> QSemaphoreReleaser;
}
// QSemaphoreReleaser() ctx.fn_proto_cpp
impl<'a> /*trait*/ QSemaphoreReleaser_QSemaphoreReleaser_0 for () {
  fn QSemaphoreReleaser_0(self) -> QSemaphoreReleaser {
    // unsafe{_ZN18QSemaphoreReleaserC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QSemaphoreReleaserC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSemaphoreReleaser{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsemaphore.h:76
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QSemaphoreReleaser(QSemaphore &, int)

/*

*/
// QSemaphoreReleaser(QSemaphore &, int) ctx.fn_proto_cpp
impl /*struct*/ QSemaphoreReleaser {
  pub fn QSemaphoreReleaser_1<T: QSemaphoreReleaser_QSemaphoreReleaser_1>(value: T) -> QSemaphoreReleaser {
    let rsthis = value.QSemaphoreReleaser_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSemaphoreReleaser_QSemaphoreReleaser_1 {
  fn QSemaphoreReleaser_1(self) -> QSemaphoreReleaser;
}
// QSemaphoreReleaser(QSemaphore &, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSemaphoreReleaser_QSemaphoreReleaser_1 for (usize,i32) {
  fn QSemaphoreReleaser_1(self) -> QSemaphoreReleaser {
    // unsafe{_ZN18QSemaphoreReleaserC2ER10QSemaphorei()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QSemaphoreReleaserC2ER10QSemaphorei", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSemaphoreReleaser{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsemaphore.h:78
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QSemaphoreReleaser(QSemaphore *, int)

/*

*/
// QSemaphoreReleaser(QSemaphore *, int) ctx.fn_proto_cpp
impl /*struct*/ QSemaphoreReleaser {
  pub fn QSemaphoreReleaser_2<T: QSemaphoreReleaser_QSemaphoreReleaser_2>(value: T) -> QSemaphoreReleaser {
    let rsthis = value.QSemaphoreReleaser_2();
    return rsthis;
    // return 1;
  }
}

pub trait QSemaphoreReleaser_QSemaphoreReleaser_2 {
  fn QSemaphoreReleaser_2(self) -> QSemaphoreReleaser;
}
// QSemaphoreReleaser(QSemaphore *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSemaphoreReleaser_QSemaphoreReleaser_2 for (usize,i32) {
  fn QSemaphoreReleaser_2(self) -> QSemaphoreReleaser {
    // unsafe{_ZN18QSemaphoreReleaserC2EP10QSemaphorei()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QSemaphoreReleaserC2EP10QSemaphorei", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSemaphoreReleaser{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsemaphore.h:86
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~QSemaphoreReleaser()

/*

*/
pub fn DeleteQSemaphoreReleaser(this :*mut QSemaphoreReleaser) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QSemaphoreReleaserD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qsemaphore.h:92
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QSemaphoreReleaser &)

/*

*/
impl /*struct*/ QSemaphoreReleaser {
  pub fn swap_0<RetType, T: QSemaphoreReleaser_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QSemaphoreReleaser_swap_0<RetType> {
  fn swap_0(self , rsthis: & QSemaphoreReleaser) -> RetType;
}
impl<'a> /*trait*/ QSemaphoreReleaser_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QSemaphoreReleaser) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QSemaphoreReleaser4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsemaphore.h:98
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSemaphore * semaphore() const

/*

*/
impl /*struct*/ QSemaphoreReleaser {
  pub fn semaphore_0<RetType, T: QSemaphoreReleaser_semaphore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.semaphore_0(self);
    // return 1;
  }
}
pub trait QSemaphoreReleaser_semaphore_0<RetType> {
  fn semaphore_0(self , rsthis: & QSemaphoreReleaser) -> RetType;
}
impl<'a> /*trait*/ QSemaphoreReleaser_semaphore_0<usize> for () {
  fn semaphore_0(self , rsthis: & QSemaphoreReleaser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QSemaphoreReleaser9semaphoreEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsemaphore.h:101
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QSemaphore * cancel()

/*

*/
impl /*struct*/ QSemaphoreReleaser {
  pub fn cancel_0<RetType, T: QSemaphoreReleaser_cancel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cancel_0(self);
    // return 1;
  }
}
pub trait QSemaphoreReleaser_cancel_0<RetType> {
  fn cancel_0(self , rsthis: & QSemaphoreReleaser) -> RetType;
}
impl<'a> /*trait*/ QSemaphoreReleaser_cancel_0<usize> for () {
  fn cancel_0(self , rsthis: & QSemaphoreReleaser) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QSemaphoreReleaser6cancelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
