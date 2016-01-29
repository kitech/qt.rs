// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtWidgets/qapplication.h
// dst-file: /src/widgets/qapplication.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::super::gui::qguiapplication::*; // 771
use std::ops::Deref;
use super::super::core::qstring::*; // 771
use super::super::gui::qpalette::*; // 771
use super::qwidget::*; // 773
use super::super::gui::qfontmetrics::*; // 771
use super::super::gui::qfont::*; // 771
use super::qstyle::*; // 773
use super::super::core::qpoint::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::qdesktopwidget::*; // 773
use super::super::core::qsize::*; // 771
// use super::qlist::*; // 775
use super::super::gui::qicon::*; // 771
use super::super::core::qobject::*; // 771
use super::super::core::qcoreevent::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QApplication_Class_Size() -> c_int;
  // proto:  QString QApplication::styleSheet();
  fn C_ZNK12QApplication10styleSheetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QPalette QApplication::palette(const char * className);
  fn C_ZN12QApplication7paletteEPKc(arg0: *mut c_char) -> *mut c_void;
  // proto: static QWidget * QApplication::activeWindow();
  fn C_ZN12QApplication12activeWindowEv() -> *mut c_void;
  // proto: static void QApplication::setKeyboardInputInterval(int );
  fn C_ZN12QApplication24setKeyboardInputIntervalEi(arg0: c_int);
  // proto: static QWidget * QApplication::focusWidget();
  fn C_ZN12QApplication11focusWidgetEv() -> *mut c_void;
  // proto: static QFontMetrics QApplication::fontMetrics();
  fn C_ZN12QApplication11fontMetricsEv() -> *mut c_void;
  // proto: static QFont QApplication::font(const char * className);
  fn C_ZN12QApplication4fontEPKc(arg0: *mut c_char) -> *mut c_void;
  // proto: static QStyle * QApplication::style();
  fn C_ZN12QApplication5styleEv() -> *mut c_void;
  // proto: static QWidget * QApplication::widgetAt(const QPoint & p);
  fn C_ZN12QApplication8widgetAtERK6QPoint(arg0: *mut c_void) -> *mut c_void;
  // proto: static void QApplication::setActiveWindow(QWidget * act);
  fn C_ZN12QApplication15setActiveWindowEP7QWidget(arg0: *mut c_void);
  // proto: static QFont QApplication::font();
  fn C_ZN12QApplication4fontEv() -> *mut c_void;
  // proto: static void QApplication::setWheelScrollLines(int );
  fn C_ZN12QApplication19setWheelScrollLinesEi(arg0: c_int);
  // proto:  void QApplication::setStyleSheet(const QString & sheet);
  fn C_ZN12QApplication13setStyleSheetERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QApplication::setAutoSipEnabled(const bool enabled);
  fn C_ZN12QApplication17setAutoSipEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  const QMetaObject * QApplication::metaObject();
  fn C_ZNK12QApplication10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static int QApplication::keyboardInputInterval();
  fn C_ZN12QApplication21keyboardInputIntervalEv() -> c_int;
  // proto: static int QApplication::cursorFlashTime();
  fn C_ZN12QApplication15cursorFlashTimeEv() -> c_int;
  // proto: static int QApplication::startDragDistance();
  fn C_ZN12QApplication17startDragDistanceEv() -> c_int;
  // proto: static QDesktopWidget * QApplication::desktop();
  fn C_ZN12QApplication7desktopEv() -> *mut c_void;
  // proto: static void QApplication::setStartDragDistance(int l);
  fn C_ZN12QApplication20setStartDragDistanceEi(arg0: c_int);
  // proto: static QFont QApplication::font(const QWidget * );
  fn C_ZN12QApplication4fontEPK7QWidget(arg0: *mut c_void) -> *mut c_void;
  // proto: static int QApplication::colorSpec();
  fn C_ZN12QApplication9colorSpecEv() -> c_int;
  // proto: static void QApplication::setFont(const QFont & , const char * className);
  fn C_ZN12QApplication7setFontERK5QFontPKc(arg0: *mut c_void, arg1: *mut c_char);
  // proto: static void QApplication::closeAllWindows();
  fn C_ZN12QApplication15closeAllWindowsEv();
  // proto: static void QApplication::setCursorFlashTime(int );
  fn C_ZN12QApplication18setCursorFlashTimeEi(arg0: c_int);
  // proto: static QWidget * QApplication::widgetAt(int x, int y);
  fn C_ZN12QApplication8widgetAtEii(arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto: static void QApplication::alert(QWidget * widget, int duration);
  fn C_ZN12QApplication5alertEP7QWidgeti(arg0: *mut c_void, arg1: c_int);
  // proto: static QPalette QApplication::palette(const QWidget * );
  fn C_ZN12QApplication7paletteEPK7QWidget(arg0: *mut c_void) -> *mut c_void;
  // proto: static int QApplication::wheelScrollLines();
  fn C_ZN12QApplication16wheelScrollLinesEv() -> c_int;
  // proto: static void QApplication::aboutQt();
  fn C_ZN12QApplication7aboutQtEv();
  // proto: static QWidget * QApplication::activeModalWidget();
  fn C_ZN12QApplication17activeModalWidgetEv() -> *mut c_void;
  // proto: static QWidget * QApplication::activePopupWidget();
  fn C_ZN12QApplication17activePopupWidgetEv() -> *mut c_void;
  // proto:  void QApplication::QApplication(int & argc, char ** argv, int );
  fn C_ZN12QApplicationC2ERiPPci(arg0: *mut c_int, arg1: *mut c_char, arg2: c_int) -> u64;
  // proto: static void QApplication::setStartDragTime(int ms);
  fn C_ZN12QApplication16setStartDragTimeEi(arg0: c_int);
  // proto: static QWidget * QApplication::topLevelAt(int x, int y);
  fn C_ZN12QApplication10topLevelAtEii(arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto: static void QApplication::setStyle(QStyle * );
  fn C_ZN12QApplication8setStyleEP6QStyle(arg0: *mut c_void);
  // proto:  void QApplication::~QApplication();
  fn C_ZN12QApplicationD2Ev(qthis: u64 /* *mut c_void*/);
  // proto: static void QApplication::setDoubleClickInterval(int );
  fn C_ZN12QApplication22setDoubleClickIntervalEi(arg0: c_int);
  // proto: static void QApplication::setGlobalStrut(const QSize & );
  fn C_ZN12QApplication14setGlobalStrutERK5QSize(arg0: *mut c_void);
  // proto: static void QApplication::setColorSpec(int );
  fn C_ZN12QApplication12setColorSpecEi(arg0: c_int);
  // proto: static QWidgetList QApplication::allWidgets();
  fn C_ZN12QApplication10allWidgetsEv() -> *mut c_void;
  // proto: static QSize QApplication::globalStrut();
  fn C_ZN12QApplication11globalStrutEv() -> *mut c_void;
  // proto: static void QApplication::setPalette(const QPalette & , const char * className);
  fn C_ZN12QApplication10setPaletteERK8QPalettePKc(arg0: *mut c_void, arg1: *mut c_char);
  // proto: static QStyle * QApplication::setStyle(const QString & );
  fn C_ZN12QApplication8setStyleERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static QWidgetList QApplication::topLevelWidgets();
  fn C_ZN12QApplication15topLevelWidgetsEv() -> *mut c_void;
  // proto: static int QApplication::exec();
  fn C_ZN12QApplication4execEv() -> c_int;
  // proto: static void QApplication::setWindowIcon(const QIcon & icon);
  fn C_ZN12QApplication13setWindowIconERK5QIcon(arg0: *mut c_void);
  // proto: static void QApplication::beep();
  fn C_ZN12QApplication4beepEv();
  // proto:  bool QApplication::notify(QObject * , QEvent * );
  fn C_ZN12QApplication6notifyEP7QObjectP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  bool QApplication::autoSipEnabled();
  fn C_ZNK12QApplication14autoSipEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QWidget * QApplication::topLevelAt(const QPoint & p);
  fn C_ZN12QApplication10topLevelAtERK6QPoint(arg0: *mut c_void) -> *mut c_void;
  // proto: static int QApplication::startDragTime();
  fn C_ZN12QApplication13startDragTimeEv() -> c_int;
  // proto: static int QApplication::doubleClickInterval();
  fn C_ZN12QApplication19doubleClickIntervalEv() -> c_int;
  // proto: static QIcon QApplication::windowIcon();
  fn C_ZN12QApplication10windowIconEv() -> *mut c_void;
  fn QApplication_SlotProxy_connect__ZN12QApplication12focusChangedEP7QWidgetS1_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QApplication)=1
#[derive(Default)]
pub struct QApplication {
  qbase: QGuiApplication,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _focusChanged: QApplication_focusChanged_signal,
}

impl /*struct*/ QApplication {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QApplication {
    return QApplication{qbase: QGuiApplication::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QApplication {
  type Target = QGuiApplication;

  fn deref(&self) -> &QGuiApplication {
    return & self.qbase;
  }
}
impl AsRef<QGuiApplication> for QApplication {
  fn as_ref(& self) -> & QGuiApplication {
    return & self.qbase;
  }
}
  // proto:  QString QApplication::styleSheet();
impl /*struct*/ QApplication {
  pub fn styleSheet<RetType, T: QApplication_styleSheet<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.styleSheet(self);
    // return 1;
  }
}

pub trait QApplication_styleSheet<RetType> {
  fn styleSheet(self , rsthis: & QApplication) -> RetType;
}

  // proto:  QString QApplication::styleSheet();
impl<'a> /*trait*/ QApplication_styleSheet<QString> for () {
  fn styleSheet(self , rsthis: & QApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QApplication10styleSheetEv()};
    let mut ret = unsafe {C_ZNK12QApplication10styleSheetEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QPalette QApplication::palette(const char * className);
impl /*struct*/ QApplication {
  pub fn palette_s<RetType, T: QApplication_palette_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.palette_s();
    // return 1;
  }
}

pub trait QApplication_palette_s<RetType> {
  fn palette_s(self ) -> RetType;
}

  // proto: static QPalette QApplication::palette(const char * className);
impl<'a> /*trait*/ QApplication_palette_s<QPalette> for (&'a  String) {
  fn palette_s(self ) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7paletteEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZN12QApplication7paletteEPKc(arg0)};
    let mut ret1 = QPalette::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QWidget * QApplication::activeWindow();
impl /*struct*/ QApplication {
  pub fn activeWindow_s<RetType, T: QApplication_activeWindow_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.activeWindow_s();
    // return 1;
  }
}

pub trait QApplication_activeWindow_s<RetType> {
  fn activeWindow_s(self ) -> RetType;
}

  // proto: static QWidget * QApplication::activeWindow();
impl<'a> /*trait*/ QApplication_activeWindow_s<QWidget> for () {
  fn activeWindow_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication12activeWindowEv()};
    let mut ret = unsafe {C_ZN12QApplication12activeWindowEv()};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QApplication::setKeyboardInputInterval(int );
