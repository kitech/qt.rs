

// mod ::gui::QStyleHints
// package qtgui
// /usr/include/qt/QtGui/qstylehints.h
// #include <qstylehints.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 20
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
#[derive(Default)] // class sizeof(QStyleHints)=16
pub struct QStyleHints {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleHints_ITF interface {
//    qtcore.QObject_ITF
//    QStyleHints_PTR() *QStyleHints
//}
//func (ptr *QStyleHints) QStyleHints_PTR() *QStyleHints { return ptr }

impl /*struct*/ QStyleHints {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleHints {
    return QStyleHints{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleHints {
//  type Target = QStyleHintsBASE;
//
//  fn deref(&self) -> &QStyleHintsBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleHintsBASE> for QStyleHints {
//  fn as_ref(& self) -> & QStyleHintsBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qstylehints.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn metaObject_0<RetType, T: QStyleHints_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QStyleHints_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QStyleHints) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMouseDoubleClickInterval(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn setMouseDoubleClickInterval_0<RetType, T: QStyleHints_setMouseDoubleClickInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMouseDoubleClickInterval_0(self);
    // return 1;
  }
}
pub trait QStyleHints_setMouseDoubleClickInterval_0<RetType> {
  fn setMouseDoubleClickInterval_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_setMouseDoubleClickInterval_0<(/*void*/)> for (i32) {
  fn setMouseDoubleClickInterval_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints27setMouseDoubleClickIntervalEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:79
// index:0
// Public Visibility=Default Availability=Available
// [4] int mouseDoubleClickInterval() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn mouseDoubleClickInterval_0<RetType, T: QStyleHints_mouseDoubleClickInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickInterval_0(self);
    // return 1;
  }
}
pub trait QStyleHints_mouseDoubleClickInterval_0<RetType> {
  fn mouseDoubleClickInterval_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_mouseDoubleClickInterval_0<i32> for () {
  fn mouseDoubleClickInterval_0(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints24mouseDoubleClickIntervalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMousePressAndHoldInterval(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn setMousePressAndHoldInterval_0<RetType, T: QStyleHints_setMousePressAndHoldInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMousePressAndHoldInterval_0(self);
    // return 1;
  }
}
pub trait QStyleHints_setMousePressAndHoldInterval_0<RetType> {
  fn setMousePressAndHoldInterval_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_setMousePressAndHoldInterval_0<(/*void*/)> for (i32) {
  fn setMousePressAndHoldInterval_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints28setMousePressAndHoldIntervalEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:81
// index:0
// Public Visibility=Default Availability=Available
// [4] int mousePressAndHoldInterval() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn mousePressAndHoldInterval_0<RetType, T: QStyleHints_mousePressAndHoldInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressAndHoldInterval_0(self);
    // return 1;
  }
}
pub trait QStyleHints_mousePressAndHoldInterval_0<RetType> {
  fn mousePressAndHoldInterval_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_mousePressAndHoldInterval_0<i32> for () {
  fn mousePressAndHoldInterval_0(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints25mousePressAndHoldIntervalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStartDragDistance(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn setStartDragDistance_0<RetType, T: QStyleHints_setStartDragDistance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStartDragDistance_0(self);
    // return 1;
  }
}
pub trait QStyleHints_setStartDragDistance_0<RetType> {
  fn setStartDragDistance_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_setStartDragDistance_0<(/*void*/)> for (i32) {
  fn setStartDragDistance_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints20setStartDragDistanceEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:83
// index:0
// Public Visibility=Default Availability=Available
// [4] int startDragDistance() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn startDragDistance_0<RetType, T: QStyleHints_startDragDistance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startDragDistance_0(self);
    // return 1;
  }
}
pub trait QStyleHints_startDragDistance_0<RetType> {
  fn startDragDistance_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_startDragDistance_0<i32> for () {
  fn startDragDistance_0(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints17startDragDistanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStartDragTime(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn setStartDragTime_0<RetType, T: QStyleHints_setStartDragTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStartDragTime_0(self);
    // return 1;
  }
}
pub trait QStyleHints_setStartDragTime_0<RetType> {
  fn setStartDragTime_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_setStartDragTime_0<(/*void*/)> for (i32) {
  fn setStartDragTime_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints16setStartDragTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:85
// index:0
// Public Visibility=Default Availability=Available
// [4] int startDragTime() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn startDragTime_0<RetType, T: QStyleHints_startDragTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startDragTime_0(self);
    // return 1;
  }
}
pub trait QStyleHints_startDragTime_0<RetType> {
  fn startDragTime_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_startDragTime_0<i32> for () {
  fn startDragTime_0(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints13startDragTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:86
// index:0
// Public Visibility=Default Availability=Available
// [4] int startDragVelocity() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn startDragVelocity_0<RetType, T: QStyleHints_startDragVelocity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startDragVelocity_0(self);
    // return 1;
  }
}
pub trait QStyleHints_startDragVelocity_0<RetType> {
  fn startDragVelocity_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_startDragVelocity_0<i32> for () {
  fn startDragVelocity_0(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints17startDragVelocityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setKeyboardInputInterval(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn setKeyboardInputInterval_0<RetType, T: QStyleHints_setKeyboardInputInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKeyboardInputInterval_0(self);
    // return 1;
  }
}
pub trait QStyleHints_setKeyboardInputInterval_0<RetType> {
  fn setKeyboardInputInterval_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_setKeyboardInputInterval_0<(/*void*/)> for (i32) {
  fn setKeyboardInputInterval_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints24setKeyboardInputIntervalEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:88
// index:0
// Public Visibility=Default Availability=Available
// [4] int keyboardInputInterval() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn keyboardInputInterval_0<RetType, T: QStyleHints_keyboardInputInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyboardInputInterval_0(self);
    // return 1;
  }
}
pub trait QStyleHints_keyboardInputInterval_0<RetType> {
  fn keyboardInputInterval_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_keyboardInputInterval_0<i32> for () {
  fn keyboardInputInterval_0(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints21keyboardInputIntervalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:89
// index:0
// Public Visibility=Default Availability=Available
// [4] int keyboardAutoRepeatRate() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn keyboardAutoRepeatRate_0<RetType, T: QStyleHints_keyboardAutoRepeatRate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyboardAutoRepeatRate_0(self);
    // return 1;
  }
}
pub trait QStyleHints_keyboardAutoRepeatRate_0<RetType> {
  fn keyboardAutoRepeatRate_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_keyboardAutoRepeatRate_0<i32> for () {
  fn keyboardAutoRepeatRate_0(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints22keyboardAutoRepeatRateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCursorFlashTime(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn setCursorFlashTime_0<RetType, T: QStyleHints_setCursorFlashTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCursorFlashTime_0(self);
    // return 1;
  }
}
pub trait QStyleHints_setCursorFlashTime_0<RetType> {
  fn setCursorFlashTime_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_setCursorFlashTime_0<(/*void*/)> for (i32) {
  fn setCursorFlashTime_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints18setCursorFlashTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:91
// index:0
// Public Visibility=Default Availability=Available
// [4] int cursorFlashTime() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn cursorFlashTime_0<RetType, T: QStyleHints_cursorFlashTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorFlashTime_0(self);
    // return 1;
  }
}
pub trait QStyleHints_cursorFlashTime_0<RetType> {
  fn cursorFlashTime_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_cursorFlashTime_0<i32> for () {
  fn cursorFlashTime_0(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints15cursorFlashTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:92
// index:0
// Public Visibility=Default Availability=Available
// [1] bool showIsFullScreen() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn showIsFullScreen_0<RetType, T: QStyleHints_showIsFullScreen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showIsFullScreen_0(self);
    // return 1;
  }
}
pub trait QStyleHints_showIsFullScreen_0<RetType> {
  fn showIsFullScreen_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_showIsFullScreen_0<bool> for () {
  fn showIsFullScreen_0(self , rsthis: & QStyleHints) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints16showIsFullScreenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:93
// index:0
// Public Visibility=Default Availability=Available
// [1] bool showIsMaximized() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn showIsMaximized_0<RetType, T: QStyleHints_showIsMaximized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showIsMaximized_0(self);
    // return 1;
  }
}
pub trait QStyleHints_showIsMaximized_0<RetType> {
  fn showIsMaximized_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_showIsMaximized_0<bool> for () {
  fn showIsMaximized_0(self , rsthis: & QStyleHints) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints15showIsMaximizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:94
// index:0
// Public Visibility=Default Availability=Available
// [1] bool showShortcutsInContextMenus() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn showShortcutsInContextMenus_0<RetType, T: QStyleHints_showShortcutsInContextMenus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showShortcutsInContextMenus_0(self);
    // return 1;
  }
}
pub trait QStyleHints_showShortcutsInContextMenus_0<RetType> {
  fn showShortcutsInContextMenus_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_showShortcutsInContextMenus_0<bool> for () {
  fn showShortcutsInContextMenus_0(self , rsthis: & QStyleHints) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints27showShortcutsInContextMenusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:95
// index:0
// Public Visibility=Default Availability=Available
// [4] int passwordMaskDelay() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn passwordMaskDelay_0<RetType, T: QStyleHints_passwordMaskDelay_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.passwordMaskDelay_0(self);
    // return 1;
  }
}
pub trait QStyleHints_passwordMaskDelay_0<RetType> {
  fn passwordMaskDelay_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_passwordMaskDelay_0<i32> for () {
  fn passwordMaskDelay_0(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints17passwordMaskDelayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:96
// index:0
// Public Visibility=Default Availability=Available
// [2] QChar passwordMaskCharacter() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn passwordMaskCharacter_0<RetType, T: QStyleHints_passwordMaskCharacter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.passwordMaskCharacter_0(self);
    // return 1;
  }
}
pub trait QStyleHints_passwordMaskCharacter_0<RetType> {
  fn passwordMaskCharacter_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_passwordMaskCharacter_0<usize> for () {
  fn passwordMaskCharacter_0(self , rsthis: & QStyleHints) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints21passwordMaskCharacterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal fontSmoothingGamma() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn fontSmoothingGamma_0<RetType, T: QStyleHints_fontSmoothingGamma_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontSmoothingGamma_0(self);
    // return 1;
  }
}
pub trait QStyleHints_fontSmoothingGamma_0<RetType> {
  fn fontSmoothingGamma_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_fontSmoothingGamma_0<f64> for () {
  fn fontSmoothingGamma_0(self , rsthis: & QStyleHints) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints18fontSmoothingGammaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:98
// index:0
// Public Visibility=Default Availability=Available
// [1] bool useRtlExtensions() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn useRtlExtensions_0<RetType, T: QStyleHints_useRtlExtensions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.useRtlExtensions_0(self);
    // return 1;
  }
}
pub trait QStyleHints_useRtlExtensions_0<RetType> {
  fn useRtlExtensions_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_useRtlExtensions_0<bool> for () {
  fn useRtlExtensions_0(self , rsthis: & QStyleHints) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints16useRtlExtensionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:99
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setFocusOnTouchRelease() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn setFocusOnTouchRelease_0<RetType, T: QStyleHints_setFocusOnTouchRelease_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFocusOnTouchRelease_0(self);
    // return 1;
  }
}
pub trait QStyleHints_setFocusOnTouchRelease_0<RetType> {
  fn setFocusOnTouchRelease_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_setFocusOnTouchRelease_0<bool> for () {
  fn setFocusOnTouchRelease_0(self , rsthis: & QStyleHints) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints22setFocusOnTouchReleaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:100
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TabFocusBehavior tabFocusBehavior() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn tabFocusBehavior_0<RetType, T: QStyleHints_tabFocusBehavior_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabFocusBehavior_0(self);
    // return 1;
  }
}
pub trait QStyleHints_tabFocusBehavior_0<RetType> {
  fn tabFocusBehavior_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_tabFocusBehavior_0<i32> for () {
  fn tabFocusBehavior_0(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints16tabFocusBehaviorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabFocusBehavior(Qt::TabFocusBehavior)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn setTabFocusBehavior_0<RetType, T: QStyleHints_setTabFocusBehavior_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabFocusBehavior_0(self);
    // return 1;
  }
}
pub trait QStyleHints_setTabFocusBehavior_0<RetType> {
  fn setTabFocusBehavior_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_setTabFocusBehavior_0<(/*void*/)> for (i32) {
  fn setTabFocusBehavior_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints19setTabFocusBehaviorEN2Qt16TabFocusBehaviorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:102
// index:0
// Public Visibility=Default Availability=Available
// [1] bool singleClickActivation() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn singleClickActivation_0<RetType, T: QStyleHints_singleClickActivation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.singleClickActivation_0(self);
    // return 1;
  }
}
pub trait QStyleHints_singleClickActivation_0<RetType> {
  fn singleClickActivation_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_singleClickActivation_0<bool> for () {
  fn singleClickActivation_0(self , rsthis: & QStyleHints) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints21singleClickActivationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:103
// index:0
// Public Visibility=Default Availability=Available
// [1] bool useHoverEffects() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn useHoverEffects_0<RetType, T: QStyleHints_useHoverEffects_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.useHoverEffects_0(self);
    // return 1;
  }
}
pub trait QStyleHints_useHoverEffects_0<RetType> {
  fn useHoverEffects_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_useHoverEffects_0<bool> for () {
  fn useHoverEffects_0(self , rsthis: & QStyleHints) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints15useHoverEffectsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUseHoverEffects(bool)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn setUseHoverEffects_0<RetType, T: QStyleHints_setUseHoverEffects_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUseHoverEffects_0(self);
    // return 1;
  }
}
pub trait QStyleHints_setUseHoverEffects_0<RetType> {
  fn setUseHoverEffects_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_setUseHoverEffects_0<(/*void*/)> for (bool) {
  fn setUseHoverEffects_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints18setUseHoverEffectsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:105
// index:0
// Public Visibility=Default Availability=Available
// [4] int wheelScrollLines() const

/*

*/
impl /*struct*/ QStyleHints {
  pub fn wheelScrollLines_0<RetType, T: QStyleHints_wheelScrollLines_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelScrollLines_0(self);
    // return 1;
  }
}
pub trait QStyleHints_wheelScrollLines_0<RetType> {
  fn wheelScrollLines_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_wheelScrollLines_0<i32> for () {
  fn wheelScrollLines_0(self , rsthis: & QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStyleHints16wheelScrollLinesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWheelScrollLines(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn setWheelScrollLines_0<RetType, T: QStyleHints_setWheelScrollLines_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWheelScrollLines_0(self);
    // return 1;
  }
}
pub trait QStyleHints_setWheelScrollLines_0<RetType> {
  fn setWheelScrollLines_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_setWheelScrollLines_0<(/*void*/)> for (i32) {
  fn setWheelScrollLines_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints19setWheelScrollLinesEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cursorFlashTimeChanged(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn cursorFlashTimeChanged_0<RetType, T: QStyleHints_cursorFlashTimeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorFlashTimeChanged_0(self);
    // return 1;
  }
}
pub trait QStyleHints_cursorFlashTimeChanged_0<RetType> {
  fn cursorFlashTimeChanged_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_cursorFlashTimeChanged_0<(/*void*/)> for (i32) {
  fn cursorFlashTimeChanged_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints22cursorFlashTimeChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void keyboardInputIntervalChanged(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn keyboardInputIntervalChanged_0<RetType, T: QStyleHints_keyboardInputIntervalChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyboardInputIntervalChanged_0(self);
    // return 1;
  }
}
pub trait QStyleHints_keyboardInputIntervalChanged_0<RetType> {
  fn keyboardInputIntervalChanged_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_keyboardInputIntervalChanged_0<(/*void*/)> for (i32) {
  fn keyboardInputIntervalChanged_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints28keyboardInputIntervalChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void mouseDoubleClickIntervalChanged(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn mouseDoubleClickIntervalChanged_0<RetType, T: QStyleHints_mouseDoubleClickIntervalChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickIntervalChanged_0(self);
    // return 1;
  }
}
pub trait QStyleHints_mouseDoubleClickIntervalChanged_0<RetType> {
  fn mouseDoubleClickIntervalChanged_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_mouseDoubleClickIntervalChanged_0<(/*void*/)> for (i32) {
  fn mouseDoubleClickIntervalChanged_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints31mouseDoubleClickIntervalChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void mousePressAndHoldIntervalChanged(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn mousePressAndHoldIntervalChanged_0<RetType, T: QStyleHints_mousePressAndHoldIntervalChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressAndHoldIntervalChanged_0(self);
    // return 1;
  }
}
pub trait QStyleHints_mousePressAndHoldIntervalChanged_0<RetType> {
  fn mousePressAndHoldIntervalChanged_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_mousePressAndHoldIntervalChanged_0<(/*void*/)> for (i32) {
  fn mousePressAndHoldIntervalChanged_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints32mousePressAndHoldIntervalChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void startDragDistanceChanged(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn startDragDistanceChanged_0<RetType, T: QStyleHints_startDragDistanceChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startDragDistanceChanged_0(self);
    // return 1;
  }
}
pub trait QStyleHints_startDragDistanceChanged_0<RetType> {
  fn startDragDistanceChanged_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_startDragDistanceChanged_0<(/*void*/)> for (i32) {
  fn startDragDistanceChanged_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints24startDragDistanceChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void startDragTimeChanged(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn startDragTimeChanged_0<RetType, T: QStyleHints_startDragTimeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startDragTimeChanged_0(self);
    // return 1;
  }
}
pub trait QStyleHints_startDragTimeChanged_0<RetType> {
  fn startDragTimeChanged_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_startDragTimeChanged_0<(/*void*/)> for (i32) {
  fn startDragTimeChanged_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints20startDragTimeChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void tabFocusBehaviorChanged(Qt::TabFocusBehavior)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn tabFocusBehaviorChanged_0<RetType, T: QStyleHints_tabFocusBehaviorChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabFocusBehaviorChanged_0(self);
    // return 1;
  }
}
pub trait QStyleHints_tabFocusBehaviorChanged_0<RetType> {
  fn tabFocusBehaviorChanged_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_tabFocusBehaviorChanged_0<(/*void*/)> for (i32) {
  fn tabFocusBehaviorChanged_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints23tabFocusBehaviorChangedEN2Qt16TabFocusBehaviorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void useHoverEffectsChanged(bool)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn useHoverEffectsChanged_0<RetType, T: QStyleHints_useHoverEffectsChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.useHoverEffectsChanged_0(self);
    // return 1;
  }
}
pub trait QStyleHints_useHoverEffectsChanged_0<RetType> {
  fn useHoverEffectsChanged_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_useHoverEffectsChanged_0<(/*void*/)> for (bool) {
  fn useHoverEffectsChanged_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints22useHoverEffectsChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qstylehints.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void wheelScrollLinesChanged(int)

/*

*/
impl /*struct*/ QStyleHints {
  pub fn wheelScrollLinesChanged_0<RetType, T: QStyleHints_wheelScrollLinesChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelScrollLinesChanged_0(self);
    // return 1;
  }
}
pub trait QStyleHints_wheelScrollLinesChanged_0<RetType> {
  fn wheelScrollLinesChanged_0(self , rsthis: & QStyleHints) -> RetType;
}
impl<'a> /*trait*/ QStyleHints_wheelScrollLinesChanged_0<(/*void*/)> for (i32) {
  fn wheelScrollLinesChanged_0(self , rsthis: & QStyleHints) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QStyleHints23wheelScrollLinesChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQStyleHints(this :*mut QStyleHints) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QStyleHintsD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
