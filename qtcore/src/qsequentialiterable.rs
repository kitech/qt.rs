

// mod ::core::QSequentialIterable
// package qtcore
// /usr/include/qt/QtCore/qvariant.h
// #include <qvariant.h>
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
#[derive(Default)] // class sizeof(QSequentialIterable)=104
pub struct QSequentialIterable {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSequentialIterable_ITF interface {
//    QSequentialIterable_PTR() *QSequentialIterable
//}
//func (ptr *QSequentialIterable) QSequentialIterable_PTR() *QSequentialIterable { return ptr }

impl /*struct*/ QSequentialIterable {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSequentialIterable {
    return QSequentialIterable{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSequentialIterable {
//  type Target = QSequentialIterableBASE;
//
//  fn deref(&self) -> &QSequentialIterableBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSequentialIterableBASE> for QSequentialIterable {
//  fn as_ref(& self) -> & QSequentialIterableBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qvariant.h:623
// index:0
// Public Visibility=Default Availability=Available
// [112] QSequentialIterable::const_iterator begin() const

/*

*/
impl /*struct*/ QSequentialIterable {
  pub fn begin_0<RetType, T: QSequentialIterable_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QSequentialIterable_begin_0<RetType> {
  fn begin_0(self , rsthis: & QSequentialIterable) -> RetType;
}
impl<'a> /*trait*/ QSequentialIterable_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QSequentialIterable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QSequentialIterable5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:624
// index:0
// Public Visibility=Default Availability=Available
// [112] QSequentialIterable::const_iterator end() const

/*

*/
impl /*struct*/ QSequentialIterable {
  pub fn end_0<RetType, T: QSequentialIterable_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QSequentialIterable_end_0<RetType> {
  fn end_0(self , rsthis: & QSequentialIterable) -> RetType;
}
impl<'a> /*trait*/ QSequentialIterable_end_0<usize> for () {
  fn end_0(self , rsthis: & QSequentialIterable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QSequentialIterable3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:626
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant at(int) const

/*

*/
impl /*struct*/ QSequentialIterable {
  pub fn at_0<RetType, T: QSequentialIterable_at_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.at_0(self);
    // return 1;
  }
}
pub trait QSequentialIterable_at_0<RetType> {
  fn at_0(self , rsthis: & QSequentialIterable) -> RetType;
}
impl<'a> /*trait*/ QSequentialIterable_at_0<usize> for (i32) {
  fn at_0(self , rsthis: & QSequentialIterable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QSequentialIterable2atEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:627
// index:0
// Public Visibility=Default Availability=Available
// [4] int size() const

/*

*/
impl /*struct*/ QSequentialIterable {
  pub fn size_0<RetType, T: QSequentialIterable_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QSequentialIterable_size_0<RetType> {
  fn size_0(self , rsthis: & QSequentialIterable) -> RetType;
}
impl<'a> /*trait*/ QSequentialIterable_size_0<i32> for () {
  fn size_0(self , rsthis: & QSequentialIterable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QSequentialIterable4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:629
// index:0
// Public Visibility=Default Availability=Available
// [1] bool canReverseIterate() const

/*

*/
impl /*struct*/ QSequentialIterable {
  pub fn canReverseIterate_0<RetType, T: QSequentialIterable_canReverseIterate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canReverseIterate_0(self);
    // return 1;
  }
}
pub trait QSequentialIterable_canReverseIterate_0<RetType> {
  fn canReverseIterate_0(self , rsthis: & QSequentialIterable) -> RetType;
}
impl<'a> /*trait*/ QSequentialIterable_canReverseIterate_0<bool> for () {
  fn canReverseIterate_0(self , rsthis: & QSequentialIterable) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QSequentialIterable17canReverseIterateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQSequentialIterable(this :*mut QSequentialIterable) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN19QSequentialIterableD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