impl /*struct*/ QApplication {
  pub fn setKeyboardInputInterval_s<RetType, T: QApplication_setKeyboardInputInterval_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setKeyboardInputInterval_s();
    // return 1;
  }
}

pub trait QApplication_setKeyboardInputInterval_s<RetType> {
  fn setKeyboardInputInterval_s(self ) -> RetType;
}

  // proto: static void QApplication::setKeyboardInputInterval(int );
impl<'a> /*trait*/ QApplication_setKeyboardInputInterval_s<()> for (i32) {
  fn setKeyboardInputInterval_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication24setKeyboardInputIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QApplication24setKeyboardInputIntervalEi(arg0)};
    // return 1;
  }
}

  // proto: static QWidget * QApplication::focusWidget();
impl /*struct*/ QApplication {
  pub fn focusWidget_s<RetType, T: QApplication_focusWidget_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.focusWidget_s();
    // return 1;
  }
}

pub trait QApplication_focusWidget_s<RetType> {
  fn focusWidget_s(self ) -> RetType;
}

  // proto: static QWidget * QApplication::focusWidget();
impl<'a> /*trait*/ QApplication_focusWidget_s<QWidget> for () {
  fn focusWidget_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication11focusWidgetEv()};
    let mut ret = unsafe {C_ZN12QApplication11focusWidgetEv()};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QFontMetrics QApplication::fontMetrics();
