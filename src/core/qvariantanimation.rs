

// mod ::core::QVariantAnimation
// package qtcore
// /usr/include/qt/QtCore/qvariantanimation.h
// #include <qvariantanimation.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 67
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// bool event(QEvent *)
// func (this *QVariantAnimation) InheritEvent(f func(event *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void updateCurrentTime(int)
// func (this *QVariantAnimation) InheritUpdateCurrentTime(f func(arg0 int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateCurrentTime", f)
// }

// void updateState(QAbstractAnimation::State, QAbstractAnimation::State)
// func (this *QVariantAnimation) InheritUpdateState(f func(newState int, oldState int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateState", f)
// }

// void updateCurrentValue(const QVariant &)
// func (this *QVariantAnimation) InheritUpdateCurrentValue(f func(value *QVariant) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateCurrentValue", f)
// }

// QVariant interpolated(const QVariant &, const QVariant &, qreal)
// func (this *QVariantAnimation) InheritInterpolated(f func(from *QVariant, to *QVariant, progress float64) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "interpolated", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QVariantAnimation)=16
pub struct QVariantAnimation {
  qbase: QAbstractAnimation,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QVariantAnimation_ITF interface {
//    QAbstractAnimation_ITF
//    QVariantAnimation_PTR() *QVariantAnimation
//}
//func (ptr *QVariantAnimation) QVariantAnimation_PTR() *QVariantAnimation { return ptr }

impl /*struct*/ QVariantAnimation {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QVariantAnimation {
    return QVariantAnimation{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QVariantAnimation {
//  type Target = QVariantAnimationBASE;
//
//  fn deref(&self) -> &QVariantAnimationBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QVariantAnimationBASE> for QVariantAnimation {
//  fn as_ref(& self) -> & QVariantAnimationBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qvariantanimation.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QVariantAnimation {
  pub fn metaObject_0<RetType, T: QVariantAnimation_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QVariantAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QVariantAnimation10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QVariantAnimation(QObject *)

/*
Construct a QVariantAnimation object. parent is passed to QAbstractAnimation's constructor.
*/
// QVariantAnimation(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QVariantAnimation {
  pub fn QVariantAnimation_0<T: QVariantAnimation_QVariantAnimation_0>(value: T) -> QVariantAnimation {
    let rsthis = value.QVariantAnimation_0();
    return rsthis;
    // return 1;
  }
}

pub trait QVariantAnimation_QVariantAnimation_0 {
  fn QVariantAnimation_0(self) -> QVariantAnimation;
}
// QVariantAnimation(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariantAnimation_QVariantAnimation_0 for (usize) {
  fn QVariantAnimation_0(self) -> QVariantAnimation {
    // unsafe{_ZN17QVariantAnimationC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QVariantAnimationC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariantAnimation{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QVariantAnimation()

/*

*/
pub fn DeleteQVariantAnimation(this :*mut QVariantAnimation) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QVariantAnimationD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qvariantanimation.h:71
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant startValue() const

/*

*/
impl /*struct*/ QVariantAnimation {
  pub fn startValue_0<RetType, T: QVariantAnimation_startValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startValue_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_startValue_0<RetType> {
  fn startValue_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_startValue_0<usize> for () {
  fn startValue_0(self , rsthis: & QVariantAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QVariantAnimation10startValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStartValue(const QVariant &)

/*

*/
impl /*struct*/ QVariantAnimation {
  pub fn setStartValue_0<RetType, T: QVariantAnimation_setStartValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStartValue_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_setStartValue_0<RetType> {
  fn setStartValue_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_setStartValue_0<(/*void*/)> for (usize) {
  fn setStartValue_0(self , rsthis: & QVariantAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QVariantAnimation13setStartValueERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:74
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant endValue() const

/*

*/
impl /*struct*/ QVariantAnimation {
  pub fn endValue_0<RetType, T: QVariantAnimation_endValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endValue_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_endValue_0<RetType> {
  fn endValue_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_endValue_0<usize> for () {
  fn endValue_0(self , rsthis: & QVariantAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QVariantAnimation8endValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEndValue(const QVariant &)

/*

*/
impl /*struct*/ QVariantAnimation {
  pub fn setEndValue_0<RetType, T: QVariantAnimation_setEndValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEndValue_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_setEndValue_0<RetType> {
  fn setEndValue_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_setEndValue_0<(/*void*/)> for (usize) {
  fn setEndValue_0(self , rsthis: & QVariantAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QVariantAnimation11setEndValueERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:77
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant keyValueAt(qreal) const

/*
Returns the key frame value for the given step. The given step must be in the range 0 to 1. If there is no KeyValue for step, it returns an invalid QVariant.

See also keyValues() and setKeyValueAt().
*/
impl /*struct*/ QVariantAnimation {
  pub fn keyValueAt_0<RetType, T: QVariantAnimation_keyValueAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyValueAt_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_keyValueAt_0<RetType> {
  fn keyValueAt_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_keyValueAt_0<usize> for (f64) {
  fn keyValueAt_0(self , rsthis: & QVariantAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QVariantAnimation10keyValueAtEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setKeyValueAt(qreal, const QVariant &)

/*
Creates a key frame at the given step with the given value. The given step must be in the range 0 to 1.

See also setKeyValues() and keyValueAt().
*/
impl /*struct*/ QVariantAnimation {
  pub fn setKeyValueAt_0<RetType, T: QVariantAnimation_setKeyValueAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKeyValueAt_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_setKeyValueAt_0<RetType> {
  fn setKeyValueAt_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_setKeyValueAt_0<(/*void*/)> for (f64,usize) {
  fn setKeyValueAt_0(self , rsthis: & QVariantAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QVariantAnimation13setKeyValueAtEdRK8QVariant", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:83
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant currentValue() const

/*

*/
impl /*struct*/ QVariantAnimation {
  pub fn currentValue_0<RetType, T: QVariantAnimation_currentValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentValue_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_currentValue_0<RetType> {
  fn currentValue_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_currentValue_0<usize> for () {
  fn currentValue_0(self , rsthis: & QVariantAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QVariantAnimation12currentValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:85
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int duration() const

/*

*/
impl /*struct*/ QVariantAnimation {
  pub fn duration_0<RetType, T: QVariantAnimation_duration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.duration_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_duration_0<RetType> {
  fn duration_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_duration_0<i32> for () {
  fn duration_0(self , rsthis: & QVariantAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QVariantAnimation8durationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDuration(int)

/*

*/
impl /*struct*/ QVariantAnimation {
  pub fn setDuration_0<RetType, T: QVariantAnimation_setDuration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDuration_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_setDuration_0<RetType> {
  fn setDuration_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_setDuration_0<(/*void*/)> for (i32) {
  fn setDuration_0(self , rsthis: & QVariantAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QVariantAnimation11setDurationEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] QEasingCurve easingCurve() const

/*

*/
impl /*struct*/ QVariantAnimation {
  pub fn easingCurve_0<RetType, T: QVariantAnimation_easingCurve_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.easingCurve_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_easingCurve_0<RetType> {
  fn easingCurve_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_easingCurve_0<usize> for () {
  fn easingCurve_0(self , rsthis: & QVariantAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QVariantAnimation11easingCurveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEasingCurve(const QEasingCurve &)

/*

*/
impl /*struct*/ QVariantAnimation {
  pub fn setEasingCurve_0<RetType, T: QVariantAnimation_setEasingCurve_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEasingCurve_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_setEasingCurve_0<RetType> {
  fn setEasingCurve_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_setEasingCurve_0<(/*void*/)> for (usize) {
  fn setEasingCurve_0(self , rsthis: & QVariantAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QVariantAnimation14setEasingCurveERK12QEasingCurve", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void valueChanged(const QVariant &)

/*
QVariantAnimation emits this signal whenever the current value changes.

Note: Notifier signal for property currentValue. 

See also currentValue, startValue, and endValue.
*/
impl /*struct*/ QVariantAnimation {
  pub fn valueChanged_0<RetType, T: QVariantAnimation_valueChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueChanged_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_valueChanged_0<RetType> {
  fn valueChanged_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_valueChanged_0<(/*void*/)> for (usize) {
  fn valueChanged_0(self , rsthis: & QVariantAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QVariantAnimation12valueChangedERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:98
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QVariantAnimation {
  pub fn event_0<RetType, T: QVariantAnimation_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_event_0<RetType> {
  fn event_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QVariantAnimation) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QVariantAnimation5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:100
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateCurrentTime(int)

/*
Reimplemented from QAbstractAnimation::updateCurrentTime().
*/
impl /*struct*/ QVariantAnimation {
  pub fn updateCurrentTime_0<RetType, T: QVariantAnimation_updateCurrentTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateCurrentTime_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_updateCurrentTime_0<RetType> {
  fn updateCurrentTime_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_updateCurrentTime_0<(/*void*/)> for (i32) {
  fn updateCurrentTime_0(self , rsthis: & QVariantAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QVariantAnimation17updateCurrentTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:101
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateState(QAbstractAnimation::State, QAbstractAnimation::State)

/*
Reimplemented from QAbstractAnimation::updateState().
*/
impl /*struct*/ QVariantAnimation {
  pub fn updateState_0<RetType, T: QVariantAnimation_updateState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateState_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_updateState_0<RetType> {
  fn updateState_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_updateState_0<(/*void*/)> for (i32,i32) {
  fn updateState_0(self , rsthis: & QVariantAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QVariantAnimation11updateStateEN18QAbstractAnimation5StateES1_", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:103
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateCurrentValue(const QVariant &)

/*
This virtual function is called every time the animation's current value changes. The value argument is the new current value.

The base class implementation does nothing.

See also currentValue.
*/
impl /*struct*/ QVariantAnimation {
  pub fn updateCurrentValue_0<RetType, T: QVariantAnimation_updateCurrentValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateCurrentValue_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_updateCurrentValue_0<RetType> {
  fn updateCurrentValue_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_updateCurrentValue_0<(/*void*/)> for (usize) {
  fn updateCurrentValue_0(self , rsthis: & QVariantAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QVariantAnimation18updateCurrentValueERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariantanimation.h:104
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant interpolated(const QVariant &, const QVariant &, qreal) const

/*
This virtual function returns the linear interpolation between variants from and to, at progress, usually a value between 0 and 1. You can reimplement this function in a subclass of QVariantAnimation to provide your own interpolation algorithm.

Note that in order for the interpolation to work with a QEasingCurve that return a value smaller than 0 or larger than 1 (such as QEasingCurve::InBack) you should make sure that it can extrapolate. If the semantic of the datatype does not allow extrapolation this function should handle that gracefully.

You should call the QVariantAnimation implementation of this function if you want your class to handle the types already supported by Qt (see class QVariantAnimation description for a list of supported types).

See also QEasingCurve.
*/
impl /*struct*/ QVariantAnimation {
  pub fn interpolated_0<RetType, T: QVariantAnimation_interpolated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.interpolated_0(self);
    // return 1;
  }
}
pub trait QVariantAnimation_interpolated_0<RetType> {
  fn interpolated_0(self , rsthis: & QVariantAnimation) -> RetType;
}
impl<'a> /*trait*/ QVariantAnimation_interpolated_0<usize> for (usize,usize,f64) {
  fn interpolated_0(self , rsthis: & QVariantAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QVariantAnimation12interpolatedERK8QVariantS2_d", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
