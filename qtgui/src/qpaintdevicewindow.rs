

// mod ::gui::QPaintDeviceWindow
// package qtgui
// /usr/include/qt/QtGui/qpaintdevicewindow.h
// #include <qpaintdevicewindow.h>
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

// void paintEvent(QPaintEvent *)
// func (this *QPaintDeviceWindow) InheritPaintEvent(f func(event *QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// int metric(QPaintDevice::PaintDeviceMetric)
// func (this *QPaintDeviceWindow) InheritMetric(f func(metric int) int) {
//  qtrt.SetAllInheritCallback(this, "metric", f)
// }

// void exposeEvent(QExposeEvent *)
// func (this *QPaintDeviceWindow) InheritExposeEvent(f func(arg0 *QExposeEvent/*777 QExposeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "exposeEvent", f)
// }

// bool event(QEvent *)
// func (this *QPaintDeviceWindow) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QPaintDeviceWindow)=64
pub struct QPaintDeviceWindow {
  qbase: QWindow,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPaintDeviceWindow_ITF interface {
//    QWindow_ITF
//    QPaintDevice_ITF
//    QPaintDeviceWindow_PTR() *QPaintDeviceWindow
//}
//func (ptr *QPaintDeviceWindow) QPaintDeviceWindow_PTR() *QPaintDeviceWindow { return ptr }

impl /*struct*/ QPaintDeviceWindow {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPaintDeviceWindow {
    return QPaintDeviceWindow{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPaintDeviceWindow {
//  type Target = QPaintDeviceWindowBASE;
//
//  fn deref(&self) -> &QPaintDeviceWindowBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPaintDeviceWindowBASE> for QPaintDeviceWindow {
//  fn as_ref(& self) -> & QPaintDeviceWindowBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpaintdevicewindow.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QPaintDeviceWindow {
  pub fn metaObject_0<RetType, T: QPaintDeviceWindow_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QPaintDeviceWindow_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QPaintDeviceWindow) -> RetType;
}
impl<'a> /*trait*/ QPaintDeviceWindow_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QPaintDeviceWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QPaintDeviceWindow10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevicewindow.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void update(const QRect &)

/*
Marks the rect of the window as dirty and schedules a repaint.

Note: Subsequent calls to this function before the next paint event will get ignored, but rect is added to the region to update.

Note: For non-exposed windows the update is deferred until the window becomes exposed again.
*/
impl /*struct*/ QPaintDeviceWindow {
  pub fn update_0<RetType, T: QPaintDeviceWindow_update_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_0(self);
    // return 1;
  }
}
pub trait QPaintDeviceWindow_update_0<RetType> {
  fn update_0(self , rsthis: & QPaintDeviceWindow) -> RetType;
}
impl<'a> /*trait*/ QPaintDeviceWindow_update_0<(/*void*/)> for (usize) {
  fn update_0(self , rsthis: & QPaintDeviceWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QPaintDeviceWindow6updateERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintdevicewindow.h:59
// index:1
// Public Visibility=Default Availability=Available
// [-2] void update(const QRegion &)

/*
Marks the rect of the window as dirty and schedules a repaint.

Note: Subsequent calls to this function before the next paint event will get ignored, but rect is added to the region to update.

Note: For non-exposed windows the update is deferred until the window becomes exposed again.
*/
impl /*struct*/ QPaintDeviceWindow {
  pub fn update_1<RetType, T: QPaintDeviceWindow_update_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_1(self);
    // return 1;
  }
}
pub trait QPaintDeviceWindow_update_1<RetType> {
  fn update_1(self , rsthis: & QPaintDeviceWindow) -> RetType;
}
impl<'a> /*trait*/ QPaintDeviceWindow_update_1<(/*void*/)> for (usize) {
  fn update_1(self , rsthis: & QPaintDeviceWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QPaintDeviceWindow6updateERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintdevicewindow.h:66
// index:2
// Public Visibility=Default Availability=Available
// [-2] void update()

/*
Marks the rect of the window as dirty and schedules a repaint.

Note: Subsequent calls to this function before the next paint event will get ignored, but rect is added to the region to update.

Note: For non-exposed windows the update is deferred until the window becomes exposed again.
*/
impl /*struct*/ QPaintDeviceWindow {
  pub fn update_2<RetType, T: QPaintDeviceWindow_update_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_2(self);
    // return 1;
  }
}
pub trait QPaintDeviceWindow_update_2<RetType> {
  fn update_2(self , rsthis: & QPaintDeviceWindow) -> RetType;
}
impl<'a> /*trait*/ QPaintDeviceWindow_update_2<(/*void*/)> for () {
  fn update_2(self , rsthis: & QPaintDeviceWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QPaintDeviceWindow6updateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintdevicewindow.h:69
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Handles paint events passed in the event parameter.

The default implementation does nothing. Reimplement this function to perform painting. If necessary, the dirty area is retrievable from the event.
*/
impl /*struct*/ QPaintDeviceWindow {
  pub fn paintEvent_0<RetType, T: QPaintDeviceWindow_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QPaintDeviceWindow_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QPaintDeviceWindow) -> RetType;
}
impl<'a> /*trait*/ QPaintDeviceWindow_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QPaintDeviceWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QPaintDeviceWindow10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintdevicewindow.h:71
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int metric(QPaintDevice::PaintDeviceMetric) const

/*

*/
impl /*struct*/ QPaintDeviceWindow {
  pub fn metric_0<RetType, T: QPaintDeviceWindow_metric_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metric_0(self);
    // return 1;
  }
}
pub trait QPaintDeviceWindow_metric_0<RetType> {
  fn metric_0(self , rsthis: & QPaintDeviceWindow) -> RetType;
}
impl<'a> /*trait*/ QPaintDeviceWindow_metric_0<i32> for (i32) {
  fn metric_0(self , rsthis: & QPaintDeviceWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QPaintDeviceWindow6metricEN12QPaintDevice17PaintDeviceMetricE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevicewindow.h:72
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void exposeEvent(QExposeEvent *)

/*

*/
impl /*struct*/ QPaintDeviceWindow {
  pub fn exposeEvent_0<RetType, T: QPaintDeviceWindow_exposeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exposeEvent_0(self);
    // return 1;
  }
}
pub trait QPaintDeviceWindow_exposeEvent_0<RetType> {
  fn exposeEvent_0(self , rsthis: & QPaintDeviceWindow) -> RetType;
}
impl<'a> /*trait*/ QPaintDeviceWindow_exposeEvent_0<(/*void*/)> for (usize) {
  fn exposeEvent_0(self , rsthis: & QPaintDeviceWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QPaintDeviceWindow11exposeEventEP12QExposeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintdevicewindow.h:73
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*

*/
impl /*struct*/ QPaintDeviceWindow {
  pub fn event_0<RetType, T: QPaintDeviceWindow_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QPaintDeviceWindow_event_0<RetType> {
  fn event_0(self , rsthis: & QPaintDeviceWindow) -> RetType;
}
impl<'a> /*trait*/ QPaintDeviceWindow_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QPaintDeviceWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QPaintDeviceWindow5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQPaintDeviceWindow(this :*mut QPaintDeviceWindow) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN18QPaintDeviceWindowD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