impl /*struct*/ QApplication {
  pub fn fontMetrics_s<RetType, T: QApplication_fontMetrics_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fontMetrics_s();
    // return 1;
  }
}

pub trait QApplication_fontMetrics_s<RetType> {
  fn fontMetrics_s(self ) -> RetType;
}

  // proto: static QFontMetrics QApplication::fontMetrics();
impl<'a> /*trait*/ QApplication_fontMetrics_s<QFontMetrics> for () {
  fn fontMetrics_s(self ) -> QFontMetrics {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication11fontMetricsEv()};
    let mut ret = unsafe {C_ZN12QApplication11fontMetricsEv()};
    let mut ret1 = QFontMetrics::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QFont QApplication::font(const char * className);
impl /*struct*/ QApplication {
  pub fn font_s<RetType, T: QApplication_font_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.font_s();
    // return 1;
  }
}

pub trait QApplication_font_s<RetType> {
  fn font_s(self ) -> RetType;
}

  // proto: static QFont QApplication::font(const char * className);
impl<'a> /*trait*/ QApplication_font_s<QFont> for (&'a  String) {
  fn font_s(self ) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4fontEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZN12QApplication4fontEPKc(arg0)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QStyle * QApplication::style();
impl /*struct*/ QApplication {
  pub fn style_s<RetType, T: QApplication_style_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.style_s();
    // return 1;
  }
}

pub trait QApplication_style_s<RetType> {
  fn style_s(self ) -> RetType;
}

  // proto: static QStyle * QApplication::style();
impl<'a> /*trait*/ QApplication_style_s<QStyle> for () {
  fn style_s(self ) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication5styleEv()};
    let mut ret = unsafe {C_ZN12QApplication5styleEv()};
    let mut ret1 = QStyle::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QWidget * QApplication::widgetAt(const QPoint & p);
impl /*struct*/ QApplication {
  pub fn widgetAt_s<RetType, T: QApplication_widgetAt_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.widgetAt_s();
    // return 1;
  }
}

pub trait QApplication_widgetAt_s<RetType> {
  fn widgetAt_s(self ) -> RetType;
}

  // proto: static QWidget * QApplication::widgetAt(const QPoint & p);
impl<'a> /*trait*/ QApplication_widgetAt_s<QWidget> for (&'a QPoint) {
  fn widgetAt_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8widgetAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QApplication8widgetAtERK6QPoint(arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QApplication::setActiveWindow(QWidget * act);
impl /*struct*/ QApplication {
  pub fn setActiveWindow_s<RetType, T: QApplication_setActiveWindow_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setActiveWindow_s();
    // return 1;
  }
}

pub trait QApplication_setActiveWindow_s<RetType> {
  fn setActiveWindow_s(self ) -> RetType;
}

  // proto: static void QApplication::setActiveWindow(QWidget * act);
impl<'a> /*trait*/ QApplication_setActiveWindow_s<()> for (&'a QWidget) {
  fn setActiveWindow_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15setActiveWindowEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QApplication15setActiveWindowEP7QWidget(arg0)};
    // return 1;
  }
}

  // proto: static QFont QApplication::font();
impl<'a> /*trait*/ QApplication_font_s<QFont> for () {
  fn font_s(self ) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4fontEv()};
    let mut ret = unsafe {C_ZN12QApplication4fontEv()};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QApplication::setWheelScrollLines(int );
impl /*struct*/ QApplication {
  pub fn setWheelScrollLines_s<RetType, T: QApplication_setWheelScrollLines_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setWheelScrollLines_s();
    // return 1;
  }
}

pub trait QApplication_setWheelScrollLines_s<RetType> {
  fn setWheelScrollLines_s(self ) -> RetType;
}

  // proto: static void QApplication::setWheelScrollLines(int );
