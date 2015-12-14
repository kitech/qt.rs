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
  fn _ZN7QWindow8xChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWindow::unsetCursor();
  fn _ZN7QWindow11unsetCursorEv(qthis: *mut c_void) ;
  // proto:  bool QWindow::isVisible();
  fn _ZNK7QWindow9isVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWindow::setScreen(QScreen * screen);
  fn _ZN7QWindow9setScreenEP7QScreen(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QWindow::maximumSize();
  fn _ZNK7QWindow11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setTransientParent(QWindow * parent);
  fn _ZN7QWindow18setTransientParentEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSurfaceFormat QWindow::format();
  fn _ZNK7QWindow6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWindow::isTopLevel();
  fn _ZNK7QWindow10isTopLevelEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWindow::NewQWindow(const QWindow & );
  fn _ZN7QWindowC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::setIcon(const QIcon & icon);
  fn _ZN7QWindow7setIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QWindow::opacity();
  fn _ZNK7QWindow7opacityEv(qthis: *mut c_void) -> c_double;
  // proto:  void QWindow::setMinimumSize(const QSize & size);
  fn _ZN7QWindow14setMinimumSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QWindow::minimumHeight();
  fn _ZNK7QWindow13minimumHeightEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QWindow::sizeIncrement();
  fn _ZNK7QWindow13sizeIncrementEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::resize(const QSize & newSize);
  fn _ZN7QWindow6resizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::screenChanged(QScreen * screen);
  fn _ZN7QWindow13screenChangedEP7QScreen(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::setTitle(const QString & );
  fn _ZN7QWindow8setTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::raise();
  fn _ZN7QWindow5raiseEv(qthis: *mut c_void) ;
  // proto:  QSize QWindow::minimumSize();
  fn _ZNK7QWindow11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QWindow::mapToGlobal(const QPoint & pos);
  fn _ZNK7QWindow11mapToGlobalERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto: static QWindow * QWindow::fromWinId(WId id);
  fn _ZN7QWindow9fromWinIdEi(arg0: *mut c_uint) -> *mut c_void;
  // proto:  QMargins QWindow::frameMargins();
  fn _ZNK7QWindow12frameMarginsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setMaximumWidth(int w);
  fn _ZN7QWindow15setMaximumWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QWindow::maximumHeight();
  fn _ZNK7QWindow13maximumHeightEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWindow::focusObjectChanged(QObject * object);
  fn _ZN7QWindow18focusObjectChangedEP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QWindow::isModal();
  fn _ZNK7QWindow7isModalEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWindow::maximumWidthChanged(int arg);
  fn _ZN7QWindow19maximumWidthChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QRect QWindow::geometry();
  fn _ZNK7QWindow8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setParent(QWindow * parent);
  fn _ZN7QWindow9setParentEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRect QWindow::frameGeometry();
  fn _ZNK7QWindow13frameGeometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSurfaceFormat QWindow::requestedFormat();
  fn _ZNK7QWindow15requestedFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setHeight(int arg);
  fn _ZN7QWindow9setHeightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWindow::requestActivate();
  fn _ZN7QWindow15requestActivateEv(qthis: *mut c_void) ;
  // proto:  QPoint QWindow::mapFromGlobal(const QPoint & pos);
  fn _ZNK7QWindow13mapFromGlobalERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::windowTitleChanged(const QString & title);
  fn _ZN7QWindow18windowTitleChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QWindow::y();
  fn _ZNK7QWindow1yEv(qthis: *mut c_void) -> c_int;
  // proto:  int QWindow::width();
  fn _ZNK7QWindow5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWindow::setFilePath(const QString & filePath);
  fn _ZN7QWindow11setFilePathERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::setCursor(const QCursor & );
  fn _ZN7QWindow9setCursorERK7QCursor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::setVisible(bool visible);
  fn _ZN7QWindow10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QWindow::FreeQWindow();
  fn _ZN7QWindowD0Ev(qthis: *mut c_void) ;
  // proto:  bool QWindow::setMouseGrabEnabled(bool grab);
  fn _ZN7QWindow19setMouseGrabEnabledEb(qthis: *mut c_void, arg0: int8_t) -> int8_t;
  // proto:  bool QWindow::isExposed();
  fn _ZNK7QWindow9isExposedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWindow::heightChanged(int arg);
  fn _ZN7QWindow13heightChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QWindow::minimumWidth();
  fn _ZNK7QWindow12minimumWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWindow::setPosition(const QPoint & pt);
  fn _ZN7QWindow11setPositionERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QWindow::close();
  fn _ZN7QWindow5closeEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QWindow::x();
  fn _ZNK7QWindow1xEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWindow::setMinimumWidth(int w);
  fn _ZN7QWindow15setMinimumWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QRegion QWindow::mask();
  fn _ZNK7QWindow4maskEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::widthChanged(int arg);
  fn _ZN7QWindow12widthChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QWindow * QWindow::parent();
  fn _ZNK7QWindow6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setFramePosition(const QPoint & point);
  fn _ZN7QWindow16setFramePositionERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::NewQWindow(QScreen * screen);
  fn _ZN7QWindowC1EP7QScreen(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::setGeometry(int posx, int posy, int w, int h);
  fn _ZN7QWindow11setGeometryEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  bool QWindow::setKeyboardGrabEnabled(bool grab);
  fn _ZN7QWindow22setKeyboardGrabEnabledEb(qthis: *mut c_void, arg0: int8_t) -> int8_t;
  // proto:  const QMetaObject * QWindow::metaObject();
  fn _ZNK7QWindow10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QWindow::NewQWindow(QWindow * parent);
  fn _ZN7QWindowC1EPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::activeChanged();
  fn _ZN7QWindow13activeChangedEv(qthis: *mut c_void) ;
  // proto:  void QWindow::setWidth(int arg);
  fn _ZN7QWindow8setWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWindow::setY(int arg);
  fn _ZN7QWindow4setYEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  double QWindow::devicePixelRatio();
  fn _ZNK7QWindow16devicePixelRatioEv(qthis: *mut c_void) -> c_double;
  // proto:  void QWindow::setBaseSize(const QSize & size);
  fn _ZN7QWindow11setBaseSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::alert(int msec);
  fn _ZN7QWindow5alertEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWindow::yChanged(int arg);
  fn _ZN7QWindow8yChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QPlatformWindow * QWindow::handle();
  fn _ZNK7QWindow6handleEv(qthis: *mut c_void) ;
  // proto:  void QWindow::destroy();
  fn _ZN7QWindow7destroyEv(qthis: *mut c_void) ;
  // proto:  QWindow * QWindow::transientParent();
  fn _ZNK7QWindow15transientParentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setMinimumHeight(int h);
  fn _ZN7QWindow16setMinimumHeightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWindow::show();
  fn _ZN7QWindow4showEv(qthis: *mut c_void) ;
  // proto:  void QWindow::minimumWidthChanged(int arg);
  fn _ZN7QWindow19minimumWidthChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QSize QWindow::baseSize();
  fn _ZNK7QWindow8baseSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QWindow::title();
  fn _ZNK7QWindow5titleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::showMaximized();
  fn _ZN7QWindow13showMaximizedEv(qthis: *mut c_void) ;
  // proto:  void QWindow::create();
  fn _ZN7QWindow6createEv(qthis: *mut c_void) ;
  // proto:  void QWindow::resize(int w, int h);
  fn _ZN7QWindow6resizeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QScreen * QWindow::screen();
  fn _ZNK7QWindow6screenEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setPosition(int posx, int posy);
  fn _ZN7QWindow11setPositionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QWindow::setOpacity(qreal level);
  fn _ZN7QWindow10setOpacityEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QWindow::setGeometry(const QRect & rect);
  fn _ZN7QWindow11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::setSizeIncrement(const QSize & size);
  fn _ZN7QWindow16setSizeIncrementERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::showMinimized();
  fn _ZN7QWindow13showMinimizedEv(qthis: *mut c_void) ;
  // proto:  QObject * QWindow::focusObject();
  fn _ZNK7QWindow11focusObjectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWindow::isActive();
  fn _ZNK7QWindow8isActiveEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWindow::maximumHeightChanged(int arg);
  fn _ZN7QWindow20maximumHeightChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QAccessibleInterface * QWindow::accessibleRoot();
  fn _ZNK7QWindow14accessibleRootEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QCursor QWindow::cursor();
  fn _ZNK7QWindow6cursorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setFormat(const QSurfaceFormat & format);
  fn _ZN7QWindow9setFormatERK14QSurfaceFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::showFullScreen();
  fn _ZN7QWindow14showFullScreenEv(qthis: *mut c_void) ;
  // proto:  void QWindow::setX(int arg);
  fn _ZN7QWindow4setXEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWindow::opacityChanged(qreal opacity);
  fn _ZN7QWindow14opacityChangedEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QWindow::lower();
  fn _ZN7QWindow5lowerEv(qthis: *mut c_void) ;
  // proto:  void QWindow::requestUpdate();
  fn _ZN7QWindow13requestUpdateEv(qthis: *mut c_void) ;
  // proto:  void QWindow::hide();
  fn _ZN7QWindow4hideEv(qthis: *mut c_void) ;
  // proto:  void QWindow::minimumHeightChanged(int arg);
  fn _ZN7QWindow20minimumHeightChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWindow::setMask(const QRegion & region);
  fn _ZN7QWindow7setMaskERK7QRegion(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWindow::setMaximumSize(const QSize & size);
  fn _ZN7QWindow14setMaximumSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QWindow::height();
  fn _ZNK7QWindow6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QWindow::size();
  fn _ZNK7QWindow4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QWindow::maximumWidth();
  fn _ZNK7QWindow12maximumWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  QPoint QWindow::position();
  fn _ZNK7QWindow8positionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::setMaximumHeight(int h);
  fn _ZN7QWindow16setMaximumHeightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QString QWindow::filePath();
  fn _ZNK7QWindow8filePathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::showNormal();
  fn _ZN7QWindow10showNormalEv(qthis: *mut c_void) ;
  // proto:  QPoint QWindow::framePosition();
  fn _ZNK7QWindow13framePositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWindow::visibleChanged(bool arg);
  fn _ZN7QWindow14visibleChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QIcon QWindow::icon();
  fn _ZNK7QWindow4iconEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QWindow)=1
pub struct QWindow {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWindow {
  pub fn xChanged<T: QWindow_xChanged>(&mut self, value: T)  {
     value.xChanged(self);
    // return 1;
  }
}

pub trait QWindow_xChanged {
  fn xChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::xChanged(int arg);
impl<'a> /*trait*/ QWindow_xChanged for (i32) {
  fn xChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8xChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow8xChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn unsetCursor<T: QWindow_unsetCursor>(&mut self, value: T)  {
     value.unsetCursor(self);
    // return 1;
  }
}

pub trait QWindow_unsetCursor {
  fn unsetCursor(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::unsetCursor();
impl<'a> /*trait*/ QWindow_unsetCursor for () {
  fn unsetCursor(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11unsetCursorEv()};
     unsafe {_ZN7QWindow11unsetCursorEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn isVisible<T: QWindow_isVisible>(&mut self, value: T) -> i8 {
    return value.isVisible(self);
    // return 1;
  }
}

pub trait QWindow_isVisible {
  fn isVisible(self, rsthis: &mut QWindow) -> i8;
}

// proto:  bool QWindow::isVisible();
impl<'a> /*trait*/ QWindow_isVisible for () {
  fn isVisible(self, rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow9isVisibleEv()};
    let mut ret = unsafe {_ZNK7QWindow9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setScreen<T: QWindow_setScreen>(&mut self, value: T)  {
     value.setScreen(self);
    // return 1;
  }
}

pub trait QWindow_setScreen {
  fn setScreen(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setScreen(QScreen * screen);
impl<'a> /*trait*/ QWindow_setScreen for (&'a mut QScreen) {
  fn setScreen(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setScreenEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow9setScreenEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn maximumSize<T: QWindow_maximumSize>(&mut self, value: T) -> QSize {
    return value.maximumSize(self);
    // return 1;
  }
}

pub trait QWindow_maximumSize {
  fn maximumSize(self, rsthis: &mut QWindow) -> QSize;
}

// proto:  QSize QWindow::maximumSize();
impl<'a> /*trait*/ QWindow_maximumSize for () {
  fn maximumSize(self, rsthis: &mut QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11maximumSizeEv()};
    let mut ret = unsafe {_ZNK7QWindow11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setTransientParent<T: QWindow_setTransientParent>(&mut self, value: T)  {
     value.setTransientParent(self);
    // return 1;
  }
}

pub trait QWindow_setTransientParent {
  fn setTransientParent(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setTransientParent(QWindow * parent);
impl<'a> /*trait*/ QWindow_setTransientParent for (&'a mut QWindow) {
  fn setTransientParent(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow18setTransientParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow18setTransientParentEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn format<T: QWindow_format>(&mut self, value: T) -> QSurfaceFormat {
    return value.format(self);
    // return 1;
  }
}

pub trait QWindow_format {
  fn format(self, rsthis: &mut QWindow) -> QSurfaceFormat;
}

// proto:  QSurfaceFormat QWindow::format();
impl<'a> /*trait*/ QWindow_format for () {
  fn format(self, rsthis: &mut QWindow) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6formatEv()};
    let mut ret = unsafe {_ZNK7QWindow6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn isTopLevel<T: QWindow_isTopLevel>(&mut self, value: T) -> i8 {
    return value.isTopLevel(self);
    // return 1;
  }
}

pub trait QWindow_isTopLevel {
  fn isTopLevel(self, rsthis: &mut QWindow) -> i8;
}

// proto:  bool QWindow::isTopLevel();
impl<'a> /*trait*/ QWindow_isTopLevel for () {
  fn isTopLevel(self, rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow10isTopLevelEv()};
    let mut ret = unsafe {_ZNK7QWindow10isTopLevelEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

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

// proto: void QWindow::NewQWindow(const QWindow & );
impl<'a> /*trait*/ QWindow_NewQWindow for (&'a  QWindow) {
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

impl /*struct*/ QWindow {
  pub fn setIcon<T: QWindow_setIcon>(&mut self, value: T)  {
     value.setIcon(self);
    // return 1;
  }
}

pub trait QWindow_setIcon {
  fn setIcon(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QWindow_setIcon for (&'a  QIcon) {
  fn setIcon(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn opacity<T: QWindow_opacity>(&mut self, value: T) -> f64 {
    return value.opacity(self);
    // return 1;
  }
}

pub trait QWindow_opacity {
  fn opacity(self, rsthis: &mut QWindow) -> f64;
}

// proto:  double QWindow::opacity();
impl<'a> /*trait*/ QWindow_opacity for () {
  fn opacity(self, rsthis: &mut QWindow) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow7opacityEv()};
    let mut ret = unsafe {_ZNK7QWindow7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMinimumSize<T: QWindow_setMinimumSize>(&mut self, value: T)  {
     value.setMinimumSize(self);
    // return 1;
  }
}

pub trait QWindow_setMinimumSize {
  fn setMinimumSize(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setMinimumSize(const QSize & size);
impl<'a> /*trait*/ QWindow_setMinimumSize for (&'a  QSize) {
  fn setMinimumSize(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14setMinimumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow14setMinimumSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn minimumHeight<T: QWindow_minimumHeight>(&mut self, value: T) -> i32 {
    return value.minimumHeight(self);
    // return 1;
  }
}

pub trait QWindow_minimumHeight {
  fn minimumHeight(self, rsthis: &mut QWindow) -> i32;
}

// proto:  int QWindow::minimumHeight();
impl<'a> /*trait*/ QWindow_minimumHeight for () {
  fn minimumHeight(self, rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13minimumHeightEv()};
    let mut ret = unsafe {_ZNK7QWindow13minimumHeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn sizeIncrement<T: QWindow_sizeIncrement>(&mut self, value: T) -> QSize {
    return value.sizeIncrement(self);
    // return 1;
  }
}

pub trait QWindow_sizeIncrement {
  fn sizeIncrement(self, rsthis: &mut QWindow) -> QSize;
}

// proto:  QSize QWindow::sizeIncrement();
impl<'a> /*trait*/ QWindow_sizeIncrement for () {
  fn sizeIncrement(self, rsthis: &mut QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13sizeIncrementEv()};
    let mut ret = unsafe {_ZNK7QWindow13sizeIncrementEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn resize<T: QWindow_resize>(&mut self, value: T)  {
     value.resize(self);
    // return 1;
  }
}

pub trait QWindow_resize {
  fn resize(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::resize(const QSize & newSize);
impl<'a> /*trait*/ QWindow_resize for (&'a  QSize) {
  fn resize(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow6resizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow6resizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn screenChanged<T: QWindow_screenChanged>(&mut self, value: T)  {
     value.screenChanged(self);
    // return 1;
  }
}

pub trait QWindow_screenChanged {
  fn screenChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::screenChanged(QScreen * screen);
impl<'a> /*trait*/ QWindow_screenChanged for (&'a mut QScreen) {
  fn screenChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13screenChangedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow13screenChangedEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setTitle<T: QWindow_setTitle>(&mut self, value: T)  {
     value.setTitle(self);
    // return 1;
  }
}

pub trait QWindow_setTitle {
  fn setTitle(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setTitle(const QString & );
impl<'a> /*trait*/ QWindow_setTitle for (&'a  QString) {
  fn setTitle(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn raise<T: QWindow_raise>(&mut self, value: T)  {
     value.raise(self);
    // return 1;
  }
}

pub trait QWindow_raise {
  fn raise(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::raise();
impl<'a> /*trait*/ QWindow_raise for () {
  fn raise(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5raiseEv()};
     unsafe {_ZN7QWindow5raiseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn minimumSize<T: QWindow_minimumSize>(&mut self, value: T) -> QSize {
    return value.minimumSize(self);
    // return 1;
  }
}

pub trait QWindow_minimumSize {
  fn minimumSize(self, rsthis: &mut QWindow) -> QSize;
}

// proto:  QSize QWindow::minimumSize();
impl<'a> /*trait*/ QWindow_minimumSize for () {
  fn minimumSize(self, rsthis: &mut QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11minimumSizeEv()};
    let mut ret = unsafe {_ZNK7QWindow11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn mapToGlobal<T: QWindow_mapToGlobal>(&mut self, value: T) -> QPoint {
    return value.mapToGlobal(self);
    // return 1;
  }
}

pub trait QWindow_mapToGlobal {
  fn mapToGlobal(self, rsthis: &mut QWindow) -> QPoint;
}

// proto:  QPoint QWindow::mapToGlobal(const QPoint & pos);
impl<'a> /*trait*/ QWindow_mapToGlobal for (&'a  QPoint) {
  fn mapToGlobal(self, rsthis: &mut QWindow) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11mapToGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWindow11mapToGlobalERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn fromWinId<T: QWindow_fromWinId>(&mut self, value: T) -> QWindow {
    return value.fromWinId(self);
    // return 1;
  }
}

pub trait QWindow_fromWinId {
  fn fromWinId(self, rsthis: &mut QWindow) -> QWindow;
}

// proto: static QWindow * QWindow::fromWinId(WId id);
impl<'a> /*trait*/ QWindow_fromWinId for (*mut i32) {
  fn fromWinId(self, rsthis: &mut QWindow) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9fromWinIdEi()};
    let arg0 = self  as *mut c_uint;
    let mut ret = unsafe {_ZN7QWindow9fromWinIdEi(arg0)};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn frameMargins<T: QWindow_frameMargins>(&mut self, value: T) -> QMargins {
    return value.frameMargins(self);
    // return 1;
  }
}

pub trait QWindow_frameMargins {
  fn frameMargins(self, rsthis: &mut QWindow) -> QMargins;
}

// proto:  QMargins QWindow::frameMargins();
impl<'a> /*trait*/ QWindow_frameMargins for () {
  fn frameMargins(self, rsthis: &mut QWindow) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow12frameMarginsEv()};
    let mut ret = unsafe {_ZNK7QWindow12frameMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMargins{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMaximumWidth<T: QWindow_setMaximumWidth>(&mut self, value: T)  {
     value.setMaximumWidth(self);
    // return 1;
  }
}

pub trait QWindow_setMaximumWidth {
  fn setMaximumWidth(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setMaximumWidth(int w);
impl<'a> /*trait*/ QWindow_setMaximumWidth for (i32) {
  fn setMaximumWidth(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow15setMaximumWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow15setMaximumWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn maximumHeight<T: QWindow_maximumHeight>(&mut self, value: T) -> i32 {
    return value.maximumHeight(self);
    // return 1;
  }
}

pub trait QWindow_maximumHeight {
  fn maximumHeight(self, rsthis: &mut QWindow) -> i32;
}

// proto:  int QWindow::maximumHeight();
impl<'a> /*trait*/ QWindow_maximumHeight for () {
  fn maximumHeight(self, rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13maximumHeightEv()};
    let mut ret = unsafe {_ZNK7QWindow13maximumHeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn focusObjectChanged<T: QWindow_focusObjectChanged>(&mut self, value: T)  {
     value.focusObjectChanged(self);
    // return 1;
  }
}

pub trait QWindow_focusObjectChanged {
  fn focusObjectChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::focusObjectChanged(QObject * object);
impl<'a> /*trait*/ QWindow_focusObjectChanged for (&'a mut QObject) {
  fn focusObjectChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow18focusObjectChangedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow18focusObjectChangedEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn isModal<T: QWindow_isModal>(&mut self, value: T) -> i8 {
    return value.isModal(self);
    // return 1;
  }
}

pub trait QWindow_isModal {
  fn isModal(self, rsthis: &mut QWindow) -> i8;
}

// proto:  bool QWindow::isModal();
impl<'a> /*trait*/ QWindow_isModal for () {
  fn isModal(self, rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow7isModalEv()};
    let mut ret = unsafe {_ZNK7QWindow7isModalEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn maximumWidthChanged<T: QWindow_maximumWidthChanged>(&mut self, value: T)  {
     value.maximumWidthChanged(self);
    // return 1;
  }
}

pub trait QWindow_maximumWidthChanged {
  fn maximumWidthChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::maximumWidthChanged(int arg);
impl<'a> /*trait*/ QWindow_maximumWidthChanged for (i32) {
  fn maximumWidthChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow19maximumWidthChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow19maximumWidthChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn geometry<T: QWindow_geometry>(&mut self, value: T) -> QRect {
    return value.geometry(self);
    // return 1;
  }
}

pub trait QWindow_geometry {
  fn geometry(self, rsthis: &mut QWindow) -> QRect;
}

// proto:  QRect QWindow::geometry();
impl<'a> /*trait*/ QWindow_geometry for () {
  fn geometry(self, rsthis: &mut QWindow) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8geometryEv()};
    let mut ret = unsafe {_ZNK7QWindow8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setParent<T: QWindow_setParent>(&mut self, value: T)  {
     value.setParent(self);
    // return 1;
  }
}

pub trait QWindow_setParent {
  fn setParent(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setParent(QWindow * parent);
impl<'a> /*trait*/ QWindow_setParent for (&'a mut QWindow) {
  fn setParent(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow9setParentEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn frameGeometry<T: QWindow_frameGeometry>(&mut self, value: T) -> QRect {
    return value.frameGeometry(self);
    // return 1;
  }
}

pub trait QWindow_frameGeometry {
  fn frameGeometry(self, rsthis: &mut QWindow) -> QRect;
}

// proto:  QRect QWindow::frameGeometry();
impl<'a> /*trait*/ QWindow_frameGeometry for () {
  fn frameGeometry(self, rsthis: &mut QWindow) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13frameGeometryEv()};
    let mut ret = unsafe {_ZNK7QWindow13frameGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn requestedFormat<T: QWindow_requestedFormat>(&mut self, value: T) -> QSurfaceFormat {
    return value.requestedFormat(self);
    // return 1;
  }
}

pub trait QWindow_requestedFormat {
  fn requestedFormat(self, rsthis: &mut QWindow) -> QSurfaceFormat;
}

// proto:  QSurfaceFormat QWindow::requestedFormat();
impl<'a> /*trait*/ QWindow_requestedFormat for () {
  fn requestedFormat(self, rsthis: &mut QWindow) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow15requestedFormatEv()};
    let mut ret = unsafe {_ZNK7QWindow15requestedFormatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setHeight<T: QWindow_setHeight>(&mut self, value: T)  {
     value.setHeight(self);
    // return 1;
  }
}

pub trait QWindow_setHeight {
  fn setHeight(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setHeight(int arg);
impl<'a> /*trait*/ QWindow_setHeight for (i32) {
  fn setHeight(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow9setHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn requestActivate<T: QWindow_requestActivate>(&mut self, value: T)  {
     value.requestActivate(self);
    // return 1;
  }
}

pub trait QWindow_requestActivate {
  fn requestActivate(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::requestActivate();
impl<'a> /*trait*/ QWindow_requestActivate for () {
  fn requestActivate(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow15requestActivateEv()};
     unsafe {_ZN7QWindow15requestActivateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn mapFromGlobal<T: QWindow_mapFromGlobal>(&mut self, value: T) -> QPoint {
    return value.mapFromGlobal(self);
    // return 1;
  }
}

pub trait QWindow_mapFromGlobal {
  fn mapFromGlobal(self, rsthis: &mut QWindow) -> QPoint;
}

// proto:  QPoint QWindow::mapFromGlobal(const QPoint & pos);
impl<'a> /*trait*/ QWindow_mapFromGlobal for (&'a  QPoint) {
  fn mapFromGlobal(self, rsthis: &mut QWindow) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13mapFromGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWindow13mapFromGlobalERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn windowTitleChanged<T: QWindow_windowTitleChanged>(&mut self, value: T)  {
     value.windowTitleChanged(self);
    // return 1;
  }
}

pub trait QWindow_windowTitleChanged {
  fn windowTitleChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::windowTitleChanged(const QString & title);
impl<'a> /*trait*/ QWindow_windowTitleChanged for (&'a  QString) {
  fn windowTitleChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow18windowTitleChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow18windowTitleChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn y<T: QWindow_y>(&mut self, value: T) -> i32 {
    return value.y(self);
    // return 1;
  }
}

pub trait QWindow_y {
  fn y(self, rsthis: &mut QWindow) -> i32;
}

// proto:  int QWindow::y();
impl<'a> /*trait*/ QWindow_y for () {
  fn y(self, rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow1yEv()};
    let mut ret = unsafe {_ZNK7QWindow1yEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn width<T: QWindow_width>(&mut self, value: T) -> i32 {
    return value.width(self);
    // return 1;
  }
}

pub trait QWindow_width {
  fn width(self, rsthis: &mut QWindow) -> i32;
}

// proto:  int QWindow::width();
impl<'a> /*trait*/ QWindow_width for () {
  fn width(self, rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow5widthEv()};
    let mut ret = unsafe {_ZNK7QWindow5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setFilePath<T: QWindow_setFilePath>(&mut self, value: T)  {
     value.setFilePath(self);
    // return 1;
  }
}

pub trait QWindow_setFilePath {
  fn setFilePath(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setFilePath(const QString & filePath);
impl<'a> /*trait*/ QWindow_setFilePath for (&'a  QString) {
  fn setFilePath(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setFilePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow11setFilePathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setCursor<T: QWindow_setCursor>(&mut self, value: T)  {
     value.setCursor(self);
    // return 1;
  }
}

pub trait QWindow_setCursor {
  fn setCursor(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setCursor(const QCursor & );
impl<'a> /*trait*/ QWindow_setCursor for (&'a  QCursor) {
  fn setCursor(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow9setCursorERK7QCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setVisible<T: QWindow_setVisible>(&mut self, value: T)  {
     value.setVisible(self);
    // return 1;
  }
}

pub trait QWindow_setVisible {
  fn setVisible(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setVisible(bool visible);
impl<'a> /*trait*/ QWindow_setVisible for (i8) {
  fn setVisible(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QWindow10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn FreeQWindow<T: QWindow_FreeQWindow>(&mut self, value: T)  {
     value.FreeQWindow(self);
    // return 1;
  }
}

pub trait QWindow_FreeQWindow {
  fn FreeQWindow(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::FreeQWindow();
impl<'a> /*trait*/ QWindow_FreeQWindow for () {
  fn FreeQWindow(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindowD0Ev()};
     unsafe {_ZN7QWindowD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMouseGrabEnabled<T: QWindow_setMouseGrabEnabled>(&mut self, value: T) -> i8 {
    return value.setMouseGrabEnabled(self);
    // return 1;
  }
}

pub trait QWindow_setMouseGrabEnabled {
  fn setMouseGrabEnabled(self, rsthis: &mut QWindow) -> i8;
}

// proto:  bool QWindow::setMouseGrabEnabled(bool grab);
impl<'a> /*trait*/ QWindow_setMouseGrabEnabled for (i8) {
  fn setMouseGrabEnabled(self, rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow19setMouseGrabEnabledEb()};
    let arg0 = self  as int8_t;
    let mut ret = unsafe {_ZN7QWindow19setMouseGrabEnabledEb(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn isExposed<T: QWindow_isExposed>(&mut self, value: T) -> i8 {
    return value.isExposed(self);
    // return 1;
  }
}

pub trait QWindow_isExposed {
  fn isExposed(self, rsthis: &mut QWindow) -> i8;
}

// proto:  bool QWindow::isExposed();
impl<'a> /*trait*/ QWindow_isExposed for () {
  fn isExposed(self, rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow9isExposedEv()};
    let mut ret = unsafe {_ZNK7QWindow9isExposedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn heightChanged<T: QWindow_heightChanged>(&mut self, value: T)  {
     value.heightChanged(self);
    // return 1;
  }
}

pub trait QWindow_heightChanged {
  fn heightChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::heightChanged(int arg);
impl<'a> /*trait*/ QWindow_heightChanged for (i32) {
  fn heightChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13heightChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow13heightChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn minimumWidth<T: QWindow_minimumWidth>(&mut self, value: T) -> i32 {
    return value.minimumWidth(self);
    // return 1;
  }
}

pub trait QWindow_minimumWidth {
  fn minimumWidth(self, rsthis: &mut QWindow) -> i32;
}

// proto:  int QWindow::minimumWidth();
impl<'a> /*trait*/ QWindow_minimumWidth for () {
  fn minimumWidth(self, rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow12minimumWidthEv()};
    let mut ret = unsafe {_ZNK7QWindow12minimumWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setPosition<T: QWindow_setPosition>(&mut self, value: T)  {
     value.setPosition(self);
    // return 1;
  }
}

pub trait QWindow_setPosition {
  fn setPosition(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setPosition(const QPoint & pt);
impl<'a> /*trait*/ QWindow_setPosition for (&'a  QPoint) {
  fn setPosition(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setPositionERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow11setPositionERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn close<T: QWindow_close>(&mut self, value: T) -> i8 {
    return value.close(self);
    // return 1;
  }
}

pub trait QWindow_close {
  fn close(self, rsthis: &mut QWindow) -> i8;
}

// proto:  bool QWindow::close();
impl<'a> /*trait*/ QWindow_close for () {
  fn close(self, rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5closeEv()};
    let mut ret = unsafe {_ZN7QWindow5closeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn x<T: QWindow_x>(&mut self, value: T) -> i32 {
    return value.x(self);
    // return 1;
  }
}

pub trait QWindow_x {
  fn x(self, rsthis: &mut QWindow) -> i32;
}

// proto:  int QWindow::x();
impl<'a> /*trait*/ QWindow_x for () {
  fn x(self, rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow1xEv()};
    let mut ret = unsafe {_ZNK7QWindow1xEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMinimumWidth<T: QWindow_setMinimumWidth>(&mut self, value: T)  {
     value.setMinimumWidth(self);
    // return 1;
  }
}

pub trait QWindow_setMinimumWidth {
  fn setMinimumWidth(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setMinimumWidth(int w);
impl<'a> /*trait*/ QWindow_setMinimumWidth for (i32) {
  fn setMinimumWidth(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow15setMinimumWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow15setMinimumWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn mask<T: QWindow_mask>(&mut self, value: T) -> QRegion {
    return value.mask(self);
    // return 1;
  }
}

pub trait QWindow_mask {
  fn mask(self, rsthis: &mut QWindow) -> QRegion;
}

// proto:  QRegion QWindow::mask();
impl<'a> /*trait*/ QWindow_mask for () {
  fn mask(self, rsthis: &mut QWindow) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow4maskEv()};
    let mut ret = unsafe {_ZNK7QWindow4maskEv(rsthis.qclsinst)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn widthChanged<T: QWindow_widthChanged>(&mut self, value: T)  {
     value.widthChanged(self);
    // return 1;
  }
}

pub trait QWindow_widthChanged {
  fn widthChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::widthChanged(int arg);
impl<'a> /*trait*/ QWindow_widthChanged for (i32) {
  fn widthChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow12widthChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow12widthChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn parent<T: QWindow_parent>(&mut self, value: T) -> QWindow {
    return value.parent(self);
    // return 1;
  }
}

pub trait QWindow_parent {
  fn parent(self, rsthis: &mut QWindow) -> QWindow;
}

// proto:  QWindow * QWindow::parent();
impl<'a> /*trait*/ QWindow_parent for () {
  fn parent(self, rsthis: &mut QWindow) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6parentEv()};
    let mut ret = unsafe {_ZNK7QWindow6parentEv(rsthis.qclsinst)};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setFramePosition<T: QWindow_setFramePosition>(&mut self, value: T)  {
     value.setFramePosition(self);
    // return 1;
  }
}

pub trait QWindow_setFramePosition {
  fn setFramePosition(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setFramePosition(const QPoint & point);
impl<'a> /*trait*/ QWindow_setFramePosition for (&'a  QPoint) {
  fn setFramePosition(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setFramePositionERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow16setFramePositionERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QWindow::NewQWindow(QScreen * screen);
impl<'a> /*trait*/ QWindow_NewQWindow for (&'a mut QScreen) {
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

impl /*struct*/ QWindow {
  pub fn setGeometry<T: QWindow_setGeometry>(&mut self, value: T)  {
     value.setGeometry(self);
    // return 1;
  }
}

pub trait QWindow_setGeometry {
  fn setGeometry(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setGeometry(int posx, int posy, int w, int h);
impl<'a> /*trait*/ QWindow_setGeometry for (i32, i32, i32, i32) {
  fn setGeometry(self, rsthis: &mut QWindow)  {
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

impl /*struct*/ QWindow {
  pub fn setKeyboardGrabEnabled<T: QWindow_setKeyboardGrabEnabled>(&mut self, value: T) -> i8 {
    return value.setKeyboardGrabEnabled(self);
    // return 1;
  }
}

pub trait QWindow_setKeyboardGrabEnabled {
  fn setKeyboardGrabEnabled(self, rsthis: &mut QWindow) -> i8;
}

// proto:  bool QWindow::setKeyboardGrabEnabled(bool grab);
impl<'a> /*trait*/ QWindow_setKeyboardGrabEnabled for (i8) {
  fn setKeyboardGrabEnabled(self, rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow22setKeyboardGrabEnabledEb()};
    let arg0 = self  as int8_t;
    let mut ret = unsafe {_ZN7QWindow22setKeyboardGrabEnabledEb(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn metaObject<T: QWindow_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QWindow_metaObject {
  fn metaObject(self, rsthis: &mut QWindow) ;
}

// proto:  const QMetaObject * QWindow::metaObject();
impl<'a> /*trait*/ QWindow_metaObject for () {
  fn metaObject(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow10metaObjectEv()};
     unsafe {_ZNK7QWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QWindow::NewQWindow(QWindow * parent);
impl<'a> /*trait*/ QWindow_NewQWindow for (&'a mut QWindow) {
  fn NewQWindow(self) -> QWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindowC1EPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWindowC1EPS_(qthis, arg0)};
    let rsthis = QWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn activeChanged<T: QWindow_activeChanged>(&mut self, value: T)  {
     value.activeChanged(self);
    // return 1;
  }
}

pub trait QWindow_activeChanged {
  fn activeChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::activeChanged();
impl<'a> /*trait*/ QWindow_activeChanged for () {
  fn activeChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13activeChangedEv()};
     unsafe {_ZN7QWindow13activeChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setWidth<T: QWindow_setWidth>(&mut self, value: T)  {
     value.setWidth(self);
    // return 1;
  }
}

pub trait QWindow_setWidth {
  fn setWidth(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setWidth(int arg);
impl<'a> /*trait*/ QWindow_setWidth for (i32) {
  fn setWidth(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8setWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow8setWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setY<T: QWindow_setY>(&mut self, value: T)  {
     value.setY(self);
    // return 1;
  }
}

pub trait QWindow_setY {
  fn setY(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setY(int arg);
impl<'a> /*trait*/ QWindow_setY for (i32) {
  fn setY(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4setYEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow4setYEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn devicePixelRatio<T: QWindow_devicePixelRatio>(&mut self, value: T) -> f64 {
    return value.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QWindow_devicePixelRatio {
  fn devicePixelRatio(self, rsthis: &mut QWindow) -> f64;
}

// proto:  double QWindow::devicePixelRatio();
impl<'a> /*trait*/ QWindow_devicePixelRatio for () {
  fn devicePixelRatio(self, rsthis: &mut QWindow) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK7QWindow16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setBaseSize<T: QWindow_setBaseSize>(&mut self, value: T)  {
     value.setBaseSize(self);
    // return 1;
  }
}

pub trait QWindow_setBaseSize {
  fn setBaseSize(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setBaseSize(const QSize & size);
impl<'a> /*trait*/ QWindow_setBaseSize for (&'a  QSize) {
  fn setBaseSize(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setBaseSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow11setBaseSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn alert<T: QWindow_alert>(&mut self, value: T)  {
     value.alert(self);
    // return 1;
  }
}

pub trait QWindow_alert {
  fn alert(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::alert(int msec);
impl<'a> /*trait*/ QWindow_alert for (i32) {
  fn alert(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5alertEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow5alertEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn yChanged<T: QWindow_yChanged>(&mut self, value: T)  {
     value.yChanged(self);
    // return 1;
  }
}

pub trait QWindow_yChanged {
  fn yChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::yChanged(int arg);
impl<'a> /*trait*/ QWindow_yChanged for (i32) {
  fn yChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8yChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow8yChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn handle<T: QWindow_handle>(&mut self, value: T)  {
     value.handle(self);
    // return 1;
  }
}

pub trait QWindow_handle {
  fn handle(self, rsthis: &mut QWindow) ;
}

// proto:  QPlatformWindow * QWindow::handle();
impl<'a> /*trait*/ QWindow_handle for () {
  fn handle(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6handleEv()};
     unsafe {_ZNK7QWindow6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn destroy<T: QWindow_destroy>(&mut self, value: T)  {
     value.destroy(self);
    // return 1;
  }
}

pub trait QWindow_destroy {
  fn destroy(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::destroy();
impl<'a> /*trait*/ QWindow_destroy for () {
  fn destroy(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow7destroyEv()};
     unsafe {_ZN7QWindow7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn transientParent<T: QWindow_transientParent>(&mut self, value: T) -> QWindow {
    return value.transientParent(self);
    // return 1;
  }
}

pub trait QWindow_transientParent {
  fn transientParent(self, rsthis: &mut QWindow) -> QWindow;
}

// proto:  QWindow * QWindow::transientParent();
impl<'a> /*trait*/ QWindow_transientParent for () {
  fn transientParent(self, rsthis: &mut QWindow) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow15transientParentEv()};
    let mut ret = unsafe {_ZNK7QWindow15transientParentEv(rsthis.qclsinst)};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMinimumHeight<T: QWindow_setMinimumHeight>(&mut self, value: T)  {
     value.setMinimumHeight(self);
    // return 1;
  }
}

pub trait QWindow_setMinimumHeight {
  fn setMinimumHeight(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setMinimumHeight(int h);
impl<'a> /*trait*/ QWindow_setMinimumHeight for (i32) {
  fn setMinimumHeight(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setMinimumHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow16setMinimumHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn show<T: QWindow_show>(&mut self, value: T)  {
     value.show(self);
    // return 1;
  }
}

pub trait QWindow_show {
  fn show(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::show();
impl<'a> /*trait*/ QWindow_show for () {
  fn show(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4showEv()};
     unsafe {_ZN7QWindow4showEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn minimumWidthChanged<T: QWindow_minimumWidthChanged>(&mut self, value: T)  {
     value.minimumWidthChanged(self);
    // return 1;
  }
}

pub trait QWindow_minimumWidthChanged {
  fn minimumWidthChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::minimumWidthChanged(int arg);
impl<'a> /*trait*/ QWindow_minimumWidthChanged for (i32) {
  fn minimumWidthChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow19minimumWidthChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow19minimumWidthChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn baseSize<T: QWindow_baseSize>(&mut self, value: T) -> QSize {
    return value.baseSize(self);
    // return 1;
  }
}

pub trait QWindow_baseSize {
  fn baseSize(self, rsthis: &mut QWindow) -> QSize;
}

// proto:  QSize QWindow::baseSize();
impl<'a> /*trait*/ QWindow_baseSize for () {
  fn baseSize(self, rsthis: &mut QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8baseSizeEv()};
    let mut ret = unsafe {_ZNK7QWindow8baseSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn title<T: QWindow_title>(&mut self, value: T) -> QString {
    return value.title(self);
    // return 1;
  }
}

pub trait QWindow_title {
  fn title(self, rsthis: &mut QWindow) -> QString;
}

// proto:  QString QWindow::title();
impl<'a> /*trait*/ QWindow_title for () {
  fn title(self, rsthis: &mut QWindow) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow5titleEv()};
    let mut ret = unsafe {_ZNK7QWindow5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn showMaximized<T: QWindow_showMaximized>(&mut self, value: T)  {
     value.showMaximized(self);
    // return 1;
  }
}

pub trait QWindow_showMaximized {
  fn showMaximized(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::showMaximized();
impl<'a> /*trait*/ QWindow_showMaximized for () {
  fn showMaximized(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13showMaximizedEv()};
     unsafe {_ZN7QWindow13showMaximizedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn create<T: QWindow_create>(&mut self, value: T)  {
     value.create(self);
    // return 1;
  }
}

pub trait QWindow_create {
  fn create(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::create();
impl<'a> /*trait*/ QWindow_create for () {
  fn create(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow6createEv()};
     unsafe {_ZN7QWindow6createEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QWindow::resize(int w, int h);
impl<'a> /*trait*/ QWindow_resize for (i32, i32) {
  fn resize(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow6resizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QWindow6resizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn screen<T: QWindow_screen>(&mut self, value: T) -> QScreen {
    return value.screen(self);
    // return 1;
  }
}

pub trait QWindow_screen {
  fn screen(self, rsthis: &mut QWindow) -> QScreen;
}

// proto:  QScreen * QWindow::screen();
impl<'a> /*trait*/ QWindow_screen for () {
  fn screen(self, rsthis: &mut QWindow) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6screenEv()};
    let mut ret = unsafe {_ZNK7QWindow6screenEv(rsthis.qclsinst)};
    let mut ret1 = QScreen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QWindow::setPosition(int posx, int posy);
impl<'a> /*trait*/ QWindow_setPosition for (i32, i32) {
  fn setPosition(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setPositionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QWindow11setPositionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setOpacity<T: QWindow_setOpacity>(&mut self, value: T)  {
     value.setOpacity(self);
    // return 1;
  }
}

pub trait QWindow_setOpacity {
  fn setOpacity(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setOpacity(qreal level);
impl<'a> /*trait*/ QWindow_setOpacity for (f64) {
  fn setOpacity(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QWindow10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QWindow::setGeometry(const QRect & rect);
impl<'a> /*trait*/ QWindow_setGeometry for (&'a  QRect) {
  fn setGeometry(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setSizeIncrement<T: QWindow_setSizeIncrement>(&mut self, value: T)  {
     value.setSizeIncrement(self);
    // return 1;
  }
}

pub trait QWindow_setSizeIncrement {
  fn setSizeIncrement(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setSizeIncrement(const QSize & size);
impl<'a> /*trait*/ QWindow_setSizeIncrement for (&'a  QSize) {
  fn setSizeIncrement(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setSizeIncrementERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow16setSizeIncrementERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn showMinimized<T: QWindow_showMinimized>(&mut self, value: T)  {
     value.showMinimized(self);
    // return 1;
  }
}

pub trait QWindow_showMinimized {
  fn showMinimized(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::showMinimized();
impl<'a> /*trait*/ QWindow_showMinimized for () {
  fn showMinimized(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13showMinimizedEv()};
     unsafe {_ZN7QWindow13showMinimizedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn focusObject<T: QWindow_focusObject>(&mut self, value: T) -> QObject {
    return value.focusObject(self);
    // return 1;
  }
}

pub trait QWindow_focusObject {
  fn focusObject(self, rsthis: &mut QWindow) -> QObject;
}

// proto:  QObject * QWindow::focusObject();
impl<'a> /*trait*/ QWindow_focusObject for () {
  fn focusObject(self, rsthis: &mut QWindow) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11focusObjectEv()};
    let mut ret = unsafe {_ZNK7QWindow11focusObjectEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn isActive<T: QWindow_isActive>(&mut self, value: T) -> i8 {
    return value.isActive(self);
    // return 1;
  }
}

pub trait QWindow_isActive {
  fn isActive(self, rsthis: &mut QWindow) -> i8;
}

// proto:  bool QWindow::isActive();
impl<'a> /*trait*/ QWindow_isActive for () {
  fn isActive(self, rsthis: &mut QWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8isActiveEv()};
    let mut ret = unsafe {_ZNK7QWindow8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn maximumHeightChanged<T: QWindow_maximumHeightChanged>(&mut self, value: T)  {
     value.maximumHeightChanged(self);
    // return 1;
  }
}

pub trait QWindow_maximumHeightChanged {
  fn maximumHeightChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::maximumHeightChanged(int arg);
impl<'a> /*trait*/ QWindow_maximumHeightChanged for (i32) {
  fn maximumHeightChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow20maximumHeightChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow20maximumHeightChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn accessibleRoot<T: QWindow_accessibleRoot>(&mut self, value: T) -> QAccessibleInterface {
    return value.accessibleRoot(self);
    // return 1;
  }
}

pub trait QWindow_accessibleRoot {
  fn accessibleRoot(self, rsthis: &mut QWindow) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QWindow::accessibleRoot();
impl<'a> /*trait*/ QWindow_accessibleRoot for () {
  fn accessibleRoot(self, rsthis: &mut QWindow) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow14accessibleRootEv()};
    let mut ret = unsafe {_ZNK7QWindow14accessibleRootEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn cursor<T: QWindow_cursor>(&mut self, value: T) -> QCursor {
    return value.cursor(self);
    // return 1;
  }
}

pub trait QWindow_cursor {
  fn cursor(self, rsthis: &mut QWindow) -> QCursor;
}

// proto:  QCursor QWindow::cursor();
impl<'a> /*trait*/ QWindow_cursor for () {
  fn cursor(self, rsthis: &mut QWindow) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6cursorEv()};
    let mut ret = unsafe {_ZNK7QWindow6cursorEv(rsthis.qclsinst)};
    let mut ret1 = QCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setFormat<T: QWindow_setFormat>(&mut self, value: T)  {
     value.setFormat(self);
    // return 1;
  }
}

pub trait QWindow_setFormat {
  fn setFormat(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QWindow_setFormat for (&'a  QSurfaceFormat) {
  fn setFormat(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn showFullScreen<T: QWindow_showFullScreen>(&mut self, value: T)  {
     value.showFullScreen(self);
    // return 1;
  }
}

pub trait QWindow_showFullScreen {
  fn showFullScreen(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::showFullScreen();
impl<'a> /*trait*/ QWindow_showFullScreen for () {
  fn showFullScreen(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14showFullScreenEv()};
     unsafe {_ZN7QWindow14showFullScreenEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setX<T: QWindow_setX>(&mut self, value: T)  {
     value.setX(self);
    // return 1;
  }
}

pub trait QWindow_setX {
  fn setX(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setX(int arg);
impl<'a> /*trait*/ QWindow_setX for (i32) {
  fn setX(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4setXEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow4setXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn opacityChanged<T: QWindow_opacityChanged>(&mut self, value: T)  {
     value.opacityChanged(self);
    // return 1;
  }
}

pub trait QWindow_opacityChanged {
  fn opacityChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::opacityChanged(qreal opacity);
impl<'a> /*trait*/ QWindow_opacityChanged for (f64) {
  fn opacityChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14opacityChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QWindow14opacityChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn lower<T: QWindow_lower>(&mut self, value: T)  {
     value.lower(self);
    // return 1;
  }
}

pub trait QWindow_lower {
  fn lower(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::lower();
impl<'a> /*trait*/ QWindow_lower for () {
  fn lower(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5lowerEv()};
     unsafe {_ZN7QWindow5lowerEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn requestUpdate<T: QWindow_requestUpdate>(&mut self, value: T)  {
     value.requestUpdate(self);
    // return 1;
  }
}

pub trait QWindow_requestUpdate {
  fn requestUpdate(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::requestUpdate();
impl<'a> /*trait*/ QWindow_requestUpdate for () {
  fn requestUpdate(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13requestUpdateEv()};
     unsafe {_ZN7QWindow13requestUpdateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn hide<T: QWindow_hide>(&mut self, value: T)  {
     value.hide(self);
    // return 1;
  }
}

pub trait QWindow_hide {
  fn hide(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::hide();
impl<'a> /*trait*/ QWindow_hide for () {
  fn hide(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4hideEv()};
     unsafe {_ZN7QWindow4hideEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn minimumHeightChanged<T: QWindow_minimumHeightChanged>(&mut self, value: T)  {
     value.minimumHeightChanged(self);
    // return 1;
  }
}

pub trait QWindow_minimumHeightChanged {
  fn minimumHeightChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::minimumHeightChanged(int arg);
impl<'a> /*trait*/ QWindow_minimumHeightChanged for (i32) {
  fn minimumHeightChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow20minimumHeightChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow20minimumHeightChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMask<T: QWindow_setMask>(&mut self, value: T)  {
     value.setMask(self);
    // return 1;
  }
}

pub trait QWindow_setMask {
  fn setMask(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setMask(const QRegion & region);
impl<'a> /*trait*/ QWindow_setMask for (&'a  QRegion) {
  fn setMask(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow7setMaskERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow7setMaskERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMaximumSize<T: QWindow_setMaximumSize>(&mut self, value: T)  {
     value.setMaximumSize(self);
    // return 1;
  }
}

pub trait QWindow_setMaximumSize {
  fn setMaximumSize(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setMaximumSize(const QSize & size);
impl<'a> /*trait*/ QWindow_setMaximumSize for (&'a  QSize) {
  fn setMaximumSize(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14setMaximumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWindow14setMaximumSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn height<T: QWindow_height>(&mut self, value: T) -> i32 {
    return value.height(self);
    // return 1;
  }
}

pub trait QWindow_height {
  fn height(self, rsthis: &mut QWindow) -> i32;
}

// proto:  int QWindow::height();
impl<'a> /*trait*/ QWindow_height for () {
  fn height(self, rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6heightEv()};
    let mut ret = unsafe {_ZNK7QWindow6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn size<T: QWindow_size>(&mut self, value: T) -> QSize {
    return value.size(self);
    // return 1;
  }
}

pub trait QWindow_size {
  fn size(self, rsthis: &mut QWindow) -> QSize;
}

// proto:  QSize QWindow::size();
impl<'a> /*trait*/ QWindow_size for () {
  fn size(self, rsthis: &mut QWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow4sizeEv()};
    let mut ret = unsafe {_ZNK7QWindow4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn maximumWidth<T: QWindow_maximumWidth>(&mut self, value: T) -> i32 {
    return value.maximumWidth(self);
    // return 1;
  }
}

pub trait QWindow_maximumWidth {
  fn maximumWidth(self, rsthis: &mut QWindow) -> i32;
}

// proto:  int QWindow::maximumWidth();
impl<'a> /*trait*/ QWindow_maximumWidth for () {
  fn maximumWidth(self, rsthis: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow12maximumWidthEv()};
    let mut ret = unsafe {_ZNK7QWindow12maximumWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn position<T: QWindow_position>(&mut self, value: T) -> QPoint {
    return value.position(self);
    // return 1;
  }
}

pub trait QWindow_position {
  fn position(self, rsthis: &mut QWindow) -> QPoint;
}

// proto:  QPoint QWindow::position();
impl<'a> /*trait*/ QWindow_position for () {
  fn position(self, rsthis: &mut QWindow) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8positionEv()};
    let mut ret = unsafe {_ZNK7QWindow8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMaximumHeight<T: QWindow_setMaximumHeight>(&mut self, value: T)  {
     value.setMaximumHeight(self);
    // return 1;
  }
}

pub trait QWindow_setMaximumHeight {
  fn setMaximumHeight(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::setMaximumHeight(int h);
impl<'a> /*trait*/ QWindow_setMaximumHeight for (i32) {
  fn setMaximumHeight(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setMaximumHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWindow16setMaximumHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn filePath<T: QWindow_filePath>(&mut self, value: T) -> QString {
    return value.filePath(self);
    // return 1;
  }
}

pub trait QWindow_filePath {
  fn filePath(self, rsthis: &mut QWindow) -> QString;
}

// proto:  QString QWindow::filePath();
impl<'a> /*trait*/ QWindow_filePath for () {
  fn filePath(self, rsthis: &mut QWindow) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8filePathEv()};
    let mut ret = unsafe {_ZNK7QWindow8filePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn showNormal<T: QWindow_showNormal>(&mut self, value: T)  {
     value.showNormal(self);
    // return 1;
  }
}

pub trait QWindow_showNormal {
  fn showNormal(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::showNormal();
impl<'a> /*trait*/ QWindow_showNormal for () {
  fn showNormal(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow10showNormalEv()};
     unsafe {_ZN7QWindow10showNormalEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn framePosition<T: QWindow_framePosition>(&mut self, value: T) -> QPoint {
    return value.framePosition(self);
    // return 1;
  }
}

pub trait QWindow_framePosition {
  fn framePosition(self, rsthis: &mut QWindow) -> QPoint;
}

// proto:  QPoint QWindow::framePosition();
impl<'a> /*trait*/ QWindow_framePosition for () {
  fn framePosition(self, rsthis: &mut QWindow) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13framePositionEv()};
    let mut ret = unsafe {_ZNK7QWindow13framePositionEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn visibleChanged<T: QWindow_visibleChanged>(&mut self, value: T)  {
     value.visibleChanged(self);
    // return 1;
  }
}

pub trait QWindow_visibleChanged {
  fn visibleChanged(self, rsthis: &mut QWindow) ;
}

// proto:  void QWindow::visibleChanged(bool arg);
impl<'a> /*trait*/ QWindow_visibleChanged for (i8) {
  fn visibleChanged(self, rsthis: &mut QWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14visibleChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QWindow14visibleChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn icon<T: QWindow_icon>(&mut self, value: T) -> QIcon {
    return value.icon(self);
    // return 1;
  }
}

pub trait QWindow_icon {
  fn icon(self, rsthis: &mut QWindow) -> QIcon;
}

// proto:  QIcon QWindow::icon();
impl<'a> /*trait*/ QWindow_icon for () {
  fn icon(self, rsthis: &mut QWindow) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow4iconEv()};
    let mut ret = unsafe {_ZNK7QWindow4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

