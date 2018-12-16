

// mod ::widgets::QScrollerProperties
// package qtwidgets
// /usr/include/qt/QtWidgets/qscrollerproperties.h
// #include <qscrollerproperties.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 15
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QScrollerProperties)=16
pub struct QScrollerProperties {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QScrollerProperties_ITF interface {
//    QScrollerProperties_PTR() *QScrollerProperties
//}
//func (ptr *QScrollerProperties) QScrollerProperties_PTR() *QScrollerProperties { return ptr }

impl /*struct*/ QScrollerProperties {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QScrollerProperties {
    return QScrollerProperties{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QScrollerProperties {
//  type Target = QScrollerPropertiesBASE;
//
//  fn deref(&self) -> &QScrollerPropertiesBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QScrollerPropertiesBASE> for QScrollerProperties {
//  fn as_ref(& self) -> & QScrollerPropertiesBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qscrollerproperties.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QScrollerProperties()

/*
Constructs new scroller properties.
*/
// QScrollerProperties() ctx.fn_proto_cpp
impl /*struct*/ QScrollerProperties {
  pub fn QScrollerProperties_0<T: QScrollerProperties_QScrollerProperties_0>(value: T) -> QScrollerProperties {
    let rsthis = value.QScrollerProperties_0();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollerProperties_QScrollerProperties_0 {
  fn QScrollerProperties_0(self) -> QScrollerProperties;
}
// QScrollerProperties() ctx.fn_proto_cpp
impl<'a> /*trait*/ QScrollerProperties_QScrollerProperties_0 for () {
  fn QScrollerProperties_0(self) -> QScrollerProperties {
    // unsafe{_ZN19QScrollerPropertiesC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QScrollerPropertiesC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QScrollerProperties{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollerproperties.h:62
// index:0
// Public Visibility=Default Availability=Available
// [16] QScrollerProperties & operator=(const QScrollerProperties &)

/*

*/
impl /*struct*/ QScrollerProperties {
  pub fn operator_equal_0<RetType, T: QScrollerProperties_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QScrollerProperties_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QScrollerProperties) -> RetType;
}
impl<'a> /*trait*/ QScrollerProperties_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QScrollerProperties) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QScrollerPropertiesaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollerproperties.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QScrollerProperties()

/*

*/
pub fn DeleteQScrollerProperties(this :*mut QScrollerProperties) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QScrollerPropertiesD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qscrollerproperties.h:65
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QScrollerProperties &) const

/*

*/
impl /*struct*/ QScrollerProperties {
  pub fn operator_equal_equal_0<RetType, T: QScrollerProperties_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QScrollerProperties_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QScrollerProperties) -> RetType;
}
impl<'a> /*trait*/ QScrollerProperties_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QScrollerProperties) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QScrollerPropertieseqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollerproperties.h:66
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QScrollerProperties &) const

/*

*/
impl /*struct*/ QScrollerProperties {
  pub fn operator_not_equal_0<RetType, T: QScrollerProperties_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QScrollerProperties_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QScrollerProperties) -> RetType;
}
impl<'a> /*trait*/ QScrollerProperties_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QScrollerProperties) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QScrollerPropertiesneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollerproperties.h:68
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setDefaultScrollerProperties(const QScrollerProperties &)

/*
Sets the scroller properties for all new QScrollerProperties objects to sp.

Use this function to override the platform default properties returned by the default constructor. If you only want to change the scroller properties of a single scroller, use QScroller::setScrollerProperties()

Note: Calling this function will not change the content of already existing QScrollerProperties objects.

See also unsetDefaultScrollerProperties().
*/
impl /*struct*/ QScrollerProperties {
  pub fn setDefaultScrollerProperties_0<RetType, T: QScrollerProperties_setDefaultScrollerProperties_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefaultScrollerProperties_0();
    // return 1;
  }
}
pub trait QScrollerProperties_setDefaultScrollerProperties_0<RetType> {
  fn setDefaultScrollerProperties_0(self ) -> RetType;
}
impl<'a> /*trait*/ QScrollerProperties_setDefaultScrollerProperties_0<(/*void*/)> for (usize) {
  fn setDefaultScrollerProperties_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QScrollerProperties28setDefaultScrollerPropertiesERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollerproperties.h:69
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void unsetDefaultScrollerProperties()

/*
Sets the scroller properties returned by the default constructor back to the platform default properties.

See also setDefaultScrollerProperties().
*/
impl /*struct*/ QScrollerProperties {
  pub fn unsetDefaultScrollerProperties_0<RetType, T: QScrollerProperties_unsetDefaultScrollerProperties_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.unsetDefaultScrollerProperties_0();
    // return 1;
  }
}
pub trait QScrollerProperties_unsetDefaultScrollerProperties_0<RetType> {
  fn unsetDefaultScrollerProperties_0(self ) -> RetType;
}
impl<'a> /*trait*/ QScrollerProperties_unsetDefaultScrollerProperties_0<(/*void*/)> for () {
  fn unsetDefaultScrollerProperties_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN19QScrollerProperties30unsetDefaultScrollerPropertiesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscrollerproperties.h:117
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant scrollMetric(QScrollerProperties::ScrollMetric) const

/*
Query the metric value of the scroller properties.

See also setScrollMetric() and ScrollMetric.
*/
impl /*struct*/ QScrollerProperties {
  pub fn scrollMetric_0<RetType, T: QScrollerProperties_scrollMetric_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollMetric_0(self);
    // return 1;
  }
}
pub trait QScrollerProperties_scrollMetric_0<RetType> {
  fn scrollMetric_0(self , rsthis: & QScrollerProperties) -> RetType;
}
impl<'a> /*trait*/ QScrollerProperties_scrollMetric_0<usize> for (i32) {
  fn scrollMetric_0(self , rsthis: & QScrollerProperties) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QScrollerProperties12scrollMetricENS_12ScrollMetricE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscrollerproperties.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScrollMetric(QScrollerProperties::ScrollMetric, const QVariant &)

/*
Set a specific value of the metric ScrollerMetric to value.

See also scrollMetric() and ScrollMetric.
*/
impl /*struct*/ QScrollerProperties {
  pub fn setScrollMetric_0<RetType, T: QScrollerProperties_setScrollMetric_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScrollMetric_0(self);
    // return 1;
  }
}
pub trait QScrollerProperties_setScrollMetric_0<RetType> {
  fn setScrollMetric_0(self , rsthis: & QScrollerProperties) -> RetType;
}
impl<'a> /*trait*/ QScrollerProperties_setScrollMetric_0<(/*void*/)> for (i32,usize) {
  fn setScrollMetric_0(self , rsthis: & QScrollerProperties) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QScrollerProperties15setScrollMetricENS_12ScrollMetricERK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum describes the various modes of overshooting.


