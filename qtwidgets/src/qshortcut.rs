

// mod ::widgets::QShortcut
// package qtwidgets
// /usr/include/qt/QtWidgets/qshortcut.h
// #include <qshortcut.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 25
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

// bool event(QEvent *)
// func (this *QShortcut) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QShortcut)=16
pub struct QShortcut {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QShortcut_ITF interface {
//    qtcore.QObject_ITF
//    QShortcut_PTR() *QShortcut
//}
//func (ptr *QShortcut) QShortcut_PTR() *QShortcut { return ptr }

impl /*struct*/ QShortcut {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QShortcut {
    return QShortcut{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QShortcut {
//  type Target = QShortcutBASE;
//
//  fn deref(&self) -> &QShortcutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QShortcutBASE> for QShortcut {
//  fn as_ref(& self) -> & QShortcutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qshortcut.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QShortcut {
  pub fn metaObject_0<RetType, T: QShortcut_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QShortcut_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QShortcut) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QShortcut10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QShortcut(QWidget *)

/*
Constructs a QShortcut object for the parent widget. Since no shortcut key sequence is specified, the shortcut will not emit any signals.

See also setKey().
*/
// QShortcut(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QShortcut {
  pub fn QShortcut_0<T: QShortcut_QShortcut_0>(value: T) -> QShortcut {
    let rsthis = value.QShortcut_0();
    return rsthis;
    // return 1;
  }
}

pub trait QShortcut_QShortcut_0 {
  fn QShortcut_0(self) -> QShortcut;
}
// QShortcut(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QShortcut_QShortcut_0 for (usize) {
  fn QShortcut_0(self) -> QShortcut {
    // unsafe{_ZN9QShortcutC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QShortcutC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QShortcut{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:64
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QShortcut(const QKeySequence &, QWidget *, const char *, const char *, Qt::ShortcutContext)

/*
Constructs a QShortcut object for the parent widget. Since no shortcut key sequence is specified, the shortcut will not emit any signals.

See also setKey().
*/
// QShortcut(const QKeySequence &, QWidget *, const char *, const char *, Qt::ShortcutContext) ctx.fn_proto_cpp
impl /*struct*/ QShortcut {
  pub fn QShortcut_1<T: QShortcut_QShortcut_1>(value: T) -> QShortcut {
    let rsthis = value.QShortcut_1();
    return rsthis;
    // return 1;
  }
}

pub trait QShortcut_QShortcut_1 {
  fn QShortcut_1(self) -> QShortcut;
}
// QShortcut(const QKeySequence &, QWidget *, const char *, const char *, Qt::ShortcutContext) ctx.fn_proto_cpp
impl<'a> /*trait*/ QShortcut_QShortcut_1 for (usize,usize,usize,usize,i32) {
  fn QShortcut_1(self) -> QShortcut {
    // unsafe{_ZN9QShortcutC2ERK12QKeySequenceP7QWidgetPKcS6_N2Qt15ShortcutContextE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (self.2) as *const usize as usize;
    let arg3 = (self.3) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QShortcutC2ERK12QKeySequenceP7QWidgetPKcS6_N2Qt15ShortcutContextE", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QShortcut{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QShortcut()

/*

*/
pub fn DeleteQShortcut(this :*mut QShortcut) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QShortcutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qshortcut.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setKey(const QKeySequence &)

/*

*/
impl /*struct*/ QShortcut {
  pub fn setKey_0<RetType, T: QShortcut_setKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKey_0(self);
    // return 1;
  }
}
pub trait QShortcut_setKey_0<RetType> {
  fn setKey_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_setKey_0<(/*void*/)> for (usize) {
  fn setKey_0(self , rsthis: & QShortcut) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QShortcut6setKeyERK12QKeySequence", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:70
// index:0
// Public Visibility=Default Availability=Available
// [8] QKeySequence key() const

/*

*/
impl /*struct*/ QShortcut {
  pub fn key_0<RetType, T: QShortcut_key_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.key_0(self);
    // return 1;
  }
}
pub trait QShortcut_key_0<RetType> {
  fn key_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_key_0<usize> for () {
  fn key_0(self , rsthis: & QShortcut) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QShortcut3keyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEnabled(bool)

/*

*/
impl /*struct*/ QShortcut {
  pub fn setEnabled_0<RetType, T: QShortcut_setEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEnabled_0(self);
    // return 1;
  }
}
pub trait QShortcut_setEnabled_0<RetType> {
  fn setEnabled_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_setEnabled_0<(/*void*/)> for (bool) {
  fn setEnabled_0(self , rsthis: & QShortcut) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QShortcut10setEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:73
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEnabled() const

/*

*/
impl /*struct*/ QShortcut {
  pub fn isEnabled_0<RetType, T: QShortcut_isEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEnabled_0(self);
    // return 1;
  }
}
pub trait QShortcut_isEnabled_0<RetType> {
  fn isEnabled_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_isEnabled_0<bool> for () {
  fn isEnabled_0(self , rsthis: & QShortcut) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QShortcut9isEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setContext(Qt::ShortcutContext)

/*

*/
impl /*struct*/ QShortcut {
  pub fn setContext_0<RetType, T: QShortcut_setContext_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setContext_0(self);
    // return 1;
  }
}
pub trait QShortcut_setContext_0<RetType> {
  fn setContext_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_setContext_0<(/*void*/)> for (i32) {
  fn setContext_0(self , rsthis: & QShortcut) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QShortcut10setContextEN2Qt15ShortcutContextE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ShortcutContext context() const

/*

*/
impl /*struct*/ QShortcut {
  pub fn context_0<RetType, T: QShortcut_context_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.context_0(self);
    // return 1;
  }
}
pub trait QShortcut_context_0<RetType> {
  fn context_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_context_0<i32> for () {
  fn context_0(self , rsthis: & QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QShortcut7contextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWhatsThis(const QString &)

/*

*/
impl /*struct*/ QShortcut {
  pub fn setWhatsThis_0<RetType, T: QShortcut_setWhatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis_0(self);
    // return 1;
  }
}
pub trait QShortcut_setWhatsThis_0<RetType> {
  fn setWhatsThis_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_setWhatsThis_0<(/*void*/)> for (usize) {
  fn setWhatsThis_0(self , rsthis: & QShortcut) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QShortcut12setWhatsThisERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] QString whatsThis() const

/*

*/
impl /*struct*/ QShortcut {
  pub fn whatsThis_0<RetType, T: QShortcut_whatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.whatsThis_0(self);
    // return 1;
  }
}
pub trait QShortcut_whatsThis_0<RetType> {
  fn whatsThis_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_whatsThis_0<usize> for () {
  fn whatsThis_0(self , rsthis: & QShortcut) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QShortcut9whatsThisEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoRepeat(bool)

/*

*/
impl /*struct*/ QShortcut {
  pub fn setAutoRepeat_0<RetType, T: QShortcut_setAutoRepeat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoRepeat_0(self);
    // return 1;
  }
}
pub trait QShortcut_setAutoRepeat_0<RetType> {
  fn setAutoRepeat_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_setAutoRepeat_0<(/*void*/)> for (bool) {
  fn setAutoRepeat_0(self , rsthis: & QShortcut) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QShortcut13setAutoRepeatEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:82
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoRepeat() const

/*

*/
impl /*struct*/ QShortcut {
  pub fn autoRepeat_0<RetType, T: QShortcut_autoRepeat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoRepeat_0(self);
    // return 1;
  }
}
pub trait QShortcut_autoRepeat_0<RetType> {
  fn autoRepeat_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_autoRepeat_0<bool> for () {
  fn autoRepeat_0(self , rsthis: & QShortcut) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QShortcut10autoRepeatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:84
// index:0
// Public Visibility=Default Availability=Available
// [4] int id() const

/*
Returns the shortcut's ID.

See also QShortcutEvent::shortcutId().
*/
impl /*struct*/ QShortcut {
  pub fn id_0<RetType, T: QShortcut_id_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.id_0(self);
    // return 1;
  }
}
pub trait QShortcut_id_0<RetType> {
  fn id_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_id_0<i32> for () {
  fn id_0(self , rsthis: & QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QShortcut2idEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:86
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QWidget * parentWidget() const

/*
Returns the shortcut's parent widget.
*/
impl /*struct*/ QShortcut {
  pub fn parentWidget_0<RetType, T: QShortcut_parentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parentWidget_0(self);
    // return 1;
  }
}
pub trait QShortcut_parentWidget_0<RetType> {
  fn parentWidget_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_parentWidget_0<usize> for () {
  fn parentWidget_0(self , rsthis: & QShortcut) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QShortcut12parentWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activated()

/*
This signal is emitted when the user types the shortcut's key sequence.

See also activatedAmbiguously().
*/
impl /*struct*/ QShortcut {
  pub fn activated_0<RetType, T: QShortcut_activated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activated_0(self);
    // return 1;
  }
}
pub trait QShortcut_activated_0<RetType> {
  fn activated_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_activated_0<(/*void*/)> for () {
  fn activated_0(self , rsthis: & QShortcut) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QShortcut9activatedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activatedAmbiguously()

/*
When a key sequence is being typed at the keyboard, it is said to be ambiguous as long as it matches the start of more than one shortcut.

When a shortcut's key sequence is completed, activatedAmbiguously() is emitted if the key sequence is still ambiguous (i.e., it is the start of one or more other shortcuts). The activated() signal is not emitted in this case.

See also activated().
*/
impl /*struct*/ QShortcut {
  pub fn activatedAmbiguously_0<RetType, T: QShortcut_activatedAmbiguously_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activatedAmbiguously_0(self);
    // return 1;
  }
}
pub trait QShortcut_activatedAmbiguously_0<RetType> {
  fn activatedAmbiguously_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_activatedAmbiguously_0<(/*void*/)> for () {
  fn activatedAmbiguously_0(self , rsthis: & QShortcut) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QShortcut20activatedAmbiguouslyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qshortcut.h:94
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*

*/
impl /*struct*/ QShortcut {
  pub fn event_0<RetType, T: QShortcut_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QShortcut_event_0<RetType> {
  fn event_0(self , rsthis: & QShortcut) -> RetType;
}
impl<'a> /*trait*/ QShortcut_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QShortcut) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QShortcut5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
