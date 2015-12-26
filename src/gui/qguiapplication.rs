// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtGui/qguiapplication.h
// dst-file: /src/gui/qguiapplication.rs
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
use super::super::core::qcoreapplication::QCoreApplication; // 771
use std::ops::Deref;
use super::qfont::QFont; // 773
use super::super::core::qstring::QString; // 771
use super::qpalette::QPalette; // 773
use super::qinputmethod::QInputMethod; // 773
use super::qscreen::QScreen; // 773
use super::qcursor::QCursor; // 773
use super::qicon::QIcon; // 773
use super::qstylehints::QStyleHints; // 773
use super::qclipboard::QClipboard; // 773
use super::super::core::qobject::QObject; // 771
use super::super::core::qcoreevent::QEvent; // 771
use super::qsessionmanager::QSessionManager; // 773
use super::qwindow::QWindow; // 773
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qbytearray::QByteArray; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGuiApplication_Class_Size() -> c_int;
  // proto:  void QGuiApplication::~QGuiApplication();
  fn _ZN15QGuiApplicationD0Ev(qthis: *mut c_void);
  // proto: static void QGuiApplication::setFont(const QFont & );
  fn _ZN15QGuiApplication7setFontERK5QFont(arg0: *mut c_void);
  // proto: static QString QGuiApplication::platformName();
  fn _ZN15QGuiApplication12platformNameEv() -> *mut c_void;
  // proto: static QList<QScreen *> QGuiApplication::screens();
  fn _ZN15QGuiApplication7screensEv();
  // proto: static void QGuiApplication::setPalette(const QPalette & pal);
  fn _ZN15QGuiApplication10setPaletteERK8QPalette(arg0: *mut c_void);
  // proto: static QInputMethod * QGuiApplication::inputMethod();
  fn _ZN15QGuiApplication11inputMethodEv() -> *mut c_void;
  // proto:  void QGuiApplication::QGuiApplication(const QGuiApplication & );
  fn dector_ZN15QGuiApplicationC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QGuiApplicationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QGuiApplication::isSavingSession();
  fn _ZNK15QGuiApplication15isSavingSessionEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGuiApplication::paletteChanged(const QPalette & pal);
  fn _ZN15QGuiApplication14paletteChangedERK8QPalette(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QFont QGuiApplication::font();
  fn _ZN15QGuiApplication4fontEv() -> *mut c_void;
  // proto:  void QGuiApplication::screenAdded(QScreen * screen);
  fn _ZN15QGuiApplication11screenAddedEP7QScreen(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QGuiApplication::isSessionRestored();
  fn _ZNK15QGuiApplication17isSessionRestoredEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QGuiApplication::sessionKey();
  fn _ZNK15QGuiApplication10sessionKeyEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static bool QGuiApplication::desktopSettingsAware();
  fn _ZN15QGuiApplication20desktopSettingsAwareEv() -> c_char;
  // proto: static void QGuiApplication::sync();
  fn _ZN15QGuiApplication4syncEv();
  // proto: static void QGuiApplication::setQuitOnLastWindowClosed(bool quit);
  fn _ZN15QGuiApplication25setQuitOnLastWindowClosedEb(arg0: c_char);
  // proto: static QScreen * QGuiApplication::primaryScreen();
  fn _ZN15QGuiApplication13primaryScreenEv() -> *mut c_void;
  // proto: static QCursor * QGuiApplication::overrideCursor();
  fn _ZN15QGuiApplication14overrideCursorEv() -> *mut c_void;
  // proto: static QIcon QGuiApplication::windowIcon();
  fn _ZN15QGuiApplication10windowIconEv() -> *mut c_void;
  // proto: static QStyleHints * QGuiApplication::styleHints();
  fn _ZN15QGuiApplication10styleHintsEv() -> *mut c_void;
  // proto: static QClipboard * QGuiApplication::clipboard();
  fn _ZN15QGuiApplication9clipboardEv() -> *mut c_void;
  // proto: static QPalette QGuiApplication::palette();
  fn _ZN15QGuiApplication7paletteEv() -> *mut c_void;
  // proto:  bool QGuiApplication::notify(QObject * , QEvent * );
  fn _ZN15QGuiApplication6notifyEP7QObjectP6QEvent(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static QWindowList QGuiApplication::topLevelWindows();
  fn _ZN15QGuiApplication15topLevelWindowsEv();
  // proto: static bool QGuiApplication::isRightToLeft();
  fn demth_ZN15QGuiApplication13isRightToLeftEv() -> c_char;
  // proto:  void QGuiApplication::focusObjectChanged(QObject * focusObject);
  fn _ZN15QGuiApplication18focusObjectChangedEP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGuiApplication::fontDatabaseChanged();
  fn _ZN15QGuiApplication19fontDatabaseChangedEv(qthis: *mut c_void);
  // proto: static void QGuiApplication::changeOverrideCursor(const QCursor & );
  fn _ZN15QGuiApplication20changeOverrideCursorERK7QCursor(arg0: *mut c_void);
  // proto: static QWindowList QGuiApplication::allWindows();
  fn _ZN15QGuiApplication10allWindowsEv();
  // proto: static void QGuiApplication::setOverrideCursor(const QCursor & );
  fn _ZN15QGuiApplication17setOverrideCursorERK7QCursor(arg0: *mut c_void);
  // proto:  void QGuiApplication::commitDataRequest(QSessionManager & sessionManager);
  fn _ZN15QGuiApplication17commitDataRequestER15QSessionManager(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static void QGuiApplication::setWindowIcon(const QIcon & icon);
  fn _ZN15QGuiApplication13setWindowIconERK5QIcon(arg0: *mut c_void);
  // proto:  QString QGuiApplication::sessionId();
  fn _ZNK15QGuiApplication9sessionIdEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGuiApplication::focusWindowChanged(QWindow * focusWindow);
  fn _ZN15QGuiApplication18focusWindowChangedEP7QWindow(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static void QGuiApplication::setApplicationDisplayName(const QString & name);
  fn _ZN15QGuiApplication25setApplicationDisplayNameERK7QString(arg0: *mut c_void);
  // proto: static bool QGuiApplication::isLeftToRight();
  fn demth_ZN15QGuiApplication13isLeftToRightEv() -> c_char;
  // proto: static QWindow * QGuiApplication::topLevelAt(const QPoint & pos);
  fn _ZN15QGuiApplication10topLevelAtERK6QPoint(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGuiApplication::QGuiApplication(int & argc, char ** argv, int );
  fn dector_ZN15QGuiApplicationC1ERiPPci(arg0: *mut c_int, arg1: *mut c_char, arg2: c_int) -> *mut c_void;
  fn _ZN15QGuiApplicationC1ERiPPci(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_char, arg2: c_int);
  // proto: static void QGuiApplication::setDesktopSettingsAware(bool on);
  fn _ZN15QGuiApplication23setDesktopSettingsAwareEb(arg0: c_char);
  // proto: static QWindow * QGuiApplication::modalWindow();
  fn _ZN15QGuiApplication11modalWindowEv() -> *mut c_void;
  // proto: static QString QGuiApplication::applicationDisplayName();
  fn _ZN15QGuiApplication22applicationDisplayNameEv() -> *mut c_void;
  // proto: static int QGuiApplication::exec();
  fn _ZN15QGuiApplication4execEv() -> c_int;
  // proto: static bool QGuiApplication::quitOnLastWindowClosed();
  fn _ZN15QGuiApplication22quitOnLastWindowClosedEv() -> c_char;
  // proto:  void QGuiApplication::lastWindowClosed();
  fn _ZN15QGuiApplication16lastWindowClosedEv(qthis: *mut c_void);
  // proto: static void QGuiApplication::restoreOverrideCursor();
  fn _ZN15QGuiApplication21restoreOverrideCursorEv();
  // proto: static QPlatformNativeInterface * QGuiApplication::platformNativeInterface();
  fn _ZN15QGuiApplication23platformNativeInterfaceEv();
  // proto:  const QMetaObject * QGuiApplication::metaObject();
  fn _ZNK15QGuiApplication10metaObjectEv(qthis: *mut c_void);
  // proto: static QObject * QGuiApplication::focusObject();
  fn _ZN15QGuiApplication11focusObjectEv() -> *mut c_void;
  // proto:  void QGuiApplication::screenRemoved(QScreen * screen);
  fn _ZN15QGuiApplication13screenRemovedEP7QScreen(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QWindow * QGuiApplication::focusWindow();
  fn _ZN15QGuiApplication11focusWindowEv() -> *mut c_void;
  // proto:  void QGuiApplication::saveStateRequest(QSessionManager & sessionManager);
  fn _ZN15QGuiApplication16saveStateRequestER15QSessionManager(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QGuiApplication::devicePixelRatio();
  fn _ZNK15QGuiApplication16devicePixelRatioEv(qthis: *mut c_void) -> c_double;
  // proto: static QFunctionPointer QGuiApplication::platformFunction(const QByteArray & function);
  fn _ZN15QGuiApplication16platformFunctionERK10QByteArray(arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QGuiApplication)=1
pub struct QGuiApplication {
  qbase: QCoreApplication,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGuiApplication {
  pub fn inheritFrom(qthis: *mut c_void) -> QGuiApplication {
    return QGuiApplication{qbase: QCoreApplication::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QGuiApplication {
  type Target = QCoreApplication;

  fn deref(&self) -> &QCoreApplication {
    return & self.qbase;
  }
}
impl AsRef<QCoreApplication> for QGuiApplication {
  fn as_ref(& self) -> & QCoreApplication {
    return & self.qbase;
  }
}
  // proto:  void QGuiApplication::~QGuiApplication();
impl /*struct*/ QGuiApplication {
  pub fn Free<RetType, T: QGuiApplication_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGuiApplication_Free<RetType> {
  fn Free(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  void QGuiApplication::~QGuiApplication();
impl<'a> /*trait*/ QGuiApplication_Free<()> for () {
  fn Free(self , rsthis: & QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplicationD0Ev()};
     unsafe {_ZN15QGuiApplicationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QGuiApplication::setFont(const QFont & );
impl /*struct*/ QGuiApplication {
  pub fn setFont_s<RetType, T: QGuiApplication_setFont_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setFont_s();
    // return 1;
  }
}

pub trait QGuiApplication_setFont_s<RetType> {
  fn setFont_s(self ) -> RetType;
}

  // proto: static void QGuiApplication::setFont(const QFont & );
impl<'a> /*trait*/ QGuiApplication_setFont_s<()> for (&'a QFont) {
  fn setFont_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication7setFontERK5QFont(arg0)};
    // return 1;
  }
}

  // proto: static QString QGuiApplication::platformName();
impl /*struct*/ QGuiApplication {
  pub fn platformName_s<RetType, T: QGuiApplication_platformName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.platformName_s();
    // return 1;
  }
}

pub trait QGuiApplication_platformName_s<RetType> {
  fn platformName_s(self ) -> RetType;
}

  // proto: static QString QGuiApplication::platformName();
impl<'a> /*trait*/ QGuiApplication_platformName_s<QString> for () {
  fn platformName_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication12platformNameEv()};
    let mut ret = unsafe {_ZN15QGuiApplication12platformNameEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QList<QScreen *> QGuiApplication::screens();
impl /*struct*/ QGuiApplication {
  pub fn screens_s<RetType, T: QGuiApplication_screens_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.screens_s();
    // return 1;
  }
}

pub trait QGuiApplication_screens_s<RetType> {
  fn screens_s(self ) -> RetType;
}

  // proto: static QList<QScreen *> QGuiApplication::screens();
impl<'a> /*trait*/ QGuiApplication_screens_s<()> for () {
  fn screens_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication7screensEv()};
     unsafe {_ZN15QGuiApplication7screensEv()};
    // return 1;
  }
}

  // proto: static void QGuiApplication::setPalette(const QPalette & pal);
impl /*struct*/ QGuiApplication {
  pub fn setPalette_s<RetType, T: QGuiApplication_setPalette_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPalette_s();
    // return 1;
  }
}

pub trait QGuiApplication_setPalette_s<RetType> {
  fn setPalette_s(self ) -> RetType;
}

  // proto: static void QGuiApplication::setPalette(const QPalette & pal);
impl<'a> /*trait*/ QGuiApplication_setPalette_s<()> for (&'a QPalette) {
  fn setPalette_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication10setPaletteERK8QPalette(arg0)};
    // return 1;
  }
}

  // proto: static QInputMethod * QGuiApplication::inputMethod();
impl /*struct*/ QGuiApplication {
  pub fn inputMethod_s<RetType, T: QGuiApplication_inputMethod_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.inputMethod_s();
    // return 1;
  }
}

pub trait QGuiApplication_inputMethod_s<RetType> {
  fn inputMethod_s(self ) -> RetType;
}

  // proto: static QInputMethod * QGuiApplication::inputMethod();
impl<'a> /*trait*/ QGuiApplication_inputMethod_s<QInputMethod> for () {
  fn inputMethod_s(self ) -> QInputMethod {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11inputMethodEv()};
    let mut ret = unsafe {_ZN15QGuiApplication11inputMethodEv()};
    let mut ret1 = QInputMethod::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGuiApplication::QGuiApplication(const QGuiApplication & );
impl /*struct*/ QGuiApplication {
  pub fn New<T: QGuiApplication_New>(value: T) -> QGuiApplication {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGuiApplication_New {
  fn New(self) -> QGuiApplication;
}

  // proto:  void QGuiApplication::QGuiApplication(const QGuiApplication & );
impl<'a> /*trait*/ QGuiApplication_New for (&'a QGuiApplication) {
  fn New(self) -> QGuiApplication {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplicationC1ERKS_()};
    let ctysz: c_int = unsafe{QGuiApplication_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QGuiApplicationC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN15QGuiApplicationC1ERKS_(arg0)};
    let rsthis = QGuiApplication{/**/qbase: QCoreApplication::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QGuiApplication::isSavingSession();
impl /*struct*/ QGuiApplication {
  pub fn isSavingSession<RetType, T: QGuiApplication_isSavingSession<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSavingSession(self);
    // return 1;
  }
}

pub trait QGuiApplication_isSavingSession<RetType> {
  fn isSavingSession(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  bool QGuiApplication::isSavingSession();
impl<'a> /*trait*/ QGuiApplication_isSavingSession<i8> for () {
  fn isSavingSession(self , rsthis: & QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication15isSavingSessionEv()};
    let mut ret = unsafe {_ZNK15QGuiApplication15isSavingSessionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGuiApplication::paletteChanged(const QPalette & pal);
impl /*struct*/ QGuiApplication {
  pub fn paletteChanged<RetType, T: QGuiApplication_paletteChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paletteChanged(self);
    // return 1;
  }
}

pub trait QGuiApplication_paletteChanged<RetType> {
  fn paletteChanged(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  void QGuiApplication::paletteChanged(const QPalette & pal);
impl<'a> /*trait*/ QGuiApplication_paletteChanged<()> for (&'a QPalette) {
  fn paletteChanged(self , rsthis: & QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication14paletteChangedERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication14paletteChangedERK8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QFont QGuiApplication::font();
impl /*struct*/ QGuiApplication {
  pub fn font_s<RetType, T: QGuiApplication_font_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.font_s();
    // return 1;
  }
}

pub trait QGuiApplication_font_s<RetType> {
  fn font_s(self ) -> RetType;
}

  // proto: static QFont QGuiApplication::font();
impl<'a> /*trait*/ QGuiApplication_font_s<QFont> for () {
  fn font_s(self ) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication4fontEv()};
    let mut ret = unsafe {_ZN15QGuiApplication4fontEv()};
    let mut ret1 = QFont::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGuiApplication::screenAdded(QScreen * screen);
impl /*struct*/ QGuiApplication {
  pub fn screenAdded<RetType, T: QGuiApplication_screenAdded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenAdded(self);
    // return 1;
  }
}

pub trait QGuiApplication_screenAdded<RetType> {
  fn screenAdded(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  void QGuiApplication::screenAdded(QScreen * screen);
impl<'a> /*trait*/ QGuiApplication_screenAdded<()> for (&'a QScreen) {
  fn screenAdded(self , rsthis: & QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11screenAddedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication11screenAddedEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGuiApplication::isSessionRestored();
impl /*struct*/ QGuiApplication {
  pub fn isSessionRestored<RetType, T: QGuiApplication_isSessionRestored<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSessionRestored(self);
    // return 1;
  }
}

pub trait QGuiApplication_isSessionRestored<RetType> {
  fn isSessionRestored(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  bool QGuiApplication::isSessionRestored();
impl<'a> /*trait*/ QGuiApplication_isSessionRestored<i8> for () {
  fn isSessionRestored(self , rsthis: & QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication17isSessionRestoredEv()};
    let mut ret = unsafe {_ZNK15QGuiApplication17isSessionRestoredEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QGuiApplication::sessionKey();
impl /*struct*/ QGuiApplication {
  pub fn sessionKey<RetType, T: QGuiApplication_sessionKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sessionKey(self);
    // return 1;
  }
}

pub trait QGuiApplication_sessionKey<RetType> {
  fn sessionKey(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  QString QGuiApplication::sessionKey();
impl<'a> /*trait*/ QGuiApplication_sessionKey<QString> for () {
  fn sessionKey(self , rsthis: & QGuiApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication10sessionKeyEv()};
    let mut ret = unsafe {_ZNK15QGuiApplication10sessionKeyEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QGuiApplication::desktopSettingsAware();
impl /*struct*/ QGuiApplication {
  pub fn desktopSettingsAware_s<RetType, T: QGuiApplication_desktopSettingsAware_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.desktopSettingsAware_s();
    // return 1;
  }
}

pub trait QGuiApplication_desktopSettingsAware_s<RetType> {
  fn desktopSettingsAware_s(self ) -> RetType;
}

  // proto: static bool QGuiApplication::desktopSettingsAware();
impl<'a> /*trait*/ QGuiApplication_desktopSettingsAware_s<i8> for () {
  fn desktopSettingsAware_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication20desktopSettingsAwareEv()};
    let mut ret = unsafe {_ZN15QGuiApplication20desktopSettingsAwareEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void QGuiApplication::sync();
impl /*struct*/ QGuiApplication {
  pub fn sync_s<RetType, T: QGuiApplication_sync_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sync_s();
    // return 1;
  }
}

pub trait QGuiApplication_sync_s<RetType> {
  fn sync_s(self ) -> RetType;
}

  // proto: static void QGuiApplication::sync();
impl<'a> /*trait*/ QGuiApplication_sync_s<()> for () {
  fn sync_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication4syncEv()};
     unsafe {_ZN15QGuiApplication4syncEv()};
    // return 1;
  }
}

  // proto: static void QGuiApplication::setQuitOnLastWindowClosed(bool quit);
impl /*struct*/ QGuiApplication {
  pub fn setQuitOnLastWindowClosed_s<RetType, T: QGuiApplication_setQuitOnLastWindowClosed_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setQuitOnLastWindowClosed_s();
    // return 1;
  }
}

pub trait QGuiApplication_setQuitOnLastWindowClosed_s<RetType> {
  fn setQuitOnLastWindowClosed_s(self ) -> RetType;
}

  // proto: static void QGuiApplication::setQuitOnLastWindowClosed(bool quit);
impl<'a> /*trait*/ QGuiApplication_setQuitOnLastWindowClosed_s<()> for (i8) {
  fn setQuitOnLastWindowClosed_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication25setQuitOnLastWindowClosedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QGuiApplication25setQuitOnLastWindowClosedEb(arg0)};
    // return 1;
  }
}

  // proto: static QScreen * QGuiApplication::primaryScreen();
impl /*struct*/ QGuiApplication {
  pub fn primaryScreen_s<RetType, T: QGuiApplication_primaryScreen_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.primaryScreen_s();
    // return 1;
  }
}

pub trait QGuiApplication_primaryScreen_s<RetType> {
  fn primaryScreen_s(self ) -> RetType;
}

  // proto: static QScreen * QGuiApplication::primaryScreen();
impl<'a> /*trait*/ QGuiApplication_primaryScreen_s<QScreen> for () {
  fn primaryScreen_s(self ) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13primaryScreenEv()};
    let mut ret = unsafe {_ZN15QGuiApplication13primaryScreenEv()};
    let mut ret1 = QScreen::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QCursor * QGuiApplication::overrideCursor();
impl /*struct*/ QGuiApplication {
  pub fn overrideCursor_s<RetType, T: QGuiApplication_overrideCursor_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.overrideCursor_s();
    // return 1;
  }
}

pub trait QGuiApplication_overrideCursor_s<RetType> {
  fn overrideCursor_s(self ) -> RetType;
}

  // proto: static QCursor * QGuiApplication::overrideCursor();
impl<'a> /*trait*/ QGuiApplication_overrideCursor_s<QCursor> for () {
  fn overrideCursor_s(self ) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication14overrideCursorEv()};
    let mut ret = unsafe {_ZN15QGuiApplication14overrideCursorEv()};
    let mut ret1 = QCursor::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QIcon QGuiApplication::windowIcon();