*/
pub type QScrollerProperties__OvershootPolicy = i32;
// Overshooting is possible when the content is scrollable. This is the default.
pub const QScrollerProperties__OvershootWhenScrollable :QScrollerProperties__OvershootPolicy = 0;
// Overshooting is never enabled, even when the content is scrollable.
pub const QScrollerProperties__OvershootAlwaysOff :QScrollerProperties__OvershootPolicy = 1;
// Overshooting is always enabled, even when the content is not scrollable.
pub const QScrollerProperties__OvershootAlwaysOn :QScrollerProperties__OvershootPolicy = 2;
pub fn QScrollerProperties_OvershootPolicyItemName(val: i32) ->String {
  match val {
     QScrollerProperties__OvershootWhenScrollable => // 0
     {return String::from("OvershootWhenScrollable");}
     QScrollerProperties__OvershootAlwaysOff => // 1
     {return String::from("OvershootAlwaysOff");}
     QScrollerProperties__OvershootAlwaysOn => // 2
     {return String::from("OvershootAlwaysOn");}
  _ => {return format!("{}", val);}
}
}
pub fn QScrollerProperties_OvershootPolicyItemName_s(val: i32) ->String {
  //var nilthis *QScrollerProperties
  //return nilthis.OvershootPolicyItemName(val);
  return QScrollerProperties_OvershootPolicyItemName(val);
}


