

// mod ::gui::QPlatformSurfaceEvent
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
#[derive(Default)] // class sizeof(QPlatformSurfaceEvent)=24
pub struct QPlatformSurfaceEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPlatformSurfaceEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QPlatformSurfaceEvent_PTR() *QPlatformSurfaceEvent
//}
//func (ptr *QPlatformSurfaceEvent) QPlatformSurfaceEvent_PTR() *QPlatformSurfaceEvent { return ptr }

impl /*struct*/ QPlatformSurfaceEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPlatformSurfaceEvent {
    return QPlatformSurfaceEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPlatformSurfaceEvent {
//  type Target = QPlatformSurfaceEventBASE;
//
//  fn deref(&self) -> &QPlatformSurfaceEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPlatformSurfaceEventBASE> for QPlatformSurfaceEvent {
//  fn as_ref(& self) -> & QPlatformSurfaceEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:451
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPlatformSurfaceEvent(QPlatformSurfaceEvent::SurfaceEventType)

/*

*/
// QPlatformSurfaceEvent(QPlatformSurfaceEvent::SurfaceEventType) ctx.fn_proto_cpp
impl /*struct*/ QPlatformSurfaceEvent {
  pub fn QPlatformSurfaceEvent_0<T: QPlatformSurfaceEvent_QPlatformSurfaceEvent_0>(value: T) -> QPlatformSurfaceEvent {
    let rsthis = value.QPlatformSurfaceEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPlatformSurfaceEvent_QPlatformSurfaceEvent_0 {
  fn QPlatformSurfaceEvent_0(self) -> QPlatformSurfaceEvent;
}
// QPlatformSurfaceEvent(QPlatformSurfaceEvent::SurfaceEventType) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPlatformSurfaceEvent_QPlatformSurfaceEvent_0 for (i32) {
  fn QPlatformSurfaceEvent_0(self) -> QPlatformSurfaceEvent {
    // unsafe{_ZN21QPlatformSurfaceEventC2ENS_16SurfaceEventTypeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QPlatformSurfaceEventC2ENS_16SurfaceEventTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPlatformSurfaceEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:452
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPlatformSurfaceEvent()

/*

*/
pub fn DeleteQPlatformSurfaceEvent(this :*mut QPlatformSurfaceEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN21QPlatformSurfaceEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:454
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QPlatformSurfaceEvent::SurfaceEventType surfaceEventType() const

/*

*/
impl /*struct*/ QPlatformSurfaceEvent {
  pub fn surfaceEventType_0<RetType, T: QPlatformSurfaceEvent_surfaceEventType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.surfaceEventType_0(self);
    // return 1;
  }
}
pub trait QPlatformSurfaceEvent_surfaceEventType_0<RetType> {
  fn surfaceEventType_0(self , rsthis: & QPlatformSurfaceEvent) -> RetType;
}
impl<'a> /*trait*/ QPlatformSurfaceEvent_surfaceEventType_0<i32> for () {
  fn surfaceEventType_0(self , rsthis: & QPlatformSurfaceEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QPlatformSurfaceEvent16surfaceEventTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QPlatformSurfaceEvent__SurfaceEventType = i32;
// 
pub const QPlatformSurfaceEvent__SurfaceCreated :QPlatformSurfaceEvent__SurfaceEventType = 0;
// 
pub const QPlatformSurfaceEvent__SurfaceAboutToBeDestroyed :QPlatformSurfaceEvent__SurfaceEventType = 1;
pub fn QPlatformSurfaceEvent_SurfaceEventTypeItemName(val: i32) ->String {
  match val {
     QPlatformSurfaceEvent__SurfaceCreated => // 0
     {return String::from("SurfaceCreated");}
     QPlatformSurfaceEvent__SurfaceAboutToBeDestroyed => // 1
     {return String::from("SurfaceAboutToBeDestroyed");}
  _ => {return format!("{}", val);}
}
}
pub fn QPlatformSurfaceEvent_SurfaceEventTypeItemName_s(val: i32) ->String {
  //var nilthis *QPlatformSurfaceEvent
  //return nilthis.SurfaceEventTypeItemName(val);
  return QPlatformSurfaceEvent_SurfaceEventTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
