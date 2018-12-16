

// mod ::widgets::QKeySequenceEdit
// package qtwidgets
// /usr/include/qt/QtWidgets/qkeysequenceedit.h
// #include <qkeysequenceedit.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 10
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
// func (this *QKeySequenceEdit) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QKeySequenceEdit) InheritKeyPressEvent(f func(arg0 *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void keyReleaseEvent(QKeyEvent *)
// func (this *QKeySequenceEdit) InheritKeyReleaseEvent(f func(arg0 *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyReleaseEvent", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QKeySequenceEdit) InheritTimerEvent(f func(arg0 *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QKeySequenceEdit)=48
pub struct QKeySequenceEdit {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QKeySequenceEdit_ITF interface {
//    QWidget_ITF
//    QKeySequenceEdit_PTR() *QKeySequenceEdit
//}
//func (ptr *QKeySequenceEdit) QKeySequenceEdit_PTR() *QKeySequenceEdit { return ptr }

impl /*struct*/ QKeySequenceEdit {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QKeySequenceEdit {
    return QKeySequenceEdit{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QKeySequenceEdit {
//  type Target = QKeySequenceEditBASE;
//
//  fn deref(&self) -> &QKeySequenceEditBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QKeySequenceEditBASE> for QKeySequenceEdit {
//  fn as_ref(& self) -> & QKeySequenceEditBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qkeysequenceedit.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QKeySequenceEdit {
  pub fn metaObject_0<RetType, T: QKeySequenceEdit_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QKeySequenceEdit_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QKeySequenceEdit) -> RetType;
}
impl<'a> /*trait*/ QKeySequenceEdit_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QKeySequenceEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QKeySequenceEdit10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qkeysequenceedit.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QKeySequenceEdit(QWidget *)

/*
Constructs a QKeySequenceEdit widget with the given parent.
*/
// QKeySequenceEdit(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QKeySequenceEdit {
  pub fn QKeySequenceEdit_0<T: QKeySequenceEdit_QKeySequenceEdit_0>(value: T) -> QKeySequenceEdit {
    let rsthis = value.QKeySequenceEdit_0();
    return rsthis;
    // return 1;
  }
}

pub trait QKeySequenceEdit_QKeySequenceEdit_0 {
  fn QKeySequenceEdit_0(self) -> QKeySequenceEdit;
}
// QKeySequenceEdit(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QKeySequenceEdit_QKeySequenceEdit_0 for (usize) {
  fn QKeySequenceEdit_0(self) -> QKeySequenceEdit {
    // unsafe{_ZN16QKeySequenceEditC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QKeySequenceEditC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QKeySequenceEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qkeysequenceedit.h:59
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QKeySequenceEdit(const QKeySequence &, QWidget *)

/*
Constructs a QKeySequenceEdit widget with the given parent.
*/
// QKeySequenceEdit(const QKeySequence &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QKeySequenceEdit {
  pub fn QKeySequenceEdit_1<T: QKeySequenceEdit_QKeySequenceEdit_1>(value: T) -> QKeySequenceEdit {
    let rsthis = value.QKeySequenceEdit_1();
    return rsthis;
    // return 1;
  }
}

pub trait QKeySequenceEdit_QKeySequenceEdit_1 {
  fn QKeySequenceEdit_1(self) -> QKeySequenceEdit;
}
// QKeySequenceEdit(const QKeySequence &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QKeySequenceEdit_QKeySequenceEdit_1 for (usize,usize) {
  fn QKeySequenceEdit_1(self) -> QKeySequenceEdit {
    // unsafe{_ZN16QKeySequenceEditC2ERK12QKeySequenceP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QKeySequenceEditC2ERK12QKeySequenceP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QKeySequenceEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qkeysequenceedit.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QKeySequenceEdit()

/*

*/
pub fn DeleteQKeySequenceEdit(this :*mut QKeySequenceEdit) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QKeySequenceEditD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qkeysequenceedit.h:62
// index:0
// Public Visibility=Default Availability=Available
// [8] QKeySequence keySequence() const

/*

*/
impl /*struct*/ QKeySequenceEdit {
  pub fn keySequence_0<RetType, T: QKeySequenceEdit_keySequence_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keySequence_0(self);
    // return 1;
  }
}
pub trait QKeySequenceEdit_keySequence_0<RetType> {
  fn keySequence_0(self , rsthis: & QKeySequenceEdit) -> RetType;
}
impl<'a> /*trait*/ QKeySequenceEdit_keySequence_0<usize> for () {
  fn keySequence_0(self , rsthis: & QKeySequenceEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QKeySequenceEdit11keySequenceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qkeysequenceedit.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setKeySequence(const QKeySequence &)

/*

*/
impl /*struct*/ QKeySequenceEdit {
  pub fn setKeySequence_0<RetType, T: QKeySequenceEdit_setKeySequence_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKeySequence_0(self);
    // return 1;
  }
}
pub trait QKeySequenceEdit_setKeySequence_0<RetType> {
  fn setKeySequence_0(self , rsthis: & QKeySequenceEdit) -> RetType;
}
impl<'a> /*trait*/ QKeySequenceEdit_setKeySequence_0<(/*void*/)> for (usize) {
  fn setKeySequence_0(self , rsthis: & QKeySequenceEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QKeySequenceEdit14setKeySequenceERK12QKeySequence", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qkeysequenceedit.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the current key sequence.
*/
impl /*struct*/ QKeySequenceEdit {
  pub fn clear_0<RetType, T: QKeySequenceEdit_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QKeySequenceEdit_clear_0<RetType> {
  fn clear_0(self , rsthis: & QKeySequenceEdit) -> RetType;
}
impl<'a> /*trait*/ QKeySequenceEdit_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QKeySequenceEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QKeySequenceEdit5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qkeysequenceedit.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void editingFinished()

/*
This signal is emitted when the user finishes entering the shortcut.

Note: there is a one second delay before releasing the last key and emitting this signal.
*/
impl /*struct*/ QKeySequenceEdit {
  pub fn editingFinished_0<RetType, T: QKeySequenceEdit_editingFinished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.editingFinished_0(self);
    // return 1;
  }
}
pub trait QKeySequenceEdit_editingFinished_0<RetType> {
  fn editingFinished_0(self , rsthis: & QKeySequenceEdit) -> RetType;
}
impl<'a> /*trait*/ QKeySequenceEdit_editingFinished_0<(/*void*/)> for () {
  fn editingFinished_0(self , rsthis: & QKeySequenceEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QKeySequenceEdit15editingFinishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qkeysequenceedit.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void keySequenceChanged(const QKeySequence &)

/*

*/
impl /*struct*/ QKeySequenceEdit {
  pub fn keySequenceChanged_0<RetType, T: QKeySequenceEdit_keySequenceChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keySequenceChanged_0(self);
    // return 1;
  }
}
pub trait QKeySequenceEdit_keySequenceChanged_0<RetType> {
  fn keySequenceChanged_0(self , rsthis: & QKeySequenceEdit) -> RetType;
}
impl<'a> /*trait*/ QKeySequenceEdit_keySequenceChanged_0<(/*void*/)> for (usize) {
  fn keySequenceChanged_0(self , rsthis: & QKeySequenceEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QKeySequenceEdit18keySequenceChangedERK12QKeySequence", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qkeysequenceedit.h:75
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QKeySequenceEdit {
  pub fn event_0<RetType, T: QKeySequenceEdit_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QKeySequenceEdit_event_0<RetType> {
  fn event_0(self , rsthis: & QKeySequenceEdit) -> RetType;
}
impl<'a> /*trait*/ QKeySequenceEdit_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QKeySequenceEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QKeySequenceEdit5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qkeysequenceedit.h:76
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QKeySequenceEdit {
  pub fn keyPressEvent_0<RetType, T: QKeySequenceEdit_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QKeySequenceEdit_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QKeySequenceEdit) -> RetType;
}
impl<'a> /*trait*/ QKeySequenceEdit_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QKeySequenceEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QKeySequenceEdit13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qkeysequenceedit.h:77
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyReleaseEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyReleaseEvent().
*/
impl /*struct*/ QKeySequenceEdit {
  pub fn keyReleaseEvent_0<RetType, T: QKeySequenceEdit_keyReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QKeySequenceEdit_keyReleaseEvent_0<RetType> {
  fn keyReleaseEvent_0(self , rsthis: & QKeySequenceEdit) -> RetType;
}
impl<'a> /*trait*/ QKeySequenceEdit_keyReleaseEvent_0<(/*void*/)> for (usize) {
  fn keyReleaseEvent_0(self , rsthis: & QKeySequenceEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QKeySequenceEdit15keyReleaseEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qkeysequenceedit.h:78
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QKeySequenceEdit {
  pub fn timerEvent_0<RetType, T: QKeySequenceEdit_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QKeySequenceEdit_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QKeySequenceEdit) -> RetType;
}
impl<'a> /*trait*/ QKeySequenceEdit_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QKeySequenceEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QKeySequenceEdit10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