impl /*struct*/ QGuiApplication {
  pub fn windowIcon_s<RetType, T: QGuiApplication_windowIcon_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.windowIcon_s();
    // return 1;
  }
}

pub trait QGuiApplication_windowIcon_s<RetType> {
  fn windowIcon_s(self ) -> RetType;
}

  // proto: static QIcon QGuiApplication::windowIcon();
impl<'a> /*trait*/ QGuiApplication_windowIcon_s<QIcon> for () {
  fn windowIcon_s(self ) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10windowIconEv()};
    let mut ret = unsafe {_ZN15QGuiApplication10windowIconEv()};
    let mut ret1 = QIcon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QStyleHints * QGuiApplication::styleHints();
impl /*struct*/ QGuiApplication {
  pub fn styleHints_s<RetType, T: QGuiApplication_styleHints_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.styleHints_s();
    // return 1;
  }
}

pub trait QGuiApplication_styleHints_s<RetType> {
  fn styleHints_s(self ) -> RetType;
}

  // proto: static QStyleHints * QGuiApplication::styleHints();
impl<'a> /*trait*/ QGuiApplication_styleHints_s<QStyleHints> for () {
  fn styleHints_s(self ) -> QStyleHints {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10styleHintsEv()};
    let mut ret = unsafe {_ZN15QGuiApplication10styleHintsEv()};
    let mut ret1 = QStyleHints::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QClipboard * QGuiApplication::clipboard();
