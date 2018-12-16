

// mod ::core::QAbstractNativeEventFilter
// package qtcore
// /usr/include/qt/QtCore/qabstractnativeeventfilter.h
// #include <qabstractnativeeventfilter.h>
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
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QAbstractNativeEventFilter)=16
pub struct QAbstractNativeEventFilter {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractNativeEventFilter_ITF interface {
//    QAbstractNativeEventFilter_PTR() *QAbstractNativeEventFilter
//}
//func (ptr *QAbstractNativeEventFilter) QAbstractNativeEventFilter_PTR() *QAbstractNativeEventFilter { return ptr }

impl /*struct*/ QAbstractNativeEventFilter {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractNativeEventFilter {
    return QAbstractNativeEventFilter{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractNativeEventFilter {
//  type Target = QAbstractNativeEventFilterBASE;
//
//  fn deref(&self) -> &QAbstractNativeEventFilterBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractNativeEventFilterBASE> for QAbstractNativeEventFilter {
//  fn as_ref(& self) -> & QAbstractNativeEventFilterBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qabstractnativeeventfilter.h:52
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractNativeEventFilter()

/*
Creates a native event filter.

By default this doesn't do anything. Remember to install it on the application object.
*/
// QAbstractNativeEventFilter() ctx.fn_proto_cpp
impl /*struct*/ QAbstractNativeEventFilter {
  pub fn QAbstractNativeEventFilter_0<T: QAbstractNativeEventFilter_QAbstractNativeEventFilter_0>(value: T) -> QAbstractNativeEventFilter {
    let rsthis = value.QAbstractNativeEventFilter_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractNativeEventFilter_QAbstractNativeEventFilter_0 {
  fn QAbstractNativeEventFilter_0(self) -> QAbstractNativeEventFilter;
}
// QAbstractNativeEventFilter() ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractNativeEventFilter_QAbstractNativeEventFilter_0 for () {
  fn QAbstractNativeEventFilter_0(self) -> QAbstractNativeEventFilter {
    // unsafe{_ZN26QAbstractNativeEventFilterC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QAbstractNativeEventFilterC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractNativeEventFilter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractnativeeventfilter.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractNativeEventFilter()

/*

*/
pub fn DeleteQAbstractNativeEventFilter(this :*mut QAbstractNativeEventFilter) {
    // let rv = qtrt::InvokeQtFunc6("_ZN26QAbstractNativeEventFilterD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qabstractnativeeventfilter.h:55
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool nativeEventFilter(const QByteArray &, void *, long *)

/*
This method is called for every native event.

Note: The filter function here receives native messages, for example, MSG or XCB event structs.

It is called by the QPA platform plugin. On Windows, it is called by the event dispatcher.

The type of event eventType is specific to the platform plugin chosen at run-time, and can be used to cast message to the right type.

On X11, eventType is set to "xcb_generic_event_t", and the message can be casted to a xcb_generic_event_t pointer.

On Windows, eventType is set to "windows_generic_MSG" for messages sent to toplevel windows, and "windows_dispatcher_MSG" for system-wide messages such as messages from a registered hot key. In both cases, the message can be casted to a MSG pointer. The result pointer is only used on Windows, and corresponds to the LRESULT pointer.

On macOS, eventType is set to "mac_generic_NSEvent", and the message can be casted to an NSEvent pointer.

In your reimplementation of this function, if you want to filter the message out, i.e. stop it being handled further, return true; otherwise return false.

Linux example


  class MyXcbEventFilter : public QAbstractNativeEventFilter
  {
  public:
      bool nativeEventFilter(const QByteArray &eventType, void *message, long *) override
      {
          if (eventType == "xcb_generic_event_t") {
              xcb_generic_event_t* ev = static_cast<xcb_generic_event_t *>(message);
              // ...
          }
          return false;
      }
  };



macOS example

mycocoaeventfilter.h:


  #include <QAbstractNativeEventFilter>

  class MyCocoaEventFilter : public QAbstractNativeEventFilter
  {
  public:
      bool nativeEventFilter(const QByteArray &eventType, void *message, long *) override;
  };



mycocoaeventfilter.mm:


  #include "mycocoaeventfilter.h"

  #import <AppKit/AppKit.h>

  bool CocoaNativeEventFilter::nativeEventFilter(const QByteArray &eventType, void *message, long *)
  {
      if (eventType == "mac_generic_NSEvent") {
          NSEvent *event = static_cast<NSEvent *>(message);
          if ([event type] == NSKeyDown) {
              // Handle key event
              qDebug() << QString::fromNSString([event characters]);
          }
      }
      return false;
  }



myapp.pro:


  HEADERS += mycocoaeventfilter.h
  OBJECTIVE_SOURCES += mycocoaeventfilter.mm
  LIBS += -framework AppKit
*/
impl /*struct*/ QAbstractNativeEventFilter {
  pub fn nativeEventFilter_0<RetType, T: QAbstractNativeEventFilter_nativeEventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nativeEventFilter_0(self);
    // return 1;
  }
}
pub trait QAbstractNativeEventFilter_nativeEventFilter_0<RetType> {
  fn nativeEventFilter_0(self , rsthis: & QAbstractNativeEventFilter) -> RetType;
}
impl<'a> /*trait*/ QAbstractNativeEventFilter_nativeEventFilter_0<bool> for (usize,usize,usize) {
  fn nativeEventFilter_0(self , rsthis: & QAbstractNativeEventFilter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN26QAbstractNativeEventFilter17nativeEventFilterERK10QByteArrayPvPl", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
