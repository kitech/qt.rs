

// mod ::gui::QPaintDevice
// package qtgui
// /usr/include/qt/QtGui/qpaintdevice.h
// #include <qpaintdevice.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 64
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
// func (this *QPaintDevice) InheritMetric(f func(metric int) int) {
//  qtrt.SetAllInheritCallback(this, "metric", f)
// }

// void initPainter(QPainter *)
// func (this *QPaintDevice) InheritInitPainter(f func(painter *QPainter/*777 QPainter **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initPainter", f)
// }

// QPaintDevice * redirected(QPoint *)
// func (this *QPaintDevice) InheritRedirected(f func(offset *qtcore.QPoint/*777 QPoint **/) unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "redirected", f)
// }

// QPainter * sharedPainter()
// func (this *QPaintDevice) InheritSharedPainter(f func() unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "sharedPainter", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QPaintDevice)=24
pub struct QPaintDevice {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPaintDevice_ITF interface {
//    QPaintDevice_PTR() *QPaintDevice
//}
//func (ptr *QPaintDevice) QPaintDevice_PTR() *QPaintDevice { return ptr }

impl /*struct*/ QPaintDevice {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPaintDevice {
    return QPaintDevice{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPaintDevice {
//  type Target = QPaintDeviceBASE;
//
//  fn deref(&self) -> &QPaintDeviceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPaintDeviceBASE> for QPaintDevice {
//  fn as_ref(& self) -> & QPaintDeviceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpaintdevice.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPaintDevice()

/*

*/
pub fn DeleteQPaintDevice(this :*mut QPaintDevice) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QPaintDeviceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpaintdevice.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int devType() const

/*

*/
impl /*struct*/ QPaintDevice {
  pub fn devType_0<RetType, T: QPaintDevice_devType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.devType_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_devType_0<RetType> {
  fn devType_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_devType_0<i32> for () {
  fn devType_0(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice7devTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:75
// index:0
// Public Visibility=Default Availability=Available
// [1] bool paintingActive() const

/*
Returns true if the device is currently being painted on, i.e. someone has called QPainter::begin() but not yet called QPainter::end() for this device; otherwise returns false.

See also QPainter::isActive().
*/
impl /*struct*/ QPaintDevice {
  pub fn paintingActive_0<RetType, T: QPaintDevice_paintingActive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintingActive_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_paintingActive_0<RetType> {
  fn paintingActive_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_paintingActive_0<bool> for () {
  fn paintingActive_0(self , rsthis: & QPaintDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice14paintingActiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:76
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QPaintEngine * paintEngine() const

/*
Returns a pointer to the paint engine used for drawing on the device.
*/
impl /*struct*/ QPaintDevice {
  pub fn paintEngine_0<RetType, T: QPaintDevice_paintEngine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEngine_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_paintEngine_0<RetType> {
  fn paintEngine_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_paintEngine_0<usize> for () {
  fn paintEngine_0(self , rsthis: & QPaintDevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice11paintEngineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int width() const

/*
Returns the width of the paint device in default coordinate system units (e.g. pixels for QPixmap and QWidget).

See also widthMM().
*/
impl /*struct*/ QPaintDevice {
  pub fn width_0<RetType, T: QPaintDevice_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_width_0<RetType> {
  fn width_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_width_0<i32> for () {
  fn width_0(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:79
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int height() const

/*
Returns the height of the paint device in default coordinate system units (e.g. pixels for QPixmap and QWidget).

See also heightMM().
*/
impl /*struct*/ QPaintDevice {
  pub fn height_0<RetType, T: QPaintDevice_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_height_0<RetType> {
  fn height_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_height_0<i32> for () {
  fn height_0(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:80
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int widthMM() const

/*
Returns the width of the paint device in millimeters. Due to platform limitations it may not be possible to use this function to determine the actual physical size of a widget on the screen.

See also width().
*/
impl /*struct*/ QPaintDevice {
  pub fn widthMM_0<RetType, T: QPaintDevice_widthMM_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widthMM_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_widthMM_0<RetType> {
  fn widthMM_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_widthMM_0<i32> for () {
  fn widthMM_0(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice7widthMMEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int heightMM() const

/*
Returns the height of the paint device in millimeters. Due to platform limitations it may not be possible to use this function to determine the actual physical size of a widget on the screen.

See also height().
*/
impl /*struct*/ QPaintDevice {
  pub fn heightMM_0<RetType, T: QPaintDevice_heightMM_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightMM_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_heightMM_0<RetType> {
  fn heightMM_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_heightMM_0<i32> for () {
  fn heightMM_0(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice8heightMMEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:82
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int logicalDpiX() const

/*
Returns the horizontal resolution of the device in dots per inch, which is used when computing font sizes. For X11, this is usually the same as could be computed from widthMM().

Note that if the logicalDpiX() doesn't equal the physicalDpiX(), the corresponding QPaintEngine must handle the resolution mapping.

See also logicalDpiY() and physicalDpiX().
*/
impl /*struct*/ QPaintDevice {
  pub fn logicalDpiX_0<RetType, T: QPaintDevice_logicalDpiX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.logicalDpiX_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_logicalDpiX_0<RetType> {
  fn logicalDpiX_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_logicalDpiX_0<i32> for () {
  fn logicalDpiX_0(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice11logicalDpiXEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:83
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int logicalDpiY() const

/*
Returns the vertical resolution of the device in dots per inch, which is used when computing font sizes. For X11, this is usually the same as could be computed from heightMM().

Note that if the logicalDpiY() doesn't equal the physicalDpiY(), the corresponding QPaintEngine must handle the resolution mapping.

See also logicalDpiX() and physicalDpiY().
*/
impl /*struct*/ QPaintDevice {
  pub fn logicalDpiY_0<RetType, T: QPaintDevice_logicalDpiY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.logicalDpiY_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_logicalDpiY_0<RetType> {
  fn logicalDpiY_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_logicalDpiY_0<i32> for () {
  fn logicalDpiY_0(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice11logicalDpiYEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:84
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int physicalDpiX() const

/*
Returns the horizontal resolution of the device in dots per inch. For example, when printing, this resolution refers to the physical printer's resolution. The logical DPI on the other hand, refers to the resolution used by the actual paint engine.

Note that if the physicalDpiX() doesn't equal the logicalDpiX(), the corresponding QPaintEngine must handle the resolution mapping.

See also physicalDpiY() and logicalDpiX().
*/
impl /*struct*/ QPaintDevice {
  pub fn physicalDpiX_0<RetType, T: QPaintDevice_physicalDpiX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.physicalDpiX_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_physicalDpiX_0<RetType> {
  fn physicalDpiX_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_physicalDpiX_0<i32> for () {
  fn physicalDpiX_0(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice12physicalDpiXEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:85
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int physicalDpiY() const

/*
Returns the horizontal resolution of the device in dots per inch. For example, when printing, this resolution refers to the physical printer's resolution. The logical DPI on the other hand, refers to the resolution used by the actual paint engine.

Note that if the physicalDpiY() doesn't equal the logicalDpiY(), the corresponding QPaintEngine must handle the resolution mapping.

See also physicalDpiX() and logicalDpiY().
*/
impl /*struct*/ QPaintDevice {
  pub fn physicalDpiY_0<RetType, T: QPaintDevice_physicalDpiY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.physicalDpiY_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_physicalDpiY_0<RetType> {
  fn physicalDpiY_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_physicalDpiY_0<i32> for () {
  fn physicalDpiY_0(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice12physicalDpiYEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:86
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int devicePixelRatio() const

/*
Returns the device pixel ratio for device.

Common values are 1 for normal-dpi displays and 2 for high-dpi "retina" displays.
*/
impl /*struct*/ QPaintDevice {
  pub fn devicePixelRatio_0<RetType, T: QPaintDevice_devicePixelRatio_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_devicePixelRatio_0<RetType> {
  fn devicePixelRatio_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_devicePixelRatio_0<i32> for () {
  fn devicePixelRatio_0(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice16devicePixelRatioEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:87
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal devicePixelRatioF() const

/*
Returns the device pixel ratio for the device as a floating point number.

This function was introduced in  Qt 5.6.
*/
impl /*struct*/ QPaintDevice {
  pub fn devicePixelRatioF_0<RetType, T: QPaintDevice_devicePixelRatioF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatioF_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_devicePixelRatioF_0<RetType> {
  fn devicePixelRatioF_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_devicePixelRatioF_0<f64> for () {
  fn devicePixelRatioF_0(self , rsthis: & QPaintDevice) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice17devicePixelRatioFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:88
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int colorCount() const

/*
Returns the number of different colors available for the paint device. If the number of colors available is too great to be represented by the int data type, then INT_MAX will be returned instead.
*/
impl /*struct*/ QPaintDevice {
  pub fn colorCount_0<RetType, T: QPaintDevice_colorCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.colorCount_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_colorCount_0<RetType> {
  fn colorCount_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_colorCount_0<i32> for () {
  fn colorCount_0(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice10colorCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:89
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int depth() const

/*
Returns the bit depth (number of bit planes) of the paint device.
*/
impl /*struct*/ QPaintDevice {
  pub fn depth_0<RetType, T: QPaintDevice_depth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.depth_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_depth_0<RetType> {
  fn depth_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_depth_0<i32> for () {
  fn depth_0(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice5depthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:91
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] qreal devicePixelRatioFScale()

/*

*/
impl /*struct*/ QPaintDevice {
  pub fn devicePixelRatioFScale_0<RetType, T: QPaintDevice_devicePixelRatioFScale_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.devicePixelRatioFScale_0();
    // return 1;
  }
}
pub trait QPaintDevice_devicePixelRatioFScale_0<RetType> {
  fn devicePixelRatioFScale_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_devicePixelRatioFScale_0<f64> for () {
  fn devicePixelRatioFScale_0(self ) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPaintDevice22devicePixelRatioFScaleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:93
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void QPaintDevice()

/*
Constructs a paint device. This constructor can be invoked only from subclasses of QPaintDevice.
*/
// QPaintDevice() ctx.fn_proto_cpp
impl /*struct*/ QPaintDevice {
  pub fn QPaintDevice_0<T: QPaintDevice_QPaintDevice_0>(value: T) -> QPaintDevice {
    let rsthis = value.QPaintDevice_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintDevice_QPaintDevice_0 {
  fn QPaintDevice_0(self) -> QPaintDevice;
}
// QPaintDevice() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPaintDevice_QPaintDevice_0 for () {
  fn QPaintDevice_0(self) -> QPaintDevice {
    // unsafe{_ZN12QPaintDeviceC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QPaintDeviceC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPaintDevice{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:101
// index:1
// Private inline Visibility=Default Availability=NotAvailable
// [-2] void QPaintDevice(const QPaintDevice &)

/*
Constructs a paint device. This constructor can be invoked only from subclasses of QPaintDevice.
*/
// QPaintDevice(const QPaintDevice &) ctx.fn_proto_cpp
impl /*struct*/ QPaintDevice {
  pub fn QPaintDevice_1<T: QPaintDevice_QPaintDevice_1>(value: T) -> QPaintDevice {
    let rsthis = value.QPaintDevice_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintDevice_QPaintDevice_1 {
  fn QPaintDevice_1(self) -> QPaintDevice;
}
// QPaintDevice(const QPaintDevice &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPaintDevice_QPaintDevice_1 for (usize) {
  fn QPaintDevice_1(self) -> QPaintDevice {
    // unsafe{_ZN12QPaintDeviceC2ERKS_()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QPaintDeviceC2ERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPaintDevice{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:94
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int metric(QPaintDevice::PaintDeviceMetric) const

/*
Returns the metric information for the given paint device metric.

See also PaintDeviceMetric.
*/
impl /*struct*/ QPaintDevice {
  pub fn metric_0<RetType, T: QPaintDevice_metric_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metric_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_metric_0<RetType> {
  fn metric_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_metric_0<i32> for (i32) {
  fn metric_0(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice6metricENS_17PaintDeviceMetricE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:95
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void initPainter(QPainter *) const

/*

*/
impl /*struct*/ QPaintDevice {
  pub fn initPainter_0<RetType, T: QPaintDevice_initPainter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initPainter_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_initPainter_0<RetType> {
  fn initPainter_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_initPainter_0<(/*void*/)> for (usize) {
  fn initPainter_0(self , rsthis: & QPaintDevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK12QPaintDevice11initPainterEP8QPainter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:96
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QPaintDevice * redirected(QPoint *) const

/*

*/
impl /*struct*/ QPaintDevice {
  pub fn redirected_0<RetType, T: QPaintDevice_redirected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redirected_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_redirected_0<RetType> {
  fn redirected_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_redirected_0<usize> for (usize) {
  fn redirected_0(self , rsthis: & QPaintDevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice10redirectedEP6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintdevice.h:97
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QPainter * sharedPainter() const

/*

*/
impl /*struct*/ QPaintDevice {
  pub fn sharedPainter_0<RetType, T: QPaintDevice_sharedPainter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sharedPainter_0(self);
    // return 1;
  }
}
pub trait QPaintDevice_sharedPainter_0<RetType> {
  fn sharedPainter_0(self , rsthis: & QPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPaintDevice_sharedPainter_0<usize> for () {
  fn sharedPainter_0(self , rsthis: & QPaintDevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintDevice13sharedPainterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
Describes the various metrics of a paint device.



See also metric() and devicePixelRatioF().

*/
pub type QPaintDevice__PaintDeviceMetric = i32;
// The width of the paint device in default coordinate system units (e.g. pixels for QPixmap and QWidget). See also width().
pub const QPaintDevice__PdmWidth :QPaintDevice__PaintDeviceMetric = 1;
// The height of the paint device in default coordinate system units (e.g. pixels for QPixmap and QWidget). See also height().
pub const QPaintDevice__PdmHeight :QPaintDevice__PaintDeviceMetric = 2;
// The width of the paint device in millimeters. See also widthMM().
pub const QPaintDevice__PdmWidthMM :QPaintDevice__PaintDeviceMetric = 3;
// The height of the paint device in millimeters. See also heightMM().
pub const QPaintDevice__PdmHeightMM :QPaintDevice__PaintDeviceMetric = 4;
// The number of different colors available for the paint device. See also colorCount().
pub const QPaintDevice__PdmNumColors :QPaintDevice__PaintDeviceMetric = 5;
// The bit depth (number of bit planes) of the paint device. See also depth().
pub const QPaintDevice__PdmDepth :QPaintDevice__PaintDeviceMetric = 6;
// The horizontal resolution of the device in dots per inch. See also logicalDpiX().
pub const QPaintDevice__PdmDpiX :QPaintDevice__PaintDeviceMetric = 7;
// The vertical resolution of the device in dots per inch. See also logicalDpiY().
pub const QPaintDevice__PdmDpiY :QPaintDevice__PaintDeviceMetric = 8;
// The horizontal resolution of the device in dots per inch. See also physicalDpiX().
pub const QPaintDevice__PdmPhysicalDpiX :QPaintDevice__PaintDeviceMetric = 9;
// 
pub const QPaintDevice__PdmPhysicalDpiY :QPaintDevice__PaintDeviceMetric = 10;
// 
pub const QPaintDevice__PdmDevicePixelRatio :QPaintDevice__PaintDeviceMetric = 11;
// 
pub const QPaintDevice__PdmDevicePixelRatioScaled :QPaintDevice__PaintDeviceMetric = 12;
pub fn QPaintDevice_PaintDeviceMetricItemName(val: i32) ->String {
  match val {
     QPaintDevice__PdmWidth => // 1
     {return String::from("PdmWidth");}
     QPaintDevice__PdmHeight => // 2
     {return String::from("PdmHeight");}
     QPaintDevice__PdmWidthMM => // 3
     {return String::from("PdmWidthMM");}
     QPaintDevice__PdmHeightMM => // 4
     {return String::from("PdmHeightMM");}
     QPaintDevice__PdmNumColors => // 5
     {return String::from("PdmNumColors");}
     QPaintDevice__PdmDepth => // 6
     {return String::from("PdmDepth");}
     QPaintDevice__PdmDpiX => // 7
     {return String::from("PdmDpiX");}
     QPaintDevice__PdmDpiY => // 8
     {return String::from("PdmDpiY");}
     QPaintDevice__PdmPhysicalDpiX => // 9
     {return String::from("PdmPhysicalDpiX");}
     QPaintDevice__PdmPhysicalDpiY => // 10
     {return String::from("PdmPhysicalDpiY");}
     QPaintDevice__PdmDevicePixelRatio => // 11
     {return String::from("PdmDevicePixelRatio");}
     QPaintDevice__PdmDevicePixelRatioScaled => // 12
     {return String::from("PdmDevicePixelRatioScaled");}
  _ => {return format!("{}", val);}
}
}
pub fn QPaintDevice_PaintDeviceMetricItemName_s(val: i32) ->String {
  //var nilthis *QPaintDevice
  //return nilthis.PaintDeviceMetricItemName(val);
  return QPaintDevice_PaintDeviceMetricItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
