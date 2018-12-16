

// mod ::gui::QWhatsThisClickedEvent
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
// extern C begin: 3
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
#[derive(Default)] // class sizeof(QWhatsThisClickedEvent)=32
pub struct QWhatsThisClickedEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWhatsThisClickedEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QWhatsThisClickedEvent_PTR() *QWhatsThisClickedEvent
//}
//func (ptr *QWhatsThisClickedEvent) QWhatsThisClickedEvent_PTR() *QWhatsThisClickedEvent { return ptr }

impl /*struct*/ QWhatsThisClickedEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWhatsThisClickedEvent {
    return QWhatsThisClickedEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWhatsThisClickedEvent {
//  type Target = QWhatsThisClickedEventBASE;
//
//  fn deref(&self) -> &QWhatsThisClickedEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWhatsThisClickedEventBASE> for QWhatsThisClickedEvent {
//  fn as_ref(& self) -> & QWhatsThisClickedEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:713
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QWhatsThisClickedEvent(const QString &)

/*

*/
// QWhatsThisClickedEvent(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QWhatsThisClickedEvent {
  pub fn QWhatsThisClickedEvent_0<T: QWhatsThisClickedEvent_QWhatsThisClickedEvent_0>(value: T) -> QWhatsThisClickedEvent {
    let rsthis = value.QWhatsThisClickedEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QWhatsThisClickedEvent_QWhatsThisClickedEvent_0 {
  fn QWhatsThisClickedEvent_0(self) -> QWhatsThisClickedEvent;
}
// QWhatsThisClickedEvent(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWhatsThisClickedEvent_QWhatsThisClickedEvent_0 for (usize) {
  fn QWhatsThisClickedEvent_0(self) -> QWhatsThisClickedEvent {
    // unsafe{_ZN22QWhatsThisClickedEventC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QWhatsThisClickedEventC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWhatsThisClickedEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:714
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QWhatsThisClickedEvent()

/*

*/
pub fn DeleteQWhatsThisClickedEvent(this :*mut QWhatsThisClickedEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN22QWhatsThisClickedEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:716
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString href() const

/*

*/
impl /*struct*/ QWhatsThisClickedEvent {
  pub fn href_0<RetType, T: QWhatsThisClickedEvent_href_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.href_0(self);
    // return 1;
  }
}
pub trait QWhatsThisClickedEvent_href_0<RetType> {
  fn href_0(self , rsthis: & QWhatsThisClickedEvent) -> RetType;
}
impl<'a> /*trait*/ QWhatsThisClickedEvent_href_0<usize> for () {
  fn href_0(self , rsthis: & QWhatsThisClickedEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QWhatsThisClickedEvent4hrefEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
