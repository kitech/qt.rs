// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qpalette::QPalette;
use super::qwidget::QWidget;
use super::qfontmetrics::QFontMetrics;
use super::qfont::QFont;
use super::qstyle::QStyle;
use super::qpoint::QPoint;
use super::qdesktopwidget::QDesktopWidget;
use super::qsize::QSize;
use super::qicon::QIcon;
use super::qobject::QObject;
use super::qevent::QEvent;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QApplication::styleSheet();
  fn _ZNK12QApplication10styleSheetEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QPalette QApplication::palette(const char * className);
  fn _ZN12QApplication7paletteEPKc(arg0: *const c_char) -> *mut c_void;
  // proto: static QWidget * QApplication::activeWindow();
  fn _ZN12QApplication12activeWindowEv() -> *mut c_void;
  // proto: static void QApplication::setKeyboardInputInterval(int );
  fn _ZN12QApplication24setKeyboardInputIntervalEi(arg0: c_int) ;
  // proto: static QWidget * QApplication::focusWidget();
  fn _ZN12QApplication11focusWidgetEv() -> *mut c_void;
  // proto: static QFontMetrics QApplication::fontMetrics();
  fn _ZN12QApplication11fontMetricsEv() -> *mut c_void;
  // proto: static QFont QApplication::font(const char * className);
  fn _ZN12QApplication4fontEPKc(arg0: *const c_char) -> *mut c_void;
  // proto: static QStyle * QApplication::style();
  fn _ZN12QApplication5styleEv() -> *mut c_void;
  // proto: static QWidget * QApplication::widgetAt(const QPoint & p);
  fn _ZN12QApplication8widgetAtERK6QPoint(arg0: *mut c_void) -> *mut c_void;
  // proto: static void QApplication::setActiveWindow(QWidget * act);
  fn _ZN12QApplication15setActiveWindowEP7QWidget(arg0: *mut c_void) ;
  // proto: static QFont QApplication::font();
  fn _ZN12QApplication4fontEv() -> *mut c_void;
  // proto: static void QApplication::setWheelScrollLines(int );
  fn _ZN12QApplication19setWheelScrollLinesEi(arg0: c_int) ;
  // proto:  void QApplication::setStyleSheet(const QString & sheet);
  fn _ZN12QApplication13setStyleSheetERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QApplication::setAutoSipEnabled(const bool enabled);
  fn _ZN12QApplication17setAutoSipEnabledEb(qthis: *mut c_void, arg0: bool) ;
  // proto:  const QMetaObject * QApplication::metaObject();
  fn _ZNK12QApplication10metaObjectEv(qthis: *mut c_void) ;
  // proto: static int QApplication::keyboardInputInterval();
  fn _ZN12QApplication21keyboardInputIntervalEv() -> c_int;
  // proto: static int QApplication::cursorFlashTime();
  fn _ZN12QApplication15cursorFlashTimeEv() -> c_int;
  // proto: static int QApplication::startDragDistance();
  fn _ZN12QApplication17startDragDistanceEv() -> c_int;
  // proto: static QDesktopWidget * QApplication::desktop();
  fn _ZN12QApplication7desktopEv() -> *mut c_void;
  // proto: static void QApplication::setStartDragDistance(int l);
  fn _ZN12QApplication20setStartDragDistanceEi(arg0: c_int) ;
  // proto: static QFont QApplication::font(const QWidget * );
  fn _ZN12QApplication4fontEPK7QWidget(arg0: *mut c_void) -> *mut c_void;
  // proto: static int QApplication::colorSpec();
  fn _ZN12QApplication9colorSpecEv() -> c_int;
  // proto: static void QApplication::setFont(const QFont & , const char * className);
  fn _ZN12QApplication7setFontERK5QFontPKc(arg0: *mut c_void, arg1: *const c_char) ;
  // proto: static void QApplication::closeAllWindows();
  fn _ZN12QApplication15closeAllWindowsEv() ;
  // proto: static void QApplication::setCursorFlashTime(int );
  fn _ZN12QApplication18setCursorFlashTimeEi(arg0: c_int) ;
  // proto: static QWidget * QApplication::widgetAt(int x, int y);
  fn _ZN12QApplication8widgetAtEii(arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto: static void QApplication::alert(QWidget * widget, int duration);
  fn _ZN12QApplication5alertEP7QWidgeti(arg0: *mut c_void, arg1: c_int) ;
  // proto: static QPalette QApplication::palette(const QWidget * );
  fn _ZN12QApplication7paletteEPK7QWidget(arg0: *mut c_void) -> *mut c_void;
  // proto: static int QApplication::wheelScrollLines();
  fn _ZN12QApplication16wheelScrollLinesEv() -> c_int;
  // proto:  void QApplication::NewQApplication(const QApplication & );
  fn _ZN12QApplicationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static void QApplication::aboutQt();
  fn _ZN12QApplication7aboutQtEv() ;
  // proto: static QWidget * QApplication::activeModalWidget();
  fn _ZN12QApplication17activeModalWidgetEv() -> *mut c_void;
  // proto: static QWidget * QApplication::activePopupWidget();
  fn _ZN12QApplication17activePopupWidgetEv() -> *mut c_void;
  // proto:  void QApplication::NewQApplication(int & argc, char ** argv, int );
  fn _ZN12QApplicationC1ERiPPci(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_char, arg2: c_int) ;
  // proto:  void QApplication::focusChanged(QWidget * old, QWidget * now);
  fn _ZN12QApplication12focusChangedEP7QWidgetS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto: static void QApplication::setStartDragTime(int ms);
  fn _ZN12QApplication16setStartDragTimeEi(arg0: c_int) ;
  // proto: static QWidget * QApplication::topLevelAt(int x, int y);
  fn _ZN12QApplication10topLevelAtEii(arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto: static void QApplication::setStyle(QStyle * );
  fn _ZN12QApplication8setStyleEP6QStyle(arg0: *mut c_void) ;
  // proto:  void QApplication::FreeQApplication();
  fn _ZN12QApplicationD0Ev(qthis: *mut c_void) ;
  // proto: static void QApplication::setDoubleClickInterval(int );
  fn _ZN12QApplication22setDoubleClickIntervalEi(arg0: c_int) ;
  // proto: static void QApplication::setGlobalStrut(const QSize & );
  fn _ZN12QApplication14setGlobalStrutERK5QSize(arg0: *mut c_void) ;
  // proto: static void QApplication::setColorSpec(int );
  fn _ZN12QApplication12setColorSpecEi(arg0: c_int) ;
  // proto: static QList<QWidget *> QApplication::allWidgets();
  fn _ZN12QApplication10allWidgetsEv() ;
  // proto: static QSize QApplication::globalStrut();
  fn _ZN12QApplication11globalStrutEv() -> *mut c_void;
  // proto: static void QApplication::setPalette(const QPalette & , const char * className);
  fn _ZN12QApplication10setPaletteERK8QPalettePKc(arg0: *mut c_void, arg1: *const c_char) ;
  // proto: static QStyle * QApplication::setStyle(const QString & );
  fn _ZN12QApplication8setStyleERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static QList<QWidget *> QApplication::topLevelWidgets();
  fn _ZN12QApplication15topLevelWidgetsEv() ;
  // proto: static int QApplication::exec();
  fn _ZN12QApplication4execEv() -> c_int;
  // proto: static void QApplication::setWindowIcon(const QIcon & icon);
  fn _ZN12QApplication13setWindowIconERK5QIcon(arg0: *mut c_void) ;
  // proto: static void QApplication::beep();
  fn _ZN12QApplication4beepEv() ;
  // proto:  bool QApplication::notify(QObject * , QEvent * );
  fn _ZN12QApplication6notifyEP7QObjectP6QEvent(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  bool QApplication::autoSipEnabled();
  fn _ZNK12QApplication14autoSipEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto: static QWidget * QApplication::topLevelAt(const QPoint & p);
  fn _ZN12QApplication10topLevelAtERK6QPoint(arg0: *mut c_void) -> *mut c_void;
  // proto: static int QApplication::startDragTime();
  fn _ZN12QApplication13startDragTimeEv() -> c_int;
  // proto: static int QApplication::doubleClickInterval();
  fn _ZN12QApplication19doubleClickIntervalEv() -> c_int;
  // proto: static QIcon QApplication::windowIcon();
  fn _ZN12QApplication10windowIconEv() -> *mut c_void;
}

// body block begin
// class sizeof(QApplication)=1
pub struct QApplication {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QApplication {
  pub fn styleSheet<RetType, T: QApplication_styleSheet<RetType>>(&mut self, value: T) -> RetType {
    return value.styleSheet(self);
    // return 1;
  }
}

pub trait QApplication_styleSheet<RetType> {
  fn styleSheet(self, rsthis: &mut QApplication) -> RetType;
}

// proto:  QString QApplication::styleSheet();
impl<'a> /*trait*/ QApplication_styleSheet<QString> for () {
  fn styleSheet(self, rsthis: &mut QApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QApplication10styleSheetEv()};
    let mut ret = unsafe {_ZNK12QApplication10styleSheetEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn palette<RetType, T: QApplication_palette<RetType>>(&mut self, value: T) -> RetType {
    return value.palette(self);
    // return 1;
  }
}

pub trait QApplication_palette<RetType> {
  fn palette(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QPalette QApplication::palette(const char * className);
impl<'a> /*trait*/ QApplication_palette<QPalette> for (&'a  String) {
  fn palette(self, rsthis: &mut QApplication) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7paletteEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN12QApplication7paletteEPKc(arg0)};
    let mut ret1 = QPalette{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn activeWindow<RetType, T: QApplication_activeWindow<RetType>>(&mut self, value: T) -> RetType {
    return value.activeWindow(self);
    // return 1;
  }
}

pub trait QApplication_activeWindow<RetType> {
  fn activeWindow(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QWidget * QApplication::activeWindow();
impl<'a> /*trait*/ QApplication_activeWindow<QWidget> for () {
  fn activeWindow(self, rsthis: &mut QApplication) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication12activeWindowEv()};
    let mut ret = unsafe {_ZN12QApplication12activeWindowEv()};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setKeyboardInputInterval<RetType, T: QApplication_setKeyboardInputInterval<RetType>>(&mut self, value: T) -> RetType {
    return value.setKeyboardInputInterval(self);
    // return 1;
  }
}

pub trait QApplication_setKeyboardInputInterval<RetType> {
  fn setKeyboardInputInterval(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::setKeyboardInputInterval(int );
impl<'a> /*trait*/ QApplication_setKeyboardInputInterval<()> for (i32) {
  fn setKeyboardInputInterval(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication24setKeyboardInputIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication24setKeyboardInputIntervalEi(arg0)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn focusWidget<RetType, T: QApplication_focusWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.focusWidget(self);
    // return 1;
  }
}

pub trait QApplication_focusWidget<RetType> {
  fn focusWidget(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QWidget * QApplication::focusWidget();
impl<'a> /*trait*/ QApplication_focusWidget<QWidget> for () {
  fn focusWidget(self, rsthis: &mut QApplication) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication11focusWidgetEv()};
    let mut ret = unsafe {_ZN12QApplication11focusWidgetEv()};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn fontMetrics<RetType, T: QApplication_fontMetrics<RetType>>(&mut self, value: T) -> RetType {
    return value.fontMetrics(self);
    // return 1;
  }
}

pub trait QApplication_fontMetrics<RetType> {
  fn fontMetrics(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QFontMetrics QApplication::fontMetrics();
impl<'a> /*trait*/ QApplication_fontMetrics<QFontMetrics> for () {
  fn fontMetrics(self, rsthis: &mut QApplication) -> QFontMetrics {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication11fontMetricsEv()};
    let mut ret = unsafe {_ZN12QApplication11fontMetricsEv()};
    let mut ret1 = QFontMetrics{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn font<RetType, T: QApplication_font<RetType>>(&mut self, value: T) -> RetType {
    return value.font(self);
    // return 1;
  }
}

pub trait QApplication_font<RetType> {
  fn font(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QFont QApplication::font(const char * className);
impl<'a> /*trait*/ QApplication_font<QFont> for (&'a  String) {
  fn font(self, rsthis: &mut QApplication) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4fontEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN12QApplication4fontEPKc(arg0)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn style<RetType, T: QApplication_style<RetType>>(&mut self, value: T) -> RetType {
    return value.style(self);
    // return 1;
  }
}

pub trait QApplication_style<RetType> {
  fn style(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QStyle * QApplication::style();
impl<'a> /*trait*/ QApplication_style<QStyle> for () {
  fn style(self, rsthis: &mut QApplication) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication5styleEv()};
    let mut ret = unsafe {_ZN12QApplication5styleEv()};
    let mut ret1 = QStyle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn widgetAt<RetType, T: QApplication_widgetAt<RetType>>(&mut self, value: T) -> RetType {
    return value.widgetAt(self);
    // return 1;
  }
}

pub trait QApplication_widgetAt<RetType> {
  fn widgetAt(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QWidget * QApplication::widgetAt(const QPoint & p);
impl<'a> /*trait*/ QApplication_widgetAt<QWidget> for (&'a  QPoint) {
  fn widgetAt(self, rsthis: &mut QApplication) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8widgetAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QApplication8widgetAtERK6QPoint(arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setActiveWindow<RetType, T: QApplication_setActiveWindow<RetType>>(&mut self, value: T) -> RetType {
    return value.setActiveWindow(self);
    // return 1;
  }
}

pub trait QApplication_setActiveWindow<RetType> {
  fn setActiveWindow(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::setActiveWindow(QWidget * act);
impl<'a> /*trait*/ QApplication_setActiveWindow<()> for (&'a mut QWidget) {
  fn setActiveWindow(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15setActiveWindowEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QApplication15setActiveWindowEP7QWidget(arg0)};
    // return 1;
  }
}

// proto: static QFont QApplication::font();
impl<'a> /*trait*/ QApplication_font<QFont> for () {
  fn font(self, rsthis: &mut QApplication) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4fontEv()};
    let mut ret = unsafe {_ZN12QApplication4fontEv()};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setWheelScrollLines<RetType, T: QApplication_setWheelScrollLines<RetType>>(&mut self, value: T) -> RetType {
    return value.setWheelScrollLines(self);
    // return 1;
  }
}

pub trait QApplication_setWheelScrollLines<RetType> {
  fn setWheelScrollLines(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::setWheelScrollLines(int );
impl<'a> /*trait*/ QApplication_setWheelScrollLines<()> for (i32) {
  fn setWheelScrollLines(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication19setWheelScrollLinesEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication19setWheelScrollLinesEi(arg0)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setStyleSheet<RetType, T: QApplication_setStyleSheet<RetType>>(&mut self, value: T) -> RetType {
    return value.setStyleSheet(self);
    // return 1;
  }
}

pub trait QApplication_setStyleSheet<RetType> {
  fn setStyleSheet(self, rsthis: &mut QApplication) -> RetType;
}

// proto:  void QApplication::setStyleSheet(const QString & sheet);
impl<'a> /*trait*/ QApplication_setStyleSheet<()> for (&'a  QString) {
  fn setStyleSheet(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication13setStyleSheetERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QApplication13setStyleSheetERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setAutoSipEnabled<RetType, T: QApplication_setAutoSipEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setAutoSipEnabled(self);
    // return 1;
  }
}

pub trait QApplication_setAutoSipEnabled<RetType> {
  fn setAutoSipEnabled(self, rsthis: &mut QApplication) -> RetType;
}

// proto:  void QApplication::setAutoSipEnabled(const bool enabled);
impl<'a> /*trait*/ QApplication_setAutoSipEnabled<()> for (bool) {
  fn setAutoSipEnabled(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17setAutoSipEnabledEb()};
    let arg0 = self  as bool;
     unsafe {_ZN12QApplication17setAutoSipEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn metaObject<RetType, T: QApplication_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QApplication_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QApplication) -> RetType;
}

// proto:  const QMetaObject * QApplication::metaObject();
impl<'a> /*trait*/ QApplication_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QApplication10metaObjectEv()};
     unsafe {_ZNK12QApplication10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn keyboardInputInterval<RetType, T: QApplication_keyboardInputInterval<RetType>>(&mut self, value: T) -> RetType {
    return value.keyboardInputInterval(self);
    // return 1;
  }
}

pub trait QApplication_keyboardInputInterval<RetType> {
  fn keyboardInputInterval(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static int QApplication::keyboardInputInterval();
impl<'a> /*trait*/ QApplication_keyboardInputInterval<i32> for () {
  fn keyboardInputInterval(self, rsthis: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication21keyboardInputIntervalEv()};
    let mut ret = unsafe {_ZN12QApplication21keyboardInputIntervalEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn cursorFlashTime<RetType, T: QApplication_cursorFlashTime<RetType>>(&mut self, value: T) -> RetType {
    return value.cursorFlashTime(self);
    // return 1;
  }
}

pub trait QApplication_cursorFlashTime<RetType> {
  fn cursorFlashTime(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static int QApplication::cursorFlashTime();
impl<'a> /*trait*/ QApplication_cursorFlashTime<i32> for () {
  fn cursorFlashTime(self, rsthis: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15cursorFlashTimeEv()};
    let mut ret = unsafe {_ZN12QApplication15cursorFlashTimeEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn startDragDistance<RetType, T: QApplication_startDragDistance<RetType>>(&mut self, value: T) -> RetType {
    return value.startDragDistance(self);
    // return 1;
  }
}

pub trait QApplication_startDragDistance<RetType> {
  fn startDragDistance(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static int QApplication::startDragDistance();
impl<'a> /*trait*/ QApplication_startDragDistance<i32> for () {
  fn startDragDistance(self, rsthis: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17startDragDistanceEv()};
    let mut ret = unsafe {_ZN12QApplication17startDragDistanceEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn desktop<RetType, T: QApplication_desktop<RetType>>(&mut self, value: T) -> RetType {
    return value.desktop(self);
    // return 1;
  }
}

pub trait QApplication_desktop<RetType> {
  fn desktop(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QDesktopWidget * QApplication::desktop();
impl<'a> /*trait*/ QApplication_desktop<QDesktopWidget> for () {
  fn desktop(self, rsthis: &mut QApplication) -> QDesktopWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7desktopEv()};
    let mut ret = unsafe {_ZN12QApplication7desktopEv()};
    let mut ret1 = QDesktopWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setStartDragDistance<RetType, T: QApplication_setStartDragDistance<RetType>>(&mut self, value: T) -> RetType {
    return value.setStartDragDistance(self);
    // return 1;
  }
}

pub trait QApplication_setStartDragDistance<RetType> {
  fn setStartDragDistance(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::setStartDragDistance(int l);
impl<'a> /*trait*/ QApplication_setStartDragDistance<()> for (i32) {
  fn setStartDragDistance(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication20setStartDragDistanceEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication20setStartDragDistanceEi(arg0)};
    // return 1;
  }
}

// proto: static QFont QApplication::font(const QWidget * );
impl<'a> /*trait*/ QApplication_font<QFont> for (&'a  QWidget) {
  fn font(self, rsthis: &mut QApplication) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4fontEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QApplication4fontEPK7QWidget(arg0)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn colorSpec<RetType, T: QApplication_colorSpec<RetType>>(&mut self, value: T) -> RetType {
    return value.colorSpec(self);
    // return 1;
  }
}

pub trait QApplication_colorSpec<RetType> {
  fn colorSpec(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static int QApplication::colorSpec();
impl<'a> /*trait*/ QApplication_colorSpec<i32> for () {
  fn colorSpec(self, rsthis: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication9colorSpecEv()};
    let mut ret = unsafe {_ZN12QApplication9colorSpecEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setFont<RetType, T: QApplication_setFont<RetType>>(&mut self, value: T) -> RetType {
    return value.setFont(self);
    // return 1;
  }
}

pub trait QApplication_setFont<RetType> {
  fn setFont(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::setFont(const QFont & , const char * className);
impl<'a> /*trait*/ QApplication_setFont<()> for (&'a  QFont, &'a  String) {
  fn setFont(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7setFontERK5QFontPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
     unsafe {_ZN12QApplication7setFontERK5QFontPKc(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn closeAllWindows<RetType, T: QApplication_closeAllWindows<RetType>>(&mut self, value: T) -> RetType {
    return value.closeAllWindows(self);
    // return 1;
  }
}

pub trait QApplication_closeAllWindows<RetType> {
  fn closeAllWindows(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::closeAllWindows();
impl<'a> /*trait*/ QApplication_closeAllWindows<()> for () {
  fn closeAllWindows(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15closeAllWindowsEv()};
     unsafe {_ZN12QApplication15closeAllWindowsEv()};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setCursorFlashTime<RetType, T: QApplication_setCursorFlashTime<RetType>>(&mut self, value: T) -> RetType {
    return value.setCursorFlashTime(self);
    // return 1;
  }
}

pub trait QApplication_setCursorFlashTime<RetType> {
  fn setCursorFlashTime(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::setCursorFlashTime(int );
impl<'a> /*trait*/ QApplication_setCursorFlashTime<()> for (i32) {
  fn setCursorFlashTime(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication18setCursorFlashTimeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication18setCursorFlashTimeEi(arg0)};
    // return 1;
  }
}

// proto: static QWidget * QApplication::widgetAt(int x, int y);
impl<'a> /*trait*/ QApplication_widgetAt<QWidget> for (i32, i32) {
  fn widgetAt(self, rsthis: &mut QApplication) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8widgetAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN12QApplication8widgetAtEii(arg0, arg1)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn alert<RetType, T: QApplication_alert<RetType>>(&mut self, value: T) -> RetType {
    return value.alert(self);
    // return 1;
  }
}

pub trait QApplication_alert<RetType> {
  fn alert(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::alert(QWidget * widget, int duration);
impl<'a> /*trait*/ QApplication_alert<()> for (&'a mut QWidget, i32) {
  fn alert(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication5alertEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QApplication5alertEP7QWidgeti(arg0, arg1)};
    // return 1;
  }
}

// proto: static QPalette QApplication::palette(const QWidget * );
impl<'a> /*trait*/ QApplication_palette<QPalette> for (&'a  QWidget) {
  fn palette(self, rsthis: &mut QApplication) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7paletteEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QApplication7paletteEPK7QWidget(arg0)};
    let mut ret1 = QPalette{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn wheelScrollLines<RetType, T: QApplication_wheelScrollLines<RetType>>(&mut self, value: T) -> RetType {
    return value.wheelScrollLines(self);
    // return 1;
  }
}

pub trait QApplication_wheelScrollLines<RetType> {
  fn wheelScrollLines(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static int QApplication::wheelScrollLines();
impl<'a> /*trait*/ QApplication_wheelScrollLines<i32> for () {
  fn wheelScrollLines(self, rsthis: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication16wheelScrollLinesEv()};
    let mut ret = unsafe {_ZN12QApplication16wheelScrollLinesEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn NewQApplication<T: QApplication_NewQApplication>(value: T) -> QApplication {
    let rsthis = value.NewQApplication();
    return rsthis;
    // return 1;
  }
}

pub trait QApplication_NewQApplication {
  fn NewQApplication(self) -> QApplication;
}

// proto: void QApplication::NewQApplication(const QApplication & );
impl<'a> /*trait*/ QApplication_NewQApplication for (&'a  QApplication) {
  fn NewQApplication(self) -> QApplication {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplicationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QApplicationC1ERKS_(qthis, arg0)};
    let rsthis = QApplication{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn aboutQt<RetType, T: QApplication_aboutQt<RetType>>(&mut self, value: T) -> RetType {
    return value.aboutQt(self);
    // return 1;
  }
}

pub trait QApplication_aboutQt<RetType> {
  fn aboutQt(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::aboutQt();
impl<'a> /*trait*/ QApplication_aboutQt<()> for () {
  fn aboutQt(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7aboutQtEv()};
     unsafe {_ZN12QApplication7aboutQtEv()};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn activeModalWidget<RetType, T: QApplication_activeModalWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.activeModalWidget(self);
    // return 1;
  }
}

pub trait QApplication_activeModalWidget<RetType> {
  fn activeModalWidget(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QWidget * QApplication::activeModalWidget();
impl<'a> /*trait*/ QApplication_activeModalWidget<QWidget> for () {
  fn activeModalWidget(self, rsthis: &mut QApplication) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17activeModalWidgetEv()};
    let mut ret = unsafe {_ZN12QApplication17activeModalWidgetEv()};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn activePopupWidget<RetType, T: QApplication_activePopupWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.activePopupWidget(self);
    // return 1;
  }
}

pub trait QApplication_activePopupWidget<RetType> {
  fn activePopupWidget(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QWidget * QApplication::activePopupWidget();
impl<'a> /*trait*/ QApplication_activePopupWidget<QWidget> for () {
  fn activePopupWidget(self, rsthis: &mut QApplication) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17activePopupWidgetEv()};
    let mut ret = unsafe {_ZN12QApplication17activePopupWidgetEv()};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QApplication::NewQApplication(int & argc, char ** argv, int );
impl<'a> /*trait*/ QApplication_NewQApplication for (&'a mut i32, &'a mut String, i32) {
  fn NewQApplication(self) -> QApplication {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplicationC1ERiPPci()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZN12QApplicationC1ERiPPci(qthis, arg0, arg1, arg2)};
    let rsthis = QApplication{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn focusChanged<RetType, T: QApplication_focusChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.focusChanged(self);
    // return 1;
  }
}

pub trait QApplication_focusChanged<RetType> {
  fn focusChanged(self, rsthis: &mut QApplication) -> RetType;
}

// proto:  void QApplication::focusChanged(QWidget * old, QWidget * now);
impl<'a> /*trait*/ QApplication_focusChanged<()> for (&'a mut QWidget, &'a mut QWidget) {
  fn focusChanged(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication12focusChangedEP7QWidgetS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QApplication12focusChangedEP7QWidgetS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setStartDragTime<RetType, T: QApplication_setStartDragTime<RetType>>(&mut self, value: T) -> RetType {
    return value.setStartDragTime(self);
    // return 1;
  }
}

pub trait QApplication_setStartDragTime<RetType> {
  fn setStartDragTime(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::setStartDragTime(int ms);
impl<'a> /*trait*/ QApplication_setStartDragTime<()> for (i32) {
  fn setStartDragTime(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication16setStartDragTimeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication16setStartDragTimeEi(arg0)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn topLevelAt<RetType, T: QApplication_topLevelAt<RetType>>(&mut self, value: T) -> RetType {
    return value.topLevelAt(self);
    // return 1;
  }
}

pub trait QApplication_topLevelAt<RetType> {
  fn topLevelAt(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QWidget * QApplication::topLevelAt(int x, int y);
impl<'a> /*trait*/ QApplication_topLevelAt<QWidget> for (i32, i32) {
  fn topLevelAt(self, rsthis: &mut QApplication) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10topLevelAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN12QApplication10topLevelAtEii(arg0, arg1)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setStyle<RetType, T: QApplication_setStyle<RetType>>(&mut self, value: T) -> RetType {
    return value.setStyle(self);
    // return 1;
  }
}

pub trait QApplication_setStyle<RetType> {
  fn setStyle(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::setStyle(QStyle * );
impl<'a> /*trait*/ QApplication_setStyle<()> for (&'a mut QStyle) {
  fn setStyle(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8setStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QApplication8setStyleEP6QStyle(arg0)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn FreeQApplication<RetType, T: QApplication_FreeQApplication<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQApplication(self);
    // return 1;
  }
}

pub trait QApplication_FreeQApplication<RetType> {
  fn FreeQApplication(self, rsthis: &mut QApplication) -> RetType;
}

// proto:  void QApplication::FreeQApplication();
impl<'a> /*trait*/ QApplication_FreeQApplication<()> for () {
  fn FreeQApplication(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplicationD0Ev()};
     unsafe {_ZN12QApplicationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setDoubleClickInterval<RetType, T: QApplication_setDoubleClickInterval<RetType>>(&mut self, value: T) -> RetType {
    return value.setDoubleClickInterval(self);
    // return 1;
  }
}

pub trait QApplication_setDoubleClickInterval<RetType> {
  fn setDoubleClickInterval(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::setDoubleClickInterval(int );
impl<'a> /*trait*/ QApplication_setDoubleClickInterval<()> for (i32) {
  fn setDoubleClickInterval(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication22setDoubleClickIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication22setDoubleClickIntervalEi(arg0)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setGlobalStrut<RetType, T: QApplication_setGlobalStrut<RetType>>(&mut self, value: T) -> RetType {
    return value.setGlobalStrut(self);
    // return 1;
  }
}

pub trait QApplication_setGlobalStrut<RetType> {
  fn setGlobalStrut(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::setGlobalStrut(const QSize & );
impl<'a> /*trait*/ QApplication_setGlobalStrut<()> for (&'a  QSize) {
  fn setGlobalStrut(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication14setGlobalStrutERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QApplication14setGlobalStrutERK5QSize(arg0)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setColorSpec<RetType, T: QApplication_setColorSpec<RetType>>(&mut self, value: T) -> RetType {
    return value.setColorSpec(self);
    // return 1;
  }
}

pub trait QApplication_setColorSpec<RetType> {
  fn setColorSpec(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::setColorSpec(int );
impl<'a> /*trait*/ QApplication_setColorSpec<()> for (i32) {
  fn setColorSpec(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication12setColorSpecEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication12setColorSpecEi(arg0)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn allWidgets<RetType, T: QApplication_allWidgets<RetType>>(&mut self, value: T) -> RetType {
    return value.allWidgets(self);
    // return 1;
  }
}

pub trait QApplication_allWidgets<RetType> {
  fn allWidgets(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QList<QWidget *> QApplication::allWidgets();
impl<'a> /*trait*/ QApplication_allWidgets<()> for () {
  fn allWidgets(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10allWidgetsEv()};
     unsafe {_ZN12QApplication10allWidgetsEv()};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn globalStrut<RetType, T: QApplication_globalStrut<RetType>>(&mut self, value: T) -> RetType {
    return value.globalStrut(self);
    // return 1;
  }
}

pub trait QApplication_globalStrut<RetType> {
  fn globalStrut(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QSize QApplication::globalStrut();
impl<'a> /*trait*/ QApplication_globalStrut<QSize> for () {
  fn globalStrut(self, rsthis: &mut QApplication) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication11globalStrutEv()};
    let mut ret = unsafe {_ZN12QApplication11globalStrutEv()};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setPalette<RetType, T: QApplication_setPalette<RetType>>(&mut self, value: T) -> RetType {
    return value.setPalette(self);
    // return 1;
  }
}

pub trait QApplication_setPalette<RetType> {
  fn setPalette(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::setPalette(const QPalette & , const char * className);
impl<'a> /*trait*/ QApplication_setPalette<()> for (&'a  QPalette, &'a  String) {
  fn setPalette(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10setPaletteERK8QPalettePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
     unsafe {_ZN12QApplication10setPaletteERK8QPalettePKc(arg0, arg1)};
    // return 1;
  }
}

// proto: static QStyle * QApplication::setStyle(const QString & );
impl<'a> /*trait*/ QApplication_setStyle<QStyle> for (&'a  QString) {
  fn setStyle(self, rsthis: &mut QApplication) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8setStyleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QApplication8setStyleERK7QString(arg0)};
    let mut ret1 = QStyle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn topLevelWidgets<RetType, T: QApplication_topLevelWidgets<RetType>>(&mut self, value: T) -> RetType {
    return value.topLevelWidgets(self);
    // return 1;
  }
}

pub trait QApplication_topLevelWidgets<RetType> {
  fn topLevelWidgets(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QList<QWidget *> QApplication::topLevelWidgets();
impl<'a> /*trait*/ QApplication_topLevelWidgets<()> for () {
  fn topLevelWidgets(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15topLevelWidgetsEv()};
     unsafe {_ZN12QApplication15topLevelWidgetsEv()};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn exec<RetType, T: QApplication_exec<RetType>>(&mut self, value: T) -> RetType {
    return value.exec(self);
    // return 1;
  }
}

pub trait QApplication_exec<RetType> {
  fn exec(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static int QApplication::exec();
impl<'a> /*trait*/ QApplication_exec<i32> for () {
  fn exec(self, rsthis: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4execEv()};
    let mut ret = unsafe {_ZN12QApplication4execEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setWindowIcon<RetType, T: QApplication_setWindowIcon<RetType>>(&mut self, value: T) -> RetType {
    return value.setWindowIcon(self);
    // return 1;
  }
}

pub trait QApplication_setWindowIcon<RetType> {
  fn setWindowIcon(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::setWindowIcon(const QIcon & icon);
impl<'a> /*trait*/ QApplication_setWindowIcon<()> for (&'a  QIcon) {
  fn setWindowIcon(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication13setWindowIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QApplication13setWindowIconERK5QIcon(arg0)};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn beep<RetType, T: QApplication_beep<RetType>>(&mut self, value: T) -> RetType {
    return value.beep(self);
    // return 1;
  }
}

pub trait QApplication_beep<RetType> {
  fn beep(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static void QApplication::beep();
impl<'a> /*trait*/ QApplication_beep<()> for () {
  fn beep(self, rsthis: &mut QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4beepEv()};
     unsafe {_ZN12QApplication4beepEv()};
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn notify<RetType, T: QApplication_notify<RetType>>(&mut self, value: T) -> RetType {
    return value.notify(self);
    // return 1;
  }
}

pub trait QApplication_notify<RetType> {
  fn notify(self, rsthis: &mut QApplication) -> RetType;
}

// proto:  bool QApplication::notify(QObject * , QEvent * );
impl<'a> /*trait*/ QApplication_notify<i8> for (&'a mut QObject, &'a mut QEvent) {
  fn notify(self, rsthis: &mut QApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication6notifyEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QApplication6notifyEP7QObjectP6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn autoSipEnabled<RetType, T: QApplication_autoSipEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.autoSipEnabled(self);
    // return 1;
  }
}

pub trait QApplication_autoSipEnabled<RetType> {
  fn autoSipEnabled(self, rsthis: &mut QApplication) -> RetType;
}

// proto:  bool QApplication::autoSipEnabled();
impl<'a> /*trait*/ QApplication_autoSipEnabled<i8> for () {
  fn autoSipEnabled(self, rsthis: &mut QApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QApplication14autoSipEnabledEv()};
    let mut ret = unsafe {_ZNK12QApplication14autoSipEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: static QWidget * QApplication::topLevelAt(const QPoint & p);
impl<'a> /*trait*/ QApplication_topLevelAt<QWidget> for (&'a  QPoint) {
  fn topLevelAt(self, rsthis: &mut QApplication) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10topLevelAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QApplication10topLevelAtERK6QPoint(arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn startDragTime<RetType, T: QApplication_startDragTime<RetType>>(&mut self, value: T) -> RetType {
    return value.startDragTime(self);
    // return 1;
  }
}

pub trait QApplication_startDragTime<RetType> {
  fn startDragTime(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static int QApplication::startDragTime();
impl<'a> /*trait*/ QApplication_startDragTime<i32> for () {
  fn startDragTime(self, rsthis: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication13startDragTimeEv()};
    let mut ret = unsafe {_ZN12QApplication13startDragTimeEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn doubleClickInterval<RetType, T: QApplication_doubleClickInterval<RetType>>(&mut self, value: T) -> RetType {
    return value.doubleClickInterval(self);
    // return 1;
  }
}

pub trait QApplication_doubleClickInterval<RetType> {
  fn doubleClickInterval(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static int QApplication::doubleClickInterval();
impl<'a> /*trait*/ QApplication_doubleClickInterval<i32> for () {
  fn doubleClickInterval(self, rsthis: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication19doubleClickIntervalEv()};
    let mut ret = unsafe {_ZN12QApplication19doubleClickIntervalEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn windowIcon<RetType, T: QApplication_windowIcon<RetType>>(&mut self, value: T) -> RetType {
    return value.windowIcon(self);
    // return 1;
  }
}

pub trait QApplication_windowIcon<RetType> {
  fn windowIcon(self, rsthis: &mut QApplication) -> RetType;
}

// proto: static QIcon QApplication::windowIcon();
impl<'a> /*trait*/ QApplication_windowIcon<QIcon> for () {
  fn windowIcon(self, rsthis: &mut QApplication) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10windowIconEv()};
    let mut ret = unsafe {_ZN12QApplication10windowIconEv()};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

