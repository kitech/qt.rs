

// mod ::widgets::QDateTimeEdit
// package qtwidgets
// /usr/include/qt/QtWidgets/qdatetimeedit.h
// #include <qdatetimeedit.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 30
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

// void keyPressEvent(QKeyEvent *)
// func (this *QDateTimeEdit) InheritKeyPressEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void wheelEvent(QWheelEvent *)
// func (this *QDateTimeEdit) InheritWheelEvent(f func(event *qtgui.QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QDateTimeEdit) InheritFocusInEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// bool focusNextPrevChild(bool)
// func (this *QDateTimeEdit) InheritFocusNextPrevChild(f func(next bool) bool) {
//  qtrt.SetAllInheritCallback(this, "focusNextPrevChild", f)
// }

// QValidator::State validate(QString &, int &)
// func (this *QDateTimeEdit) InheritValidate(f func(input string, pos int) int) {
//  qtrt.SetAllInheritCallback(this, "validate", f)
// }

// void fixup(QString &)
// func (this *QDateTimeEdit) InheritFixup(f func(input string) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "fixup", f)
// }

// QDateTime dateTimeFromText(const QString &)
// func (this *QDateTimeEdit) InheritDateTimeFromText(f func(text string) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "dateTimeFromText", f)
// }

// QString textFromDateTime(const QDateTime &)
// func (this *QDateTimeEdit) InheritTextFromDateTime(f func(dt *qtcore.QDateTime) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "textFromDateTime", f)
// }