impl /*struct*/ QGuiApplication {
  pub fn clipboard_s<RetType, T: QGuiApplication_clipboard_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.clipboard_s();
    // return 1;
  }
}

pub trait QGuiApplication_clipboard_s<RetType> {
  fn clipboard_s(self ) -> RetType;
}

  // proto: static QClipboard * QGuiApplication::clipboard();
impl<'a> /*trait*/ QGuiApplication_clipboard_s<QClipboard> for () {
  fn clipboard_s(self ) -> QClipboard {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication9clipboardEv()};
    let mut ret = unsafe {_ZN15QGuiApplication9clipboardEv()};
    let mut ret1 = QClipboard::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QPalette QGuiApplication::palette();
impl /*struct*/ QGuiApplication {
  pub fn palette_s<RetType, T: QGuiApplication_palette_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.palette_s();
    // return 1;
  }
}

pub trait QGuiApplication_palette_s<RetType> {
  fn palette_s(self ) -> RetType;
}

  // proto: static QPalette QGuiApplication::palette();
impl<'a> /*trait*/ QGuiApplication_palette_s<QPalette> for () {
  fn palette_s(self ) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication7paletteEv()};
    let mut ret = unsafe {_ZN15QGuiApplication7paletteEv()};
    let mut ret1 = QPalette::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGuiApplication::notify(QObject * , QEvent * );
