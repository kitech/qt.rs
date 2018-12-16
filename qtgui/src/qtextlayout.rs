

// mod ::gui::QTextLayout
// package qtgui
// /usr/include/qt/QtGui/qtextlayout.h
// #include <qtextlayout.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 14
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
#[derive(Default)] // class sizeof(QTextLayout)=8
pub struct QTextLayout {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextLayout_ITF interface {
//    QTextLayout_PTR() *QTextLayout
//}
//func (ptr *QTextLayout) QTextLayout_PTR() *QTextLayout { return ptr }

impl /*struct*/ QTextLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextLayout {
    return QTextLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextLayout {
//  type Target = QTextLayoutBASE;
//
//  fn deref(&self) -> &QTextLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextLayoutBASE> for QTextLayout {
//  fn as_ref(& self) -> & QTextLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextlayout.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextLayout()

/*
Constructs an empty text layout.

See also setText().
*/
// QTextLayout() ctx.fn_proto_cpp
impl /*struct*/ QTextLayout {
  pub fn QTextLayout_0<T: QTextLayout_QTextLayout_0>(value: T) -> QTextLayout {
    let rsthis = value.QTextLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLayout_QTextLayout_0 {
  fn QTextLayout_0(self) -> QTextLayout;
}
// QTextLayout() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextLayout_QTextLayout_0 for () {
  fn QTextLayout_0(self) -> QTextLayout {
    // unsafe{_ZN11QTextLayoutC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextLayoutC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:109
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTextLayout(const QString &)

/*
Constructs an empty text layout.

See also setText().
*/
// QTextLayout(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QTextLayout {
  pub fn QTextLayout_1<T: QTextLayout_QTextLayout_1>(value: T) -> QTextLayout {
    let rsthis = value.QTextLayout_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLayout_QTextLayout_1 {
  fn QTextLayout_1(self) -> QTextLayout;
}
// QTextLayout(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextLayout_QTextLayout_1 for (usize) {
  fn QTextLayout_1(self) -> QTextLayout {
    // unsafe{_ZN11QTextLayoutC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextLayoutC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:110
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QTextLayout(const QString &, const QFont &, QPaintDevice *)

/*
Constructs an empty text layout.

See also setText().
*/
// QTextLayout(const QString &, const QFont &, QPaintDevice *) ctx.fn_proto_cpp
impl /*struct*/ QTextLayout {
  pub fn QTextLayout_2<T: QTextLayout_QTextLayout_2>(value: T) -> QTextLayout {
    let rsthis = value.QTextLayout_2();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLayout_QTextLayout_2 {
  fn QTextLayout_2(self) -> QTextLayout;
}
// QTextLayout(const QString &, const QFont &, QPaintDevice *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextLayout_QTextLayout_2 for (usize,usize,usize) {
  fn QTextLayout_2(self) -> QTextLayout {
    // unsafe{_ZN11QTextLayoutC2ERK7QStringRK5QFontP12QPaintDevice()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextLayoutC2ERK7QStringRK5QFontP12QPaintDevice", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:111
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QTextLayout(const QTextBlock &)

/*
Constructs an empty text layout.

See also setText().
*/
// QTextLayout(const QTextBlock &) ctx.fn_proto_cpp
impl /*struct*/ QTextLayout {
  pub fn QTextLayout_3<T: QTextLayout_QTextLayout_3>(value: T) -> QTextLayout {
    let rsthis = value.QTextLayout_3();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLayout_QTextLayout_3 {
  fn QTextLayout_3(self) -> QTextLayout;
}
// QTextLayout(const QTextBlock &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextLayout_QTextLayout_3 for (usize) {
  fn QTextLayout_3(self) -> QTextLayout {
    // unsafe{_ZN11QTextLayoutC2ERK10QTextBlock()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextLayoutC2ERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTextLayout()

/*

*/
pub fn DeleteQTextLayout(this :*mut QTextLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QTextLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtextlayout.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*
Sets the layout's font to the given font. The layout is invalidated and must be laid out again.

See also font().
*/
impl /*struct*/ QTextLayout {
  pub fn setFont_0<RetType, T: QTextLayout_setFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_0(self);
    // return 1;
  }
}
pub trait QTextLayout_setFont_0<RetType> {
  fn setFont_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_setFont_0<(/*void*/)> for (usize) {
  fn setFont_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextLayout7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:115
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont font() const

/*
Returns the current font that is used for the layout, or a default font if none is set.

See also setFont().
*/
impl /*struct*/ QTextLayout {
  pub fn font_0<RetType, T: QTextLayout_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QTextLayout_font_0<RetType> {
  fn font_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_font_0<usize> for () {
  fn font_0(self , rsthis: & QTextLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRawFont(const QRawFont &)

/*

*/
impl /*struct*/ QTextLayout {
  pub fn setRawFont_0<RetType, T: QTextLayout_setRawFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRawFont_0(self);
    // return 1;
  }
}
pub trait QTextLayout_setRawFont_0<RetType> {
  fn setRawFont_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_setRawFont_0<(/*void*/)> for (usize) {
  fn setRawFont_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextLayout10setRawFontERK8QRawFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*
Sets the layout's text to the given string. The layout is invalidated and must be laid out again.

Notice that when using this QTextLayout as part of a QTextDocument this method will have no effect.

See also text().
*/
impl /*struct*/ QTextLayout {
  pub fn setText_0<RetType, T: QTextLayout_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QTextLayout_setText_0<RetType> {
  fn setText_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextLayout7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:122
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*
Returns the layout's text.

See also setText().
*/
impl /*struct*/ QTextLayout {
  pub fn text_0<RetType, T: QTextLayout_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QTextLayout_text_0<RetType> {
  fn text_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_text_0<usize> for () {
  fn text_0(self , rsthis: & QTextLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:124
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextOption(const QTextOption &)

/*
Sets the text option structure that controls the layout process to the given option.

See also textOption().
*/
impl /*struct*/ QTextLayout {
  pub fn setTextOption_0<RetType, T: QTextLayout_setTextOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextOption_0(self);
    // return 1;
  }
}
pub trait QTextLayout_setTextOption_0<RetType> {
  fn setTextOption_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_setTextOption_0<(/*void*/)> for (usize) {
  fn setTextOption_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextLayout13setTextOptionERK11QTextOption", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:125
// index:0
// Public Visibility=Default Availability=Available
// [32] const QTextOption & textOption() const

/*
Returns the current text option used to control the layout process.

See also setTextOption().
*/
impl /*struct*/ QTextLayout {
  pub fn textOption_0<RetType, T: QTextLayout_textOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textOption_0(self);
    // return 1;
  }
}
pub trait QTextLayout_textOption_0<RetType> {
  fn textOption_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_textOption_0<usize> for () {
  fn textOption_0(self , rsthis: & QTextLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout10textOptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPreeditArea(int, const QString &)

/*
Sets the position and text of the area in the layout that is processed before editing occurs. The layout is invalidated and must be laid out again.

See also preeditAreaPosition() and preeditAreaText().
*/
impl /*struct*/ QTextLayout {
  pub fn setPreeditArea_0<RetType, T: QTextLayout_setPreeditArea_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPreeditArea_0(self);
    // return 1;
  }
}
pub trait QTextLayout_setPreeditArea_0<RetType> {
  fn setPreeditArea_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_setPreeditArea_0<(/*void*/)> for (i32,usize) {
  fn setPreeditArea_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextLayout14setPreeditAreaEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:128
// index:0
// Public Visibility=Default Availability=Available
// [4] int preeditAreaPosition() const

/*
Returns the position of the area in the text layout that will be processed before editing occurs.

See also preeditAreaText().
*/
impl /*struct*/ QTextLayout {
  pub fn preeditAreaPosition_0<RetType, T: QTextLayout_preeditAreaPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.preeditAreaPosition_0(self);
    // return 1;
  }
}
pub trait QTextLayout_preeditAreaPosition_0<RetType> {
  fn preeditAreaPosition_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_preeditAreaPosition_0<i32> for () {
  fn preeditAreaPosition_0(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout19preeditAreaPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:129
// index:0
// Public Visibility=Default Availability=Available
// [8] QString preeditAreaText() const

/*
Returns the text that is inserted in the layout before editing occurs.

See also preeditAreaPosition().
*/
impl /*struct*/ QTextLayout {
  pub fn preeditAreaText_0<RetType, T: QTextLayout_preeditAreaText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.preeditAreaText_0(self);
    // return 1;
  }
}
pub trait QTextLayout_preeditAreaText_0<RetType> {
  fn preeditAreaText_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_preeditAreaText_0<usize> for () {
  fn preeditAreaText_0(self , rsthis: & QTextLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout15preeditAreaTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearAdditionalFormats()

/*

*/
impl /*struct*/ QTextLayout {
  pub fn clearAdditionalFormats_0<RetType, T: QTextLayout_clearAdditionalFormats_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearAdditionalFormats_0(self);
    // return 1;
  }
}
pub trait QTextLayout_clearAdditionalFormats_0<RetType> {
  fn clearAdditionalFormats_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_clearAdditionalFormats_0<(/*void*/)> for () {
  fn clearAdditionalFormats_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextLayout22clearAdditionalFormatsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearFormats()

/*
Clears the list of additional formats supported by the text layout.

This function was introduced in  Qt 5.6.

See also formats() and setFormats().
*/
impl /*struct*/ QTextLayout {
  pub fn clearFormats_0<RetType, T: QTextLayout_clearFormats_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearFormats_0(self);
    // return 1;
  }
}
pub trait QTextLayout_clearFormats_0<RetType> {
  fn clearFormats_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_clearFormats_0<(/*void*/)> for () {
  fn clearFormats_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextLayout12clearFormatsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:150
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCacheEnabled(bool)

/*
Enables caching of the complete layout information if enable is true; otherwise disables layout caching. Usually QTextLayout throws most of the layouting information away after a call to endLayout() to reduce memory consumption. If you however want to draw the laid out text directly afterwards enabling caching might speed up drawing significantly.

See also cacheEnabled().
*/
impl /*struct*/ QTextLayout {
  pub fn setCacheEnabled_0<RetType, T: QTextLayout_setCacheEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCacheEnabled_0(self);
    // return 1;
  }
}
pub trait QTextLayout_setCacheEnabled_0<RetType> {
  fn setCacheEnabled_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_setCacheEnabled_0<(/*void*/)> for (bool) {
  fn setCacheEnabled_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextLayout15setCacheEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:151
// index:0
// Public Visibility=Default Availability=Available
// [1] bool cacheEnabled() const

/*
Returns true if the complete layout information is cached; otherwise returns false.

See also setCacheEnabled().
*/
impl /*struct*/ QTextLayout {
  pub fn cacheEnabled_0<RetType, T: QTextLayout_cacheEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cacheEnabled_0(self);
    // return 1;
  }
}
pub trait QTextLayout_cacheEnabled_0<RetType> {
  fn cacheEnabled_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_cacheEnabled_0<bool> for () {
  fn cacheEnabled_0(self , rsthis: & QTextLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout12cacheEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:153
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCursorMoveStyle(Qt::CursorMoveStyle)

/*
Sets the visual cursor movement style to the given style. If the QTextLayout is backed by a document, you can ignore this and use the option in QTextDocument, this option is for widgets like QLineEdit or custom widgets without a QTextDocument. Default value is Qt::LogicalMoveStyle.

See also cursorMoveStyle().
*/
impl /*struct*/ QTextLayout {
  pub fn setCursorMoveStyle_0<RetType, T: QTextLayout_setCursorMoveStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCursorMoveStyle_0(self);
    // return 1;
  }
}
pub trait QTextLayout_setCursorMoveStyle_0<RetType> {
  fn setCursorMoveStyle_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_setCursorMoveStyle_0<(/*void*/)> for (i32) {
  fn setCursorMoveStyle_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextLayout18setCursorMoveStyleEN2Qt15CursorMoveStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:154
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::CursorMoveStyle cursorMoveStyle() const

/*
The cursor movement style of this QTextLayout. The default is Qt::LogicalMoveStyle.

See also setCursorMoveStyle().
*/
impl /*struct*/ QTextLayout {
  pub fn cursorMoveStyle_0<RetType, T: QTextLayout_cursorMoveStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorMoveStyle_0(self);
    // return 1;
  }
}
pub trait QTextLayout_cursorMoveStyle_0<RetType> {
  fn cursorMoveStyle_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_cursorMoveStyle_0<i32> for () {
  fn cursorMoveStyle_0(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout15cursorMoveStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void beginLayout()

/*
Begins the layout process.

Warning: This will invalidate the layout, so all existing QTextLine objects that refer to the previous contents should now be discarded.

See also endLayout().
*/
impl /*struct*/ QTextLayout {
  pub fn beginLayout_0<RetType, T: QTextLayout_beginLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginLayout_0(self);
    // return 1;
  }
}
pub trait QTextLayout_beginLayout_0<RetType> {
  fn beginLayout_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_beginLayout_0<(/*void*/)> for () {
  fn beginLayout_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextLayout11beginLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void endLayout()

/*
Ends the layout process.

See also beginLayout().
*/
impl /*struct*/ QTextLayout {
  pub fn endLayout_0<RetType, T: QTextLayout_endLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endLayout_0(self);
    // return 1;
  }
}
pub trait QTextLayout_endLayout_0<RetType> {
  fn endLayout_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_endLayout_0<(/*void*/)> for () {
  fn endLayout_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextLayout9endLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearLayout()

/*
Clears the line information in the layout. After having called this function, lineCount() returns 0.

Warning: This will invalidate the layout, so all existing QTextLine objects that refer to the previous contents should now be discarded.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QTextLayout {
  pub fn clearLayout_0<RetType, T: QTextLayout_clearLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearLayout_0(self);
    // return 1;
  }
}
pub trait QTextLayout_clearLayout_0<RetType> {
  fn clearLayout_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_clearLayout_0<(/*void*/)> for () {
  fn clearLayout_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextLayout11clearLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:160
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextLine createLine()

/*
Returns a new text line to be laid out if there is text to be inserted into the layout; otherwise returns an invalid text line.

The text layout creates a new line object that starts after the last line in the layout, or at the beginning if the layout is empty. The layout maintains an internal cursor, and each line is filled with text from the cursor position onwards when the QTextLine::setLineWidth() function is called.

Once QTextLine::setLineWidth() is called, a new line can be created and filled with text. Repeating this process will lay out the whole block of text contained in the QTextLayout. If there is no text left to be inserted into the layout, the QTextLine returned will not be valid (isValid() will return false).
*/
impl /*struct*/ QTextLayout {
  pub fn createLine_0<RetType, T: QTextLayout_createLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createLine_0(self);
    // return 1;
  }
}
pub trait QTextLayout_createLine_0<RetType> {
  fn createLine_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_createLine_0<usize> for () {
  fn createLine_0(self , rsthis: & QTextLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextLayout10createLineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:162
// index:0
// Public Visibility=Default Availability=Available
// [4] int lineCount() const

/*
Returns the number of lines in this text layout.

See also lineAt().
*/
impl /*struct*/ QTextLayout {
  pub fn lineCount_0<RetType, T: QTextLayout_lineCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineCount_0(self);
    // return 1;
  }
}
pub trait QTextLayout_lineCount_0<RetType> {
  fn lineCount_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_lineCount_0<i32> for () {
  fn lineCount_0(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout9lineCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:163
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextLine lineAt(int) const

/*
Returns the i-th line of text in this text layout.

See also lineCount() and lineForTextPosition().
*/
impl /*struct*/ QTextLayout {
  pub fn lineAt_0<RetType, T: QTextLayout_lineAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineAt_0(self);
    // return 1;
  }
}
pub trait QTextLayout_lineAt_0<RetType> {
  fn lineAt_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_lineAt_0<usize> for (i32) {
  fn lineAt_0(self , rsthis: & QTextLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout6lineAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:164
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextLine lineForTextPosition(int) const

/*
Returns the line that contains the cursor position specified by pos.

See also isValidCursorPosition() and lineAt().
*/
impl /*struct*/ QTextLayout {
  pub fn lineForTextPosition_0<RetType, T: QTextLayout_lineForTextPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineForTextPosition_0(self);
    // return 1;
  }
}
pub trait QTextLayout_lineForTextPosition_0<RetType> {
  fn lineForTextPosition_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_lineForTextPosition_0<usize> for (i32) {
  fn lineForTextPosition_0(self , rsthis: & QTextLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout19lineForTextPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:170
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValidCursorPosition(int) const

/*
/ Returns true if position pos is a valid cursor position.

In a Unicode context some positions in the text are not valid cursor positions, because the position is inside a Unicode surrogate or a grapheme cluster.

A grapheme cluster is a sequence of two or more Unicode characters that form one indivisible entity on the screen. For example the latin character `Ä' can be represented in Unicode by two characters, `A' (0x41), and the combining diaresis (0x308). A text cursor can only validly be positioned before or after these two characters, never between them since that wouldn't make sense. In indic languages every syllable forms a grapheme cluster.
*/
impl /*struct*/ QTextLayout {
  pub fn isValidCursorPosition_0<RetType, T: QTextLayout_isValidCursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValidCursorPosition_0(self);
    // return 1;
  }
}
pub trait QTextLayout_isValidCursorPosition_0<RetType> {
  fn isValidCursorPosition_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_isValidCursorPosition_0<bool> for (i32) {
  fn isValidCursorPosition_0(self , rsthis: & QTextLayout) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout21isValidCursorPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:171
// index:0
// Public Visibility=Default Availability=Available
// [4] int nextCursorPosition(int, QTextLayout::CursorMode) const

/*
Returns the next valid cursor position after oldPos that respects the given cursor mode. Returns value of oldPos, if oldPos is not a valid cursor position.

See also isValidCursorPosition() and previousCursorPosition().
*/
impl /*struct*/ QTextLayout {
  pub fn nextCursorPosition_0<RetType, T: QTextLayout_nextCursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nextCursorPosition_0(self);
    // return 1;
  }
}
pub trait QTextLayout_nextCursorPosition_0<RetType> {
  fn nextCursorPosition_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_nextCursorPosition_0<i32> for (i32,i32) {
  fn nextCursorPosition_0(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout18nextCursorPositionEiNS_10CursorModeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:172
// index:0
// Public Visibility=Default Availability=Available
// [4] int previousCursorPosition(int, QTextLayout::CursorMode) const

/*
Returns the first valid cursor position before oldPos that respects the given cursor mode. Returns value of oldPos, if oldPos is not a valid cursor position.

See also isValidCursorPosition() and nextCursorPosition().
*/
impl /*struct*/ QTextLayout {
  pub fn previousCursorPosition_0<RetType, T: QTextLayout_previousCursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.previousCursorPosition_0(self);
    // return 1;
  }
}
pub trait QTextLayout_previousCursorPosition_0<RetType> {
  fn previousCursorPosition_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_previousCursorPosition_0<i32> for (i32,i32) {
  fn previousCursorPosition_0(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout22previousCursorPositionEiNS_10CursorModeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:173
// index:0
// Public Visibility=Default Availability=Available
// [4] int leftCursorPosition(int) const

/*
Returns the cursor position to the left of oldPos, next to it. It's dependent on the visual position of characters, after bi-directional reordering.

See also rightCursorPosition() and previousCursorPosition().
*/
impl /*struct*/ QTextLayout {
  pub fn leftCursorPosition_0<RetType, T: QTextLayout_leftCursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leftCursorPosition_0(self);
    // return 1;
  }
}
pub trait QTextLayout_leftCursorPosition_0<RetType> {
  fn leftCursorPosition_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_leftCursorPosition_0<i32> for (i32) {
  fn leftCursorPosition_0(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout18leftCursorPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:174
// index:0
// Public Visibility=Default Availability=Available
// [4] int rightCursorPosition(int) const

/*
Returns the cursor position to the right of oldPos, next to it. It's dependent on the visual position of characters, after bi-directional reordering.

See also leftCursorPosition() and nextCursorPosition().
*/
impl /*struct*/ QTextLayout {
  pub fn rightCursorPosition_0<RetType, T: QTextLayout_rightCursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rightCursorPosition_0(self);
    // return 1;
  }
}
pub trait QTextLayout_rightCursorPosition_0<RetType> {
  fn rightCursorPosition_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_rightCursorPosition_0<i32> for (i32) {
  fn rightCursorPosition_0(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout19rightCursorPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawCursor(QPainter *, const QPointF &, int) const

/*
Draws a text cursor with the current pen and the specified width at the given position using the painter specified. The corresponding position within the text is specified by cursorPosition.
*/
impl /*struct*/ QTextLayout {
  pub fn drawCursor_0<RetType, T: QTextLayout_drawCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawCursor_0(self);
    // return 1;
  }
}
pub trait QTextLayout_drawCursor_0<RetType> {
  fn drawCursor_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_drawCursor_0<(/*void*/)> for (usize,usize,i32) {
  fn drawCursor_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFi", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:179
// index:1
// Public Visibility=Default Availability=Available
// [-2] void drawCursor(QPainter *, const QPointF &, int, int) const

/*
Draws a text cursor with the current pen and the specified width at the given position using the painter specified. The corresponding position within the text is specified by cursorPosition.
*/
impl /*struct*/ QTextLayout {
  pub fn drawCursor_1<RetType, T: QTextLayout_drawCursor_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawCursor_1(self);
    // return 1;
  }
}
pub trait QTextLayout_drawCursor_1<RetType> {
  fn drawCursor_1(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_drawCursor_1<(/*void*/)> for (usize,usize,i32,i32) {
  fn drawCursor_1(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFii", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:181
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF position() const

/*
The global position of the layout. This is independent of the bounding rectangle and of the layout process.

This function was introduced in  Qt 4.2.

See also setPosition().
*/
impl /*struct*/ QTextLayout {
  pub fn position_0<RetType, T: QTextLayout_position_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.position_0(self);
    // return 1;
  }
}
pub trait QTextLayout_position_0<RetType> {
  fn position_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_position_0<usize> for () {
  fn position_0(self , rsthis: & QTextLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout8positionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:182
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPosition(const QPointF &)

/*
Moves the text layout to point p.

See also position().
*/
impl /*struct*/ QTextLayout {
  pub fn setPosition_0<RetType, T: QTextLayout_setPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPosition_0(self);
    // return 1;
  }
}
pub trait QTextLayout_setPosition_0<RetType> {
  fn setPosition_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_setPosition_0<(/*void*/)> for (usize) {
  fn setPosition_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextLayout11setPositionERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:184
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF boundingRect() const

/*
The smallest rectangle that contains all the lines in the layout.
*/
impl /*struct*/ QTextLayout {
  pub fn boundingRect_0<RetType, T: QTextLayout_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QTextLayout_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_boundingRect_0<usize> for () {
  fn boundingRect_0(self , rsthis: & QTextLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout12boundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:186
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal minimumWidth() const

/*
The minimum width the layout needs. This is the width of the layout's smallest non-breakable substring.

Warning: This function only returns a valid value after the layout has been done.

See also maximumWidth().
*/
impl /*struct*/ QTextLayout {
  pub fn minimumWidth_0<RetType, T: QTextLayout_minimumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumWidth_0(self);
    // return 1;
  }
}
pub trait QTextLayout_minimumWidth_0<RetType> {
  fn minimumWidth_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_minimumWidth_0<f64> for () {
  fn minimumWidth_0(self , rsthis: & QTextLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout12minimumWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:187
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal maximumWidth() const

/*
The maximum width the layout could expand to; this is essentially the width of the entire text.

Warning: This function only returns a valid value after the layout has been done.

See also minimumWidth().
*/
impl /*struct*/ QTextLayout {
  pub fn maximumWidth_0<RetType, T: QTextLayout_maximumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumWidth_0(self);
    // return 1;
  }
}
pub trait QTextLayout_maximumWidth_0<RetType> {
  fn maximumWidth_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_maximumWidth_0<f64> for () {
  fn maximumWidth_0(self , rsthis: & QTextLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLayout12maximumWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:194
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlags(int)

/*

*/
impl /*struct*/ QTextLayout {
  pub fn setFlags_0<RetType, T: QTextLayout_setFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlags_0(self);
    // return 1;
  }
}
pub trait QTextLayout_setFlags_0<RetType> {
  fn setFlags_0(self , rsthis: & QTextLayout) -> RetType;
}
impl<'a> /*trait*/ QTextLayout_setFlags_0<(/*void*/)> for (i32) {
  fn setFlags_0(self , rsthis: & QTextLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextLayout8setFlagsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
ConstantValue
QTextLayout::SkipCharacters0
QTextLayout::SkipWords1

*/
pub type QTextLayout__CursorMode = i32;
// 
pub const QTextLayout__SkipCharacters :QTextLayout__CursorMode = 0;
// 
pub const QTextLayout__SkipWords :QTextLayout__CursorMode = 1;
pub fn QTextLayout_CursorModeItemName(val: i32) ->String {
  match val {
     QTextLayout__SkipCharacters => // 0
     {return String::from("SkipCharacters");}
     QTextLayout__SkipWords => // 1
     {return String::from("SkipWords");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextLayout_CursorModeItemName_s(val: i32) ->String {
  //var nilthis *QTextLayout
  //return nilthis.CursorModeItemName(val);
  return QTextLayout_CursorModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
