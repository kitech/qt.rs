

// mod ::gui::QScreen
// package qtgui
// /usr/include/qt/QtGui/qscreen.h
// #include <qscreen.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 5
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
#[derive(Default)] // class sizeof(QScreen)=16
pub struct QScreen {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QScreen_ITF interface {
//    qtcore.QObject_ITF
//    QScreen_PTR() *QScreen
//}
//func (ptr *QScreen) QScreen_PTR() *QScreen { return ptr }

impl /*struct*/ QScreen {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QScreen {
    return QScreen{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QScreen {
//  type Target = QScreenBASE;
//
//  fn deref(&self) -> &QScreenBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QScreenBASE> for QScreen {
//  fn as_ref(& self) -> & QScreenBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qscreen.h:68
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QScreen {
  pub fn metaObject_0<RetType, T: QScreen_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QScreen_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:98
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QScreen()

/*

*/
pub fn DeleteQScreen(this :*mut QScreen) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QScreenD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qscreen.h:101
// index:0
// Public Visibility=Default Availability=Available
// [8] QString name() const

/*

*/
impl /*struct*/ QScreen {
  pub fn name_0<RetType, T: QScreen_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QScreen_name_0<RetType> {
  fn name_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_name_0<usize> for () {
  fn name_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] QString manufacturer() const

/*

*/
impl /*struct*/ QScreen {
  pub fn manufacturer_0<RetType, T: QScreen_manufacturer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.manufacturer_0(self);
    // return 1;
  }
}
pub trait QScreen_manufacturer_0<RetType> {
  fn manufacturer_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_manufacturer_0<usize> for () {
  fn manufacturer_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen12manufacturerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:104
// index:0
// Public Visibility=Default Availability=Available
// [8] QString model() const

/*

*/
impl /*struct*/ QScreen {
  pub fn model_0<RetType, T: QScreen_model_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.model_0(self);
    // return 1;
  }
}
pub trait QScreen_model_0<RetType> {
  fn model_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_model_0<usize> for () {
  fn model_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen5modelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] QString serialNumber() const

/*

*/
impl /*struct*/ QScreen {
  pub fn serialNumber_0<RetType, T: QScreen_serialNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.serialNumber_0(self);
    // return 1;
  }
}
pub trait QScreen_serialNumber_0<RetType> {
  fn serialNumber_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_serialNumber_0<usize> for () {
  fn serialNumber_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen12serialNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:107
// index:0
// Public Visibility=Default Availability=Available
// [4] int depth() const

/*

*/
impl /*struct*/ QScreen {
  pub fn depth_0<RetType, T: QScreen_depth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.depth_0(self);
    // return 1;
  }
}
pub trait QScreen_depth_0<RetType> {
  fn depth_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_depth_0<i32> for () {
  fn depth_0(self , rsthis: & QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen5depthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:109
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize size() const

/*

*/
impl /*struct*/ QScreen {
  pub fn size_0<RetType, T: QScreen_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QScreen_size_0<RetType> {
  fn size_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_size_0<usize> for () {
  fn size_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:110
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect geometry() const

/*

*/
impl /*struct*/ QScreen {
  pub fn geometry_0<RetType, T: QScreen_geometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.geometry_0(self);
    // return 1;
  }
}
pub trait QScreen_geometry_0<RetType> {
  fn geometry_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_geometry_0<usize> for () {
  fn geometry_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen8geometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:112
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF physicalSize() const

/*

*/
impl /*struct*/ QScreen {
  pub fn physicalSize_0<RetType, T: QScreen_physicalSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.physicalSize_0(self);
    // return 1;
  }
}
pub trait QScreen_physicalSize_0<RetType> {
  fn physicalSize_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_physicalSize_0<usize> for () {
  fn physicalSize_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen12physicalSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:114
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal physicalDotsPerInchX() const

/*

*/
impl /*struct*/ QScreen {
  pub fn physicalDotsPerInchX_0<RetType, T: QScreen_physicalDotsPerInchX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.physicalDotsPerInchX_0(self);
    // return 1;
  }
}
pub trait QScreen_physicalDotsPerInchX_0<RetType> {
  fn physicalDotsPerInchX_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_physicalDotsPerInchX_0<f64> for () {
  fn physicalDotsPerInchX_0(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen20physicalDotsPerInchXEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:115
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal physicalDotsPerInchY() const

/*

*/
impl /*struct*/ QScreen {
  pub fn physicalDotsPerInchY_0<RetType, T: QScreen_physicalDotsPerInchY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.physicalDotsPerInchY_0(self);
    // return 1;
  }
}
pub trait QScreen_physicalDotsPerInchY_0<RetType> {
  fn physicalDotsPerInchY_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_physicalDotsPerInchY_0<f64> for () {
  fn physicalDotsPerInchY_0(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen20physicalDotsPerInchYEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:116
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal physicalDotsPerInch() const

/*

*/
impl /*struct*/ QScreen {
  pub fn physicalDotsPerInch_0<RetType, T: QScreen_physicalDotsPerInch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.physicalDotsPerInch_0(self);
    // return 1;
  }
}
pub trait QScreen_physicalDotsPerInch_0<RetType> {
  fn physicalDotsPerInch_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_physicalDotsPerInch_0<f64> for () {
  fn physicalDotsPerInch_0(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen19physicalDotsPerInchEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:118
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal logicalDotsPerInchX() const

/*

*/
impl /*struct*/ QScreen {
  pub fn logicalDotsPerInchX_0<RetType, T: QScreen_logicalDotsPerInchX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.logicalDotsPerInchX_0(self);
    // return 1;
  }
}
pub trait QScreen_logicalDotsPerInchX_0<RetType> {
  fn logicalDotsPerInchX_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_logicalDotsPerInchX_0<f64> for () {
  fn logicalDotsPerInchX_0(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen19logicalDotsPerInchXEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:119
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal logicalDotsPerInchY() const

/*

*/
impl /*struct*/ QScreen {
  pub fn logicalDotsPerInchY_0<RetType, T: QScreen_logicalDotsPerInchY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.logicalDotsPerInchY_0(self);
    // return 1;
  }
}
pub trait QScreen_logicalDotsPerInchY_0<RetType> {
  fn logicalDotsPerInchY_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_logicalDotsPerInchY_0<f64> for () {
  fn logicalDotsPerInchY_0(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen19logicalDotsPerInchYEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:120
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal logicalDotsPerInch() const

/*

*/
impl /*struct*/ QScreen {
  pub fn logicalDotsPerInch_0<RetType, T: QScreen_logicalDotsPerInch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.logicalDotsPerInch_0(self);
    // return 1;
  }
}
pub trait QScreen_logicalDotsPerInch_0<RetType> {
  fn logicalDotsPerInch_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_logicalDotsPerInch_0<f64> for () {
  fn logicalDotsPerInch_0(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen18logicalDotsPerInchEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:122
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal devicePixelRatio() const

/*

*/
impl /*struct*/ QScreen {
  pub fn devicePixelRatio_0<RetType, T: QScreen_devicePixelRatio_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio_0(self);
    // return 1;
  }
}
pub trait QScreen_devicePixelRatio_0<RetType> {
  fn devicePixelRatio_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_devicePixelRatio_0<f64> for () {
  fn devicePixelRatio_0(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen16devicePixelRatioEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:124
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize availableSize() const

/*

*/
impl /*struct*/ QScreen {
  pub fn availableSize_0<RetType, T: QScreen_availableSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.availableSize_0(self);
    // return 1;
  }
}
pub trait QScreen_availableSize_0<RetType> {
  fn availableSize_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_availableSize_0<usize> for () {
  fn availableSize_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen13availableSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:125
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect availableGeometry() const

/*

*/
impl /*struct*/ QScreen {
  pub fn availableGeometry_0<RetType, T: QScreen_availableGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.availableGeometry_0(self);
    // return 1;
  }
}
pub trait QScreen_availableGeometry_0<RetType> {
  fn availableGeometry_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_availableGeometry_0<usize> for () {
  fn availableGeometry_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen17availableGeometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:129
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize virtualSize() const

/*

*/
impl /*struct*/ QScreen {
  pub fn virtualSize_0<RetType, T: QScreen_virtualSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.virtualSize_0(self);
    // return 1;
  }
}
pub trait QScreen_virtualSize_0<RetType> {
  fn virtualSize_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_virtualSize_0<usize> for () {
  fn virtualSize_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen11virtualSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:130
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect virtualGeometry() const

/*

*/
impl /*struct*/ QScreen {
  pub fn virtualGeometry_0<RetType, T: QScreen_virtualGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.virtualGeometry_0(self);
    // return 1;
  }
}
pub trait QScreen_virtualGeometry_0<RetType> {
  fn virtualGeometry_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_virtualGeometry_0<usize> for () {
  fn virtualGeometry_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen15virtualGeometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:132
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize availableVirtualSize() const

/*

*/
impl /*struct*/ QScreen {
  pub fn availableVirtualSize_0<RetType, T: QScreen_availableVirtualSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.availableVirtualSize_0(self);
    // return 1;
  }
}
pub trait QScreen_availableVirtualSize_0<RetType> {
  fn availableVirtualSize_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_availableVirtualSize_0<usize> for () {
  fn availableVirtualSize_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen20availableVirtualSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:133
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect availableVirtualGeometry() const

/*

*/
impl /*struct*/ QScreen {
  pub fn availableVirtualGeometry_0<RetType, T: QScreen_availableVirtualGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.availableVirtualGeometry_0(self);
    // return 1;
  }
}
pub trait QScreen_availableVirtualGeometry_0<RetType> {
  fn availableVirtualGeometry_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_availableVirtualGeometry_0<usize> for () {
  fn availableVirtualGeometry_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen24availableVirtualGeometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:135
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ScreenOrientation primaryOrientation() const

/*

*/
impl /*struct*/ QScreen {
  pub fn primaryOrientation_0<RetType, T: QScreen_primaryOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.primaryOrientation_0(self);
    // return 1;
  }
}
pub trait QScreen_primaryOrientation_0<RetType> {
  fn primaryOrientation_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_primaryOrientation_0<i32> for () {
  fn primaryOrientation_0(self , rsthis: & QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen18primaryOrientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:136
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ScreenOrientation orientation() const

/*

*/
impl /*struct*/ QScreen {
  pub fn orientation_0<RetType, T: QScreen_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QScreen_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:137
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ScreenOrientation nativeOrientation() const

/*

*/
impl /*struct*/ QScreen {
  pub fn nativeOrientation_0<RetType, T: QScreen_nativeOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nativeOrientation_0(self);
    // return 1;
  }
}
pub trait QScreen_nativeOrientation_0<RetType> {
  fn nativeOrientation_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_nativeOrientation_0<i32> for () {
  fn nativeOrientation_0(self , rsthis: & QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen17nativeOrientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:139
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ScreenOrientations orientationUpdateMask() const

/*
Returns the currently set orientation update mask.

See also setOrientationUpdateMask().
*/
impl /*struct*/ QScreen {
  pub fn orientationUpdateMask_0<RetType, T: QScreen_orientationUpdateMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientationUpdateMask_0(self);
    // return 1;
  }
}
pub trait QScreen_orientationUpdateMask_0<RetType> {
  fn orientationUpdateMask_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_orientationUpdateMask_0<i32> for () {
  fn orientationUpdateMask_0(self , rsthis: & QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen21orientationUpdateMaskEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrientationUpdateMask(Qt::ScreenOrientations)

/*
Sets the orientations that the application is interested in receiving updates for in conjunction with this screen.

For example, to receive orientation() updates and thus have orientationChanged() signals being emitted for LandscapeOrientation and InvertedLandscapeOrientation, call setOrientationUpdateMask() with mask set to Qt::LandscapeOrientation | Qt::InvertedLandscapeOrientation.

The default, 0, means no orientationChanged() signals are fired.

See also orientationUpdateMask().
*/
impl /*struct*/ QScreen {
  pub fn setOrientationUpdateMask_0<RetType, T: QScreen_setOrientationUpdateMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrientationUpdateMask_0(self);
    // return 1;
  }
}
pub trait QScreen_setOrientationUpdateMask_0<RetType> {
  fn setOrientationUpdateMask_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_setOrientationUpdateMask_0<(/*void*/)> for (i32) {
  fn setOrientationUpdateMask_0(self , rsthis: & QScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QScreen24setOrientationUpdateMaskE6QFlagsIN2Qt17ScreenOrientationEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qscreen.h:142
// index:0
// Public Visibility=Default Availability=Available
// [4] int angleBetween(Qt::ScreenOrientation, Qt::ScreenOrientation) const

/*
Convenience function to compute the angle of rotation to get from rotation a to rotation b.

The result will be 0, 90, 180, or 270.

Qt::PrimaryOrientation is interpreted as the screen's primaryOrientation().
*/
impl /*struct*/ QScreen {
  pub fn angleBetween_0<RetType, T: QScreen_angleBetween_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.angleBetween_0(self);
    // return 1;
  }
}
pub trait QScreen_angleBetween_0<RetType> {
  fn angleBetween_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_angleBetween_0<i32> for (i32,i32) {
  fn angleBetween_0(self , rsthis: & QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen12angleBetweenEN2Qt17ScreenOrientationES1_", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:143
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform transformBetween(Qt::ScreenOrientation, Qt::ScreenOrientation, const QRect &) const

/*
Convenience function to compute a transform that maps from the coordinate system defined by orientation a into the coordinate system defined by orientation b and target dimensions target.

Example, a is Qt::Landscape, b is Qt::Portrait, and target is QRect(0, 0, w, h) the resulting transform will be such that the point QPoint(0, 0) is mapped to QPoint(0, w), and QPoint(h, w) is mapped to QPoint(0, h). Thus, the landscape coordinate system QRect(0, 0, h, w) is mapped (with a 90 degree rotation) into the portrait coordinate system QRect(0, 0, w, h).

Qt::PrimaryOrientation is interpreted as the screen's primaryOrientation().
*/
impl /*struct*/ QScreen {
  pub fn transformBetween_0<RetType, T: QScreen_transformBetween_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transformBetween_0(self);
    // return 1;
  }
}
pub trait QScreen_transformBetween_0<RetType> {
  fn transformBetween_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_transformBetween_0<usize> for (i32,i32,usize) {
  fn transformBetween_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen16transformBetweenEN2Qt17ScreenOrientationES1_RK5QRect", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:144
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect mapBetween(Qt::ScreenOrientation, Qt::ScreenOrientation, const QRect &) const

/*
Maps the rect between two screen orientations.

This will flip the x and y dimensions of the rectangle rect if the orientation a is Qt::PortraitOrientation or Qt::InvertedPortraitOrientation and orientation b is Qt::LandscapeOrientation or Qt::InvertedLandscapeOrientation, or vice versa.

Qt::PrimaryOrientation is interpreted as the screen's primaryOrientation().
*/
impl /*struct*/ QScreen {
  pub fn mapBetween_0<RetType, T: QScreen_mapBetween_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapBetween_0(self);
    // return 1;
  }
}
pub trait QScreen_mapBetween_0<RetType> {
  fn mapBetween_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_mapBetween_0<usize> for (i32,i32,usize) {
  fn mapBetween_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen10mapBetweenEN2Qt17ScreenOrientationES1_RK5QRect", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:146
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isPortrait(Qt::ScreenOrientation) const

/*
Convenience function that returns true if o is either portrait or inverted portrait; otherwise returns false.

Qt::PrimaryOrientation is interpreted as the screen's primaryOrientation().
*/
impl /*struct*/ QScreen {
  pub fn isPortrait_0<RetType, T: QScreen_isPortrait_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPortrait_0(self);
    // return 1;
  }
}
pub trait QScreen_isPortrait_0<RetType> {
  fn isPortrait_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_isPortrait_0<bool> for (i32) {
  fn isPortrait_0(self , rsthis: & QScreen) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen10isPortraitEN2Qt17ScreenOrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:147
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isLandscape(Qt::ScreenOrientation) const

/*
Convenience function that returns true if o is either landscape or inverted landscape; otherwise returns false.

Qt::PrimaryOrientation is interpreted as the screen's primaryOrientation().
*/
impl /*struct*/ QScreen {
  pub fn isLandscape_0<RetType, T: QScreen_isLandscape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLandscape_0(self);
    // return 1;
  }
}
pub trait QScreen_isLandscape_0<RetType> {
  fn isLandscape_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_isLandscape_0<bool> for (i32) {
  fn isLandscape_0(self , rsthis: & QScreen) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen11isLandscapeEN2Qt17ScreenOrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:149
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap grabWindow(WId, int, int, int, int)

/*
Creates and returns a pixmap constructed by grabbing the contents of the given window restricted by QRect(x, y, width, height).

The arguments (x, y) specify the offset in the window, whereas (width, height) specify the area to be copied. If width is negative, the function copies everything to the right border of the window. If height is negative, the function copies everything to the bottom of the window.

The window system identifier (WId) can be retrieved using the QWidget::winId() function. The rationale for using a window identifier and not a QWidget, is to enable grabbing of windows that are not part of the application, window system frames, and so on.

Warning: Grabbing windows that are not part of the application is not supported on systems such as iOS, where sandboxing/security prevents reading pixels of windows not owned by the application.

The grabWindow() function grabs pixels from the screen, not from the window, i.e. if there is another window partially or entirely over the one you grab, you get pixels from the overlying window, too. The mouse cursor is generally not grabbed.

Note on X11 that if the given window doesn't have the same depth as the root window, and another window partially or entirely obscures the one you grab, you will not get pixels from the overlying window. The contents of the obscured areas in the pixmap will be undefined and uninitialized.

On Windows Vista and above grabbing a layered window, which is created by setting the Qt::WA_TranslucentBackground attribute, will not work. Instead grabbing the desktop widget should work.

Warning: In general, grabbing an area outside the screen is not safe. This depends on the underlying window system.
*/
impl /*struct*/ QScreen {
  pub fn grabWindow_0<RetType, T: QScreen_grabWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.grabWindow_0(self);
    // return 1;
  }
}
pub trait QScreen_grabWindow_0<RetType> {
  fn grabWindow_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_grabWindow_0<usize> for (u64,i32,i32,i32,i32) {
  fn grabWindow_0(self , rsthis: & QScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QScreen10grabWindowEyiiii", 5,qtrt::FFITY_UINT64,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:151
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal refreshRate() const

/*

*/
impl /*struct*/ QScreen {
  pub fn refreshRate_0<RetType, T: QScreen_refreshRate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.refreshRate_0(self);
    // return 1;
  }
}
pub trait QScreen_refreshRate_0<RetType> {
  fn refreshRate_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_refreshRate_0<f64> for () {
  fn refreshRate_0(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QScreen11refreshRateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qscreen.h:154
// index:0
// Public Visibility=Default Availability=Available
// [-2] void geometryChanged(const QRect &)

/*

*/
impl /*struct*/ QScreen {
  pub fn geometryChanged_0<RetType, T: QScreen_geometryChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.geometryChanged_0(self);
    // return 1;
  }
}
pub trait QScreen_geometryChanged_0<RetType> {
  fn geometryChanged_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_geometryChanged_0<(/*void*/)> for (usize) {
  fn geometryChanged_0(self , rsthis: & QScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QScreen15geometryChangedERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qscreen.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void availableGeometryChanged(const QRect &)

/*

*/
impl /*struct*/ QScreen {
  pub fn availableGeometryChanged_0<RetType, T: QScreen_availableGeometryChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.availableGeometryChanged_0(self);
    // return 1;
  }
}
pub trait QScreen_availableGeometryChanged_0<RetType> {
  fn availableGeometryChanged_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_availableGeometryChanged_0<(/*void*/)> for (usize) {
  fn availableGeometryChanged_0(self , rsthis: & QScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QScreen24availableGeometryChangedERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qscreen.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void physicalSizeChanged(const QSizeF &)

/*

*/
impl /*struct*/ QScreen {
  pub fn physicalSizeChanged_0<RetType, T: QScreen_physicalSizeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.physicalSizeChanged_0(self);
    // return 1;
  }
}
pub trait QScreen_physicalSizeChanged_0<RetType> {
  fn physicalSizeChanged_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_physicalSizeChanged_0<(/*void*/)> for (usize) {
  fn physicalSizeChanged_0(self , rsthis: & QScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QScreen19physicalSizeChangedERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qscreen.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void physicalDotsPerInchChanged(qreal)

/*

*/
impl /*struct*/ QScreen {
  pub fn physicalDotsPerInchChanged_0<RetType, T: QScreen_physicalDotsPerInchChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.physicalDotsPerInchChanged_0(self);
    // return 1;
  }
}
pub trait QScreen_physicalDotsPerInchChanged_0<RetType> {
  fn physicalDotsPerInchChanged_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_physicalDotsPerInchChanged_0<(/*void*/)> for (f64) {
  fn physicalDotsPerInchChanged_0(self , rsthis: & QScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN7QScreen26physicalDotsPerInchChangedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qscreen.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void logicalDotsPerInchChanged(qreal)

/*

*/
impl /*struct*/ QScreen {
  pub fn logicalDotsPerInchChanged_0<RetType, T: QScreen_logicalDotsPerInchChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.logicalDotsPerInchChanged_0(self);
    // return 1;
  }
}
pub trait QScreen_logicalDotsPerInchChanged_0<RetType> {
  fn logicalDotsPerInchChanged_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_logicalDotsPerInchChanged_0<(/*void*/)> for (f64) {
  fn logicalDotsPerInchChanged_0(self , rsthis: & QScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN7QScreen25logicalDotsPerInchChangedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qscreen.h:159
// index:0
// Public Visibility=Default Availability=Available
// [-2] void virtualGeometryChanged(const QRect &)

/*

*/
impl /*struct*/ QScreen {
  pub fn virtualGeometryChanged_0<RetType, T: QScreen_virtualGeometryChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.virtualGeometryChanged_0(self);
    // return 1;
  }
}
pub trait QScreen_virtualGeometryChanged_0<RetType> {
  fn virtualGeometryChanged_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_virtualGeometryChanged_0<(/*void*/)> for (usize) {
  fn virtualGeometryChanged_0(self , rsthis: & QScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QScreen22virtualGeometryChangedERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qscreen.h:160
// index:0
// Public Visibility=Default Availability=Available
// [-2] void primaryOrientationChanged(Qt::ScreenOrientation)

/*
This signal is emitted when the primary orientation of the screen changes with orientation as an argument.

Note: Notifier signal for property primaryOrientation. 

See also primaryOrientation().
*/
impl /*struct*/ QScreen {
  pub fn primaryOrientationChanged_0<RetType, T: QScreen_primaryOrientationChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.primaryOrientationChanged_0(self);
    // return 1;
  }
}
pub trait QScreen_primaryOrientationChanged_0<RetType> {
  fn primaryOrientationChanged_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_primaryOrientationChanged_0<(/*void*/)> for (i32) {
  fn primaryOrientationChanged_0(self , rsthis: & QScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QScreen25primaryOrientationChangedEN2Qt17ScreenOrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qscreen.h:161
// index:0
// Public Visibility=Default Availability=Available
// [-2] void orientationChanged(Qt::ScreenOrientation)

/*
This signal is emitted when the orientation of the screen changes with orientation as an argument.

Note: Notifier signal for property orientation. 

See also orientation().
*/
impl /*struct*/ QScreen {
  pub fn orientationChanged_0<RetType, T: QScreen_orientationChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientationChanged_0(self);
    // return 1;
  }
}
pub trait QScreen_orientationChanged_0<RetType> {
  fn orientationChanged_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_orientationChanged_0<(/*void*/)> for (i32) {
  fn orientationChanged_0(self , rsthis: & QScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QScreen18orientationChangedEN2Qt17ScreenOrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qscreen.h:162
// index:0
// Public Visibility=Default Availability=Available
// [-2] void refreshRateChanged(qreal)

/*

*/
impl /*struct*/ QScreen {
  pub fn refreshRateChanged_0<RetType, T: QScreen_refreshRateChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.refreshRateChanged_0(self);
    // return 1;
  }
}
pub trait QScreen_refreshRateChanged_0<RetType> {
  fn refreshRateChanged_0(self , rsthis: & QScreen) -> RetType;
}
impl<'a> /*trait*/ QScreen_refreshRateChanged_0<(/*void*/)> for (f64) {
  fn refreshRateChanged_0(self , rsthis: & QScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN7QScreen18refreshRateChangedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
