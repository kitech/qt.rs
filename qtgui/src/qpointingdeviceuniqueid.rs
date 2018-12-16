

// mod ::gui::QPointingDeviceUniqueId
// package qtgui
// /usr/include/qt/QtGui/qevent.h
// #include <qevent.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 4
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QPointingDeviceUniqueId)=8
pub struct QPointingDeviceUniqueId {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPointingDeviceUniqueId_ITF interface {
//    QPointingDeviceUniqueId_PTR() *QPointingDeviceUniqueId
//}
//func (ptr *QPointingDeviceUniqueId) QPointingDeviceUniqueId_PTR() *QPointingDeviceUniqueId { return ptr }

impl /*struct*/ QPointingDeviceUniqueId {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPointingDeviceUniqueId {
    return QPointingDeviceUniqueId{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPointingDeviceUniqueId {
//  type Target = QPointingDeviceUniqueIdBASE;
//
//  fn deref(&self) -> &QPointingDeviceUniqueIdBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPointingDeviceUniqueIdBASE> for QPointingDeviceUniqueId {
//  fn as_ref(& self) -> & QPointingDeviceUniqueIdBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:809
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QPointingDeviceUniqueId()

/*

*/
// QPointingDeviceUniqueId() ctx.fn_proto_cpp
impl /*struct*/ QPointingDeviceUniqueId {
  pub fn QPointingDeviceUniqueId_0<T: QPointingDeviceUniqueId_QPointingDeviceUniqueId_0>(value: T) -> QPointingDeviceUniqueId {
    let rsthis = value.QPointingDeviceUniqueId_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPointingDeviceUniqueId_QPointingDeviceUniqueId_0 {
  fn QPointingDeviceUniqueId_0(self) -> QPointingDeviceUniqueId;
}
// QPointingDeviceUniqueId() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPointingDeviceUniqueId_QPointingDeviceUniqueId_0 for () {
  fn QPointingDeviceUniqueId_0(self) -> QPointingDeviceUniqueId {
    // unsafe{_ZN23QPointingDeviceUniqueIdC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QPointingDeviceUniqueIdC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPointingDeviceUniqueId{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:813
// index:0
// Public static Visibility=Default Availability=Available
// [8] QPointingDeviceUniqueId fromNumericId(qint64)

/*

*/
impl /*struct*/ QPointingDeviceUniqueId {
  pub fn fromNumericId_0<RetType, T: QPointingDeviceUniqueId_fromNumericId_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromNumericId_0();
    // return 1;
  }
}
pub trait QPointingDeviceUniqueId_fromNumericId_0<RetType> {
  fn fromNumericId_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPointingDeviceUniqueId_fromNumericId_0<usize> for (i64) {
  fn fromNumericId_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QPointingDeviceUniqueId13fromNumericIdEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:815
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QPointingDeviceUniqueId {
  pub fn isValid_0<RetType, T: QPointingDeviceUniqueId_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QPointingDeviceUniqueId_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QPointingDeviceUniqueId) -> RetType;
}
impl<'a> /*trait*/ QPointingDeviceUniqueId_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QPointingDeviceUniqueId) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QPointingDeviceUniqueId7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:816
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 numericId() const

/*

*/
impl /*struct*/ QPointingDeviceUniqueId {
  pub fn numericId_0<RetType, T: QPointingDeviceUniqueId_numericId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.numericId_0(self);
    // return 1;
  }
}
pub trait QPointingDeviceUniqueId_numericId_0<RetType> {
  fn numericId_0(self , rsthis: & QPointingDeviceUniqueId) -> RetType;
}
impl<'a> /*trait*/ QPointingDeviceUniqueId_numericId_0<i64> for () {
  fn numericId_0(self , rsthis: & QPointingDeviceUniqueId) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QPointingDeviceUniqueId9numericIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}


pub fn DeleteQPointingDeviceUniqueId(this :*mut QPointingDeviceUniqueId) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN23QPointingDeviceUniqueIdD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
