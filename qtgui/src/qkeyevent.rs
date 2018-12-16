

// mod ::gui::QKeyEvent
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
// extern C begin: 11
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
#[derive(Default)] // class sizeof(QKeyEvent)=64
pub struct QKeyEvent {
  qbase: QInputEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QKeyEvent_ITF interface {
//    QInputEvent_ITF
//    QKeyEvent_PTR() *QKeyEvent
//}
//func (ptr *QKeyEvent) QKeyEvent_PTR() *QKeyEvent { return ptr }

impl /*struct*/ QKeyEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QKeyEvent {
    return QKeyEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QKeyEvent {
//  type Target = QKeyEventBASE;
//
//  fn deref(&self) -> &QKeyEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QKeyEventBASE> for QKeyEvent {
//  fn as_ref(& self) -> & QKeyEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:338
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QKeyEvent(QEvent::Type, int, Qt::KeyboardModifiers, const QString &, bool, ushort)

/*

*/
// QKeyEvent(QEvent::Type, int, Qt::KeyboardModifiers, const QString &, bool, ushort) ctx.fn_proto_cpp
impl /*struct*/ QKeyEvent {
  pub fn QKeyEvent_0<T: QKeyEvent_QKeyEvent_0>(value: T) -> QKeyEvent {
    let rsthis = value.QKeyEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QKeyEvent_QKeyEvent_0 {
  fn QKeyEvent_0(self) -> QKeyEvent;
}
// QKeyEvent(QEvent::Type, int, Qt::KeyboardModifiers, const QString &, bool, ushort) ctx.fn_proto_cpp
impl<'a> /*trait*/ QKeyEvent_QKeyEvent_0 for (i32,i32,i32,usize,bool,u16) {
  fn QKeyEvent_0(self) -> QKeyEvent {
    // unsafe{_ZN9QKeyEventC2EN6QEvent4TypeEi6QFlagsIN2Qt16KeyboardModifierEERK7QStringbt()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const bool as usize;
    let arg5 = (&self.5) as *const u16 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QKeyEventC2EN6QEvent4TypeEi6QFlagsIN2Qt16KeyboardModifierEERK7QStringbt", 6,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QKeyEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:340
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QKeyEvent(QEvent::Type, int, Qt::KeyboardModifiers, quint32, quint32, quint32, const QString &, bool, ushort)

/*

*/
// QKeyEvent(QEvent::Type, int, Qt::KeyboardModifiers, quint32, quint32, quint32, const QString &, bool, ushort) ctx.fn_proto_cpp
impl /*struct*/ QKeyEvent {
  pub fn QKeyEvent_1<T: QKeyEvent_QKeyEvent_1>(value: T) -> QKeyEvent {
    let rsthis = value.QKeyEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QKeyEvent_QKeyEvent_1 {
  fn QKeyEvent_1(self) -> QKeyEvent;
}
// QKeyEvent(QEvent::Type, int, Qt::KeyboardModifiers, quint32, quint32, quint32, const QString &, bool, ushort) ctx.fn_proto_cpp
impl<'a> /*trait*/ QKeyEvent_QKeyEvent_1 for (i32,i32,i32,u32,u32,u32,usize,bool,u16) {
  fn QKeyEvent_1(self) -> QKeyEvent {
    // unsafe{_ZN9QKeyEventC2EN6QEvent4TypeEi6QFlagsIN2Qt16KeyboardModifierEEjjjRK7QStringbt()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const u32 as usize;
    let arg4 = (&self.4) as *const u32 as usize;
    let arg5 = (&self.5) as *const u32 as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let arg7 = (&self.7) as *const bool as usize;
    let arg8 = (&self.8) as *const u16 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QKeyEventC2EN6QEvent4TypeEi6QFlagsIN2Qt16KeyboardModifierEEjjjRK7QStringbt", 9,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_UINT32,qtrt::FFITY_UINT32,qtrt::FFITY_UINT32,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,0,0,0,0,0,0,0);
    let rsthis = QKeyEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:343
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QKeyEvent()

/*

*/
pub fn DeleteQKeyEvent(this :*mut QKeyEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QKeyEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 64)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:345
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int key() const

/*

*/
impl /*struct*/ QKeyEvent {
  pub fn key_0<RetType, T: QKeyEvent_key_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.key_0(self);
    // return 1;
  }
}
pub trait QKeyEvent_key_0<RetType> {
  fn key_0(self , rsthis: & QKeyEvent) -> RetType;
}
impl<'a> /*trait*/ QKeyEvent_key_0<i32> for () {
  fn key_0(self , rsthis: & QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QKeyEvent3keyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:347
// index:0
// Public Visibility=Default Availability=Available
// [1] bool matches(QKeySequence::StandardKey) const

/*

*/
impl /*struct*/ QKeyEvent {
  pub fn matches_0<RetType, T: QKeyEvent_matches_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.matches_0(self);
    // return 1;
  }
}
pub trait QKeyEvent_matches_0<RetType> {
  fn matches_0(self , rsthis: & QKeyEvent) -> RetType;
}
impl<'a> /*trait*/ QKeyEvent_matches_0<bool> for (i32) {
  fn matches_0(self , rsthis: & QKeyEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QKeyEvent7matchesEN12QKeySequence11StandardKeyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:349
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::KeyboardModifiers modifiers() const

/*

*/
impl /*struct*/ QKeyEvent {
  pub fn modifiers_0<RetType, T: QKeyEvent_modifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modifiers_0(self);
    // return 1;
  }
}
pub trait QKeyEvent_modifiers_0<RetType> {
  fn modifiers_0(self , rsthis: & QKeyEvent) -> RetType;
}
impl<'a> /*trait*/ QKeyEvent_modifiers_0<i32> for () {
  fn modifiers_0(self , rsthis: & QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QKeyEvent9modifiersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:350
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QKeyEvent {
  pub fn text_0<RetType, T: QKeyEvent_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QKeyEvent_text_0<RetType> {
  fn text_0(self , rsthis: & QKeyEvent) -> RetType;
}
impl<'a> /*trait*/ QKeyEvent_text_0<usize> for () {
  fn text_0(self , rsthis: & QKeyEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QKeyEvent4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:351
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isAutoRepeat() const

/*

*/
impl /*struct*/ QKeyEvent {
  pub fn isAutoRepeat_0<RetType, T: QKeyEvent_isAutoRepeat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAutoRepeat_0(self);
    // return 1;
  }
}
pub trait QKeyEvent_isAutoRepeat_0<RetType> {
  fn isAutoRepeat_0(self , rsthis: & QKeyEvent) -> RetType;
}
impl<'a> /*trait*/ QKeyEvent_isAutoRepeat_0<bool> for () {
  fn isAutoRepeat_0(self , rsthis: & QKeyEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QKeyEvent12isAutoRepeatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:352
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int count() const

/*

*/
impl /*struct*/ QKeyEvent {
  pub fn count_0<RetType, T: QKeyEvent_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QKeyEvent_count_0<RetType> {
  fn count_0(self , rsthis: & QKeyEvent) -> RetType;
}
impl<'a> /*trait*/ QKeyEvent_count_0<i32> for () {
  fn count_0(self , rsthis: & QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QKeyEvent5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:354
// index:0
// Public inline Visibility=Default Availability=Available
// [4] quint32 nativeScanCode() const

/*

*/
impl /*struct*/ QKeyEvent {
  pub fn nativeScanCode_0<RetType, T: QKeyEvent_nativeScanCode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nativeScanCode_0(self);
    // return 1;
  }
}
pub trait QKeyEvent_nativeScanCode_0<RetType> {
  fn nativeScanCode_0(self , rsthis: & QKeyEvent) -> RetType;
}
impl<'a> /*trait*/ QKeyEvent_nativeScanCode_0<u32> for () {
  fn nativeScanCode_0(self , rsthis: & QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QKeyEvent14nativeScanCodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:355
// index:0
// Public inline Visibility=Default Availability=Available
// [4] quint32 nativeVirtualKey() const

/*

*/
impl /*struct*/ QKeyEvent {
  pub fn nativeVirtualKey_0<RetType, T: QKeyEvent_nativeVirtualKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nativeVirtualKey_0(self);
    // return 1;
  }
}
pub trait QKeyEvent_nativeVirtualKey_0<RetType> {
  fn nativeVirtualKey_0(self , rsthis: & QKeyEvent) -> RetType;
}
impl<'a> /*trait*/ QKeyEvent_nativeVirtualKey_0<u32> for () {
  fn nativeVirtualKey_0(self , rsthis: & QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QKeyEvent16nativeVirtualKeyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:356
// index:0
// Public inline Visibility=Default Availability=Available
// [4] quint32 nativeModifiers() const

/*

*/
impl /*struct*/ QKeyEvent {
  pub fn nativeModifiers_0<RetType, T: QKeyEvent_nativeModifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nativeModifiers_0(self);
    // return 1;
  }
}
pub trait QKeyEvent_nativeModifiers_0<RetType> {
  fn nativeModifiers_0(self , rsthis: & QKeyEvent) -> RetType;
}
impl<'a> /*trait*/ QKeyEvent_nativeModifiers_0<u32> for () {
  fn nativeModifiers_0(self , rsthis: & QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QKeyEvent15nativeModifiersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
