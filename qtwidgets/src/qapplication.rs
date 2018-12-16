

// mod ::widgets::QApplication
// package qtwidgets
// /usr/include/qt/QtWidgets/qapplication.h
// #include <qapplication.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 17
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
// func (this *QApplication) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QApplication)=16
pub struct QApplication {
  qbase: QGuiApplication,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QApplication_ITF interface {
//    qtgui.QGuiApplication_ITF
//    QApplication_PTR() *QApplication
//}
//func (ptr *QApplication) QApplication_PTR() *QApplication { return ptr }

impl /*struct*/ QApplication {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QApplication {
    return QApplication{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QApplication {
//  type Target = QApplicationBASE;
//
//  fn deref(&self) -> &QApplicationBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QApplicationBASE> for QApplication {
//  fn as_ref(& self) -> & QApplicationBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qapplication.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QApplication {
  pub fn metaObject_0<RetType, T: QApplication_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QApplication_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QApplication) -> RetType;
}
impl<'a> /*trait*/ QApplication_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QApplication) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QApplication10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QApplication(int &, char **, int)

/*
Initializes the window system and constructs an application object with argc command line arguments in argv.

Warning: The data referred to by argc and argv must stay valid for the entire lifetime of the QApplication object. In addition, argc must be greater than zero and argv must contain at least one valid character string.

The global qApp pointer refers to this application object. Only one application object should be created.

This application object must be constructed before any paint devices (including widgets, pixmaps, bitmaps etc.).

Note: argc and argv might be changed as Qt removes command line arguments that it recognizes.

All Qt programs automatically support the following command line options:


-style= style, sets the application GUI style. Possible values depend on your system configuration. If you compiled Qt with additional styles or have additional styles as plugins these will be available to the -style command line option. You can also set the style for all Qt applications by setting the QT_STYLE_OVERRIDE environment variable.
-style style, is the same as listed above.
-stylesheet= stylesheet, sets the application styleSheet. The value must be a path to a file that contains the Style Sheet.Note: Relative URLs in the Style Sheet file are relative to the Style Sheet file's path.
-stylesheet stylesheet, is the same as listed above.
-widgetcount, prints debug message at the end about number of widgets left undestroyed and maximum number of widgets existed at the same time
-reverse, sets the application's layout direction to Qt::RightToLeft
-qmljsdebugger=, activates the QML/JS debugger with a specified port. The value must be of format port:1234[,block], where block is optional and will make the application wait until a debugger connects to it.


See also QCoreApplication::arguments().
*/
// QApplication(int &, char **, int) ctx.fn_proto_cpp
impl /*struct*/ QApplication {
  pub fn QApplication_0<T: QApplication_QApplication_0>(value: T) -> QApplication {
    let rsthis = value.QApplication_0();
    return rsthis;
    // return 1;
  }
}

pub trait QApplication_QApplication_0 {
  fn QApplication_0(self) -> QApplication;
}
// QApplication(int &, char **, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QApplication_QApplication_0 for (usize,usize,i32) {
  fn QApplication_0(self) -> QApplication {
    // unsafe{_ZN12QApplicationC2ERiPPci()};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QApplicationC2ERiPPci", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QApplication{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:96
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QApplication()

/*

*/
pub fn DeleteQApplication(this :*mut QApplication) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QApplicationD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qapplication.h:98
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStyle * style()

/*
Returns the application's style object.

See also setStyle() and QStyle.
*/
impl /*struct*/ QApplication {
  pub fn style_0<RetType, T: QApplication_style_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.style_0();
    // return 1;
  }
}
pub trait QApplication_style_0<RetType> {
  fn style_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_style_0<usize> for () {
  fn style_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication5styleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:99
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setStyle(QStyle *)

/*
Sets the application's GUI style to style. Ownership of the style object is transferred to QApplication, so QApplication will delete the style object on application exit or when a new style is set and the old style is still the parent of the application object.

Example usage:


  QApplication::setStyle(QStyleFactory::create("Fusion"));



When switching application styles, the color palette is set back to the initial colors or the system defaults. This is necessary since certain styles have to adapt the color palette to be fully style-guide compliant.

Setting the style before a palette has been set, i.e., before creating QApplication, will cause the application to use QStyle::standardPalette() for the palette.

Warning: Qt style sheets are currently not supported for custom QStyle subclasses. We plan to address this in some future release.

See also style(), QStyle, setPalette(), and desktopSettingsAware().
*/
impl /*struct*/ QApplication {
  pub fn setStyle_0<RetType, T: QApplication_setStyle_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setStyle_0();
    // return 1;
  }
}
pub trait QApplication_setStyle_0<RetType> {
  fn setStyle_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setStyle_0<(/*void*/)> for (usize) {
  fn setStyle_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication8setStyleEP6QStyle", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:100
// index:1
// Public static Visibility=Default Availability=Available
// [8] QStyle * setStyle(const QString &)

/*
Sets the application's GUI style to style. Ownership of the style object is transferred to QApplication, so QApplication will delete the style object on application exit or when a new style is set and the old style is still the parent of the application object.

Example usage:


  QApplication::setStyle(QStyleFactory::create("Fusion"));



When switching application styles, the color palette is set back to the initial colors or the system defaults. This is necessary since certain styles have to adapt the color palette to be fully style-guide compliant.

Setting the style before a palette has been set, i.e., before creating QApplication, will cause the application to use QStyle::standardPalette() for the palette.

Warning: Qt style sheets are currently not supported for custom QStyle subclasses. We plan to address this in some future release.

See also style(), QStyle, setPalette(), and desktopSettingsAware().
*/
impl /*struct*/ QApplication {
  pub fn setStyle_1<RetType, T: QApplication_setStyle_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.setStyle_1();
    // return 1;
  }
}
pub trait QApplication_setStyle_1<RetType> {
  fn setStyle_1(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setStyle_1<usize> for (usize) {
  fn setStyle_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication8setStyleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:103
// index:0
// Public static Visibility=Default Availability=Available
// [4] int colorSpec()

/*

*/
impl /*struct*/ QApplication {
  pub fn colorSpec_0<RetType, T: QApplication_colorSpec_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.colorSpec_0();
    // return 1;
  }
}
pub trait QApplication_colorSpec_0<RetType> {
  fn colorSpec_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_colorSpec_0<i32> for () {
  fn colorSpec_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication9colorSpecEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:104
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setColorSpec(int)

/*

*/
impl /*struct*/ QApplication {
  pub fn setColorSpec_0<RetType, T: QApplication_setColorSpec_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setColorSpec_0();
    // return 1;
  }
}
pub trait QApplication_setColorSpec_0<RetType> {
  fn setColorSpec_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setColorSpec_0<(/*void*/)> for (i32) {
  fn setColorSpec_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication12setColorSpecEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:111
// index:0
// Public static Visibility=Default Availability=Available
// [16] QPalette palette(const QWidget *)

/*
This is an overloaded function.

If a widget is passed, the default palette for the widget's class is returned. This may or may not be the application palette. In most cases there is no special palette for certain types of widgets, but one notable exception is the popup menu under Windows, if the user has defined a special background color for menus in the display settings.

See also setPalette() and QWidget::palette().
*/
impl /*struct*/ QApplication {
  pub fn palette_0<RetType, T: QApplication_palette_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.palette_0();
    // return 1;
  }
}
pub trait QApplication_palette_0<RetType> {
  fn palette_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_palette_0<usize> for (usize) {
  fn palette_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication7paletteEPK7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:112
// index:1
// Public static Visibility=Default Availability=Available
// [16] QPalette palette(const char *)

/*
This is an overloaded function.

If a widget is passed, the default palette for the widget's class is returned. This may or may not be the application palette. In most cases there is no special palette for certain types of widgets, but one notable exception is the popup menu under Windows, if the user has defined a special background color for menus in the display settings.

See also setPalette() and QWidget::palette().
*/
impl /*struct*/ QApplication {
  pub fn palette_1<RetType, T: QApplication_palette_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.palette_1();
    // return 1;
  }
}
pub trait QApplication_palette_1<RetType> {
  fn palette_1(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_palette_1<usize> for (usize) {
  fn palette_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication7paletteEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:113
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setPalette(const QPalette &, const char *)

/*
Changes the default application palette to palette.

If className is passed, the change applies only to widgets that inherit className (as reported by QObject::inherits()). If className is left 0, the change affects all widgets, thus overriding any previously set class specific palettes.

The palette may be changed according to the current GUI style in QStyle::polish().

Warning: Do not use this function in conjunction with Qt Style Sheets. When using style sheets, the palette of a widget can be customized using the "color", "background-color", "selection-color", "selection-background-color" and "alternate-background-color".

Note: Some styles do not use the palette for all drawing, for instance, if they make use of native theme engines. This is the case for the Windows Vista and macOS styles.

See also QWidget::setPalette(), palette(), and QStyle::polish().
*/
impl /*struct*/ QApplication {
  pub fn setPalette_0<RetType, T: QApplication_setPalette_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPalette_0();
    // return 1;
  }
}
pub trait QApplication_setPalette_0<RetType> {
  fn setPalette_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setPalette_0<(/*void*/)> for (usize,usize) {
  fn setPalette_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication10setPaletteERK8QPalettePKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:114
// index:0
// Public static Visibility=Default Availability=Available
// [16] QFont font()

/*
Returns the default application font.

See also setFont(), fontMetrics(), and QWidget::font().
*/
impl /*struct*/ QApplication {
  pub fn font_0<RetType, T: QApplication_font_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.font_0();
    // return 1;
  }
}
pub trait QApplication_font_0<RetType> {
  fn font_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_font_0<usize> for () {
  fn font_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:115
// index:1
// Public static Visibility=Default Availability=Available
// [16] QFont font(const QWidget *)

/*
Returns the default application font.

See also setFont(), fontMetrics(), and QWidget::font().
*/
impl /*struct*/ QApplication {
  pub fn font_1<RetType, T: QApplication_font_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.font_1();
    // return 1;
  }
}
pub trait QApplication_font_1<RetType> {
  fn font_1(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_font_1<usize> for (usize) {
  fn font_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication4fontEPK7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:116
// index:2
// Public static Visibility=Default Availability=Available
// [16] QFont font(const char *)

/*
Returns the default application font.

See also setFont(), fontMetrics(), and QWidget::font().
*/
impl /*struct*/ QApplication {
  pub fn font_2<RetType, T: QApplication_font_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.font_2();
    // return 1;
  }
}
pub trait QApplication_font_2<RetType> {
  fn font_2(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_font_2<usize> for (usize) {
  fn font_2(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication4fontEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:117
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setFont(const QFont &, const char *)

/*
Changes the default application font to font. If className is passed, the change applies only to classes that inherit className (as reported by QObject::inherits()).

On application start-up, the default font depends on the window system. It can vary depending on both the window system version and the locale. This function lets you override the default font; but overriding may be a bad idea because, for example, some locales need extra large fonts to support their special characters.

Warning: Do not use this function in conjunction with Qt Style Sheets. The font of an application can be customized using the "font" style sheet property. To set a bold font for all QPushButtons, set the application styleSheet() as "QPushButton { font: bold }"

See also font(), fontMetrics(), and QWidget::setFont().
*/
impl /*struct*/ QApplication {
  pub fn setFont_0<RetType, T: QApplication_setFont_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setFont_0();
    // return 1;
  }
}
pub trait QApplication_setFont_0<RetType> {
  fn setFont_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setFont_0<(/*void*/)> for (usize,usize) {
  fn setFont_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication7setFontERK5QFontPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:118
// index:0
// Public static Visibility=Default Availability=Available
// [8] QFontMetrics fontMetrics()

/*
Returns display (screen) font metrics for the application font.

See also font(), setFont(), QWidget::fontMetrics(), and QPainter::fontMetrics().
*/
impl /*struct*/ QApplication {
  pub fn fontMetrics_0<RetType, T: QApplication_fontMetrics_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fontMetrics_0();
    // return 1;
  }
}
pub trait QApplication_fontMetrics_0<RetType> {
  fn fontMetrics_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_fontMetrics_0<usize> for () {
  fn fontMetrics_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication11fontMetricsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:121
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setWindowIcon(const QIcon &)

/*

*/
impl /*struct*/ QApplication {
  pub fn setWindowIcon_0<RetType, T: QApplication_setWindowIcon_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setWindowIcon_0();
    // return 1;
  }
}
pub trait QApplication_setWindowIcon_0<RetType> {
  fn setWindowIcon_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setWindowIcon_0<(/*void*/)> for (usize) {
  fn setWindowIcon_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication13setWindowIconERK5QIcon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:122
// index:0
// Public static Visibility=Default Availability=Available
// [8] QIcon windowIcon()

/*

*/
impl /*struct*/ QApplication {
  pub fn windowIcon_0<RetType, T: QApplication_windowIcon_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.windowIcon_0();
    // return 1;
  }
}
pub trait QApplication_windowIcon_0<RetType> {
  fn windowIcon_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_windowIcon_0<usize> for () {
  fn windowIcon_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication10windowIconEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:125
// index:0
// Public static Visibility=Default Availability=Available
// [-2] QWidgetList allWidgets()

/*
Returns a list of all the widgets in the application.

The list is empty (QList::isEmpty()) if there are no widgets.

Note: Some of the widgets may be hidden.

Example:


  void updateAllWidgets()
  {
      foreach (QWidget *widget, QApplication::allWidgets())
          widget->update();
  }



See also topLevelWidgets() and QWidget::isVisible().
*/
impl /*struct*/ QApplication {
  pub fn allWidgets_0<RetType, T: QApplication_allWidgets_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.allWidgets_0();
    // return 1;
  }
}
pub trait QApplication_allWidgets_0<RetType> {
  fn allWidgets_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_allWidgets_0<usize> for () {
  fn allWidgets_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication10allWidgetsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:126
// index:0
// Public static Visibility=Default Availability=Available
// [-2] QWidgetList topLevelWidgets()

/*
Returns a list of the top-level widgets (windows) in the application.

Note: Some of the top-level widgets may be hidden, for example a tooltip if no tooltip is currently shown.

Example:


  void showAllHiddenTopLevelWidgets()
  {
      foreach (QWidget *widget, QApplication::topLevelWidgets()) {
          if (widget->isHidden())
              widget->show();
      }
  }



See also allWidgets(), QWidget::isWindow(), and QWidget::isHidden().
*/
impl /*struct*/ QApplication {
  pub fn topLevelWidgets_0<RetType, T: QApplication_topLevelWidgets_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.topLevelWidgets_0();
    // return 1;
  }
}
pub trait QApplication_topLevelWidgets_0<RetType> {
  fn topLevelWidgets_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_topLevelWidgets_0<usize> for () {
  fn topLevelWidgets_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication15topLevelWidgetsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:128
// index:0
// Public static Visibility=Default Availability=Available
// [8] QDesktopWidget * desktop()

/*
Returns the desktop widget (also called the root window).

The desktop may be composed of multiple screens, so it would be incorrect, for example, to attempt to center some widget in the desktop's geometry. QDesktopWidget has various functions for obtaining useful geometries upon the desktop, such as QDesktopWidget::screenGeometry() and QDesktopWidget::availableGeometry().

On X11, it is also possible to draw on the desktop.
*/
impl /*struct*/ QApplication {
  pub fn desktop_0<RetType, T: QApplication_desktop_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.desktop_0();
    // return 1;
  }
}
pub trait QApplication_desktop_0<RetType> {
  fn desktop_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_desktop_0<usize> for () {
  fn desktop_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication7desktopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:130
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWidget * activePopupWidget()

/*
Returns the active popup widget.

A popup widget is a special top-level widget that sets the Qt::WType_Popup widget flag, e.g. the QMenu widget. When the application opens a popup widget, all events are sent to the popup. Normal widgets and modal widgets cannot be accessed before the popup widget is closed.

Only other popup widgets may be opened when a popup widget is shown. The popup widgets are organized in a stack. This function returns the active popup widget at the top of the stack.

See also activeModalWidget() and topLevelWidgets().
*/
impl /*struct*/ QApplication {
  pub fn activePopupWidget_0<RetType, T: QApplication_activePopupWidget_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.activePopupWidget_0();
    // return 1;
  }
}
pub trait QApplication_activePopupWidget_0<RetType> {
  fn activePopupWidget_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_activePopupWidget_0<usize> for () {
  fn activePopupWidget_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication17activePopupWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:131
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWidget * activeModalWidget()

/*
Returns the active modal widget.

A modal widget is a special top-level widget which is a subclass of QDialog that specifies the modal parameter of the constructor as true. A modal widget must be closed before the user can continue with other parts of the program.

Modal widgets are organized in a stack. This function returns the active modal widget at the top of the stack.

See also activePopupWidget() and topLevelWidgets().
*/
impl /*struct*/ QApplication {
  pub fn activeModalWidget_0<RetType, T: QApplication_activeModalWidget_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.activeModalWidget_0();
    // return 1;
  }
}
pub trait QApplication_activeModalWidget_0<RetType> {
  fn activeModalWidget_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_activeModalWidget_0<usize> for () {
  fn activeModalWidget_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication17activeModalWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:132
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWidget * focusWidget()

/*
Returns the application widget that has the keyboard input focus, or 0 if no widget in this application has the focus.

See also QWidget::setFocus(), QWidget::hasFocus(), activeWindow(), and focusChanged().
*/
impl /*struct*/ QApplication {
  pub fn focusWidget_0<RetType, T: QApplication_focusWidget_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.focusWidget_0();
    // return 1;
  }
}
pub trait QApplication_focusWidget_0<RetType> {
  fn focusWidget_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_focusWidget_0<usize> for () {
  fn focusWidget_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication11focusWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:134
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWidget * activeWindow()

/*
Returns the application top-level window that has the keyboard input focus, or 0 if no application window has the focus. There might be an activeWindow() even if there is no focusWidget(), for example if no widget in that window accepts key events.

See also setActiveWindow(), QWidget::setFocus(), QWidget::hasFocus(), and focusWidget().
*/
impl /*struct*/ QApplication {
  pub fn activeWindow_0<RetType, T: QApplication_activeWindow_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.activeWindow_0();
    // return 1;
  }
}
pub trait QApplication_activeWindow_0<RetType> {
  fn activeWindow_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_activeWindow_0<usize> for () {
  fn activeWindow_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication12activeWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:135
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setActiveWindow(QWidget *)

/*
Sets the active window to the active widget in response to a system event. The function is called from the platform specific event handlers.

Warning: This function does not set the keyboard focus to the active widget. Call QWidget::activateWindow() instead.

It sets the activeWindow() and focusWidget() attributes and sends proper WindowActivate/WindowDeactivate and FocusIn/FocusOut events to all appropriate widgets. The window will then be painted in active state (e.g. cursors in line edits will blink), and it will have tool tips enabled.

See also activeWindow() and QWidget::activateWindow().
*/
impl /*struct*/ QApplication {
  pub fn setActiveWindow_0<RetType, T: QApplication_setActiveWindow_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setActiveWindow_0();
    // return 1;
  }
}
pub trait QApplication_setActiveWindow_0<RetType> {
  fn setActiveWindow_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setActiveWindow_0<(/*void*/)> for (usize) {
  fn setActiveWindow_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication15setActiveWindowEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:137
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWidget * widgetAt(const QPoint &)

/*
Returns the widget at global screen position point, or 0 if there is no Qt widget there.

This function can be slow.

See also QCursor::pos(), QWidget::grabMouse(), and QWidget::grabKeyboard().
*/
impl /*struct*/ QApplication {
  pub fn widgetAt_0<RetType, T: QApplication_widgetAt_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.widgetAt_0();
    // return 1;
  }
}
pub trait QApplication_widgetAt_0<RetType> {
  fn widgetAt_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_widgetAt_0<usize> for (usize) {
  fn widgetAt_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication8widgetAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:138
// index:1
// Public static inline Visibility=Default Availability=Available
// [8] QWidget * widgetAt(int, int)

/*
Returns the widget at global screen position point, or 0 if there is no Qt widget there.

This function can be slow.

See also QCursor::pos(), QWidget::grabMouse(), and QWidget::grabKeyboard().
*/
impl /*struct*/ QApplication {
  pub fn widgetAt_1<RetType, T: QApplication_widgetAt_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.widgetAt_1();
    // return 1;
  }
}
pub trait QApplication_widgetAt_1<RetType> {
  fn widgetAt_1(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_widgetAt_1<usize> for (i32,i32) {
  fn widgetAt_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication8widgetAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:139
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWidget * topLevelAt(const QPoint &)

/*
Returns the top-level widget at the given point; returns 0 if there is no such widget.
*/
impl /*struct*/ QApplication {
  pub fn topLevelAt_0<RetType, T: QApplication_topLevelAt_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.topLevelAt_0();
    // return 1;
  }
}
pub trait QApplication_topLevelAt_0<RetType> {
  fn topLevelAt_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_topLevelAt_0<usize> for (usize) {
  fn topLevelAt_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication10topLevelAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:140
// index:1
// Public static inline Visibility=Default Availability=Available
// [8] QWidget * topLevelAt(int, int)

/*
Returns the top-level widget at the given point; returns 0 if there is no such widget.
*/
impl /*struct*/ QApplication {
  pub fn topLevelAt_1<RetType, T: QApplication_topLevelAt_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.topLevelAt_1();
    // return 1;
  }
}
pub trait QApplication_topLevelAt_1<RetType> {
  fn topLevelAt_1(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_topLevelAt_1<usize> for (i32,i32) {
  fn topLevelAt_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication10topLevelAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:145
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void beep()

/*
Sounds the bell, using the default volume and sound. The function is not available in Qt for Embedded Linux.
*/
impl /*struct*/ QApplication {
  pub fn beep_0<RetType, T: QApplication_beep_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.beep_0();
    // return 1;
  }
}
pub trait QApplication_beep_0<RetType> {
  fn beep_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_beep_0<(/*void*/)> for () {
  fn beep_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QApplication4beepEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:146
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void alert(QWidget *, int)

/*
Causes an alert to be shown for widget if the window is not the active window. The alert is shown for msec miliseconds. If msec is zero (the default), then the alert is shown indefinitely until the window becomes active again.

Currently this function does nothing on Qt for Embedded Linux.

On macOS, this works more at the application level and will cause the application icon to bounce in the dock.

On Windows, this causes the window's taskbar entry to flash for a time. If msec is zero, the flashing will stop and the taskbar entry will turn a different color (currently orange).

On X11, this will cause the window to be marked as "demands attention", the window must not be hidden (i.e. not have hide() called on it, but be visible in some sort of way) in order for this to work.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QApplication {
  pub fn alert_0<RetType, T: QApplication_alert_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.alert_0();
    // return 1;
  }
}
pub trait QApplication_alert_0<RetType> {
  fn alert_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_alert_0<(/*void*/)> for (usize,i32) {
  fn alert_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication5alertEP7QWidgeti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:148
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setCursorFlashTime(int)

/*

*/
impl /*struct*/ QApplication {
  pub fn setCursorFlashTime_0<RetType, T: QApplication_setCursorFlashTime_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setCursorFlashTime_0();
    // return 1;
  }
}
pub trait QApplication_setCursorFlashTime_0<RetType> {
  fn setCursorFlashTime_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setCursorFlashTime_0<(/*void*/)> for (i32) {
  fn setCursorFlashTime_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication18setCursorFlashTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:149
// index:0
// Public static Visibility=Default Availability=Available
// [4] int cursorFlashTime()

/*

*/
impl /*struct*/ QApplication {
  pub fn cursorFlashTime_0<RetType, T: QApplication_cursorFlashTime_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.cursorFlashTime_0();
    // return 1;
  }
}
pub trait QApplication_cursorFlashTime_0<RetType> {
  fn cursorFlashTime_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_cursorFlashTime_0<i32> for () {
  fn cursorFlashTime_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication15cursorFlashTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:151
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setDoubleClickInterval(int)

/*

*/
impl /*struct*/ QApplication {
  pub fn setDoubleClickInterval_0<RetType, T: QApplication_setDoubleClickInterval_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDoubleClickInterval_0();
    // return 1;
  }
}
pub trait QApplication_setDoubleClickInterval_0<RetType> {
  fn setDoubleClickInterval_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setDoubleClickInterval_0<(/*void*/)> for (i32) {
  fn setDoubleClickInterval_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication22setDoubleClickIntervalEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:152
// index:0
// Public static Visibility=Default Availability=Available
// [4] int doubleClickInterval()

/*

*/
impl /*struct*/ QApplication {
  pub fn doubleClickInterval_0<RetType, T: QApplication_doubleClickInterval_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.doubleClickInterval_0();
    // return 1;
  }
}
pub trait QApplication_doubleClickInterval_0<RetType> {
  fn doubleClickInterval_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_doubleClickInterval_0<i32> for () {
  fn doubleClickInterval_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication19doubleClickIntervalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:154
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setKeyboardInputInterval(int)

/*

*/
impl /*struct*/ QApplication {
  pub fn setKeyboardInputInterval_0<RetType, T: QApplication_setKeyboardInputInterval_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setKeyboardInputInterval_0();
    // return 1;
  }
}
pub trait QApplication_setKeyboardInputInterval_0<RetType> {
  fn setKeyboardInputInterval_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setKeyboardInputInterval_0<(/*void*/)> for (i32) {
  fn setKeyboardInputInterval_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication24setKeyboardInputIntervalEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:155
// index:0
// Public static Visibility=Default Availability=Available
// [4] int keyboardInputInterval()

/*

*/
impl /*struct*/ QApplication {
  pub fn keyboardInputInterval_0<RetType, T: QApplication_keyboardInputInterval_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.keyboardInputInterval_0();
    // return 1;
  }
}
pub trait QApplication_keyboardInputInterval_0<RetType> {
  fn keyboardInputInterval_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_keyboardInputInterval_0<i32> for () {
  fn keyboardInputInterval_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication21keyboardInputIntervalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:158
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setWheelScrollLines(int)

/*

*/
impl /*struct*/ QApplication {
  pub fn setWheelScrollLines_0<RetType, T: QApplication_setWheelScrollLines_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setWheelScrollLines_0();
    // return 1;
  }
}
pub trait QApplication_setWheelScrollLines_0<RetType> {
  fn setWheelScrollLines_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setWheelScrollLines_0<(/*void*/)> for (i32) {
  fn setWheelScrollLines_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication19setWheelScrollLinesEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:159
// index:0
// Public static Visibility=Default Availability=Available
// [4] int wheelScrollLines()

/*

*/
impl /*struct*/ QApplication {
  pub fn wheelScrollLines_0<RetType, T: QApplication_wheelScrollLines_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.wheelScrollLines_0();
    // return 1;
  }
}
pub trait QApplication_wheelScrollLines_0<RetType> {
  fn wheelScrollLines_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_wheelScrollLines_0<i32> for () {
  fn wheelScrollLines_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication16wheelScrollLinesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:161
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setGlobalStrut(const QSize &)

/*

*/
impl /*struct*/ QApplication {
  pub fn setGlobalStrut_0<RetType, T: QApplication_setGlobalStrut_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setGlobalStrut_0();
    // return 1;
  }
}
pub trait QApplication_setGlobalStrut_0<RetType> {
  fn setGlobalStrut_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setGlobalStrut_0<(/*void*/)> for (usize) {
  fn setGlobalStrut_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication14setGlobalStrutERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:162
// index:0
// Public static Visibility=Default Availability=Available
// [8] QSize globalStrut()

/*

*/
impl /*struct*/ QApplication {
  pub fn globalStrut_0<RetType, T: QApplication_globalStrut_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.globalStrut_0();
    // return 1;
  }
}
pub trait QApplication_globalStrut_0<RetType> {
  fn globalStrut_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_globalStrut_0<usize> for () {
  fn globalStrut_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication11globalStrutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:164
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setStartDragTime(int)

/*

*/
impl /*struct*/ QApplication {
  pub fn setStartDragTime_0<RetType, T: QApplication_setStartDragTime_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setStartDragTime_0();
    // return 1;
  }
}
pub trait QApplication_setStartDragTime_0<RetType> {
  fn setStartDragTime_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setStartDragTime_0<(/*void*/)> for (i32) {
  fn setStartDragTime_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication16setStartDragTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:165
// index:0
// Public static Visibility=Default Availability=Available
// [4] int startDragTime()

/*

*/
impl /*struct*/ QApplication {
  pub fn startDragTime_0<RetType, T: QApplication_startDragTime_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.startDragTime_0();
    // return 1;
  }
}
pub trait QApplication_startDragTime_0<RetType> {
  fn startDragTime_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_startDragTime_0<i32> for () {
  fn startDragTime_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication13startDragTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:166
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setStartDragDistance(int)

/*

*/
impl /*struct*/ QApplication {
  pub fn setStartDragDistance_0<RetType, T: QApplication_setStartDragDistance_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setStartDragDistance_0();
    // return 1;
  }
}
pub trait QApplication_setStartDragDistance_0<RetType> {
  fn setStartDragDistance_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setStartDragDistance_0<(/*void*/)> for (i32) {
  fn setStartDragDistance_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication20setStartDragDistanceEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:167
// index:0
// Public static Visibility=Default Availability=Available
// [4] int startDragDistance()

/*

*/
impl /*struct*/ QApplication {
  pub fn startDragDistance_0<RetType, T: QApplication_startDragDistance_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.startDragDistance_0();
    // return 1;
  }
}
pub trait QApplication_startDragDistance_0<RetType> {
  fn startDragDistance_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_startDragDistance_0<i32> for () {
  fn startDragDistance_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication17startDragDistanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:169
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isEffectEnabled(Qt::UIEffect)

/*
Returns true if effect is enabled; otherwise returns false.

By default, Qt will try to use the desktop settings. To prevent this, call setDesktopSettingsAware(false).

Note: All effects are disabled on screens running at less than 16-bit color depth.

See also setEffectEnabled() and Qt::UIEffect.
*/
impl /*struct*/ QApplication {
  pub fn isEffectEnabled_0<RetType, T: QApplication_isEffectEnabled_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isEffectEnabled_0();
    // return 1;
  }
}
pub trait QApplication_isEffectEnabled_0<RetType> {
  fn isEffectEnabled_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_isEffectEnabled_0<bool> for (i32) {
  fn isEffectEnabled_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication15isEffectEnabledEN2Qt8UIEffectE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:170
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setEffectEnabled(Qt::UIEffect, bool)

/*
Enables the UI effect effect if enable is true, otherwise the effect will not be used.

Note: All effects are disabled on screens running at less than 16-bit color depth.

See also isEffectEnabled(), Qt::UIEffect, and setDesktopSettingsAware().
*/
impl /*struct*/ QApplication {
  pub fn setEffectEnabled_0<RetType, T: QApplication_setEffectEnabled_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setEffectEnabled_0();
    // return 1;
  }
}
pub trait QApplication_setEffectEnabled_0<RetType> {
  fn setEffectEnabled_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_setEffectEnabled_0<(/*void*/)> for (i32,bool) {
  fn setEffectEnabled_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication16setEffectEnabledEN2Qt8UIEffectEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:179
// index:0
// Public static Visibility=Default Availability=Available
// [4] int exec()

/*
Enters the main event loop and waits until exit() is called, then returns the value that was set to exit() (which is 0 if exit() is called via quit()).

It is necessary to call this function to start event handling. The main event loop receives events from the window system and dispatches these to the application widgets.

Generally, no user interaction can take place before calling exec(). As a special case, modal widgets like QMessageBox can be used before calling exec(), because modal widgets call exec() to start a local event loop.

To make your application perform idle processing, i.e., executing a special function whenever there are no pending events, use a QTimer with 0 timeout. More advanced idle processing schemes can be achieved using processEvents().

We recommend that you connect clean-up code to the aboutToQuit() signal, instead of putting it in your application's main() function. This is because, on some platforms the QApplication::exec() call may not return. For example, on the Windows platform, when the user logs off, the system terminates the process after Qt closes all top-level windows. Hence, there is no guarantee that the application will have time to exit its event loop and execute code at the end of the main() function, after the QApplication::exec() call.

See also quitOnLastWindowClosed, QCoreApplication::quit(), QCoreApplication::exit(), QCoreApplication::processEvents(), and QCoreApplication::exec().
*/
impl /*struct*/ QApplication {
  pub fn exec_0<RetType, T: QApplication_exec_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.exec_0();
    // return 1;
  }
}
pub trait QApplication_exec_0<RetType> {
  fn exec_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_exec_0<i32> for () {
  fn exec_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication4execEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:180
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool notify(QObject *, QEvent *)

/*

*/
impl /*struct*/ QApplication {
  pub fn notify_0<RetType, T: QApplication_notify_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notify_0(self);
    // return 1;
  }
}
pub trait QApplication_notify_0<RetType> {
  fn notify_0(self , rsthis: & QApplication) -> RetType;
}
impl<'a> /*trait*/ QApplication_notify_0<bool> for (usize,usize) {
  fn notify_0(self , rsthis: & QApplication) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication6notifyEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void focusChanged(QWidget *, QWidget *)

/*
This signal is emitted when the widget that has keyboard focus changed from old to now, i.e., because the user pressed the tab-key, clicked into a widget or changed the active window. Both old and now can be the null-pointer.

The signal is emitted after both widget have been notified about the change through QFocusEvent.

This function was introduced in  Qt 4.1.

See also QWidget::setFocus(), QWidget::clearFocus(), and Qt::FocusReason.
*/
impl /*struct*/ QApplication {
  pub fn focusChanged_0<RetType, T: QApplication_focusChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusChanged_0(self);
    // return 1;
  }
}
pub trait QApplication_focusChanged_0<RetType> {
  fn focusChanged_0(self , rsthis: & QApplication) -> RetType;
}
impl<'a> /*trait*/ QApplication_focusChanged_0<(/*void*/)> for (usize,usize) {
  fn focusChanged_0(self , rsthis: & QApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication12focusChangedEP7QWidgetS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:193
// index:0
// Public Visibility=Default Availability=Available
// [8] QString styleSheet() const

/*

*/
impl /*struct*/ QApplication {
  pub fn styleSheet_0<RetType, T: QApplication_styleSheet_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styleSheet_0(self);
    // return 1;
  }
}
pub trait QApplication_styleSheet_0<RetType> {
  fn styleSheet_0(self , rsthis: & QApplication) -> RetType;
}
impl<'a> /*trait*/ QApplication_styleSheet_0<usize> for () {
  fn styleSheet_0(self , rsthis: & QApplication) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QApplication10styleSheetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:196
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStyleSheet(const QString &)

/*

*/
impl /*struct*/ QApplication {
  pub fn setStyleSheet_0<RetType, T: QApplication_setStyleSheet_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStyleSheet_0(self);
    // return 1;
  }
}
pub trait QApplication_setStyleSheet_0<RetType> {
  fn setStyleSheet_0(self , rsthis: & QApplication) -> RetType;
}
impl<'a> /*trait*/ QApplication_setStyleSheet_0<(/*void*/)> for (usize) {
  fn setStyleSheet_0(self , rsthis: & QApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication13setStyleSheetERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:198
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoSipEnabled(const bool)

/*

*/
impl /*struct*/ QApplication {
  pub fn setAutoSipEnabled_0<RetType, T: QApplication_setAutoSipEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoSipEnabled_0(self);
    // return 1;
  }
}
pub trait QApplication_setAutoSipEnabled_0<RetType> {
  fn setAutoSipEnabled_0(self , rsthis: & QApplication) -> RetType;
}
impl<'a> /*trait*/ QApplication_setAutoSipEnabled_0<(/*void*/)> for (bool) {
  fn setAutoSipEnabled_0(self , rsthis: & QApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QApplication17setAutoSipEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:199
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoSipEnabled() const

/*

*/
impl /*struct*/ QApplication {
  pub fn autoSipEnabled_0<RetType, T: QApplication_autoSipEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoSipEnabled_0(self);
    // return 1;
  }
}
pub trait QApplication_autoSipEnabled_0<RetType> {
  fn autoSipEnabled_0(self , rsthis: & QApplication) -> RetType;
}
impl<'a> /*trait*/ QApplication_autoSipEnabled_0<bool> for () {
  fn autoSipEnabled_0(self , rsthis: & QApplication) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QApplication14autoSipEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:200
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void closeAllWindows()

/*
Closes all top-level windows.

This function is particularly useful for applications with many top-level windows. It could, for example, be connected to a Exit entry in the File menu:


      const QIcon exitIcon = QIcon::fromTheme("application-exit");
      QAction *exitAct = fileMenu->addAction(exitIcon, tr("E&xit"), qApp, &QApplication::closeAllWindows);
      exitAct->setShortcuts(QKeySequence::Quit);
      exitAct->setStatusTip(tr("Exit the application"));
      fileMenu->addAction(exitAct);



The windows are closed in random order, until one window does not accept the close event. The application quits when the last window was successfully closed; this can be turned off by setting quitOnLastWindowClosed to false.

See also quitOnLastWindowClosed, lastWindowClosed(), QWidget::close(), QWidget::closeEvent(), lastWindowClosed(), QCoreApplication::quit(), topLevelWidgets(), and QWidget::isWindow().
*/
impl /*struct*/ QApplication {
  pub fn closeAllWindows_0<RetType, T: QApplication_closeAllWindows_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.closeAllWindows_0();
    // return 1;
  }
}
pub trait QApplication_closeAllWindows_0<RetType> {
  fn closeAllWindows_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_closeAllWindows_0<(/*void*/)> for () {
  fn closeAllWindows_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QApplication15closeAllWindowsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:201
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void aboutQt()

/*
Displays a simple message box about Qt. The message includes the version number of Qt being used by the application.

This is useful for inclusion in the Help menu of an application, as shown in the Menus example.

This function is a convenience slot for QMessageBox::aboutQt().
*/
impl /*struct*/ QApplication {
  pub fn aboutQt_0<RetType, T: QApplication_aboutQt_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.aboutQt_0();
    // return 1;
  }
}
pub trait QApplication_aboutQt_0<RetType> {
  fn aboutQt_0(self ) -> RetType;
}
impl<'a> /*trait*/ QApplication_aboutQt_0<(/*void*/)> for () {
  fn aboutQt_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QApplication7aboutQtEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qapplication.h:204
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*

*/
impl /*struct*/ QApplication {
  pub fn event_0<RetType, T: QApplication_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QApplication_event_0<RetType> {
  fn event_0(self , rsthis: & QApplication) -> RetType;
}
impl<'a> /*trait*/ QApplication_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QApplication) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QApplication5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


*/
pub type QApplication__ColorSpec = i32;
// 
pub const QApplication__NormalColor :QApplication__ColorSpec = 0;
// 
pub const QApplication__CustomColor :QApplication__ColorSpec = 1;
// 
pub const QApplication__ManyColor :QApplication__ColorSpec = 2;
pub fn QApplication_ColorSpecItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QApplication", val);
}
pub fn QApplication_ColorSpecItemName_s(val: i32) ->String {
  //var nilthis *QApplication
  //return nilthis.ColorSpecItemName(val);
  return QApplication_ColorSpecItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
