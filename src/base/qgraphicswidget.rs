// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qgraphicslayout::QGraphicsLayout;
use super::qrectf::QRectF;
use super::qsizef::QSizeF;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qpalette::QPalette;
use super::qstyle::QStyle;
use super::qpainterpath::QPainterPath;
use super::qaction::QAction;
use super::qfont::QFont;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsWidget::setAutoFillBackground(bool enabled);
  fn _ZN15QGraphicsWidget21setAutoFillBackgroundEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QGraphicsWidget::setWindowTitle(const QString & title);
  fn _ZN15QGraphicsWidget14setWindowTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsWidget::setLayout(QGraphicsLayout * layout);
  fn _ZN15QGraphicsWidget9setLayoutEP15QGraphicsLayout(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsWidget::setGeometry(const QRectF & rect);
  fn _ZN15QGraphicsWidget11setGeometryERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRectF QGraphicsWidget::rect();
  fn _ZNK15QGraphicsWidget4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizeF QGraphicsWidget::size();
  fn _ZNK15QGraphicsWidget4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsWidget::releaseShortcut(int id);
  fn _ZN15QGraphicsWidget15releaseShortcutEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QGraphicsWidget::setWindowFrameMargins(qreal left, qreal top, qreal right, qreal bottom);
  fn _ZN15QGraphicsWidget21setWindowFrameMarginsEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  int QGraphicsWidget::type_();
  fn _ZNK15QGraphicsWidget4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGraphicsWidget::unsetLayoutDirection();
  fn _ZN15QGraphicsWidget20unsetLayoutDirectionEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsWidget::NewQGraphicsWidget(const QGraphicsWidget & );
  fn _ZN15QGraphicsWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRectF QGraphicsWidget::windowFrameGeometry();
  fn _ZNK15QGraphicsWidget19windowFrameGeometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsWidget::resize(qreal w, qreal h);
  fn _ZN15QGraphicsWidget6resizeEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  QRectF QGraphicsWidget::windowFrameRect();
  fn _ZNK15QGraphicsWidget15windowFrameRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsWidget::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN15QGraphicsWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QGraphicsWidget::adjustSize();
  fn _ZN15QGraphicsWidget10adjustSizeEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsWidget::paintWindowFrame(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN15QGraphicsWidget16paintWindowFrameEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  QPalette QGraphicsWidget::palette();
  fn _ZNK15QGraphicsWidget7paletteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsWidget::unsetWindowFrameMargins();
  fn _ZN15QGraphicsWidget23unsetWindowFrameMarginsEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsWidget::resize(const QSizeF & size);
  fn _ZN15QGraphicsWidget6resizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsWidget::setPalette(const QPalette & palette);
  fn _ZN15QGraphicsWidget10setPaletteERK8QPalette(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QGraphicsWidget::autoFillBackground();
  fn _ZNK15QGraphicsWidget18autoFillBackgroundEv(qthis: *mut c_void) -> int8_t;
  // proto:  QStyle * QGraphicsWidget::style();
  fn _ZNK15QGraphicsWidget5styleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QGraphicsWidget::shape();
  fn _ZNK15QGraphicsWidget5shapeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsWidget::setShortcutEnabled(int id, bool enabled);
  fn _ZN15QGraphicsWidget18setShortcutEnabledEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  void QGraphicsWidget::removeAction(QAction * action);
  fn _ZN15QGraphicsWidget12removeActionEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsWidget::insertAction(QAction * before, QAction * action);
  fn _ZN15QGraphicsWidget12insertActionEP7QActionS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QGraphicsWidget::close();
  fn _ZN15QGraphicsWidget5closeEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QGraphicsWidget::metaObject();
  fn _ZNK15QGraphicsWidget10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QRectF QGraphicsWidget::boundingRect();
  fn _ZNK15QGraphicsWidget12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsWidget::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
  fn _ZN15QGraphicsWidget18setContentsMarginsEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  void QGraphicsWidget::setFont(const QFont & font);
  fn _ZN15QGraphicsWidget7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsWidget::geometryChanged();
  fn _ZN15QGraphicsWidget15geometryChangedEv(qthis: *mut c_void) ;
  // proto:  QString QGraphicsWidget::windowTitle();
  fn _ZNK15QGraphicsWidget11windowTitleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsLayout * QGraphicsWidget::layout();
  fn _ZNK15QGraphicsWidget6layoutEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsWidget::FreeQGraphicsWidget();
  fn _ZN15QGraphicsWidgetD0Ev(qthis: *mut c_void) ;
  // proto:  QGraphicsWidget * QGraphicsWidget::focusWidget();
  fn _ZNK15QGraphicsWidget11focusWidgetEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsWidget::addAction(QAction * action);
  fn _ZN15QGraphicsWidget9addActionEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QFont QGraphicsWidget::font();
  fn _ZNK15QGraphicsWidget4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QList<QAction *> QGraphicsWidget::actions();
  fn _ZNK15QGraphicsWidget7actionsEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsWidget::layoutChanged();
  fn _ZN15QGraphicsWidget13layoutChangedEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsWidget::setShortcutAutoRepeat(int id, bool enabled);
  fn _ZN15QGraphicsWidget21setShortcutAutoRepeatEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto: static void QGraphicsWidget::setTabOrder(QGraphicsWidget * first, QGraphicsWidget * second);
  fn _ZN15QGraphicsWidget11setTabOrderEPS_S0_(arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QGraphicsWidget::getWindowFrameMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn _ZNK15QGraphicsWidget21getWindowFrameMarginsEPdS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) ;
  // proto:  void QGraphicsWidget::setStyle(QStyle * style);
  fn _ZN15QGraphicsWidget8setStyleEP6QStyle(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsWidget::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn _ZNK15QGraphicsWidget18getContentsMarginsEPdS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) ;
  // proto:  bool QGraphicsWidget::isActiveWindow();
  fn _ZNK15QGraphicsWidget14isActiveWindowEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsWidget::setGeometry(qreal x, qreal y, qreal w, qreal h);
  fn _ZN15QGraphicsWidget11setGeometryEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
}

// body block begin
// class sizeof(QGraphicsWidget)=1
pub struct QGraphicsWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsWidget {
  pub fn setAutoFillBackground<RetType, T: QGraphicsWidget_setAutoFillBackground<RetType>>(&mut self, value: T) -> RetType {
    return value.setAutoFillBackground(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setAutoFillBackground<RetType> {
  fn setAutoFillBackground(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::setAutoFillBackground(bool enabled);
impl<'a> /*trait*/ QGraphicsWidget_setAutoFillBackground<()> for (i8) {
  fn setAutoFillBackground(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget21setAutoFillBackgroundEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QGraphicsWidget21setAutoFillBackgroundEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setWindowTitle<RetType, T: QGraphicsWidget_setWindowTitle<RetType>>(&mut self, value: T) -> RetType {
    return value.setWindowTitle(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setWindowTitle<RetType> {
  fn setWindowTitle(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::setWindowTitle(const QString & title);
impl<'a> /*trait*/ QGraphicsWidget_setWindowTitle<()> for (&'a  QString) {
  fn setWindowTitle(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget14setWindowTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget14setWindowTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setLayout<RetType, T: QGraphicsWidget_setLayout<RetType>>(&mut self, value: T) -> RetType {
    return value.setLayout(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setLayout<RetType> {
  fn setLayout(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::setLayout(QGraphicsLayout * layout);
impl<'a> /*trait*/ QGraphicsWidget_setLayout<()> for (&'a mut QGraphicsLayout) {
  fn setLayout(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget9setLayoutEP15QGraphicsLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget9setLayoutEP15QGraphicsLayout(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setGeometry<RetType, T: QGraphicsWidget_setGeometry<RetType>>(&mut self, value: T) -> RetType {
    return value.setGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setGeometry<RetType> {
  fn setGeometry(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsWidget_setGeometry<()> for (&'a  QRectF) {
  fn setGeometry(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget11setGeometryERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn rect<RetType, T: QGraphicsWidget_rect<RetType>>(&mut self, value: T) -> RetType {
    return value.rect(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_rect<RetType> {
  fn rect(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  QRectF QGraphicsWidget::rect();
impl<'a> /*trait*/ QGraphicsWidget_rect<QRectF> for () {
  fn rect(self, rsthis: &mut QGraphicsWidget) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget4rectEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn size<RetType, T: QGraphicsWidget_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_size<RetType> {
  fn size(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  QSizeF QGraphicsWidget::size();
impl<'a> /*trait*/ QGraphicsWidget_size<QSizeF> for () {
  fn size(self, rsthis: &mut QGraphicsWidget) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget4sizeEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn releaseShortcut<RetType, T: QGraphicsWidget_releaseShortcut<RetType>>(&mut self, value: T) -> RetType {
    return value.releaseShortcut(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_releaseShortcut<RetType> {
  fn releaseShortcut(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::releaseShortcut(int id);
impl<'a> /*trait*/ QGraphicsWidget_releaseShortcut<()> for (i32) {
  fn releaseShortcut(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget15releaseShortcutEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QGraphicsWidget15releaseShortcutEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setWindowFrameMargins<RetType, T: QGraphicsWidget_setWindowFrameMargins<RetType>>(&mut self, value: T) -> RetType {
    return value.setWindowFrameMargins(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setWindowFrameMargins<RetType> {
  fn setWindowFrameMargins(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::setWindowFrameMargins(qreal left, qreal top, qreal right, qreal bottom);
impl<'a> /*trait*/ QGraphicsWidget_setWindowFrameMargins<()> for (f64, f64, f64, f64) {
  fn setWindowFrameMargins(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget21setWindowFrameMarginsEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN15QGraphicsWidget21setWindowFrameMarginsEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn type_<RetType, T: QGraphicsWidget_type_<RetType>>(&mut self, value: T) -> RetType {
    return value.type_(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_type_<RetType> {
  fn type_(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  int QGraphicsWidget::type_();
impl<'a> /*trait*/ QGraphicsWidget_type_<i32> for () {
  fn type_(self, rsthis: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget4typeEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn unsetLayoutDirection<RetType, T: QGraphicsWidget_unsetLayoutDirection<RetType>>(&mut self, value: T) -> RetType {
    return value.unsetLayoutDirection(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_unsetLayoutDirection<RetType> {
  fn unsetLayoutDirection(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::unsetLayoutDirection();
impl<'a> /*trait*/ QGraphicsWidget_unsetLayoutDirection<()> for () {
  fn unsetLayoutDirection(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget20unsetLayoutDirectionEv()};
     unsafe {_ZN15QGraphicsWidget20unsetLayoutDirectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn NewQGraphicsWidget<T: QGraphicsWidget_NewQGraphicsWidget>(value: T) -> QGraphicsWidget {
    let rsthis = value.NewQGraphicsWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsWidget_NewQGraphicsWidget {
  fn NewQGraphicsWidget(self) -> QGraphicsWidget;
}

// proto: void QGraphicsWidget::NewQGraphicsWidget(const QGraphicsWidget & );
impl<'a> /*trait*/ QGraphicsWidget_NewQGraphicsWidget for (&'a  QGraphicsWidget) {
  fn NewQGraphicsWidget(self) -> QGraphicsWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn windowFrameGeometry<RetType, T: QGraphicsWidget_windowFrameGeometry<RetType>>(&mut self, value: T) -> RetType {
    return value.windowFrameGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_windowFrameGeometry<RetType> {
  fn windowFrameGeometry(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  QRectF QGraphicsWidget::windowFrameGeometry();
impl<'a> /*trait*/ QGraphicsWidget_windowFrameGeometry<QRectF> for () {
  fn windowFrameGeometry(self, rsthis: &mut QGraphicsWidget) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget19windowFrameGeometryEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget19windowFrameGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn resize<RetType, T: QGraphicsWidget_resize<RetType>>(&mut self, value: T) -> RetType {
    return value.resize(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_resize<RetType> {
  fn resize(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::resize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsWidget_resize<()> for (f64, f64) {
  fn resize(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget6resizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN15QGraphicsWidget6resizeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn windowFrameRect<RetType, T: QGraphicsWidget_windowFrameRect<RetType>>(&mut self, value: T) -> RetType {
    return value.windowFrameRect(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_windowFrameRect<RetType> {
  fn windowFrameRect(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  QRectF QGraphicsWidget::windowFrameRect();
impl<'a> /*trait*/ QGraphicsWidget_windowFrameRect<QRectF> for () {
  fn windowFrameRect(self, rsthis: &mut QGraphicsWidget) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget15windowFrameRectEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget15windowFrameRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn paint<RetType, T: QGraphicsWidget_paint<RetType>>(&mut self, value: T) -> RetType {
    return value.paint(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_paint<RetType> {
  fn paint(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsWidget_paint<()> for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn adjustSize<RetType, T: QGraphicsWidget_adjustSize<RetType>>(&mut self, value: T) -> RetType {
    return value.adjustSize(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_adjustSize<RetType> {
  fn adjustSize(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::adjustSize();
impl<'a> /*trait*/ QGraphicsWidget_adjustSize<()> for () {
  fn adjustSize(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget10adjustSizeEv()};
     unsafe {_ZN15QGraphicsWidget10adjustSizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn paintWindowFrame<RetType, T: QGraphicsWidget_paintWindowFrame<RetType>>(&mut self, value: T) -> RetType {
    return value.paintWindowFrame(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_paintWindowFrame<RetType> {
  fn paintWindowFrame(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::paintWindowFrame(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsWidget_paintWindowFrame<()> for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paintWindowFrame(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget16paintWindowFrameEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget16paintWindowFrameEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn palette<RetType, T: QGraphicsWidget_palette<RetType>>(&mut self, value: T) -> RetType {
    return value.palette(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_palette<RetType> {
  fn palette(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  QPalette QGraphicsWidget::palette();
impl<'a> /*trait*/ QGraphicsWidget_palette<QPalette> for () {
  fn palette(self, rsthis: &mut QGraphicsWidget) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget7paletteEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget7paletteEv(rsthis.qclsinst)};
    let mut ret1 = QPalette{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn unsetWindowFrameMargins<RetType, T: QGraphicsWidget_unsetWindowFrameMargins<RetType>>(&mut self, value: T) -> RetType {
    return value.unsetWindowFrameMargins(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_unsetWindowFrameMargins<RetType> {
  fn unsetWindowFrameMargins(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::unsetWindowFrameMargins();
impl<'a> /*trait*/ QGraphicsWidget_unsetWindowFrameMargins<()> for () {
  fn unsetWindowFrameMargins(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget23unsetWindowFrameMarginsEv()};
     unsafe {_ZN15QGraphicsWidget23unsetWindowFrameMarginsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QGraphicsWidget::resize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsWidget_resize<()> for (&'a  QSizeF) {
  fn resize(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget6resizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget6resizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setPalette<RetType, T: QGraphicsWidget_setPalette<RetType>>(&mut self, value: T) -> RetType {
    return value.setPalette(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setPalette<RetType> {
  fn setPalette(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::setPalette(const QPalette & palette);
impl<'a> /*trait*/ QGraphicsWidget_setPalette<()> for (&'a  QPalette) {
  fn setPalette(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget10setPaletteERK8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn autoFillBackground<RetType, T: QGraphicsWidget_autoFillBackground<RetType>>(&mut self, value: T) -> RetType {
    return value.autoFillBackground(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_autoFillBackground<RetType> {
  fn autoFillBackground(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  bool QGraphicsWidget::autoFillBackground();
impl<'a> /*trait*/ QGraphicsWidget_autoFillBackground<i8> for () {
  fn autoFillBackground(self, rsthis: &mut QGraphicsWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget18autoFillBackgroundEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget18autoFillBackgroundEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn style<RetType, T: QGraphicsWidget_style<RetType>>(&mut self, value: T) -> RetType {
    return value.style(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_style<RetType> {
  fn style(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  QStyle * QGraphicsWidget::style();
impl<'a> /*trait*/ QGraphicsWidget_style<QStyle> for () {
  fn style(self, rsthis: &mut QGraphicsWidget) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget5styleEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget5styleEv(rsthis.qclsinst)};
    let mut ret1 = QStyle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn shape<RetType, T: QGraphicsWidget_shape<RetType>>(&mut self, value: T) -> RetType {
    return value.shape(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_shape<RetType> {
  fn shape(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  QPainterPath QGraphicsWidget::shape();
impl<'a> /*trait*/ QGraphicsWidget_shape<QPainterPath> for () {
  fn shape(self, rsthis: &mut QGraphicsWidget) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget5shapeEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setShortcutEnabled<RetType, T: QGraphicsWidget_setShortcutEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setShortcutEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setShortcutEnabled<RetType> {
  fn setShortcutEnabled(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::setShortcutEnabled(int id, bool enabled);
impl<'a> /*trait*/ QGraphicsWidget_setShortcutEnabled<()> for (i32, i8) {
  fn setShortcutEnabled(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget18setShortcutEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN15QGraphicsWidget18setShortcutEnabledEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn removeAction<RetType, T: QGraphicsWidget_removeAction<RetType>>(&mut self, value: T) -> RetType {
    return value.removeAction(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_removeAction<RetType> {
  fn removeAction(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::removeAction(QAction * action);
impl<'a> /*trait*/ QGraphicsWidget_removeAction<()> for (&'a mut QAction) {
  fn removeAction(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget12removeActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget12removeActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn insertAction<RetType, T: QGraphicsWidget_insertAction<RetType>>(&mut self, value: T) -> RetType {
    return value.insertAction(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_insertAction<RetType> {
  fn insertAction(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::insertAction(QAction * before, QAction * action);
impl<'a> /*trait*/ QGraphicsWidget_insertAction<()> for (&'a mut QAction, &'a mut QAction) {
  fn insertAction(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget12insertActionEP7QActionS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget12insertActionEP7QActionS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn close<RetType, T: QGraphicsWidget_close<RetType>>(&mut self, value: T) -> RetType {
    return value.close(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_close<RetType> {
  fn close(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  bool QGraphicsWidget::close();
impl<'a> /*trait*/ QGraphicsWidget_close<i8> for () {
  fn close(self, rsthis: &mut QGraphicsWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget5closeEv()};
    let mut ret = unsafe {_ZN15QGraphicsWidget5closeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn metaObject<RetType, T: QGraphicsWidget_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  const QMetaObject * QGraphicsWidget::metaObject();
impl<'a> /*trait*/ QGraphicsWidget_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget10metaObjectEv()};
     unsafe {_ZNK15QGraphicsWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn boundingRect<RetType, T: QGraphicsWidget_boundingRect<RetType>>(&mut self, value: T) -> RetType {
    return value.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_boundingRect<RetType> {
  fn boundingRect(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  QRectF QGraphicsWidget::boundingRect();
impl<'a> /*trait*/ QGraphicsWidget_boundingRect<QRectF> for () {
  fn boundingRect(self, rsthis: &mut QGraphicsWidget) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget12boundingRectEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setContentsMargins<RetType, T: QGraphicsWidget_setContentsMargins<RetType>>(&mut self, value: T) -> RetType {
    return value.setContentsMargins(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setContentsMargins<RetType> {
  fn setContentsMargins(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
impl<'a> /*trait*/ QGraphicsWidget_setContentsMargins<()> for (f64, f64, f64, f64) {
  fn setContentsMargins(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget18setContentsMarginsEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN15QGraphicsWidget18setContentsMarginsEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setFont<RetType, T: QGraphicsWidget_setFont<RetType>>(&mut self, value: T) -> RetType {
    return value.setFont(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setFont<RetType> {
  fn setFont(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::setFont(const QFont & font);
impl<'a> /*trait*/ QGraphicsWidget_setFont<()> for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn geometryChanged<RetType, T: QGraphicsWidget_geometryChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.geometryChanged(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_geometryChanged<RetType> {
  fn geometryChanged(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::geometryChanged();
impl<'a> /*trait*/ QGraphicsWidget_geometryChanged<()> for () {
  fn geometryChanged(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget15geometryChangedEv()};
     unsafe {_ZN15QGraphicsWidget15geometryChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn windowTitle<RetType, T: QGraphicsWidget_windowTitle<RetType>>(&mut self, value: T) -> RetType {
    return value.windowTitle(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_windowTitle<RetType> {
  fn windowTitle(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  QString QGraphicsWidget::windowTitle();
impl<'a> /*trait*/ QGraphicsWidget_windowTitle<QString> for () {
  fn windowTitle(self, rsthis: &mut QGraphicsWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget11windowTitleEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget11windowTitleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn layout<RetType, T: QGraphicsWidget_layout<RetType>>(&mut self, value: T) -> RetType {
    return value.layout(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_layout<RetType> {
  fn layout(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  QGraphicsLayout * QGraphicsWidget::layout();
impl<'a> /*trait*/ QGraphicsWidget_layout<()> for () {
  fn layout(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget6layoutEv()};
     unsafe {_ZNK15QGraphicsWidget6layoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn FreeQGraphicsWidget<RetType, T: QGraphicsWidget_FreeQGraphicsWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQGraphicsWidget(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_FreeQGraphicsWidget<RetType> {
  fn FreeQGraphicsWidget(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::FreeQGraphicsWidget();
impl<'a> /*trait*/ QGraphicsWidget_FreeQGraphicsWidget<()> for () {
  fn FreeQGraphicsWidget(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidgetD0Ev()};
     unsafe {_ZN15QGraphicsWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn focusWidget<RetType, T: QGraphicsWidget_focusWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.focusWidget(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_focusWidget<RetType> {
  fn focusWidget(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  QGraphicsWidget * QGraphicsWidget::focusWidget();
impl<'a> /*trait*/ QGraphicsWidget_focusWidget<()> for () {
  fn focusWidget(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget11focusWidgetEv()};
     unsafe {_ZNK15QGraphicsWidget11focusWidgetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn addAction<RetType, T: QGraphicsWidget_addAction<RetType>>(&mut self, value: T) -> RetType {
    return value.addAction(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_addAction<RetType> {
  fn addAction(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::addAction(QAction * action);
impl<'a> /*trait*/ QGraphicsWidget_addAction<()> for (&'a mut QAction) {
  fn addAction(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget9addActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget9addActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn font<RetType, T: QGraphicsWidget_font<RetType>>(&mut self, value: T) -> RetType {
    return value.font(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_font<RetType> {
  fn font(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  QFont QGraphicsWidget::font();
impl<'a> /*trait*/ QGraphicsWidget_font<QFont> for () {
  fn font(self, rsthis: &mut QGraphicsWidget) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget4fontEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn actions<RetType, T: QGraphicsWidget_actions<RetType>>(&mut self, value: T) -> RetType {
    return value.actions(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_actions<RetType> {
  fn actions(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  QList<QAction *> QGraphicsWidget::actions();
impl<'a> /*trait*/ QGraphicsWidget_actions<()> for () {
  fn actions(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget7actionsEv()};
     unsafe {_ZNK15QGraphicsWidget7actionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn layoutChanged<RetType, T: QGraphicsWidget_layoutChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.layoutChanged(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_layoutChanged<RetType> {
  fn layoutChanged(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::layoutChanged();
impl<'a> /*trait*/ QGraphicsWidget_layoutChanged<()> for () {
  fn layoutChanged(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget13layoutChangedEv()};
     unsafe {_ZN15QGraphicsWidget13layoutChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setShortcutAutoRepeat<RetType, T: QGraphicsWidget_setShortcutAutoRepeat<RetType>>(&mut self, value: T) -> RetType {
    return value.setShortcutAutoRepeat(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setShortcutAutoRepeat<RetType> {
  fn setShortcutAutoRepeat(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::setShortcutAutoRepeat(int id, bool enabled);
impl<'a> /*trait*/ QGraphicsWidget_setShortcutAutoRepeat<()> for (i32, i8) {
  fn setShortcutAutoRepeat(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget21setShortcutAutoRepeatEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN15QGraphicsWidget21setShortcutAutoRepeatEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setTabOrder<RetType, T: QGraphicsWidget_setTabOrder<RetType>>(&mut self, value: T) -> RetType {
    return value.setTabOrder(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setTabOrder<RetType> {
  fn setTabOrder(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto: static void QGraphicsWidget::setTabOrder(QGraphicsWidget * first, QGraphicsWidget * second);
impl<'a> /*trait*/ QGraphicsWidget_setTabOrder<()> for (&'a mut QGraphicsWidget, &'a mut QGraphicsWidget) {
  fn setTabOrder(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget11setTabOrderEPS_S0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget11setTabOrderEPS_S0_(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn getWindowFrameMargins<RetType, T: QGraphicsWidget_getWindowFrameMargins<RetType>>(&mut self, value: T) -> RetType {
    return value.getWindowFrameMargins(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_getWindowFrameMargins<RetType> {
  fn getWindowFrameMargins(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::getWindowFrameMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsWidget_getWindowFrameMargins<()> for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getWindowFrameMargins(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget21getWindowFrameMarginsEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
     unsafe {_ZNK15QGraphicsWidget21getWindowFrameMarginsEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setStyle<RetType, T: QGraphicsWidget_setStyle<RetType>>(&mut self, value: T) -> RetType {
    return value.setStyle(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setStyle<RetType> {
  fn setStyle(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::setStyle(QStyle * style);
impl<'a> /*trait*/ QGraphicsWidget_setStyle<()> for (&'a mut QStyle) {
  fn setStyle(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget8setStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget8setStyleEP6QStyle(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn getContentsMargins<RetType, T: QGraphicsWidget_getContentsMargins<RetType>>(&mut self, value: T) -> RetType {
    return value.getContentsMargins(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_getContentsMargins<RetType> {
  fn getContentsMargins(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  void QGraphicsWidget::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsWidget_getContentsMargins<()> for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getContentsMargins(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget18getContentsMarginsEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
     unsafe {_ZNK15QGraphicsWidget18getContentsMarginsEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn isActiveWindow<RetType, T: QGraphicsWidget_isActiveWindow<RetType>>(&mut self, value: T) -> RetType {
    return value.isActiveWindow(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_isActiveWindow<RetType> {
  fn isActiveWindow(self, rsthis: &mut QGraphicsWidget) -> RetType;
}

// proto:  bool QGraphicsWidget::isActiveWindow();
impl<'a> /*trait*/ QGraphicsWidget_isActiveWindow<i8> for () {
  fn isActiveWindow(self, rsthis: &mut QGraphicsWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget14isActiveWindowEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget14isActiveWindowEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QGraphicsWidget::setGeometry(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsWidget_setGeometry<()> for (f64, f64, f64, f64) {
  fn setGeometry(self, rsthis: &mut QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget11setGeometryEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN15QGraphicsWidget11setGeometryEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

