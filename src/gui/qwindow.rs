// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qscreen::QScreen;
use super::qsize::QSize;
use super::qsurfaceformat::QSurfaceFormat;
use super::qicon::QIcon;
use super::qstring::QString;
use super::qpoint::QPoint;
use super::qmargins::QMargins;
use super::qobject::QObject;
use super::qrect::QRect;
use super::qcursor::QCursor;
use super::qregion::QRegion;
use super::qaccessibleinterface::QAccessibleInterface;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QWindow::xChanged(int arg);
  fn _ZN7QWindow8xChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QWindow::unsetCursor();
  fn _ZN7QWindow11unsetCursorEv(qthis: *mut c_void);
  // proto:  bool QWindow::isVisible();
  fn _ZNK7QWindow9isVisibleEv(qthis: *mut c_void) -> c_char;
  // proto:  void QWindow::setScreen(QScreen * screen);
  fn _ZN7QWindow9setScreenEP7QScreen(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSize QWindow::maximumSize();
  fn _ZNK7QWindow11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setTransientParent(QWindow * parent);
  fn _ZN7QWindow18setTransientParentEPS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSurfaceFormat QWindow::format();
  fn _ZNK7QWindow6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWindow::isTopLevel();
  fn _ZNK7QWindow10isTopLevelEv(qthis: *mut c_void) -> c_char;
  // proto:  void QWindow::QWindow(const QWindow & );
  fn _ZN7QWindowC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::setIcon(const QIcon & icon);
  fn _ZN7QWindow7setIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QWindow::opacity();
  fn _ZNK7QWindow7opacityEv(qthis: *mut c_void) -> c_double;
  // proto:  void QWindow::setMinimumSize(const QSize & size);
  fn _ZN7QWindow14setMinimumSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QWindow::minimumHeight();
  fn _ZNK7QWindow13minimumHeightEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QWindow::sizeIncrement();
  fn _ZNK7QWindow13sizeIncrementEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::resize(const QSize & newSize);
  fn _ZN7QWindow6resizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::screenChanged(QScreen * screen);
  fn _ZN7QWindow13screenChangedEP7QScreen(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::setTitle(const QString & );
  fn _ZN7QWindow8setTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::raise();
  fn _ZN7QWindow5raiseEv(qthis: *mut c_void);
  // proto:  QSize QWindow::minimumSize();
  fn _ZNK7QWindow11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QWindow::mapToGlobal(const QPoint & pos);
  fn _ZNK7QWindow11mapToGlobalERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto: static QWindow * QWindow::fromWinId(WId id);
  fn _ZN7QWindow9fromWinIdEi(arg0: *mut c_uint) -> *mut c_void;
  // proto:  QMargins QWindow::frameMargins();
  fn _ZNK7QWindow12frameMarginsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setMaximumWidth(int w);
  fn _ZN7QWindow15setMaximumWidthEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QWindow::maximumHeight();
  fn _ZNK7QWindow13maximumHeightEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWindow::focusObjectChanged(QObject * object);
  fn _ZN7QWindow18focusObjectChangedEP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QWindow::isModal();
  fn _ZNK7QWindow7isModalEv(qthis: *mut c_void) -> c_char;
  // proto:  void QWindow::maximumWidthChanged(int arg);
  fn _ZN7QWindow19maximumWidthChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QRect QWindow::geometry();
  fn _ZNK7QWindow8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setParent(QWindow * parent);
  fn _ZN7QWindow9setParentEPS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRect QWindow::frameGeometry();
  fn _ZNK7QWindow13frameGeometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSurfaceFormat QWindow::requestedFormat();
  fn _ZNK7QWindow15requestedFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setHeight(int arg);
  fn _ZN7QWindow9setHeightEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QWindow::requestActivate();
  fn _ZN7QWindow15requestActivateEv(qthis: *mut c_void);
  // proto:  QPoint QWindow::mapFromGlobal(const QPoint & pos);
  fn _ZNK7QWindow13mapFromGlobalERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::windowTitleChanged(const QString & title);
  fn _ZN7QWindow18windowTitleChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QWindow::y();
  fn _ZNK7QWindow1yEv(qthis: *mut c_void);
  // proto:  int QWindow::width();
  fn _ZNK7QWindow5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWindow::setFilePath(const QString & filePath);
  fn _ZN7QWindow11setFilePathERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::setCursor(const QCursor & );
  fn _ZN7QWindow9setCursorERK7QCursor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::setVisible(bool visible);
  fn _ZN7QWindow10setVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QWindow::~QWindow();
  fn _ZN7QWindowD0Ev(qthis: *mut c_void);
  // proto:  bool QWindow::setMouseGrabEnabled(bool grab);
  fn _ZN7QWindow19setMouseGrabEnabledEb(qthis: *mut c_void, arg0: c_char) -> c_char;
  // proto:  bool QWindow::isExposed();
  fn _ZNK7QWindow9isExposedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QWindow::heightChanged(int arg);
  fn _ZN7QWindow13heightChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QWindow::minimumWidth();
  fn _ZNK7QWindow12minimumWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWindow::setPosition(const QPoint & pt);
  fn _ZN7QWindow11setPositionERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QWindow::close();
  fn _ZN7QWindow5closeEv(qthis: *mut c_void) -> c_char;
  // proto:  int QWindow::x();
  fn _ZNK7QWindow1xEv(qthis: *mut c_void);
  // proto:  void QWindow::setMinimumWidth(int w);
  fn _ZN7QWindow15setMinimumWidthEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QRegion QWindow::mask();
  fn _ZNK7QWindow4maskEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::widthChanged(int arg);
  fn _ZN7QWindow12widthChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QWindow * QWindow::parent();
  fn _ZNK7QWindow6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setFramePosition(const QPoint & point);
  fn _ZN7QWindow16setFramePositionERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::QWindow(QScreen * screen);
  fn _ZN7QWindowC1EP7QScreen(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::setGeometry(int posx, int posy, int w, int h);
  fn _ZN7QWindow11setGeometryEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  bool QWindow::setKeyboardGrabEnabled(bool grab);
  fn _ZN7QWindow22setKeyboardGrabEnabledEb(qthis: *mut c_void, arg0: c_char) -> c_char;
  // proto:  const QMetaObject * QWindow::metaObject();
  fn _ZNK7QWindow10metaObjectEv(qthis: *mut c_void);
  // proto:  void QWindow::QWindow(QWindow * parent);
  fn _ZN7QWindowC1EPS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::activeChanged();
  fn _ZN7QWindow13activeChangedEv(qthis: *mut c_void);
  // proto:  void QWindow::setWidth(int arg);
  fn _ZN7QWindow8setWidthEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QWindow::setY(int arg);
  fn _ZN7QWindow4setYEi(qthis: *mut c_void, arg0: c_int);
  // proto:  qreal QWindow::devicePixelRatio();
  fn _ZNK7QWindow16devicePixelRatioEv(qthis: *mut c_void) -> c_double;
  // proto:  void QWindow::setBaseSize(const QSize & size);
  fn _ZN7QWindow11setBaseSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::alert(int msec);
  fn _ZN7QWindow5alertEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QWindow::yChanged(int arg);
  fn _ZN7QWindow8yChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QPlatformWindow * QWindow::handle();
  fn _ZNK7QWindow6handleEv(qthis: *mut c_void);
  // proto:  void QWindow::destroy();
  fn _ZN7QWindow7destroyEv(qthis: *mut c_void);
  // proto:  QWindow * QWindow::transientParent();
  fn _ZNK7QWindow15transientParentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setMinimumHeight(int h);
  fn _ZN7QWindow16setMinimumHeightEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QWindow::show();
  fn _ZN7QWindow4showEv(qthis: *mut c_void);
  // proto:  void QWindow::minimumWidthChanged(int arg);
  fn _ZN7QWindow19minimumWidthChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QSize QWindow::baseSize();
  fn _ZNK7QWindow8baseSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QWindow::title();
  fn _ZNK7QWindow5titleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::showMaximized();
  fn _ZN7QWindow13showMaximizedEv(qthis: *mut c_void);
  // proto:  void QWindow::create();
  fn _ZN7QWindow6createEv(qthis: *mut c_void);
  // proto:  void QWindow::resize(int w, int h);
  fn _ZN7QWindow6resizeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  QScreen * QWindow::screen();
  fn _ZNK7QWindow6screenEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setPosition(int posx, int posy);
  fn _ZN7QWindow11setPositionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QWindow::setOpacity(qreal level);
  fn _ZN7QWindow10setOpacityEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QWindow::setGeometry(const QRect & rect);
  fn _ZN7QWindow11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::setSizeIncrement(const QSize & size);
  fn _ZN7QWindow16setSizeIncrementERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::showMinimized();
  fn _ZN7QWindow13showMinimizedEv(qthis: *mut c_void);
  // proto:  QObject * QWindow::focusObject();
  fn _ZNK7QWindow11focusObjectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWindow::isActive();
  fn _ZNK7QWindow8isActiveEv(qthis: *mut c_void) -> c_char;
  // proto:  void QWindow::maximumHeightChanged(int arg);
  fn _ZN7QWindow20maximumHeightChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QAccessibleInterface * QWindow::accessibleRoot();
  fn _ZNK7QWindow14accessibleRootEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QCursor QWindow::cursor();
  fn _ZNK7QWindow6cursorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setFormat(const QSurfaceFormat & format);
  fn _ZN7QWindow9setFormatERK14QSurfaceFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::showFullScreen();
  fn _ZN7QWindow14showFullScreenEv(qthis: *mut c_void);
  // proto:  void QWindow::setX(int arg);
  fn _ZN7QWindow4setXEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QWindow::opacityChanged(qreal opacity);
  fn _ZN7QWindow14opacityChangedEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QWindow::lower();
  fn _ZN7QWindow5lowerEv(qthis: *mut c_void);
  // proto:  void QWindow::requestUpdate();
  fn _ZN7QWindow13requestUpdateEv(qthis: *mut c_void);
  // proto:  void QWindow::hide();
  fn _ZN7QWindow4hideEv(qthis: *mut c_void);
  // proto:  void QWindow::minimumHeightChanged(int arg);
  fn _ZN7QWindow20minimumHeightChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QWindow::setMask(const QRegion & region);
  fn _ZN7QWindow7setMaskERK7QRegion(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWindow::setMaximumSize(const QSize & size);
  fn _ZN7QWindow14setMaximumSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QWindow::height();
  fn _ZNK7QWindow6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QWindow::size();
  fn _ZNK7QWindow4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QWindow::maximumWidth();
  fn _ZNK7QWindow12maximumWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  QPoint QWindow::position();
  fn _ZNK7QWindow8positionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setMaximumHeight(int h);
  fn _ZN7QWindow16setMaximumHeightEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QString QWindow::filePath();
  fn _ZNK7QWindow8filePathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::showNormal();
  fn _ZN7QWindow10showNormalEv(qthis: *mut c_void);
  // proto:  QPoint QWindow::framePosition();
  fn _ZNK7QWindow13framePositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::visibleChanged(bool arg);
  fn _ZN7QWindow14visibleChangedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QIcon QWindow::icon();
  fn _ZNK7QWindow4iconEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QWindow)=1
pub struct QWindow {
  pub qclsinst: *mut c_void,
}

  // proto:  void QWindow::xChanged(int arg);
impl /*struct*/ QWindow {
  pub fn xChanged<RetType, T: QWindow_xChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.xChanged(self);
    // return 1;
  }
}

pub trait QWindow_xChanged<RetType> {
  fn xChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::xChanged(int arg);
impl<'a> /*trait*/ QWindow_xChanged<()> for (i32) {
  fn xChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8xChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow8xChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::unsetCursor();
impl /*struct*/ QWindow {
  pub fn unsetCursor<RetType, T: QWindow_unsetCursor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unsetCursor(self);
    // return 1;
  }
}

pub trait QWindow_unsetCursor<RetType> {
  fn unsetCursor(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::unsetCursor();
impl<'a> /*trait*/ QWindow_unsetCursor<()> for () {
  fn unsetCursor(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11unsetCursorEv()};
     unsafe {_ZN7QWindow11unsetCursorEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QWindow::isVisible();
impl /*struct*/ QWindow {
  pub fn isVisible<RetType, T: QWindow_isVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isVisible(self);
    // return 1;
  }
}

pub trait QWindow_isVisible<RetType> {
  fn isVisible(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  bool QWindow::isVisible();
impl<'a> /*trait*/ QWindow_isVisible<i8> for () {
  fn isVisible(self , rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow9isVisibleEv()};
    let mut ret = unsafe {_ZNK7QWindow9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWindow::setScreen(QScreen * screen);
impl /*struct*/ QWindow {
  pub fn setScreen<RetType, T: QWindow_setScreen<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setScreen(self);
    // return 1;
  }
}

pub trait QWindow_setScreen<RetType> {
  fn setScreen(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setScreen(QScreen * screen);
impl<'a> /*trait*/ QWindow_setScreen<()> for (QScreen) {
  fn setScreen(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setScreenEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow9setScreenEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QWindow::maximumSize();
impl /*struct*/ QWindow {
  pub fn maximumSize<RetType, T: QWindow_maximumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QWindow_maximumSize<RetType> {
  fn maximumSize(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QSize QWindow::maximumSize();
impl<'a> /*trait*/ QWindow_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: &mut QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11maximumSizeEv()};
    let mut ret = unsafe {_ZNK7QWindow11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setTransientParent(QWindow * parent);
impl /*struct*/ QWindow {
  pub fn setTransientParent<RetType, T: QWindow_setTransientParent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTransientParent(self);
    // return 1;
  }
}

pub trait QWindow_setTransientParent<RetType> {
  fn setTransientParent(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setTransientParent(QWindow * parent);
impl<'a> /*trait*/ QWindow_setTransientParent<()> for (QWindow) {
  fn setTransientParent(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow18setTransientParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow18setTransientParentEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSurfaceFormat QWindow::format();
impl /*struct*/ QWindow {
  pub fn format<RetType, T: QWindow_format<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QWindow_format<RetType> {
  fn format(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QSurfaceFormat QWindow::format();
impl<'a> /*trait*/ QWindow_format<QSurfaceFormat> for () {
  fn format(self , rsthis: &mut QWindow) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6formatEv()};
    let mut ret = unsafe {_ZNK7QWindow6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWindow::isTopLevel();
impl /*struct*/ QWindow {
  pub fn isTopLevel<RetType, T: QWindow_isTopLevel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isTopLevel(self);
    // return 1;
  }
}

pub trait QWindow_isTopLevel<RetType> {
  fn isTopLevel(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  bool QWindow::isTopLevel();
impl<'a> /*trait*/ QWindow_isTopLevel<i8> for () {
  fn isTopLevel(self , rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow10isTopLevelEv()};
    let mut ret = unsafe {_ZNK7QWindow10isTopLevelEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWindow::QWindow(const QWindow & );
impl /*struct*/ QWindow {
  pub fn NewQWindow<T: QWindow_NewQWindow>(value: T) -> QWindow {
    let rsthis = value.NewQWindow();
    return rsthis;
    // return 1;
  }
}

pub trait QWindow_NewQWindow {
  fn NewQWindow(self) -> QWindow;
}

  // proto:  void QWindow::QWindow(const QWindow & );
impl<'a> /*trait*/ QWindow_NewQWindow for (QWindow) {
  fn NewQWindow(self) -> QWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindowC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWindowC1ERKS_(qthis, arg0)};
    let rsthis = QWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QWindow::setIcon(const QIcon & icon);
impl /*struct*/ QWindow {
  pub fn setIcon<RetType, T: QWindow_setIcon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIcon(self);
    // return 1;
  }
}

pub trait QWindow_setIcon<RetType> {
  fn setIcon(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QWindow_setIcon<()> for (QIcon) {
  fn setIcon(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QWindow::opacity();
impl /*struct*/ QWindow {
  pub fn opacity<RetType, T: QWindow_opacity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.opacity(self);
    // return 1;
  }
}

pub trait QWindow_opacity<RetType> {
  fn opacity(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  qreal QWindow::opacity();
impl<'a> /*trait*/ QWindow_opacity<f64> for () {
  fn opacity(self , rsthis: &mut QWindow) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow7opacityEv()};
    let mut ret = unsafe {_ZNK7QWindow7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QWindow::setMinimumSize(const QSize & size);
impl /*struct*/ QWindow {
  pub fn setMinimumSize<RetType, T: QWindow_setMinimumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMinimumSize(self);
    // return 1;
  }
}

pub trait QWindow_setMinimumSize<RetType> {
  fn setMinimumSize(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setMinimumSize(const QSize & size);
impl<'a> /*trait*/ QWindow_setMinimumSize<()> for (QSize) {
  fn setMinimumSize(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14setMinimumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow14setMinimumSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWindow::minimumHeight();
impl /*struct*/ QWindow {
  pub fn minimumHeight<RetType, T: QWindow_minimumHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumHeight(self);
    // return 1;
  }
}

pub trait QWindow_minimumHeight<RetType> {
  fn minimumHeight(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  int QWindow::minimumHeight();
impl<'a> /*trait*/ QWindow_minimumHeight<i32> for () {
  fn minimumHeight(self , rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13minimumHeightEv()};
    let mut ret = unsafe {_ZNK7QWindow13minimumHeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QWindow::sizeIncrement();
impl /*struct*/ QWindow {
  pub fn sizeIncrement<RetType, T: QWindow_sizeIncrement<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeIncrement(self);
    // return 1;
  }
}

pub trait QWindow_sizeIncrement<RetType> {
  fn sizeIncrement(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QSize QWindow::sizeIncrement();
impl<'a> /*trait*/ QWindow_sizeIncrement<QSize> for () {
  fn sizeIncrement(self , rsthis: &mut QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13sizeIncrementEv()};
    let mut ret = unsafe {_ZNK7QWindow13sizeIncrementEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::resize(const QSize & newSize);
impl /*struct*/ QWindow {
  pub fn resize<RetType, T: QWindow_resize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QWindow_resize<RetType> {
  fn resize(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::resize(const QSize & newSize);
impl<'a> /*trait*/ QWindow_resize<()> for (QSize) {
  fn resize(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow6resizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow6resizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::screenChanged(QScreen * screen);
impl /*struct*/ QWindow {
  pub fn screenChanged<RetType, T: QWindow_screenChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.screenChanged(self);
    // return 1;
  }
}

pub trait QWindow_screenChanged<RetType> {
  fn screenChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::screenChanged(QScreen * screen);
impl<'a> /*trait*/ QWindow_screenChanged<()> for (QScreen) {
  fn screenChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13screenChangedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow13screenChangedEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setTitle(const QString & );
impl /*struct*/ QWindow {
  pub fn setTitle<RetType, T: QWindow_setTitle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTitle(self);
    // return 1;
  }
}

pub trait QWindow_setTitle<RetType> {
  fn setTitle(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setTitle(const QString & );
impl<'a> /*trait*/ QWindow_setTitle<()> for (QString) {
  fn setTitle(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::raise();
impl /*struct*/ QWindow {
  pub fn raise<RetType, T: QWindow_raise<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.raise(self);
    // return 1;
  }
}

pub trait QWindow_raise<RetType> {
  fn raise(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::raise();
impl<'a> /*trait*/ QWindow_raise<()> for () {
  fn raise(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5raiseEv()};
     unsafe {_ZN7QWindow5raiseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QWindow::minimumSize();
impl /*struct*/ QWindow {
  pub fn minimumSize<RetType, T: QWindow_minimumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QWindow_minimumSize<RetType> {
  fn minimumSize(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QSize QWindow::minimumSize();
impl<'a> /*trait*/ QWindow_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: &mut QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11minimumSizeEv()};
    let mut ret = unsafe {_ZNK7QWindow11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QWindow::mapToGlobal(const QPoint & pos);
impl /*struct*/ QWindow {
  pub fn mapToGlobal<RetType, T: QWindow_mapToGlobal<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapToGlobal(self);
    // return 1;
  }
}

pub trait QWindow_mapToGlobal<RetType> {
  fn mapToGlobal(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QPoint QWindow::mapToGlobal(const QPoint & pos);
impl<'a> /*trait*/ QWindow_mapToGlobal<QPoint> for (QPoint) {
  fn mapToGlobal(self , rsthis: &mut QWindow) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11mapToGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWindow11mapToGlobalERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint{qclsinst: ret};
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
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QMargins QWindow::frameMargins();
impl /*struct*/ QWindow {
  pub fn frameMargins<RetType, T: QWindow_frameMargins<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.frameMargins(self);
    // return 1;
  }
}

pub trait QWindow_frameMargins<RetType> {
  fn frameMargins(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QMargins QWindow::frameMargins();
impl<'a> /*trait*/ QWindow_frameMargins<QMargins> for () {
  fn frameMargins(self , rsthis: &mut QWindow) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow12frameMarginsEv()};
    let mut ret = unsafe {_ZNK7QWindow12frameMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMargins{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setMaximumWidth(int w);
impl /*struct*/ QWindow {
  pub fn setMaximumWidth<RetType, T: QWindow_setMaximumWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMaximumWidth(self);
    // return 1;
  }
}

pub trait QWindow_setMaximumWidth<RetType> {
  fn setMaximumWidth(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setMaximumWidth(int w);
impl<'a> /*trait*/ QWindow_setMaximumWidth<()> for (i32) {
  fn setMaximumWidth(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow15setMaximumWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow15setMaximumWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWindow::maximumHeight();
impl /*struct*/ QWindow {
  pub fn maximumHeight<RetType, T: QWindow_maximumHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumHeight(self);
    // return 1;
  }
}

pub trait QWindow_maximumHeight<RetType> {
  fn maximumHeight(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  int QWindow::maximumHeight();
impl<'a> /*trait*/ QWindow_maximumHeight<i32> for () {
  fn maximumHeight(self , rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13maximumHeightEv()};
    let mut ret = unsafe {_ZNK7QWindow13maximumHeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWindow::focusObjectChanged(QObject * object);
impl /*struct*/ QWindow {
  pub fn focusObjectChanged<RetType, T: QWindow_focusObjectChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.focusObjectChanged(self);
    // return 1;
  }
}

pub trait QWindow_focusObjectChanged<RetType> {
  fn focusObjectChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::focusObjectChanged(QObject * object);
impl<'a> /*trait*/ QWindow_focusObjectChanged<()> for (QObject) {
  fn focusObjectChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow18focusObjectChangedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow18focusObjectChangedEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWindow::isModal();
impl /*struct*/ QWindow {
  pub fn isModal<RetType, T: QWindow_isModal<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isModal(self);
    // return 1;
  }
}

pub trait QWindow_isModal<RetType> {
  fn isModal(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  bool QWindow::isModal();
impl<'a> /*trait*/ QWindow_isModal<i8> for () {
  fn isModal(self , rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow7isModalEv()};
    let mut ret = unsafe {_ZNK7QWindow7isModalEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWindow::maximumWidthChanged(int arg);
impl /*struct*/ QWindow {
  pub fn maximumWidthChanged<RetType, T: QWindow_maximumWidthChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumWidthChanged(self);
    // return 1;
  }
}

pub trait QWindow_maximumWidthChanged<RetType> {
  fn maximumWidthChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::maximumWidthChanged(int arg);
impl<'a> /*trait*/ QWindow_maximumWidthChanged<()> for (i32) {
  fn maximumWidthChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow19maximumWidthChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow19maximumWidthChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QWindow::geometry();
impl /*struct*/ QWindow {
  pub fn geometry<RetType, T: QWindow_geometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QWindow_geometry<RetType> {
  fn geometry(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QRect QWindow::geometry();
impl<'a> /*trait*/ QWindow_geometry<QRect> for () {
  fn geometry(self , rsthis: &mut QWindow) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8geometryEv()};
    let mut ret = unsafe {_ZNK7QWindow8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setParent(QWindow * parent);
impl /*struct*/ QWindow {
  pub fn setParent<RetType, T: QWindow_setParent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setParent(self);
    // return 1;
  }
}

pub trait QWindow_setParent<RetType> {
  fn setParent(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setParent(QWindow * parent);
impl<'a> /*trait*/ QWindow_setParent<()> for (QWindow) {
  fn setParent(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow9setParentEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QWindow::frameGeometry();
impl /*struct*/ QWindow {
  pub fn frameGeometry<RetType, T: QWindow_frameGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.frameGeometry(self);
    // return 1;
  }
}

pub trait QWindow_frameGeometry<RetType> {
  fn frameGeometry(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QRect QWindow::frameGeometry();
impl<'a> /*trait*/ QWindow_frameGeometry<QRect> for () {
  fn frameGeometry(self , rsthis: &mut QWindow) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13frameGeometryEv()};
    let mut ret = unsafe {_ZNK7QWindow13frameGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QSurfaceFormat QWindow::requestedFormat();
impl /*struct*/ QWindow {
  pub fn requestedFormat<RetType, T: QWindow_requestedFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.requestedFormat(self);
    // return 1;
  }
}

pub trait QWindow_requestedFormat<RetType> {
  fn requestedFormat(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QSurfaceFormat QWindow::requestedFormat();
impl<'a> /*trait*/ QWindow_requestedFormat<QSurfaceFormat> for () {
  fn requestedFormat(self , rsthis: &mut QWindow) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow15requestedFormatEv()};
    let mut ret = unsafe {_ZNK7QWindow15requestedFormatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setHeight(int arg);
impl /*struct*/ QWindow {
  pub fn setHeight<RetType, T: QWindow_setHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeight(self);
    // return 1;
  }
}

pub trait QWindow_setHeight<RetType> {
  fn setHeight(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setHeight(int arg);
impl<'a> /*trait*/ QWindow_setHeight<()> for (i32) {
  fn setHeight(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow9setHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::requestActivate();
impl /*struct*/ QWindow {
  pub fn requestActivate<RetType, T: QWindow_requestActivate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.requestActivate(self);
    // return 1;
  }
}

pub trait QWindow_requestActivate<RetType> {
  fn requestActivate(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::requestActivate();
impl<'a> /*trait*/ QWindow_requestActivate<()> for () {
  fn requestActivate(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow15requestActivateEv()};
     unsafe {_ZN7QWindow15requestActivateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QWindow::mapFromGlobal(const QPoint & pos);
impl /*struct*/ QWindow {
  pub fn mapFromGlobal<RetType, T: QWindow_mapFromGlobal<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapFromGlobal(self);
    // return 1;
  }
}

pub trait QWindow_mapFromGlobal<RetType> {
  fn mapFromGlobal(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QPoint QWindow::mapFromGlobal(const QPoint & pos);
impl<'a> /*trait*/ QWindow_mapFromGlobal<QPoint> for (QPoint) {
  fn mapFromGlobal(self , rsthis: &mut QWindow) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13mapFromGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWindow13mapFromGlobalERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::windowTitleChanged(const QString & title);
impl /*struct*/ QWindow {
  pub fn windowTitleChanged<RetType, T: QWindow_windowTitleChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.windowTitleChanged(self);
    // return 1;
  }
}

pub trait QWindow_windowTitleChanged<RetType> {
  fn windowTitleChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::windowTitleChanged(const QString & title);
impl<'a> /*trait*/ QWindow_windowTitleChanged<()> for (QString) {
  fn windowTitleChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow18windowTitleChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow18windowTitleChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWindow::y();
impl /*struct*/ QWindow {
  pub fn y<RetType, T: QWindow_y<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QWindow_y<RetType> {
  fn y(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  int QWindow::y();
impl<'a> /*trait*/ QWindow_y<()> for () {
  fn y(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow1yEv()};
     unsafe {_ZNK7QWindow1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QWindow::width();
impl /*struct*/ QWindow {
  pub fn width<RetType, T: QWindow_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QWindow_width<RetType> {
  fn width(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  int QWindow::width();
impl<'a> /*trait*/ QWindow_width<i32> for () {
  fn width(self , rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow5widthEv()};
    let mut ret = unsafe {_ZNK7QWindow5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWindow::setFilePath(const QString & filePath);
impl /*struct*/ QWindow {
  pub fn setFilePath<RetType, T: QWindow_setFilePath<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFilePath(self);
    // return 1;
  }
}

pub trait QWindow_setFilePath<RetType> {
  fn setFilePath(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setFilePath(const QString & filePath);
impl<'a> /*trait*/ QWindow_setFilePath<()> for (QString) {
  fn setFilePath(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setFilePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow11setFilePathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setCursor(const QCursor & );
impl /*struct*/ QWindow {
  pub fn setCursor<RetType, T: QWindow_setCursor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCursor(self);
    // return 1;
  }
}

pub trait QWindow_setCursor<RetType> {
  fn setCursor(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setCursor(const QCursor & );
impl<'a> /*trait*/ QWindow_setCursor<()> for (QCursor) {
  fn setCursor(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow9setCursorERK7QCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setVisible(bool visible);
impl /*struct*/ QWindow {
  pub fn setVisible<RetType, T: QWindow_setVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QWindow_setVisible<RetType> {
  fn setVisible(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setVisible(bool visible);
impl<'a> /*trait*/ QWindow_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QWindow10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::~QWindow();
impl /*struct*/ QWindow {
  pub fn FreeQWindow<RetType, T: QWindow_FreeQWindow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQWindow(self);
    // return 1;
  }
}

pub trait QWindow_FreeQWindow<RetType> {
  fn FreeQWindow(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::~QWindow();
impl<'a> /*trait*/ QWindow_FreeQWindow<()> for () {
  fn FreeQWindow(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindowD0Ev()};
     unsafe {_ZN7QWindowD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QWindow::setMouseGrabEnabled(bool grab);
impl /*struct*/ QWindow {
  pub fn setMouseGrabEnabled<RetType, T: QWindow_setMouseGrabEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMouseGrabEnabled(self);
    // return 1;
  }
}

pub trait QWindow_setMouseGrabEnabled<RetType> {
  fn setMouseGrabEnabled(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  bool QWindow::setMouseGrabEnabled(bool grab);
impl<'a> /*trait*/ QWindow_setMouseGrabEnabled<i8> for (i8) {
  fn setMouseGrabEnabled(self , rsthis: &mut QWindow) -> i8 {
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
  pub fn isExposed<RetType, T: QWindow_isExposed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isExposed(self);
    // return 1;
  }
}

pub trait QWindow_isExposed<RetType> {
  fn isExposed(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  bool QWindow::isExposed();
impl<'a> /*trait*/ QWindow_isExposed<i8> for () {
  fn isExposed(self , rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow9isExposedEv()};
    let mut ret = unsafe {_ZNK7QWindow9isExposedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWindow::heightChanged(int arg);
impl /*struct*/ QWindow {
  pub fn heightChanged<RetType, T: QWindow_heightChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.heightChanged(self);
    // return 1;
  }
}

pub trait QWindow_heightChanged<RetType> {
  fn heightChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::heightChanged(int arg);
impl<'a> /*trait*/ QWindow_heightChanged<()> for (i32) {
  fn heightChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13heightChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow13heightChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWindow::minimumWidth();
impl /*struct*/ QWindow {
  pub fn minimumWidth<RetType, T: QWindow_minimumWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumWidth(self);
    // return 1;
  }
}

pub trait QWindow_minimumWidth<RetType> {
  fn minimumWidth(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  int QWindow::minimumWidth();
impl<'a> /*trait*/ QWindow_minimumWidth<i32> for () {
  fn minimumWidth(self , rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow12minimumWidthEv()};
    let mut ret = unsafe {_ZNK7QWindow12minimumWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWindow::setPosition(const QPoint & pt);
impl /*struct*/ QWindow {
  pub fn setPosition<RetType, T: QWindow_setPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPosition(self);
    // return 1;
  }
}

pub trait QWindow_setPosition<RetType> {
  fn setPosition(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setPosition(const QPoint & pt);
impl<'a> /*trait*/ QWindow_setPosition<()> for (QPoint) {
  fn setPosition(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setPositionERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow11setPositionERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWindow::close();
impl /*struct*/ QWindow {
  pub fn close<RetType, T: QWindow_close<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QWindow_close<RetType> {
  fn close(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  bool QWindow::close();
impl<'a> /*trait*/ QWindow_close<i8> for () {
  fn close(self , rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5closeEv()};
    let mut ret = unsafe {_ZN7QWindow5closeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QWindow::x();
impl /*struct*/ QWindow {
  pub fn x<RetType, T: QWindow_x<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QWindow_x<RetType> {
  fn x(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  int QWindow::x();
impl<'a> /*trait*/ QWindow_x<()> for () {
  fn x(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow1xEv()};
     unsafe {_ZNK7QWindow1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::setMinimumWidth(int w);
impl /*struct*/ QWindow {
  pub fn setMinimumWidth<RetType, T: QWindow_setMinimumWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMinimumWidth(self);
    // return 1;
  }
}

pub trait QWindow_setMinimumWidth<RetType> {
  fn setMinimumWidth(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setMinimumWidth(int w);
impl<'a> /*trait*/ QWindow_setMinimumWidth<()> for (i32) {
  fn setMinimumWidth(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow15setMinimumWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow15setMinimumWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRegion QWindow::mask();
impl /*struct*/ QWindow {
  pub fn mask<RetType, T: QWindow_mask<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mask(self);
    // return 1;
  }
}

pub trait QWindow_mask<RetType> {
  fn mask(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QRegion QWindow::mask();
impl<'a> /*trait*/ QWindow_mask<QRegion> for () {
  fn mask(self , rsthis: &mut QWindow) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow4maskEv()};
    let mut ret = unsafe {_ZNK7QWindow4maskEv(rsthis.qclsinst)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::widthChanged(int arg);
impl /*struct*/ QWindow {
  pub fn widthChanged<RetType, T: QWindow_widthChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widthChanged(self);
    // return 1;
  }
}

pub trait QWindow_widthChanged<RetType> {
  fn widthChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::widthChanged(int arg);
impl<'a> /*trait*/ QWindow_widthChanged<()> for (i32) {
  fn widthChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow12widthChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow12widthChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWindow * QWindow::parent();
impl /*struct*/ QWindow {
  pub fn parent<RetType, T: QWindow_parent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QWindow_parent<RetType> {
  fn parent(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QWindow * QWindow::parent();
impl<'a> /*trait*/ QWindow_parent<QWindow> for () {
  fn parent(self , rsthis: &mut QWindow) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6parentEv()};
    let mut ret = unsafe {_ZNK7QWindow6parentEv(rsthis.qclsinst)};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setFramePosition(const QPoint & point);
impl /*struct*/ QWindow {
  pub fn setFramePosition<RetType, T: QWindow_setFramePosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFramePosition(self);
    // return 1;
  }
}

pub trait QWindow_setFramePosition<RetType> {
  fn setFramePosition(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setFramePosition(const QPoint & point);
impl<'a> /*trait*/ QWindow_setFramePosition<()> for (QPoint) {
  fn setFramePosition(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setFramePositionERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow16setFramePositionERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::QWindow(QScreen * screen);
impl<'a> /*trait*/ QWindow_NewQWindow for (QScreen) {
  fn NewQWindow(self) -> QWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindowC1EP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWindowC1EP7QScreen(qthis, arg0)};
    let rsthis = QWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QWindow::setGeometry(int posx, int posy, int w, int h);
impl /*struct*/ QWindow {
  pub fn setGeometry<RetType, T: QWindow_setGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QWindow_setGeometry<RetType> {
  fn setGeometry(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setGeometry(int posx, int posy, int w, int h);
impl<'a> /*trait*/ QWindow_setGeometry<()> for (i32, i32, i32, i32) {
  fn setGeometry(self , rsthis: &mut QWindow) -> () {
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
  pub fn setKeyboardGrabEnabled<RetType, T: QWindow_setKeyboardGrabEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setKeyboardGrabEnabled(self);
    // return 1;
  }
}

pub trait QWindow_setKeyboardGrabEnabled<RetType> {
  fn setKeyboardGrabEnabled(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  bool QWindow::setKeyboardGrabEnabled(bool grab);
impl<'a> /*trait*/ QWindow_setKeyboardGrabEnabled<i8> for (i8) {
  fn setKeyboardGrabEnabled(self , rsthis: &mut QWindow) -> i8 {
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
  pub fn metaObject<RetType, T: QWindow_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QWindow_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  const QMetaObject * QWindow::metaObject();
impl<'a> /*trait*/ QWindow_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow10metaObjectEv()};
     unsafe {_ZNK7QWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::activeChanged();
impl /*struct*/ QWindow {
  pub fn activeChanged<RetType, T: QWindow_activeChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.activeChanged(self);
    // return 1;
  }
}

pub trait QWindow_activeChanged<RetType> {
  fn activeChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::activeChanged();
impl<'a> /*trait*/ QWindow_activeChanged<()> for () {
  fn activeChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13activeChangedEv()};
     unsafe {_ZN7QWindow13activeChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::setWidth(int arg);
impl /*struct*/ QWindow {
  pub fn setWidth<RetType, T: QWindow_setWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QWindow_setWidth<RetType> {
  fn setWidth(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setWidth(int arg);
impl<'a> /*trait*/ QWindow_setWidth<()> for (i32) {
  fn setWidth(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8setWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow8setWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setY(int arg);
impl /*struct*/ QWindow {
  pub fn setY<RetType, T: QWindow_setY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QWindow_setY<RetType> {
  fn setY(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setY(int arg);
impl<'a> /*trait*/ QWindow_setY<()> for (i32) {
  fn setY(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4setYEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow4setYEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QWindow::devicePixelRatio();
impl /*struct*/ QWindow {
  pub fn devicePixelRatio<RetType, T: QWindow_devicePixelRatio<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QWindow_devicePixelRatio<RetType> {
  fn devicePixelRatio(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  qreal QWindow::devicePixelRatio();
impl<'a> /*trait*/ QWindow_devicePixelRatio<f64> for () {
  fn devicePixelRatio(self , rsthis: &mut QWindow) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK7QWindow16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QWindow::setBaseSize(const QSize & size);
impl /*struct*/ QWindow {
  pub fn setBaseSize<RetType, T: QWindow_setBaseSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBaseSize(self);
    // return 1;
  }
}

pub trait QWindow_setBaseSize<RetType> {
  fn setBaseSize(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setBaseSize(const QSize & size);
impl<'a> /*trait*/ QWindow_setBaseSize<()> for (QSize) {
  fn setBaseSize(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setBaseSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow11setBaseSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::alert(int msec);
impl /*struct*/ QWindow {
  pub fn alert<RetType, T: QWindow_alert<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.alert(self);
    // return 1;
  }
}

pub trait QWindow_alert<RetType> {
  fn alert(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::alert(int msec);
impl<'a> /*trait*/ QWindow_alert<()> for (i32) {
  fn alert(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5alertEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow5alertEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::yChanged(int arg);
impl /*struct*/ QWindow {
  pub fn yChanged<RetType, T: QWindow_yChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.yChanged(self);
    // return 1;
  }
}

pub trait QWindow_yChanged<RetType> {
  fn yChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::yChanged(int arg);
impl<'a> /*trait*/ QWindow_yChanged<()> for (i32) {
  fn yChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8yChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow8yChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPlatformWindow * QWindow::handle();
impl /*struct*/ QWindow {
  pub fn handle<RetType, T: QWindow_handle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.handle(self);
    // return 1;
  }
}

pub trait QWindow_handle<RetType> {
  fn handle(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QPlatformWindow * QWindow::handle();
impl<'a> /*trait*/ QWindow_handle<()> for () {
  fn handle(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6handleEv()};
     unsafe {_ZNK7QWindow6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::destroy();
impl /*struct*/ QWindow {
  pub fn destroy<RetType, T: QWindow_destroy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.destroy(self);
    // return 1;
  }
}

pub trait QWindow_destroy<RetType> {
  fn destroy(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::destroy();
impl<'a> /*trait*/ QWindow_destroy<()> for () {
  fn destroy(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow7destroyEv()};
     unsafe {_ZN7QWindow7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QWindow * QWindow::transientParent();
impl /*struct*/ QWindow {
  pub fn transientParent<RetType, T: QWindow_transientParent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.transientParent(self);
    // return 1;
  }
}

pub trait QWindow_transientParent<RetType> {
  fn transientParent(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QWindow * QWindow::transientParent();
impl<'a> /*trait*/ QWindow_transientParent<QWindow> for () {
  fn transientParent(self , rsthis: &mut QWindow) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow15transientParentEv()};
    let mut ret = unsafe {_ZNK7QWindow15transientParentEv(rsthis.qclsinst)};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setMinimumHeight(int h);
impl /*struct*/ QWindow {
  pub fn setMinimumHeight<RetType, T: QWindow_setMinimumHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMinimumHeight(self);
    // return 1;
  }
}

pub trait QWindow_setMinimumHeight<RetType> {
  fn setMinimumHeight(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setMinimumHeight(int h);
impl<'a> /*trait*/ QWindow_setMinimumHeight<()> for (i32) {
  fn setMinimumHeight(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setMinimumHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow16setMinimumHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::show();
impl /*struct*/ QWindow {
  pub fn show<RetType, T: QWindow_show<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.show(self);
    // return 1;
  }
}

pub trait QWindow_show<RetType> {
  fn show(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::show();
impl<'a> /*trait*/ QWindow_show<()> for () {
  fn show(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4showEv()};
     unsafe {_ZN7QWindow4showEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::minimumWidthChanged(int arg);
impl /*struct*/ QWindow {
  pub fn minimumWidthChanged<RetType, T: QWindow_minimumWidthChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumWidthChanged(self);
    // return 1;
  }
}

pub trait QWindow_minimumWidthChanged<RetType> {
  fn minimumWidthChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::minimumWidthChanged(int arg);
impl<'a> /*trait*/ QWindow_minimumWidthChanged<()> for (i32) {
  fn minimumWidthChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow19minimumWidthChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow19minimumWidthChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QWindow::baseSize();
impl /*struct*/ QWindow {
  pub fn baseSize<RetType, T: QWindow_baseSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.baseSize(self);
    // return 1;
  }
}

pub trait QWindow_baseSize<RetType> {
  fn baseSize(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QSize QWindow::baseSize();
impl<'a> /*trait*/ QWindow_baseSize<QSize> for () {
  fn baseSize(self , rsthis: &mut QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8baseSizeEv()};
    let mut ret = unsafe {_ZNK7QWindow8baseSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QWindow::title();
impl /*struct*/ QWindow {
  pub fn title<RetType, T: QWindow_title<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.title(self);
    // return 1;
  }
}

pub trait QWindow_title<RetType> {
  fn title(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QString QWindow::title();
impl<'a> /*trait*/ QWindow_title<QString> for () {
  fn title(self , rsthis: &mut QWindow) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow5titleEv()};
    let mut ret = unsafe {_ZNK7QWindow5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::showMaximized();
impl /*struct*/ QWindow {
  pub fn showMaximized<RetType, T: QWindow_showMaximized<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showMaximized(self);
    // return 1;
  }
}

pub trait QWindow_showMaximized<RetType> {
  fn showMaximized(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::showMaximized();
impl<'a> /*trait*/ QWindow_showMaximized<()> for () {
  fn showMaximized(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13showMaximizedEv()};
     unsafe {_ZN7QWindow13showMaximizedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::create();
impl /*struct*/ QWindow {
  pub fn create<RetType, T: QWindow_create<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QWindow_create<RetType> {
  fn create(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::create();
impl<'a> /*trait*/ QWindow_create<()> for () {
  fn create(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow6createEv()};
     unsafe {_ZN7QWindow6createEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::resize(int w, int h);
impl<'a> /*trait*/ QWindow_resize<()> for (i32, i32) {
  fn resize(self , rsthis: &mut QWindow) -> () {
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
  pub fn screen<RetType, T: QWindow_screen<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.screen(self);
    // return 1;
  }
}

pub trait QWindow_screen<RetType> {
  fn screen(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QScreen * QWindow::screen();
impl<'a> /*trait*/ QWindow_screen<QScreen> for () {
  fn screen(self , rsthis: &mut QWindow) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6screenEv()};
    let mut ret = unsafe {_ZNK7QWindow6screenEv(rsthis.qclsinst)};
    let mut ret1 = QScreen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setPosition(int posx, int posy);
impl<'a> /*trait*/ QWindow_setPosition<()> for (i32, i32) {
  fn setPosition(self , rsthis: &mut QWindow) -> () {
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
  pub fn setOpacity<RetType, T: QWindow_setOpacity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOpacity(self);
    // return 1;
  }
}

pub trait QWindow_setOpacity<RetType> {
  fn setOpacity(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setOpacity(qreal level);
impl<'a> /*trait*/ QWindow_setOpacity<()> for (f64) {
  fn setOpacity(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QWindow10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setGeometry(const QRect & rect);
impl<'a> /*trait*/ QWindow_setGeometry<()> for (QRect) {
  fn setGeometry(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setSizeIncrement(const QSize & size);
impl /*struct*/ QWindow {
  pub fn setSizeIncrement<RetType, T: QWindow_setSizeIncrement<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSizeIncrement(self);
    // return 1;
  }
}

pub trait QWindow_setSizeIncrement<RetType> {
  fn setSizeIncrement(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setSizeIncrement(const QSize & size);
impl<'a> /*trait*/ QWindow_setSizeIncrement<()> for (QSize) {
  fn setSizeIncrement(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setSizeIncrementERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow16setSizeIncrementERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::showMinimized();
impl /*struct*/ QWindow {
  pub fn showMinimized<RetType, T: QWindow_showMinimized<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showMinimized(self);
    // return 1;
  }
}

pub trait QWindow_showMinimized<RetType> {
  fn showMinimized(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::showMinimized();
impl<'a> /*trait*/ QWindow_showMinimized<()> for () {
  fn showMinimized(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13showMinimizedEv()};
     unsafe {_ZN7QWindow13showMinimizedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QObject * QWindow::focusObject();
impl /*struct*/ QWindow {
  pub fn focusObject<RetType, T: QWindow_focusObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.focusObject(self);
    // return 1;
  }
}

pub trait QWindow_focusObject<RetType> {
  fn focusObject(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QObject * QWindow::focusObject();
impl<'a> /*trait*/ QWindow_focusObject<QObject> for () {
  fn focusObject(self , rsthis: &mut QWindow) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11focusObjectEv()};
    let mut ret = unsafe {_ZNK7QWindow11focusObjectEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWindow::isActive();
impl /*struct*/ QWindow {
  pub fn isActive<RetType, T: QWindow_isActive<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isActive(self);
    // return 1;
  }
}

pub trait QWindow_isActive<RetType> {
  fn isActive(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  bool QWindow::isActive();
impl<'a> /*trait*/ QWindow_isActive<i8> for () {
  fn isActive(self , rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8isActiveEv()};
    let mut ret = unsafe {_ZNK7QWindow8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWindow::maximumHeightChanged(int arg);
impl /*struct*/ QWindow {
  pub fn maximumHeightChanged<RetType, T: QWindow_maximumHeightChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumHeightChanged(self);
    // return 1;
  }
}

pub trait QWindow_maximumHeightChanged<RetType> {
  fn maximumHeightChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::maximumHeightChanged(int arg);
impl<'a> /*trait*/ QWindow_maximumHeightChanged<()> for (i32) {
  fn maximumHeightChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow20maximumHeightChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow20maximumHeightChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QWindow::accessibleRoot();
impl /*struct*/ QWindow {
  pub fn accessibleRoot<RetType, T: QWindow_accessibleRoot<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.accessibleRoot(self);
    // return 1;
  }
}

pub trait QWindow_accessibleRoot<RetType> {
  fn accessibleRoot(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QAccessibleInterface * QWindow::accessibleRoot();
impl<'a> /*trait*/ QWindow_accessibleRoot<QAccessibleInterface> for () {
  fn accessibleRoot(self , rsthis: &mut QWindow) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow14accessibleRootEv()};
    let mut ret = unsafe {_ZNK7QWindow14accessibleRootEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QCursor QWindow::cursor();
impl /*struct*/ QWindow {
  pub fn cursor<RetType, T: QWindow_cursor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cursor(self);
    // return 1;
  }
}

pub trait QWindow_cursor<RetType> {
  fn cursor(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QCursor QWindow::cursor();
impl<'a> /*trait*/ QWindow_cursor<QCursor> for () {
  fn cursor(self , rsthis: &mut QWindow) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6cursorEv()};
    let mut ret = unsafe {_ZNK7QWindow6cursorEv(rsthis.qclsinst)};
    let mut ret1 = QCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setFormat(const QSurfaceFormat & format);
impl /*struct*/ QWindow {
  pub fn setFormat<RetType, T: QWindow_setFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QWindow_setFormat<RetType> {
  fn setFormat(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QWindow_setFormat<()> for (QSurfaceFormat) {
  fn setFormat(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::showFullScreen();
impl /*struct*/ QWindow {
  pub fn showFullScreen<RetType, T: QWindow_showFullScreen<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showFullScreen(self);
    // return 1;
  }
}

pub trait QWindow_showFullScreen<RetType> {
  fn showFullScreen(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::showFullScreen();
impl<'a> /*trait*/ QWindow_showFullScreen<()> for () {
  fn showFullScreen(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14showFullScreenEv()};
     unsafe {_ZN7QWindow14showFullScreenEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::setX(int arg);
impl /*struct*/ QWindow {
  pub fn setX<RetType, T: QWindow_setX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QWindow_setX<RetType> {
  fn setX(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setX(int arg);
impl<'a> /*trait*/ QWindow_setX<()> for (i32) {
  fn setX(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4setXEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow4setXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::opacityChanged(qreal opacity);
impl /*struct*/ QWindow {
  pub fn opacityChanged<RetType, T: QWindow_opacityChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.opacityChanged(self);
    // return 1;
  }
}

pub trait QWindow_opacityChanged<RetType> {
  fn opacityChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::opacityChanged(qreal opacity);
impl<'a> /*trait*/ QWindow_opacityChanged<()> for (f64) {
  fn opacityChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14opacityChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QWindow14opacityChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::lower();
impl /*struct*/ QWindow {
  pub fn lower<RetType, T: QWindow_lower<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lower(self);
    // return 1;
  }
}

pub trait QWindow_lower<RetType> {
  fn lower(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::lower();
impl<'a> /*trait*/ QWindow_lower<()> for () {
  fn lower(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5lowerEv()};
     unsafe {_ZN7QWindow5lowerEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::requestUpdate();
impl /*struct*/ QWindow {
  pub fn requestUpdate<RetType, T: QWindow_requestUpdate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.requestUpdate(self);
    // return 1;
  }
}

pub trait QWindow_requestUpdate<RetType> {
  fn requestUpdate(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::requestUpdate();
impl<'a> /*trait*/ QWindow_requestUpdate<()> for () {
  fn requestUpdate(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13requestUpdateEv()};
     unsafe {_ZN7QWindow13requestUpdateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::hide();
impl /*struct*/ QWindow {
  pub fn hide<RetType, T: QWindow_hide<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hide(self);
    // return 1;
  }
}

pub trait QWindow_hide<RetType> {
  fn hide(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::hide();
impl<'a> /*trait*/ QWindow_hide<()> for () {
  fn hide(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4hideEv()};
     unsafe {_ZN7QWindow4hideEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWindow::minimumHeightChanged(int arg);
impl /*struct*/ QWindow {
  pub fn minimumHeightChanged<RetType, T: QWindow_minimumHeightChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumHeightChanged(self);
    // return 1;
  }
}

pub trait QWindow_minimumHeightChanged<RetType> {
  fn minimumHeightChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::minimumHeightChanged(int arg);
impl<'a> /*trait*/ QWindow_minimumHeightChanged<()> for (i32) {
  fn minimumHeightChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow20minimumHeightChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow20minimumHeightChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setMask(const QRegion & region);
impl /*struct*/ QWindow {
  pub fn setMask<RetType, T: QWindow_setMask<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMask(self);
    // return 1;
  }
}

pub trait QWindow_setMask<RetType> {
  fn setMask(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setMask(const QRegion & region);
impl<'a> /*trait*/ QWindow_setMask<()> for (QRegion) {
  fn setMask(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow7setMaskERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow7setMaskERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWindow::setMaximumSize(const QSize & size);
impl /*struct*/ QWindow {
  pub fn setMaximumSize<RetType, T: QWindow_setMaximumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMaximumSize(self);
    // return 1;
  }
}

pub trait QWindow_setMaximumSize<RetType> {
  fn setMaximumSize(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setMaximumSize(const QSize & size);
impl<'a> /*trait*/ QWindow_setMaximumSize<()> for (QSize) {
  fn setMaximumSize(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14setMaximumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow14setMaximumSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWindow::height();
impl /*struct*/ QWindow {
  pub fn height<RetType, T: QWindow_height<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QWindow_height<RetType> {
  fn height(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  int QWindow::height();
impl<'a> /*trait*/ QWindow_height<i32> for () {
  fn height(self , rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6heightEv()};
    let mut ret = unsafe {_ZNK7QWindow6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QWindow::size();
impl /*struct*/ QWindow {
  pub fn size<RetType, T: QWindow_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QWindow_size<RetType> {
  fn size(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QSize QWindow::size();
impl<'a> /*trait*/ QWindow_size<QSize> for () {
  fn size(self , rsthis: &mut QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow4sizeEv()};
    let mut ret = unsafe {_ZNK7QWindow4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QWindow::maximumWidth();
impl /*struct*/ QWindow {
  pub fn maximumWidth<RetType, T: QWindow_maximumWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumWidth(self);
    // return 1;
  }
}

pub trait QWindow_maximumWidth<RetType> {
  fn maximumWidth(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  int QWindow::maximumWidth();
impl<'a> /*trait*/ QWindow_maximumWidth<i32> for () {
  fn maximumWidth(self , rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow12maximumWidthEv()};
    let mut ret = unsafe {_ZNK7QWindow12maximumWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPoint QWindow::position();
impl /*struct*/ QWindow {
  pub fn position<RetType, T: QWindow_position<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QWindow_position<RetType> {
  fn position(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QPoint QWindow::position();
impl<'a> /*trait*/ QWindow_position<QPoint> for () {
  fn position(self , rsthis: &mut QWindow) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8positionEv()};
    let mut ret = unsafe {_ZNK7QWindow8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::setMaximumHeight(int h);
impl /*struct*/ QWindow {
  pub fn setMaximumHeight<RetType, T: QWindow_setMaximumHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMaximumHeight(self);
    // return 1;
  }
}

pub trait QWindow_setMaximumHeight<RetType> {
  fn setMaximumHeight(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::setMaximumHeight(int h);
impl<'a> /*trait*/ QWindow_setMaximumHeight<()> for (i32) {
  fn setMaximumHeight(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setMaximumHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow16setMaximumHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QWindow::filePath();
impl /*struct*/ QWindow {
  pub fn filePath<RetType, T: QWindow_filePath<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.filePath(self);
    // return 1;
  }
}

pub trait QWindow_filePath<RetType> {
  fn filePath(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QString QWindow::filePath();
impl<'a> /*trait*/ QWindow_filePath<QString> for () {
  fn filePath(self , rsthis: &mut QWindow) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8filePathEv()};
    let mut ret = unsafe {_ZNK7QWindow8filePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::showNormal();
impl /*struct*/ QWindow {
  pub fn showNormal<RetType, T: QWindow_showNormal<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showNormal(self);
    // return 1;
  }
}

pub trait QWindow_showNormal<RetType> {
  fn showNormal(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::showNormal();
impl<'a> /*trait*/ QWindow_showNormal<()> for () {
  fn showNormal(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow10showNormalEv()};
     unsafe {_ZN7QWindow10showNormalEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QWindow::framePosition();
impl /*struct*/ QWindow {
  pub fn framePosition<RetType, T: QWindow_framePosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.framePosition(self);
    // return 1;
  }
}

pub trait QWindow_framePosition<RetType> {
  fn framePosition(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QPoint QWindow::framePosition();
impl<'a> /*trait*/ QWindow_framePosition<QPoint> for () {
  fn framePosition(self , rsthis: &mut QWindow) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13framePositionEv()};
    let mut ret = unsafe {_ZNK7QWindow13framePositionEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWindow::visibleChanged(bool arg);
impl /*struct*/ QWindow {
  pub fn visibleChanged<RetType, T: QWindow_visibleChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.visibleChanged(self);
    // return 1;
  }
}

pub trait QWindow_visibleChanged<RetType> {
  fn visibleChanged(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  void QWindow::visibleChanged(bool arg);
impl<'a> /*trait*/ QWindow_visibleChanged<()> for (i8) {
  fn visibleChanged(self , rsthis: &mut QWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14visibleChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QWindow14visibleChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIcon QWindow::icon();
impl /*struct*/ QWindow {
  pub fn icon<RetType, T: QWindow_icon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.icon(self);
    // return 1;
  }
}

pub trait QWindow_icon<RetType> {
  fn icon(self , rsthis: &mut QWindow) -> RetType;
}

  // proto:  QIcon QWindow::icon();
impl<'a> /*trait*/ QWindow_icon<QIcon> for () {
  fn icon(self , rsthis: &mut QWindow) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow4iconEv()};
    let mut ret = unsafe {_ZNK7QWindow4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

