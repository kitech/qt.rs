

// mod ::gui::QRasterWindow
// package qtgui
// /usr/include/qt/QtGui/qrasterwindow.h
// #include <qrasterwindow.h>
// #include <QtGui>

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
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// int metric(QPaintDevice::PaintDeviceMetric)
// func (this *QRasterWindow) InheritMetric(f func(metric int) int) {
//  qtrt.SetAllInheritCallback(this, "metric", f)
// }

// QPaintDevice * redirected(QPoint *)
// func (this *QRasterWindow) InheritRedirected(f func(arg0 *qtcore.QPoint/*777 QPoint **/) unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "redirected", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QRasterWindow)=64
pub struct QRasterWindow {
  qbase: QPaintDeviceWindow,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRasterWindow_ITF interface {
//    QPaintDeviceWindow_ITF
//    QRasterWindow_PTR() *QRasterWindow
//}
//func (ptr *QRasterWindow) QRasterWindow_PTR() *QRasterWindow { return ptr }

impl /*struct*/ QRasterWindow {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRasterWindow {
    return QRasterWindow{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRasterWindow {
//  type Target = QRasterWindowBASE;
//
//  fn deref(&self) -> &QRasterWindowBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRasterWindowBASE> for QRasterWindow {
//  fn as_ref(& self) -> & QRasterWindowBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qrasterwindow.h:52
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QRasterWindow {
  pub fn metaObject_0<RetType, T: QRasterWindow_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QRasterWindow_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QRasterWindow) -> RetType;
}
impl<'a> /*trait*/ QRasterWindow_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QRasterWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QRasterWindow10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrasterwindow.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QRasterWindow(QWindow *)

/*
Constructs a new QRasterWindow with parent.
*/
// QRasterWindow(QWindow *) ctx.fn_proto_cpp
impl /*struct*/ QRasterWindow {
  pub fn QRasterWindow_0<T: QRasterWindow_QRasterWindow_0>(value: T) -> QRasterWindow {
    let rsthis = value.QRasterWindow_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRasterWindow_QRasterWindow_0 {
  fn QRasterWindow_0(self) -> QRasterWindow;
}
// QRasterWindow(QWindow *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRasterWindow_QRasterWindow_0 for (usize) {
  fn QRasterWindow_0(self) -> QRasterWindow {
    // unsafe{_ZN13QRasterWindowC2EP7QWindow()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QRasterWindowC2EP7QWindow", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRasterWindow{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qrasterwindow.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QRasterWindow()

/*

*/
pub fn DeleteQRasterWindow(this :*mut QRasterWindow) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QRasterWindowD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 64)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qrasterwindow.h:60
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int metric(QPaintDevice::PaintDeviceMetric) const

/*

*/
impl /*struct*/ QRasterWindow {
  pub fn metric_0<RetType, T: QRasterWindow_metric_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metric_0(self);
    // return 1;
  }
}
pub trait QRasterWindow_metric_0<RetType> {
  fn metric_0(self , rsthis: & QRasterWindow) -> RetType;
}
impl<'a> /*trait*/ QRasterWindow_metric_0<i32> for (i32) {
  fn metric_0(self , rsthis: & QRasterWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QRasterWindow6metricEN12QPaintDevice17PaintDeviceMetricE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrasterwindow.h:61
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QPaintDevice * redirected(QPoint *) const

/*

*/
impl /*struct*/ QRasterWindow {
  pub fn redirected_0<RetType, T: QRasterWindow_redirected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redirected_0(self);
    // return 1;
  }
}
pub trait QRasterWindow_redirected_0<RetType> {
  fn redirected_0(self , rsthis: & QRasterWindow) -> RetType;
}
impl<'a> /*trait*/ QRasterWindow_redirected_0<usize> for (usize) {
  fn redirected_0(self , rsthis: & QRasterWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QRasterWindow10redirectedEP6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
