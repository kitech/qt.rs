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
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qsizef::QSizeF;
use super::qpalette::QPalette;
use super::qaction::QAction;
use super::qfont::QFont;
use super::qstyle::QStyle;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsWidget::setAutoFillBackground(bool enabled);
  fn _ZN15QGraphicsWidget21setAutoFillBackgroundEb(arg0: int8_t) -> i32;
  // proto: void QGraphicsWidget::setWindowTitle(const QString & title);
  fn _ZN15QGraphicsWidget14setWindowTitleERK7QString(arg0: *const c_void) -> i32;
  // proto: void QGraphicsWidget::setLayout(QGraphicsLayout * layout);
  fn _ZN15QGraphicsWidget9setLayoutEP15QGraphicsLayout(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsWidget::setGeometry(const QRectF & rect);
  fn _ZN15QGraphicsWidget11setGeometryERK6QRectF(arg0: *const c_void) -> i32;
  // proto: QRectF QGraphicsWidget::rect();
  fn _ZNK15QGraphicsWidget4rectEv() -> i32;
  // proto: QSizeF QGraphicsWidget::size();
  fn _ZNK15QGraphicsWidget4sizeEv() -> i32;
  // proto: void QGraphicsWidget::releaseShortcut(int id);
  fn _ZN15QGraphicsWidget15releaseShortcutEi(arg0: c_int) -> i32;
  // proto: void QGraphicsWidget::setWindowFrameMargins(qreal left, qreal top, qreal right, qreal bottom);
  fn _ZN15QGraphicsWidget21setWindowFrameMarginsEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: int QGraphicsWidget::type_();
  fn _ZNK15QGraphicsWidget4typeEv() -> i32;
  // proto: void QGraphicsWidget::unsetLayoutDirection();
  fn _ZN15QGraphicsWidget20unsetLayoutDirectionEv() -> i32;
  // proto: void QGraphicsWidget::NewQGraphicsWidget(const QGraphicsWidget & );
  fn _ZN15QGraphicsWidgetC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QRectF QGraphicsWidget::windowFrameGeometry();
  fn _ZNK15QGraphicsWidget19windowFrameGeometryEv() -> i32;
  // proto: void QGraphicsWidget::resize(qreal w, qreal h);
  fn _ZN15QGraphicsWidget6resizeEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QRectF QGraphicsWidget::windowFrameRect();
  fn _ZNK15QGraphicsWidget15windowFrameRectEv() -> i32;
  // proto: void QGraphicsWidget::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN15QGraphicsWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: void QGraphicsWidget::adjustSize();
  fn _ZN15QGraphicsWidget10adjustSizeEv() -> i32;
  // proto: void QGraphicsWidget::paintWindowFrame(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN15QGraphicsWidget16paintWindowFrameEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: QPalette QGraphicsWidget::palette();
  fn _ZNK15QGraphicsWidget7paletteEv() -> i32;
  // proto: void QGraphicsWidget::unsetWindowFrameMargins();
  fn _ZN15QGraphicsWidget23unsetWindowFrameMarginsEv() -> i32;
  // proto: void QGraphicsWidget::resize(const QSizeF & size);
  fn _ZN15QGraphicsWidget6resizeERK6QSizeF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsWidget::setPalette(const QPalette & palette);
  fn _ZN15QGraphicsWidget10setPaletteERK8QPalette(arg0: *const c_void) -> i32;
  // proto: bool QGraphicsWidget::autoFillBackground();
  fn _ZNK15QGraphicsWidget18autoFillBackgroundEv() -> i32;
  // proto: QStyle * QGraphicsWidget::style();
  fn _ZNK15QGraphicsWidget5styleEv() -> i32;
  // proto: QPainterPath QGraphicsWidget::shape();
  fn _ZNK15QGraphicsWidget5shapeEv() -> i32;
  // proto: void QGraphicsWidget::setShortcutEnabled(int id, bool enabled);
  fn _ZN15QGraphicsWidget18setShortcutEnabledEib(arg0: c_int, arg1: int8_t) -> i32;
  // proto: void QGraphicsWidget::removeAction(QAction * action);
  fn _ZN15QGraphicsWidget12removeActionEP7QAction(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsWidget::insertAction(QAction * before, QAction * action);
  fn _ZN15QGraphicsWidget12insertActionEP7QActionS1_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: bool QGraphicsWidget::close();
  fn _ZN15QGraphicsWidget5closeEv() -> i32;
  // proto: const QMetaObject * QGraphicsWidget::metaObject();
  fn _ZNK15QGraphicsWidget10metaObjectEv() -> i32;
  // proto: QRectF QGraphicsWidget::boundingRect();
  fn _ZNK15QGraphicsWidget12boundingRectEv() -> i32;
  // proto: void QGraphicsWidget::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
  fn _ZN15QGraphicsWidget18setContentsMarginsEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsWidget::setFont(const QFont & font);
  fn _ZN15QGraphicsWidget7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: void QGraphicsWidget::geometryChanged();
  fn _ZN15QGraphicsWidget15geometryChangedEv() -> i32;
  // proto: QString QGraphicsWidget::windowTitle();
  fn _ZNK15QGraphicsWidget11windowTitleEv() -> i32;
  // proto: QGraphicsLayout * QGraphicsWidget::layout();
  fn _ZNK15QGraphicsWidget6layoutEv() -> i32;
  // proto: void QGraphicsWidget::FreeQGraphicsWidget();
  fn _ZN15QGraphicsWidgetD0Ev() -> i32;
  // proto: QGraphicsWidget * QGraphicsWidget::focusWidget();
  fn _ZNK15QGraphicsWidget11focusWidgetEv() -> i32;
  // proto: void QGraphicsWidget::addAction(QAction * action);
  fn _ZN15QGraphicsWidget9addActionEP7QAction(arg0: *mut c_void) -> i32;
  // proto: QFont QGraphicsWidget::font();
  fn _ZNK15QGraphicsWidget4fontEv() -> i32;
  // proto: QList<QAction *> QGraphicsWidget::actions();
  fn _ZNK15QGraphicsWidget7actionsEv() -> i32;
  // proto: void QGraphicsWidget::layoutChanged();
  fn _ZN15QGraphicsWidget13layoutChangedEv() -> i32;
  // proto: void QGraphicsWidget::setShortcutAutoRepeat(int id, bool enabled);
  fn _ZN15QGraphicsWidget21setShortcutAutoRepeatEib(arg0: c_int, arg1: int8_t) -> i32;
  // proto: void QGraphicsWidget::setTabOrder(QGraphicsWidget * first, QGraphicsWidget * second);
  fn _ZN15QGraphicsWidget11setTabOrderEPS_S0_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: void QGraphicsWidget::getWindowFrameMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn _ZNK15QGraphicsWidget21getWindowFrameMarginsEPdS0_S0_S0_(arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) -> i32;
  // proto: void QGraphicsWidget::setStyle(QStyle * style);
  fn _ZN15QGraphicsWidget8setStyleEP6QStyle(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsWidget::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn _ZNK15QGraphicsWidget18getContentsMarginsEPdS0_S0_S0_(arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) -> i32;
  // proto: bool QGraphicsWidget::isActiveWindow();
  fn _ZNK15QGraphicsWidget14isActiveWindowEv() -> i32;
  // proto: void QGraphicsWidget::setGeometry(qreal x, qreal y, qreal w, qreal h);
  fn _ZN15QGraphicsWidget11setGeometryEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
}

// body block begin
// class sizeof(QGraphicsWidget)=1
pub struct QGraphicsWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsWidget {
  pub fn setAutoFillBackground<T: QGraphicsWidget_setAutoFillBackground>(&mut self, value: T) -> i32 {
    value.setAutoFillBackground(self);
    return 1;
  }
}

pub trait QGraphicsWidget_setAutoFillBackground {
  fn setAutoFillBackground(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::setAutoFillBackground(bool enabled);
impl<'a> /*trait*/ QGraphicsWidget_setAutoFillBackground for (i8) {
  fn setAutoFillBackground(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget21setAutoFillBackgroundEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QGraphicsWidget21setAutoFillBackgroundEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setWindowTitle<T: QGraphicsWidget_setWindowTitle>(&mut self, value: T) -> i32 {
    value.setWindowTitle(self);
    return 1;
  }
}

pub trait QGraphicsWidget_setWindowTitle {
  fn setWindowTitle(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::setWindowTitle(const QString & title);
impl<'a> /*trait*/ QGraphicsWidget_setWindowTitle for (&'a  QString) {
  fn setWindowTitle(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget14setWindowTitleERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGraphicsWidget14setWindowTitleERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setLayout<T: QGraphicsWidget_setLayout>(&mut self, value: T) -> i32 {
    value.setLayout(self);
    return 1;
  }
}

pub trait QGraphicsWidget_setLayout {
  fn setLayout(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::setLayout(QGraphicsLayout * layout);
impl<'a> /*trait*/ QGraphicsWidget_setLayout for (&'a mut QGraphicsLayout) {
  fn setLayout(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget9setLayoutEP15QGraphicsLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsWidget9setLayoutEP15QGraphicsLayout(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setGeometry<T: QGraphicsWidget_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QGraphicsWidget_setGeometry {
  fn setGeometry(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsWidget_setGeometry for (&'a  QRectF) {
  fn setGeometry(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGraphicsWidget11setGeometryERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn rect<T: QGraphicsWidget_rect>(&mut self, value: T) -> i32 {
    value.rect(self);
    return 1;
  }
}

pub trait QGraphicsWidget_rect {
  fn rect(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: QRectF QGraphicsWidget::rect();
impl<'a> /*trait*/ QGraphicsWidget_rect for () {
  fn rect(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget4rectEv()};
    unsafe {_ZNK15QGraphicsWidget4rectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn size<T: QGraphicsWidget_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QGraphicsWidget_size {
  fn size(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: QSizeF QGraphicsWidget::size();
impl<'a> /*trait*/ QGraphicsWidget_size for () {
  fn size(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget4sizeEv()};
    unsafe {_ZNK15QGraphicsWidget4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn releaseShortcut<T: QGraphicsWidget_releaseShortcut>(&mut self, value: T) -> i32 {
    value.releaseShortcut(self);
    return 1;
  }
}

pub trait QGraphicsWidget_releaseShortcut {
  fn releaseShortcut(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::releaseShortcut(int id);
impl<'a> /*trait*/ QGraphicsWidget_releaseShortcut for (i32) {
  fn releaseShortcut(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget15releaseShortcutEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QGraphicsWidget15releaseShortcutEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setWindowFrameMargins<T: QGraphicsWidget_setWindowFrameMargins>(&mut self, value: T) -> i32 {
    value.setWindowFrameMargins(self);
    return 1;
  }
}

pub trait QGraphicsWidget_setWindowFrameMargins {
  fn setWindowFrameMargins(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::setWindowFrameMargins(qreal left, qreal top, qreal right, qreal bottom);
impl<'a> /*trait*/ QGraphicsWidget_setWindowFrameMargins for (f64, f64, f64, f64) {
  fn setWindowFrameMargins(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget21setWindowFrameMarginsEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN15QGraphicsWidget21setWindowFrameMarginsEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn type_<T: QGraphicsWidget_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QGraphicsWidget_type_ {
  fn type_(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: int QGraphicsWidget::type_();
impl<'a> /*trait*/ QGraphicsWidget_type_ for () {
  fn type_(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget4typeEv()};
    unsafe {_ZNK15QGraphicsWidget4typeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn unsetLayoutDirection<T: QGraphicsWidget_unsetLayoutDirection>(&mut self, value: T) -> i32 {
    value.unsetLayoutDirection(self);
    return 1;
  }
}

pub trait QGraphicsWidget_unsetLayoutDirection {
  fn unsetLayoutDirection(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::unsetLayoutDirection();
impl<'a> /*trait*/ QGraphicsWidget_unsetLayoutDirection for () {
  fn unsetLayoutDirection(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget20unsetLayoutDirectionEv()};
    unsafe {_ZN15QGraphicsWidget20unsetLayoutDirectionEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGraphicsWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn windowFrameGeometry<T: QGraphicsWidget_windowFrameGeometry>(&mut self, value: T) -> i32 {
    value.windowFrameGeometry(self);
    return 1;
  }
}

pub trait QGraphicsWidget_windowFrameGeometry {
  fn windowFrameGeometry(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: QRectF QGraphicsWidget::windowFrameGeometry();
impl<'a> /*trait*/ QGraphicsWidget_windowFrameGeometry for () {
  fn windowFrameGeometry(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget19windowFrameGeometryEv()};
    unsafe {_ZNK15QGraphicsWidget19windowFrameGeometryEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn resize<T: QGraphicsWidget_resize>(&mut self, value: T) -> i32 {
    value.resize(self);
    return 1;
  }
}

pub trait QGraphicsWidget_resize {
  fn resize(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::resize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsWidget_resize for (f64, f64) {
  fn resize(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget6resizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN15QGraphicsWidget6resizeEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn windowFrameRect<T: QGraphicsWidget_windowFrameRect>(&mut self, value: T) -> i32 {
    value.windowFrameRect(self);
    return 1;
  }
}

pub trait QGraphicsWidget_windowFrameRect {
  fn windowFrameRect(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: QRectF QGraphicsWidget::windowFrameRect();
impl<'a> /*trait*/ QGraphicsWidget_windowFrameRect for () {
  fn windowFrameRect(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget15windowFrameRectEv()};
    unsafe {_ZNK15QGraphicsWidget15windowFrameRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn paint<T: QGraphicsWidget_paint>(&mut self, value: T) -> i32 {
    value.paint(self);
    return 1;
  }
}

pub trait QGraphicsWidget_paint {
  fn paint(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsWidget_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn adjustSize<T: QGraphicsWidget_adjustSize>(&mut self, value: T) -> i32 {
    value.adjustSize(self);
    return 1;
  }
}

pub trait QGraphicsWidget_adjustSize {
  fn adjustSize(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::adjustSize();
impl<'a> /*trait*/ QGraphicsWidget_adjustSize for () {
  fn adjustSize(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget10adjustSizeEv()};
    unsafe {_ZN15QGraphicsWidget10adjustSizeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn paintWindowFrame<T: QGraphicsWidget_paintWindowFrame>(&mut self, value: T) -> i32 {
    value.paintWindowFrame(self);
    return 1;
  }
}

pub trait QGraphicsWidget_paintWindowFrame {
  fn paintWindowFrame(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::paintWindowFrame(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsWidget_paintWindowFrame for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paintWindowFrame(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget16paintWindowFrameEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsWidget16paintWindowFrameEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn palette<T: QGraphicsWidget_palette>(&mut self, value: T) -> i32 {
    value.palette(self);
    return 1;
  }
}

pub trait QGraphicsWidget_palette {
  fn palette(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: QPalette QGraphicsWidget::palette();
impl<'a> /*trait*/ QGraphicsWidget_palette for () {
  fn palette(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget7paletteEv()};
    unsafe {_ZNK15QGraphicsWidget7paletteEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn unsetWindowFrameMargins<T: QGraphicsWidget_unsetWindowFrameMargins>(&mut self, value: T) -> i32 {
    value.unsetWindowFrameMargins(self);
    return 1;
  }
}

pub trait QGraphicsWidget_unsetWindowFrameMargins {
  fn unsetWindowFrameMargins(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::unsetWindowFrameMargins();
impl<'a> /*trait*/ QGraphicsWidget_unsetWindowFrameMargins for () {
  fn unsetWindowFrameMargins(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget23unsetWindowFrameMarginsEv()};
    unsafe {_ZN15QGraphicsWidget23unsetWindowFrameMarginsEv()};
    return 1;
  }
}

// proto: void QGraphicsWidget::resize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsWidget_resize for (&'a  QSizeF) {
  fn resize(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget6resizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGraphicsWidget6resizeERK6QSizeF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setPalette<T: QGraphicsWidget_setPalette>(&mut self, value: T) -> i32 {
    value.setPalette(self);
    return 1;
  }
}

pub trait QGraphicsWidget_setPalette {
  fn setPalette(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::setPalette(const QPalette & palette);
impl<'a> /*trait*/ QGraphicsWidget_setPalette for (&'a  QPalette) {
  fn setPalette(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGraphicsWidget10setPaletteERK8QPalette(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn autoFillBackground<T: QGraphicsWidget_autoFillBackground>(&mut self, value: T) -> i32 {
    value.autoFillBackground(self);
    return 1;
  }
}

pub trait QGraphicsWidget_autoFillBackground {
  fn autoFillBackground(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: bool QGraphicsWidget::autoFillBackground();
impl<'a> /*trait*/ QGraphicsWidget_autoFillBackground for () {
  fn autoFillBackground(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget18autoFillBackgroundEv()};
    unsafe {_ZNK15QGraphicsWidget18autoFillBackgroundEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn style<T: QGraphicsWidget_style>(&mut self, value: T) -> i32 {
    value.style(self);
    return 1;
  }
}

pub trait QGraphicsWidget_style {
  fn style(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: QStyle * QGraphicsWidget::style();
impl<'a> /*trait*/ QGraphicsWidget_style for () {
  fn style(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget5styleEv()};
    unsafe {_ZNK15QGraphicsWidget5styleEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn shape<T: QGraphicsWidget_shape>(&mut self, value: T) -> i32 {
    value.shape(self);
    return 1;
  }
}

pub trait QGraphicsWidget_shape {
  fn shape(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: QPainterPath QGraphicsWidget::shape();
impl<'a> /*trait*/ QGraphicsWidget_shape for () {
  fn shape(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget5shapeEv()};
    unsafe {_ZNK15QGraphicsWidget5shapeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setShortcutEnabled<T: QGraphicsWidget_setShortcutEnabled>(&mut self, value: T) -> i32 {
    value.setShortcutEnabled(self);
    return 1;
  }
}

pub trait QGraphicsWidget_setShortcutEnabled {
  fn setShortcutEnabled(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::setShortcutEnabled(int id, bool enabled);
impl<'a> /*trait*/ QGraphicsWidget_setShortcutEnabled for (i32, i8) {
  fn setShortcutEnabled(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget18setShortcutEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN15QGraphicsWidget18setShortcutEnabledEib(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn removeAction<T: QGraphicsWidget_removeAction>(&mut self, value: T) -> i32 {
    value.removeAction(self);
    return 1;
  }
}

pub trait QGraphicsWidget_removeAction {
  fn removeAction(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::removeAction(QAction * action);
impl<'a> /*trait*/ QGraphicsWidget_removeAction for (&'a mut QAction) {
  fn removeAction(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget12removeActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsWidget12removeActionEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn insertAction<T: QGraphicsWidget_insertAction>(&mut self, value: T) -> i32 {
    value.insertAction(self);
    return 1;
  }
}

pub trait QGraphicsWidget_insertAction {
  fn insertAction(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::insertAction(QAction * before, QAction * action);
impl<'a> /*trait*/ QGraphicsWidget_insertAction for (&'a mut QAction, &'a mut QAction) {
  fn insertAction(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget12insertActionEP7QActionS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsWidget12insertActionEP7QActionS1_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn close<T: QGraphicsWidget_close>(&mut self, value: T) -> i32 {
    value.close(self);
    return 1;
  }
}

pub trait QGraphicsWidget_close {
  fn close(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: bool QGraphicsWidget::close();
impl<'a> /*trait*/ QGraphicsWidget_close for () {
  fn close(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget5closeEv()};
    unsafe {_ZN15QGraphicsWidget5closeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn metaObject<T: QGraphicsWidget_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGraphicsWidget_metaObject {
  fn metaObject(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: const QMetaObject * QGraphicsWidget::metaObject();
impl<'a> /*trait*/ QGraphicsWidget_metaObject for () {
  fn metaObject(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget10metaObjectEv()};
    unsafe {_ZNK15QGraphicsWidget10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn boundingRect<T: QGraphicsWidget_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QGraphicsWidget_boundingRect {
  fn boundingRect(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: QRectF QGraphicsWidget::boundingRect();
impl<'a> /*trait*/ QGraphicsWidget_boundingRect for () {
  fn boundingRect(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget12boundingRectEv()};
    unsafe {_ZNK15QGraphicsWidget12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setContentsMargins<T: QGraphicsWidget_setContentsMargins>(&mut self, value: T) -> i32 {
    value.setContentsMargins(self);
    return 1;
  }
}

pub trait QGraphicsWidget_setContentsMargins {
  fn setContentsMargins(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
impl<'a> /*trait*/ QGraphicsWidget_setContentsMargins for (f64, f64, f64, f64) {
  fn setContentsMargins(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget18setContentsMarginsEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN15QGraphicsWidget18setContentsMarginsEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setFont<T: QGraphicsWidget_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QGraphicsWidget_setFont {
  fn setFont(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::setFont(const QFont & font);
impl<'a> /*trait*/ QGraphicsWidget_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGraphicsWidget7setFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn geometryChanged<T: QGraphicsWidget_geometryChanged>(&mut self, value: T) -> i32 {
    value.geometryChanged(self);
    return 1;
  }
}

pub trait QGraphicsWidget_geometryChanged {
  fn geometryChanged(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::geometryChanged();
impl<'a> /*trait*/ QGraphicsWidget_geometryChanged for () {
  fn geometryChanged(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget15geometryChangedEv()};
    unsafe {_ZN15QGraphicsWidget15geometryChangedEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn windowTitle<T: QGraphicsWidget_windowTitle>(&mut self, value: T) -> i32 {
    value.windowTitle(self);
    return 1;
  }
}

pub trait QGraphicsWidget_windowTitle {
  fn windowTitle(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: QString QGraphicsWidget::windowTitle();
impl<'a> /*trait*/ QGraphicsWidget_windowTitle for () {
  fn windowTitle(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget11windowTitleEv()};
    unsafe {_ZNK15QGraphicsWidget11windowTitleEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn layout<T: QGraphicsWidget_layout>(&mut self, value: T) -> i32 {
    value.layout(self);
    return 1;
  }
}

pub trait QGraphicsWidget_layout {
  fn layout(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: QGraphicsLayout * QGraphicsWidget::layout();
impl<'a> /*trait*/ QGraphicsWidget_layout for () {
  fn layout(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget6layoutEv()};
    unsafe {_ZNK15QGraphicsWidget6layoutEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn FreeQGraphicsWidget<T: QGraphicsWidget_FreeQGraphicsWidget>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsWidget(self);
    return 1;
  }
}

pub trait QGraphicsWidget_FreeQGraphicsWidget {
  fn FreeQGraphicsWidget(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::FreeQGraphicsWidget();
impl<'a> /*trait*/ QGraphicsWidget_FreeQGraphicsWidget for () {
  fn FreeQGraphicsWidget(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidgetD0Ev()};
    unsafe {_ZN15QGraphicsWidgetD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn focusWidget<T: QGraphicsWidget_focusWidget>(&mut self, value: T) -> i32 {
    value.focusWidget(self);
    return 1;
  }
}

pub trait QGraphicsWidget_focusWidget {
  fn focusWidget(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: QGraphicsWidget * QGraphicsWidget::focusWidget();
impl<'a> /*trait*/ QGraphicsWidget_focusWidget for () {
  fn focusWidget(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget11focusWidgetEv()};
    unsafe {_ZNK15QGraphicsWidget11focusWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn addAction<T: QGraphicsWidget_addAction>(&mut self, value: T) -> i32 {
    value.addAction(self);
    return 1;
  }
}

pub trait QGraphicsWidget_addAction {
  fn addAction(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::addAction(QAction * action);
impl<'a> /*trait*/ QGraphicsWidget_addAction for (&'a mut QAction) {
  fn addAction(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget9addActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsWidget9addActionEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn font<T: QGraphicsWidget_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QGraphicsWidget_font {
  fn font(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: QFont QGraphicsWidget::font();
impl<'a> /*trait*/ QGraphicsWidget_font for () {
  fn font(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget4fontEv()};
    unsafe {_ZNK15QGraphicsWidget4fontEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn actions<T: QGraphicsWidget_actions>(&mut self, value: T) -> i32 {
    value.actions(self);
    return 1;
  }
}

pub trait QGraphicsWidget_actions {
  fn actions(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: QList<QAction *> QGraphicsWidget::actions();
impl<'a> /*trait*/ QGraphicsWidget_actions for () {
  fn actions(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget7actionsEv()};
    unsafe {_ZNK15QGraphicsWidget7actionsEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn layoutChanged<T: QGraphicsWidget_layoutChanged>(&mut self, value: T) -> i32 {
    value.layoutChanged(self);
    return 1;
  }
}

pub trait QGraphicsWidget_layoutChanged {
  fn layoutChanged(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::layoutChanged();
impl<'a> /*trait*/ QGraphicsWidget_layoutChanged for () {
  fn layoutChanged(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget13layoutChangedEv()};
    unsafe {_ZN15QGraphicsWidget13layoutChangedEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setShortcutAutoRepeat<T: QGraphicsWidget_setShortcutAutoRepeat>(&mut self, value: T) -> i32 {
    value.setShortcutAutoRepeat(self);
    return 1;
  }
}

pub trait QGraphicsWidget_setShortcutAutoRepeat {
  fn setShortcutAutoRepeat(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::setShortcutAutoRepeat(int id, bool enabled);
impl<'a> /*trait*/ QGraphicsWidget_setShortcutAutoRepeat for (i32, i8) {
  fn setShortcutAutoRepeat(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget21setShortcutAutoRepeatEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN15QGraphicsWidget21setShortcutAutoRepeatEib(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setTabOrder<T: QGraphicsWidget_setTabOrder>(&mut self, value: T) -> i32 {
    value.setTabOrder(self);
    return 1;
  }
}

pub trait QGraphicsWidget_setTabOrder {
  fn setTabOrder(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::setTabOrder(QGraphicsWidget * first, QGraphicsWidget * second);
impl<'a> /*trait*/ QGraphicsWidget_setTabOrder for (&'a mut QGraphicsWidget, &'a mut QGraphicsWidget) {
  fn setTabOrder(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget11setTabOrderEPS_S0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsWidget11setTabOrderEPS_S0_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn getWindowFrameMargins<T: QGraphicsWidget_getWindowFrameMargins>(&mut self, value: T) -> i32 {
    value.getWindowFrameMargins(self);
    return 1;
  }
}

pub trait QGraphicsWidget_getWindowFrameMargins {
  fn getWindowFrameMargins(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::getWindowFrameMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsWidget_getWindowFrameMargins for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getWindowFrameMargins(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget21getWindowFrameMarginsEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
    unsafe {_ZNK15QGraphicsWidget21getWindowFrameMarginsEPdS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn setStyle<T: QGraphicsWidget_setStyle>(&mut self, value: T) -> i32 {
    value.setStyle(self);
    return 1;
  }
}

pub trait QGraphicsWidget_setStyle {
  fn setStyle(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::setStyle(QStyle * style);
impl<'a> /*trait*/ QGraphicsWidget_setStyle for (&'a mut QStyle) {
  fn setStyle(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget8setStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsWidget8setStyleEP6QStyle(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn getContentsMargins<T: QGraphicsWidget_getContentsMargins>(&mut self, value: T) -> i32 {
    value.getContentsMargins(self);
    return 1;
  }
}

pub trait QGraphicsWidget_getContentsMargins {
  fn getContentsMargins(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: void QGraphicsWidget::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsWidget_getContentsMargins for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getContentsMargins(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget18getContentsMarginsEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
    unsafe {_ZNK15QGraphicsWidget18getContentsMarginsEPdS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsWidget {
  pub fn isActiveWindow<T: QGraphicsWidget_isActiveWindow>(&mut self, value: T) -> i32 {
    value.isActiveWindow(self);
    return 1;
  }
}

pub trait QGraphicsWidget_isActiveWindow {
  fn isActiveWindow(self, this: &mut QGraphicsWidget) -> i32;
}

// proto: bool QGraphicsWidget::isActiveWindow();
impl<'a> /*trait*/ QGraphicsWidget_isActiveWindow for () {
  fn isActiveWindow(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget14isActiveWindowEv()};
    unsafe {_ZNK15QGraphicsWidget14isActiveWindowEv()};
    return 1;
  }
}

// proto: void QGraphicsWidget::setGeometry(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsWidget_setGeometry for (f64, f64, f64, f64) {
  fn setGeometry(self, this: &mut QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget11setGeometryEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN15QGraphicsWidget11setGeometryEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

