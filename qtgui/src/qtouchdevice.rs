

// mod ::gui::QTouchDevice
// package qtgui
// /usr/include/qt/QtGui/qtouchdevice.h
// #include <qtouchdevice.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 31
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
#[derive(Default)] // class sizeof(QTouchDevice)=8
pub struct QTouchDevice {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTouchDevice_ITF interface {
//    QTouchDevice_PTR() *QTouchDevice
//}
//func (ptr *QTouchDevice) QTouchDevice_PTR() *QTouchDevice { return ptr }

impl /*struct*/ QTouchDevice {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTouchDevice {
    return QTouchDevice{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTouchDevice {
//  type Target = QTouchDeviceBASE;
//
//  fn deref(&self) -> &QTouchDeviceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTouchDeviceBASE> for QTouchDevice {
//  fn as_ref(& self) -> & QTouchDeviceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtouchdevice.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTouchDevice()

/*
Creates a new touch device instance. By default the name is empty, the only capability is Position and type is TouchScreen.
*/
// QTouchDevice() ctx.fn_proto_cpp
impl /*struct*/ QTouchDevice {
  pub fn QTouchDevice_0<T: QTouchDevice_QTouchDevice_0>(value: T) -> QTouchDevice {
    let rsthis = value.QTouchDevice_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTouchDevice_QTouchDevice_0 {
  fn QTouchDevice_0(self) -> QTouchDevice;
}
// QTouchDevice() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTouchDevice_QTouchDevice_0 for () {
  fn QTouchDevice_0(self) -> QTouchDevice {
    // unsafe{_ZN12QTouchDeviceC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QTouchDeviceC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTouchDevice{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtouchdevice.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTouchDevice()

/*

*/
pub fn DeleteQTouchDevice(this :*mut QTouchDevice) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QTouchDeviceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtouchdevice.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QString name() const

/*
Returns the touch device name.

This string may often be empty. It is however useful for systems that have more than one touch input device because there it can be used to differentiate between the devices (i.e. to tell from which device a QTouchEvent originates from).

See also setName().
*/
impl /*struct*/ QTouchDevice {
  pub fn name_0<RetType, T: QTouchDevice_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QTouchDevice_name_0<RetType> {
  fn name_0(self , rsthis: & QTouchDevice) -> RetType;
}
impl<'a> /*trait*/ QTouchDevice_name_0<usize> for () {
  fn name_0(self , rsthis: & QTouchDevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTouchDevice4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtouchdevice.h:79
// index:0
// Public Visibility=Default Availability=Available
// [4] QTouchDevice::DeviceType type() const

/*
Returns the touch device type.

See also setType().
*/
impl /*struct*/ QTouchDevice {
  pub fn type__0<RetType, T: QTouchDevice_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QTouchDevice_type__0<RetType> {
  fn type__0(self , rsthis: & QTouchDevice) -> RetType;
}
impl<'a> /*trait*/ QTouchDevice_type__0<i32> for () {
  fn type__0(self , rsthis: & QTouchDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTouchDevice4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtouchdevice.h:80
// index:0
// Public Visibility=Default Availability=Available
// [4] QTouchDevice::Capabilities capabilities() const

/*
Returns the touch device capabilities.

See also setCapabilities().
*/
impl /*struct*/ QTouchDevice {
  pub fn capabilities_0<RetType, T: QTouchDevice_capabilities_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capabilities_0(self);
    // return 1;
  }
}
pub trait QTouchDevice_capabilities_0<RetType> {
  fn capabilities_0(self , rsthis: & QTouchDevice) -> RetType;
}
impl<'a> /*trait*/ QTouchDevice_capabilities_0<i32> for () {
  fn capabilities_0(self , rsthis: & QTouchDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTouchDevice12capabilitiesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtouchdevice.h:81
// index:0
// Public Visibility=Default Availability=Available
// [4] int maximumTouchPoints() const

/*
Returns the maximum number of simultaneous touch points (fingers) that can be detected.

This function was introduced in  Qt 5.2.

See also setMaximumTouchPoints().
*/
impl /*struct*/ QTouchDevice {
  pub fn maximumTouchPoints_0<RetType, T: QTouchDevice_maximumTouchPoints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumTouchPoints_0(self);
    // return 1;
  }
}
pub trait QTouchDevice_maximumTouchPoints_0<RetType> {
  fn maximumTouchPoints_0(self , rsthis: & QTouchDevice) -> RetType;
}
impl<'a> /*trait*/ QTouchDevice_maximumTouchPoints_0<i32> for () {
  fn maximumTouchPoints_0(self , rsthis: & QTouchDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTouchDevice18maximumTouchPointsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtouchdevice.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setName(const QString &)

/*
Sets the name (a unique identifier) for the device. In most systems it is enough to leave this unset and keep the default empty name. This identifier becomes important when having multiple touch devices and a need to differentiate between them.

See also name().
*/
impl /*struct*/ QTouchDevice {
  pub fn setName_0<RetType, T: QTouchDevice_setName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setName_0(self);
    // return 1;
  }
}
pub trait QTouchDevice_setName_0<RetType> {
  fn setName_0(self , rsthis: & QTouchDevice) -> RetType;
}
impl<'a> /*trait*/ QTouchDevice_setName_0<(/*void*/)> for (usize) {
  fn setName_0(self , rsthis: & QTouchDevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QTouchDevice7setNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtouchdevice.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setType(QTouchDevice::DeviceType)

/*
Sets the device type devType.

See also type().
*/
impl /*struct*/ QTouchDevice {
  pub fn setType_0<RetType, T: QTouchDevice_setType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setType_0(self);
    // return 1;
  }
}
pub trait QTouchDevice_setType_0<RetType> {
  fn setType_0(self , rsthis: & QTouchDevice) -> RetType;
}
impl<'a> /*trait*/ QTouchDevice_setType_0<(/*void*/)> for (i32) {
  fn setType_0(self , rsthis: & QTouchDevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTouchDevice7setTypeENS_10DeviceTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtouchdevice.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCapabilities(QTouchDevice::Capabilities)

/*
Sets the capabilities caps supported by the device and its driver.

See also capabilities().
*/
impl /*struct*/ QTouchDevice {
  pub fn setCapabilities_0<RetType, T: QTouchDevice_setCapabilities_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCapabilities_0(self);
    // return 1;
  }
}
pub trait QTouchDevice_setCapabilities_0<RetType> {
  fn setCapabilities_0(self , rsthis: & QTouchDevice) -> RetType;
}
impl<'a> /*trait*/ QTouchDevice_setCapabilities_0<(/*void*/)> for (i32) {
  fn setCapabilities_0(self , rsthis: & QTouchDevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTouchDevice15setCapabilitiesE6QFlagsINS_14CapabilityFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtouchdevice.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumTouchPoints(int)

/*
Sets the maximum number of simultaneous touchpoints max supported by the device and its driver.

See also maximumTouchPoints().
*/
impl /*struct*/ QTouchDevice {
  pub fn setMaximumTouchPoints_0<RetType, T: QTouchDevice_setMaximumTouchPoints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumTouchPoints_0(self);
    // return 1;
  }
}
pub trait QTouchDevice_setMaximumTouchPoints_0<RetType> {
  fn setMaximumTouchPoints_0(self , rsthis: & QTouchDevice) -> RetType;
}
impl<'a> /*trait*/ QTouchDevice_setMaximumTouchPoints_0<(/*void*/)> for (i32) {
  fn setMaximumTouchPoints_0(self , rsthis: & QTouchDevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTouchDevice21setMaximumTouchPointsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum represents the type of device that generated a QTouchEvent.


*/
pub type QTouchDevice__DeviceType = i32;
// In this type of device, the touch surface and display are integrated. This means the surface and display typically have the same size, such that there is a direct relationship between the touch points' physical positions and the coordinate reported by QTouchEvent::TouchPoint. As a result, Qt allows the user to interact directly with multiple QWidgets and QGraphicsItems at the same time.
pub const QTouchDevice__TouchScreen :QTouchDevice__DeviceType = 0;
// In this type of device, the touch surface is separate from the display. There is not a direct relationship between the physical touch location and the on-screen coordinates. Instead, they are calculated relative to the current mouse position, and the user must use the touch-pad to move this reference point. Unlike touch-screens, Qt allows users to only interact with a single QWidget or QGraphicsItem at a time.
pub const QTouchDevice__TouchPad :QTouchDevice__DeviceType = 1;
pub fn QTouchDevice_DeviceTypeItemName(val: i32) ->String {
  match val {
     QTouchDevice__TouchScreen => // 0
     {return String::from("TouchScreen");}
     QTouchDevice__TouchPad => // 1
     {return String::from("TouchPad");}
  _ => {return format!("{}", val);}
}
}
pub fn QTouchDevice_DeviceTypeItemName_s(val: i32) ->String {
  //var nilthis *QTouchDevice
  //return nilthis.DeviceTypeItemName(val);
  return QTouchDevice_DeviceTypeItemName(val);
}


/*


*/
pub type QTouchDevice__CapabilityFlag = i32;
// 
pub const QTouchDevice__Position :QTouchDevice__CapabilityFlag = 1;
// 
pub const QTouchDevice__Area :QTouchDevice__CapabilityFlag = 2;
// 
pub const QTouchDevice__Pressure :QTouchDevice__CapabilityFlag = 4;
// 
pub const QTouchDevice__Velocity :QTouchDevice__CapabilityFlag = 8;
// 
pub const QTouchDevice__RawPositions :QTouchDevice__CapabilityFlag = 16;
// 
pub const QTouchDevice__NormalizedPosition :QTouchDevice__CapabilityFlag = 32;
// 
pub const QTouchDevice__MouseEmulation :QTouchDevice__CapabilityFlag = 64;
pub fn QTouchDevice_CapabilityFlagItemName(val: i32) ->String {
  match val {
     QTouchDevice__Position => // 1
     {return String::from("Position");}
     QTouchDevice__Area => // 2
     {return String::from("Area");}
     QTouchDevice__Pressure => // 4
     {return String::from("Pressure");}
     QTouchDevice__Velocity => // 8
     {return String::from("Velocity");}
     QTouchDevice__RawPositions => // 16
     {return String::from("RawPositions");}
     QTouchDevice__NormalizedPosition => // 32
     {return String::from("NormalizedPosition");}
     QTouchDevice__MouseEmulation => // 64
     {return String::from("MouseEmulation");}
  _ => {return format!("{}", val);}
}
}
pub fn QTouchDevice_CapabilityFlagItemName_s(val: i32) ->String {
  //var nilthis *QTouchDevice
  //return nilthis.CapabilityFlagItemName(val);
  return QTouchDevice_CapabilityFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