impl<'a> /*trait*/ QApplication_setWheelScrollLines_s<()> for (i32) {
  fn setWheelScrollLines_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication19setWheelScrollLinesEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QApplication19setWheelScrollLinesEi(arg0)};
    // return 1;
  }
}

  // proto:  void QApplication::setStyleSheet(const QString & sheet);
impl /*struct*/ QApplication {
  pub fn setStyleSheet<RetType, T: QApplication_setStyleSheet<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStyleSheet(self);
    // return 1;
  }
}

pub trait QApplication_setStyleSheet<RetType> {
  fn setStyleSheet(self , rsthis: & QApplication) -> RetType;
}

  // proto:  void QApplication::setStyleSheet(const QString & sheet);
impl<'a> /*trait*/ QApplication_setStyleSheet<()> for (&'a QString) {
  fn setStyleSheet(self , rsthis: & QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication13setStyleSheetERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QApplication13setStyleSheetERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QApplication::setAutoSipEnabled(const bool enabled);
impl /*struct*/ QApplication {
  pub fn setAutoSipEnabled<RetType, T: QApplication_setAutoSipEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoSipEnabled(self);
    // return 1;
  }
}

pub trait QApplication_setAutoSipEnabled<RetType> {
  fn setAutoSipEnabled(self , rsthis: & QApplication) -> RetType;
}

  // proto:  void QApplication::setAutoSipEnabled(const bool enabled);
impl<'a> /*trait*/ QApplication_setAutoSipEnabled<()> for (i8) {
  fn setAutoSipEnabled(self , rsthis: & QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17setAutoSipEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QApplication17setAutoSipEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QApplication::metaObject();
impl /*struct*/ QApplication {
  pub fn metaObject<RetType, T: QApplication_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QApplication_metaObject<RetType> {
  fn metaObject(self , rsthis: & QApplication) -> RetType;
}

  // proto:  const QMetaObject * QApplication::metaObject();
impl<'a> /*trait*/ QApplication_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QApplication) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QApplication10metaObjectEv()};
    let mut ret = unsafe {C_ZNK12QApplication10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static int QApplication::keyboardInputInterval();
impl /*struct*/ QApplication {
  pub fn keyboardInputInterval_s<RetType, T: QApplication_keyboardInputInterval_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.keyboardInputInterval_s();
    // return 1;
  }
}

pub trait QApplication_keyboardInputInterval_s<RetType> {
  fn keyboardInputInterval_s(self ) -> RetType;
}

  // proto: static int QApplication::keyboardInputInterval();
impl<'a> /*trait*/ QApplication_keyboardInputInterval_s<i32> for () {
  fn keyboardInputInterval_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication21keyboardInputIntervalEv()};
    let mut ret = unsafe {C_ZN12QApplication21keyboardInputIntervalEv()};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static int QApplication::cursorFlashTime();
impl /*struct*/ QApplication {
  pub fn cursorFlashTime_s<RetType, T: QApplication_cursorFlashTime_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.cursorFlashTime_s();
    // return 1;
  }
}

pub trait QApplication_cursorFlashTime_s<RetType> {
  fn cursorFlashTime_s(self ) -> RetType;
}

  // proto: static int QApplication::cursorFlashTime();
impl<'a> /*trait*/ QApplication_cursorFlashTime_s<i32> for () {
  fn cursorFlashTime_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15cursorFlashTimeEv()};
    let mut ret = unsafe {C_ZN12QApplication15cursorFlashTimeEv()};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static int QApplication::startDragDistance();
impl /*struct*/ QApplication {
  pub fn startDragDistance_s<RetType, T: QApplication_startDragDistance_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.startDragDistance_s();
    // return 1;
  }
}

pub trait QApplication_startDragDistance_s<RetType> {
  fn startDragDistance_s(self ) -> RetType;
}

  // proto: static int QApplication::startDragDistance();
impl<'a> /*trait*/ QApplication_startDragDistance_s<i32> for () {
  fn startDragDistance_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17startDragDistanceEv()};
    let mut ret = unsafe {C_ZN12QApplication17startDragDistanceEv()};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static QDesktopWidget * QApplication::desktop();
impl /*struct*/ QApplication {
  pub fn desktop_s<RetType, T: QApplication_desktop_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.desktop_s();
    // return 1;
  }
}

pub trait QApplication_desktop_s<RetType> {
  fn desktop_s(self ) -> RetType;
}

  // proto: static QDesktopWidget * QApplication::desktop();
impl<'a> /*trait*/ QApplication_desktop_s<QDesktopWidget> for () {
  fn desktop_s(self ) -> QDesktopWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7desktopEv()};
    let mut ret = unsafe {C_ZN12QApplication7desktopEv()};
    let mut ret1 = QDesktopWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QApplication::setStartDragDistance(int l);
impl /*struct*/ QApplication {
  pub fn setStartDragDistance_s<RetType, T: QApplication_setStartDragDistance_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setStartDragDistance_s();
    // return 1;
  }
}

pub trait QApplication_setStartDragDistance_s<RetType> {
  fn setStartDragDistance_s(self ) -> RetType;
}

  // proto: static void QApplication::setStartDragDistance(int l);
impl<'a> /*trait*/ QApplication_setStartDragDistance_s<()> for (i32) {
  fn setStartDragDistance_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication20setStartDragDistanceEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QApplication20setStartDragDistanceEi(arg0)};
    // return 1;
  }
}

  // proto: static QFont QApplication::font(const QWidget * );