impl /*struct*/ QGuiApplication {
  pub fn notify<RetType, T: QGuiApplication_notify<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.notify(self);
    // return 1;
  }
}

pub trait QGuiApplication_notify<RetType> {
  fn notify(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  bool QGuiApplication::notify(QObject * , QEvent * );
impl<'a> /*trait*/ QGuiApplication_notify<i8> for (&'a QObject, &'a QEvent) {
  fn notify(self , rsthis: & QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication6notifyEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN15QGuiApplication6notifyEP7QObjectP6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QWindowList QGuiApplication::topLevelWindows();
impl /*struct*/ QGuiApplication {
  pub fn topLevelWindows_s<RetType, T: QGuiApplication_topLevelWindows_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.topLevelWindows_s();
    // return 1;
  }
}

pub trait QGuiApplication_topLevelWindows_s<RetType> {
  fn topLevelWindows_s(self ) -> RetType;
}

  // proto: static QWindowList QGuiApplication::topLevelWindows();
impl<'a> /*trait*/ QGuiApplication_topLevelWindows_s<()> for () {
  fn topLevelWindows_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication15topLevelWindowsEv()};
     unsafe {_ZN15QGuiApplication15topLevelWindowsEv()};
    // return 1;
  }
}

  // proto: static bool QGuiApplication::isRightToLeft();