/*
This enum describes the available frame rates used while dragging or scrolling.


*/
pub type QScrollerProperties__FrameRates = i32;
// 
pub const QScrollerProperties__Standard :QScrollerProperties__FrameRates = 0;
// 
pub const QScrollerProperties__Fps60 :QScrollerProperties__FrameRates = 1;
// 
pub const QScrollerProperties__Fps30 :QScrollerProperties__FrameRates = 2;
// 
pub const QScrollerProperties__Fps20 :QScrollerProperties__FrameRates = 3;
pub fn QScrollerProperties_FrameRatesItemName(val: i32) ->String {
  match val {
     QScrollerProperties__Standard => // 0
     {return String::from("Standard");}
     QScrollerProperties__Fps60 => // 1
     {return String::from("Fps60");}
     QScrollerProperties__Fps30 => // 2
     {return String::from("Fps30");}
     QScrollerProperties__Fps20 => // 3
     {return String::from("Fps20");}
  _ => {return format!("{}", val);}
}
}
pub fn QScrollerProperties_FrameRatesItemName_s(val: i32) ->String {
  //var nilthis *QScrollerProperties
  //return nilthis.FrameRatesItemName(val);
  return QScrollerProperties_FrameRatesItemName(val);
}


/*
This enum contains the different scroll metric types. When not indicated otherwise the setScrollMetric function expects a QVariant of type qreal.

See the QScroller documentation for further details of the concepts behind the different values.


*/
pub type QScrollerProperties__ScrollMetric = i32;
// This is the time a mouse press event is delayed when starting a flick gesture in [s]. If the gesture is triggered within that time, no mouse press or release is sent to the scrolled object. If it triggers after that delay the delayed mouse press plus a faked release event at global position QPoint(-QWIDGETSIZE_MAX, -QWIDGETSIZE_MAX) is sent. If the gesture is canceled, then both the delayed mouse press plus the real release event are delivered.
pub const QScrollerProperties__MousePressEventDelay :QScrollerProperties__ScrollMetric = 0;
// This is the minimum distance the touch or mouse point needs to be moved before the flick gesture is triggered in m.
pub const QScrollerProperties__DragStartDistance :QScrollerProperties__ScrollMetric = 1;
// 
pub const QScrollerProperties__DragVelocitySmoothingFactor :QScrollerProperties__ScrollMetric = 2;
// 
pub const QScrollerProperties__AxisLockThreshold :QScrollerProperties__ScrollMetric = 3;
// 
pub const QScrollerProperties__ScrollingCurve :QScrollerProperties__ScrollMetric = 4;
// 
pub const QScrollerProperties__DecelerationFactor :QScrollerProperties__ScrollMetric = 5;
// The minimum velocity that is needed after ending the touch or releasing the mouse to start scrolling in m/s.
pub const QScrollerProperties__MinimumVelocity :QScrollerProperties__ScrollMetric = 6;
// This is the maximum velocity that can be reached in m/s.
pub const QScrollerProperties__MaximumVelocity :QScrollerProperties__ScrollMetric = 7;
// This is the maximum allowed scroll speed for a click-through in m/s. This means that a click on a currently (slowly) scrolling object will not only stop the scrolling but the click event will also be delivered to the UI control. This is useful when using exponential-type scrolling curves.
pub const QScrollerProperties__MaximumClickThroughVelocity :QScrollerProperties__ScrollMetric = 8;
// This is the maximum time in seconds that a flick gesture can take to be recognized as an accelerating flick. If set to zero no such gesture is detected. An "accelerating flick" is a flick gesture executed on an already scrolling object. In such cases the scrolling speed is multiplied by AcceleratingFlickSpeedupFactor in order to accelerate it.
pub const QScrollerProperties__AcceleratingFlickMaximumTime :QScrollerProperties__ScrollMetric = 9;
// 
pub const QScrollerProperties__AcceleratingFlickSpeedupFactor :QScrollerProperties__ScrollMetric = 10;
// 
pub const QScrollerProperties__SnapPositionRatio :QScrollerProperties__ScrollMetric = 11;
// 
pub const QScrollerProperties__SnapTime :QScrollerProperties__ScrollMetric = 12;
// 
pub const QScrollerProperties__OvershootDragResistanceFactor :QScrollerProperties__ScrollMetric = 13;
// 
pub const QScrollerProperties__OvershootDragDistanceFactor :QScrollerProperties__ScrollMetric = 14;
// 
pub const QScrollerProperties__OvershootScrollDistanceFactor :QScrollerProperties__ScrollMetric = 15;
// 
pub const QScrollerProperties__OvershootScrollTime :QScrollerProperties__ScrollMetric = 16;
// 
pub const QScrollerProperties__HorizontalOvershootPolicy :QScrollerProperties__ScrollMetric = 17;
// 
pub const QScrollerProperties__VerticalOvershootPolicy :QScrollerProperties__ScrollMetric = 18;
// 
pub const QScrollerProperties__FrameRate :QScrollerProperties__ScrollMetric = 19;
// 
pub const QScrollerProperties__ScrollMetricCount :QScrollerProperties__ScrollMetric = 20;
pub fn QScrollerProperties_ScrollMetricItemName(val: i32) ->String {
  match val {
     QScrollerProperties__MousePressEventDelay => // 0
     {return String::from("MousePressEventDelay");}
     QScrollerProperties__DragStartDistance => // 1
     {return String::from("DragStartDistance");}
     QScrollerProperties__DragVelocitySmoothingFactor => // 2
     {return String::from("DragVelocitySmoothingFactor");}
     QScrollerProperties__AxisLockThreshold => // 3
     {return String::from("AxisLockThreshold");}
     QScrollerProperties__ScrollingCurve => // 4
     {return String::from("ScrollingCurve");}
     QScrollerProperties__DecelerationFactor => // 5
     {return String::from("DecelerationFactor");}
     QScrollerProperties__MinimumVelocity => // 6
     {return String::from("MinimumVelocity");}
     QScrollerProperties__MaximumVelocity => // 7
     {return String::from("MaximumVelocity");}
     QScrollerProperties__MaximumClickThroughVelocity => // 8
     {return String::from("MaximumClickThroughVelocity");}
     QScrollerProperties__AcceleratingFlickMaximumTime => // 9
     {return String::from("AcceleratingFlickMaximumTime");}
     QScrollerProperties__AcceleratingFlickSpeedupFactor => // 10
     {return String::from("AcceleratingFlickSpeedupFactor");}
     QScrollerProperties__SnapPositionRatio => // 11
     {return String::from("SnapPositionRatio");}
     QScrollerProperties__SnapTime => // 12
     {return String::from("SnapTime");}
     QScrollerProperties__OvershootDragResistanceFactor => // 13
     {return String::from("OvershootDragResistanceFactor");}
     QScrollerProperties__OvershootDragDistanceFactor => // 14
     {return String::from("OvershootDragDistanceFactor");}
     QScrollerProperties__OvershootScrollDistanceFactor => // 15
     {return String::from("OvershootScrollDistanceFactor");}
     QScrollerProperties__OvershootScrollTime => // 16
     {return String::from("OvershootScrollTime");}
     QScrollerProperties__HorizontalOvershootPolicy => // 17
     {return String::from("HorizontalOvershootPolicy");}
     QScrollerProperties__VerticalOvershootPolicy => // 18
     {return String::from("VerticalOvershootPolicy");}
     QScrollerProperties__FrameRate => // 19
     {return String::from("FrameRate");}
     QScrollerProperties__ScrollMetricCount => // 20
     {return String::from("ScrollMetricCount");}
  _ => {return format!("{}", val);}
}
}
pub fn QScrollerProperties_ScrollMetricItemName_s(val: i32) ->String {
  //var nilthis *QScrollerProperties
  //return nilthis.ScrollMetricItemName(val);
  return QScrollerProperties_ScrollMetricItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
