

// mod ::widgets::QCompleter
// package qtwidgets
// /usr/include/qt/QtWidgets/qcompleter.h
// #include <qcompleter.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 21
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

// bool eventFilter(QObject *, QEvent *)
// func (this *QCompleter) InheritEventFilter(f func(o *qtcore.QObject/*777 QObject **/, e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// bool event(QEvent *)
// func (this *QCompleter) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QCompleter)=16
pub struct QCompleter {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCompleter_ITF interface {
//    qtcore.QObject_ITF
//    QCompleter_PTR() *QCompleter
//}
//func (ptr *QCompleter) QCompleter_PTR() *QCompleter { return ptr }

impl /*struct*/ QCompleter {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCompleter {
    return QCompleter{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCompleter {
//  type Target = QCompleterBASE;
//
//  fn deref(&self) -> &QCompleterBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCompleterBASE> for QCompleter {
//  fn as_ref(& self) -> & QCompleterBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qcompleter.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QCompleter {
  pub fn metaObject_0<RetType, T: QCompleter_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QCompleter_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QCompleter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QCompleter(QObject *)

/*
Constructs a completer object with the given parent.
*/
// QCompleter(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QCompleter {
  pub fn QCompleter_0<T: QCompleter_QCompleter_0>(value: T) -> QCompleter {
    let rsthis = value.QCompleter_0();
    return rsthis;
    // return 1;
  }
}

pub trait QCompleter_QCompleter_0 {
  fn QCompleter_0(self) -> QCompleter;
}
// QCompleter(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCompleter_QCompleter_0 for (usize) {
  fn QCompleter_0(self) -> QCompleter {
    // unsafe{_ZN10QCompleterC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QCompleterC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCompleter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:86
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QCompleter(QAbstractItemModel *, QObject *)

/*
Constructs a completer object with the given parent.
*/
// QCompleter(QAbstractItemModel *, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QCompleter {
  pub fn QCompleter_1<T: QCompleter_QCompleter_1>(value: T) -> QCompleter {
    let rsthis = value.QCompleter_1();
    return rsthis;
    // return 1;
  }
}

pub trait QCompleter_QCompleter_1 {
  fn QCompleter_1(self) -> QCompleter;
}
// QCompleter(QAbstractItemModel *, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCompleter_QCompleter_1 for (usize,usize) {
  fn QCompleter_1(self) -> QCompleter {
    // unsafe{_ZN10QCompleterC2EP18QAbstractItemModelP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QCompleterC2EP18QAbstractItemModelP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCompleter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:88
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QCompleter(const QStringList &, QObject *)

/*
Constructs a completer object with the given parent.
*/
// QCompleter(const QStringList &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QCompleter {
  pub fn QCompleter_2<T: QCompleter_QCompleter_2>(value: T) -> QCompleter {
    let rsthis = value.QCompleter_2();
    return rsthis;
    // return 1;
  }
}

pub trait QCompleter_QCompleter_2 {
  fn QCompleter_2(self) -> QCompleter;
}
// QCompleter(const QStringList &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCompleter_QCompleter_2 for (usize,usize) {
  fn QCompleter_2(self) -> QCompleter {
    // unsafe{_ZN10QCompleterC2ERK11QStringListP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QCompleterC2ERK11QStringListP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCompleter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:90
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QCompleter()

/*

*/
pub fn DeleteQCompleter(this :*mut QCompleter) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QCompleterD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qcompleter.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidget(QWidget *)

/*
Sets the widget for which completion are provided for to widget. This function is automatically called when a QCompleter is set on a QLineEdit using QLineEdit::setCompleter() or on a QComboBox using QComboBox::setCompleter(). The widget needs to be set explicitly when providing completions for custom widgets.

See also widget(), setModel(), and setPopup().
*/
impl /*struct*/ QCompleter {
  pub fn setWidget_0<RetType, T: QCompleter_setWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidget_0(self);
    // return 1;
  }
}
pub trait QCompleter_setWidget_0<RetType> {
  fn setWidget_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_setWidget_0<(/*void*/)> for (usize) {
  fn setWidget_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter9setWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:93
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widget() const

/*
Returns the widget for which the completer object is providing completions.

See also setWidget().
*/
impl /*struct*/ QCompleter {
  pub fn widget_0<RetType, T: QCompleter_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QCompleter_widget_0<RetType> {
  fn widget_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_widget_0<usize> for () {
  fn widget_0(self , rsthis: & QCompleter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter6widgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModel(QAbstractItemModel *)

/*
Sets the model which provides completions to model. The model can be list model or a tree model. If a model has been already previously set and it has the QCompleter as its parent, it is deleted.

For convenience, if model is a QFileSystemModel, QCompleter switches its caseSensitivity to Qt::CaseInsensitive on Windows and Qt::CaseSensitive on other platforms.

See also completionModel(), modelSorting, and Handling Tree Models.
*/
impl /*struct*/ QCompleter {
  pub fn setModel_0<RetType, T: QCompleter_setModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModel_0(self);
    // return 1;
  }
}
pub trait QCompleter_setModel_0<RetType> {
  fn setModel_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_setModel_0<(/*void*/)> for (usize) {
  fn setModel_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter8setModelEP18QAbstractItemModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:96
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemModel * model() const

/*
Returns the model that provides completion strings.

See also setModel() and completionModel().
*/
impl /*struct*/ QCompleter {
  pub fn model_0<RetType, T: QCompleter_model_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.model_0(self);
    // return 1;
  }
}
pub trait QCompleter_model_0<RetType> {
  fn model_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_model_0<usize> for () {
  fn model_0(self , rsthis: & QCompleter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter5modelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCompletionMode(QCompleter::CompletionMode)

/*

*/
impl /*struct*/ QCompleter {
  pub fn setCompletionMode_0<RetType, T: QCompleter_setCompletionMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCompletionMode_0(self);
    // return 1;
  }
}
pub trait QCompleter_setCompletionMode_0<RetType> {
  fn setCompletionMode_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_setCompletionMode_0<(/*void*/)> for (i32) {
  fn setCompletionMode_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter17setCompletionModeENS_14CompletionModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] QCompleter::CompletionMode completionMode() const

/*

*/
impl /*struct*/ QCompleter {
  pub fn completionMode_0<RetType, T: QCompleter_completionMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.completionMode_0(self);
    // return 1;
  }
}
pub trait QCompleter_completionMode_0<RetType> {
  fn completionMode_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_completionMode_0<i32> for () {
  fn completionMode_0(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter14completionModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFilterMode(Qt::MatchFlags)

/*

*/
impl /*struct*/ QCompleter {
  pub fn setFilterMode_0<RetType, T: QCompleter_setFilterMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilterMode_0(self);
    // return 1;
  }
}
pub trait QCompleter_setFilterMode_0<RetType> {
  fn setFilterMode_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_setFilterMode_0<(/*void*/)> for (i32) {
  fn setFilterMode_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter13setFilterModeE6QFlagsIN2Qt9MatchFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:102
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::MatchFlags filterMode() const

/*

*/
impl /*struct*/ QCompleter {
  pub fn filterMode_0<RetType, T: QCompleter_filterMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filterMode_0(self);
    // return 1;
  }
}
pub trait QCompleter_filterMode_0<RetType> {
  fn filterMode_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_filterMode_0<i32> for () {
  fn filterMode_0(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter10filterModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:104
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemView * popup() const

/*
Returns the popup used to display completions.

See also setPopup().
*/
impl /*struct*/ QCompleter {
  pub fn popup_0<RetType, T: QCompleter_popup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.popup_0(self);
    // return 1;
  }
}
pub trait QCompleter_popup_0<RetType> {
  fn popup_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_popup_0<usize> for () {
  fn popup_0(self , rsthis: & QCompleter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter5popupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPopup(QAbstractItemView *)

/*
Sets the popup used to display completions to popup. QCompleter takes ownership of the view.

A QListView is automatically created when the completionMode() is set to QCompleter::PopupCompletion or QCompleter::UnfilteredPopupCompletion. The default popup displays the completionColumn().

Ensure that this function is called before the view settings are modified. This is required since view's properties may require that a model has been set on the view (for example, hiding columns in the view requires a model to be set on the view).

See also popup().
*/
impl /*struct*/ QCompleter {
  pub fn setPopup_0<RetType, T: QCompleter_setPopup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPopup_0(self);
    // return 1;
  }
}
pub trait QCompleter_setPopup_0<RetType> {
  fn setPopup_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_setPopup_0<(/*void*/)> for (usize) {
  fn setPopup_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter8setPopupEP17QAbstractItemView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCaseSensitivity(Qt::CaseSensitivity)

/*

*/
impl /*struct*/ QCompleter {
  pub fn setCaseSensitivity_0<RetType, T: QCompleter_setCaseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCaseSensitivity_0(self);
    // return 1;
  }
}
pub trait QCompleter_setCaseSensitivity_0<RetType> {
  fn setCaseSensitivity_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_setCaseSensitivity_0<(/*void*/)> for (i32) {
  fn setCaseSensitivity_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter18setCaseSensitivityEN2Qt15CaseSensitivityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:108
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::CaseSensitivity caseSensitivity() const

/*

*/
impl /*struct*/ QCompleter {
  pub fn caseSensitivity_0<RetType, T: QCompleter_caseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.caseSensitivity_0(self);
    // return 1;
  }
}
pub trait QCompleter_caseSensitivity_0<RetType> {
  fn caseSensitivity_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_caseSensitivity_0<i32> for () {
  fn caseSensitivity_0(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter15caseSensitivityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModelSorting(QCompleter::ModelSorting)

/*

*/
impl /*struct*/ QCompleter {
  pub fn setModelSorting_0<RetType, T: QCompleter_setModelSorting_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModelSorting_0(self);
    // return 1;
  }
}
pub trait QCompleter_setModelSorting_0<RetType> {
  fn setModelSorting_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_setModelSorting_0<(/*void*/)> for (i32) {
  fn setModelSorting_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter15setModelSortingENS_12ModelSortingE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:111
// index:0
// Public Visibility=Default Availability=Available
// [4] QCompleter::ModelSorting modelSorting() const

/*

*/
impl /*struct*/ QCompleter {
  pub fn modelSorting_0<RetType, T: QCompleter_modelSorting_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modelSorting_0(self);
    // return 1;
  }
}
pub trait QCompleter_modelSorting_0<RetType> {
  fn modelSorting_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_modelSorting_0<i32> for () {
  fn modelSorting_0(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter12modelSortingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCompletionColumn(int)

/*

*/
impl /*struct*/ QCompleter {
  pub fn setCompletionColumn_0<RetType, T: QCompleter_setCompletionColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCompletionColumn_0(self);
    // return 1;
  }
}
pub trait QCompleter_setCompletionColumn_0<RetType> {
  fn setCompletionColumn_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_setCompletionColumn_0<(/*void*/)> for (i32) {
  fn setCompletionColumn_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter19setCompletionColumnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:114
// index:0
// Public Visibility=Default Availability=Available
// [4] int completionColumn() const

/*

*/
impl /*struct*/ QCompleter {
  pub fn completionColumn_0<RetType, T: QCompleter_completionColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.completionColumn_0(self);
    // return 1;
  }
}
pub trait QCompleter_completionColumn_0<RetType> {
  fn completionColumn_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_completionColumn_0<i32> for () {
  fn completionColumn_0(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter16completionColumnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCompletionRole(int)

/*

*/
impl /*struct*/ QCompleter {
  pub fn setCompletionRole_0<RetType, T: QCompleter_setCompletionRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCompletionRole_0(self);
    // return 1;
  }
}
pub trait QCompleter_setCompletionRole_0<RetType> {
  fn setCompletionRole_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_setCompletionRole_0<(/*void*/)> for (i32) {
  fn setCompletionRole_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter17setCompletionRoleEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:117
// index:0
// Public Visibility=Default Availability=Available
// [4] int completionRole() const

/*

*/
impl /*struct*/ QCompleter {
  pub fn completionRole_0<RetType, T: QCompleter_completionRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.completionRole_0(self);
    // return 1;
  }
}
pub trait QCompleter_completionRole_0<RetType> {
  fn completionRole_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_completionRole_0<i32> for () {
  fn completionRole_0(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter14completionRoleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:119
// index:0
// Public Visibility=Default Availability=Available
// [1] bool wrapAround() const

/*

*/
impl /*struct*/ QCompleter {
  pub fn wrapAround_0<RetType, T: QCompleter_wrapAround_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wrapAround_0(self);
    // return 1;
  }
}
pub trait QCompleter_wrapAround_0<RetType> {
  fn wrapAround_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_wrapAround_0<bool> for () {
  fn wrapAround_0(self , rsthis: & QCompleter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter10wrapAroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:121
// index:0
// Public Visibility=Default Availability=Available
// [4] int maxVisibleItems() const

/*

*/
impl /*struct*/ QCompleter {
  pub fn maxVisibleItems_0<RetType, T: QCompleter_maxVisibleItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maxVisibleItems_0(self);
    // return 1;
  }
}
pub trait QCompleter_maxVisibleItems_0<RetType> {
  fn maxVisibleItems_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_maxVisibleItems_0<i32> for () {
  fn maxVisibleItems_0(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter15maxVisibleItemsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:122
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaxVisibleItems(int)

/*

*/
impl /*struct*/ QCompleter {
  pub fn setMaxVisibleItems_0<RetType, T: QCompleter_setMaxVisibleItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaxVisibleItems_0(self);
    // return 1;
  }
}
pub trait QCompleter_setMaxVisibleItems_0<RetType> {
  fn setMaxVisibleItems_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_setMaxVisibleItems_0<(/*void*/)> for (i32) {
  fn setMaxVisibleItems_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter18setMaxVisibleItemsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:124
// index:0
// Public Visibility=Default Availability=Available
// [4] int completionCount() const

/*
Returns the number of completions for the current prefix. For an unsorted model with a large number of items this can be expensive. Use setCurrentRow() and currentCompletion() to iterate through all the completions.
*/
impl /*struct*/ QCompleter {
  pub fn completionCount_0<RetType, T: QCompleter_completionCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.completionCount_0(self);
    // return 1;
  }
}
pub trait QCompleter_completionCount_0<RetType> {
  fn completionCount_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_completionCount_0<i32> for () {
  fn completionCount_0(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter15completionCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:125
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setCurrentRow(int)

/*
Sets the current row to the row specified. Returns true if successful; otherwise returns false.

This function may be used along with currentCompletion() to iterate through all the possible completions.

See also currentRow(), currentCompletion(), and completionCount().
*/
impl /*struct*/ QCompleter {
  pub fn setCurrentRow_0<RetType, T: QCompleter_setCurrentRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentRow_0(self);
    // return 1;
  }
}
pub trait QCompleter_setCurrentRow_0<RetType> {
  fn setCurrentRow_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_setCurrentRow_0<bool> for (i32) {
  fn setCurrentRow_0(self , rsthis: & QCompleter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QCompleter13setCurrentRowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:126
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentRow() const

/*
Returns the current row.

See also setCurrentRow().
*/
impl /*struct*/ QCompleter {
  pub fn currentRow_0<RetType, T: QCompleter_currentRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentRow_0(self);
    // return 1;
  }
}
pub trait QCompleter_currentRow_0<RetType> {
  fn currentRow_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_currentRow_0<i32> for () {
  fn currentRow_0(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter10currentRowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:128
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex currentIndex() const

/*
Returns the model index of the current completion in the completionModel().

See also setCurrentRow(), currentCompletion(), and model().
*/
impl /*struct*/ QCompleter {
  pub fn currentIndex_0<RetType, T: QCompleter_currentIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentIndex_0(self);
    // return 1;
  }
}
pub trait QCompleter_currentIndex_0<RetType> {
  fn currentIndex_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_currentIndex_0<usize> for () {
  fn currentIndex_0(self , rsthis: & QCompleter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter12currentIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:129
// index:0
// Public Visibility=Default Availability=Available
// [8] QString currentCompletion() const

/*
Returns the current completion string. This includes the completionPrefix. When used alongside setCurrentRow(), it can be used to iterate through all the matches.

See also setCurrentRow() and currentIndex().
*/
impl /*struct*/ QCompleter {
  pub fn currentCompletion_0<RetType, T: QCompleter_currentCompletion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentCompletion_0(self);
    // return 1;
  }
}
pub trait QCompleter_currentCompletion_0<RetType> {
  fn currentCompletion_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_currentCompletion_0<usize> for () {
  fn currentCompletion_0(self , rsthis: & QCompleter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter17currentCompletionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:131
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemModel * completionModel() const

/*
Returns the completion model. The completion model is a read-only list model that contains all the possible matches for the current completion prefix. The completion model is auto-updated to reflect the current completions.

Note: The return value of this function is defined to be an QAbstractItemModel purely for generality. This actual kind of model returned is an instance of an QAbstractProxyModel subclass.

See also completionPrefix and model().
*/
impl /*struct*/ QCompleter {
  pub fn completionModel_0<RetType, T: QCompleter_completionModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.completionModel_0(self);
    // return 1;
  }
}
pub trait QCompleter_completionModel_0<RetType> {
  fn completionModel_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_completionModel_0<usize> for () {
  fn completionModel_0(self , rsthis: & QCompleter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter15completionModelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:133
// index:0
// Public Visibility=Default Availability=Available
// [8] QString completionPrefix() const

/*

*/
impl /*struct*/ QCompleter {
  pub fn completionPrefix_0<RetType, T: QCompleter_completionPrefix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.completionPrefix_0(self);
    // return 1;
  }
}
pub trait QCompleter_completionPrefix_0<RetType> {
  fn completionPrefix_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_completionPrefix_0<usize> for () {
  fn completionPrefix_0(self , rsthis: & QCompleter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter16completionPrefixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:136
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCompletionPrefix(const QString &)

/*

*/
impl /*struct*/ QCompleter {
  pub fn setCompletionPrefix_0<RetType, T: QCompleter_setCompletionPrefix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCompletionPrefix_0(self);
    // return 1;
  }
}
pub trait QCompleter_setCompletionPrefix_0<RetType> {
  fn setCompletionPrefix_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_setCompletionPrefix_0<(/*void*/)> for (usize) {
  fn setCompletionPrefix_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter19setCompletionPrefixERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void complete(const QRect &)

/*
For QCompleter::PopupCompletion and QCompletion::UnfilteredPopupCompletion modes, calling this function displays the popup displaying the current completions. By default, if rect is not specified, the popup is displayed on the bottom of the widget(). If rect is specified the popup is displayed on the left edge of the rectangle.

For QCompleter::InlineCompletion mode, the highlighted() signal is fired with the current completion.
*/
impl /*struct*/ QCompleter {
  pub fn complete_0<RetType, T: QCompleter_complete_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.complete_0(self);
    // return 1;
  }
}
pub trait QCompleter_complete_0<RetType> {
  fn complete_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_complete_0<(/*void*/)> for (usize) {
  fn complete_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter8completeERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:138
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWrapAround(bool)

/*

*/
impl /*struct*/ QCompleter {
  pub fn setWrapAround_0<RetType, T: QCompleter_setWrapAround_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWrapAround_0(self);
    // return 1;
  }
}
pub trait QCompleter_setWrapAround_0<RetType> {
  fn setWrapAround_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_setWrapAround_0<(/*void*/)> for (bool) {
  fn setWrapAround_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter13setWrapAroundEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:141
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString pathFromIndex(const QModelIndex &) const

/*
Returns the path for the given index. The completer object uses this to obtain the completion text from the underlying model.

The default implementation returns the edit role of the item for list models. It returns the absolute file path if the model is a QFileSystemModel.

See also splitPath().
*/
impl /*struct*/ QCompleter {
  pub fn pathFromIndex_0<RetType, T: QCompleter_pathFromIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pathFromIndex_0(self);
    // return 1;
  }
}
pub trait QCompleter_pathFromIndex_0<RetType> {
  fn pathFromIndex_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_pathFromIndex_0<usize> for (usize) {
  fn pathFromIndex_0(self , rsthis: & QCompleter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter13pathFromIndexERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:142
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QStringList splitPath(const QString &) const

/*
Splits the given path into strings that are used to match at each level in the model().

The default implementation of splitPath() splits a file system path based on QDir::separator() when the sourceModel() is a QFileSystemModel.

When used with list models, the first item in the returned list is used for matching.

See also pathFromIndex() and Handling Tree Models.
*/
impl /*struct*/ QCompleter {
  pub fn splitPath_0<RetType, T: QCompleter_splitPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.splitPath_0(self);
    // return 1;
  }
}
pub trait QCompleter_splitPath_0<RetType> {
  fn splitPath_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_splitPath_0<usize> for (usize) {
  fn splitPath_0(self , rsthis: & QCompleter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QCompleter9splitPathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:145
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().
*/
impl /*struct*/ QCompleter {
  pub fn eventFilter_0<RetType, T: QCompleter_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QCompleter_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QCompleter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QCompleter11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:146
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QCompleter {
  pub fn event_0<RetType, T: QCompleter_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QCompleter_event_0<RetType> {
  fn event_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QCompleter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QCompleter5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:149
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activated(const QString &)

/*
This signal is sent when an item in the popup() is activated by the user (by clicking or pressing return). The item's text is given.

Note: Signal activated is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(completer, QOverload<const QString &>::of(&QCompleter::activated),
      [=](const QString &text){ /-* ... *-/ });
*/
impl /*struct*/ QCompleter {
  pub fn activated_0<RetType, T: QCompleter_activated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activated_0(self);
    // return 1;
  }
}
pub trait QCompleter_activated_0<RetType> {
  fn activated_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_activated_0<(/*void*/)> for (usize) {
  fn activated_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter9activatedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:150
// index:1
// Public Visibility=Default Availability=Available
// [-2] void activated(const QModelIndex &)

/*
This signal is sent when an item in the popup() is activated by the user (by clicking or pressing return). The item's text is given.

Note: Signal activated is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(completer, QOverload<const QString &>::of(&QCompleter::activated),
      [=](const QString &text){ /-* ... *-/ });
*/
impl /*struct*/ QCompleter {
  pub fn activated_1<RetType, T: QCompleter_activated_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activated_1(self);
    // return 1;
  }
}
pub trait QCompleter_activated_1<RetType> {
  fn activated_1(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_activated_1<(/*void*/)> for (usize) {
  fn activated_1(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter9activatedERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:151
// index:0
// Public Visibility=Default Availability=Available
// [-2] void highlighted(const QString &)

/*
This signal is sent when an item in the popup() is highlighted by the user. It is also sent if complete() is called with the completionMode() set to QCompleter::InlineCompletion. The item's text is given.

Note: Signal highlighted is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(completer, QOverload<const QString &>::of(&QCompleter::highlighted),
      [=](const QString &text){ /-* ... *-/ });
*/
impl /*struct*/ QCompleter {
  pub fn highlighted_0<RetType, T: QCompleter_highlighted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.highlighted_0(self);
    // return 1;
  }
}
pub trait QCompleter_highlighted_0<RetType> {
  fn highlighted_0(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_highlighted_0<(/*void*/)> for (usize) {
  fn highlighted_0(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter11highlightedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcompleter.h:152
// index:1
// Public Visibility=Default Availability=Available
// [-2] void highlighted(const QModelIndex &)

/*
This signal is sent when an item in the popup() is highlighted by the user. It is also sent if complete() is called with the completionMode() set to QCompleter::InlineCompletion. The item's text is given.

Note: Signal highlighted is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(completer, QOverload<const QString &>::of(&QCompleter::highlighted),
      [=](const QString &text){ /-* ... *-/ });
*/
impl /*struct*/ QCompleter {
  pub fn highlighted_1<RetType, T: QCompleter_highlighted_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.highlighted_1(self);
    // return 1;
  }
}
pub trait QCompleter_highlighted_1<RetType> {
  fn highlighted_1(self , rsthis: & QCompleter) -> RetType;
}
impl<'a> /*trait*/ QCompleter_highlighted_1<(/*void*/)> for (usize) {
  fn highlighted_1(self , rsthis: & QCompleter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QCompleter11highlightedERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum specifies how completions are provided to the user.



See also setCompletionMode().

*/
pub type QCompleter__CompletionMode = i32;
// Current completions are displayed in a popup window.
pub const QCompleter__PopupCompletion :QCompleter__CompletionMode = 0;
// All possible completions are displayed in a popup window with the most likely suggestion indicated as current.
pub const QCompleter__UnfilteredPopupCompletion :QCompleter__CompletionMode = 1;
// Completions appear inline (as selected text).
pub const QCompleter__InlineCompletion :QCompleter__CompletionMode = 2;
pub fn QCompleter_CompletionModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QCompleter", val);
}
pub fn QCompleter_CompletionModeItemName_s(val: i32) ->String {
  //var nilthis *QCompleter
  //return nilthis.CompletionModeItemName(val);
  return QCompleter_CompletionModeItemName(val);
}


/*
This enum specifies how the items in the model are sorted.



See also setModelSorting().

*/
pub type QCompleter__ModelSorting = i32;
// The model is unsorted.
pub const QCompleter__UnsortedModel :QCompleter__ModelSorting = 0;
// The model is sorted case sensitively.
pub const QCompleter__CaseSensitivelySortedModel :QCompleter__ModelSorting = 1;
// The model is sorted case insensitively.
pub const QCompleter__CaseInsensitivelySortedModel :QCompleter__ModelSorting = 2;
pub fn QCompleter_ModelSortingItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QCompleter", val);
}
pub fn QCompleter_ModelSortingItemName_s(val: i32) ->String {
  //var nilthis *QCompleter
  //return nilthis.ModelSortingItemName(val);
  return QCompleter_ModelSortingItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