impl<'a> /*trait*/ QApplication_font_s<QFont> for (&'a QWidget) {
  fn font_s(self ) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4fontEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QApplication4fontEPK7QWidget(arg0)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static int QApplication::colorSpec();
impl /*struct*/ QApplication {
  pub fn colorSpec_s<RetType, T: QApplication_colorSpec_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.colorSpec_s();
    // return 1;
  }
}

pub trait QApplication_colorSpec_s<RetType> {
  fn colorSpec_s(self ) -> RetType;
}

  // proto: static int QApplication::colorSpec();
impl<'a> /*trait*/ QApplication_colorSpec_s<i32> for () {
  fn colorSpec_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication9colorSpecEv()};
    let mut ret = unsafe {C_ZN12QApplication9colorSpecEv()};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static void QApplication::setFont(const QFont & , const char * className);
impl /*struct*/ QApplication {
  pub fn setFont_s<RetType, T: QApplication_setFont_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setFont_s();
    // return 1;
  }
}

pub trait QApplication_setFont_s<RetType> {
  fn setFont_s(self ) -> RetType;
}

  // proto: static void QApplication::setFont(const QFont & , const char * className);
impl<'a> /*trait*/ QApplication_setFont_s<()> for (&'a QFont, &'a  String) {
  fn setFont_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7setFontERK5QFontPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {C_ZN12QApplication7setFontERK5QFontPKc(arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QApplication::closeAllWindows();
impl /*struct*/ QApplication {
  pub fn closeAllWindows_s<RetType, T: QApplication_closeAllWindows_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.closeAllWindows_s();
    // return 1;
  }
}

pub trait QApplication_closeAllWindows_s<RetType> {
  fn closeAllWindows_s(self ) -> RetType;
}

  // proto: static void QApplication::closeAllWindows();
impl<'a> /*trait*/ QApplication_closeAllWindows_s<()> for () {
  fn closeAllWindows_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15closeAllWindowsEv()};
     unsafe {C_ZN12QApplication15closeAllWindowsEv()};
    // return 1;
  }
}

  // proto: static void QApplication::setCursorFlashTime(int );
impl /*struct*/ QApplication {
  pub fn setCursorFlashTime_s<RetType, T: QApplication_setCursorFlashTime_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setCursorFlashTime_s();
    // return 1;
  }
}

pub trait QApplication_setCursorFlashTime_s<RetType> {
  fn setCursorFlashTime_s(self ) -> RetType;
}

  // proto: static void QApplication::setCursorFlashTime(int );
impl<'a> /*trait*/ QApplication_setCursorFlashTime_s<()> for (i32) {
  fn setCursorFlashTime_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication18setCursorFlashTimeEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QApplication18setCursorFlashTimeEi(arg0)};
    // return 1;
  }
}

  // proto: static QWidget * QApplication::widgetAt(int x, int y);
impl<'a> /*trait*/ QApplication_widgetAt_s<QWidget> for (i32, i32) {
  fn widgetAt_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8widgetAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN12QApplication8widgetAtEii(arg0, arg1)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QApplication::alert(QWidget * widget, int duration);
impl /*struct*/ QApplication {
  pub fn alert_s<RetType, T: QApplication_alert_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.alert_s();
    // return 1;
  }
}

pub trait QApplication_alert_s<RetType> {
  fn alert_s(self ) -> RetType;
}

  // proto: static void QApplication::alert(QWidget * widget, int duration);
impl<'a> /*trait*/ QApplication_alert_s<()> for (&'a QWidget, i32) {
  fn alert_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication5alertEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN12QApplication5alertEP7QWidgeti(arg0, arg1)};
    // return 1;
  }
}

  // proto: static QPalette QApplication::palette(const QWidget * );
impl<'a> /*trait*/ QApplication_palette_s<QPalette> for (&'a QWidget) {
  fn palette_s(self ) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7paletteEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QApplication7paletteEPK7QWidget(arg0)};
    let mut ret1 = QPalette::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static int QApplication::wheelScrollLines();
impl /*struct*/ QApplication {
  pub fn wheelScrollLines_s<RetType, T: QApplication_wheelScrollLines_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.wheelScrollLines_s();
    // return 1;
  }
}

pub trait QApplication_wheelScrollLines_s<RetType> {
  fn wheelScrollLines_s(self ) -> RetType;
}

  // proto: static int QApplication::wheelScrollLines();
impl<'a> /*trait*/ QApplication_wheelScrollLines_s<i32> for () {
  fn wheelScrollLines_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication16wheelScrollLinesEv()};
    let mut ret = unsafe {C_ZN12QApplication16wheelScrollLinesEv()};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static void QApplication::aboutQt();
impl /*struct*/ QApplication {
  pub fn aboutQt_s<RetType, T: QApplication_aboutQt_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.aboutQt_s();
    // return 1;
  }
}

pub trait QApplication_aboutQt_s<RetType> {
  fn aboutQt_s(self ) -> RetType;
}

  // proto: static void QApplication::aboutQt();
impl<'a> /*trait*/ QApplication_aboutQt_s<()> for () {
  fn aboutQt_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7aboutQtEv()};
     unsafe {C_ZN12QApplication7aboutQtEv()};
    // return 1;
  }
}

  // proto: static QWidget * QApplication::activeModalWidget();