// QAbstractSpinBox::StepEnabled stepEnabled()
// func (this *QDateTimeEdit) InheritStepEnabled(f func() int) {
//  qtrt.SetAllInheritCallback(this, "stepEnabled", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QDateTimeEdit) InheritMousePressEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QDateTimeEdit) InheritPaintEvent(f func(event *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void initStyleOption(QStyleOptionSpinBox *)
// func (this *QDateTimeEdit) InheritInitStyleOption(f func(option *QStyleOptionSpinBox/*777 QStyleOptionSpinBox **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QDateTimeEdit)=48
pub struct QDateTimeEdit {
  qbase: QAbstractSpinBox,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDateTimeEdit_ITF interface {
//    QAbstractSpinBox_ITF
//    QDateTimeEdit_PTR() *QDateTimeEdit
//}
//func (ptr *QDateTimeEdit) QDateTimeEdit_PTR() *QDateTimeEdit { return ptr }

impl /*struct*/ QDateTimeEdit {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDateTimeEdit {
    return QDateTimeEdit{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDateTimeEdit {
//  type Target = QDateTimeEditBASE;
//
//  fn deref(&self) -> &QDateTimeEditBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDateTimeEditBASE> for QDateTimeEdit {
//  fn as_ref(& self) -> & QDateTimeEditBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qdatetimeedit.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn metaObject_0<RetType, T: QDateTimeEdit_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDateTimeEdit(QWidget *)

/*
Constructs an empty date time editor with a parent.
*/
// QDateTimeEdit(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QDateTimeEdit {
  pub fn QDateTimeEdit_0<T: QDateTimeEdit_QDateTimeEdit_0>(value: T) -> QDateTimeEdit {
    let rsthis = value.QDateTimeEdit_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTimeEdit_QDateTimeEdit_0 {
  fn QDateTimeEdit_0(self) -> QDateTimeEdit;
}
// QDateTimeEdit(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDateTimeEdit_QDateTimeEdit_0 for (usize) {
  fn QDateTimeEdit_0(self) -> QDateTimeEdit {
    // unsafe{_ZN13QDateTimeEditC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QDateTimeEditC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDateTimeEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:96
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QDateTimeEdit(const QDateTime &, QWidget *)

/*
Constructs an empty date time editor with a parent.
*/
// QDateTimeEdit(const QDateTime &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QDateTimeEdit {
  pub fn QDateTimeEdit_1<T: QDateTimeEdit_QDateTimeEdit_1>(value: T) -> QDateTimeEdit {
    let rsthis = value.QDateTimeEdit_1();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTimeEdit_QDateTimeEdit_1 {
  fn QDateTimeEdit_1(self) -> QDateTimeEdit;
}
// QDateTimeEdit(const QDateTime &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDateTimeEdit_QDateTimeEdit_1 for (usize,usize) {
  fn QDateTimeEdit_1(self) -> QDateTimeEdit {
    // unsafe{_ZN13QDateTimeEditC2ERK9QDateTimeP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QDateTimeEditC2ERK9QDateTimeP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDateTimeEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:97
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QDateTimeEdit(const QDate &, QWidget *)

/*
Constructs an empty date time editor with a parent.
*/
// QDateTimeEdit(const QDate &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QDateTimeEdit {
  pub fn QDateTimeEdit_2<T: QDateTimeEdit_QDateTimeEdit_2>(value: T) -> QDateTimeEdit {
    let rsthis = value.QDateTimeEdit_2();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTimeEdit_QDateTimeEdit_2 {
  fn QDateTimeEdit_2(self) -> QDateTimeEdit;
}
// QDateTimeEdit(const QDate &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDateTimeEdit_QDateTimeEdit_2 for (usize,usize) {
  fn QDateTimeEdit_2(self) -> QDateTimeEdit {
    // unsafe{_ZN13QDateTimeEditC2ERK5QDateP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QDateTimeEditC2ERK5QDateP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDateTimeEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:98
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QDateTimeEdit(const QTime &, QWidget *)

/*
Constructs an empty date time editor with a parent.
*/
// QDateTimeEdit(const QTime &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QDateTimeEdit {
  pub fn QDateTimeEdit_3<T: QDateTimeEdit_QDateTimeEdit_3>(value: T) -> QDateTimeEdit {
    let rsthis = value.QDateTimeEdit_3();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTimeEdit_QDateTimeEdit_3 {
  fn QDateTimeEdit_3(self) -> QDateTimeEdit;
}
// QDateTimeEdit(const QTime &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDateTimeEdit_QDateTimeEdit_3 for (usize,usize) {
  fn QDateTimeEdit_3(self) -> QDateTimeEdit {
    // unsafe{_ZN13QDateTimeEditC2ERK5QTimeP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QDateTimeEditC2ERK5QTimeP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDateTimeEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:194
// index:4
// Protected Visibility=Default Availability=Available
// [-2] void QDateTimeEdit(const QVariant &, QVariant::Type, QWidget *)

/*
Constructs an empty date time editor with a parent.
*/
// QDateTimeEdit(const QVariant &, QVariant::Type, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QDateTimeEdit {
  pub fn QDateTimeEdit_4<T: QDateTimeEdit_QDateTimeEdit_4>(value: T) -> QDateTimeEdit {
    let rsthis = value.QDateTimeEdit_4();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTimeEdit_QDateTimeEdit_4 {
  fn QDateTimeEdit_4(self) -> QDateTimeEdit;
}
// QDateTimeEdit(const QVariant &, QVariant::Type, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDateTimeEdit_QDateTimeEdit_4 for (usize,i32,usize) {
  fn QDateTimeEdit_4(self) -> QDateTimeEdit {
    // unsafe{_ZN13QDateTimeEditC2ERK8QVariantNS0_4TypeEP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QDateTimeEditC2ERK8QVariantNS0_4TypeEP7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDateTimeEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:99
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDateTimeEdit()

/*

*/
pub fn DeleteQDateTimeEdit(this :*mut QDateTimeEdit) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QDateTimeEditD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qdatetimeedit.h:101
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime dateTime() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn dateTime_0<RetType, T: QDateTimeEdit_dateTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dateTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_dateTime_0<RetType> {
  fn dateTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_dateTime_0<usize> for () {
  fn dateTime_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit8dateTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:102
// index:0
// Public Visibility=Default Availability=Available
// [8] QDate date() const

/*
Returns the date of the date time edit.

Note: Getter function for property date. 

See also setDate().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn date_0<RetType, T: QDateTimeEdit_date_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.date_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_date_0<RetType> {
  fn date_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_date_0<usize> for () {
  fn date_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit4dateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:103
// index:0
// Public Visibility=Default Availability=Available
// [4] QTime time() const

/*
Returns the time of the date time edit.

Note: Getter function for property time. 

See also setTime().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn time_0<RetType, T: QDateTimeEdit_time_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.time_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_time_0<RetType> {
  fn time_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_time_0<usize> for () {
  fn time_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit4timeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime minimumDateTime() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn minimumDateTime_0<RetType, T: QDateTimeEdit_minimumDateTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumDateTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_minimumDateTime_0<RetType> {
  fn minimumDateTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_minimumDateTime_0<usize> for () {
  fn minimumDateTime_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit15minimumDateTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearMinimumDateTime()

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn clearMinimumDateTime_0<RetType, T: QDateTimeEdit_clearMinimumDateTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearMinimumDateTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_clearMinimumDateTime_0<RetType> {
  fn clearMinimumDateTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_clearMinimumDateTime_0<(/*void*/)> for () {
  fn clearMinimumDateTime_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit20clearMinimumDateTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumDateTime(const QDateTime &)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setMinimumDateTime_0<RetType, T: QDateTimeEdit_setMinimumDateTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumDateTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setMinimumDateTime_0<RetType> {
  fn setMinimumDateTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setMinimumDateTime_0<(/*void*/)> for (usize) {
  fn setMinimumDateTime_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit18setMinimumDateTimeERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:109
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime maximumDateTime() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn maximumDateTime_0<RetType, T: QDateTimeEdit_maximumDateTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumDateTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_maximumDateTime_0<RetType> {
  fn maximumDateTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_maximumDateTime_0<usize> for () {
  fn maximumDateTime_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit15maximumDateTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearMaximumDateTime()

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn clearMaximumDateTime_0<RetType, T: QDateTimeEdit_clearMaximumDateTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearMaximumDateTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_clearMaximumDateTime_0<RetType> {
  fn clearMaximumDateTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_clearMaximumDateTime_0<(/*void*/)> for () {
  fn clearMaximumDateTime_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit20clearMaximumDateTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumDateTime(const QDateTime &)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setMaximumDateTime_0<RetType, T: QDateTimeEdit_setMaximumDateTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumDateTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setMaximumDateTime_0<RetType> {
  fn setMaximumDateTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setMaximumDateTime_0<(/*void*/)> for (usize) {
  fn setMaximumDateTime_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit18setMaximumDateTimeERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDateTimeRange(const QDateTime &, const QDateTime &)

/*
Convenience function to set minimum and maximum date time with one function call.


  setDateTimeRange(min, max);



is analogous to:


  setMinimumDateTime(min);
  setMaximumDateTime(max);



If either min or max are not valid, this function does nothing.

This function was introduced in  Qt 4.4.

See also setMinimumDate(), maximumDate(), setMaximumDate(), clearMinimumDate(), setMinimumTime(), maximumTime(), setMaximumTime(), clearMinimumTime(), and QDateTime::isValid().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn setDateTimeRange_0<RetType, T: QDateTimeEdit_setDateTimeRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDateTimeRange_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setDateTimeRange_0<RetType> {
  fn setDateTimeRange_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setDateTimeRange_0<(/*void*/)> for (usize,usize) {
  fn setDateTimeRange_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit16setDateTimeRangeERK9QDateTimeS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:115
// index:0
// Public Visibility=Default Availability=Available
// [8] QDate minimumDate() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn minimumDate_0<RetType, T: QDateTimeEdit_minimumDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumDate_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_minimumDate_0<RetType> {
  fn minimumDate_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_minimumDate_0<usize> for () {
  fn minimumDate_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit11minimumDateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumDate(const QDate &)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setMinimumDate_0<RetType, T: QDateTimeEdit_setMinimumDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumDate_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setMinimumDate_0<RetType> {
  fn setMinimumDate_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setMinimumDate_0<(/*void*/)> for (usize) {
  fn setMinimumDate_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit14setMinimumDateERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearMinimumDate()

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn clearMinimumDate_0<RetType, T: QDateTimeEdit_clearMinimumDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearMinimumDate_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_clearMinimumDate_0<RetType> {
  fn clearMinimumDate_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_clearMinimumDate_0<(/*void*/)> for () {
  fn clearMinimumDate_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit16clearMinimumDateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:119
// index:0
// Public Visibility=Default Availability=Available
// [8] QDate maximumDate() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn maximumDate_0<RetType, T: QDateTimeEdit_maximumDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumDate_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_maximumDate_0<RetType> {
  fn maximumDate_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_maximumDate_0<usize> for () {
  fn maximumDate_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit11maximumDateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumDate(const QDate &)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setMaximumDate_0<RetType, T: QDateTimeEdit_setMaximumDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumDate_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setMaximumDate_0<RetType> {
  fn setMaximumDate_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setMaximumDate_0<(/*void*/)> for (usize) {
  fn setMaximumDate_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit14setMaximumDateERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearMaximumDate()

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn clearMaximumDate_0<RetType, T: QDateTimeEdit_clearMaximumDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearMaximumDate_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_clearMaximumDate_0<RetType> {
  fn clearMaximumDate_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_clearMaximumDate_0<(/*void*/)> for () {
  fn clearMaximumDate_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit16clearMaximumDateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDateRange(const QDate &, const QDate &)

/*
Convenience function to set minimum and maximum date with one function call.


  setDateRange(min, max);



is analogous to:


  setMinimumDate(min);
  setMaximumDate(max);



If either min or max are not valid, this function does nothing.

See also setMinimumDate(), maximumDate(), setMaximumDate(), clearMinimumDate(), setMinimumTime(), maximumTime(), setMaximumTime(), clearMinimumTime(), and QDate::isValid().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn setDateRange_0<RetType, T: QDateTimeEdit_setDateRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDateRange_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setDateRange_0<RetType> {
  fn setDateRange_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setDateRange_0<(/*void*/)> for (usize,usize) {
  fn setDateRange_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit12setDateRangeERK5QDateS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:125
// index:0
// Public Visibility=Default Availability=Available
// [4] QTime minimumTime() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn minimumTime_0<RetType, T: QDateTimeEdit_minimumTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_minimumTime_0<RetType> {
  fn minimumTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_minimumTime_0<usize> for () {
  fn minimumTime_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit11minimumTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumTime(const QTime &)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setMinimumTime_0<RetType, T: QDateTimeEdit_setMinimumTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setMinimumTime_0<RetType> {
  fn setMinimumTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setMinimumTime_0<(/*void*/)> for (usize) {
  fn setMinimumTime_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit14setMinimumTimeERK5QTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearMinimumTime()

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn clearMinimumTime_0<RetType, T: QDateTimeEdit_clearMinimumTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearMinimumTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_clearMinimumTime_0<RetType> {
  fn clearMinimumTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_clearMinimumTime_0<(/*void*/)> for () {
  fn clearMinimumTime_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit16clearMinimumTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:129
// index:0
// Public Visibility=Default Availability=Available
// [4] QTime maximumTime() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn maximumTime_0<RetType, T: QDateTimeEdit_maximumTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_maximumTime_0<RetType> {
  fn maximumTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_maximumTime_0<usize> for () {
  fn maximumTime_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit11maximumTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumTime(const QTime &)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setMaximumTime_0<RetType, T: QDateTimeEdit_setMaximumTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setMaximumTime_0<RetType> {
  fn setMaximumTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setMaximumTime_0<(/*void*/)> for (usize) {
  fn setMaximumTime_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit14setMaximumTimeERK5QTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:131
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearMaximumTime()

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn clearMaximumTime_0<RetType, T: QDateTimeEdit_clearMaximumTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearMaximumTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_clearMaximumTime_0<RetType> {
  fn clearMaximumTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_clearMaximumTime_0<(/*void*/)> for () {
  fn clearMaximumTime_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit16clearMaximumTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTimeRange(const QTime &, const QTime &)

/*
Convenience function to set minimum and maximum time with one function call.


  setTimeRange(min, max);



is analogous to:


  setMinimumTime(min);
  setMaximumTime(max);



If either min or max are not valid, this function does nothing.

See also setMinimumDate(), maximumDate(), setMaximumDate(), clearMinimumDate(), setMinimumTime(), maximumTime(), setMaximumTime(), clearMinimumTime(), and QTime::isValid().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn setTimeRange_0<RetType, T: QDateTimeEdit_setTimeRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTimeRange_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setTimeRange_0<RetType> {
  fn setTimeRange_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setTimeRange_0<(/*void*/)> for (usize,usize) {
  fn setTimeRange_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit12setTimeRangeERK5QTimeS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:135
// index:0
// Public Visibility=Default Availability=Available
// [4] QDateTimeEdit::Sections displayedSections() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn displayedSections_0<RetType, T: QDateTimeEdit_displayedSections_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.displayedSections_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_displayedSections_0<RetType> {
  fn displayedSections_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_displayedSections_0<i32> for () {
  fn displayedSections_0(self , rsthis: & QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit17displayedSectionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:136
// index:0
// Public Visibility=Default Availability=Available
// [4] QDateTimeEdit::Section currentSection() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn currentSection_0<RetType, T: QDateTimeEdit_currentSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentSection_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_currentSection_0<RetType> {
  fn currentSection_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_currentSection_0<i32> for () {
  fn currentSection_0(self , rsthis: & QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit14currentSectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:137
// index:0
// Public Visibility=Default Availability=Available
// [4] QDateTimeEdit::Section sectionAt(int) const

/*
Returns the Section at index.

If the format is 'yyyy/MM/dd', sectionAt(0) returns YearSection, sectionAt(1) returns MonthSection, and sectionAt(2) returns YearSection,

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QDateTimeEdit {
  pub fn sectionAt_0<RetType, T: QDateTimeEdit_sectionAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionAt_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_sectionAt_0<RetType> {
  fn sectionAt_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_sectionAt_0<i32> for (i32) {
  fn sectionAt_0(self , rsthis: & QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit9sectionAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:138
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentSection(QDateTimeEdit::Section)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setCurrentSection_0<RetType, T: QDateTimeEdit_setCurrentSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentSection_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setCurrentSection_0<RetType> {
  fn setCurrentSection_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setCurrentSection_0<(/*void*/)> for (i32) {
  fn setCurrentSection_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit17setCurrentSectionENS_7SectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:140
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentSectionIndex() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn currentSectionIndex_0<RetType, T: QDateTimeEdit_currentSectionIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentSectionIndex_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_currentSectionIndex_0<RetType> {
  fn currentSectionIndex_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_currentSectionIndex_0<i32> for () {
  fn currentSectionIndex_0(self , rsthis: & QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit19currentSectionIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:141
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentSectionIndex(int)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setCurrentSectionIndex_0<RetType, T: QDateTimeEdit_setCurrentSectionIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentSectionIndex_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setCurrentSectionIndex_0<RetType> {
  fn setCurrentSectionIndex_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setCurrentSectionIndex_0<(/*void*/)> for (i32) {
  fn setCurrentSectionIndex_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit22setCurrentSectionIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:143
// index:0
// Public Visibility=Default Availability=Available
// [8] QCalendarWidget * calendarWidget() const

/*
Returns the calendar widget for the editor if calendarPopup is set to true and (sections() & DateSections_Mask) != 0.

This function creates and returns a calendar widget if none has been set.

This function was introduced in  Qt 4.4.

See also setCalendarWidget().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn calendarWidget_0<RetType, T: QDateTimeEdit_calendarWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.calendarWidget_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_calendarWidget_0<RetType> {
  fn calendarWidget_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_calendarWidget_0<usize> for () {
  fn calendarWidget_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit14calendarWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCalendarWidget(QCalendarWidget *)

/*
Sets the given calendarWidget as the widget to be used for the calendar pop-up. The editor does not automatically take ownership of the calendar widget.

Note: calendarPopup must be set to true before setting the calendar widget.

This function was introduced in  Qt 4.4.

See also calendarWidget() and calendarPopup.
*/
impl /*struct*/ QDateTimeEdit {
  pub fn setCalendarWidget_0<RetType, T: QDateTimeEdit_setCalendarWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCalendarWidget_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setCalendarWidget_0<RetType> {
  fn setCalendarWidget_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setCalendarWidget_0<(/*void*/)> for (usize) {
  fn setCalendarWidget_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit17setCalendarWidgetEP15QCalendarWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:146
// index:0
// Public Visibility=Default Availability=Available
// [4] int sectionCount() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn sectionCount_0<RetType, T: QDateTimeEdit_sectionCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionCount_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_sectionCount_0<RetType> {
  fn sectionCount_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_sectionCount_0<i32> for () {
  fn sectionCount_0(self , rsthis: & QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit12sectionCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSelectedSection(QDateTimeEdit::Section)

/*
Selects section. If section doesn't exist in the currently displayed sections, this function does nothing. If section is NoSection, this function will unselect all text in the editor. Otherwise, this function will move the cursor and the current section to the selected section.

This function was introduced in  Qt 4.2.

See also currentSection().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn setSelectedSection_0<RetType, T: QDateTimeEdit_setSelectedSection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectedSection_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setSelectedSection_0<RetType> {
  fn setSelectedSection_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setSelectedSection_0<(/*void*/)> for (i32) {
  fn setSelectedSection_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit18setSelectedSectionENS_7SectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:150
// index:0
// Public Visibility=Default Availability=Available
// [8] QString sectionText(QDateTimeEdit::Section) const

/*
Returns the text from the given section.

See also currentSection().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn sectionText_0<RetType, T: QDateTimeEdit_sectionText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sectionText_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_sectionText_0<RetType> {
  fn sectionText_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_sectionText_0<usize> for (i32) {
  fn sectionText_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit11sectionTextENS_7SectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:152
// index:0
// Public Visibility=Default Availability=Available
// [8] QString displayFormat() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn displayFormat_0<RetType, T: QDateTimeEdit_displayFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.displayFormat_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_displayFormat_0<RetType> {
  fn displayFormat_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_displayFormat_0<usize> for () {
  fn displayFormat_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit13displayFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:153
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDisplayFormat(const QString &)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setDisplayFormat_0<RetType, T: QDateTimeEdit_setDisplayFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDisplayFormat_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setDisplayFormat_0<RetType> {
  fn setDisplayFormat_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setDisplayFormat_0<(/*void*/)> for (usize) {
  fn setDisplayFormat_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit16setDisplayFormatERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:155
// index:0
// Public Visibility=Default Availability=Available
// [1] bool calendarPopup() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn calendarPopup_0<RetType, T: QDateTimeEdit_calendarPopup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.calendarPopup_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_calendarPopup_0<RetType> {
  fn calendarPopup_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_calendarPopup_0<bool> for () {
  fn calendarPopup_0(self , rsthis: & QDateTimeEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit13calendarPopupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCalendarPopup(bool)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setCalendarPopup_0<RetType, T: QDateTimeEdit_setCalendarPopup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCalendarPopup_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setCalendarPopup_0<RetType> {
  fn setCalendarPopup_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setCalendarPopup_0<(/*void*/)> for (bool) {
  fn setCalendarPopup_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit16setCalendarPopupEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:158
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TimeSpec timeSpec() const

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn timeSpec_0<RetType, T: QDateTimeEdit_timeSpec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timeSpec_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_timeSpec_0<RetType> {
  fn timeSpec_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_timeSpec_0<i32> for () {
  fn timeSpec_0(self , rsthis: & QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit8timeSpecEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:159
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTimeSpec(Qt::TimeSpec)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setTimeSpec_0<RetType, T: QDateTimeEdit_setTimeSpec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTimeSpec_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setTimeSpec_0<RetType> {
  fn setTimeSpec_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setTimeSpec_0<(/*void*/)> for (i32) {
  fn setTimeSpec_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit11setTimeSpecEN2Qt8TimeSpecE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:161
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn sizeHint_0<RetType, T: QDateTimeEdit_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:163
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void clear()

/*
Reimplemented from QAbstractSpinBox::clear().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn clear_0<RetType, T: QDateTimeEdit_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_clear_0<RetType> {
  fn clear_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:164
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void stepBy(int)

/*
Reimplemented from QAbstractSpinBox::stepBy().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn stepBy_0<RetType, T: QDateTimeEdit_stepBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stepBy_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_stepBy_0<RetType> {
  fn stepBy_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_stepBy_0<(/*void*/)> for (i32) {
  fn stepBy_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit6stepByEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:166
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn event_0<RetType, T: QDateTimeEdit_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_event_0<RetType> {
  fn event_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QDateTimeEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:168
// index:0
// Public Visibility=Default Availability=Available
// [-2] void dateTimeChanged(const QDateTime &)

/*
This signal is emitted whenever the date or time is changed. The new date and time is passed in datetime.

Note: Notifier signal for property dateTime.
*/
impl /*struct*/ QDateTimeEdit {
  pub fn dateTimeChanged_0<RetType, T: QDateTimeEdit_dateTimeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dateTimeChanged_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_dateTimeChanged_0<RetType> {
  fn dateTimeChanged_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_dateTimeChanged_0<(/*void*/)> for (usize) {
  fn dateTimeChanged_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit15dateTimeChangedERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:169
// index:0
// Public Visibility=Default Availability=Available
// [-2] void timeChanged(const QTime &)

/*
This signal is emitted whenever the time is changed. The new time is passed in time.

Note: Notifier signal for property time.
*/
impl /*struct*/ QDateTimeEdit {
  pub fn timeChanged_0<RetType, T: QDateTimeEdit_timeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timeChanged_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_timeChanged_0<RetType> {
  fn timeChanged_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_timeChanged_0<(/*void*/)> for (usize) {
  fn timeChanged_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit11timeChangedERK5QTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:170
// index:0
// Public Visibility=Default Availability=Available
// [-2] void dateChanged(const QDate &)

/*
This signal is emitted whenever the date is changed. The new date is passed in date.

Note: Notifier signal for property date.
*/
impl /*struct*/ QDateTimeEdit {
  pub fn dateChanged_0<RetType, T: QDateTimeEdit_dateChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dateChanged_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_dateChanged_0<RetType> {
  fn dateChanged_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_dateChanged_0<(/*void*/)> for (usize) {
  fn dateChanged_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit11dateChangedERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:173
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDateTime(const QDateTime &)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setDateTime_0<RetType, T: QDateTimeEdit_setDateTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDateTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setDateTime_0<RetType> {
  fn setDateTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setDateTime_0<(/*void*/)> for (usize) {
  fn setDateTime_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit11setDateTimeERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:174
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDate(const QDate &)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setDate_0<RetType, T: QDateTimeEdit_setDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDate_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setDate_0<RetType> {
  fn setDate_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setDate_0<(/*void*/)> for (usize) {
  fn setDate_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit7setDateERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:175
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTime(const QTime &)

/*

*/
impl /*struct*/ QDateTimeEdit {
  pub fn setTime_0<RetType, T: QDateTimeEdit_setTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_setTime_0<RetType> {
  fn setTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_setTime_0<(/*void*/)> for (usize) {
  fn setTime_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit7setTimeERK5QTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:178
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn keyPressEvent_0<RetType, T: QDateTimeEdit_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:180
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
Reimplemented from QWidget::wheelEvent().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn wheelEvent_0<RetType, T: QDateTimeEdit_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:182
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn focusInEvent_0<RetType, T: QDateTimeEdit_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:183
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool focusNextPrevChild(bool)

/*
Reimplemented from QWidget::focusNextPrevChild().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn focusNextPrevChild_0<RetType, T: QDateTimeEdit_focusNextPrevChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_focusNextPrevChild_0<RetType> {
  fn focusNextPrevChild_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_focusNextPrevChild_0<bool> for (bool) {
  fn focusNextPrevChild_0(self , rsthis: & QDateTimeEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit18focusNextPrevChildEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:184
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] QValidator::State validate(QString &, int &) const

/*
Reimplemented from QAbstractSpinBox::validate().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn validate_0<RetType, T: QDateTimeEdit_validate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.validate_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_validate_0<RetType> {
  fn validate_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_validate_0<i32> for (usize,usize) {
  fn validate_0(self , rsthis: & QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit8validateER7QStringRi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:185
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void fixup(QString &) const

/*
Reimplemented from QAbstractSpinBox::fixup().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn fixup_0<RetType, T: QDateTimeEdit_fixup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fixup_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_fixup_0<RetType> {
  fn fixup_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_fixup_0<(/*void*/)> for (usize) {
  fn fixup_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit5fixupER7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:187
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QDateTime dateTimeFromText(const QString &) const

/*
Returns an appropriate datetime for the given text.

This virtual function is used by the datetime edit whenever it needs to interpret text entered by the user as a value.

See also textFromDateTime() and validate().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn dateTimeFromText_0<RetType, T: QDateTimeEdit_dateTimeFromText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dateTimeFromText_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_dateTimeFromText_0<RetType> {
  fn dateTimeFromText_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_dateTimeFromText_0<usize> for (usize) {
  fn dateTimeFromText_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit16dateTimeFromTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:188
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QString textFromDateTime(const QDateTime &) const

/*
This virtual function is used by the date time edit whenever it needs to display dateTime.

If you reimplement this, you may also need to reimplement validate().

See also dateTimeFromText() and validate().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn textFromDateTime_0<RetType, T: QDateTimeEdit_textFromDateTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textFromDateTime_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_textFromDateTime_0<RetType> {
  fn textFromDateTime_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_textFromDateTime_0<usize> for (usize) {
  fn textFromDateTime_0(self , rsthis: & QDateTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit16textFromDateTimeERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:189
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] QAbstractSpinBox::StepEnabled stepEnabled() const

/*
Reimplemented from QAbstractSpinBox::stepEnabled().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn stepEnabled_0<RetType, T: QDateTimeEdit_stepEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stepEnabled_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_stepEnabled_0<RetType> {
  fn stepEnabled_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_stepEnabled_0<i32> for () {
  fn stepEnabled_0(self , rsthis: & QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit11stepEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:190
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn mousePressEvent_0<RetType, T: QDateTimeEdit_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:191
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn paintEvent_0<RetType, T: QDateTimeEdit_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QDateTimeEdit10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:192
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionSpinBox *) const

/*
Initialize option with the values from this QDataTimeEdit. This method is useful for subclasses when they need a QStyleOptionSpinBox, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QDateTimeEdit {
  pub fn initStyleOption_0<RetType, T: QDateTimeEdit_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QDateTimeEdit_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QDateTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QDateTimeEdit_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QDateTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK13QDateTimeEdit15initStyleOptionEP19QStyleOptionSpinBox", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QDateTimeEdit__Section = i32;
// 
pub const QDateTimeEdit__NoSection :QDateTimeEdit__Section = 0;
// 
pub const QDateTimeEdit__AmPmSection :QDateTimeEdit__Section = 1;
// 
pub const QDateTimeEdit__MSecSection :QDateTimeEdit__Section = 2;
// 
pub const QDateTimeEdit__SecondSection :QDateTimeEdit__Section = 4;
// 
pub const QDateTimeEdit__MinuteSection :QDateTimeEdit__Section = 8;
// 
pub const QDateTimeEdit__HourSection :QDateTimeEdit__Section = 16;
// 
pub const QDateTimeEdit__DaySection :QDateTimeEdit__Section = 256;
// 
pub const QDateTimeEdit__MonthSection :QDateTimeEdit__Section = 512;
// 
pub const QDateTimeEdit__YearSection :QDateTimeEdit__Section = 1024;
// 
pub const QDateTimeEdit__TimeSections_Mask :QDateTimeEdit__Section = 31;
// 
pub const QDateTimeEdit__DateSections_Mask :QDateTimeEdit__Section = 1792;
pub fn QDateTimeEdit_SectionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QDateTimeEdit", val);
}
pub fn QDateTimeEdit_SectionItemName_s(val: i32) ->String {
  //var nilthis *QDateTimeEdit
  //return nilthis.SectionItemName(val);
  return QDateTimeEdit_SectionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
