

// mod ::widgets::QToolTip
// package qtwidgets
// /usr/include/qt/QtWidgets/qtooltip.h
// #include <qtooltip.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 31
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
#[derive(Default)] // class sizeof(QToolTip)=1
pub struct QToolTip {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QToolTip_ITF interface {
//    QToolTip_PTR() *QToolTip
//}
//func (ptr *QToolTip) QToolTip_PTR() *QToolTip { return ptr }

impl /*struct*/ QToolTip {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QToolTip {
    return QToolTip{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QToolTip {
//  type Target = QToolTipBASE;
//
//  fn deref(&self) -> &QToolTipBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QToolTipBASE> for QToolTip {
//  fn as_ref(& self) -> & QToolTipBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtooltip.h:56
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void showText(const QPoint &, const QString &, QWidget *)

/*
Shows text as a tool tip, with the global position pos as the point of interest. The tool tip will be shown with a platform specific offset from this point of interest.

If you specify a non-empty rect the tip will be hidden as soon as you move your cursor out of this area.

The rect is in the coordinates of the widget you specify with w. If the rect is not empty you must specify a widget. Otherwise this argument can be 0 but it is used to determine the appropriate screen on multi-head systems.

If text is empty the tool tip is hidden. If the text is the same as the currently shown tooltip, the tip will not move. You can force moving by first hiding the tip with an empty text, and then showing the new tip at the new position.
*/
impl /*struct*/ QToolTip {
  pub fn showText_0<RetType, T: QToolTip_showText_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.showText_0();
    // return 1;
  }
}
pub trait QToolTip_showText_0<RetType> {
  fn showText_0(self ) -> RetType;
}
impl<'a> /*trait*/ QToolTip_showText_0<(/*void*/)> for (usize,usize,usize) {
  fn showText_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtooltip.h:57
// index:1
// Public static Visibility=Default Availability=Available
// [-2] void showText(const QPoint &, const QString &, QWidget *, const QRect &)

/*
Shows text as a tool tip, with the global position pos as the point of interest. The tool tip will be shown with a platform specific offset from this point of interest.

If you specify a non-empty rect the tip will be hidden as soon as you move your cursor out of this area.

The rect is in the coordinates of the widget you specify with w. If the rect is not empty you must specify a widget. Otherwise this argument can be 0 but it is used to determine the appropriate screen on multi-head systems.

If text is empty the tool tip is hidden. If the text is the same as the currently shown tooltip, the tip will not move. You can force moving by first hiding the tip with an empty text, and then showing the new tip at the new position.
*/
impl /*struct*/ QToolTip {
  pub fn showText_1<RetType, T: QToolTip_showText_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.showText_1();
    // return 1;
  }
}
pub trait QToolTip_showText_1<RetType> {
  fn showText_1(self ) -> RetType;
}
impl<'a> /*trait*/ QToolTip_showText_1<(/*void*/)> for (usize,usize,usize,usize) {
  fn showText_1(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRect", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtooltip.h:58
// index:2
// Public static Visibility=Default Availability=Available
// [-2] void showText(const QPoint &, const QString &, QWidget *, const QRect &, int)

/*
Shows text as a tool tip, with the global position pos as the point of interest. The tool tip will be shown with a platform specific offset from this point of interest.

If you specify a non-empty rect the tip will be hidden as soon as you move your cursor out of this area.

The rect is in the coordinates of the widget you specify with w. If the rect is not empty you must specify a widget. Otherwise this argument can be 0 but it is used to determine the appropriate screen on multi-head systems.

If text is empty the tool tip is hidden. If the text is the same as the currently shown tooltip, the tip will not move. You can force moving by first hiding the tip with an empty text, and then showing the new tip at the new position.
*/
impl /*struct*/ QToolTip {
  pub fn showText_2<RetType, T: QToolTip_showText_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.showText_2();
    // return 1;
  }
}
pub trait QToolTip_showText_2<RetType> {
  fn showText_2(self ) -> RetType;
}
impl<'a> /*trait*/ QToolTip_showText_2<(/*void*/)> for (usize,usize,usize,usize,i32) {
  fn showText_2(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRecti", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtooltip.h:59
// index:0
// Public static inline Visibility=Default Availability=Available
// [-2] void hideText()

/*
Hides the tool tip. This is the same as calling showText() with an empty string.

This function was introduced in  Qt 4.2.

See also showText().
*/
impl /*struct*/ QToolTip {
  pub fn hideText_0<RetType, T: QToolTip_hideText_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.hideText_0();
    // return 1;
  }
}
pub trait QToolTip_hideText_0<RetType> {
  fn hideText_0(self ) -> RetType;
}
impl<'a> /*trait*/ QToolTip_hideText_0<(/*void*/)> for () {
  fn hideText_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QToolTip8hideTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtooltip.h:61
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isVisible()

/*
Returns true if this tooltip is currently shown.

This function was introduced in  Qt 4.4.

See also showText().
*/
impl /*struct*/ QToolTip {
  pub fn isVisible_0<RetType, T: QToolTip_isVisible_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isVisible_0();
    // return 1;
  }
}
pub trait QToolTip_isVisible_0<RetType> {
  fn isVisible_0(self ) -> RetType;
}
impl<'a> /*trait*/ QToolTip_isVisible_0<bool> for () {
  fn isVisible_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolTip9isVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtooltip.h:62
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString text()

/*
Returns the tooltip text, if a tooltip is visible, or an empty string if a tooltip is not visible.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QToolTip {
  pub fn text_0<RetType, T: QToolTip_text_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.text_0();
    // return 1;
  }
}
pub trait QToolTip_text_0<RetType> {
  fn text_0(self ) -> RetType;
}
impl<'a> /*trait*/ QToolTip_text_0<usize> for () {
  fn text_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolTip4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtooltip.h:64
// index:0
// Public static Visibility=Default Availability=Available
// [16] QPalette palette()

/*
Returns the palette used to render tooltips.

Note: Tool tips use the inactive color group of QPalette, because tool tips are not active windows.

See also setPalette().
*/
impl /*struct*/ QToolTip {
  pub fn palette_0<RetType, T: QToolTip_palette_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.palette_0();
    // return 1;
  }
}
pub trait QToolTip_palette_0<RetType> {
  fn palette_0(self ) -> RetType;
}
impl<'a> /*trait*/ QToolTip_palette_0<usize> for () {
  fn palette_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolTip7paletteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtooltip.h:65
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setPalette(const QPalette &)

/*
Sets the palette used to render tooltips.

Note: Tool tips use the inactive color group of QPalette, because tool tips are not active windows.

This function was introduced in  Qt 4.2.

See also palette().
*/
impl /*struct*/ QToolTip {
  pub fn setPalette_0<RetType, T: QToolTip_setPalette_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPalette_0();
    // return 1;
  }
}
pub trait QToolTip_setPalette_0<RetType> {
  fn setPalette_0(self ) -> RetType;
}
impl<'a> /*trait*/ QToolTip_setPalette_0<(/*void*/)> for (usize) {
  fn setPalette_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolTip10setPaletteERK8QPalette", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtooltip.h:66
// index:0
// Public static Visibility=Default Availability=Available
// [16] QFont font()

/*
Returns the font used to render tooltips.

This function was introduced in  Qt 4.2.

See also setFont().
*/
impl /*struct*/ QToolTip {
  pub fn font_0<RetType, T: QToolTip_font_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.font_0();
    // return 1;
  }
}
pub trait QToolTip_font_0<RetType> {
  fn font_0(self ) -> RetType;
}
impl<'a> /*trait*/ QToolTip_font_0<usize> for () {
  fn font_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QToolTip4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtooltip.h:67
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*
Sets the font used to render tooltips.

This function was introduced in  Qt 4.2.

See also font().
*/
impl /*struct*/ QToolTip {
  pub fn setFont_0<RetType, T: QToolTip_setFont_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setFont_0();
    // return 1;
  }
}
pub trait QToolTip_setFont_0<RetType> {
  fn setFont_0(self ) -> RetType;
}
impl<'a> /*trait*/ QToolTip_setFont_0<(/*void*/)> for (usize) {
  fn setFont_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QToolTip7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQToolTip(this :*mut QToolTip) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN8QToolTipD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