impl /*struct*/ QApplication {
  pub fn activeModalWidget_s<RetType, T: QApplication_activeModalWidget_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.activeModalWidget_s();
    // return 1;
  }
}

pub trait QApplication_activeModalWidget_s<RetType> {
  fn activeModalWidget_s(self ) -> RetType;
}

  // proto: static QWidget * QApplication::activeModalWidget();
impl<'a> /*trait*/ QApplication_activeModalWidget_s<QWidget> for () {
  fn activeModalWidget_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17activeModalWidgetEv()};
    let mut ret = unsafe {C_ZN12QApplication17activeModalWidgetEv()};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QWidget * QApplication::activePopupWidget();
impl /*struct*/ QApplication {
  pub fn activePopupWidget_s<RetType, T: QApplication_activePopupWidget_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.activePopupWidget_s();
    // return 1;
  }
}

pub trait QApplication_activePopupWidget_s<RetType> {
  fn activePopupWidget_s(self ) -> RetType;
}

  // proto: static QWidget * QApplication::activePopupWidget();
impl<'a> /*trait*/ QApplication_activePopupWidget_s<QWidget> for () {
  fn activePopupWidget_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17activePopupWidgetEv()};
    let mut ret = unsafe {C_ZN12QApplication17activePopupWidgetEv()};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QApplication::QApplication(int & argc, char ** argv, int );
impl /*struct*/ QApplication {
  pub fn new<T: QApplication_new>(value: T) -> QApplication {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QApplication_new {
  fn new(self) -> QApplication;
}

  // proto:  void QApplication::QApplication(int & argc, char ** argv, int );
impl<'a> /*trait*/ QApplication_new for (&'a mut i32, &'a mut String, i32) {
  fn new(self) -> QApplication {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplicationC2ERiPPci()};
    let ctysz: c_int = unsafe{QApplication_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
    let qthis: u64 = unsafe {C_ZN12QApplicationC2ERiPPci(arg0, arg1, arg2)};
    let rsthis = QApplication{qbase: QGuiApplication::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static void QApplication::setStartDragTime(int ms);
impl /*struct*/ QApplication {
  pub fn setStartDragTime_s<RetType, T: QApplication_setStartDragTime_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setStartDragTime_s();
    // return 1;
  }
}

pub trait QApplication_setStartDragTime_s<RetType> {
  fn setStartDragTime_s(self ) -> RetType;
}

  // proto: static void QApplication::setStartDragTime(int ms);
impl<'a> /*trait*/ QApplication_setStartDragTime_s<()> for (i32) {
  fn setStartDragTime_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication16setStartDragTimeEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QApplication16setStartDragTimeEi(arg0)};
    // return 1;
  }
}

  // proto: static QWidget * QApplication::topLevelAt(int x, int y);
impl /*struct*/ QApplication {
  pub fn topLevelAt_s<RetType, T: QApplication_topLevelAt_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.topLevelAt_s();
    // return 1;
  }
}

pub trait QApplication_topLevelAt_s<RetType> {
  fn topLevelAt_s(self ) -> RetType;
}

  // proto: static QWidget * QApplication::topLevelAt(int x, int y);
impl<'a> /*trait*/ QApplication_topLevelAt_s<QWidget> for (i32, i32) {
  fn topLevelAt_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10topLevelAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN12QApplication10topLevelAtEii(arg0, arg1)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QApplication::setStyle(QStyle * );
impl /*struct*/ QApplication {
  pub fn setStyle_s<RetType, T: QApplication_setStyle_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setStyle_s();
    // return 1;
  }
}

pub trait QApplication_setStyle_s<RetType> {
  fn setStyle_s(self ) -> RetType;
}

  // proto: static void QApplication::setStyle(QStyle * );
impl<'a> /*trait*/ QApplication_setStyle_s<()> for (&'a QStyle) {
  fn setStyle_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8setStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QApplication8setStyleEP6QStyle(arg0)};
    // return 1;
  }
}

  // proto:  void QApplication::~QApplication();
impl /*struct*/ QApplication {
  pub fn free<RetType, T: QApplication_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QApplication_free<RetType> {
  fn free(self , rsthis: & QApplication) -> RetType;
}

  // proto:  void QApplication::~QApplication();
impl<'a> /*trait*/ QApplication_free<()> for () {
  fn free(self , rsthis: & QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplicationD2Ev()};
     unsafe {C_ZN12QApplicationD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QApplication::setDoubleClickInterval(int );
impl /*struct*/ QApplication {
  pub fn setDoubleClickInterval_s<RetType, T: QApplication_setDoubleClickInterval_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDoubleClickInterval_s();
    // return 1;
  }
}

pub trait QApplication_setDoubleClickInterval_s<RetType> {
  fn setDoubleClickInterval_s(self ) -> RetType;
}

  // proto: static void QApplication::setDoubleClickInterval(int );
impl<'a> /*trait*/ QApplication_setDoubleClickInterval_s<()> for (i32) {
  fn setDoubleClickInterval_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication22setDoubleClickIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QApplication22setDoubleClickIntervalEi(arg0)};
    // return 1;
  }
}

  // proto: static void QApplication::setGlobalStrut(const QSize & );
impl /*struct*/ QApplication {
  pub fn setGlobalStrut_s<RetType, T: QApplication_setGlobalStrut_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setGlobalStrut_s();
    // return 1;
  }
}