impl /*struct*/ QGuiApplication {
  pub fn isRightToLeft_s<RetType, T: QGuiApplication_isRightToLeft_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isRightToLeft_s();
    // return 1;
  }
}

pub trait QGuiApplication_isRightToLeft_s<RetType> {
  fn isRightToLeft_s(self ) -> RetType;
}

  // proto: static bool QGuiApplication::isRightToLeft();
impl<'a> /*trait*/ QGuiApplication_isRightToLeft_s<i8> for () {
  fn isRightToLeft_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13isRightToLeftEv()};
    let mut ret = unsafe {demth_ZN15QGuiApplication13isRightToLeftEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGuiApplication::focusObjectChanged(QObject * focusObject);
impl /*struct*/ QGuiApplication {
  pub fn focusObjectChanged<RetType, T: QGuiApplication_focusObjectChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusObjectChanged(self);
    // return 1;
  }
}

pub trait QGuiApplication_focusObjectChanged<RetType> {
  fn focusObjectChanged(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  void QGuiApplication::focusObjectChanged(QObject * focusObject);
impl<'a> /*trait*/ QGuiApplication_focusObjectChanged<()> for (&'a QObject) {
  fn focusObjectChanged(self , rsthis: & QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication18focusObjectChangedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication18focusObjectChangedEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGuiApplication::fontDatabaseChanged();
impl /*struct*/ QGuiApplication {
  pub fn fontDatabaseChanged<RetType, T: QGuiApplication_fontDatabaseChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontDatabaseChanged(self);
    // return 1;
  }
}

pub trait QGuiApplication_fontDatabaseChanged<RetType> {
  fn fontDatabaseChanged(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  void QGuiApplication::fontDatabaseChanged();
impl<'a> /*trait*/ QGuiApplication_fontDatabaseChanged<()> for () {
  fn fontDatabaseChanged(self , rsthis: & QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication19fontDatabaseChangedEv()};
     unsafe {_ZN15QGuiApplication19fontDatabaseChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QGuiApplication::changeOverrideCursor(const QCursor & );
impl /*struct*/ QGuiApplication {
  pub fn changeOverrideCursor_s<RetType, T: QGuiApplication_changeOverrideCursor_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.changeOverrideCursor_s();
    // return 1;
  }
}

pub trait QGuiApplication_changeOverrideCursor_s<RetType> {
  fn changeOverrideCursor_s(self ) -> RetType;
}

  // proto: static void QGuiApplication::changeOverrideCursor(const QCursor & );
impl<'a> /*trait*/ QGuiApplication_changeOverrideCursor_s<()> for (&'a QCursor) {
  fn changeOverrideCursor_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication20changeOverrideCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication20changeOverrideCursorERK7QCursor(arg0)};
    // return 1;
  }
}

  // proto: static QWindowList QGuiApplication::allWindows();
impl /*struct*/ QGuiApplication {
  pub fn allWindows_s<RetType, T: QGuiApplication_allWindows_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.allWindows_s();
    // return 1;
  }
}

