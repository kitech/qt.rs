

// mod ::widgets::QFontComboBox
// package qtwidgets
// /usr/include/qt/QtWidgets/qfontcombobox.h
// #include <qfontcombobox.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 9
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
// func (this *QFontComboBox) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QFontComboBox)=48
pub struct QFontComboBox {
  qbase: QComboBox,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFontComboBox_ITF interface {
//    QComboBox_ITF
//    QFontComboBox_PTR() *QFontComboBox
//}
//func (ptr *QFontComboBox) QFontComboBox_PTR() *QFontComboBox { return ptr }

impl /*struct*/ QFontComboBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFontComboBox {
    return QFontComboBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFontComboBox {
//  type Target = QFontComboBoxBASE;
//
//  fn deref(&self) -> &QFontComboBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFontComboBoxBASE> for QFontComboBox {
//  fn as_ref(& self) -> & QFontComboBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qfontcombobox.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QFontComboBox {
  pub fn metaObject_0<RetType, T: QFontComboBox_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QFontComboBox_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QFontComboBox) -> RetType;
}
impl<'a> /*trait*/ QFontComboBox_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QFontComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontComboBox10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfontcombobox.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFontComboBox(QWidget *)

/*
Constructs a font combobox with the given parent.
*/
// QFontComboBox(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QFontComboBox {
  pub fn QFontComboBox_0<T: QFontComboBox_QFontComboBox_0>(value: T) -> QFontComboBox {
    let rsthis = value.QFontComboBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFontComboBox_QFontComboBox_0 {
  fn QFontComboBox_0(self) -> QFontComboBox;
}
// QFontComboBox(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFontComboBox_QFontComboBox_0 for (usize) {
  fn QFontComboBox_0(self) -> QFontComboBox {
    // unsafe{_ZN13QFontComboBoxC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QFontComboBoxC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFontComboBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontcombobox.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFontComboBox()

/*

*/
pub fn DeleteQFontComboBox(this :*mut QFontComboBox) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QFontComboBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qfontcombobox.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWritingSystem(QFontDatabase::WritingSystem)

/*

*/
impl /*struct*/ QFontComboBox {
  pub fn setWritingSystem_0<RetType, T: QFontComboBox_setWritingSystem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWritingSystem_0(self);
    // return 1;
  }
}
pub trait QFontComboBox_setWritingSystem_0<RetType> {
  fn setWritingSystem_0(self , rsthis: & QFontComboBox) -> RetType;
}
impl<'a> /*trait*/ QFontComboBox_setWritingSystem_0<(/*void*/)> for (i32) {
  fn setWritingSystem_0(self , rsthis: & QFontComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QFontComboBox16setWritingSystemEN13QFontDatabase13WritingSystemE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontcombobox.h:65
// index:0
// Public Visibility=Default Availability=Available
// [4] QFontDatabase::WritingSystem writingSystem() const

/*

*/
impl /*struct*/ QFontComboBox {
  pub fn writingSystem_0<RetType, T: QFontComboBox_writingSystem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writingSystem_0(self);
    // return 1;
  }
}
pub trait QFontComboBox_writingSystem_0<RetType> {
  fn writingSystem_0(self , rsthis: & QFontComboBox) -> RetType;
}
impl<'a> /*trait*/ QFontComboBox_writingSystem_0<i32> for () {
  fn writingSystem_0(self , rsthis: & QFontComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontComboBox13writingSystemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfontcombobox.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFontFilters(QFontComboBox::FontFilters)

/*

*/
impl /*struct*/ QFontComboBox {
  pub fn setFontFilters_0<RetType, T: QFontComboBox_setFontFilters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontFilters_0(self);
    // return 1;
  }
}
pub trait QFontComboBox_setFontFilters_0<RetType> {
  fn setFontFilters_0(self , rsthis: & QFontComboBox) -> RetType;
}
impl<'a> /*trait*/ QFontComboBox_setFontFilters_0<(/*void*/)> for (i32) {
  fn setFontFilters_0(self , rsthis: & QFontComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QFontComboBox14setFontFiltersE6QFlagsINS_10FontFilterEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontcombobox.h:78
// index:0
// Public Visibility=Default Availability=Available
// [4] QFontComboBox::FontFilters fontFilters() const

/*

*/
impl /*struct*/ QFontComboBox {
  pub fn fontFilters_0<RetType, T: QFontComboBox_fontFilters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontFilters_0(self);
    // return 1;
  }
}
pub trait QFontComboBox_fontFilters_0<RetType> {
  fn fontFilters_0(self , rsthis: & QFontComboBox) -> RetType;
}
impl<'a> /*trait*/ QFontComboBox_fontFilters_0<i32> for () {
  fn fontFilters_0(self , rsthis: & QFontComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontComboBox11fontFiltersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfontcombobox.h:80
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont currentFont() const

/*

*/
impl /*struct*/ QFontComboBox {
  pub fn currentFont_0<RetType, T: QFontComboBox_currentFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentFont_0(self);
    // return 1;
  }
}
pub trait QFontComboBox_currentFont_0<RetType> {
  fn currentFont_0(self , rsthis: & QFontComboBox) -> RetType;
}
impl<'a> /*trait*/ QFontComboBox_currentFont_0<usize> for () {
  fn currentFont_0(self , rsthis: & QFontComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontComboBox11currentFontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfontcombobox.h:81
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QFontComboBox {
  pub fn sizeHint_0<RetType, T: QFontComboBox_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QFontComboBox_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QFontComboBox) -> RetType;
}
impl<'a> /*trait*/ QFontComboBox_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QFontComboBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontComboBox8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfontcombobox.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentFont(const QFont &)

/*

*/
impl /*struct*/ QFontComboBox {
  pub fn setCurrentFont_0<RetType, T: QFontComboBox_setCurrentFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentFont_0(self);
    // return 1;
  }
}
pub trait QFontComboBox_setCurrentFont_0<RetType> {
  fn setCurrentFont_0(self , rsthis: & QFontComboBox) -> RetType;
}
impl<'a> /*trait*/ QFontComboBox_setCurrentFont_0<(/*void*/)> for (usize) {
  fn setCurrentFont_0(self , rsthis: & QFontComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QFontComboBox14setCurrentFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontcombobox.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentFontChanged(const QFont &)

/*
This signal is emitted whenever the current font changes, with the new font.

Note: Notifier signal for property currentFont. 

See also currentFont.
*/
impl /*struct*/ QFontComboBox {
  pub fn currentFontChanged_0<RetType, T: QFontComboBox_currentFontChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentFontChanged_0(self);
    // return 1;
  }
}
pub trait QFontComboBox_currentFontChanged_0<RetType> {
  fn currentFontChanged_0(self , rsthis: & QFontComboBox) -> RetType;
}
impl<'a> /*trait*/ QFontComboBox_currentFontChanged_0<(/*void*/)> for (usize) {
  fn currentFontChanged_0(self , rsthis: & QFontComboBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QFontComboBox18currentFontChangedERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfontcombobox.h:90
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QFontComboBox {
  pub fn event_0<RetType, T: QFontComboBox_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QFontComboBox_event_0<RetType> {
  fn event_0(self , rsthis: & QFontComboBox) -> RetType;
}
impl<'a> /*trait*/ QFontComboBox_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QFontComboBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontComboBox5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


*/
pub type QFontComboBox__FontFilter = i32;
// 
pub const QFontComboBox__AllFonts :QFontComboBox__FontFilter = 0;
// 
pub const QFontComboBox__ScalableFonts :QFontComboBox__FontFilter = 1;
// 
pub const QFontComboBox__NonScalableFonts :QFontComboBox__FontFilter = 2;
// 
pub const QFontComboBox__MonospacedFonts :QFontComboBox__FontFilter = 4;
// 
pub const QFontComboBox__ProportionalFonts :QFontComboBox__FontFilter = 8;
pub fn QFontComboBox_FontFilterItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFontComboBox", val);
}
pub fn QFontComboBox_FontFilterItemName_s(val: i32) ->String {
  //var nilthis *QFontComboBox
  //return nilthis.FontFilterItemName(val);
  return QFontComboBox_FontFilterItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