pub trait QApplication_setGlobalStrut_s<RetType> {
  fn setGlobalStrut_s(self ) -> RetType;
}

  // proto: static void QApplication::setGlobalStrut(const QSize & );
impl<'a> /*trait*/ QApplication_setGlobalStrut_s<()> for (&'a QSize) {
  fn setGlobalStrut_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication14setGlobalStrutERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QApplication14setGlobalStrutERK5QSize(arg0)};
    // return 1;
  }
}

  // proto: static void QApplication::setColorSpec(int );
impl /*struct*/ QApplication {
  pub fn setColorSpec_s<RetType, T: QApplication_setColorSpec_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setColorSpec_s();
    // return 1;
  }
}

pub trait QApplication_setColorSpec_s<RetType> {
  fn setColorSpec_s(self ) -> RetType;
}

  // proto: static void QApplication::setColorSpec(int );
impl<'a> /*trait*/ QApplication_setColorSpec_s<()> for (i32) {
  fn setColorSpec_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication12setColorSpecEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QApplication12setColorSpecEi(arg0)};
    // return 1;
  }
}

  // proto: static QWidgetList QApplication::allWidgets();
impl /*struct*/ QApplication {
  pub fn allWidgets_s<RetType, T: QApplication_allWidgets_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.allWidgets_s();
    // return 1;
  }
}

pub trait QApplication_allWidgets_s<RetType> {
  fn allWidgets_s(self ) -> RetType;
}

  // proto: static QWidgetList QApplication::allWidgets();
impl<'a> /*trait*/ QApplication_allWidgets_s<u64> for () {
  fn allWidgets_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10allWidgetsEv()};
    let mut ret = unsafe {C_ZN12QApplication10allWidgetsEv()};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto: static QSize QApplication::globalStrut();
impl /*struct*/ QApplication {
  pub fn globalStrut_s<RetType, T: QApplication_globalStrut_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.globalStrut_s();
    // return 1;
  }
}

pub trait QApplication_globalStrut_s<RetType> {
  fn globalStrut_s(self ) -> RetType;
}

  // proto: static QSize QApplication::globalStrut();
impl<'a> /*trait*/ QApplication_globalStrut_s<QSize> for () {
  fn globalStrut_s(self ) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication11globalStrutEv()};
    let mut ret = unsafe {C_ZN12QApplication11globalStrutEv()};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QApplication::setPalette(const QPalette & , const char * className);
impl /*struct*/ QApplication {
  pub fn setPalette_s<RetType, T: QApplication_setPalette_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPalette_s();
    // return 1;
  }
}

pub trait QApplication_setPalette_s<RetType> {
  fn setPalette_s(self ) -> RetType;
}

  // proto: static void QApplication::setPalette(const QPalette & , const char * className);
impl<'a> /*trait*/ QApplication_setPalette_s<()> for (&'a QPalette, &'a  String) {
  fn setPalette_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10setPaletteERK8QPalettePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {C_ZN12QApplication10setPaletteERK8QPalettePKc(arg0, arg1)};
    // return 1;
  }
}

  // proto: static QStyle * QApplication::setStyle(const QString & );
impl<'a> /*trait*/ QApplication_setStyle_s<QStyle> for (&'a QString) {
  fn setStyle_s(self ) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8setStyleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QApplication8setStyleERK7QString(arg0)};
    let mut ret1 = QStyle::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QWidgetList QApplication::topLevelWidgets();
impl /*struct*/ QApplication {
  pub fn topLevelWidgets_s<RetType, T: QApplication_topLevelWidgets_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.topLevelWidgets_s();
    // return 1;
  }
}

pub trait QApplication_topLevelWidgets_s<RetType> {
  fn topLevelWidgets_s(self ) -> RetType;
}

  // proto: static QWidgetList QApplication::topLevelWidgets();
impl<'a> /*trait*/ QApplication_topLevelWidgets_s<u64> for () {
  fn topLevelWidgets_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15topLevelWidgetsEv()};
    let mut ret = unsafe {C_ZN12QApplication15topLevelWidgetsEv()};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto: static int QApplication::exec();
impl /*struct*/ QApplication {
  pub fn exec_s<RetType, T: QApplication_exec_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.exec_s();
    // return 1;
  }
}

pub trait QApplication_exec_s<RetType> {
  fn exec_s(self ) -> RetType;
}

  // proto: static int QApplication::exec();
impl<'a> /*trait*/ QApplication_exec_s<i32> for () {
  fn exec_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4execEv()};
    let mut ret = unsafe {C_ZN12QApplication4execEv()};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static void QApplication::setWindowIcon(const QIcon & icon);
impl /*struct*/ QApplication {
  pub fn setWindowIcon_s<RetType, T: QApplication_setWindowIcon_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setWindowIcon_s();
    // return 1;
  }
}

pub trait QApplication_setWindowIcon_s<RetType> {
  fn setWindowIcon_s(self ) -> RetType;
}

  // proto: static void QApplication::setWindowIcon(const QIcon & icon);
impl<'a> /*trait*/ QApplication_setWindowIcon_s<()> for (&'a QIcon) {
  fn setWindowIcon_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication13setWindowIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QApplication13setWindowIconERK5QIcon(arg0)};
    // return 1;
  }
}

  // proto: static void QApplication::beep();
impl /*struct*/ QApplication {
  pub fn beep_s<RetType, T: QApplication_beep_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.beep_s();
    // return 1;
  }
}