pub trait QGuiApplication_allWindows_s<RetType> {
  fn allWindows_s(self ) -> RetType;
}

  // proto: static QWindowList QGuiApplication::allWindows();
impl<'a> /*trait*/ QGuiApplication_allWindows_s<()> for () {
  fn allWindows_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10allWindowsEv()};
     unsafe {_ZN15QGuiApplication10allWindowsEv()};
    // return 1;
  }
}

  // proto: static void QGuiApplication::setOverrideCursor(const QCursor & );
impl /*struct*/ QGuiApplication {
  pub fn setOverrideCursor_s<RetType, T: QGuiApplication_setOverrideCursor_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setOverrideCursor_s();
    // return 1;
  }
}

pub trait QGuiApplication_setOverrideCursor_s<RetType> {
  fn setOverrideCursor_s(self ) -> RetType;
}

  // proto: static void QGuiApplication::setOverrideCursor(const QCursor & );
impl<'a> /*trait*/ QGuiApplication_setOverrideCursor_s<()> for (&'a QCursor) {
  fn setOverrideCursor_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication17setOverrideCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication17setOverrideCursorERK7QCursor(arg0)};
    // return 1;
  }
}

  // proto:  void QGuiApplication::commitDataRequest(QSessionManager & sessionManager);
impl /*struct*/ QGuiApplication {
  pub fn commitDataRequest<RetType, T: QGuiApplication_commitDataRequest<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.commitDataRequest(self);
    // return 1;
  }
}

pub trait QGuiApplication_commitDataRequest<RetType> {
  fn commitDataRequest(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  void QGuiApplication::commitDataRequest(QSessionManager & sessionManager);
impl<'a> /*trait*/ QGuiApplication_commitDataRequest<()> for (&'a QSessionManager) {
  fn commitDataRequest(self , rsthis: & QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication17commitDataRequestER15QSessionManager()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication17commitDataRequestER15QSessionManager(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static void QGuiApplication::setWindowIcon(const QIcon & icon);
impl /*struct*/ QGuiApplication {
  pub fn setWindowIcon_s<RetType, T: QGuiApplication_setWindowIcon_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setWindowIcon_s();
    // return 1;
  }
}

pub trait QGuiApplication_setWindowIcon_s<RetType> {
  fn setWindowIcon_s(self ) -> RetType;
}

  // proto: static void QGuiApplication::setWindowIcon(const QIcon & icon);
impl<'a> /*trait*/ QGuiApplication_setWindowIcon_s<()> for (&'a QIcon) {
  fn setWindowIcon_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13setWindowIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication13setWindowIconERK5QIcon(arg0)};
    // return 1;
  }
}

  // proto:  QString QGuiApplication::sessionId();
impl /*struct*/ QGuiApplication {
  pub fn sessionId<RetType, T: QGuiApplication_sessionId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sessionId(self);
    // return 1;
  }
}

pub trait QGuiApplication_sessionId<RetType> {
  fn sessionId(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  QString QGuiApplication::sessionId();
impl<'a> /*trait*/ QGuiApplication_sessionId<QString> for () {
  fn sessionId(self , rsthis: & QGuiApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication9sessionIdEv()};
    let mut ret = unsafe {_ZNK15QGuiApplication9sessionIdEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGuiApplication::focusWindowChanged(QWindow * focusWindow);
impl /*struct*/ QGuiApplication {
  pub fn focusWindowChanged<RetType, T: QGuiApplication_focusWindowChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusWindowChanged(self);
    // return 1;
  }
}

pub trait QGuiApplication_focusWindowChanged<RetType> {
  fn focusWindowChanged(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  void QGuiApplication::focusWindowChanged(QWindow * focusWindow);
impl<'a> /*trait*/ QGuiApplication_focusWindowChanged<()> for (&'a QWindow) {
  fn focusWindowChanged(self , rsthis: & QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication18focusWindowChangedEP7QWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication18focusWindowChangedEP7QWindow(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static void QGuiApplication::setApplicationDisplayName(const QString & name);
impl /*struct*/ QGuiApplication {
  pub fn setApplicationDisplayName_s<RetType, T: QGuiApplication_setApplicationDisplayName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setApplicationDisplayName_s();
    // return 1;
  }
}

pub trait QGuiApplication_setApplicationDisplayName_s<RetType> {
  fn setApplicationDisplayName_s(self ) -> RetType;
}

  // proto: static void QGuiApplication::setApplicationDisplayName(const QString & name);
impl<'a> /*trait*/ QGuiApplication_setApplicationDisplayName_s<()> for (&'a QString) {
  fn setApplicationDisplayName_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication25setApplicationDisplayNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication25setApplicationDisplayNameERK7QString(arg0)};
    // return 1;
  }
}

  // proto: static bool QGuiApplication::isLeftToRight();
impl /*struct*/ QGuiApplication {
  pub fn isLeftToRight_s<RetType, T: QGuiApplication_isLeftToRight_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLeftToRight_s();
    // return 1;
  }
}

pub trait QGuiApplication_isLeftToRight_s<RetType> {
  fn isLeftToRight_s(self ) -> RetType;
}

  // proto: static bool QGuiApplication::isLeftToRight();
impl<'a> /*trait*/ QGuiApplication_isLeftToRight_s<i8> for () {
  fn isLeftToRight_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13isLeftToRightEv()};
    let mut ret = unsafe {demth_ZN15QGuiApplication13isLeftToRightEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QWindow * QGuiApplication::topLevelAt(const QPoint & pos);
impl /*struct*/ QGuiApplication {
  pub fn topLevelAt_s<RetType, T: QGuiApplication_topLevelAt_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.topLevelAt_s();
    // return 1;
  }
}

