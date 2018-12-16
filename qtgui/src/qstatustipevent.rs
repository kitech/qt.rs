

// mod ::gui::QStatusTipEvent
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
// extern C begin: 8
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
#[derive(Default)] // class sizeof(QStatusTipEvent)=32
pub struct QStatusTipEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStatusTipEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QStatusTipEvent_PTR() *QStatusTipEvent
//}
//func (ptr *QStatusTipEvent) QStatusTipEvent_PTR() *QStatusTipEvent { return ptr }

impl /*struct*/ QStatusTipEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStatusTipEvent {
    return QStatusTipEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStatusTipEvent {
//  type Target = QStatusTipEventBASE;
//
//  fn deref(&self) -> &QStatusTipEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStatusTipEventBASE> for QStatusTipEvent {
//  fn as_ref(& self) -> & QStatusTipEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:700
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStatusTipEvent(const QString &)

/*

*/
// QStatusTipEvent(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QStatusTipEvent {
  pub fn QStatusTipEvent_0<T: QStatusTipEvent_QStatusTipEvent_0>(value: T) -> QStatusTipEvent {
    let rsthis = value.QStatusTipEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStatusTipEvent_QStatusTipEvent_0 {
  fn QStatusTipEvent_0(self) -> QStatusTipEvent;
}
// QStatusTipEvent(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStatusTipEvent_QStatusTipEvent_0 for (usize) {
  fn QStatusTipEvent_0(self) -> QStatusTipEvent {
    // unsafe{_ZN15QStatusTipEventC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QStatusTipEventC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStatusTipEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:701
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QStatusTipEvent()

/*

*/
pub fn DeleteQStatusTipEvent(this :*mut QStatusTipEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QStatusTipEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:703
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString tip() const

/*

*/
impl /*struct*/ QStatusTipEvent {
  pub fn tip_0<RetType, T: QStatusTipEvent_tip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tip_0(self);
    // return 1;
  }
}
pub trait QStatusTipEvent_tip_0<RetType> {
  fn tip_0(self , rsthis: & QStatusTipEvent) -> RetType;
}
impl<'a> /*trait*/ QStatusTipEvent_tip_0<usize> for () {
  fn tip_0(self , rsthis: & QStatusTipEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QStatusTipEvent3tipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