pub trait QApplication_beep_s<RetType> {
  fn beep_s(self ) -> RetType;
}

  // proto: static void QApplication::beep();
impl<'a> /*trait*/ QApplication_beep_s<()> for () {
  fn beep_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4beepEv()};
     unsafe {C_ZN12QApplication4beepEv()};
    // return 1;
  }
}

  // proto:  bool QApplication::notify(QObject * , QEvent * );
impl /*struct*/ QApplication {
  pub fn notify<RetType, T: QApplication_notify<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.notify(self);
    // return 1;
  }
}

pub trait QApplication_notify<RetType> {
  fn notify(self , rsthis: & QApplication) -> RetType;
}

  // proto:  bool QApplication::notify(QObject * , QEvent * );
impl<'a> /*trait*/ QApplication_notify<i8> for (&'a QObject, &'a QEvent) {
  fn notify(self , rsthis: & QApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication6notifyEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QApplication6notifyEP7QObjectP6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QApplication::autoSipEnabled();
impl /*struct*/ QApplication {
  pub fn autoSipEnabled<RetType, T: QApplication_autoSipEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoSipEnabled(self);
    // return 1;
  }
}

pub trait QApplication_autoSipEnabled<RetType> {
  fn autoSipEnabled(self , rsthis: & QApplication) -> RetType;
}

  // proto:  bool QApplication::autoSipEnabled();
impl<'a> /*trait*/ QApplication_autoSipEnabled<i8> for () {
  fn autoSipEnabled(self , rsthis: & QApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QApplication14autoSipEnabledEv()};
    let mut ret = unsafe {C_ZNK12QApplication14autoSipEnabledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static QWidget * QApplication::topLevelAt(const QPoint & p);
impl<'a> /*trait*/ QApplication_topLevelAt_s<QWidget> for (&'a QPoint) {
  fn topLevelAt_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10topLevelAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QApplication10topLevelAtERK6QPoint(arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static int QApplication::startDragTime();
impl /*struct*/ QApplication {
  pub fn startDragTime_s<RetType, T: QApplication_startDragTime_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.startDragTime_s();
    // return 1;
  }
}

pub trait QApplication_startDragTime_s<RetType> {
  fn startDragTime_s(self ) -> RetType;
}

  // proto: static int QApplication::startDragTime();
impl<'a> /*trait*/ QApplication_startDragTime_s<i32> for () {
  fn startDragTime_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication13startDragTimeEv()};
    let mut ret = unsafe {C_ZN12QApplication13startDragTimeEv()};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static int QApplication::doubleClickInterval();
impl /*struct*/ QApplication {
  pub fn doubleClickInterval_s<RetType, T: QApplication_doubleClickInterval_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.doubleClickInterval_s();
    // return 1;
  }
}

pub trait QApplication_doubleClickInterval_s<RetType> {
  fn doubleClickInterval_s(self ) -> RetType;
}

  // proto: static int QApplication::doubleClickInterval();
impl<'a> /*trait*/ QApplication_doubleClickInterval_s<i32> for () {
  fn doubleClickInterval_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication19doubleClickIntervalEv()};
    let mut ret = unsafe {C_ZN12QApplication19doubleClickIntervalEv()};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static QIcon QApplication::windowIcon();
impl /*struct*/ QApplication {
  pub fn windowIcon_s<RetType, T: QApplication_windowIcon_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.windowIcon_s();
    // return 1;
  }
}

pub trait QApplication_windowIcon_s<RetType> {
  fn windowIcon_s(self ) -> RetType;
}

  // proto: static QIcon QApplication::windowIcon();
impl<'a> /*trait*/ QApplication_windowIcon_s<QIcon> for () {
  fn windowIcon_s(self ) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10windowIconEv()};
    let mut ret = unsafe {C_ZN12QApplication10windowIconEv()};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QApplication_focusChanged
pub struct QApplication_focusChanged_signal{poi:u64}
impl /* struct */ QApplication {
  pub fn focusChanged(&self) -> QApplication_focusChanged_signal {
     return QApplication_focusChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QApplication_focusChanged_signal {
  pub fn connect<T: QApplication_focusChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QApplication_focusChanged_signal_connect {
  fn connect(self, sigthis: QApplication_focusChanged_signal);
}

// focusChanged(class QWidget *, class QWidget *)
extern fn QApplication_focusChanged_signal_connect_cb_0(rsfptr:fn(QWidget, QWidget), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QWidget::inheritFrom(arg0 as u64);
  let rsarg1 = QWidget::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
extern fn QApplication_focusChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QWidget, QWidget)>, arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QWidget::inheritFrom(arg0 as u64);
  let rsarg1 = QWidget::inheritFrom(arg1 as u64);
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QApplication_focusChanged_signal_connect for fn(QWidget, QWidget) {
  fn connect(self, sigthis: QApplication_focusChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QApplication_focusChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QApplication_SlotProxy_connect__ZN12QApplication12focusChangedEP7QWidgetS1_(arg0, arg1, arg2)};
  }
}
impl /* trait */ QApplication_focusChanged_signal_connect for Box<Fn(QWidget, QWidget)> {
  fn connect(self, sigthis: QApplication_focusChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QApplication_focusChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QApplication_SlotProxy_connect__ZN12QApplication12focusChangedEP7QWidgetS1_(arg0, arg1, arg2)};
  }
}
// <= body block end