pub trait QGuiApplication_topLevelAt_s<RetType> {
  fn topLevelAt_s(self ) -> RetType;
}

  // proto: static QWindow * QGuiApplication::topLevelAt(const QPoint & pos);
impl<'a> /*trait*/ QGuiApplication_topLevelAt_s<QWindow> for (&'a QPoint) {
  fn topLevelAt_s(self ) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10topLevelAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN15QGuiApplication10topLevelAtERK6QPoint(arg0)};
    let mut ret1 = QWindow::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGuiApplication::QGuiApplication(int & argc, char ** argv, int );
impl<'a> /*trait*/ QGuiApplication_New for (&'a mut i32, &'a mut String, i32) {
  fn New(self) -> QGuiApplication {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplicationC1ERiPPci()};
    let ctysz: c_int = unsafe{QGuiApplication_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
    // unsafe {_ZN15QGuiApplicationC1ERiPPci(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN15QGuiApplicationC1ERiPPci(arg0, arg1, arg2)};
    let rsthis = QGuiApplication{/**/qbase: QCoreApplication::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static void QGuiApplication::setDesktopSettingsAware(bool on);
impl /*struct*/ QGuiApplication {
  pub fn setDesktopSettingsAware_s<RetType, T: QGuiApplication_setDesktopSettingsAware_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDesktopSettingsAware_s();
    // return 1;
  }
}

pub trait QGuiApplication_setDesktopSettingsAware_s<RetType> {
  fn setDesktopSettingsAware_s(self ) -> RetType;
}

  // proto: static void QGuiApplication::setDesktopSettingsAware(bool on);
impl<'a> /*trait*/ QGuiApplication_setDesktopSettingsAware_s<()> for (i8) {
  fn setDesktopSettingsAware_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication23setDesktopSettingsAwareEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QGuiApplication23setDesktopSettingsAwareEb(arg0)};
    // return 1;
  }
}

  // proto: static QWindow * QGuiApplication::modalWindow();
impl /*struct*/ QGuiApplication {
  pub fn modalWindow_s<RetType, T: QGuiApplication_modalWindow_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.modalWindow_s();
    // return 1;
  }
}

pub trait QGuiApplication_modalWindow_s<RetType> {
  fn modalWindow_s(self ) -> RetType;
}

  // proto: static QWindow * QGuiApplication::modalWindow();
impl<'a> /*trait*/ QGuiApplication_modalWindow_s<QWindow> for () {
  fn modalWindow_s(self ) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11modalWindowEv()};
    let mut ret = unsafe {_ZN15QGuiApplication11modalWindowEv()};
    let mut ret1 = QWindow::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QGuiApplication::applicationDisplayName();
impl /*struct*/ QGuiApplication {
  pub fn applicationDisplayName_s<RetType, T: QGuiApplication_applicationDisplayName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationDisplayName_s();
    // return 1;
  }
}

pub trait QGuiApplication_applicationDisplayName_s<RetType> {
  fn applicationDisplayName_s(self ) -> RetType;
}

  // proto: static QString QGuiApplication::applicationDisplayName();
impl<'a> /*trait*/ QGuiApplication_applicationDisplayName_s<QString> for () {
  fn applicationDisplayName_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication22applicationDisplayNameEv()};
    let mut ret = unsafe {_ZN15QGuiApplication22applicationDisplayNameEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static int QGuiApplication::exec();
impl /*struct*/ QGuiApplication {
  pub fn exec_s<RetType, T: QGuiApplication_exec_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.exec_s();
    // return 1;
  }
}

pub trait QGuiApplication_exec_s<RetType> {
  fn exec_s(self ) -> RetType;
}

  // proto: static int QGuiApplication::exec();
impl<'a> /*trait*/ QGuiApplication_exec_s<i32> for () {
  fn exec_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication4execEv()};
    let mut ret = unsafe {_ZN15QGuiApplication4execEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto: static bool QGuiApplication::quitOnLastWindowClosed();
impl /*struct*/ QGuiApplication {
  pub fn quitOnLastWindowClosed_s<RetType, T: QGuiApplication_quitOnLastWindowClosed_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.quitOnLastWindowClosed_s();
    // return 1;
  }
}

pub trait QGuiApplication_quitOnLastWindowClosed_s<RetType> {
  fn quitOnLastWindowClosed_s(self ) -> RetType;
}

  // proto: static bool QGuiApplication::quitOnLastWindowClosed();
impl<'a> /*trait*/ QGuiApplication_quitOnLastWindowClosed_s<i8> for () {
  fn quitOnLastWindowClosed_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication22quitOnLastWindowClosedEv()};
    let mut ret = unsafe {_ZN15QGuiApplication22quitOnLastWindowClosedEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGuiApplication::lastWindowClosed();
impl /*struct*/ QGuiApplication {
  pub fn lastWindowClosed<RetType, T: QGuiApplication_lastWindowClosed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastWindowClosed(self);
    // return 1;
  }
}

pub trait QGuiApplication_lastWindowClosed<RetType> {
  fn lastWindowClosed(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  void QGuiApplication::lastWindowClosed();
impl<'a> /*trait*/ QGuiApplication_lastWindowClosed<()> for () {
  fn lastWindowClosed(self , rsthis: & QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication16lastWindowClosedEv()};
     unsafe {_ZN15QGuiApplication16lastWindowClosedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QGuiApplication::restoreOverrideCursor();
impl /*struct*/ QGuiApplication {
  pub fn restoreOverrideCursor_s<RetType, T: QGuiApplication_restoreOverrideCursor_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.restoreOverrideCursor_s();
    // return 1;
  }
}

pub trait QGuiApplication_restoreOverrideCursor_s<RetType> {
  fn restoreOverrideCursor_s(self ) -> RetType;
}

  // proto: static void QGuiApplication::restoreOverrideCursor();
impl<'a> /*trait*/ QGuiApplication_restoreOverrideCursor_s<()> for () {
  fn restoreOverrideCursor_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication21restoreOverrideCursorEv()};
     unsafe {_ZN15QGuiApplication21restoreOverrideCursorEv()};
    // return 1;
  }
}

  // proto: static QPlatformNativeInterface * QGuiApplication::platformNativeInterface();
