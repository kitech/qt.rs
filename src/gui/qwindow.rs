// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtGui/qwindow.h
// dst-file: /src/gui/qwindow.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::qscreen::QScreen; // 773
use super::super::core::qsize::QSize; // 771
use super::qsurfaceformat::QSurfaceFormat; // 773
use super::qicon::QIcon; // 773
use super::super::core::qstring::QString; // 771
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qmargins::QMargins; // 771
use super::super::core::qrect::QRect; // 771
use super::qcursor::QCursor; // 773
use super::qregion::QRegion; // 773
use super::qaccessible::QAccessibleInterface; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QWindow_Class_Size() -> c_int;
  // proto:  void QWindow::xChanged(int arg);
  fn _ZN7QWindow8xChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWindow::unsetCursor();
  fn _ZN7QWindow11unsetCursorEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QWindow::isVisible();
  fn _ZNK7QWindow9isVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWindow::setScreen(QScreen * screen);
  fn _ZN7QWindow9setScreenEP7QScreen(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSize QWindow::maximumSize();
  fn _ZNK7QWindow11maximumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::setTransientParent(QWindow * parent);
  fn _ZN7QWindow18setTransientParentEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSurfaceFormat QWindow::format();
  fn _ZNK7QWindow6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QWindow::isTopLevel();
  fn _ZNK7QWindow10isTopLevelEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWindow::QWindow(const QWindow & );
  fn dector_ZN7QWindowC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QWindowC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::setIcon(const QIcon & icon);
  fn _ZN7QWindow7setIconERK5QIcon(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QWindow::opacity();
  fn _ZNK7QWindow7opacityEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QWindow::setMinimumSize(const QSize & size);
  fn _ZN7QWindow14setMinimumSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QWindow::minimumHeight();
  fn demth_ZNK7QWindow13minimumHeightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QSize QWindow::sizeIncrement();
  fn _ZNK7QWindow13sizeIncrementEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::resize(const QSize & newSize);
  fn _ZN7QWindow6resizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::screenChanged(QScreen * screen);
  fn _ZN7QWindow13screenChangedEP7QScreen(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::setTitle(const QString & );
  fn _ZN7QWindow8setTitleERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::raise();
  fn _ZN7QWindow5raiseEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QWindow::minimumSize();
  fn _ZNK7QWindow11minimumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPoint QWindow::mapToGlobal(const QPoint & pos);
  fn _ZNK7QWindow11mapToGlobalERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto: static QWindow * QWindow::fromWinId(WId id);
  fn _ZN7QWindow9fromWinIdEi(arg0: *mut c_uint) -> *mut c_void;
  // proto:  QMargins QWindow::frameMargins();
  fn _ZNK7QWindow12frameMarginsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::setMaximumWidth(int w);
  fn _ZN7QWindow15setMaximumWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QWindow::maximumHeight();
  fn demth_ZNK7QWindow13maximumHeightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QWindow::focusObjectChanged(QObject * object);
  fn _ZN7QWindow18focusObjectChangedEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QWindow::isModal();
  fn _ZNK7QWindow7isModalEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWindow::maximumWidthChanged(int arg);
  fn _ZN7QWindow19maximumWidthChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QRect QWindow::geometry();
  fn _ZNK7QWindow8geometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::setParent(QWindow * parent);
  fn _ZN7QWindow9setParentEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRect QWindow::frameGeometry();
  fn _ZNK7QWindow13frameGeometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSurfaceFormat QWindow::requestedFormat();
  fn _ZNK7QWindow15requestedFormatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::setHeight(int arg);
  fn _ZN7QWindow9setHeightEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWindow::requestActivate();
  fn _ZN7QWindow15requestActivateEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPoint QWindow::mapFromGlobal(const QPoint & pos);
  fn _ZNK7QWindow13mapFromGlobalERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::windowTitleChanged(const QString & title);
  fn _ZN7QWindow18windowTitleChangedERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QWindow::y();
  fn demth_ZNK7QWindow1yEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QWindow::width();
  fn demth_ZNK7QWindow5widthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QWindow::setFilePath(const QString & filePath);
  fn _ZN7QWindow11setFilePathERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::setCursor(const QCursor & );
  fn _ZN7QWindow9setCursorERK7QCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::setVisible(bool visible);
  fn _ZN7QWindow10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QWindow::~QWindow();
  fn _ZN7QWindowD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QWindow::setMouseGrabEnabled(bool grab);
  fn _ZN7QWindow19setMouseGrabEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char) -> c_char;
  // proto:  bool QWindow::isExposed();
  fn _ZNK7QWindow9isExposedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWindow::heightChanged(int arg);
  fn _ZN7QWindow13heightChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QWindow::minimumWidth();
  fn demth_ZNK7QWindow12minimumWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QWindow::setPosition(const QPoint & pt);
  fn _ZN7QWindow11setPositionERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QWindow::close();
  fn _ZN7QWindow5closeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QWindow::x();
  fn demth_ZNK7QWindow1xEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWindow::setMinimumWidth(int w);
  fn _ZN7QWindow15setMinimumWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QRegion QWindow::mask();
  fn _ZNK7QWindow4maskEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::widthChanged(int arg);
  fn _ZN7QWindow12widthChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QWindow * QWindow::parent();
  fn _ZNK7QWindow6parentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::setFramePosition(const QPoint & point);
  fn _ZN7QWindow16setFramePositionERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::QWindow(QScreen * screen);
  fn dector_ZN7QWindowC1EP7QScreen(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QWindowC1EP7QScreen(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::setGeometry(int posx, int posy, int w, int h);
  fn _ZN7QWindow11setGeometryEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  bool QWindow::setKeyboardGrabEnabled(bool grab);
  fn _ZN7QWindow22setKeyboardGrabEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char) -> c_char;
  // proto:  const QMetaObject * QWindow::metaObject();
  fn _ZNK7QWindow10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWindow::QWindow(QWindow * parent);
  fn dector_ZN7QWindowC1EPS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QWindowC1EPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::activeChanged();
  fn _ZN7QWindow13activeChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWindow::setWidth(int arg);
  fn _ZN7QWindow8setWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWindow::setY(int arg);
  fn _ZN7QWindow4setYEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  qreal QWindow::devicePixelRatio();
  fn _ZNK7QWindow16devicePixelRatioEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QWindow::setBaseSize(const QSize & size);
  fn _ZN7QWindow11setBaseSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::alert(int msec);
  fn _ZN7QWindow5alertEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWindow::yChanged(int arg);
  fn _ZN7QWindow8yChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QPlatformWindow * QWindow::handle();
  fn _ZNK7QWindow6handleEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWindow::destroy();
  fn _ZN7QWindow7destroyEv(qthis: u64 /* *mut c_void*/);
  // proto:  QWindow * QWindow::transientParent();
  fn _ZNK7QWindow15transientParentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::setMinimumHeight(int h);
  fn _ZN7QWindow16setMinimumHeightEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWindow::show();
  fn _ZN7QWindow4showEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWindow::minimumWidthChanged(int arg);
  fn _ZN7QWindow19minimumWidthChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QSize QWindow::baseSize();
  fn _ZNK7QWindow8baseSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QWindow::title();
  fn _ZNK7QWindow5titleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::showMaximized();
  fn _ZN7QWindow13showMaximizedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWindow::create();
  fn _ZN7QWindow6createEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWindow::resize(int w, int h);
  fn _ZN7QWindow6resizeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  QScreen * QWindow::screen();
  fn _ZNK7QWindow6screenEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::setPosition(int posx, int posy);
  fn _ZN7QWindow11setPositionEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QWindow::setOpacity(qreal level);
  fn _ZN7QWindow10setOpacityEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QWindow::setGeometry(const QRect & rect);
  fn _ZN7QWindow11setGeometryERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::setSizeIncrement(const QSize & size);
  fn _ZN7QWindow16setSizeIncrementERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::showMinimized();
  fn _ZN7QWindow13showMinimizedEv(qthis: u64 /* *mut c_void*/);
  // proto:  QObject * QWindow::focusObject();
  fn _ZNK7QWindow11focusObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QWindow::isActive();
  fn _ZNK7QWindow8isActiveEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWindow::maximumHeightChanged(int arg);
  fn _ZN7QWindow20maximumHeightChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QAccessibleInterface * QWindow::accessibleRoot();
  fn _ZNK7QWindow14accessibleRootEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QCursor QWindow::cursor();
  fn _ZNK7QWindow6cursorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::setFormat(const QSurfaceFormat & format);
  fn _ZN7QWindow9setFormatERK14QSurfaceFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::showFullScreen();
  fn _ZN7QWindow14showFullScreenEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWindow::setX(int arg);
  fn _ZN7QWindow4setXEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWindow::opacityChanged(qreal opacity);
  fn _ZN7QWindow14opacityChangedEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QWindow::lower();
  fn _ZN7QWindow5lowerEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWindow::requestUpdate();
  fn _ZN7QWindow13requestUpdateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWindow::hide();
  fn _ZN7QWindow4hideEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWindow::minimumHeightChanged(int arg);
  fn _ZN7QWindow20minimumHeightChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWindow::setMask(const QRegion & region);
  fn _ZN7QWindow7setMaskERK7QRegion(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWindow::setMaximumSize(const QSize & size);
  fn _ZN7QWindow14setMaximumSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QWindow::height();
  fn demth_ZNK7QWindow6heightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QSize QWindow::size();
  fn demth_ZNK7QWindow4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QWindow::maximumWidth();
  fn demth_ZNK7QWindow12maximumWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QPoint QWindow::position();
  fn demth_ZNK7QWindow8positionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::setMaximumHeight(int h);
  fn _ZN7QWindow16setMaximumHeightEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QString QWindow::filePath();
  fn _ZNK7QWindow8filePathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::showNormal();
  fn _ZN7QWindow10showNormalEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPoint QWindow::framePosition();
  fn _ZNK7QWindow13framePositionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWindow::visibleChanged(bool arg);
  fn _ZN7QWindow14visibleChangedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QIcon QWindow::icon();
  fn _ZNK7QWindow4iconEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QWindow_SlotProxy_connect__ZN7QWindow8xChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow12widthChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow25contentOrientationChangedEN2Qt17ScreenOrientationE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow15modalityChangedEN2Qt14WindowModalityE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow18windowStateChangedEN2Qt11WindowStateE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow17visibilityChangedENS_10VisibilityE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow14opacityChangedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow8yChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow20minimumHeightChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow13heightChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow19maximumWidthChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow13screenChangedEP7QScreen(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow19minimumWidthChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow18windowTitleChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow18focusObjectChangedEP7QObject(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow14visibleChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow20maximumHeightChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWindow_SlotProxy_connect__ZN7QWindow13activeChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QWindow)=1
#[derive(Default)]
pub struct QWindow {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _modalityChanged_1: QWindow_modalityChanged_signal,
  pub _activeChanged_1: QWindow_activeChanged_signal,
  pub _heightChanged_1: QWindow_heightChanged_signal,
  pub _contentOrientationChanged_1: QWindow_contentOrientationChanged_signal,
  pub _minimumWidthChanged_1: QWindow_minimumWidthChanged_signal,
  pub _opacityChanged_1: QWindow_opacityChanged_signal,
  pub _visibleChanged_1: QWindow_visibleChanged_signal,
  pub _screenChanged_1: QWindow_screenChanged_signal,
  pub _maximumHeightChanged_1: QWindow_maximumHeightChanged_signal,
  pub _yChanged_1: QWindow_yChanged_signal,
  pub _widthChanged_1: QWindow_widthChanged_signal,
  pub _windowStateChanged_1: QWindow_windowStateChanged_signal,
  pub _windowTitleChanged_1: QWindow_windowTitleChanged_signal,
  pub _visibilityChanged_1: QWindow_visibilityChanged_signal,
  pub _minimumHeightChanged_1: QWindow_minimumHeightChanged_signal,
  pub _xChanged_1: QWindow_xChanged_signal,
  pub _focusObjectChanged_1: QWindow_focusObjectChanged_signal,
  pub _maximumWidthChanged_1: QWindow_maximumWidthChanged_signal,
}

impl /*struct*/ QWindow {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QWindow {
    return QWindow{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QWindow {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QWindow {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QWindow::xChanged(int arg);
impl /*struct*/ QWindow {
  pub fn xChanged<RetType, T: QWindow_xChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.xChanged(self);
    // return 1;
  }
}

pub trait QWindow_xChanged<RetType> {
  fn xChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::xChanged(int arg);
impl<'a> /*trait*/ QWindow_xChanged<()> for (i32) {
  fn xChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8xChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow8xChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::unsetCursor();
impl /*struct*/ QWindow {
  pub fn unsetCursor<RetType, T: QWindow_unsetCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unsetCursor(self);
    // return 1;
  }
}

pub trait QWindow_unsetCursor<RetType> {
  fn unsetCursor(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::unsetCursor();
impl<'a> /*trait*/ QWindow_unsetCursor<()> for () {
  fn unsetCursor(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11unsetCursorEv()};
     unsafe {_ZN7QWindow11unsetCursorEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QWindow::isVisible();
impl /*struct*/ QWindow {
  pub fn isVisible<RetType, T: QWindow_isVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isVisible(self);
    // return 1;
  }
}

pub trait QWindow_isVisible<RetType> {
  fn isVisible(self , rsthis: & QWindow) -> RetType;
}

  // proto:  bool QWindow::isVisible();
impl<'a> /*trait*/ QWindow_isVisible<i8> for () {
  fn isVisible(self , rsthis: & QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow9isVisibleEv()};
    let mut ret = unsafe {_ZNK7QWindow9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWindow::setScreen(QScreen * screen);
impl /*struct*/ QWindow {
  pub fn setScreen<RetType, T: QWindow_setScreen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScreen(self);
    // return 1;
  }
}

pub trait QWindow_setScreen<RetType> {
  fn setScreen(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setScreen(QScreen * screen);
impl<'a> /*trait*/ QWindow_setScreen<()> for (&'a QScreen) {
  fn setScreen(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setScreenEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow9setScreenEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QWindow::maximumSize();
impl /*struct*/ QWindow {
  pub fn maximumSize<RetType, T: QWindow_maximumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QWindow_maximumSize<RetType> {
  fn maximumSize(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QSize QWindow::maximumSize();
impl<'a> /*trait*/ QWindow_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: & QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11maximumSizeEv()};
    let mut ret = unsafe {_ZNK7QWindow11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setTransientParent(QWindow * parent);
impl /*struct*/ QWindow {
  pub fn setTransientParent<RetType, T: QWindow_setTransientParent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTransientParent(self);
    // return 1;
  }
}

pub trait QWindow_setTransientParent<RetType> {
  fn setTransientParent(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setTransientParent(QWindow * parent);
impl<'a> /*trait*/ QWindow_setTransientParent<()> for (&'a QWindow) {
  fn setTransientParent(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow18setTransientParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow18setTransientParentEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSurfaceFormat QWindow::format();
impl /*struct*/ QWindow {
  pub fn format<RetType, T: QWindow_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QWindow_format<RetType> {
  fn format(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QSurfaceFormat QWindow::format();
impl<'a> /*trait*/ QWindow_format<QSurfaceFormat> for () {
  fn format(self , rsthis: & QWindow) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6formatEv()};
    let mut ret = unsafe {_ZNK7QWindow6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWindow::isTopLevel();
impl /*struct*/ QWindow {
  pub fn isTopLevel<RetType, T: QWindow_isTopLevel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTopLevel(self);
    // return 1;
  }
}

pub trait QWindow_isTopLevel<RetType> {
  fn isTopLevel(self , rsthis: & QWindow) -> RetType;
}

  // proto:  bool QWindow::isTopLevel();
impl<'a> /*trait*/ QWindow_isTopLevel<i8> for () {
  fn isTopLevel(self , rsthis: & QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow10isTopLevelEv()};
    let mut ret = unsafe {_ZNK7QWindow10isTopLevelEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWindow::QWindow(const QWindow & );
impl /*struct*/ QWindow {
  pub fn new<T: QWindow_new>(value: T) -> QWindow {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QWindow_new {
  fn new(self) -> QWindow;
}

  // proto:  void QWindow::QWindow(const QWindow & );
impl<'a> /*trait*/ QWindow_new for (&'a QWindow) {
  fn new(self) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindowC1ERKS_()};
    let ctysz: c_int = unsafe{QWindow_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN7QWindowC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN7QWindowC1ERKS_(arg0)} as u64;
    let rsthis = QWindow{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QWindow::setIcon(const QIcon & icon);
impl /*struct*/ QWindow {
  pub fn setIcon<RetType, T: QWindow_setIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIcon(self);
    // return 1;
  }
}

pub trait QWindow_setIcon<RetType> {
  fn setIcon(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QWindow_setIcon<()> for (&'a QIcon) {
  fn setIcon(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QWindow::opacity();
impl /*struct*/ QWindow {
  pub fn opacity<RetType, T: QWindow_opacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opacity(self);
    // return 1;
  }
}

pub trait QWindow_opacity<RetType> {
  fn opacity(self , rsthis: & QWindow) -> RetType;
}

  // proto:  qreal QWindow::opacity();
impl<'a> /*trait*/ QWindow_opacity<f64> for () {
  fn opacity(self , rsthis: & QWindow) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow7opacityEv()};
    let mut ret = unsafe {_ZNK7QWindow7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QWindow::setMinimumSize(const QSize & size);
impl /*struct*/ QWindow {
  pub fn setMinimumSize<RetType, T: QWindow_setMinimumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumSize(self);
    // return 1;
  }
}

pub trait QWindow_setMinimumSize<RetType> {
  fn setMinimumSize(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setMinimumSize(const QSize & size);
impl<'a> /*trait*/ QWindow_setMinimumSize<()> for (&'a QSize) {
  fn setMinimumSize(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14setMinimumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow14setMinimumSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWindow::minimumHeight();
impl /*struct*/ QWindow {
  pub fn minimumHeight<RetType, T: QWindow_minimumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumHeight(self);
    // return 1;
  }
}

pub trait QWindow_minimumHeight<RetType> {
  fn minimumHeight(self , rsthis: & QWindow) -> RetType;
}

  // proto:  int QWindow::minimumHeight();
impl<'a> /*trait*/ QWindow_minimumHeight<i32> for () {
  fn minimumHeight(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13minimumHeightEv()};
    let mut ret = unsafe {demth_ZNK7QWindow13minimumHeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QWindow::sizeIncrement();
impl /*struct*/ QWindow {
  pub fn sizeIncrement<RetType, T: QWindow_sizeIncrement<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeIncrement(self);
    // return 1;
  }
}

pub trait QWindow_sizeIncrement<RetType> {
  fn sizeIncrement(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QSize QWindow::sizeIncrement();
impl<'a> /*trait*/ QWindow_sizeIncrement<QSize> for () {
  fn sizeIncrement(self , rsthis: & QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13sizeIncrementEv()};
    let mut ret = unsafe {_ZNK7QWindow13sizeIncrementEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::resize(const QSize & newSize);
impl /*struct*/ QWindow {
  pub fn resize<RetType, T: QWindow_resize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QWindow_resize<RetType> {
  fn resize(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::resize(const QSize & newSize);
impl<'a> /*trait*/ QWindow_resize<()> for (&'a QSize) {
  fn resize(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow6resizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow6resizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::screenChanged(QScreen * screen);
impl /*struct*/ QWindow {
  pub fn screenChanged<RetType, T: QWindow_screenChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenChanged(self);
    // return 1;
  }
}

pub trait QWindow_screenChanged<RetType> {
  fn screenChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::screenChanged(QScreen * screen);
impl<'a> /*trait*/ QWindow_screenChanged<()> for (&'a QScreen) {
  fn screenChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13screenChangedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow13screenChangedEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setTitle(const QString & );
impl /*struct*/ QWindow {
  pub fn setTitle<RetType, T: QWindow_setTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTitle(self);
    // return 1;
  }
}

pub trait QWindow_setTitle<RetType> {
  fn setTitle(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setTitle(const QString & );
impl<'a> /*trait*/ QWindow_setTitle<()> for (&'a QString) {
  fn setTitle(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::raise();
impl /*struct*/ QWindow {
  pub fn raise<RetType, T: QWindow_raise<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.raise(self);
    // return 1;
  }
}

pub trait QWindow_raise<RetType> {
  fn raise(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::raise();
impl<'a> /*trait*/ QWindow_raise<()> for () {
  fn raise(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5raiseEv()};
     unsafe {_ZN7QWindow5raiseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QWindow::minimumSize();
impl /*struct*/ QWindow {
  pub fn minimumSize<RetType, T: QWindow_minimumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QWindow_minimumSize<RetType> {
  fn minimumSize(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QSize QWindow::minimumSize();
impl<'a> /*trait*/ QWindow_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: & QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11minimumSizeEv()};
    let mut ret = unsafe {_ZNK7QWindow11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QWindow::mapToGlobal(const QPoint & pos);
impl /*struct*/ QWindow {
  pub fn mapToGlobal<RetType, T: QWindow_mapToGlobal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToGlobal(self);
    // return 1;
  }
}

pub trait QWindow_mapToGlobal<RetType> {
  fn mapToGlobal(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QPoint QWindow::mapToGlobal(const QPoint & pos);
impl<'a> /*trait*/ QWindow_mapToGlobal<QPoint> for (&'a QPoint) {
  fn mapToGlobal(self , rsthis: & QWindow) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11mapToGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWindow11mapToGlobalERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QWindow * QWindow::fromWinId(WId id);
impl /*struct*/ QWindow {
  pub fn fromWinId_s<RetType, T: QWindow_fromWinId_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromWinId_s();
    // return 1;
  }
}

pub trait QWindow_fromWinId_s<RetType> {
  fn fromWinId_s(self ) -> RetType;
}

  // proto: static QWindow * QWindow::fromWinId(WId id);
impl<'a> /*trait*/ QWindow_fromWinId_s<QWindow> for (*mut i32) {
  fn fromWinId_s(self ) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9fromWinIdEi()};
    let arg0 = self  as *mut c_uint;
    let mut ret = unsafe {_ZN7QWindow9fromWinIdEi(arg0)};
    let mut ret1 = QWindow::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QMargins QWindow::frameMargins();
impl /*struct*/ QWindow {
  pub fn frameMargins<RetType, T: QWindow_frameMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameMargins(self);
    // return 1;
  }
}

pub trait QWindow_frameMargins<RetType> {
  fn frameMargins(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QMargins QWindow::frameMargins();
impl<'a> /*trait*/ QWindow_frameMargins<QMargins> for () {
  fn frameMargins(self , rsthis: & QWindow) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow12frameMarginsEv()};
    let mut ret = unsafe {_ZNK7QWindow12frameMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMargins::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setMaximumWidth(int w);
impl /*struct*/ QWindow {
  pub fn setMaximumWidth<RetType, T: QWindow_setMaximumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumWidth(self);
    // return 1;
  }
}

pub trait QWindow_setMaximumWidth<RetType> {
  fn setMaximumWidth(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setMaximumWidth(int w);
impl<'a> /*trait*/ QWindow_setMaximumWidth<()> for (i32) {
  fn setMaximumWidth(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow15setMaximumWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow15setMaximumWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWindow::maximumHeight();
impl /*struct*/ QWindow {
  pub fn maximumHeight<RetType, T: QWindow_maximumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumHeight(self);
    // return 1;
  }
}

pub trait QWindow_maximumHeight<RetType> {
  fn maximumHeight(self , rsthis: & QWindow) -> RetType;
}

  // proto:  int QWindow::maximumHeight();
impl<'a> /*trait*/ QWindow_maximumHeight<i32> for () {
  fn maximumHeight(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13maximumHeightEv()};
    let mut ret = unsafe {demth_ZNK7QWindow13maximumHeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWindow::focusObjectChanged(QObject * object);
impl /*struct*/ QWindow {
  pub fn focusObjectChanged<RetType, T: QWindow_focusObjectChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusObjectChanged(self);
    // return 1;
  }
}

pub trait QWindow_focusObjectChanged<RetType> {
  fn focusObjectChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::focusObjectChanged(QObject * object);
impl<'a> /*trait*/ QWindow_focusObjectChanged<()> for (&'a QObject) {
  fn focusObjectChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow18focusObjectChangedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow18focusObjectChangedEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWindow::isModal();
impl /*struct*/ QWindow {
  pub fn isModal<RetType, T: QWindow_isModal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isModal(self);
    // return 1;
  }
}

pub trait QWindow_isModal<RetType> {
  fn isModal(self , rsthis: & QWindow) -> RetType;
}

  // proto:  bool QWindow::isModal();
impl<'a> /*trait*/ QWindow_isModal<i8> for () {
  fn isModal(self , rsthis: & QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow7isModalEv()};
    let mut ret = unsafe {_ZNK7QWindow7isModalEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWindow::maximumWidthChanged(int arg);
impl /*struct*/ QWindow {
  pub fn maximumWidthChanged<RetType, T: QWindow_maximumWidthChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumWidthChanged(self);
    // return 1;
  }
}

pub trait QWindow_maximumWidthChanged<RetType> {
  fn maximumWidthChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::maximumWidthChanged(int arg);
impl<'a> /*trait*/ QWindow_maximumWidthChanged<()> for (i32) {
  fn maximumWidthChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow19maximumWidthChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow19maximumWidthChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QWindow::geometry();
impl /*struct*/ QWindow {
  pub fn geometry<RetType, T: QWindow_geometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QWindow_geometry<RetType> {
  fn geometry(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QRect QWindow::geometry();
impl<'a> /*trait*/ QWindow_geometry<QRect> for () {
  fn geometry(self , rsthis: & QWindow) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8geometryEv()};
    let mut ret = unsafe {_ZNK7QWindow8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setParent(QWindow * parent);
impl /*struct*/ QWindow {
  pub fn setParent<RetType, T: QWindow_setParent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setParent(self);
    // return 1;
  }
}

pub trait QWindow_setParent<RetType> {
  fn setParent(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setParent(QWindow * parent);
impl<'a> /*trait*/ QWindow_setParent<()> for (&'a QWindow) {
  fn setParent(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow9setParentEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QWindow::frameGeometry();
impl /*struct*/ QWindow {
  pub fn frameGeometry<RetType, T: QWindow_frameGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameGeometry(self);
    // return 1;
  }
}

pub trait QWindow_frameGeometry<RetType> {
  fn frameGeometry(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QRect QWindow::frameGeometry();
impl<'a> /*trait*/ QWindow_frameGeometry<QRect> for () {
  fn frameGeometry(self , rsthis: & QWindow) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13frameGeometryEv()};
    let mut ret = unsafe {_ZNK7QWindow13frameGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSurfaceFormat QWindow::requestedFormat();
impl /*struct*/ QWindow {
  pub fn requestedFormat<RetType, T: QWindow_requestedFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.requestedFormat(self);
    // return 1;
  }
}

pub trait QWindow_requestedFormat<RetType> {
  fn requestedFormat(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QSurfaceFormat QWindow::requestedFormat();
impl<'a> /*trait*/ QWindow_requestedFormat<QSurfaceFormat> for () {
  fn requestedFormat(self , rsthis: & QWindow) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow15requestedFormatEv()};
    let mut ret = unsafe {_ZNK7QWindow15requestedFormatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setHeight(int arg);
impl /*struct*/ QWindow {
  pub fn setHeight<RetType, T: QWindow_setHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHeight(self);
    // return 1;
  }
}

pub trait QWindow_setHeight<RetType> {
  fn setHeight(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setHeight(int arg);
impl<'a> /*trait*/ QWindow_setHeight<()> for (i32) {
  fn setHeight(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow9setHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::requestActivate();
impl /*struct*/ QWindow {
  pub fn requestActivate<RetType, T: QWindow_requestActivate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.requestActivate(self);
    // return 1;
  }
}

pub trait QWindow_requestActivate<RetType> {
  fn requestActivate(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::requestActivate();
impl<'a> /*trait*/ QWindow_requestActivate<()> for () {
  fn requestActivate(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow15requestActivateEv()};
     unsafe {_ZN7QWindow15requestActivateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QWindow::mapFromGlobal(const QPoint & pos);
impl /*struct*/ QWindow {
  pub fn mapFromGlobal<RetType, T: QWindow_mapFromGlobal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapFromGlobal(self);
    // return 1;
  }
}

pub trait QWindow_mapFromGlobal<RetType> {
  fn mapFromGlobal(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QPoint QWindow::mapFromGlobal(const QPoint & pos);
impl<'a> /*trait*/ QWindow_mapFromGlobal<QPoint> for (&'a QPoint) {
  fn mapFromGlobal(self , rsthis: & QWindow) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13mapFromGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWindow13mapFromGlobalERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::windowTitleChanged(const QString & title);
impl /*struct*/ QWindow {
  pub fn windowTitleChanged<RetType, T: QWindow_windowTitleChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowTitleChanged(self);
    // return 1;
  }
}

pub trait QWindow_windowTitleChanged<RetType> {
  fn windowTitleChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::windowTitleChanged(const QString & title);
impl<'a> /*trait*/ QWindow_windowTitleChanged<()> for (&'a QString) {
  fn windowTitleChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow18windowTitleChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow18windowTitleChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWindow::y();
impl /*struct*/ QWindow {
  pub fn y<RetType, T: QWindow_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QWindow_y<RetType> {
  fn y(self , rsthis: & QWindow) -> RetType;
}

  // proto:  int QWindow::y();
impl<'a> /*trait*/ QWindow_y<()> for () {
  fn y(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow1yEv()};
     unsafe {demth_ZNK7QWindow1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QWindow::width();
impl /*struct*/ QWindow {
  pub fn width<RetType, T: QWindow_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QWindow_width<RetType> {
  fn width(self , rsthis: & QWindow) -> RetType;
}

  // proto:  int QWindow::width();
impl<'a> /*trait*/ QWindow_width<i32> for () {
  fn width(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow5widthEv()};
    let mut ret = unsafe {demth_ZNK7QWindow5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWindow::setFilePath(const QString & filePath);
impl /*struct*/ QWindow {
  pub fn setFilePath<RetType, T: QWindow_setFilePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFilePath(self);
    // return 1;
  }
}

pub trait QWindow_setFilePath<RetType> {
  fn setFilePath(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setFilePath(const QString & filePath);
impl<'a> /*trait*/ QWindow_setFilePath<()> for (&'a QString) {
  fn setFilePath(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setFilePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow11setFilePathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setCursor(const QCursor & );
impl /*struct*/ QWindow {
  pub fn setCursor<RetType, T: QWindow_setCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCursor(self);
    // return 1;
  }
}

pub trait QWindow_setCursor<RetType> {
  fn setCursor(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setCursor(const QCursor & );
impl<'a> /*trait*/ QWindow_setCursor<()> for (&'a QCursor) {
  fn setCursor(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow9setCursorERK7QCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setVisible(bool visible);
impl /*struct*/ QWindow {
  pub fn setVisible<RetType, T: QWindow_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QWindow_setVisible<RetType> {
  fn setVisible(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setVisible(bool visible);
impl<'a> /*trait*/ QWindow_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QWindow10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::~QWindow();
impl /*struct*/ QWindow {
  pub fn free<RetType, T: QWindow_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QWindow_free<RetType> {
  fn free(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::~QWindow();
impl<'a> /*trait*/ QWindow_free<()> for () {
  fn free(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindowD0Ev()};
     unsafe {_ZN7QWindowD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QWindow::setMouseGrabEnabled(bool grab);
impl /*struct*/ QWindow {
  pub fn setMouseGrabEnabled<RetType, T: QWindow_setMouseGrabEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMouseGrabEnabled(self);
    // return 1;
  }
}

pub trait QWindow_setMouseGrabEnabled<RetType> {
  fn setMouseGrabEnabled(self , rsthis: & QWindow) -> RetType;
}

  // proto:  bool QWindow::setMouseGrabEnabled(bool grab);
impl<'a> /*trait*/ QWindow_setMouseGrabEnabled<i8> for (i8) {
  fn setMouseGrabEnabled(self , rsthis: & QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow19setMouseGrabEnabledEb()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZN7QWindow19setMouseGrabEnabledEb(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QWindow::isExposed();
impl /*struct*/ QWindow {
  pub fn isExposed<RetType, T: QWindow_isExposed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isExposed(self);
    // return 1;
  }
}

pub trait QWindow_isExposed<RetType> {
  fn isExposed(self , rsthis: & QWindow) -> RetType;
}

  // proto:  bool QWindow::isExposed();
impl<'a> /*trait*/ QWindow_isExposed<i8> for () {
  fn isExposed(self , rsthis: & QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow9isExposedEv()};
    let mut ret = unsafe {_ZNK7QWindow9isExposedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWindow::heightChanged(int arg);
impl /*struct*/ QWindow {
  pub fn heightChanged<RetType, T: QWindow_heightChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.heightChanged(self);
    // return 1;
  }
}

pub trait QWindow_heightChanged<RetType> {
  fn heightChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::heightChanged(int arg);
impl<'a> /*trait*/ QWindow_heightChanged<()> for (i32) {
  fn heightChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13heightChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow13heightChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWindow::minimumWidth();
impl /*struct*/ QWindow {
  pub fn minimumWidth<RetType, T: QWindow_minimumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumWidth(self);
    // return 1;
  }
}

pub trait QWindow_minimumWidth<RetType> {
  fn minimumWidth(self , rsthis: & QWindow) -> RetType;
}

  // proto:  int QWindow::minimumWidth();
impl<'a> /*trait*/ QWindow_minimumWidth<i32> for () {
  fn minimumWidth(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow12minimumWidthEv()};
    let mut ret = unsafe {demth_ZNK7QWindow12minimumWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWindow::setPosition(const QPoint & pt);
impl /*struct*/ QWindow {
  pub fn setPosition<RetType, T: QWindow_setPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPosition(self);
    // return 1;
  }
}

pub trait QWindow_setPosition<RetType> {
  fn setPosition(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setPosition(const QPoint & pt);
impl<'a> /*trait*/ QWindow_setPosition<()> for (&'a QPoint) {
  fn setPosition(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setPositionERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow11setPositionERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWindow::close();
impl /*struct*/ QWindow {
  pub fn close<RetType, T: QWindow_close<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QWindow_close<RetType> {
  fn close(self , rsthis: & QWindow) -> RetType;
}

  // proto:  bool QWindow::close();
impl<'a> /*trait*/ QWindow_close<i8> for () {
  fn close(self , rsthis: & QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5closeEv()};
    let mut ret = unsafe {_ZN7QWindow5closeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QWindow::x();
impl /*struct*/ QWindow {
  pub fn x<RetType, T: QWindow_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QWindow_x<RetType> {
  fn x(self , rsthis: & QWindow) -> RetType;
}

  // proto:  int QWindow::x();
impl<'a> /*trait*/ QWindow_x<()> for () {
  fn x(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow1xEv()};
     unsafe {demth_ZNK7QWindow1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::setMinimumWidth(int w);
impl /*struct*/ QWindow {
  pub fn setMinimumWidth<RetType, T: QWindow_setMinimumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumWidth(self);
    // return 1;
  }
}

pub trait QWindow_setMinimumWidth<RetType> {
  fn setMinimumWidth(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setMinimumWidth(int w);
impl<'a> /*trait*/ QWindow_setMinimumWidth<()> for (i32) {
  fn setMinimumWidth(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow15setMinimumWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow15setMinimumWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRegion QWindow::mask();
impl /*struct*/ QWindow {
  pub fn mask<RetType, T: QWindow_mask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mask(self);
    // return 1;
  }
}

pub trait QWindow_mask<RetType> {
  fn mask(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QRegion QWindow::mask();
impl<'a> /*trait*/ QWindow_mask<QRegion> for () {
  fn mask(self , rsthis: & QWindow) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow4maskEv()};
    let mut ret = unsafe {_ZNK7QWindow4maskEv(rsthis.qclsinst)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::widthChanged(int arg);
impl /*struct*/ QWindow {
  pub fn widthChanged<RetType, T: QWindow_widthChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widthChanged(self);
    // return 1;
  }
}

pub trait QWindow_widthChanged<RetType> {
  fn widthChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::widthChanged(int arg);
impl<'a> /*trait*/ QWindow_widthChanged<()> for (i32) {
  fn widthChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow12widthChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow12widthChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWindow * QWindow::parent();
impl /*struct*/ QWindow {
  pub fn parent<RetType, T: QWindow_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QWindow_parent<RetType> {
  fn parent(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QWindow * QWindow::parent();
impl<'a> /*trait*/ QWindow_parent<QWindow> for () {
  fn parent(self , rsthis: & QWindow) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6parentEv()};
    let mut ret = unsafe {_ZNK7QWindow6parentEv(rsthis.qclsinst)};
    let mut ret1 = QWindow::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setFramePosition(const QPoint & point);
impl /*struct*/ QWindow {
  pub fn setFramePosition<RetType, T: QWindow_setFramePosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFramePosition(self);
    // return 1;
  }
}

pub trait QWindow_setFramePosition<RetType> {
  fn setFramePosition(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setFramePosition(const QPoint & point);
impl<'a> /*trait*/ QWindow_setFramePosition<()> for (&'a QPoint) {
  fn setFramePosition(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setFramePositionERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow16setFramePositionERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::QWindow(QScreen * screen);
impl<'a> /*trait*/ QWindow_new for (&'a QScreen) {
  fn new(self) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindowC1EP7QScreen()};
    let ctysz: c_int = unsafe{QWindow_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN7QWindowC1EP7QScreen(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN7QWindowC1EP7QScreen(arg0)} as u64;
    let rsthis = QWindow{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QWindow::setGeometry(int posx, int posy, int w, int h);
impl /*struct*/ QWindow {
  pub fn setGeometry<RetType, T: QWindow_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QWindow_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setGeometry(int posx, int posy, int w, int h);
impl<'a> /*trait*/ QWindow_setGeometry<()> for (i32, i32, i32, i32) {
  fn setGeometry(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setGeometryEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN7QWindow11setGeometryEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  bool QWindow::setKeyboardGrabEnabled(bool grab);
impl /*struct*/ QWindow {
  pub fn setKeyboardGrabEnabled<RetType, T: QWindow_setKeyboardGrabEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKeyboardGrabEnabled(self);
    // return 1;
  }
}

pub trait QWindow_setKeyboardGrabEnabled<RetType> {
  fn setKeyboardGrabEnabled(self , rsthis: & QWindow) -> RetType;
}

  // proto:  bool QWindow::setKeyboardGrabEnabled(bool grab);
impl<'a> /*trait*/ QWindow_setKeyboardGrabEnabled<i8> for (i8) {
  fn setKeyboardGrabEnabled(self , rsthis: & QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow22setKeyboardGrabEnabledEb()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZN7QWindow22setKeyboardGrabEnabledEb(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QWindow::metaObject();
impl /*struct*/ QWindow {
  pub fn metaObject<RetType, T: QWindow_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QWindow_metaObject<RetType> {
  fn metaObject(self , rsthis: & QWindow) -> RetType;
}

  // proto:  const QMetaObject * QWindow::metaObject();
impl<'a> /*trait*/ QWindow_metaObject<()> for () {
  fn metaObject(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow10metaObjectEv()};
     unsafe {_ZNK7QWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::activeChanged();
impl /*struct*/ QWindow {
  pub fn activeChanged<RetType, T: QWindow_activeChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeChanged(self);
    // return 1;
  }
}

pub trait QWindow_activeChanged<RetType> {
  fn activeChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::activeChanged();
impl<'a> /*trait*/ QWindow_activeChanged<()> for () {
  fn activeChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13activeChangedEv()};
     unsafe {_ZN7QWindow13activeChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::setWidth(int arg);
impl /*struct*/ QWindow {
  pub fn setWidth<RetType, T: QWindow_setWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QWindow_setWidth<RetType> {
  fn setWidth(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setWidth(int arg);
impl<'a> /*trait*/ QWindow_setWidth<()> for (i32) {
  fn setWidth(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8setWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow8setWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setY(int arg);
impl /*struct*/ QWindow {
  pub fn setY<RetType, T: QWindow_setY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QWindow_setY<RetType> {
  fn setY(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setY(int arg);
impl<'a> /*trait*/ QWindow_setY<()> for (i32) {
  fn setY(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4setYEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow4setYEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QWindow::devicePixelRatio();
impl /*struct*/ QWindow {
  pub fn devicePixelRatio<RetType, T: QWindow_devicePixelRatio<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QWindow_devicePixelRatio<RetType> {
  fn devicePixelRatio(self , rsthis: & QWindow) -> RetType;
}

  // proto:  qreal QWindow::devicePixelRatio();
impl<'a> /*trait*/ QWindow_devicePixelRatio<f64> for () {
  fn devicePixelRatio(self , rsthis: & QWindow) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK7QWindow16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QWindow::setBaseSize(const QSize & size);
impl /*struct*/ QWindow {
  pub fn setBaseSize<RetType, T: QWindow_setBaseSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBaseSize(self);
    // return 1;
  }
}

pub trait QWindow_setBaseSize<RetType> {
  fn setBaseSize(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setBaseSize(const QSize & size);
impl<'a> /*trait*/ QWindow_setBaseSize<()> for (&'a QSize) {
  fn setBaseSize(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setBaseSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow11setBaseSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::alert(int msec);
impl /*struct*/ QWindow {
  pub fn alert<RetType, T: QWindow_alert<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.alert(self);
    // return 1;
  }
}

pub trait QWindow_alert<RetType> {
  fn alert(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::alert(int msec);
impl<'a> /*trait*/ QWindow_alert<()> for (i32) {
  fn alert(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5alertEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow5alertEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::yChanged(int arg);
impl /*struct*/ QWindow {
  pub fn yChanged<RetType, T: QWindow_yChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.yChanged(self);
    // return 1;
  }
}

pub trait QWindow_yChanged<RetType> {
  fn yChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::yChanged(int arg);
impl<'a> /*trait*/ QWindow_yChanged<()> for (i32) {
  fn yChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8yChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow8yChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPlatformWindow * QWindow::handle();
impl /*struct*/ QWindow {
  pub fn handle<RetType, T: QWindow_handle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.handle(self);
    // return 1;
  }
}

pub trait QWindow_handle<RetType> {
  fn handle(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QPlatformWindow * QWindow::handle();
impl<'a> /*trait*/ QWindow_handle<()> for () {
  fn handle(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6handleEv()};
     unsafe {_ZNK7QWindow6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::destroy();
impl /*struct*/ QWindow {
  pub fn destroy<RetType, T: QWindow_destroy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.destroy(self);
    // return 1;
  }
}

pub trait QWindow_destroy<RetType> {
  fn destroy(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::destroy();
impl<'a> /*trait*/ QWindow_destroy<()> for () {
  fn destroy(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow7destroyEv()};
     unsafe {_ZN7QWindow7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QWindow * QWindow::transientParent();
impl /*struct*/ QWindow {
  pub fn transientParent<RetType, T: QWindow_transientParent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transientParent(self);
    // return 1;
  }
}

pub trait QWindow_transientParent<RetType> {
  fn transientParent(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QWindow * QWindow::transientParent();
impl<'a> /*trait*/ QWindow_transientParent<QWindow> for () {
  fn transientParent(self , rsthis: & QWindow) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow15transientParentEv()};
    let mut ret = unsafe {_ZNK7QWindow15transientParentEv(rsthis.qclsinst)};
    let mut ret1 = QWindow::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setMinimumHeight(int h);
impl /*struct*/ QWindow {
  pub fn setMinimumHeight<RetType, T: QWindow_setMinimumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumHeight(self);
    // return 1;
  }
}

pub trait QWindow_setMinimumHeight<RetType> {
  fn setMinimumHeight(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setMinimumHeight(int h);
impl<'a> /*trait*/ QWindow_setMinimumHeight<()> for (i32) {
  fn setMinimumHeight(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setMinimumHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow16setMinimumHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::show();
impl /*struct*/ QWindow {
  pub fn show<RetType, T: QWindow_show<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.show(self);
    // return 1;
  }
}

pub trait QWindow_show<RetType> {
  fn show(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::show();
impl<'a> /*trait*/ QWindow_show<()> for () {
  fn show(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4showEv()};
     unsafe {_ZN7QWindow4showEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::minimumWidthChanged(int arg);
impl /*struct*/ QWindow {
  pub fn minimumWidthChanged<RetType, T: QWindow_minimumWidthChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumWidthChanged(self);
    // return 1;
  }
}

pub trait QWindow_minimumWidthChanged<RetType> {
  fn minimumWidthChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::minimumWidthChanged(int arg);
impl<'a> /*trait*/ QWindow_minimumWidthChanged<()> for (i32) {
  fn minimumWidthChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow19minimumWidthChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow19minimumWidthChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QWindow::baseSize();
impl /*struct*/ QWindow {
  pub fn baseSize<RetType, T: QWindow_baseSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.baseSize(self);
    // return 1;
  }
}

pub trait QWindow_baseSize<RetType> {
  fn baseSize(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QSize QWindow::baseSize();
impl<'a> /*trait*/ QWindow_baseSize<QSize> for () {
  fn baseSize(self , rsthis: & QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8baseSizeEv()};
    let mut ret = unsafe {_ZNK7QWindow8baseSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QWindow::title();
impl /*struct*/ QWindow {
  pub fn title<RetType, T: QWindow_title<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.title(self);
    // return 1;
  }
}

pub trait QWindow_title<RetType> {
  fn title(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QString QWindow::title();
impl<'a> /*trait*/ QWindow_title<QString> for () {
  fn title(self , rsthis: & QWindow) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow5titleEv()};
    let mut ret = unsafe {_ZNK7QWindow5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::showMaximized();
impl /*struct*/ QWindow {
  pub fn showMaximized<RetType, T: QWindow_showMaximized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showMaximized(self);
    // return 1;
  }
}

pub trait QWindow_showMaximized<RetType> {
  fn showMaximized(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::showMaximized();
impl<'a> /*trait*/ QWindow_showMaximized<()> for () {
  fn showMaximized(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13showMaximizedEv()};
     unsafe {_ZN7QWindow13showMaximizedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::create();
impl /*struct*/ QWindow {
  pub fn create<RetType, T: QWindow_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QWindow_create<RetType> {
  fn create(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::create();
impl<'a> /*trait*/ QWindow_create<()> for () {
  fn create(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow6createEv()};
     unsafe {_ZN7QWindow6createEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::resize(int w, int h);
impl<'a> /*trait*/ QWindow_resize<()> for (i32, i32) {
  fn resize(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow6resizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QWindow6resizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QScreen * QWindow::screen();
impl /*struct*/ QWindow {
  pub fn screen<RetType, T: QWindow_screen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screen(self);
    // return 1;
  }
}

pub trait QWindow_screen<RetType> {
  fn screen(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QScreen * QWindow::screen();
impl<'a> /*trait*/ QWindow_screen<QScreen> for () {
  fn screen(self , rsthis: & QWindow) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6screenEv()};
    let mut ret = unsafe {_ZNK7QWindow6screenEv(rsthis.qclsinst)};
    let mut ret1 = QScreen::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setPosition(int posx, int posy);
impl<'a> /*trait*/ QWindow_setPosition<()> for (i32, i32) {
  fn setPosition(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setPositionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QWindow11setPositionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QWindow::setOpacity(qreal level);
impl /*struct*/ QWindow {
  pub fn setOpacity<RetType, T: QWindow_setOpacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpacity(self);
    // return 1;
  }
}

pub trait QWindow_setOpacity<RetType> {
  fn setOpacity(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setOpacity(qreal level);
impl<'a> /*trait*/ QWindow_setOpacity<()> for (f64) {
  fn setOpacity(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QWindow10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setGeometry(const QRect & rect);
impl<'a> /*trait*/ QWindow_setGeometry<()> for (&'a QRect) {
  fn setGeometry(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setSizeIncrement(const QSize & size);
impl /*struct*/ QWindow {
  pub fn setSizeIncrement<RetType, T: QWindow_setSizeIncrement<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSizeIncrement(self);
    // return 1;
  }
}

pub trait QWindow_setSizeIncrement<RetType> {
  fn setSizeIncrement(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setSizeIncrement(const QSize & size);
impl<'a> /*trait*/ QWindow_setSizeIncrement<()> for (&'a QSize) {
  fn setSizeIncrement(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setSizeIncrementERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow16setSizeIncrementERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::showMinimized();
impl /*struct*/ QWindow {
  pub fn showMinimized<RetType, T: QWindow_showMinimized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showMinimized(self);
    // return 1;
  }
}

pub trait QWindow_showMinimized<RetType> {
  fn showMinimized(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::showMinimized();
impl<'a> /*trait*/ QWindow_showMinimized<()> for () {
  fn showMinimized(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13showMinimizedEv()};
     unsafe {_ZN7QWindow13showMinimizedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QObject * QWindow::focusObject();
impl /*struct*/ QWindow {
  pub fn focusObject<RetType, T: QWindow_focusObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusObject(self);
    // return 1;
  }
}

pub trait QWindow_focusObject<RetType> {
  fn focusObject(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QObject * QWindow::focusObject();
impl<'a> /*trait*/ QWindow_focusObject<QObject> for () {
  fn focusObject(self , rsthis: & QWindow) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11focusObjectEv()};
    let mut ret = unsafe {_ZNK7QWindow11focusObjectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWindow::isActive();
impl /*struct*/ QWindow {
  pub fn isActive<RetType, T: QWindow_isActive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isActive(self);
    // return 1;
  }
}

pub trait QWindow_isActive<RetType> {
  fn isActive(self , rsthis: & QWindow) -> RetType;
}

  // proto:  bool QWindow::isActive();
impl<'a> /*trait*/ QWindow_isActive<i8> for () {
  fn isActive(self , rsthis: & QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8isActiveEv()};
    let mut ret = unsafe {_ZNK7QWindow8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWindow::maximumHeightChanged(int arg);
impl /*struct*/ QWindow {
  pub fn maximumHeightChanged<RetType, T: QWindow_maximumHeightChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumHeightChanged(self);
    // return 1;
  }
}

pub trait QWindow_maximumHeightChanged<RetType> {
  fn maximumHeightChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::maximumHeightChanged(int arg);
impl<'a> /*trait*/ QWindow_maximumHeightChanged<()> for (i32) {
  fn maximumHeightChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow20maximumHeightChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow20maximumHeightChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QWindow::accessibleRoot();
impl /*struct*/ QWindow {
  pub fn accessibleRoot<RetType, T: QWindow_accessibleRoot<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.accessibleRoot(self);
    // return 1;
  }
}

pub trait QWindow_accessibleRoot<RetType> {
  fn accessibleRoot(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QAccessibleInterface * QWindow::accessibleRoot();
impl<'a> /*trait*/ QWindow_accessibleRoot<QAccessibleInterface> for () {
  fn accessibleRoot(self , rsthis: & QWindow) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow14accessibleRootEv()};
    let mut ret = unsafe {_ZNK7QWindow14accessibleRootEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QCursor QWindow::cursor();
impl /*struct*/ QWindow {
  pub fn cursor<RetType, T: QWindow_cursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursor(self);
    // return 1;
  }
}

pub trait QWindow_cursor<RetType> {
  fn cursor(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QCursor QWindow::cursor();
impl<'a> /*trait*/ QWindow_cursor<QCursor> for () {
  fn cursor(self , rsthis: & QWindow) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6cursorEv()};
    let mut ret = unsafe {_ZNK7QWindow6cursorEv(rsthis.qclsinst)};
    let mut ret1 = QCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setFormat(const QSurfaceFormat & format);
impl /*struct*/ QWindow {
  pub fn setFormat<RetType, T: QWindow_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QWindow_setFormat<RetType> {
  fn setFormat(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QWindow_setFormat<()> for (&'a QSurfaceFormat) {
  fn setFormat(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::showFullScreen();
impl /*struct*/ QWindow {
  pub fn showFullScreen<RetType, T: QWindow_showFullScreen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showFullScreen(self);
    // return 1;
  }
}

pub trait QWindow_showFullScreen<RetType> {
  fn showFullScreen(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::showFullScreen();
impl<'a> /*trait*/ QWindow_showFullScreen<()> for () {
  fn showFullScreen(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14showFullScreenEv()};
     unsafe {_ZN7QWindow14showFullScreenEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::setX(int arg);
impl /*struct*/ QWindow {
  pub fn setX<RetType, T: QWindow_setX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QWindow_setX<RetType> {
  fn setX(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setX(int arg);
impl<'a> /*trait*/ QWindow_setX<()> for (i32) {
  fn setX(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4setXEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow4setXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::opacityChanged(qreal opacity);
impl /*struct*/ QWindow {
  pub fn opacityChanged<RetType, T: QWindow_opacityChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opacityChanged(self);
    // return 1;
  }
}

pub trait QWindow_opacityChanged<RetType> {
  fn opacityChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::opacityChanged(qreal opacity);
impl<'a> /*trait*/ QWindow_opacityChanged<()> for (f64) {
  fn opacityChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14opacityChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QWindow14opacityChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::lower();
impl /*struct*/ QWindow {
  pub fn lower<RetType, T: QWindow_lower<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lower(self);
    // return 1;
  }
}

pub trait QWindow_lower<RetType> {
  fn lower(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::lower();
impl<'a> /*trait*/ QWindow_lower<()> for () {
  fn lower(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5lowerEv()};
     unsafe {_ZN7QWindow5lowerEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::requestUpdate();
impl /*struct*/ QWindow {
  pub fn requestUpdate<RetType, T: QWindow_requestUpdate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.requestUpdate(self);
    // return 1;
  }
}

pub trait QWindow_requestUpdate<RetType> {
  fn requestUpdate(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::requestUpdate();
impl<'a> /*trait*/ QWindow_requestUpdate<()> for () {
  fn requestUpdate(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13requestUpdateEv()};
     unsafe {_ZN7QWindow13requestUpdateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::hide();
impl /*struct*/ QWindow {
  pub fn hide<RetType, T: QWindow_hide<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hide(self);
    // return 1;
  }
}

pub trait QWindow_hide<RetType> {
  fn hide(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::hide();
impl<'a> /*trait*/ QWindow_hide<()> for () {
  fn hide(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4hideEv()};
     unsafe {_ZN7QWindow4hideEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::minimumHeightChanged(int arg);
impl /*struct*/ QWindow {
  pub fn minimumHeightChanged<RetType, T: QWindow_minimumHeightChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumHeightChanged(self);
    // return 1;
  }
}

pub trait QWindow_minimumHeightChanged<RetType> {
  fn minimumHeightChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::minimumHeightChanged(int arg);
impl<'a> /*trait*/ QWindow_minimumHeightChanged<()> for (i32) {
  fn minimumHeightChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow20minimumHeightChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow20minimumHeightChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setMask(const QRegion & region);
impl /*struct*/ QWindow {
  pub fn setMask<RetType, T: QWindow_setMask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMask(self);
    // return 1;
  }
}

pub trait QWindow_setMask<RetType> {
  fn setMask(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setMask(const QRegion & region);
impl<'a> /*trait*/ QWindow_setMask<()> for (&'a QRegion) {
  fn setMask(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow7setMaskERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow7setMaskERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setMaximumSize(const QSize & size);
impl /*struct*/ QWindow {
  pub fn setMaximumSize<RetType, T: QWindow_setMaximumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumSize(self);
    // return 1;
  }
}

pub trait QWindow_setMaximumSize<RetType> {
  fn setMaximumSize(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setMaximumSize(const QSize & size);
impl<'a> /*trait*/ QWindow_setMaximumSize<()> for (&'a QSize) {
  fn setMaximumSize(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14setMaximumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow14setMaximumSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWindow::height();
impl /*struct*/ QWindow {
  pub fn height<RetType, T: QWindow_height<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QWindow_height<RetType> {
  fn height(self , rsthis: & QWindow) -> RetType;
}

  // proto:  int QWindow::height();
impl<'a> /*trait*/ QWindow_height<i32> for () {
  fn height(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6heightEv()};
    let mut ret = unsafe {demth_ZNK7QWindow6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QWindow::size();
impl /*struct*/ QWindow {
  pub fn size<RetType, T: QWindow_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QWindow_size<RetType> {
  fn size(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QSize QWindow::size();
impl<'a> /*trait*/ QWindow_size<QSize> for () {
  fn size(self , rsthis: & QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow4sizeEv()};
    let mut ret = unsafe {demth_ZNK7QWindow4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QWindow::maximumWidth();
impl /*struct*/ QWindow {
  pub fn maximumWidth<RetType, T: QWindow_maximumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumWidth(self);
    // return 1;
  }
}

pub trait QWindow_maximumWidth<RetType> {
  fn maximumWidth(self , rsthis: & QWindow) -> RetType;
}

  // proto:  int QWindow::maximumWidth();
impl<'a> /*trait*/ QWindow_maximumWidth<i32> for () {
  fn maximumWidth(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow12maximumWidthEv()};
    let mut ret = unsafe {demth_ZNK7QWindow12maximumWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPoint QWindow::position();
impl /*struct*/ QWindow {
  pub fn position<RetType, T: QWindow_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QWindow_position<RetType> {
  fn position(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QPoint QWindow::position();
impl<'a> /*trait*/ QWindow_position<QPoint> for () {
  fn position(self , rsthis: & QWindow) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8positionEv()};
    let mut ret = unsafe {demth_ZNK7QWindow8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setMaximumHeight(int h);
impl /*struct*/ QWindow {
  pub fn setMaximumHeight<RetType, T: QWindow_setMaximumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumHeight(self);
    // return 1;
  }
}

pub trait QWindow_setMaximumHeight<RetType> {
  fn setMaximumHeight(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::setMaximumHeight(int h);
impl<'a> /*trait*/ QWindow_setMaximumHeight<()> for (i32) {
  fn setMaximumHeight(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setMaximumHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow16setMaximumHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QWindow::filePath();
impl /*struct*/ QWindow {
  pub fn filePath<RetType, T: QWindow_filePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.filePath(self);
    // return 1;
  }
}

pub trait QWindow_filePath<RetType> {
  fn filePath(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QString QWindow::filePath();
impl<'a> /*trait*/ QWindow_filePath<QString> for () {
  fn filePath(self , rsthis: & QWindow) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8filePathEv()};
    let mut ret = unsafe {_ZNK7QWindow8filePathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::showNormal();
impl /*struct*/ QWindow {
  pub fn showNormal<RetType, T: QWindow_showNormal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showNormal(self);
    // return 1;
  }
}

pub trait QWindow_showNormal<RetType> {
  fn showNormal(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::showNormal();
impl<'a> /*trait*/ QWindow_showNormal<()> for () {
  fn showNormal(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow10showNormalEv()};
     unsafe {_ZN7QWindow10showNormalEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QWindow::framePosition();
impl /*struct*/ QWindow {
  pub fn framePosition<RetType, T: QWindow_framePosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.framePosition(self);
    // return 1;
  }
}

pub trait QWindow_framePosition<RetType> {
  fn framePosition(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QPoint QWindow::framePosition();
impl<'a> /*trait*/ QWindow_framePosition<QPoint> for () {
  fn framePosition(self , rsthis: & QWindow) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13framePositionEv()};
    let mut ret = unsafe {_ZNK7QWindow13framePositionEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::visibleChanged(bool arg);
impl /*struct*/ QWindow {
  pub fn visibleChanged<RetType, T: QWindow_visibleChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visibleChanged(self);
    // return 1;
  }
}

pub trait QWindow_visibleChanged<RetType> {
  fn visibleChanged(self , rsthis: & QWindow) -> RetType;
}

  // proto:  void QWindow::visibleChanged(bool arg);
impl<'a> /*trait*/ QWindow_visibleChanged<()> for (i8) {
  fn visibleChanged(self , rsthis: & QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14visibleChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QWindow14visibleChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIcon QWindow::icon();
impl /*struct*/ QWindow {
  pub fn icon<RetType, T: QWindow_icon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.icon(self);
    // return 1;
  }
}

pub trait QWindow_icon<RetType> {
  fn icon(self , rsthis: & QWindow) -> RetType;
}

  // proto:  QIcon QWindow::icon();
impl<'a> /*trait*/ QWindow_icon<QIcon> for () {
  fn icon(self , rsthis: & QWindow) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow4iconEv()};
    let mut ret = unsafe {_ZNK7QWindow4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QWindow_modalityChanged
pub struct QWindow_modalityChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn modalityChanged_1(&self) -> QWindow_modalityChanged_signal {
     return QWindow_modalityChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_modalityChanged_signal {
  pub fn connect<T: QWindow_modalityChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_modalityChanged_signal_connect {
  fn connect(self, sigthis: QWindow_modalityChanged_signal);
}

#[derive(Default)] // for QWindow_activeChanged
pub struct QWindow_activeChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn activeChanged_1(&self) -> QWindow_activeChanged_signal {
     return QWindow_activeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_activeChanged_signal {
  pub fn connect<T: QWindow_activeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_activeChanged_signal_connect {
  fn connect(self, sigthis: QWindow_activeChanged_signal);
}

#[derive(Default)] // for QWindow_heightChanged
pub struct QWindow_heightChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn heightChanged_1(&self) -> QWindow_heightChanged_signal {
     return QWindow_heightChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_heightChanged_signal {
  pub fn connect<T: QWindow_heightChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_heightChanged_signal_connect {
  fn connect(self, sigthis: QWindow_heightChanged_signal);
}

#[derive(Default)] // for QWindow_contentOrientationChanged
pub struct QWindow_contentOrientationChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn contentOrientationChanged_1(&self) -> QWindow_contentOrientationChanged_signal {
     return QWindow_contentOrientationChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_contentOrientationChanged_signal {
  pub fn connect<T: QWindow_contentOrientationChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_contentOrientationChanged_signal_connect {
  fn connect(self, sigthis: QWindow_contentOrientationChanged_signal);
}

#[derive(Default)] // for QWindow_minimumWidthChanged
pub struct QWindow_minimumWidthChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn minimumWidthChanged_1(&self) -> QWindow_minimumWidthChanged_signal {
     return QWindow_minimumWidthChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_minimumWidthChanged_signal {
  pub fn connect<T: QWindow_minimumWidthChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_minimumWidthChanged_signal_connect {
  fn connect(self, sigthis: QWindow_minimumWidthChanged_signal);
}

#[derive(Default)] // for QWindow_opacityChanged
pub struct QWindow_opacityChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn opacityChanged_1(&self) -> QWindow_opacityChanged_signal {
     return QWindow_opacityChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_opacityChanged_signal {
  pub fn connect<T: QWindow_opacityChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_opacityChanged_signal_connect {
  fn connect(self, sigthis: QWindow_opacityChanged_signal);
}

#[derive(Default)] // for QWindow_visibleChanged
pub struct QWindow_visibleChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn visibleChanged_1(&self) -> QWindow_visibleChanged_signal {
     return QWindow_visibleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_visibleChanged_signal {
  pub fn connect<T: QWindow_visibleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_visibleChanged_signal_connect {
  fn connect(self, sigthis: QWindow_visibleChanged_signal);
}

#[derive(Default)] // for QWindow_screenChanged
pub struct QWindow_screenChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn screenChanged_1(&self) -> QWindow_screenChanged_signal {
     return QWindow_screenChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_screenChanged_signal {
  pub fn connect<T: QWindow_screenChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_screenChanged_signal_connect {
  fn connect(self, sigthis: QWindow_screenChanged_signal);
}

#[derive(Default)] // for QWindow_maximumHeightChanged
pub struct QWindow_maximumHeightChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn maximumHeightChanged_1(&self) -> QWindow_maximumHeightChanged_signal {
     return QWindow_maximumHeightChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_maximumHeightChanged_signal {
  pub fn connect<T: QWindow_maximumHeightChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_maximumHeightChanged_signal_connect {
  fn connect(self, sigthis: QWindow_maximumHeightChanged_signal);
}

#[derive(Default)] // for QWindow_yChanged
pub struct QWindow_yChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn yChanged_1(&self) -> QWindow_yChanged_signal {
     return QWindow_yChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_yChanged_signal {
  pub fn connect<T: QWindow_yChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_yChanged_signal_connect {
  fn connect(self, sigthis: QWindow_yChanged_signal);
}

#[derive(Default)] // for QWindow_widthChanged
pub struct QWindow_widthChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn widthChanged_1(&self) -> QWindow_widthChanged_signal {
     return QWindow_widthChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_widthChanged_signal {
  pub fn connect<T: QWindow_widthChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_widthChanged_signal_connect {
  fn connect(self, sigthis: QWindow_widthChanged_signal);
}

#[derive(Default)] // for QWindow_windowStateChanged
pub struct QWindow_windowStateChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn windowStateChanged_1(&self) -> QWindow_windowStateChanged_signal {
     return QWindow_windowStateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_windowStateChanged_signal {
  pub fn connect<T: QWindow_windowStateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_windowStateChanged_signal_connect {
  fn connect(self, sigthis: QWindow_windowStateChanged_signal);
}

#[derive(Default)] // for QWindow_windowTitleChanged
pub struct QWindow_windowTitleChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn windowTitleChanged_1(&self) -> QWindow_windowTitleChanged_signal {
     return QWindow_windowTitleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_windowTitleChanged_signal {
  pub fn connect<T: QWindow_windowTitleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_windowTitleChanged_signal_connect {
  fn connect(self, sigthis: QWindow_windowTitleChanged_signal);
}

#[derive(Default)] // for QWindow_visibilityChanged
pub struct QWindow_visibilityChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn visibilityChanged_1(&self) -> QWindow_visibilityChanged_signal {
     return QWindow_visibilityChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_visibilityChanged_signal {
  pub fn connect<T: QWindow_visibilityChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_visibilityChanged_signal_connect {
  fn connect(self, sigthis: QWindow_visibilityChanged_signal);
}

#[derive(Default)] // for QWindow_minimumHeightChanged
pub struct QWindow_minimumHeightChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn minimumHeightChanged_1(&self) -> QWindow_minimumHeightChanged_signal {
     return QWindow_minimumHeightChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_minimumHeightChanged_signal {
  pub fn connect<T: QWindow_minimumHeightChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_minimumHeightChanged_signal_connect {
  fn connect(self, sigthis: QWindow_minimumHeightChanged_signal);
}

#[derive(Default)] // for QWindow_xChanged
pub struct QWindow_xChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn xChanged_1(&self) -> QWindow_xChanged_signal {
     return QWindow_xChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_xChanged_signal {
  pub fn connect<T: QWindow_xChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_xChanged_signal_connect {
  fn connect(self, sigthis: QWindow_xChanged_signal);
}

#[derive(Default)] // for QWindow_focusObjectChanged
pub struct QWindow_focusObjectChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn focusObjectChanged_1(&self) -> QWindow_focusObjectChanged_signal {
     return QWindow_focusObjectChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_focusObjectChanged_signal {
  pub fn connect<T: QWindow_focusObjectChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_focusObjectChanged_signal_connect {
  fn connect(self, sigthis: QWindow_focusObjectChanged_signal);
}

#[derive(Default)] // for QWindow_maximumWidthChanged
pub struct QWindow_maximumWidthChanged_signal{poi:u64}
impl /* struct */ QWindow {
  pub fn maximumWidthChanged_1(&self) -> QWindow_maximumWidthChanged_signal {
     return QWindow_maximumWidthChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWindow_maximumWidthChanged_signal {
  pub fn connect<T: QWindow_maximumWidthChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWindow_maximumWidthChanged_signal_connect {
  fn connect(self, sigthis: QWindow_maximumWidthChanged_signal);
}

// xChanged(int)
extern fn QWindow_xChanged_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWindow_xChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_xChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWindow_xChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_xChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow8xChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_xChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWindow_xChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_xChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow8xChangedEi(arg0, arg1, arg2)};
  }
}
// widthChanged(int)
extern fn QWindow_widthChanged_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWindow_widthChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_widthChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWindow_widthChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_widthChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow12widthChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_widthChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWindow_widthChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_widthChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow12widthChangedEi(arg0, arg1, arg2)};
  }
}
// contentOrientationChanged(Qt::ScreenOrientation)
extern fn QWindow_contentOrientationChanged_signal_connect_cb_2(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWindow_contentOrientationChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_contentOrientationChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWindow_contentOrientationChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_contentOrientationChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow25contentOrientationChangedEN2Qt17ScreenOrientationE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_contentOrientationChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWindow_contentOrientationChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_contentOrientationChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow25contentOrientationChangedEN2Qt17ScreenOrientationE(arg0, arg1, arg2)};
  }
}
// modalityChanged(Qt::WindowModality)
extern fn QWindow_modalityChanged_signal_connect_cb_3(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWindow_modalityChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_modalityChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWindow_modalityChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_modalityChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow15modalityChangedEN2Qt14WindowModalityE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_modalityChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWindow_modalityChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_modalityChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow15modalityChangedEN2Qt14WindowModalityE(arg0, arg1, arg2)};
  }
}
// windowStateChanged(Qt::WindowState)
extern fn QWindow_windowStateChanged_signal_connect_cb_4(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWindow_windowStateChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_windowStateChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWindow_windowStateChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_windowStateChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow18windowStateChangedEN2Qt11WindowStateE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_windowStateChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWindow_windowStateChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_windowStateChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow18windowStateChangedEN2Qt11WindowStateE(arg0, arg1, arg2)};
  }
}
// visibilityChanged(class QWindow::Visibility)
extern fn QWindow_visibilityChanged_signal_connect_cb_5(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWindow_visibilityChanged_signal_connect_cb_box_5(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_visibilityChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWindow_visibilityChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_visibilityChanged_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow17visibilityChangedENS_10VisibilityE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_visibilityChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWindow_visibilityChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_visibilityChanged_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow17visibilityChangedENS_10VisibilityE(arg0, arg1, arg2)};
  }
}
// opacityChanged(qreal)
extern fn QWindow_opacityChanged_signal_connect_cb_6(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QWindow_opacityChanged_signal_connect_cb_box_6(rsfptr_raw:*mut Fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_opacityChanged_signal_connect for fn(f64) {
  fn connect(self, sigthis: QWindow_opacityChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_opacityChanged_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow14opacityChangedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_opacityChanged_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QWindow_opacityChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_opacityChanged_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow14opacityChangedEd(arg0, arg1, arg2)};
  }
}
// yChanged(int)
extern fn QWindow_yChanged_signal_connect_cb_7(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWindow_yChanged_signal_connect_cb_box_7(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_yChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWindow_yChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_yChanged_signal_connect_cb_7 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow8yChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_yChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWindow_yChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_yChanged_signal_connect_cb_box_7 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow8yChangedEi(arg0, arg1, arg2)};
  }
}
// minimumHeightChanged(int)
extern fn QWindow_minimumHeightChanged_signal_connect_cb_8(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWindow_minimumHeightChanged_signal_connect_cb_box_8(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_minimumHeightChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWindow_minimumHeightChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_minimumHeightChanged_signal_connect_cb_8 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow20minimumHeightChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_minimumHeightChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWindow_minimumHeightChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_minimumHeightChanged_signal_connect_cb_box_8 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow20minimumHeightChangedEi(arg0, arg1, arg2)};
  }
}
// heightChanged(int)
extern fn QWindow_heightChanged_signal_connect_cb_9(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWindow_heightChanged_signal_connect_cb_box_9(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_heightChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWindow_heightChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_heightChanged_signal_connect_cb_9 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow13heightChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_heightChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWindow_heightChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_heightChanged_signal_connect_cb_box_9 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow13heightChangedEi(arg0, arg1, arg2)};
  }
}
// maximumWidthChanged(int)
extern fn QWindow_maximumWidthChanged_signal_connect_cb_10(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWindow_maximumWidthChanged_signal_connect_cb_box_10(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_maximumWidthChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWindow_maximumWidthChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_maximumWidthChanged_signal_connect_cb_10 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow19maximumWidthChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_maximumWidthChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWindow_maximumWidthChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_maximumWidthChanged_signal_connect_cb_box_10 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow19maximumWidthChangedEi(arg0, arg1, arg2)};
  }
}
// screenChanged(class QScreen *)
extern fn QWindow_screenChanged_signal_connect_cb_11(rsfptr:fn(QScreen), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QScreen::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QWindow_screenChanged_signal_connect_cb_box_11(rsfptr_raw:*mut Fn(QScreen), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QScreen::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_screenChanged_signal_connect for fn(QScreen) {
  fn connect(self, sigthis: QWindow_screenChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_screenChanged_signal_connect_cb_11 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow13screenChangedEP7QScreen(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_screenChanged_signal_connect for Box<Fn(QScreen)> {
  fn connect(self, sigthis: QWindow_screenChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_screenChanged_signal_connect_cb_box_11 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow13screenChangedEP7QScreen(arg0, arg1, arg2)};
  }
}
// minimumWidthChanged(int)
extern fn QWindow_minimumWidthChanged_signal_connect_cb_12(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWindow_minimumWidthChanged_signal_connect_cb_box_12(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_minimumWidthChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWindow_minimumWidthChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_minimumWidthChanged_signal_connect_cb_12 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow19minimumWidthChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_minimumWidthChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWindow_minimumWidthChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_minimumWidthChanged_signal_connect_cb_box_12 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow19minimumWidthChangedEi(arg0, arg1, arg2)};
  }
}
// windowTitleChanged(const class QString &)
extern fn QWindow_windowTitleChanged_signal_connect_cb_13(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QWindow_windowTitleChanged_signal_connect_cb_box_13(rsfptr_raw:*mut Fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_windowTitleChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QWindow_windowTitleChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_windowTitleChanged_signal_connect_cb_13 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow18windowTitleChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_windowTitleChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QWindow_windowTitleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_windowTitleChanged_signal_connect_cb_box_13 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow18windowTitleChangedERK7QString(arg0, arg1, arg2)};
  }
}
// focusObjectChanged(class QObject *)
extern fn QWindow_focusObjectChanged_signal_connect_cb_14(rsfptr:fn(QObject), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QObject::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QWindow_focusObjectChanged_signal_connect_cb_box_14(rsfptr_raw:*mut Fn(QObject), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QObject::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_focusObjectChanged_signal_connect for fn(QObject) {
  fn connect(self, sigthis: QWindow_focusObjectChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_focusObjectChanged_signal_connect_cb_14 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow18focusObjectChangedEP7QObject(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_focusObjectChanged_signal_connect for Box<Fn(QObject)> {
  fn connect(self, sigthis: QWindow_focusObjectChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_focusObjectChanged_signal_connect_cb_box_14 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow18focusObjectChangedEP7QObject(arg0, arg1, arg2)};
  }
}
// visibleChanged(_Bool)
extern fn QWindow_visibleChanged_signal_connect_cb_15(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QWindow_visibleChanged_signal_connect_cb_box_15(rsfptr_raw:*mut Fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_visibleChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QWindow_visibleChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_visibleChanged_signal_connect_cb_15 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow14visibleChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_visibleChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QWindow_visibleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_visibleChanged_signal_connect_cb_box_15 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow14visibleChangedEb(arg0, arg1, arg2)};
  }
}
// maximumHeightChanged(int)
extern fn QWindow_maximumHeightChanged_signal_connect_cb_16(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWindow_maximumHeightChanged_signal_connect_cb_box_16(rsfptr_raw:*mut Fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QWindow_maximumHeightChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWindow_maximumHeightChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_maximumHeightChanged_signal_connect_cb_16 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow20maximumHeightChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_maximumHeightChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWindow_maximumHeightChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_maximumHeightChanged_signal_connect_cb_box_16 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow20maximumHeightChangedEi(arg0, arg1, arg2)};
  }
}
// activeChanged()
extern fn QWindow_activeChanged_signal_connect_cb_17(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QWindow_activeChanged_signal_connect_cb_box_17(rsfptr_raw:*mut Fn(), ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  rsfptr();
}
impl /* trait */ QWindow_activeChanged_signal_connect for fn() {
  fn connect(self, sigthis: QWindow_activeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_activeChanged_signal_connect_cb_17 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow13activeChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWindow_activeChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QWindow_activeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWindow_activeChanged_signal_connect_cb_box_17 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QWindow_SlotProxy_connect__ZN7QWindow13activeChangedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

