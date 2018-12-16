

// mod ::widgets::QStyle
// package qtwidgets
// /usr/include/qt/QtWidgets/qstyle.h
// #include <qstyle.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 16
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



/*

*/
#[derive(Default)] // class sizeof(QStyle)=16
pub struct QStyle {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyle_ITF interface {
//    qtcore.QObject_ITF
//    QStyle_PTR() *QStyle
//}
//func (ptr *QStyle) QStyle_PTR() *QStyle { return ptr }

impl /*struct*/ QStyle {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyle {
    return QStyle{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyle {
//  type Target = QStyleBASE;
//
//  fn deref(&self) -> &QStyleBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleBASE> for QStyle {
//  fn as_ref(& self) -> & QStyleBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyle.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QStyle {
  pub fn metaObject_0<RetType, T: QStyle_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QStyle_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyle()

/*
Constructs a style object.
*/
// QStyle() ctx.fn_proto_cpp
impl /*struct*/ QStyle {
  pub fn QStyle_0<T: QStyle_QStyle_0>(value: T) -> QStyle {
    let rsthis = value.QStyle_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyle_QStyle_0 {
  fn QStyle_0(self) -> QStyle;
}
// QStyle() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyle_QStyle_0 for () {
  fn QStyle_0(self) -> QStyle {
    // unsafe{_ZN6QStyleC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QStyleC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyle{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QStyle()

/*

*/
pub fn DeleteQStyle(this :*mut QStyle) {
    // let rv = qtrt::InvokeQtFunc6("_ZN6QStyleD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qstyle.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void polish(QWidget *)

/*
Initializes the appearance of the given widget.

This function is called for every widget at some point after it has been fully created but just before it is shown for the very first time.

Note that the default implementation does nothing. Reasonable actions in this function might be to call the QWidget::setBackgroundMode() function for the widget. Do not use the function to set, for example, the geometry. Reimplementing this function provides a back-door through which the appearance of a widget can be changed, but with Qt's style engine it is rarely necessary to implement this function; reimplement drawItemPixmap(), drawItemText(), drawPrimitive(), etc. instead.

The QWidget::inherits() function may provide enough information to allow class-specific customizations. But because new QStyle subclasses are expected to work reasonably with all current and future widgets, limited use of hard-coded customization is recommended.

See also unpolish().
*/
impl /*struct*/ QStyle {
  pub fn polish_0<RetType, T: QStyle_polish_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.polish_0(self);
    // return 1;
  }
}
pub trait QStyle_polish_0<RetType> {
  fn polish_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_polish_0<(/*void*/)> for (usize) {
  fn polish_0(self , rsthis: & QStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QStyle6polishEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:79
// index:1
// Public virtual Visibility=Default Availability=Available
// [-2] void polish(QApplication *)

/*
Initializes the appearance of the given widget.

This function is called for every widget at some point after it has been fully created but just before it is shown for the very first time.

Note that the default implementation does nothing. Reasonable actions in this function might be to call the QWidget::setBackgroundMode() function for the widget. Do not use the function to set, for example, the geometry. Reimplementing this function provides a back-door through which the appearance of a widget can be changed, but with Qt's style engine it is rarely necessary to implement this function; reimplement drawItemPixmap(), drawItemText(), drawPrimitive(), etc. instead.

The QWidget::inherits() function may provide enough information to allow class-specific customizations. But because new QStyle subclasses are expected to work reasonably with all current and future widgets, limited use of hard-coded customization is recommended.

See also unpolish().
*/
impl /*struct*/ QStyle {
  pub fn polish_1<RetType, T: QStyle_polish_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.polish_1(self);
    // return 1;
  }
}
pub trait QStyle_polish_1<RetType> {
  fn polish_1(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_polish_1<(/*void*/)> for (usize) {
  fn polish_1(self , rsthis: & QStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QStyle6polishEP12QApplication", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:82
// index:2
// Public virtual Visibility=Default Availability=Available
// [-2] void polish(QPalette &)

/*
Initializes the appearance of the given widget.

This function is called for every widget at some point after it has been fully created but just before it is shown for the very first time.

Note that the default implementation does nothing. Reasonable actions in this function might be to call the QWidget::setBackgroundMode() function for the widget. Do not use the function to set, for example, the geometry. Reimplementing this function provides a back-door through which the appearance of a widget can be changed, but with Qt's style engine it is rarely necessary to implement this function; reimplement drawItemPixmap(), drawItemText(), drawPrimitive(), etc. instead.

The QWidget::inherits() function may provide enough information to allow class-specific customizations. But because new QStyle subclasses are expected to work reasonably with all current and future widgets, limited use of hard-coded customization is recommended.

See also unpolish().
*/
impl /*struct*/ QStyle {
  pub fn polish_2<RetType, T: QStyle_polish_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.polish_2(self);
    // return 1;
  }
}
pub trait QStyle_polish_2<RetType> {
  fn polish_2(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_polish_2<(/*void*/)> for (usize) {
  fn polish_2(self , rsthis: & QStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QStyle6polishER8QPalette", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:77
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void unpolish(QWidget *)

/*
Uninitialize the given widget's appearance.

This function is the counterpart to polish(). It is called for every polished widget whenever the style is dynamically changed; the former style has to unpolish its settings before the new style can polish them again.

Note that unpolish() will only be called if the widget is destroyed. This can cause problems in some cases, e.g, if you remove a widget from the UI, cache it, and then reinsert it after the style has changed; some of Qt's classes cache their widgets.

See also polish().
*/
impl /*struct*/ QStyle {
  pub fn unpolish_0<RetType, T: QStyle_unpolish_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unpolish_0(self);
    // return 1;
  }
}
pub trait QStyle_unpolish_0<RetType> {
  fn unpolish_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_unpolish_0<(/*void*/)> for (usize) {
  fn unpolish_0(self , rsthis: & QStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QStyle8unpolishEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:80
// index:1
// Public virtual Visibility=Default Availability=Available
// [-2] void unpolish(QApplication *)

/*
Uninitialize the given widget's appearance.

This function is the counterpart to polish(). It is called for every polished widget whenever the style is dynamically changed; the former style has to unpolish its settings before the new style can polish them again.

Note that unpolish() will only be called if the widget is destroyed. This can cause problems in some cases, e.g, if you remove a widget from the UI, cache it, and then reinsert it after the style has changed; some of Qt's classes cache their widgets.

See also polish().
*/
impl /*struct*/ QStyle {
  pub fn unpolish_1<RetType, T: QStyle_unpolish_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unpolish_1(self);
    // return 1;
  }
}
pub trait QStyle_unpolish_1<RetType> {
  fn unpolish_1(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_unpolish_1<(/*void*/)> for (usize) {
  fn unpolish_1(self , rsthis: & QStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QStyle8unpolishEP12QApplication", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:84
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect itemTextRect(const QFontMetrics &, const QRect &, int, bool, const QString &) const

/*
Returns the area within the given rectangle in which to draw the provided text according to the specified font metrics and alignment. The enabled parameter indicates whether or not the associated item is enabled.

If the given rectangle is larger than the area needed to render the text, the rectangle that is returned will be offset within rectangle according to the specified alignment. For example, if alignment is Qt::AlignCenter, the returned rectangle will be centered within rectangle. If the given rectangle is smaller than the area needed, the returned rectangle will be the smallest rectangle large enough to render the text.

See also Qt::Alignment.
*/
impl /*struct*/ QStyle {
  pub fn itemTextRect_0<RetType, T: QStyle_itemTextRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemTextRect_0(self);
    // return 1;
  }
}
pub trait QStyle_itemTextRect_0<RetType> {
  fn itemTextRect_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_itemTextRect_0<usize> for (usize,usize,i32,bool,usize) {
  fn itemTextRect_0(self , rsthis: & QStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const bool as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_SINT8,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:88
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect itemPixmapRect(const QRect &, int, const QPixmap &) const

/*
Returns the area within the given rectangle in which to draw the specified pixmap according to the defined alignment.
*/
impl /*struct*/ QStyle {
  pub fn itemPixmapRect_0<RetType, T: QStyle_itemPixmapRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemPixmapRect_0(self);
    // return 1;
  }
}
pub trait QStyle_itemPixmapRect_0<RetType> {
  fn itemPixmapRect_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_itemPixmapRect_0<usize> for (usize,i32,usize) {
  fn itemPixmapRect_0(self , rsthis: & QStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle14itemPixmapRectERK5QRectiRK7QPixmap", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:90
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawItemText(QPainter *, const QRect &, int, const QPalette &, bool, const QString &, QPalette::ColorRole) const

/*
Draws the given text in the specified rectangle using the provided painter and palette.

The text is drawn using the painter's pen, and aligned and wrapped according to the specified alignment. If an explicit textRole is specified, the text is drawn using the palette's color for the given role. The enabled parameter indicates whether or not the item is enabled; when reimplementing this function, the enabled parameter should influence how the item is drawn.

See also Qt::Alignment and drawItemPixmap().
*/
impl /*struct*/ QStyle {
  pub fn drawItemText_0<RetType, T: QStyle_drawItemText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawItemText_0(self);
    // return 1;
  }
}
pub trait QStyle_drawItemText_0<RetType> {
  fn drawItemText_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_drawItemText_0<(/*void*/)> for (usize,usize,i32,usize,bool,usize,i32) {
  fn drawItemText_0(self , rsthis: & QStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const bool as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZNK6QStyle12drawItemTextEP8QPainterRK5QRectiRK8QPalettebRK7QStringNS5_9ColorRoleE", 7,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:94
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawItemPixmap(QPainter *, const QRect &, int, const QPixmap &) const

/*
Draws the given pixmap in the specified rectangle, according to the specified alignment, using the provided painter.

See also drawItemText().
*/
impl /*struct*/ QStyle {
  pub fn drawItemPixmap_0<RetType, T: QStyle_drawItemPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawItemPixmap_0(self);
    // return 1;
  }
}
pub trait QStyle_drawItemPixmap_0<RetType> {
  fn drawItemPixmap_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_drawItemPixmap_0<(/*void*/)> for (usize,usize,i32,usize) {
  fn drawItemPixmap_0(self , rsthis: & QStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK6QStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:97
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QPalette standardPalette() const

/*
Returns the style's standard palette.

Note that on systems that support system colors, the style's standard palette is not used. In particular, the Windows Vista and Mac styles do not use the standard palette, but make use of native theme engines. With these styles, you should not set the palette with QApplication::setPalette().

See also QApplication::setPalette().
*/
impl /*struct*/ QStyle {
  pub fn standardPalette_0<RetType, T: QStyle_standardPalette_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standardPalette_0(self);
    // return 1;
  }
}
pub trait QStyle_standardPalette_0<RetType> {
  fn standardPalette_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_standardPalette_0<usize> for () {
  fn standardPalette_0(self , rsthis: & QStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle15standardPaletteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:204
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void drawPrimitive(QStyle::PrimitiveElement, const QStyleOption *, QPainter *, const QWidget *) const

/*
Draws the given primitive element with the provided painter using the style options specified by option.

The widget argument is optional and may contain a widget that may aid in drawing the primitive element.

The table below is listing the primitive elements and their associated style option subclasses. The style options contain all the parameters required to draw the elements, including QStyleOption::state which holds the style flags that are used when drawing. The table also describes which flags that are set when casting the given option to the appropriate subclass.

Note that if a primitive element is not listed here, it is because it uses a plain QStyleOption object.


 Primitive ElementQStyleOption SubclassStyle FlagRemark
PE_FrameFocusRectQStyleOptionFocusRectState_FocusAtBorderWhether the focus is is at the border or inside the widget.
PE_IndicatorCheckBoxQStyleOptionButtonState_NoChangeIndicates a "tri-state" checkbox.
State_OnIndicates the indicator is checked.
PE_IndicatorRadioButtonQStyleOptionButtonState_OnIndicates that a radio button is selected.
State_NoChangeIndicates a "tri-state" controller.
State_EnabledIndicates the controller is enabled.
PE_IndicatorBranchQStyleOptionState_ChildrenIndicates that the control for expanding the tree to show child items, should be drawn.
State_ItemIndicates that a horizontal branch (to show a child item), should be drawn.
State_OpenIndicates that the tree branch is expanded.
State_SiblingIndicates that a vertical line (to show a sibling item), should be drawn.
PE_IndicatorHeaderArrowQStyleOptionHeaderState_UpArrowIndicates that the arrow should be drawn up; otherwise it should be down.
PE_FrameGroupBox, PE_Frame, PE_FrameLineEdit, PE_FrameMenu, PE_FrameDockWidget, PE_FrameWindowQStyleOptionFrameState_SunkenIndicates that the Frame should be sunken.
PE_IndicatorToolBarHandleQStyleOptionState_HorizontalIndicates that the window handle is horizontal instead of vertical.
PE_IndicatorSpinPlus, PE_IndicatorSpinMinus, PE_IndicatorSpinUp, PE_IndicatorSpinDown,QStyleOptionSpinBoxState_SunkenIndicates that the button is pressed.
PE_PanelButtonCommandQStyleOptionButtonState_EnabledSet if the button is enabled.
State_HasFocusSet if the button has input focus.
State_RaisedSet if the button is not down, not on and not flat.
State_OnSet if the button is a toggle button and is toggled on.
State_SunkenSet if the button is down (i.e., the mouse button or the space bar is pressed on the button).


See also drawComplexControl() and drawControl().
*/
impl /*struct*/ QStyle {
  pub fn drawPrimitive_0<RetType, T: QStyle_drawPrimitive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPrimitive_0(self);
    // return 1;
  }
}
pub trait QStyle_drawPrimitive_0<RetType> {
  fn drawPrimitive_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_drawPrimitive_0<(/*void*/)> for (i32,usize,usize,usize) {
  fn drawPrimitive_0(self , rsthis: & QStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK6QStyle13drawPrimitiveENS_16PrimitiveElementEPK12QStyleOptionP8QPainterPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:275
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void drawControl(QStyle::ControlElement, const QStyleOption *, QPainter *, const QWidget *) const

/*
Draws the given element with the provided painter with the style options specified by option.

The widget argument is optional and can be used as aid in drawing the control. The option parameter is a pointer to a QStyleOption object that can be cast to the correct subclass using the qstyleoption_cast() function.

The table below is listing the control elements and their associated style option subclass. The style options contain all the parameters required to draw the controls, including QStyleOption::state which holds the style flags that are used when drawing. The table also describes which flags that are set when casting the given option to the appropriate subclass.

Note that if a control element is not listed here, it is because it uses a plain QStyleOption object.


 Control ElementQStyleOption SubclassStyle FlagRemark
CE_MenuItem, CE_MenuBarItemQStyleOptionMenuItemState_SelectedThe menu item is currently selected item.
State_EnabledThe item is enabled.
State_DownArrowIndicates that a scroll down arrow should be drawn.
State_UpArrowIndicates that a scroll up arrow should be drawn
State_HasFocusSet if the menu bar has input focus.
CE_PushButton, CE_PushButtonBevel, CE_PushButtonLabelQStyleOptionButtonState_EnabledSet if the button is enabled.
State_HasFocusSet if the button has input focus.
State_RaisedSet if the button is not down, not on and not flat.
State_OnSet if the button is a toggle button and is toggled on.
State_SunkenSet if the button is down (i.e., the mouse button or the space bar is pressed on the button).
CE_RadioButton, CE_RadioButtonLabel, CE_CheckBox, CE_CheckBoxLabelQStyleOptionButtonState_EnabledSet if the button is enabled.
State_HasFocusSet if the button has input focus.
State_OnSet if the button is checked.
State_OffSet if the button is not checked.
State_NoChangeSet if the button is in the NoChange state.
State_SunkenSet if the button is down (i.e., the mouse button or the space bar is pressed on the button).
CE_ProgressBarContents, CE_ProgressBarLabel, CE_ProgressBarGrooveQStyleOptionProgressBarState_EnabledSet if the progress bar is enabled.
State_HasFocusSet if the progress bar has input focus.
CE_Header, CE_HeaderSection, CE_HeaderLabelQStyleOptionHeader
CE_TabBarTab, CE_TabBarTabShape, CE_TabBarTabLabelQStyleOptionTabState_EnabledSet if the tab bar is enabled.
State_SelectedThe tab bar is the currently selected tab bar.
State_HasFocusSet if the tab bar tab has input focus.
CE_ToolButtonLabelQStyleOptionToolButtonState_EnabledSet if the tool button is enabled.
State_HasFocusSet if the tool button has input focus.
State_SunkenSet if the tool button is down (i.e., a mouse button or the space bar is pressed).
State_OnSet if the tool button is a toggle button and is toggled on.
State_AutoRaiseSet if the tool button has auto-raise enabled.
State_MouseOverSet if the mouse pointer is over the tool button.
State_RaisedSet if the button is not down and is not on.
CE_ToolBoxTabQStyleOptionToolBoxState_SelectedThe tab is the currently selected tab.
CE_HeaderSectionQStyleOptionHeaderState_SunkenIndicates that the section is pressed.
State_UpArrowIndicates that the sort indicator should be pointing up.
State_DownArrowIndicates that the sort indicator should be pointing down.


See also drawPrimitive() and drawComplexControl().
*/
impl /*struct*/ QStyle {
  pub fn drawControl_0<RetType, T: QStyle_drawControl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawControl_0(self);
    // return 1;
  }
}
pub trait QStyle_drawControl_0<RetType> {
  fn drawControl_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_drawControl_0<(/*void*/)> for (i32,usize,usize,usize) {
  fn drawControl_0(self , rsthis: & QStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK6QStyle11drawControlENS_14ControlElementEPK12QStyleOptionP8QPainterPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:364
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [16] QRect subElementRect(QStyle::SubElement, const QStyleOption *, const QWidget *) const

/*
Returns the sub-area for the given element as described in the provided style option. The returned rectangle is defined in screen coordinates.

The widget argument is optional and can be used to aid determining the area. The QStyleOption object can be cast to the appropriate type using the qstyleoption_cast() function. See the table below for the appropriate option casts:


 Sub ElementQStyleOption Subclass
SE_PushButtonContentsQStyleOptionButton
SE_PushButtonFocusRectQStyleOptionButton
SE_CheckBoxIndicatorQStyleOptionButton
SE_CheckBoxContentsQStyleOptionButton
SE_CheckBoxFocusRectQStyleOptionButton
SE_RadioButtonIndicatorQStyleOptionButton
SE_RadioButtonContentsQStyleOptionButton
SE_RadioButtonFocusRectQStyleOptionButton
SE_ComboBoxFocusRectQStyleOptionComboBox
SE_ProgressBarGrooveQStyleOptionProgressBar
SE_ProgressBarContentsQStyleOptionProgressBar
SE_ProgressBarLabelQStyleOptionProgressBar
*/
impl /*struct*/ QStyle {
  pub fn subElementRect_0<RetType, T: QStyle_subElementRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subElementRect_0(self);
    // return 1;
  }
}
pub trait QStyle_subElementRect_0<RetType> {
  fn subElementRect_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_subElementRect_0<usize> for (i32,usize,usize) {
  fn subElementRect_0(self , rsthis: & QStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle14subElementRectENS_10SubElementEPK12QStyleOptionPK7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:443
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void drawComplexControl(QStyle::ComplexControl, const QStyleOptionComplex *, QPainter *, const QWidget *) const

/*
Draws the given control using the provided painter with the style options specified by option.

The widget argument is optional and can be used as aid in drawing the control.

The option parameter is a pointer to a QStyleOptionComplex object that can be cast to the correct subclass using the qstyleoption_cast() function. Note that the rect member of the specified option must be in logical coordinates. Reimplementations of this function should use visualRect() to change the logical coordinates into screen coordinates before calling the drawPrimitive() or drawControl() function.

The table below is listing the complex control elements and their associated style option subclass. The style options contain all the parameters required to draw the controls, including QStyleOption::state which holds the style flags that are used when drawing. The table also describes which flags that are set when casting the given option to the appropriate subclass.


 Complex ControlQStyleOptionComplex SubclassStyle FlagRemark
CC_SpinBoxQStyleOptionSpinBoxState_EnabledSet if the spin box is enabled.
State_HasFocusSet if the spin box has input focus.
CC_ComboBoxQStyleOptionComboBoxState_EnabledSet if the combobox is enabled.
State_HasFocusSet if the combobox has input focus.
CC_ScrollBarQStyleOptionSliderState_EnabledSet if the scroll bar is enabled.
State_HasFocusSet if the scroll bar has input focus.
CC_SliderQStyleOptionSliderState_EnabledSet if the slider is enabled.
State_HasFocusSet if the slider has input focus.
CC_DialQStyleOptionSliderState_EnabledSet if the dial is enabled.
State_HasFocusSet if the dial has input focus.
CC_ToolButtonQStyleOptionToolButtonState_EnabledSet if the tool button is enabled.
State_HasFocusSet if the tool button has input focus.
State_DownArrowSet if the tool button is down (i.e., a mouse button or the space bar is pressed).
State_OnSet if the tool button is a toggle button and is toggled on.
State_AutoRaiseSet if the tool button has auto-raise enabled.
State_RaisedSet if the button is not down, not on, and doesn't contain the mouse when auto-raise is enabled.
CC_TitleBarQStyleOptionTitleBarState_EnabledSet if the title bar is enabled.


See also drawPrimitive() and drawControl().
*/
impl /*struct*/ QStyle {
  pub fn drawComplexControl_0<RetType, T: QStyle_drawComplexControl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawComplexControl_0(self);
    // return 1;
  }
}
pub trait QStyle_drawComplexControl_0<RetType> {
  fn drawComplexControl_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_drawComplexControl_0<(/*void*/)> for (i32,usize,usize,usize) {
  fn drawComplexControl_0(self , rsthis: & QStyle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK6QStyle18drawComplexControlENS_14ComplexControlEPK19QStyleOptionComplexP8QPainterPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:445
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] QStyle::SubControl hitTestComplexControl(QStyle::ComplexControl, const QStyleOptionComplex *, const QPoint &, const QWidget *) const

/*
Returns the sub control at the given position in the given complex control (with the style options specified by option).

Note that the position is expressed in screen coordinates.

The option argument is a pointer to a QStyleOptionComplex object (or one of its subclasses). The object can be cast to the appropriate type using the qstyleoption_cast() function. See drawComplexControl() for details. The widget argument is optional and can contain additional information for the function.

See also drawComplexControl() and subControlRect().
*/
impl /*struct*/ QStyle {
  pub fn hitTestComplexControl_0<RetType, T: QStyle_hitTestComplexControl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hitTestComplexControl_0(self);
    // return 1;
  }
}
pub trait QStyle_hitTestComplexControl_0<RetType> {
  fn hitTestComplexControl_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_hitTestComplexControl_0<i32> for (i32,usize,usize,usize) {
  fn hitTestComplexControl_0(self , rsthis: & QStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle21hitTestComplexControlENS_14ComplexControlEPK19QStyleOptionComplexRK6QPointPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:447
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [16] QRect subControlRect(QStyle::ComplexControl, const QStyleOptionComplex *, QStyle::SubControl, const QWidget *) const

/*
Returns the rectangle containing the specified subControl of the given complex control (with the style specified by option). The rectangle is defined in screen coordinates.

The option argument is a pointer to QStyleOptionComplex or one of its subclasses, and can be cast to the appropriate type using the qstyleoption_cast() function. See drawComplexControl() for details. The widget is optional and can contain additional information for the function.

See also drawComplexControl().
*/
impl /*struct*/ QStyle {
  pub fn subControlRect_0<RetType, T: QStyle_subControlRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subControlRect_0(self);
    // return 1;
  }
}
pub trait QStyle_subControlRect_0<RetType> {
  fn subControlRect_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_subControlRect_0<usize> for (i32,usize,i32,usize) {
  fn subControlRect_0(self , rsthis: & QStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle14subControlRectENS_14ComplexControlEPK19QStyleOptionComplexNS_10SubControlEPK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:582
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int pixelMetric(QStyle::PixelMetric, const QStyleOption *, const QWidget *) const

/*
Returns the value of the given pixel metric.

The specified option and widget can be used for calculating the metric. In general, the widget argument is not used. The option can be cast to the appropriate type using the qstyleoption_cast() function. Note that the option may be zero even for PixelMetrics that can make use of it. See the table below for the appropriate option casts:


 Pixel MetricQStyleOption Subclass
PM_SliderControlThicknessQStyleOptionSlider
PM_SliderLengthQStyleOptionSlider
PM_SliderTickmarkOffsetQStyleOptionSlider
PM_SliderSpaceAvailableQStyleOptionSlider
PM_ScrollBarExtentQStyleOptionSlider
PM_TabBarTabOverlapQStyleOptionTab
PM_TabBarTabHSpaceQStyleOptionTab
PM_TabBarTabVSpaceQStyleOptionTab
PM_TabBarBaseHeightQStyleOptionTab
PM_TabBarBaseOverlapQStyleOptionTab


Some pixel metrics are called from widgets and some are only called internally by the style. If the metric is not called by a widget, it is the discretion of the style author to make use of it. For some styles, this may not be appropriate.
*/
impl /*struct*/ QStyle {
  pub fn pixelMetric_0<RetType, T: QStyle_pixelMetric_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixelMetric_0(self);
    // return 1;
  }
}
pub trait QStyle_pixelMetric_0<RetType> {
  fn pixelMetric_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_pixelMetric_0<i32> for (i32,usize,usize) {
  fn pixelMetric_0(self , rsthis: & QStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle11pixelMetricENS_11PixelMetricEPK12QStyleOptionPK7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:614
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QSize sizeFromContents(QStyle::ContentsType, const QStyleOption *, const QSize &, const QWidget *) const

/*
Returns the size of the element described by the specified option and type, based on the provided contentsSize.

The option argument is a pointer to a QStyleOption or one of its subclasses. The option can be cast to the appropriate type using the qstyleoption_cast() function. The widget is an optional argument and can contain extra information used for calculating the size.

See the table below for the appropriate option casts:


 Contents TypeQStyleOption Subclass
CT_CheckBoxQStyleOptionButton
CT_ComboBoxQStyleOptionComboBox
CT_GroupBoxQStyleOptionGroupBox
CT_HeaderSectionQStyleOptionHeader
CT_ItemViewItemQStyleOptionViewItem
CT_LineEditQStyleOptionFrame
CT_MdiControlsQStyleOptionComplex
CT_MenuQStyleOption
CT_MenuItemQStyleOptionMenuItem
CT_MenuBarQStyleOptionMenuItem
CT_MenuBarItemQStyleOptionMenuItem
CT_ProgressBarQStyleOptionProgressBar
CT_PushButtonQStyleOptionButton
CT_RadioButtonQStyleOptionButton
CT_ScrollBarQStyleOptionSlider
CT_SizeGripQStyleOption
CT_SliderQStyleOptionSlider
CT_SpinBoxQStyleOptionSpinBox
CT_SplitterQStyleOption
CT_TabBarTabQStyleOptionTab
CT_TabWidgetQStyleOptionTabWidgetFrame
CT_ToolButtonQStyleOptionToolButton


See also ContentsType and QStyleOption.
*/
impl /*struct*/ QStyle {
  pub fn sizeFromContents_0<RetType, T: QStyle_sizeFromContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeFromContents_0(self);
    // return 1;
  }
}
pub trait QStyle_sizeFromContents_0<RetType> {
  fn sizeFromContents_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_sizeFromContents_0<usize> for (i32,usize,usize,usize) {
  fn sizeFromContents_0(self , rsthis: & QStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle16sizeFromContentsENS_12ContentsTypeEPK12QStyleOptionRK5QSizePK7QWidget", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:748
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int styleHint(QStyle::StyleHint, const QStyleOption *, const QWidget *, QStyleHintReturn *) const

/*
Returns an integer representing the specified style hint for the given widget described by the provided style option.

returnData is used when the querying widget needs more detailed data than the integer that styleHint() returns. See the QStyleHintReturn class description for details.
*/
impl /*struct*/ QStyle {
  pub fn styleHint_0<RetType, T: QStyle_styleHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styleHint_0(self);
    // return 1;
  }
}
pub trait QStyle_styleHint_0<RetType> {
  fn styleHint_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_styleHint_0<i32> for (i32,usize,usize,usize) {
  fn styleHint_0(self , rsthis: & QStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle9styleHintENS_9StyleHintEPK12QStyleOptionPK7QWidgetP16QStyleHintReturn", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:828
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [32] QPixmap standardPixmap(QStyle::StandardPixmap, const QStyleOption *, const QWidget *) const

/*

*/
impl /*struct*/ QStyle {
  pub fn standardPixmap_0<RetType, T: QStyle_standardPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standardPixmap_0(self);
    // return 1;
  }
}
pub trait QStyle_standardPixmap_0<RetType> {
  fn standardPixmap_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_standardPixmap_0<usize> for (i32,usize,usize) {
  fn standardPixmap_0(self , rsthis: & QStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle14standardPixmapENS_14StandardPixmapEPK12QStyleOptionPK7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:831
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QIcon standardIcon(QStyle::StandardPixmap, const QStyleOption *, const QWidget *) const

/*
Returns an icon for the given standardIcon.

The standardIcon is a standard pixmap which can follow some existing GUI style or guideline. The option argument can be used to pass extra information required when defining the appropriate icon. The widget argument is optional and can also be used to aid the determination of the icon.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QStyle {
  pub fn standardIcon_0<RetType, T: QStyle_standardIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standardIcon_0(self);
    // return 1;
  }
}
pub trait QStyle_standardIcon_0<RetType> {
  fn standardIcon_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_standardIcon_0<usize> for (i32,usize,usize) {
  fn standardIcon_0(self , rsthis: & QStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle12standardIconENS_14StandardPixmapEPK12QStyleOptionPK7QWidget", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:834
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [32] QPixmap generatedIconPixmap(QIcon::Mode, const QPixmap &, const QStyleOption *) const

/*
Returns a copy of the given pixmap, styled to conform to the specified iconMode and taking into account the palette specified by option.

The option parameter can pass extra information, but it must contain a palette.

Note that not all pixmaps will conform, in which case the returned pixmap is a plain copy.

See also QIcon.
*/
impl /*struct*/ QStyle {
  pub fn generatedIconPixmap_0<RetType, T: QStyle_generatedIconPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.generatedIconPixmap_0(self);
    // return 1;
  }
}
pub trait QStyle_generatedIconPixmap_0<RetType> {
  fn generatedIconPixmap_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_generatedIconPixmap_0<usize> for (i32,usize,usize) {
  fn generatedIconPixmap_0(self , rsthis: & QStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle19generatedIconPixmapEN5QIcon4ModeERK7QPixmapPK12QStyleOption", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:837
// index:0
// Public static Visibility=Default Availability=Available
// [16] QRect visualRect(Qt::LayoutDirection, const QRect &, const QRect &)

/*
Returns the given logicalRectangle converted to screen coordinates based on the specified direction. The boundingRectangle is used when performing the translation.

This function is provided to support right-to-left desktops, and is typically used in implementations of the subControlRect() function.

See also QWidget::layoutDirection.
*/
impl /*struct*/ QStyle {
  pub fn visualRect_0<RetType, T: QStyle_visualRect_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.visualRect_0();
    // return 1;
  }
}
pub trait QStyle_visualRect_0<RetType> {
  fn visualRect_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStyle_visualRect_0<usize> for (i32,usize,usize) {
  fn visualRect_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QStyle10visualRectEN2Qt15LayoutDirectionERK5QRectS4_", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:839
// index:0
// Public static Visibility=Default Availability=Available
// [8] QPoint visualPos(Qt::LayoutDirection, const QRect &, const QPoint &)

/*
Returns the given logicalPosition converted to screen coordinates based on the specified direction. The boundingRectangle is used when performing the translation.

See also QWidget::layoutDirection.
*/
impl /*struct*/ QStyle {
  pub fn visualPos_0<RetType, T: QStyle_visualPos_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.visualPos_0();
    // return 1;
  }
}
pub trait QStyle_visualPos_0<RetType> {
  fn visualPos_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStyle_visualPos_0<usize> for (i32,usize,usize) {
  fn visualPos_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QStyle9visualPosEN2Qt15LayoutDirectionERK5QRectRK6QPoint", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:841
// index:0
// Public static Visibility=Default Availability=Available
// [4] int sliderPositionFromValue(int, int, int, int, bool)

/*
Converts the given logicalValue to a pixel position. The min parameter maps to 0, max maps to span and other values are distributed evenly in-between.

This function can handle the entire integer range without overflow, providing that span is less than 4096.

By default, this function assumes that the maximum value is on the right for horizontal items and on the bottom for vertical items. Set the upsideDown parameter to true to reverse this behavior.

See also sliderValueFromPosition().
*/
impl /*struct*/ QStyle {
  pub fn sliderPositionFromValue_0<RetType, T: QStyle_sliderPositionFromValue_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.sliderPositionFromValue_0();
    // return 1;
  }
}
pub trait QStyle_sliderPositionFromValue_0<RetType> {
  fn sliderPositionFromValue_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStyle_sliderPositionFromValue_0<i32> for (i32,i32,i32,i32,bool) {
  fn sliderPositionFromValue_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QStyle23sliderPositionFromValueEiiiib", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:843
// index:0
// Public static Visibility=Default Availability=Available
// [4] int sliderValueFromPosition(int, int, int, int, bool)

/*
Converts the given pixel position to a logical value. 0 maps to the min parameter, span maps to max and other values are distributed evenly in-between.

This function can handle the entire integer range without overflow.

By default, this function assumes that the maximum value is on the right for horizontal items and on the bottom for vertical items. Set the upsideDown parameter to true to reverse this behavior.

See also sliderPositionFromValue().
*/
impl /*struct*/ QStyle {
  pub fn sliderValueFromPosition_0<RetType, T: QStyle_sliderValueFromPosition_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.sliderValueFromPosition_0();
    // return 1;
  }
}
pub trait QStyle_sliderValueFromPosition_0<RetType> {
  fn sliderValueFromPosition_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStyle_sliderValueFromPosition_0<i32> for (i32,i32,i32,i32,bool) {
  fn sliderValueFromPosition_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QStyle23sliderValueFromPositionEiiiib", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:845
// index:0
// Public static Visibility=Default Availability=Available
// [4] Qt::Alignment visualAlignment(Qt::LayoutDirection, Qt::Alignment)

/*
Transforms an alignment of Qt::AlignLeft or Qt::AlignRight without Qt::AlignAbsolute into Qt::AlignLeft or Qt::AlignRight with Qt::AlignAbsolute according to the layout direction. The other alignment flags are left untouched.

If no horizontal alignment was specified, the function returns the default alignment for the given layout direction.

QWidget::layoutDirection
*/
impl /*struct*/ QStyle {
  pub fn visualAlignment_0<RetType, T: QStyle_visualAlignment_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.visualAlignment_0();
    // return 1;
  }
}
pub trait QStyle_visualAlignment_0<RetType> {
  fn visualAlignment_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStyle_visualAlignment_0<i32> for (i32,i32) {
  fn visualAlignment_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QStyle15visualAlignmentEN2Qt15LayoutDirectionE6QFlagsINS0_13AlignmentFlagEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:846
// index:0
// Public static Visibility=Default Availability=Available
// [16] QRect alignedRect(Qt::LayoutDirection, Qt::Alignment, const QSize &, const QRect &)

/*
Returns a new rectangle of the specified size that is aligned to the given rectangle according to the specified alignment and direction.
*/
impl /*struct*/ QStyle {
  pub fn alignedRect_0<RetType, T: QStyle_alignedRect_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.alignedRect_0();
    // return 1;
  }
}
pub trait QStyle_alignedRect_0<RetType> {
  fn alignedRect_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStyle_alignedRect_0<usize> for (i32,i32,usize,usize) {
  fn alignedRect_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QStyle11alignedRectEN2Qt15LayoutDirectionE6QFlagsINS0_13AlignmentFlagEERK5QSizeRK5QRect", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:849
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int layoutSpacing(QSizePolicy::ControlType, QSizePolicy::ControlType, Qt::Orientation, const QStyleOption *, const QWidget *) const

/*
Returns the spacing that should be used between control1 and control2 in a layout. orientation specifies whether the controls are laid out side by side or stacked vertically. The option parameter can be used to pass extra information about the parent widget. The widget parameter is optional and can also be used if option is 0.

This function is called by the layout system. It is used only if PM_LayoutHorizontalSpacing or PM_LayoutVerticalSpacing returns a negative value.

This function was introduced in  Qt 4.3.

See also combinedLayoutSpacing().
*/
impl /*struct*/ QStyle {
  pub fn layoutSpacing_0<RetType, T: QStyle_layoutSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layoutSpacing_0(self);
    // return 1;
  }
}
pub trait QStyle_layoutSpacing_0<RetType> {
  fn layoutSpacing_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_layoutSpacing_0<i32> for (i32,i32,i32,usize,usize) {
  fn layoutSpacing_0(self , rsthis: & QStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle13layoutSpacingEN11QSizePolicy11ControlTypeES1_N2Qt11OrientationEPK12QStyleOptionPK7QWidget", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:852
// index:0
// Public Visibility=Default Availability=Available
// [4] int combinedLayoutSpacing(QSizePolicy::ControlTypes, QSizePolicy::ControlTypes, Qt::Orientation, QStyleOption *, QWidget *) const

/*
Returns the spacing that should be used between controls1 and controls2 in a layout. orientation specifies whether the controls are laid out side by side or stacked vertically. The option parameter can be used to pass extra information about the parent widget. The widget parameter is optional and can also be used if option is 0.

controls1 and controls2 are OR-combination of zero or more control types.

This function is called by the layout system. It is used only if PM_LayoutHorizontalSpacing or PM_LayoutVerticalSpacing returns a negative value.

This function was introduced in  Qt 4.3.

See also layoutSpacing().
*/
impl /*struct*/ QStyle {
  pub fn combinedLayoutSpacing_0<RetType, T: QStyle_combinedLayoutSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.combinedLayoutSpacing_0(self);
    // return 1;
  }
}
pub trait QStyle_combinedLayoutSpacing_0<RetType> {
  fn combinedLayoutSpacing_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_combinedLayoutSpacing_0<i32> for (i32,i32,i32,usize,usize) {
  fn combinedLayoutSpacing_0(self , rsthis: & QStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle21combinedLayoutSpacingE6QFlagsIN11QSizePolicy11ControlTypeEES3_N2Qt11OrientationEP12QStyleOptionP7QWidget", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstyle.h:856
// index:0
// Public Visibility=Default Availability=Available
// [8] const QStyle * proxy() const

/*
This function returns the current proxy for this style. By default most styles will return themselves. However when a proxy style is in use, it will allow the style to call back into its proxy.

This function was introduced in  Qt 4.6.
*/
impl /*struct*/ QStyle {
  pub fn proxy_0<RetType, T: QStyle_proxy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.proxy_0(self);
    // return 1;
  }
}
pub trait QStyle_proxy_0<RetType> {
  fn proxy_0(self , rsthis: & QStyle) -> RetType;
}
impl<'a> /*trait*/ QStyle_proxy_0<usize> for () {
  fn proxy_0(self , rsthis: & QStyle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QStyle5proxyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*


*/
pub type QStyle__StateFlag = i32;
// 
pub const QStyle__State_None :QStyle__StateFlag = 0;
// 
pub const QStyle__State_Enabled :QStyle__StateFlag = 1;
// 
pub const QStyle__State_Raised :QStyle__StateFlag = 2;
// 
pub const QStyle__State_Sunken :QStyle__StateFlag = 4;
// 
pub const QStyle__State_Off :QStyle__StateFlag = 8;
// 
pub const QStyle__State_NoChange :QStyle__StateFlag = 16;
// 
pub const QStyle__State_On :QStyle__StateFlag = 32;
// 
pub const QStyle__State_DownArrow :QStyle__StateFlag = 64;
// 
pub const QStyle__State_Horizontal :QStyle__StateFlag = 128;
// 
pub const QStyle__State_HasFocus :QStyle__StateFlag = 256;
// 
pub const QStyle__State_Top :QStyle__StateFlag = 512;
// 
pub const QStyle__State_Bottom :QStyle__StateFlag = 1024;
// 
pub const QStyle__State_FocusAtBorder :QStyle__StateFlag = 2048;
// 
pub const QStyle__State_AutoRaise :QStyle__StateFlag = 4096;
// 
pub const QStyle__State_MouseOver :QStyle__StateFlag = 8192;
// 
pub const QStyle__State_UpArrow :QStyle__StateFlag = 16384;
// 
pub const QStyle__State_Selected :QStyle__StateFlag = 32768;
// 
pub const QStyle__State_Active :QStyle__StateFlag = 65536;
// 
pub const QStyle__State_Window :QStyle__StateFlag = 131072;
// 
pub const QStyle__State_Open :QStyle__StateFlag = 262144;
// 
pub const QStyle__State_Children :QStyle__StateFlag = 524288;
// 
pub const QStyle__State_Item :QStyle__StateFlag = 1048576;
// 
pub const QStyle__State_Sibling :QStyle__StateFlag = 2097152;
// 
pub const QStyle__State_Editing :QStyle__StateFlag = 4194304;
// 
pub const QStyle__State_KeyboardFocusChange :QStyle__StateFlag = 8388608;
// 
pub const QStyle__State_ReadOnly :QStyle__StateFlag = 33554432;
// 
pub const QStyle__State_Small :QStyle__StateFlag = 67108864;
// 
pub const QStyle__State_Mini :QStyle__StateFlag = 134217728;
pub fn QStyle_StateFlagItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStyle", val);
}
pub fn QStyle_StateFlagItemName_s(val: i32) ->String {
  //var nilthis *QStyle
  //return nilthis.StateFlagItemName(val);
  return QStyle_StateFlagItemName(val);
}


/*
This enum describes the various primitive elements. A primitive element is a common GUI element, such as a checkbox indicator or button bevel.

QStyle::PE_PanelButtonCommand?Button used to initiate an action, for example, a QPushButton.
QStyle::PE_PanelButtonBevel?Generic panel with a button bevel.
QStyle::PE_PanelButtonTool?Panel for a Tool button, used with QToolButton.
QStyle::PE_PanelLineEdit?Panel for a QLineEdit.
QStyle::PE_IndicatorButtonDropDown?Indicator for a drop down button, for example, a tool button that displays a menu.
QStyle::PE_IndicatorArrowUp?Generic Up arrow.
QStyle::PE_IndicatorArrowDown?Generic Down arrow.
QStyle::PE_IndicatorArrowRight?Generic Right arrow.
QStyle::PE_IndicatorArrowLeft?Generic Left arrow.
QStyle::PE_IndicatorSpinUp?Up symbol for a spin widget, for example a QSpinBox.
QStyle::PE_IndicatorSpinDown?Down symbol for a spin widget.
QStyle::PE_IndicatorSpinPlus?Increase symbol for a spin widget.
QStyle::PE_IndicatorSpinMinus?Decrease symbol for a spin widget.
QStyle::PE_IndicatorItemViewItemCheckPE_IndicatorViewItemCheckOn/off indicator for a view item.
QStyle::PE_IndicatorCheckBox?On/off indicator, for example, a QCheckBox.
QStyle::PE_IndicatorRadioButton?Exclusive on/off indicator, for example, a QRadioButton.
QStyle::PE_IndicatorDockWidgetResizeHandle?Resize handle for dock windows.
QStyle::PE_PanelMenuBar?Panel for menu bars.
QStyle::PE_PanelScrollAreaCorner?Panel at the bottom-right (or bottom-left) corner of a scroll area.
QStyle::PE_FrameTabWidget?Frame for tab widgets.
QStyle::PE_FrameButtonBevel?Panel frame for a button bevel.
QStyle::PE_FrameButtonTool?Panel frame for a tool button.
QStyle::PE_IndicatorHeaderArrow?Arrow used to indicate sorting on a list or table header.
QStyle::PE_FrameStatusBarItemPE_FrameStatusBarFrame for an item of a status bar; see also QStatusBar.
QStyle::PE_FrameWindow?Frame around a MDI window or a docking window.
QStyle::PE_IndicatorMenuCheckMark?Check mark used in a menu.
QStyle::PE_IndicatorProgressChunk?Section of a progress bar indicator; see also QProgressBar.
QStyle::PE_IndicatorBranch?Lines used to represent the branch of a tree in a tree view.
QStyle::PE_IndicatorToolBarHandle?The handle of a toolbar.
QStyle::PE_IndicatorToolBarSeparator?The separator in a toolbar.
QStyle::PE_PanelToolBar?The panel for a toolbar.
QStyle::PE_PanelTipLabel?The panel for a tip label.
QStyle::PE_FrameTabBarBase?The frame that is drawn for a tab bar, ususally drawn for a tab bar that isn't part of a tab widget.
QStyle::PE_IndicatorTabTear?Deprecated. Use PE_IndicatorTabTearLeft instead.
QStyle::PE_IndicatorTabTearLeftPE_IndicatorTabTearAn indicator that a tab is partially scrolled out on the left side of the visible tab bar when there are many tabs.
QStyle::PE_IndicatorTabTearRight?An indicator that a tab is partially scrolled out on the right side of the visible tab bar when there are many tabs.
QStyle::PE_IndicatorColumnViewArrow?An arrow in a QColumnView.
QStyle::PE_Widget?A plain QWidget.
QStyle::PE_IndicatorItemViewItemDrop?An indicator that is drawn to show where an item in an item view is about to be dropped during a drag-and-drop operation in an item view.
QStyle::PE_PanelItemViewItem?The background for an item in an item view.
QStyle::PE_PanelItemViewRow?The background of a row in an item view.
QStyle::PE_PanelStatusBar?The panel for a status bar.
QStyle::PE_IndicatorTabClose?The close button on a tab bar.
QStyle::PE_PanelMenu?The panel for a menu.


See also drawPrimitive().

*/
pub type QStyle__PrimitiveElement = i32;
// Generic frame
pub const QStyle__PE_Frame :QStyle__PrimitiveElement = 0;
// This frame around a default button, e.g. in a dialog.
pub const QStyle__PE_FrameDefaultButton :QStyle__PrimitiveElement = 1;
// Panel frame for dock windows and toolbars.
pub const QStyle__PE_FrameDockWidget :QStyle__PrimitiveElement = 2;
// Generic focus indicator.
pub const QStyle__PE_FrameFocusRect :QStyle__PrimitiveElement = 3;
// Panel frame around group boxes.
pub const QStyle__PE_FrameGroupBox :QStyle__PrimitiveElement = 4;
// Panel frame for line edits.
pub const QStyle__PE_FrameLineEdit :QStyle__PrimitiveElement = 5;
// Frame for popup windows/menus; see also QMenu.
pub const QStyle__PE_FrameMenu :QStyle__PrimitiveElement = 6;
// Obsolete. Use PE_FrameStatusBarItem instead.
pub const QStyle__PE_FrameStatusBar :QStyle__PrimitiveElement = 7;
// 
pub const QStyle__PE_FrameStatusBarItem :QStyle__PrimitiveElement = 7;
// 
pub const QStyle__PE_FrameTabWidget :QStyle__PrimitiveElement = 8;
// 
pub const QStyle__PE_FrameWindow :QStyle__PrimitiveElement = 9;
// 
pub const QStyle__PE_FrameButtonBevel :QStyle__PrimitiveElement = 10;
// 
pub const QStyle__PE_FrameButtonTool :QStyle__PrimitiveElement = 11;
// 
pub const QStyle__PE_FrameTabBarBase :QStyle__PrimitiveElement = 12;
// 
pub const QStyle__PE_PanelButtonCommand :QStyle__PrimitiveElement = 13;
// 
pub const QStyle__PE_PanelButtonBevel :QStyle__PrimitiveElement = 14;
// 
pub const QStyle__PE_PanelButtonTool :QStyle__PrimitiveElement = 15;
// 
pub const QStyle__PE_PanelMenuBar :QStyle__PrimitiveElement = 16;
// 
pub const QStyle__PE_PanelToolBar :QStyle__PrimitiveElement = 17;
// 
pub const QStyle__PE_PanelLineEdit :QStyle__PrimitiveElement = 18;
// 
pub const QStyle__PE_IndicatorArrowDown :QStyle__PrimitiveElement = 19;
// 
pub const QStyle__PE_IndicatorArrowLeft :QStyle__PrimitiveElement = 20;
// 
pub const QStyle__PE_IndicatorArrowRight :QStyle__PrimitiveElement = 21;
// 
pub const QStyle__PE_IndicatorArrowUp :QStyle__PrimitiveElement = 22;
// 
pub const QStyle__PE_IndicatorBranch :QStyle__PrimitiveElement = 23;
// 
pub const QStyle__PE_IndicatorButtonDropDown :QStyle__PrimitiveElement = 24;
// 
pub const QStyle__PE_IndicatorViewItemCheck :QStyle__PrimitiveElement = 25;
// 
pub const QStyle__PE_IndicatorItemViewItemCheck :QStyle__PrimitiveElement = 25;
// 
pub const QStyle__PE_IndicatorCheckBox :QStyle__PrimitiveElement = 26;
// 
pub const QStyle__PE_IndicatorDockWidgetResizeHandle :QStyle__PrimitiveElement = 27;
// 
pub const QStyle__PE_IndicatorHeaderArrow :QStyle__PrimitiveElement = 28;
// 
pub const QStyle__PE_IndicatorMenuCheckMark :QStyle__PrimitiveElement = 29;
// 
pub const QStyle__PE_IndicatorProgressChunk :QStyle__PrimitiveElement = 30;
// 
pub const QStyle__PE_IndicatorRadioButton :QStyle__PrimitiveElement = 31;
// 
pub const QStyle__PE_IndicatorSpinDown :QStyle__PrimitiveElement = 32;
// 
pub const QStyle__PE_IndicatorSpinMinus :QStyle__PrimitiveElement = 33;
// 
pub const QStyle__PE_IndicatorSpinPlus :QStyle__PrimitiveElement = 34;
// 
pub const QStyle__PE_IndicatorSpinUp :QStyle__PrimitiveElement = 35;
// 
pub const QStyle__PE_IndicatorToolBarHandle :QStyle__PrimitiveElement = 36;
// 
pub const QStyle__PE_IndicatorToolBarSeparator :QStyle__PrimitiveElement = 37;
// 
pub const QStyle__PE_PanelTipLabel :QStyle__PrimitiveElement = 38;
// 
pub const QStyle__PE_IndicatorTabTear :QStyle__PrimitiveElement = 39;
// 
pub const QStyle__PE_IndicatorTabTearLeft :QStyle__PrimitiveElement = 39;
// 
pub const QStyle__PE_PanelScrollAreaCorner :QStyle__PrimitiveElement = 40;
// 
pub const QStyle__PE_Widget :QStyle__PrimitiveElement = 41;
// 
pub const QStyle__PE_IndicatorColumnViewArrow :QStyle__PrimitiveElement = 42;
// 
pub const QStyle__PE_IndicatorItemViewItemDrop :QStyle__PrimitiveElement = 43;
// 
pub const QStyle__PE_PanelItemViewItem :QStyle__PrimitiveElement = 44;
// 
pub const QStyle__PE_PanelItemViewRow :QStyle__PrimitiveElement = 45;
// 
pub const QStyle__PE_PanelStatusBar :QStyle__PrimitiveElement = 46;
// 
pub const QStyle__PE_IndicatorTabClose :QStyle__PrimitiveElement = 47;
// 
pub const QStyle__PE_PanelMenu :QStyle__PrimitiveElement = 48;
// 
pub const QStyle__PE_IndicatorTabTearRight :QStyle__PrimitiveElement = 49;
// 
pub const QStyle__PE_CustomBase :QStyle__PrimitiveElement = 251658240;
pub fn QStyle_PrimitiveElementItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStyle", val);
}
pub fn QStyle_PrimitiveElementItemName_s(val: i32) ->String {
  //var nilthis *QStyle
  //return nilthis.PrimitiveElementItemName(val);
  return QStyle_PrimitiveElementItemName(val);
}


/*
This enum represents a control element. A control element is a part of a widget that performs some action or displays information to the user.



See also drawControl().

*/
pub type QStyle__ControlElement = i32;
// A QPushButton, draws CE_PushButtonBevel, CE_PushButtonLabel and PE_FrameFocusRect.
pub const QStyle__CE_PushButton :QStyle__ControlElement = 0;
// The bevel and default indicator of a QPushButton.
pub const QStyle__CE_PushButtonBevel :QStyle__ControlElement = 1;
// The label (an icon with text or pixmap) of a QPushButton.
pub const QStyle__CE_PushButtonLabel :QStyle__ControlElement = 2;
// A QCheckBox, draws a PE_IndicatorCheckBox, a CE_CheckBoxLabel and a PE_FrameFocusRect.
pub const QStyle__CE_CheckBox :QStyle__ControlElement = 3;
// The label (text or pixmap) of a QCheckBox.
pub const QStyle__CE_CheckBoxLabel :QStyle__ControlElement = 4;
// A QRadioButton, draws a PE_IndicatorRadioButton, a CE_RadioButtonLabel and a PE_FrameFocusRect.
pub const QStyle__CE_RadioButton :QStyle__ControlElement = 5;
// The label (text or pixmap) of a QRadioButton.
pub const QStyle__CE_RadioButtonLabel :QStyle__ControlElement = 6;
// The tab and label within a QTabBar.
pub const QStyle__CE_TabBarTab :QStyle__ControlElement = 7;
// The tab shape within a tab bar.
pub const QStyle__CE_TabBarTabShape :QStyle__ControlElement = 8;
// The label within a tab.
pub const QStyle__CE_TabBarTabLabel :QStyle__ControlElement = 9;
// 
pub const QStyle__CE_ProgressBar :QStyle__ControlElement = 10;
// 
pub const QStyle__CE_ProgressBarGroove :QStyle__ControlElement = 11;
// 
pub const QStyle__CE_ProgressBarContents :QStyle__ControlElement = 12;
// 
pub const QStyle__CE_ProgressBarLabel :QStyle__ControlElement = 13;
// 
pub const QStyle__CE_MenuItem :QStyle__ControlElement = 14;
// 
pub const QStyle__CE_MenuScroller :QStyle__ControlElement = 15;
// 
pub const QStyle__CE_MenuVMargin :QStyle__ControlElement = 16;
// 
pub const QStyle__CE_MenuHMargin :QStyle__ControlElement = 17;
// 
pub const QStyle__CE_MenuTearoff :QStyle__ControlElement = 18;
// 
pub const QStyle__CE_MenuEmptyArea :QStyle__ControlElement = 19;
// 
pub const QStyle__CE_MenuBarItem :QStyle__ControlElement = 20;
// 
pub const QStyle__CE_MenuBarEmptyArea :QStyle__ControlElement = 21;
// 
pub const QStyle__CE_ToolButtonLabel :QStyle__ControlElement = 22;
// 
pub const QStyle__CE_Header :QStyle__ControlElement = 23;
// 
pub const QStyle__CE_HeaderSection :QStyle__ControlElement = 24;
// 
pub const QStyle__CE_HeaderLabel :QStyle__ControlElement = 25;
// 
pub const QStyle__CE_ToolBoxTab :QStyle__ControlElement = 26;
// 
pub const QStyle__CE_SizeGrip :QStyle__ControlElement = 27;
// 
pub const QStyle__CE_Splitter :QStyle__ControlElement = 28;
// 
pub const QStyle__CE_RubberBand :QStyle__ControlElement = 29;
// 
pub const QStyle__CE_DockWidgetTitle :QStyle__ControlElement = 30;
// 
pub const QStyle__CE_ScrollBarAddLine :QStyle__ControlElement = 31;
// 
pub const QStyle__CE_ScrollBarSubLine :QStyle__ControlElement = 32;
// 
pub const QStyle__CE_ScrollBarAddPage :QStyle__ControlElement = 33;
// 
pub const QStyle__CE_ScrollBarSubPage :QStyle__ControlElement = 34;
// 
pub const QStyle__CE_ScrollBarSlider :QStyle__ControlElement = 35;
// 
pub const QStyle__CE_ScrollBarFirst :QStyle__ControlElement = 36;
// 
pub const QStyle__CE_ScrollBarLast :QStyle__ControlElement = 37;
// 
pub const QStyle__CE_FocusFrame :QStyle__ControlElement = 38;
// 
pub const QStyle__CE_ComboBoxLabel :QStyle__ControlElement = 39;
// 
pub const QStyle__CE_ToolBar :QStyle__ControlElement = 40;
// 
pub const QStyle__CE_ToolBoxTabShape :QStyle__ControlElement = 41;
// 
pub const QStyle__CE_ToolBoxTabLabel :QStyle__ControlElement = 42;
// 
pub const QStyle__CE_HeaderEmptyArea :QStyle__ControlElement = 43;
// 
pub const QStyle__CE_ColumnViewGrip :QStyle__ControlElement = 44;
// 
pub const QStyle__CE_ItemViewItem :QStyle__ControlElement = 45;
// 
pub const QStyle__CE_ShapedFrame :QStyle__ControlElement = 46;
// 
pub const QStyle__CE_CustomBase :QStyle__ControlElement = -268435456;
pub fn QStyle_ControlElementItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStyle", val);
}
pub fn QStyle_ControlElementItemName_s(val: i32) ->String {
  //var nilthis *QStyle
  //return nilthis.ControlElementItemName(val);
  return QStyle_ControlElementItemName(val);
}


/*
This enum represents a sub-area of a widget. Style implementations use these areas to draw the different parts of a widget.

QStyle::SE_PushButtonLayoutItem?Area that counts for the parent layout.
QStyle::SE_CheckBoxLayoutItem?Area that counts for the parent layout.
QStyle::SE_DateTimeEditLayoutItem?Area that counts for the parent layout.
QStyle::SE_RadioButtonLayoutItem?Area that counts for the parent layout.
QStyle::SE_SliderLayoutItem?Area that counts for the parent layout.
QStyle::SE_SpinBoxLayoutItem?Area that counts for the parent layout.
QStyle::SE_ProgressBarLayoutItem?Area that counts for the parent layout.
QStyle::SE_FrameContents?Area for a frame's contents.
QStyle::SE_ShapedFrameContents?Area for a frame's contents using the shape in QStyleOptionFrame; see QFrame
QStyle::SE_FrameLayoutItem?Area that counts for the parent layout.
QStyle::SE_LabelLayoutItem?Area that counts for the parent layout.
QStyle::SE_LineEditContents?Area for a line edit's contents.
QStyle::SE_TabWidgetLayoutItem?Area that counts for the parent layout.
QStyle::SE_ToolButtonLayoutItem?Area that counts for the parent layout.
QStyle::SE_ItemViewItemCheckIndicatorSE_ViewItemCheckIndicatorArea for a view item's check mark.
QStyle::SE_TabBarTearIndicator?Deprecated. Use SE_TabBarTearIndicatorLeft instead.
QStyle::SE_TabBarTearIndicatorLeftSE_TabBarTearIndicatorArea for the tear indicator on the left side of a tab bar with scroll arrows.
QStyle::SE_TabBarTearIndicatorRight?Area for the tear indicator on the right side of a tab bar with scroll arrows.
QStyle::SE_TabBarScrollLeftButton?Area for the scroll left button on a tab bar with scroll buttons.
QStyle::SE_TabBarScrollRightButton?Area for the scroll right button on a tab bar with scroll buttons.
QStyle::SE_TreeViewDisclosureItem?Area for the actual disclosure item in a tree branch.
QStyle::SE_DialogButtonBoxLayoutItem?Area that counts for the parent layout.
QStyle::SE_GroupBoxLayoutItem?Area that counts for the parent layout.
QStyle::SE_DockWidgetFloatButton?The float button of a dock widget.
QStyle::SE_DockWidgetTitleBarText?The text bounds of the dock widgets title.
QStyle::SE_DockWidgetCloseButton?The close button of a dock widget.
QStyle::SE_DockWidgetIcon?The icon of a dock widget.
QStyle::SE_ComboBoxLayoutItem?Area that counts for the parent layout.
QStyle::SE_ItemViewItemDecoration?Area for a view item's decoration (icon).
QStyle::SE_ItemViewItemText?Area for a view item's text.
QStyle::SE_ItemViewItemFocusRect?Area for a view item's focus rect.
QStyle::SE_TabBarTabLeftButton?Area for a widget on the left side of a tab in a tab bar.
QStyle::SE_TabBarTabRightButton?Area for a widget on the right side of a tab in a tab bar.
QStyle::SE_TabBarTabText?Area for the text on a tab in a tab bar.
QStyle::SE_ToolBarHandle?Area for the handle of a tool bar.


See also subElementRect().

*/
pub type QStyle__SubElement = i32;
// Area containing the label (icon with text or pixmap).
pub const QStyle__SE_PushButtonContents :QStyle__SubElement = 0;
// Area for the focus rect (usually larger than the contents rect).
pub const QStyle__SE_PushButtonFocusRect :QStyle__SubElement = 1;
// Area for the state indicator (e.g., check mark).
pub const QStyle__SE_CheckBoxIndicator :QStyle__SubElement = 2;
// Area for the label (text or pixmap).
pub const QStyle__SE_CheckBoxContents :QStyle__SubElement = 3;
// Area for the focus indicator.
pub const QStyle__SE_CheckBoxFocusRect :QStyle__SubElement = 4;
// Clickable area, defaults to SE_CheckBoxFocusRect.
pub const QStyle__SE_CheckBoxClickRect :QStyle__SubElement = 5;
// Area for the state indicator.
pub const QStyle__SE_RadioButtonIndicator :QStyle__SubElement = 6;
// Area for the label.
pub const QStyle__SE_RadioButtonContents :QStyle__SubElement = 7;
// Area for the focus indicator.
pub const QStyle__SE_RadioButtonFocusRect :QStyle__SubElement = 8;
// Clickable area, defaults to SE_RadioButtonFocusRect.
pub const QStyle__SE_RadioButtonClickRect :QStyle__SubElement = 9;
// 
pub const QStyle__SE_ComboBoxFocusRect :QStyle__SubElement = 10;
// 
pub const QStyle__SE_SliderFocusRect :QStyle__SubElement = 11;
// 
pub const QStyle__SE_ProgressBarGroove :QStyle__SubElement = 12;
// 
pub const QStyle__SE_ProgressBarContents :QStyle__SubElement = 13;
// 
pub const QStyle__SE_ProgressBarLabel :QStyle__SubElement = 14;
// 
pub const QStyle__SE_ToolBoxTabContents :QStyle__SubElement = 15;
// 
pub const QStyle__SE_HeaderLabel :QStyle__SubElement = 16;
// 
pub const QStyle__SE_HeaderArrow :QStyle__SubElement = 17;
// 
pub const QStyle__SE_TabWidgetTabBar :QStyle__SubElement = 18;
// 
pub const QStyle__SE_TabWidgetTabPane :QStyle__SubElement = 19;
// 
pub const QStyle__SE_TabWidgetTabContents :QStyle__SubElement = 20;
// 
pub const QStyle__SE_TabWidgetLeftCorner :QStyle__SubElement = 21;
// 
pub const QStyle__SE_TabWidgetRightCorner :QStyle__SubElement = 22;
// 
pub const QStyle__SE_ViewItemCheckIndicator :QStyle__SubElement = 23;
// 
pub const QStyle__SE_ItemViewItemCheckIndicator :QStyle__SubElement = 23;
// 
pub const QStyle__SE_TabBarTearIndicator :QStyle__SubElement = 24;
// 
pub const QStyle__SE_TabBarTearIndicatorLeft :QStyle__SubElement = 24;
// 
pub const QStyle__SE_TreeViewDisclosureItem :QStyle__SubElement = 25;
// 
pub const QStyle__SE_LineEditContents :QStyle__SubElement = 26;
// 
pub const QStyle__SE_FrameContents :QStyle__SubElement = 27;
// 
pub const QStyle__SE_DockWidgetCloseButton :QStyle__SubElement = 28;
// 
pub const QStyle__SE_DockWidgetFloatButton :QStyle__SubElement = 29;
// 
pub const QStyle__SE_DockWidgetTitleBarText :QStyle__SubElement = 30;
// 
pub const QStyle__SE_DockWidgetIcon :QStyle__SubElement = 31;
// 
pub const QStyle__SE_CheckBoxLayoutItem :QStyle__SubElement = 32;
// 
pub const QStyle__SE_ComboBoxLayoutItem :QStyle__SubElement = 33;
// 
pub const QStyle__SE_DateTimeEditLayoutItem :QStyle__SubElement = 34;
// 
pub const QStyle__SE_DialogButtonBoxLayoutItem :QStyle__SubElement = 35;
// 
pub const QStyle__SE_LabelLayoutItem :QStyle__SubElement = 36;
// 
pub const QStyle__SE_ProgressBarLayoutItem :QStyle__SubElement = 37;
// 
pub const QStyle__SE_PushButtonLayoutItem :QStyle__SubElement = 38;
// 
pub const QStyle__SE_RadioButtonLayoutItem :QStyle__SubElement = 39;
// 
pub const QStyle__SE_SliderLayoutItem :QStyle__SubElement = 40;
// 
pub const QStyle__SE_SpinBoxLayoutItem :QStyle__SubElement = 41;
// 
pub const QStyle__SE_ToolButtonLayoutItem :QStyle__SubElement = 42;
// 
pub const QStyle__SE_FrameLayoutItem :QStyle__SubElement = 43;
// 
pub const QStyle__SE_GroupBoxLayoutItem :QStyle__SubElement = 44;
// 
pub const QStyle__SE_TabWidgetLayoutItem :QStyle__SubElement = 45;
// 
pub const QStyle__SE_ItemViewItemDecoration :QStyle__SubElement = 46;
// 
pub const QStyle__SE_ItemViewItemText :QStyle__SubElement = 47;
// 
pub const QStyle__SE_ItemViewItemFocusRect :QStyle__SubElement = 48;
// 
pub const QStyle__SE_TabBarTabLeftButton :QStyle__SubElement = 49;
// 
pub const QStyle__SE_TabBarTabRightButton :QStyle__SubElement = 50;
// 
pub const QStyle__SE_TabBarTabText :QStyle__SubElement = 51;
// 
pub const QStyle__SE_ShapedFrameContents :QStyle__SubElement = 52;
// 
pub const QStyle__SE_ToolBarHandle :QStyle__SubElement = 53;
// 
pub const QStyle__SE_TabBarScrollLeftButton :QStyle__SubElement = 54;
// 
pub const QStyle__SE_TabBarScrollRightButton :QStyle__SubElement = 55;
// 
pub const QStyle__SE_TabBarTearIndicatorRight :QStyle__SubElement = 56;
// 
pub const QStyle__SE_CustomBase :QStyle__SubElement = -268435456;
pub fn QStyle_SubElementItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStyle", val);
}
pub fn QStyle_SubElementItemName_s(val: i32) ->String {
  //var nilthis *QStyle
  //return nilthis.SubElementItemName(val);
  return QStyle_SubElementItemName(val);
}


/*
This enum describes the available complex controls. Complex controls have different behavior depending upon where the user clicks on them or which keys are pressed.



See also SubControl and drawComplexControl().

*/
pub type QStyle__ComplexControl = i32;
// A spinbox, like QSpinBox.
pub const QStyle__CC_SpinBox :QStyle__ComplexControl = 0;
// A combobox, like QComboBox.
pub const QStyle__CC_ComboBox :QStyle__ComplexControl = 1;
// A scroll bar, like QScrollBar.
pub const QStyle__CC_ScrollBar :QStyle__ComplexControl = 2;
// A slider, like QSlider.
pub const QStyle__CC_Slider :QStyle__ComplexControl = 3;
// A tool button, like QToolButton.
pub const QStyle__CC_ToolButton :QStyle__ComplexControl = 4;
// A Title bar, like those used in QMdiSubWindow.
pub const QStyle__CC_TitleBar :QStyle__ComplexControl = 5;
// A dial, like QDial.
pub const QStyle__CC_Dial :QStyle__ComplexControl = 6;
// A group box, like QGroupBox.
pub const QStyle__CC_GroupBox :QStyle__ComplexControl = 7;
// The minimize, close, and normal button in the menu bar for a maximized MDI subwindow.
pub const QStyle__CC_MdiControls :QStyle__ComplexControl = 8;
// 
pub const QStyle__CC_CustomBase :QStyle__ComplexControl = -268435456;
pub fn QStyle_ComplexControlItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStyle", val);
}
pub fn QStyle_ComplexControlItemName_s(val: i32) ->String {
  //var nilthis *QStyle
  //return nilthis.ComplexControlItemName(val);
  return QStyle_ComplexControlItemName(val);
}


/*


*/
pub type QStyle__SubControl = i32;
// 
pub const QStyle__SC_None :QStyle__SubControl = 0;
// 
pub const QStyle__SC_ScrollBarAddLine :QStyle__SubControl = 1;
// 
pub const QStyle__SC_ScrollBarSubLine :QStyle__SubControl = 2;
// 
pub const QStyle__SC_ScrollBarAddPage :QStyle__SubControl = 4;
// 
pub const QStyle__SC_ScrollBarSubPage :QStyle__SubControl = 8;
// 
pub const QStyle__SC_ScrollBarFirst :QStyle__SubControl = 16;
// 
pub const QStyle__SC_ScrollBarLast :QStyle__SubControl = 32;
// 
pub const QStyle__SC_ScrollBarSlider :QStyle__SubControl = 64;
// 
pub const QStyle__SC_ScrollBarGroove :QStyle__SubControl = 128;
// 
pub const QStyle__SC_SpinBoxUp :QStyle__SubControl = 1;
// 
pub const QStyle__SC_SpinBoxDown :QStyle__SubControl = 2;
// 
pub const QStyle__SC_SpinBoxFrame :QStyle__SubControl = 4;
// 
pub const QStyle__SC_SpinBoxEditField :QStyle__SubControl = 8;
// 
pub const QStyle__SC_ComboBoxFrame :QStyle__SubControl = 1;
// 
pub const QStyle__SC_ComboBoxEditField :QStyle__SubControl = 2;
// 
pub const QStyle__SC_ComboBoxArrow :QStyle__SubControl = 4;
// 
pub const QStyle__SC_ComboBoxListBoxPopup :QStyle__SubControl = 8;
// 
pub const QStyle__SC_SliderGroove :QStyle__SubControl = 1;
// 
pub const QStyle__SC_SliderHandle :QStyle__SubControl = 2;
// 
pub const QStyle__SC_SliderTickmarks :QStyle__SubControl = 4;
// 
pub const QStyle__SC_ToolButton :QStyle__SubControl = 1;
// 
pub const QStyle__SC_ToolButtonMenu :QStyle__SubControl = 2;
// 
pub const QStyle__SC_TitleBarSysMenu :QStyle__SubControl = 1;
// 
pub const QStyle__SC_TitleBarMinButton :QStyle__SubControl = 2;
// 
pub const QStyle__SC_TitleBarMaxButton :QStyle__SubControl = 4;
// 
pub const QStyle__SC_TitleBarCloseButton :QStyle__SubControl = 8;
// 
pub const QStyle__SC_TitleBarNormalButton :QStyle__SubControl = 16;
// 
pub const QStyle__SC_TitleBarShadeButton :QStyle__SubControl = 32;
// 
pub const QStyle__SC_TitleBarUnshadeButton :QStyle__SubControl = 64;
// 
pub const QStyle__SC_TitleBarContextHelpButton :QStyle__SubControl = 128;
// 
pub const QStyle__SC_TitleBarLabel :QStyle__SubControl = 256;
// 
pub const QStyle__SC_DialGroove :QStyle__SubControl = 1;
// 
pub const QStyle__SC_DialHandle :QStyle__SubControl = 2;
// 
pub const QStyle__SC_DialTickmarks :QStyle__SubControl = 4;
// 
pub const QStyle__SC_GroupBoxCheckBox :QStyle__SubControl = 1;
// 
pub const QStyle__SC_GroupBoxLabel :QStyle__SubControl = 2;
// 
pub const QStyle__SC_GroupBoxContents :QStyle__SubControl = 4;
// 
pub const QStyle__SC_GroupBoxFrame :QStyle__SubControl = 8;
// 
pub const QStyle__SC_MdiMinButton :QStyle__SubControl = 1;
// 
pub const QStyle__SC_MdiNormalButton :QStyle__SubControl = 2;
// 
pub const QStyle__SC_MdiCloseButton :QStyle__SubControl = 4;
// 
pub const QStyle__SC_CustomBase :QStyle__SubControl = -268435456;
// 
pub const QStyle__SC_All :QStyle__SubControl = -1;
pub fn QStyle_SubControlItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStyle", val);
}
pub fn QStyle_SubControlItemName_s(val: i32) ->String {
  //var nilthis *QStyle
  //return nilthis.SubControlItemName(val);
  return QStyle_SubControlItemName(val);
}


/*
This enum describes the various available pixel metrics. A pixel metric is a style dependent size represented by a single pixel value.

QStyle::PM_DockWidgetTitleBarButtonMargin?Amount of whitespace between dock widget's title bar button labels and the frame.
QStyle::PM_MDIFrameWidthPM_MdiSubWindowFrameWidthObsolete. Use PM_MdiSubWindowFrameWidth instead.
QStyle::PM_MDIMinimizedWidthPM_MdiSubWindowMinimizedWidthObsolete. Use PM_MdiSubWindowMinimizedWidth instead.
QStyle::PM_MdiSubWindowMinimizedWidth?Width of a minimized MDI window.
QStyle::PM_LayoutLeftMargin?Default left margin for a QLayout.
QStyle::PM_LayoutTopMargin?Default top margin for a QLayout.
QStyle::PM_LayoutRightMargin?Default right margin for a QLayout.
QStyle::PM_LayoutBottomMargin?Default bottom margin for a QLayout.
QStyle::PM_LayoutHorizontalSpacing?Default horizontal spacing for a QLayout.
QStyle::PM_LayoutVerticalSpacing?Default vertical spacing for a QLayout.
QStyle::PM_DockWidgetTitleMargin?Margin of the dock window title.
QStyle::PM_ToolBarFrameWidth?Width of the frame around toolbars.
QStyle::PM_ToolBarHandleExtent?Width of a toolbar handle in a horizontal toolbar and the height of the handle in a vertical toolbar.
QStyle::PM_ToolBarItemMargin?Spacing between the toolbar frame and the items.
QStyle::PM_ToolBarItemSpacing?Spacing between toolbar items.
QStyle::PM_ToolBarSeparatorExtent?Width of a toolbar separator in a horizontal toolbar and the height of a separator in a vertical toolbar.
QStyle::PM_ToolBarExtensionExtent?Width of a toolbar extension button in a horizontal toolbar and the height of the button in a vertical toolbar.
QStyle::PM_TabBarScrollButtonWidth? 
QStyle::PM_TabBarTabShiftHorizontal?Horizontal pixel shift when a tab is selected.
QStyle::PM_TabBarTabShiftVertical?Vertical pixel shift when a tab is selected.
QStyle::PM_HeaderMarkSize?The size of the sort indicator in a header.
QStyle::PM_HeaderGripMargin?The size of the resize grip in a header.
QStyle::PM_HeaderMargin?The size of the margin between the sort indicator and the text.
QStyle::PM_SpinBoxSliderHeight?The height of the optional spin box slider.
QStyle::PM_ToolBarIconSize?Default tool bar icon size
QStyle::PM_SmallIconSize?Default small icon size
QStyle::PM_LargeIconSize?Default large icon size
QStyle::PM_FocusFrameHMargin?Horizontal margin that the focus frame will outset the widget by.
QStyle::PM_FocusFrameVMargin?Vertical margin that the focus frame will outset the widget by.
QStyle::PM_IconViewIconSize?The default size for icons in an icon view.
QStyle::PM_ListViewIconSize?The default size for icons in a list view.
QStyle::PM_ToolTipLabelFrameWidth?The frame width for a tool tip label.
QStyle::PM_CheckBoxLabelSpacing?The spacing between a check box indicator and its label.
QStyle::PM_RadioButtonLabelSpacing?The spacing between a radio button indicator and its label.
QStyle::PM_TabBarIconSize?The default icon size for a tab bar.
QStyle::PM_SizeGripSize?The size of a size grip.
QStyle::PM_MessageBoxIconSize?The size of the standard icons in a message box
QStyle::PM_ButtonIconSize?The default size of button icons
QStyle::PM_TextCursorWidth?The width of the cursor in a line edit or text edit
QStyle::PM_TabBar_ScrollButtonOverlap?The distance between the left and right buttons in a tab bar.
QStyle::PM_TabCloseIndicatorWidth?The default width of a close button on a tab in a tab bar.
QStyle::PM_TabCloseIndicatorHeight?The default height of a close button on a tab in a tab bar.
QStyle::PM_ScrollView_ScrollBarSpacing?Distance between frame and scrollbar with SH_ScrollView_FrameOnlyAroundContents set.
QStyle::PM_ScrollView_ScrollBarOverlap?Overlap between scroll bars and scroll content
QStyle::PM_SubMenuOverlap?The horizontal overlap between a submenu and its parent.


The following values are obsolete:

QStyle::PM_DefaultTopLevelMargin?Use PM_LayoutLeftMargin, PM_LayoutTopMargin, PM_LayoutRightMargin, and PM_LayoutBottomMargin instead.
QStyle::PM_DefaultChildMargin?Use PM_LayoutLeftMargin, PM_LayoutTopMargin, PM_LayoutRightMargin, and PM_LayoutBottomMargin instead.
QStyle::PM_DefaultLayoutSpacing?Use PM_LayoutHorizontalSpacing and PM_LayoutVerticalSpacing instead.


See also pixelMetric().

*/
pub type QStyle__PixelMetric = i32;
// Amount of whitespace between push button labels and the frame.
pub const QStyle__PM_ButtonMargin :QStyle__PixelMetric = 0;
// Width of the default-button indicator frame.
pub const QStyle__PM_ButtonDefaultIndicator :QStyle__PixelMetric = 1;
// Width of the menu button indicator proportional to the widget height.
pub const QStyle__PM_MenuButtonIndicator :QStyle__PixelMetric = 2;
// Horizontal contents shift of a button when the button is down.
pub const QStyle__PM_ButtonShiftHorizontal :QStyle__PixelMetric = 3;
// Vertical contents shift of a button when the button is down.
pub const QStyle__PM_ButtonShiftVertical :QStyle__PixelMetric = 4;
// 
pub const QStyle__PM_DefaultFrameWidth :QStyle__PixelMetric = 5;
// Frame width of a spin box, defaults to PM_DefaultFrameWidth.
pub const QStyle__PM_SpinBoxFrameWidth :QStyle__PixelMetric = 6;
// Frame width of a combo box, defaults to PM_DefaultFrameWidth.
pub const QStyle__PM_ComboBoxFrameWidth :QStyle__PixelMetric = 7;
// 
pub const QStyle__PM_MaximumDragDistance :QStyle__PixelMetric = 8;
// Width of a vertical scroll bar and the height of a horizontal scroll bar.
pub const QStyle__PM_ScrollBarExtent :QStyle__PixelMetric = 9;
// 
pub const QStyle__PM_ScrollBarSliderMin :QStyle__PixelMetric = 10;
// 
pub const QStyle__PM_SliderThickness :QStyle__PixelMetric = 11;
// 
pub const QStyle__PM_SliderControlThickness :QStyle__PixelMetric = 12;
// 
pub const QStyle__PM_SliderLength :QStyle__PixelMetric = 13;
// 
pub const QStyle__PM_SliderTickmarkOffset :QStyle__PixelMetric = 14;
// 
pub const QStyle__PM_SliderSpaceAvailable :QStyle__PixelMetric = 15;
// 
pub const QStyle__PM_DockWidgetSeparatorExtent :QStyle__PixelMetric = 16;
// 
pub const QStyle__PM_DockWidgetHandleExtent :QStyle__PixelMetric = 17;
// 
pub const QStyle__PM_DockWidgetFrameWidth :QStyle__PixelMetric = 18;
// 
pub const QStyle__PM_TabBarTabOverlap :QStyle__PixelMetric = 19;
// 
pub const QStyle__PM_TabBarTabHSpace :QStyle__PixelMetric = 20;
// 
pub const QStyle__PM_TabBarTabVSpace :QStyle__PixelMetric = 21;
// 
pub const QStyle__PM_TabBarBaseHeight :QStyle__PixelMetric = 22;
// 
pub const QStyle__PM_TabBarBaseOverlap :QStyle__PixelMetric = 23;
// 
pub const QStyle__PM_ProgressBarChunkWidth :QStyle__PixelMetric = 24;
// 
pub const QStyle__PM_SplitterWidth :QStyle__PixelMetric = 25;
// 
pub const QStyle__PM_TitleBarHeight :QStyle__PixelMetric = 26;
// 
pub const QStyle__PM_MenuScrollerHeight :QStyle__PixelMetric = 27;
// 
pub const QStyle__PM_MenuHMargin :QStyle__PixelMetric = 28;
// 
pub const QStyle__PM_MenuVMargin :QStyle__PixelMetric = 29;
// 
pub const QStyle__PM_MenuPanelWidth :QStyle__PixelMetric = 30;
// 
pub const QStyle__PM_MenuTearoffHeight :QStyle__PixelMetric = 31;
// 
pub const QStyle__PM_MenuDesktopFrameWidth :QStyle__PixelMetric = 32;
// 
pub const QStyle__PM_MenuBarPanelWidth :QStyle__PixelMetric = 33;
// 
pub const QStyle__PM_MenuBarItemSpacing :QStyle__PixelMetric = 34;
// 
pub const QStyle__PM_MenuBarVMargin :QStyle__PixelMetric = 35;
// 
pub const QStyle__PM_MenuBarHMargin :QStyle__PixelMetric = 36;
// 
pub const QStyle__PM_IndicatorWidth :QStyle__PixelMetric = 37;
// 
pub const QStyle__PM_IndicatorHeight :QStyle__PixelMetric = 38;
// 
pub const QStyle__PM_ExclusiveIndicatorWidth :QStyle__PixelMetric = 39;
// 
pub const QStyle__PM_ExclusiveIndicatorHeight :QStyle__PixelMetric = 40;
// 
pub const QStyle__PM_DialogButtonsSeparator :QStyle__PixelMetric = 41;
// 
pub const QStyle__PM_DialogButtonsButtonWidth :QStyle__PixelMetric = 42;
// 
pub const QStyle__PM_DialogButtonsButtonHeight :QStyle__PixelMetric = 43;
// 
pub const QStyle__PM_MdiSubWindowFrameWidth :QStyle__PixelMetric = 44;
// 
pub const QStyle__PM_MDIFrameWidth :QStyle__PixelMetric = 44;
// 
pub const QStyle__PM_MdiSubWindowMinimizedWidth :QStyle__PixelMetric = 45;
// 
pub const QStyle__PM_MDIMinimizedWidth :QStyle__PixelMetric = 45;
// 
pub const QStyle__PM_HeaderMargin :QStyle__PixelMetric = 46;
// 
pub const QStyle__PM_HeaderMarkSize :QStyle__PixelMetric = 47;
// 
pub const QStyle__PM_HeaderGripMargin :QStyle__PixelMetric = 48;
// 
pub const QStyle__PM_TabBarTabShiftHorizontal :QStyle__PixelMetric = 49;
// 
pub const QStyle__PM_TabBarTabShiftVertical :QStyle__PixelMetric = 50;
// 
pub const QStyle__PM_TabBarScrollButtonWidth :QStyle__PixelMetric = 51;
// 
pub const QStyle__PM_ToolBarFrameWidth :QStyle__PixelMetric = 52;
// 
pub const QStyle__PM_ToolBarHandleExtent :QStyle__PixelMetric = 53;
// 
pub const QStyle__PM_ToolBarItemSpacing :QStyle__PixelMetric = 54;
// 
pub const QStyle__PM_ToolBarItemMargin :QStyle__PixelMetric = 55;
// 
pub const QStyle__PM_ToolBarSeparatorExtent :QStyle__PixelMetric = 56;
// 
pub const QStyle__PM_ToolBarExtensionExtent :QStyle__PixelMetric = 57;
// 
pub const QStyle__PM_SpinBoxSliderHeight :QStyle__PixelMetric = 58;
// 
pub const QStyle__PM_DefaultTopLevelMargin :QStyle__PixelMetric = 59;
// 
pub const QStyle__PM_DefaultChildMargin :QStyle__PixelMetric = 60;
// 
pub const QStyle__PM_DefaultLayoutSpacing :QStyle__PixelMetric = 61;
// 
pub const QStyle__PM_ToolBarIconSize :QStyle__PixelMetric = 62;
// 
pub const QStyle__PM_ListViewIconSize :QStyle__PixelMetric = 63;
// 
pub const QStyle__PM_IconViewIconSize :QStyle__PixelMetric = 64;
// 
pub const QStyle__PM_SmallIconSize :QStyle__PixelMetric = 65;
// 
pub const QStyle__PM_LargeIconSize :QStyle__PixelMetric = 66;
// 
pub const QStyle__PM_FocusFrameVMargin :QStyle__PixelMetric = 67;
// 
pub const QStyle__PM_FocusFrameHMargin :QStyle__PixelMetric = 68;
// 
pub const QStyle__PM_ToolTipLabelFrameWidth :QStyle__PixelMetric = 69;
// 
pub const QStyle__PM_CheckBoxLabelSpacing :QStyle__PixelMetric = 70;
// 
pub const QStyle__PM_TabBarIconSize :QStyle__PixelMetric = 71;
// 
pub const QStyle__PM_SizeGripSize :QStyle__PixelMetric = 72;
// 
pub const QStyle__PM_DockWidgetTitleMargin :QStyle__PixelMetric = 73;
// 
pub const QStyle__PM_MessageBoxIconSize :QStyle__PixelMetric = 74;
// 
pub const QStyle__PM_ButtonIconSize :QStyle__PixelMetric = 75;
// 
pub const QStyle__PM_DockWidgetTitleBarButtonMargin :QStyle__PixelMetric = 76;
// 
pub const QStyle__PM_RadioButtonLabelSpacing :QStyle__PixelMetric = 77;
// 
pub const QStyle__PM_LayoutLeftMargin :QStyle__PixelMetric = 78;
// 
pub const QStyle__PM_LayoutTopMargin :QStyle__PixelMetric = 79;
// 
pub const QStyle__PM_LayoutRightMargin :QStyle__PixelMetric = 80;
// 
pub const QStyle__PM_LayoutBottomMargin :QStyle__PixelMetric = 81;
// 
pub const QStyle__PM_LayoutHorizontalSpacing :QStyle__PixelMetric = 82;
// 
pub const QStyle__PM_LayoutVerticalSpacing :QStyle__PixelMetric = 83;
// 
pub const QStyle__PM_TabBar_ScrollButtonOverlap :QStyle__PixelMetric = 84;
// 
pub const QStyle__PM_TextCursorWidth :QStyle__PixelMetric = 85;
// 
pub const QStyle__PM_TabCloseIndicatorWidth :QStyle__PixelMetric = 86;
// 
pub const QStyle__PM_TabCloseIndicatorHeight :QStyle__PixelMetric = 87;
// 
pub const QStyle__PM_ScrollView_ScrollBarSpacing :QStyle__PixelMetric = 88;
// 
pub const QStyle__PM_ScrollView_ScrollBarOverlap :QStyle__PixelMetric = 89;
// 
pub const QStyle__PM_SubMenuOverlap :QStyle__PixelMetric = 90;
// 
pub const QStyle__PM_TreeViewIndentation :QStyle__PixelMetric = 91;
// 
pub const QStyle__PM_HeaderDefaultSectionSizeHorizontal :QStyle__PixelMetric = 92;
// 
pub const QStyle__PM_HeaderDefaultSectionSizeVertical :QStyle__PixelMetric = 93;
// 
pub const QStyle__PM_TitleBarButtonIconSize :QStyle__PixelMetric = 94;
// 
pub const QStyle__PM_TitleBarButtonSize :QStyle__PixelMetric = 95;
// 
pub const QStyle__PM_CustomBase :QStyle__PixelMetric = -268435456;
pub fn QStyle_PixelMetricItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStyle", val);
}
pub fn QStyle_PixelMetricItemName_s(val: i32) ->String {
  //var nilthis *QStyle
  //return nilthis.PixelMetricItemName(val);
  return QStyle_PixelMetricItemName(val);
}


/*
This enum describes the available contents types. These are used to calculate sizes for the contents of various widgets.



See also sizeFromContents().

*/
pub type QStyle__ContentsType = i32;
// A push button, like QPushButton.
pub const QStyle__CT_PushButton :QStyle__ContentsType = 0;
// A check box, like QCheckBox.
pub const QStyle__CT_CheckBox :QStyle__ContentsType = 1;
// A radio button, like QRadioButton.
pub const QStyle__CT_RadioButton :QStyle__ContentsType = 2;
// A tool button, like QToolButton.
pub const QStyle__CT_ToolButton :QStyle__ContentsType = 3;
// A combo box, like QComboBox.
pub const QStyle__CT_ComboBox :QStyle__ContentsType = 4;
// A splitter, like QSplitter.
pub const QStyle__CT_Splitter :QStyle__ContentsType = 5;
// A progress bar, like QProgressBar.
pub const QStyle__CT_ProgressBar :QStyle__ContentsType = 6;
// A menu item, like QMenuItem.
pub const QStyle__CT_MenuItem :QStyle__ContentsType = 7;
// A menu bar item, like the buttons in a QMenuBar.
pub const QStyle__CT_MenuBarItem :QStyle__ContentsType = 8;
// A menu bar, like QMenuBar.
pub const QStyle__CT_MenuBar :QStyle__ContentsType = 9;
// 
pub const QStyle__CT_Menu :QStyle__ContentsType = 10;
// 
pub const QStyle__CT_TabBarTab :QStyle__ContentsType = 11;
// 
pub const QStyle__CT_Slider :QStyle__ContentsType = 12;
// 
pub const QStyle__CT_ScrollBar :QStyle__ContentsType = 13;
// 
pub const QStyle__CT_LineEdit :QStyle__ContentsType = 14;
// 
pub const QStyle__CT_SpinBox :QStyle__ContentsType = 15;
// 
pub const QStyle__CT_SizeGrip :QStyle__ContentsType = 16;
// 
pub const QStyle__CT_TabWidget :QStyle__ContentsType = 17;
// 
pub const QStyle__CT_DialogButtons :QStyle__ContentsType = 18;
// 
pub const QStyle__CT_HeaderSection :QStyle__ContentsType = 19;
// 
pub const QStyle__CT_GroupBox :QStyle__ContentsType = 20;
// 
pub const QStyle__CT_MdiControls :QStyle__ContentsType = 21;
// 
pub const QStyle__CT_ItemViewItem :QStyle__ContentsType = 22;
// 
pub const QStyle__CT_CustomBase :QStyle__ContentsType = -268435456;
pub fn QStyle_ContentsTypeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStyle", val);
}
pub fn QStyle_ContentsTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyle
  //return nilthis.ContentsTypeItemName(val);
  return QStyle_ContentsTypeItemName(val);
}


/*
This enum describes under what circumstances a software input panel will be requested by input capable widgets.



See also QInputMethod.

*/
pub type QStyle__RequestSoftwareInputPanel = i32;
// Requests an input panel if the user clicks on the widget, but only if it is already focused.
pub const QStyle__RSIP_OnMouseClickAndAlreadyFocused :QStyle__RequestSoftwareInputPanel = 0;
// Requests an input panel if the user clicks on the widget.
pub const QStyle__RSIP_OnMouseClick :QStyle__RequestSoftwareInputPanel = 1;
pub fn QStyle_RequestSoftwareInputPanelItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStyle", val);
}
pub fn QStyle_RequestSoftwareInputPanelItemName_s(val: i32) ->String {
  //var nilthis *QStyle
  //return nilthis.RequestSoftwareInputPanelItemName(val);
  return QStyle_RequestSoftwareInputPanelItemName(val);
}


/*
This enum describes the available style hints. A style hint is a general look and/or feel hint.

QStyle::SH_ScrollBar_ContextMenu?Whether or not a scroll bar has a context menu.
QStyle::SH_ScrollBar_LeftClickAbsolutePosition?A boolean value. If true, left clicking on a scroll bar causes the slider to jump to that position. If false, left clicking will behave as appropriate for each control.
QStyle::SH_ScrollBar_RollBetweenButtons?A boolean value. If true, when clicking a scroll bar button (SC_ScrollBarAddLine or SC_ScrollBarSubLine) and dragging over to the opposite button (rolling) will press the new button and release the old one. When it is false, the original button is released and nothing happens (like a push button).
QStyle::SH_Menu_KeyboardSearch?Typing causes a menu to be search for relevant items, otherwise only mnemnonic is considered.
QStyle::SH_Menu_Scrollable?Whether popup menus must support scrolling.
QStyle::SH_Menu_SloppySubMenus?Whether popup menus must support the user moving the mouse cursor to a submenu while crossing other items of the menu. This is supported on most modern desktop platforms.
QStyle::SH_Menu_FillScreenWithScroll?Whether scrolling popups should fill the screen as they are scrolled.
QStyle::SH_Menu_SelectionWrap?Whether popups should allow the selections to wrap, that is when selection should the next item be the first item.
QStyle::SH_ListViewExpand_SelectMouseType?Which type of mouse event should cause a list view expansion to be selected.
QStyle::SH_TabBar_PreferNoArrows?Whether a tab bar should suggest a size to prevent scoll arrows.
QStyle::SH_ScrollBar_StopMouseOverSliderSH_Slider_StopMouseOverSliderObsolete. Use SH_Slider_StopMouseOverSlider instead.
QStyle::SH_BlinkCursorWhenTextSelected?Whether cursor should blink when text is selected.
QStyle::SH_RichText_FullWidthSelection?Whether richtext selections should extend to the full width of the document.
QStyle::SH_GroupBox_TextLabelVerticalAlignment?How to vertically align a group box's text label.
QStyle::SH_GroupBox_TextLabelColor?How to paint a group box's text label.
QStyle::SH_DialogButtons_DefaultButton?Which button gets the default status in a dialog's button widget.
QStyle::SH_ToolBox_SelectedPageTitleBold?Boldness of the selected page title in a QToolBox.
QStyle::SH_LineEdit_PasswordCharacter?The Unicode character to be used for passwords.
QStyle::SH_Table_GridLineColor?The RGB value of the grid for a table.
QStyle::SH_UnderlineShortcut?Whether shortcuts are underlined.
QStyle::SH_SpellCheckUnderlineStyle?Obsolete. Use SpellCheckUnderlineStyle hint in QPlatformTheme instead.
QStyle::SH_SpinBox_AnimateButton?Animate a click when up or down is pressed in a spin box.
QStyle::SH_SpinBox_KeyPressAutoRepeatRate?Auto-repeat interval for spinbox key presses.
QStyle::SH_SpinBox_ClickAutoRepeatRate?Auto-repeat interval for spinbox mouse clicks.
QStyle::SH_SpinBox_ClickAutoRepeatThreshold?Auto-repeat threshold for spinbox mouse clicks.
QStyle::SH_DrawMenuBarSeparator?Indicates whether or not the menu bar draws separators.
QStyle::SH_TitleBar_ModifyNotification?Indicates if the title bar should show a '*' for windows that are modified.
QStyle::SH_Button_FocusPolicy?The default focus policy for buttons.
QStyle::SH_MessageBox_UseBorderForButtonSpacing?A boolean indicating what the to use the border of the buttons (computed as half the button height) for the spacing of the button in a message box.
QStyle::SH_MessageBox_CenterButtons?A boolean indicating whether the buttons in the message box should be centered or not (see QDialogButtonBox::setCentered()).
QStyle::SH_MessageBox_TextInteractionFlags?A boolean indicating if the text in a message box should allow user interfactions (e.g. selection) or not.
QStyle::SH_TitleBar_AutoRaise?A boolean indicating whether controls on a title bar ought to update when the mouse is over them.
QStyle::SH_ToolButton_PopupDelay?An int indicating the popup delay in milliseconds for menus attached to tool buttons.
QStyle::SH_FocusFrame_Mask?The mask of the focus frame.
QStyle::SH_RubberBand_Mask?The mask of the rubber band.
QStyle::SH_WindowFrame_Mask?The mask of the window frame.
QStyle::SH_SpinControls_DisableOnBounds?Determines if the spin controls will shown as disabled when reaching the spin range boundary.
QStyle::SH_Dial_BackgroundRole?Defines the style's preferred background role (as QPalette::ColorRole) for a dial widget.
QStyle::SH_ComboBox_LayoutDirection?The layout direction for the combo box. By default it should be the same as indicated by the QStyleOption::direction variable.
QStyle::SH_ItemView_EllipsisLocation?The location where ellipses should be added for item text that is too long to fit in an view item.
QStyle::SH_ItemView_ShowDecorationSelected?When an item in an item view is selected, also highlight the branch or other decoration.
QStyle::SH_ItemView_ActivateItemOnSingleClick?Emit the activated signal when the user single clicks on an item in an item in an item view. Otherwise the signal is emitted when the user double clicks on an item.
QStyle::SH_Slider_AbsoluteSetButtons?Which mouse buttons cause a slider to set the value to the position clicked on.
QStyle::SH_Slider_PageSetButtons?Which mouse buttons cause a slider to page step the value.
QStyle::SH_TabBar_ElideMode?The default eliding style for a tab bar.
QStyle::SH_DialogButtonLayout?Controls how buttons are laid out in a QDialogButtonBox, returns a QDialogButtonBox::ButtonLayout enum.
QStyle::SH_WizardStyle?Controls the look and feel of a QWizard. Returns a QWizard::WizardStyle enum.
QStyle::SH_FormLayoutWrapPolicy?Provides a default for how rows are wrapped in a QFormLayout. Returns a QFormLayout::RowWrapPolicy enum.
QStyle::SH_FormLayoutFieldGrowthPolicy?Provides a default for how fields can grow in a QFormLayout. Returns a QFormLayout::FieldGrowthPolicy enum.
QStyle::SH_FormLayoutFormAlignment?Provides a default for how a QFormLayout aligns its contents within the available space. Returns a Qt::Alignment enum.
QStyle::SH_FormLayoutLabelAlignment?Provides a default for how a QFormLayout aligns labels within the available space. Returns a Qt::Alignment enum.
QStyle::SH_ItemView_ArrowKeysNavigateIntoChildren?Controls whether the tree view will select the first child when it is exapanded and the right arrow key is pressed.
QStyle::SH_ComboBox_PopupFrameStyle?The frame style used when drawing a combobox popup menu.
QStyle::SH_DialogButtonBox_ButtonsHaveIcons?Indicates whether or not StandardButtons in QDialogButtonBox should have icons or not.
QStyle::SH_ItemView_MovementWithoutUpdatingSelection?The item view is able to indicate a current item without changing the selection.
QStyle::SH_ToolTip_Mask?The mask of a tool tip.
QStyle::SH_FocusFrame_AboveWidget?The FocusFrame is stacked above the widget that it is "focusing on".
QStyle::SH_TextControl_FocusIndicatorTextCharFormat?Specifies the text format used to highlight focused anchors in rich text documents displayed for example in QTextBrowser. The format has to be a QTextCharFormat returned in the variant of the QStyleHintReturnVariant return value. The QTextFormat::OutlinePen property is used for the outline and QTextFormat::BackgroundBrush for the background of the highlighted area.
QStyle::SH_Menu_FlashTriggeredItem?Flash triggered item.
QStyle::SH_Menu_FadeOutOnHide?Fade out the menu instead of hiding it immediately.
QStyle::SH_TabWidget_DefaultTabPosition?Default position of the tab bar in a tab widget.
QStyle::SH_ToolBar_Movable?Determines if the tool bar is movable by default.
QStyle::SH_ItemView_PaintAlternatingRowColorsForEmptyArea?Whether QTreeView paints alternating row colors for the area that does not have any items.
QStyle::SH_Menu_Mask?The mask for a popup menu.
QStyle::SH_ItemView_DrawDelegateFrame?Determines if there should be a frame for a delegate widget.
QStyle::SH_TabBar_CloseButtonPosition?Determines the position of the close button on a tab in a tab bar.
QStyle::SH_DockWidget_ButtonsHaveFrame?Determines if dockwidget buttons should have frames. Default is true.
QStyle::SH_ToolButtonStyle?Determines the default system style for tool buttons that uses Qt::ToolButtonFollowStyle.
QStyle::SH_RequestSoftwareInputPanel?Determines when a software input panel should be requested by input widgets. Returns an enum of type QStyle::RequestSoftwareInputPanel.
QStyle::SH_ScrollBar_Transient?Determines if the style supports transient scroll bars. Transient scroll bars appear when the content is scrolled and disappear when they are no longer needed.
QStyle::SH_Menu_SupportsSections?Determines if the style displays sections in menus or treat them as plain separators. Sections are separators with a text and icon hint.
QStyle::SH_ToolTip_WakeUpDelay?Determines the delay before a tooltip is shown, in milliseconds.
QStyle::SH_ToolTip_FallAsleepDelay?Determines the delay (in milliseconds) before a new wake time is needed when a tooltip is shown (notice: shown, not hidden). When a new wake isn't needed, a user-requested tooltip will be shown nearly instantly.
QStyle::SH_Widget_Animate?Deprecated. Use SH_Widget_Animation_Duration instead.


See also styleHint().

*/
pub type QStyle__StyleHint = i32;
// Disabled text is "etched" as it is on Windows.
pub const QStyle__SH_EtchDisabledText :QStyle__StyleHint = 0;
// Disabled text is dithered as it is on Motif.
pub const QStyle__SH_DitherDisabledText :QStyle__StyleHint = 1;
// A boolean value. If true, middle clicking on a scroll bar causes the slider to jump to that position. If false, middle clicking is ignored.
pub const QStyle__SH_ScrollBar_MiddleClickAbsolutePosition :QStyle__StyleHint = 2;
// A boolean value. If true, when clicking a scroll bar SubControl, holding the mouse button down and moving the pointer outside the SubControl, the scroll bar continues to scroll. If false, the scollbar stops scrolling when the pointer leaves the SubControl.
pub const QStyle__SH_ScrollBar_ScrollWhenPointerLeavesControl :QStyle__StyleHint = 3;
// Which type of mouse event should cause a tab to be selected.
pub const QStyle__SH_TabBar_SelectMouseType :QStyle__StyleHint = 4;
// The alignment for tabs in a QTabWidget. Possible values are Qt::AlignLeft, Qt::AlignCenter and Qt::AlignRight.
pub const QStyle__SH_TabBar_Alignment :QStyle__StyleHint = 5;
// The placement of the sorting indicator may appear in list or table headers. Possible values are Qt::Alignment values (that is, an OR combination of Qt::AlignmentFlag flags).
pub const QStyle__SH_Header_ArrowAlignment :QStyle__StyleHint = 6;
// Sliders snap to values while moving, as they do on Windows.
pub const QStyle__SH_Slider_SnapToValue :QStyle__StyleHint = 7;
// Key presses handled in a sloppy manner, i.e., left on a vertical slider subtracts a line.
pub const QStyle__SH_Slider_SloppyKeyEvents :QStyle__StyleHint = 8;
// Center button on progress dialogs, otherwise right aligned.
pub const QStyle__SH_ProgressDialog_CenterCancelButton :QStyle__StyleHint = 9;
// 
pub const QStyle__SH_ProgressDialog_TextLabelAlignment :QStyle__StyleHint = 10;
// 
pub const QStyle__SH_PrintDialog_RightAlignButtons :QStyle__StyleHint = 11;
// 
pub const QStyle__SH_MainWindow_SpaceBelowMenuBar :QStyle__StyleHint = 12;
// 
pub const QStyle__SH_FontDialog_SelectAssociatedText :QStyle__StyleHint = 13;
// 
pub const QStyle__SH_Menu_AllowActiveAndDisabled :QStyle__StyleHint = 14;
// 
pub const QStyle__SH_Menu_SpaceActivatesItem :QStyle__StyleHint = 15;
// 
pub const QStyle__SH_Menu_SubMenuPopupDelay :QStyle__StyleHint = 16;
// 
pub const QStyle__SH_ScrollView_FrameOnlyAroundContents :QStyle__StyleHint = 17;
// 
pub const QStyle__SH_MenuBar_AltKeyNavigation :QStyle__StyleHint = 18;
// 
pub const QStyle__SH_ComboBox_ListMouseTracking :QStyle__StyleHint = 19;
// 
pub const QStyle__SH_Menu_MouseTracking :QStyle__StyleHint = 20;
// 
pub const QStyle__SH_MenuBar_MouseTracking :QStyle__StyleHint = 21;
// 
pub const QStyle__SH_ItemView_ChangeHighlightOnFocus :QStyle__StyleHint = 22;
// 
pub const QStyle__SH_Widget_ShareActivation :QStyle__StyleHint = 23;
// 
pub const QStyle__SH_Workspace_FillSpaceOnMaximize :QStyle__StyleHint = 24;
// 
pub const QStyle__SH_ComboBox_Popup :QStyle__StyleHint = 25;
// 
pub const QStyle__SH_TitleBar_NoBorder :QStyle__StyleHint = 26;
// 
pub const QStyle__SH_Slider_StopMouseOverSlider :QStyle__StyleHint = 27;
// 
pub const QStyle__SH_ScrollBar_StopMouseOverSlider :QStyle__StyleHint = 27;
// 
pub const QStyle__SH_BlinkCursorWhenTextSelected :QStyle__StyleHint = 28;
// 
pub const QStyle__SH_RichText_FullWidthSelection :QStyle__StyleHint = 29;
// 
pub const QStyle__SH_Menu_Scrollable :QStyle__StyleHint = 30;
// 
pub const QStyle__SH_GroupBox_TextLabelVerticalAlignment :QStyle__StyleHint = 31;
// 
pub const QStyle__SH_GroupBox_TextLabelColor :QStyle__StyleHint = 32;
// 
pub const QStyle__SH_Menu_SloppySubMenus :QStyle__StyleHint = 33;
// 
pub const QStyle__SH_Table_GridLineColor :QStyle__StyleHint = 34;
// 
pub const QStyle__SH_LineEdit_PasswordCharacter :QStyle__StyleHint = 35;
// 
pub const QStyle__SH_DialogButtons_DefaultButton :QStyle__StyleHint = 36;
// 
pub const QStyle__SH_ToolBox_SelectedPageTitleBold :QStyle__StyleHint = 37;
// 
pub const QStyle__SH_TabBar_PreferNoArrows :QStyle__StyleHint = 38;
// 
pub const QStyle__SH_ScrollBar_LeftClickAbsolutePosition :QStyle__StyleHint = 39;
// 
pub const QStyle__SH_ListViewExpand_SelectMouseType :QStyle__StyleHint = 40;
// 
pub const QStyle__SH_UnderlineShortcut :QStyle__StyleHint = 41;
// 
pub const QStyle__SH_SpinBox_AnimateButton :QStyle__StyleHint = 42;
// 
pub const QStyle__SH_SpinBox_KeyPressAutoRepeatRate :QStyle__StyleHint = 43;
// 
pub const QStyle__SH_SpinBox_ClickAutoRepeatRate :QStyle__StyleHint = 44;
// 
pub const QStyle__SH_Menu_FillScreenWithScroll :QStyle__StyleHint = 45;
// 
pub const QStyle__SH_ToolTipLabel_Opacity :QStyle__StyleHint = 46;
// 
pub const QStyle__SH_DrawMenuBarSeparator :QStyle__StyleHint = 47;
// 
pub const QStyle__SH_TitleBar_ModifyNotification :QStyle__StyleHint = 48;
// 
pub const QStyle__SH_Button_FocusPolicy :QStyle__StyleHint = 49;
// 
pub const QStyle__SH_MessageBox_UseBorderForButtonSpacing :QStyle__StyleHint = 50;
// 
pub const QStyle__SH_TitleBar_AutoRaise :QStyle__StyleHint = 51;
// 
pub const QStyle__SH_ToolButton_PopupDelay :QStyle__StyleHint = 52;
// 
pub const QStyle__SH_FocusFrame_Mask :QStyle__StyleHint = 53;
// 
pub const QStyle__SH_RubberBand_Mask :QStyle__StyleHint = 54;
// 
pub const QStyle__SH_WindowFrame_Mask :QStyle__StyleHint = 55;
// 
pub const QStyle__SH_SpinControls_DisableOnBounds :QStyle__StyleHint = 56;
// 
pub const QStyle__SH_Dial_BackgroundRole :QStyle__StyleHint = 57;
// 
pub const QStyle__SH_ComboBox_LayoutDirection :QStyle__StyleHint = 58;
// 
pub const QStyle__SH_ItemView_EllipsisLocation :QStyle__StyleHint = 59;
// 
pub const QStyle__SH_ItemView_ShowDecorationSelected :QStyle__StyleHint = 60;
// 
pub const QStyle__SH_ItemView_ActivateItemOnSingleClick :QStyle__StyleHint = 61;
// 
pub const QStyle__SH_ScrollBar_ContextMenu :QStyle__StyleHint = 62;
// 
pub const QStyle__SH_ScrollBar_RollBetweenButtons :QStyle__StyleHint = 63;
// 
pub const QStyle__SH_Slider_AbsoluteSetButtons :QStyle__StyleHint = 64;
// 
pub const QStyle__SH_Slider_PageSetButtons :QStyle__StyleHint = 65;
// 
pub const QStyle__SH_Menu_KeyboardSearch :QStyle__StyleHint = 66;
// 
pub const QStyle__SH_TabBar_ElideMode :QStyle__StyleHint = 67;
// 
pub const QStyle__SH_DialogButtonLayout :QStyle__StyleHint = 68;
// 
pub const QStyle__SH_ComboBox_PopupFrameStyle :QStyle__StyleHint = 69;
// 
pub const QStyle__SH_MessageBox_TextInteractionFlags :QStyle__StyleHint = 70;
// 
pub const QStyle__SH_DialogButtonBox_ButtonsHaveIcons :QStyle__StyleHint = 71;
// 
pub const QStyle__SH_SpellCheckUnderlineStyle :QStyle__StyleHint = 72;
// 
pub const QStyle__SH_MessageBox_CenterButtons :QStyle__StyleHint = 73;
// 
pub const QStyle__SH_Menu_SelectionWrap :QStyle__StyleHint = 74;
// 
pub const QStyle__SH_ItemView_MovementWithoutUpdatingSelection :QStyle__StyleHint = 75;
// 
pub const QStyle__SH_ToolTip_Mask :QStyle__StyleHint = 76;
// 
pub const QStyle__SH_FocusFrame_AboveWidget :QStyle__StyleHint = 77;
// 
pub const QStyle__SH_TextControl_FocusIndicatorTextCharFormat :QStyle__StyleHint = 78;
// 
pub const QStyle__SH_WizardStyle :QStyle__StyleHint = 79;
// 
pub const QStyle__SH_ItemView_ArrowKeysNavigateIntoChildren :QStyle__StyleHint = 80;
// 
pub const QStyle__SH_Menu_Mask :QStyle__StyleHint = 81;
// 
pub const QStyle__SH_Menu_FlashTriggeredItem :QStyle__StyleHint = 82;
// 
pub const QStyle__SH_Menu_FadeOutOnHide :QStyle__StyleHint = 83;
// 
pub const QStyle__SH_SpinBox_ClickAutoRepeatThreshold :QStyle__StyleHint = 84;
// 
pub const QStyle__SH_ItemView_PaintAlternatingRowColorsForEmptyArea :QStyle__StyleHint = 85;
// 
pub const QStyle__SH_FormLayoutWrapPolicy :QStyle__StyleHint = 86;
// 
pub const QStyle__SH_TabWidget_DefaultTabPosition :QStyle__StyleHint = 87;
// 
pub const QStyle__SH_ToolBar_Movable :QStyle__StyleHint = 88;
// 
pub const QStyle__SH_FormLayoutFieldGrowthPolicy :QStyle__StyleHint = 89;
// 
pub const QStyle__SH_FormLayoutFormAlignment :QStyle__StyleHint = 90;
// 
pub const QStyle__SH_FormLayoutLabelAlignment :QStyle__StyleHint = 91;
// 
pub const QStyle__SH_ItemView_DrawDelegateFrame :QStyle__StyleHint = 92;
// 
pub const QStyle__SH_TabBar_CloseButtonPosition :QStyle__StyleHint = 93;
// 
pub const QStyle__SH_DockWidget_ButtonsHaveFrame :QStyle__StyleHint = 94;
// 
pub const QStyle__SH_ToolButtonStyle :QStyle__StyleHint = 95;
// 
pub const QStyle__SH_RequestSoftwareInputPanel :QStyle__StyleHint = 96;
// 
pub const QStyle__SH_ScrollBar_Transient :QStyle__StyleHint = 97;
// 
pub const QStyle__SH_Menu_SupportsSections :QStyle__StyleHint = 98;
// 
pub const QStyle__SH_ToolTip_WakeUpDelay :QStyle__StyleHint = 99;
// 
pub const QStyle__SH_ToolTip_FallAsleepDelay :QStyle__StyleHint = 100;
// 
pub const QStyle__SH_Widget_Animate :QStyle__StyleHint = 101;
// 
pub const QStyle__SH_Splitter_OpaqueResize :QStyle__StyleHint = 102;
// 
pub const QStyle__SH_ComboBox_UseNativePopup :QStyle__StyleHint = 103;
// 
pub const QStyle__SH_LineEdit_PasswordMaskDelay :QStyle__StyleHint = 104;
// 
pub const QStyle__SH_TabBar_ChangeCurrentDelay :QStyle__StyleHint = 105;
// 
pub const QStyle__SH_Menu_SubMenuUniDirection :QStyle__StyleHint = 106;
// 
pub const QStyle__SH_Menu_SubMenuUniDirectionFailCount :QStyle__StyleHint = 107;
// 
pub const QStyle__SH_Menu_SubMenuSloppySelectOtherActions :QStyle__StyleHint = 108;
// 
pub const QStyle__SH_Menu_SubMenuSloppyCloseTimeout :QStyle__StyleHint = 109;
// 
pub const QStyle__SH_Menu_SubMenuResetWhenReenteringParent :QStyle__StyleHint = 110;
// 
pub const QStyle__SH_Menu_SubMenuDontStartSloppyOnLeave :QStyle__StyleHint = 111;
// 
pub const QStyle__SH_ItemView_ScrollMode :QStyle__StyleHint = 112;
// 
pub const QStyle__SH_TitleBar_ShowToolTipsOnButtons :QStyle__StyleHint = 113;
// 
pub const QStyle__SH_Widget_Animation_Duration :QStyle__StyleHint = 114;
// 
pub const QStyle__SH_CustomBase :QStyle__StyleHint = -268435456;
pub fn QStyle_StyleHintItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStyle", val);
}
pub fn QStyle_StyleHintItemName_s(val: i32) ->String {
  //var nilthis *QStyle
  //return nilthis.StyleHintItemName(val);
  return QStyle_StyleHintItemName(val);
}


/*
This enum describes the available standard pixmaps. A standard pixmap is a pixmap that can follow some existing GUI style or guideline.



See also standardIcon().

*/
pub type QStyle__StandardPixmap = i32;
// Menu button on a title bar.
pub const QStyle__SP_TitleBarMenuButton :QStyle__StandardPixmap = 0;
// Minimize button on title bars (e.g., in QMdiSubWindow).
pub const QStyle__SP_TitleBarMinButton :QStyle__StandardPixmap = 1;
// Maximize button on title bars.
pub const QStyle__SP_TitleBarMaxButton :QStyle__StandardPixmap = 2;
// Close button on title bars.
pub const QStyle__SP_TitleBarCloseButton :QStyle__StandardPixmap = 3;
// Normal (restore) button on title bars.
pub const QStyle__SP_TitleBarNormalButton :QStyle__StandardPixmap = 4;
// Shade button on title bars.
pub const QStyle__SP_TitleBarShadeButton :QStyle__StandardPixmap = 5;
// Unshade button on title bars.
pub const QStyle__SP_TitleBarUnshadeButton :QStyle__StandardPixmap = 6;
// The Context help button on title bars.
pub const QStyle__SP_TitleBarContextHelpButton :QStyle__StandardPixmap = 7;
// Close button on dock windows (see also QDockWidget).
pub const QStyle__SP_DockWidgetCloseButton :QStyle__StandardPixmap = 8;
// The "information" icon.
pub const QStyle__SP_MessageBoxInformation :QStyle__StandardPixmap = 9;
// 
pub const QStyle__SP_MessageBoxWarning :QStyle__StandardPixmap = 10;
// 
pub const QStyle__SP_MessageBoxCritical :QStyle__StandardPixmap = 11;
// 
pub const QStyle__SP_MessageBoxQuestion :QStyle__StandardPixmap = 12;
// 
pub const QStyle__SP_DesktopIcon :QStyle__StandardPixmap = 13;
// 
pub const QStyle__SP_TrashIcon :QStyle__StandardPixmap = 14;
// 
pub const QStyle__SP_ComputerIcon :QStyle__StandardPixmap = 15;
// 
pub const QStyle__SP_DriveFDIcon :QStyle__StandardPixmap = 16;
// 
pub const QStyle__SP_DriveHDIcon :QStyle__StandardPixmap = 17;
// 
pub const QStyle__SP_DriveCDIcon :QStyle__StandardPixmap = 18;
// 
pub const QStyle__SP_DriveDVDIcon :QStyle__StandardPixmap = 19;
// 
pub const QStyle__SP_DriveNetIcon :QStyle__StandardPixmap = 20;
// 
pub const QStyle__SP_DirOpenIcon :QStyle__StandardPixmap = 21;
// 
pub const QStyle__SP_DirClosedIcon :QStyle__StandardPixmap = 22;
// 
pub const QStyle__SP_DirLinkIcon :QStyle__StandardPixmap = 23;
// 
pub const QStyle__SP_DirLinkOpenIcon :QStyle__StandardPixmap = 24;
// 
pub const QStyle__SP_FileIcon :QStyle__StandardPixmap = 25;
// 
pub const QStyle__SP_FileLinkIcon :QStyle__StandardPixmap = 26;
// 
pub const QStyle__SP_ToolBarHorizontalExtensionButton :QStyle__StandardPixmap = 27;
// 
pub const QStyle__SP_ToolBarVerticalExtensionButton :QStyle__StandardPixmap = 28;
// 
pub const QStyle__SP_FileDialogStart :QStyle__StandardPixmap = 29;
// 
pub const QStyle__SP_FileDialogEnd :QStyle__StandardPixmap = 30;
// 
pub const QStyle__SP_FileDialogToParent :QStyle__StandardPixmap = 31;
// 
pub const QStyle__SP_FileDialogNewFolder :QStyle__StandardPixmap = 32;
// 
pub const QStyle__SP_FileDialogDetailedView :QStyle__StandardPixmap = 33;
// 
pub const QStyle__SP_FileDialogInfoView :QStyle__StandardPixmap = 34;
// 
pub const QStyle__SP_FileDialogContentsView :QStyle__StandardPixmap = 35;
// 
pub const QStyle__SP_FileDialogListView :QStyle__StandardPixmap = 36;
// 
pub const QStyle__SP_FileDialogBack :QStyle__StandardPixmap = 37;
// 
pub const QStyle__SP_DirIcon :QStyle__StandardPixmap = 38;
// 
pub const QStyle__SP_DialogOkButton :QStyle__StandardPixmap = 39;
// 
pub const QStyle__SP_DialogCancelButton :QStyle__StandardPixmap = 40;
// 
pub const QStyle__SP_DialogHelpButton :QStyle__StandardPixmap = 41;
// 
pub const QStyle__SP_DialogOpenButton :QStyle__StandardPixmap = 42;
// 
pub const QStyle__SP_DialogSaveButton :QStyle__StandardPixmap = 43;
// 
pub const QStyle__SP_DialogCloseButton :QStyle__StandardPixmap = 44;
// 
pub const QStyle__SP_DialogApplyButton :QStyle__StandardPixmap = 45;
// 
pub const QStyle__SP_DialogResetButton :QStyle__StandardPixmap = 46;
// 
pub const QStyle__SP_DialogDiscardButton :QStyle__StandardPixmap = 47;
// 
pub const QStyle__SP_DialogYesButton :QStyle__StandardPixmap = 48;
// 
pub const QStyle__SP_DialogNoButton :QStyle__StandardPixmap = 49;
// 
pub const QStyle__SP_ArrowUp :QStyle__StandardPixmap = 50;
// 
pub const QStyle__SP_ArrowDown :QStyle__StandardPixmap = 51;
// 
pub const QStyle__SP_ArrowLeft :QStyle__StandardPixmap = 52;
// 
pub const QStyle__SP_ArrowRight :QStyle__StandardPixmap = 53;
// 
pub const QStyle__SP_ArrowBack :QStyle__StandardPixmap = 54;
// 
pub const QStyle__SP_ArrowForward :QStyle__StandardPixmap = 55;
// 
pub const QStyle__SP_DirHomeIcon :QStyle__StandardPixmap = 56;
// 
pub const QStyle__SP_CommandLink :QStyle__StandardPixmap = 57;
// 
pub const QStyle__SP_VistaShield :QStyle__StandardPixmap = 58;
// 
pub const QStyle__SP_BrowserReload :QStyle__StandardPixmap = 59;
// 
pub const QStyle__SP_BrowserStop :QStyle__StandardPixmap = 60;
// 
pub const QStyle__SP_MediaPlay :QStyle__StandardPixmap = 61;
// 
pub const QStyle__SP_MediaStop :QStyle__StandardPixmap = 62;
// 
pub const QStyle__SP_MediaPause :QStyle__StandardPixmap = 63;
// 
pub const QStyle__SP_MediaSkipForward :QStyle__StandardPixmap = 64;
// 
pub const QStyle__SP_MediaSkipBackward :QStyle__StandardPixmap = 65;
// 
pub const QStyle__SP_MediaSeekForward :QStyle__StandardPixmap = 66;
// 
pub const QStyle__SP_MediaSeekBackward :QStyle__StandardPixmap = 67;
// 
pub const QStyle__SP_MediaVolume :QStyle__StandardPixmap = 68;
// 
pub const QStyle__SP_MediaVolumeMuted :QStyle__StandardPixmap = 69;
// 
pub const QStyle__SP_LineEditClearButton :QStyle__StandardPixmap = 70;
// 
pub const QStyle__SP_CustomBase :QStyle__StandardPixmap = -268435456;
pub fn QStyle_StandardPixmapItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStyle", val);
}
pub fn QStyle_StandardPixmapItemName_s(val: i32) ->String {
  //var nilthis *QStyle
  //return nilthis.StandardPixmapItemName(val);
  return QStyle_StandardPixmapItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