impl /*struct*/ QGuiApplication {
  pub fn platformNativeInterface_s<RetType, T: QGuiApplication_platformNativeInterface_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.platformNativeInterface_s();
    // return 1;
  }
}

pub trait QGuiApplication_platformNativeInterface_s<RetType> {
  fn platformNativeInterface_s(self ) -> RetType;
}

  // proto: static QPlatformNativeInterface * QGuiApplication::platformNativeInterface();
impl<'a> /*trait*/ QGuiApplication_platformNativeInterface_s<()> for () {
  fn platformNativeInterface_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication23platformNativeInterfaceEv()};
     unsafe {_ZN15QGuiApplication23platformNativeInterfaceEv()};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGuiApplication::metaObject();
impl /*struct*/ QGuiApplication {
  pub fn metaObject<RetType, T: QGuiApplication_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGuiApplication_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  const QMetaObject * QGuiApplication::metaObject();
impl<'a> /*trait*/ QGuiApplication_metaObject<()> for () {
  fn metaObject(self , rsthis: & QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication10metaObjectEv()};
     unsafe {_ZNK15QGuiApplication10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QObject * QGuiApplication::focusObject();
impl /*struct*/ QGuiApplication {
  pub fn focusObject_s<RetType, T: QGuiApplication_focusObject_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.focusObject_s();
    // return 1;
  }
}

pub trait QGuiApplication_focusObject_s<RetType> {
  fn focusObject_s(self ) -> RetType;
}

  // proto: static QObject * QGuiApplication::focusObject();
impl<'a> /*trait*/ QGuiApplication_focusObject_s<QObject> for () {
  fn focusObject_s(self ) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11focusObjectEv()};
    let mut ret = unsafe {_ZN15QGuiApplication11focusObjectEv()};
    let mut ret1 = QObject::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGuiApplication::screenRemoved(QScreen * screen);
impl /*struct*/ QGuiApplication {
  pub fn screenRemoved<RetType, T: QGuiApplication_screenRemoved<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenRemoved(self);
    // return 1;
  }
}

pub trait QGuiApplication_screenRemoved<RetType> {
  fn screenRemoved(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  void QGuiApplication::screenRemoved(QScreen * screen);
impl<'a> /*trait*/ QGuiApplication_screenRemoved<()> for (&'a QScreen) {
  fn screenRemoved(self , rsthis: & QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13screenRemovedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication13screenRemovedEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QWindow * QGuiApplication::focusWindow();
impl /*struct*/ QGuiApplication {
  pub fn focusWindow_s<RetType, T: QGuiApplication_focusWindow_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.focusWindow_s();
    // return 1;
  }
}

pub trait QGuiApplication_focusWindow_s<RetType> {
  fn focusWindow_s(self ) -> RetType;
}

  // proto: static QWindow * QGuiApplication::focusWindow();
impl<'a> /*trait*/ QGuiApplication_focusWindow_s<QWindow> for () {
  fn focusWindow_s(self ) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11focusWindowEv()};
    let mut ret = unsafe {_ZN15QGuiApplication11focusWindowEv()};
    let mut ret1 = QWindow::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGuiApplication::saveStateRequest(QSessionManager & sessionManager);
impl /*struct*/ QGuiApplication {
  pub fn saveStateRequest<RetType, T: QGuiApplication_saveStateRequest<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.saveStateRequest(self);
    // return 1;
  }
}

pub trait QGuiApplication_saveStateRequest<RetType> {
  fn saveStateRequest(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  void QGuiApplication::saveStateRequest(QSessionManager & sessionManager);
impl<'a> /*trait*/ QGuiApplication_saveStateRequest<()> for (&'a QSessionManager) {
  fn saveStateRequest(self , rsthis: & QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication16saveStateRequestER15QSessionManager()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication16saveStateRequestER15QSessionManager(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGuiApplication::devicePixelRatio();
impl /*struct*/ QGuiApplication {
  pub fn devicePixelRatio<RetType, T: QGuiApplication_devicePixelRatio<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QGuiApplication_devicePixelRatio<RetType> {
  fn devicePixelRatio(self , rsthis: & QGuiApplication) -> RetType;
}

  // proto:  qreal QGuiApplication::devicePixelRatio();
impl<'a> /*trait*/ QGuiApplication_devicePixelRatio<f64> for () {
  fn devicePixelRatio(self , rsthis: & QGuiApplication) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK15QGuiApplication16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto: static QFunctionPointer QGuiApplication::platformFunction(const QByteArray & function);
impl /*struct*/ QGuiApplication {
  pub fn platformFunction_s<RetType, T: QGuiApplication_platformFunction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.platformFunction_s();
    // return 1;
  }
}

pub trait QGuiApplication_platformFunction_s<RetType> {
  fn platformFunction_s(self ) -> RetType;
}

  // proto: static QFunctionPointer QGuiApplication::platformFunction(const QByteArray & function);
impl<'a> /*trait*/ QGuiApplication_platformFunction_s<()> for (&'a QByteArray) {
  fn platformFunction_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication16platformFunctionERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication16platformFunctionERK10QByteArray(arg0)};
    // return 1;
  }
}

// <= body block end

