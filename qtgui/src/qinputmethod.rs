

// mod ::gui::QInputMethod
// package qtgui
// /usr/include/qt/QtGui/qinputmethod.h
// #include <qinputmethod.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 2
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
#[derive(Default)] // class sizeof(QInputMethod)=16
pub struct QInputMethod {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QInputMethod_ITF interface {
//    qtcore.QObject_ITF
//    QInputMethod_PTR() *QInputMethod
//}
//func (ptr *QInputMethod) QInputMethod_PTR() *QInputMethod { return ptr }

impl /*struct*/ QInputMethod {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QInputMethod {
    return QInputMethod{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QInputMethod {
//  type Target = QInputMethodBASE;
//
//  fn deref(&self) -> &QInputMethodBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QInputMethodBASE> for QInputMethod {
//  fn as_ref(& self) -> & QInputMethodBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qinputmethod.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QInputMethod {
  pub fn metaObject_0<RetType, T: QInputMethod_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QInputMethod_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QInputMethod) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputMethod10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:68
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform inputItemTransform() const

/*
Returns the transformation from input item coordinates to the window coordinates.

See also setInputItemTransform().
*/
impl /*struct*/ QInputMethod {
  pub fn inputItemTransform_0<RetType, T: QInputMethod_inputItemTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputItemTransform_0(self);
    // return 1;
  }
}
pub trait QInputMethod_inputItemTransform_0<RetType> {
  fn inputItemTransform_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_inputItemTransform_0<usize> for () {
  fn inputItemTransform_0(self , rsthis: & QInputMethod) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputMethod18inputItemTransformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInputItemTransform(const QTransform &)

/*
Sets the transformation from input item coordinates to window coordinates to be transform. Item transform needs to be updated by the focused window like QQuickCanvas whenever item is moved inside the scene.

See also inputItemTransform().
*/
impl /*struct*/ QInputMethod {
  pub fn setInputItemTransform_0<RetType, T: QInputMethod_setInputItemTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInputItemTransform_0(self);
    // return 1;
  }
}
pub trait QInputMethod_setInputItemTransform_0<RetType> {
  fn setInputItemTransform_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_setInputItemTransform_0<(/*void*/)> for (usize) {
  fn setInputItemTransform_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputMethod21setInputItemTransformERK10QTransform", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:71
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF inputItemRectangle() const

/*
Returns the input item's geometry in input item coordinates.

This function was introduced in  Qt 5.1.

See also setInputItemRectangle().
*/
impl /*struct*/ QInputMethod {
  pub fn inputItemRectangle_0<RetType, T: QInputMethod_inputItemRectangle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputItemRectangle_0(self);
    // return 1;
  }
}
pub trait QInputMethod_inputItemRectangle_0<RetType> {
  fn inputItemRectangle_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_inputItemRectangle_0<usize> for () {
  fn inputItemRectangle_0(self , rsthis: & QInputMethod) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputMethod18inputItemRectangleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInputItemRectangle(const QRectF &)

/*
Sets the input item's geometry to be rect, in input item coordinates. This needs to be updated by the focused window like QQuickCanvas whenever item is moved inside the scene, or focus is changed.

This function was introduced in  Qt 5.1.

See also inputItemRectangle().
*/
impl /*struct*/ QInputMethod {
  pub fn setInputItemRectangle_0<RetType, T: QInputMethod_setInputItemRectangle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInputItemRectangle_0(self);
    // return 1;
  }
}
pub trait QInputMethod_setInputItemRectangle_0<RetType> {
  fn setInputItemRectangle_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_setInputItemRectangle_0<(/*void*/)> for (usize) {
  fn setInputItemRectangle_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputMethod21setInputItemRectangleERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:75
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF cursorRectangle() const

/*

*/
impl /*struct*/ QInputMethod {
  pub fn cursorRectangle_0<RetType, T: QInputMethod_cursorRectangle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorRectangle_0(self);
    // return 1;
  }
}
pub trait QInputMethod_cursorRectangle_0<RetType> {
  fn cursorRectangle_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_cursorRectangle_0<usize> for () {
  fn cursorRectangle_0(self , rsthis: & QInputMethod) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputMethod15cursorRectangleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:76
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF anchorRectangle() const

/*

*/
impl /*struct*/ QInputMethod {
  pub fn anchorRectangle_0<RetType, T: QInputMethod_anchorRectangle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.anchorRectangle_0(self);
    // return 1;
  }
}
pub trait QInputMethod_anchorRectangle_0<RetType> {
  fn anchorRectangle_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_anchorRectangle_0<usize> for () {
  fn anchorRectangle_0(self , rsthis: & QInputMethod) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputMethod15anchorRectangleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:79
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF keyboardRectangle() const

/*

*/
impl /*struct*/ QInputMethod {
  pub fn keyboardRectangle_0<RetType, T: QInputMethod_keyboardRectangle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyboardRectangle_0(self);
    // return 1;
  }
}
pub trait QInputMethod_keyboardRectangle_0<RetType> {
  fn keyboardRectangle_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_keyboardRectangle_0<usize> for () {
  fn keyboardRectangle_0(self , rsthis: & QInputMethod) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputMethod17keyboardRectangleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:81
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF inputItemClipRectangle() const

/*

*/
impl /*struct*/ QInputMethod {
  pub fn inputItemClipRectangle_0<RetType, T: QInputMethod_inputItemClipRectangle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputItemClipRectangle_0(self);
    // return 1;
  }
}
pub trait QInputMethod_inputItemClipRectangle_0<RetType> {
  fn inputItemClipRectangle_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_inputItemClipRectangle_0<usize> for () {
  fn inputItemClipRectangle_0(self , rsthis: & QInputMethod) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputMethod22inputItemClipRectangleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:89
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isVisible() const

/*

*/
impl /*struct*/ QInputMethod {
  pub fn isVisible_0<RetType, T: QInputMethod_isVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isVisible_0(self);
    // return 1;
  }
}
pub trait QInputMethod_isVisible_0<RetType> {
  fn isVisible_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_isVisible_0<bool> for () {
  fn isVisible_0(self , rsthis: & QInputMethod) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputMethod9isVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*
Controls the keyboard visibility. Equivalent to calling show() (if visible is true) or hide() (if visible is false).

See also isVisible(), show(), and hide().
*/
impl /*struct*/ QInputMethod {
  pub fn setVisible_0<RetType, T: QInputMethod_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QInputMethod_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputMethod10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:92
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isAnimating() const

/*

*/
impl /*struct*/ QInputMethod {
  pub fn isAnimating_0<RetType, T: QInputMethod_isAnimating_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAnimating_0(self);
    // return 1;
  }
}
pub trait QInputMethod_isAnimating_0<RetType> {
  fn isAnimating_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_isAnimating_0<bool> for () {
  fn isAnimating_0(self , rsthis: & QInputMethod) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputMethod11isAnimatingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:94
// index:0
// Public Visibility=Default Availability=Available
// [8] QLocale locale() const

/*

*/
impl /*struct*/ QInputMethod {
  pub fn locale_0<RetType, T: QInputMethod_locale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.locale_0(self);
    // return 1;
  }
}
pub trait QInputMethod_locale_0<RetType> {
  fn locale_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_locale_0<usize> for () {
  fn locale_0(self , rsthis: & QInputMethod) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputMethod6localeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:95
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::LayoutDirection inputDirection() const

/*

*/
impl /*struct*/ QInputMethod {
  pub fn inputDirection_0<RetType, T: QInputMethod_inputDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputDirection_0(self);
    // return 1;
  }
}
pub trait QInputMethod_inputDirection_0<RetType> {
  fn inputDirection_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_inputDirection_0<i32> for () {
  fn inputDirection_0(self , rsthis: & QInputMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QInputMethod14inputDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:97
// index:0
// Public static Visibility=Default Availability=Available
// [16] QVariant queryFocusObject(Qt::InputMethodQuery, QVariant)

/*
Send query to the current focus object with parameters argument and return the result.
*/
impl /*struct*/ QInputMethod {
  pub fn queryFocusObject_0<RetType, T: QInputMethod_queryFocusObject_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.queryFocusObject_0();
    // return 1;
  }
}
pub trait QInputMethod_queryFocusObject_0<RetType> {
  fn queryFocusObject_0(self ) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_queryFocusObject_0<usize> for (i32,usize) {
  fn queryFocusObject_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QInputMethod16queryFocusObjectEN2Qt16InputMethodQueryE8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void show()

/*
Requests virtual keyboard to open. If the platform doesn't provide virtual keyboard the visibility remains false.

Normally applications should not need to call this function, keyboard should automatically open when the text editor gains focus.
*/
impl /*struct*/ QInputMethod {
  pub fn show_0<RetType, T: QInputMethod_show_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.show_0(self);
    // return 1;
  }
}
pub trait QInputMethod_show_0<RetType> {
  fn show_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_show_0<(/*void*/)> for () {
  fn show_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QInputMethod4showEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void hide()

/*
Requests virtual keyboard to close.

Normally applications should not need to call this function, keyboard should automatically close when the text editor loses focus, for example when the parent view is closed.
*/
impl /*struct*/ QInputMethod {
  pub fn hide_0<RetType, T: QInputMethod_hide_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hide_0(self);
    // return 1;
  }
}
pub trait QInputMethod_hide_0<RetType> {
  fn hide_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_hide_0<(/*void*/)> for () {
  fn hide_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QInputMethod4hideEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void update(Qt::InputMethodQueries)

/*
Called by the input item to inform the platform input methods when there has been state changes in editor's input method query attributes. When calling the function queries parameter has to be used to tell what has changes, which input method can use to make queries for attributes it's interested with QInputMethodQueryEvent.

In particular calling update whenever the cursor position changes is important as that often causes other query attributes like surrounding text and text selection to change as well. The attributes that often change together with cursor position have been grouped in Qt::ImQueryInput value for convenience.
*/
impl /*struct*/ QInputMethod {
  pub fn update_0<RetType, T: QInputMethod_update_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_0(self);
    // return 1;
  }
}
pub trait QInputMethod_update_0<RetType> {
  fn update_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_update_0<(/*void*/)> for (i32) {
  fn update_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputMethod6updateE6QFlagsIN2Qt16InputMethodQueryEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reset()

/*
Resets the input method state. For example, a text editor normally calls this method before inserting a text to make widget ready to accept a text.

Input method resets automatically when the focused editor changes.
*/
impl /*struct*/ QInputMethod {
  pub fn reset_0<RetType, T: QInputMethod_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QInputMethod_reset_0<RetType> {
  fn reset_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QInputMethod5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void commit()

/*
Commits the word user is currently composing to the editor. The function is mostly needed by the input methods with text prediction features and by the methods where the script used for typing characters is different from the script that actually gets appended to the editor. Any kind of action that interrupts the text composing needs to flush the composing state by calling the commit() function, for example when the cursor is moved elsewhere.
*/
impl /*struct*/ QInputMethod {
  pub fn commit_0<RetType, T: QInputMethod_commit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.commit_0(self);
    // return 1;
  }
}
pub trait QInputMethod_commit_0<RetType> {
  fn commit_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_commit_0<(/*void*/)> for () {
  fn commit_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QInputMethod6commitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void invokeAction(QInputMethod::Action, int)

/*
Called by the input item when the word currently being composed is tapped by the user, as indicated by the action a and the given cursorPosition. Input methods often use this information to offer more word suggestions to the user.
*/
impl /*struct*/ QInputMethod {
  pub fn invokeAction_0<RetType, T: QInputMethod_invokeAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invokeAction_0(self);
    // return 1;
  }
}
pub trait QInputMethod_invokeAction_0<RetType> {
  fn invokeAction_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_invokeAction_0<(/*void*/)> for (i32,i32) {
  fn invokeAction_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputMethod12invokeActionENS_6ActionEi", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cursorRectangleChanged()

/*

*/
impl /*struct*/ QInputMethod {
  pub fn cursorRectangleChanged_0<RetType, T: QInputMethod_cursorRectangleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorRectangleChanged_0(self);
    // return 1;
  }
}
pub trait QInputMethod_cursorRectangleChanged_0<RetType> {
  fn cursorRectangleChanged_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_cursorRectangleChanged_0<(/*void*/)> for () {
  fn cursorRectangleChanged_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QInputMethod22cursorRectangleChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void anchorRectangleChanged()

/*

*/
impl /*struct*/ QInputMethod {
  pub fn anchorRectangleChanged_0<RetType, T: QInputMethod_anchorRectangleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.anchorRectangleChanged_0(self);
    // return 1;
  }
}
pub trait QInputMethod_anchorRectangleChanged_0<RetType> {
  fn anchorRectangleChanged_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_anchorRectangleChanged_0<(/*void*/)> for () {
  fn anchorRectangleChanged_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QInputMethod22anchorRectangleChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void keyboardRectangleChanged()

/*

*/
impl /*struct*/ QInputMethod {
  pub fn keyboardRectangleChanged_0<RetType, T: QInputMethod_keyboardRectangleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyboardRectangleChanged_0(self);
    // return 1;
  }
}
pub trait QInputMethod_keyboardRectangleChanged_0<RetType> {
  fn keyboardRectangleChanged_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_keyboardRectangleChanged_0<(/*void*/)> for () {
  fn keyboardRectangleChanged_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QInputMethod24keyboardRectangleChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void inputItemClipRectangleChanged()

/*

*/
impl /*struct*/ QInputMethod {
  pub fn inputItemClipRectangleChanged_0<RetType, T: QInputMethod_inputItemClipRectangleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputItemClipRectangleChanged_0(self);
    // return 1;
  }
}
pub trait QInputMethod_inputItemClipRectangleChanged_0<RetType> {
  fn inputItemClipRectangleChanged_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_inputItemClipRectangleChanged_0<(/*void*/)> for () {
  fn inputItemClipRectangleChanged_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QInputMethod29inputItemClipRectangleChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void visibleChanged()

/*

*/
impl /*struct*/ QInputMethod {
  pub fn visibleChanged_0<RetType, T: QInputMethod_visibleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visibleChanged_0(self);
    // return 1;
  }
}
pub trait QInputMethod_visibleChanged_0<RetType> {
  fn visibleChanged_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_visibleChanged_0<(/*void*/)> for () {
  fn visibleChanged_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QInputMethod14visibleChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void animatingChanged()

/*

*/
impl /*struct*/ QInputMethod {
  pub fn animatingChanged_0<RetType, T: QInputMethod_animatingChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.animatingChanged_0(self);
    // return 1;
  }
}
pub trait QInputMethod_animatingChanged_0<RetType> {
  fn animatingChanged_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_animatingChanged_0<(/*void*/)> for () {
  fn animatingChanged_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QInputMethod16animatingChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void localeChanged()

/*

*/
impl /*struct*/ QInputMethod {
  pub fn localeChanged_0<RetType, T: QInputMethod_localeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.localeChanged_0(self);
    // return 1;
  }
}
pub trait QInputMethod_localeChanged_0<RetType> {
  fn localeChanged_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_localeChanged_0<(/*void*/)> for () {
  fn localeChanged_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QInputMethod13localeChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qinputmethod.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void inputDirectionChanged(Qt::LayoutDirection)

/*

*/
impl /*struct*/ QInputMethod {
  pub fn inputDirectionChanged_0<RetType, T: QInputMethod_inputDirectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputDirectionChanged_0(self);
    // return 1;
  }
}
pub trait QInputMethod_inputDirectionChanged_0<RetType> {
  fn inputDirectionChanged_0(self , rsthis: & QInputMethod) -> RetType;
}
impl<'a> /*trait*/ QInputMethod_inputDirectionChanged_0<(/*void*/)> for (i32) {
  fn inputDirectionChanged_0(self , rsthis: & QInputMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QInputMethod21inputDirectionChangedEN2Qt15LayoutDirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQInputMethod(this :*mut QInputMethod) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN12QInputMethodD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
Indicates the kind of action performed by the user.



See also invokeAction().

*/
pub type QInputMethod__Action = i32;
// A normal click/tap
pub const QInputMethod__Click :QInputMethod__Action = 0;
// A context menu click/tap (e.g. right-button or tap-and-hold)
pub const QInputMethod__ContextMenu :QInputMethod__Action = 1;
pub fn QInputMethod_ActionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QInputMethod", val);
}
pub fn QInputMethod_ActionItemName_s(val: i32) ->String {
  //var nilthis *QInputMethod
  //return nilthis.ActionItemName(val);
  return QInputMethod_ActionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
