// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbrush::QBrush;
use super::qrectf::QRectF;
use super::qgraphicsitem::QGraphicsItem;
use super::qevent::QEvent;
use super::qpolygonf::QPolygonF;
use super::qpen::QPen;
use super::qlinef::QLineF;
use super::qobject::QObject;
use super::qstring::QString;
use super::qfont::QFont;
use super::qtransform::QTransform;
use super::qpixmap::QPixmap;
use super::qstyle::QStyle;
use super::qpalette::QPalette;
use super::qpainterpath::QPainterPath;
use super::qgraphicswidget::QGraphicsWidget;
use super::qpointf::QPointF;
use super::qgraphicsitemgroup::QGraphicsItemGroup;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsScene::setForegroundBrush(const QBrush & brush);
  fn _ZN14QGraphicsScene18setForegroundBrushERK6QBrush(arg0: *const c_void) -> i32;
  // proto: void QGraphicsScene::setSceneRect(const QRectF & rect);
  fn _ZN14QGraphicsScene12setSceneRectERK6QRectF(arg0: *const c_void) -> i32;
  // proto: bool QGraphicsScene::isActive();
  fn _ZNK14QGraphicsScene8isActiveEv() -> i32;
  // proto: bool QGraphicsScene::hasFocus();
  fn _ZNK14QGraphicsScene8hasFocusEv() -> i32;
  // proto: QRectF QGraphicsScene::itemsBoundingRect();
  fn _ZNK14QGraphicsScene17itemsBoundingRectEv() -> i32;
  // proto: bool QGraphicsScene::sendEvent(QGraphicsItem * item, QEvent * event);
  fn _ZN14QGraphicsScene9sendEventEP13QGraphicsItemP6QEvent(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: double QGraphicsScene::minimumRenderSize();
  fn _ZNK14QGraphicsScene17minimumRenderSizeEv() -> i32;
  // proto: QPainterPath QGraphicsScene::selectionArea();
  fn _ZNK14QGraphicsScene13selectionAreaEv() -> i32;
  // proto: void QGraphicsScene::update(const QRectF & rect);
  fn _ZN14QGraphicsScene6updateERK6QRectF(arg0: *const c_void) -> i32;
  // proto: QGraphicsPolygonItem * QGraphicsScene::addPolygon(const QPolygonF & polygon, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene10addPolygonERK9QPolygonFRK4QPenRK6QBrush(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: QGraphicsLineItem * QGraphicsScene::addLine(const QLineF & line, const QPen & pen);
  fn _ZN14QGraphicsScene7addLineERK6QLineFRK4QPen(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QPalette QGraphicsScene::palette();
  fn _ZNK14QGraphicsScene7paletteEv() -> i32;
  // proto: bool QGraphicsScene::isSortCacheEnabled();
  fn _ZNK14QGraphicsScene18isSortCacheEnabledEv() -> i32;
  // proto: void QGraphicsScene::NewQGraphicsScene(const QRectF & sceneRect, QObject * parent);
  fn _ZN14QGraphicsSceneC1ERK6QRectFP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QGraphicsScene::NewQGraphicsScene(QObject * parent);
  fn _ZN14QGraphicsSceneC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QGraphicsScene::clearFocus();
  fn _ZN14QGraphicsScene10clearFocusEv() -> i32;
  // proto: const QMetaObject * QGraphicsScene::metaObject();
  fn _ZNK14QGraphicsScene10metaObjectEv() -> i32;
  // proto: QGraphicsSimpleTextItem * QGraphicsScene::addSimpleText(const QString & text, const QFont & font);
  fn _ZN14QGraphicsScene13addSimpleTextERK7QStringRK5QFont(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QGraphicsLineItem * QGraphicsScene::addLine(qreal x1, qreal y1, qreal x2, qreal y2, const QPen & pen);
  fn _ZN14QGraphicsScene7addLineEddddRK4QPen(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *const c_void) -> i32;
  // proto: void QGraphicsScene::setBspTreeDepth(int depth);
  fn _ZN14QGraphicsScene15setBspTreeDepthEi(arg0: c_int) -> i32;
  // proto: QRectF QGraphicsScene::sceneRect();
  fn _ZNK14QGraphicsScene9sceneRectEv() -> i32;
  // proto: QGraphicsWidget * QGraphicsScene::activeWindow();
  fn _ZNK14QGraphicsScene12activeWindowEv() -> i32;
  // proto: QBrush QGraphicsScene::backgroundBrush();
  fn _ZNK14QGraphicsScene15backgroundBrushEv() -> i32;
  // proto: QGraphicsItem * QGraphicsScene::itemAt(qreal x, qreal y, const QTransform & deviceTransform);
  fn _ZNK14QGraphicsScene6itemAtEddRK10QTransform(arg0: c_double, arg1: c_double, arg2: *const c_void) -> i32;
  // proto: void QGraphicsScene::advance();
  fn _ZN14QGraphicsScene7advanceEv() -> i32;
  // proto: void QGraphicsScene::setStickyFocus(bool enabled);
  fn _ZN14QGraphicsScene14setStickyFocusEb(arg0: int8_t) -> i32;
  // proto: QList<QGraphicsItem *> QGraphicsScene::selectedItems();
  fn _ZNK14QGraphicsScene13selectedItemsEv() -> i32;
  // proto: void QGraphicsScene::clear();
  fn _ZN14QGraphicsScene5clearEv() -> i32;
  // proto: void QGraphicsScene::setActivePanel(QGraphicsItem * item);
  fn _ZN14QGraphicsScene14setActivePanelEP13QGraphicsItem(arg0: *mut c_void) -> i32;
  // proto: QGraphicsPixmapItem * QGraphicsScene::addPixmap(const QPixmap & pixmap);
  fn _ZN14QGraphicsScene9addPixmapERK7QPixmap(arg0: *const c_void) -> i32;
  // proto: QBrush QGraphicsScene::foregroundBrush();
  fn _ZNK14QGraphicsScene15foregroundBrushEv() -> i32;
  // proto: void QGraphicsScene::selectionChanged();
  fn _ZN14QGraphicsScene16selectionChangedEv() -> i32;
  // proto: QList<QGraphicsView *> QGraphicsScene::views();
  fn _ZNK14QGraphicsScene5viewsEv() -> i32;
  // proto: void QGraphicsScene::FreeQGraphicsScene();
  fn _ZN14QGraphicsSceneD0Ev() -> i32;
  // proto: QGraphicsRectItem * QGraphicsScene::addRect(qreal x, qreal y, qreal w, qreal h, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene7addRectEddddRK4QPenRK6QBrush(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *const c_void, arg5: *const c_void) -> i32;
  // proto: int QGraphicsScene::bspTreeDepth();
  fn _ZNK14QGraphicsScene12bspTreeDepthEv() -> i32;
  // proto: void QGraphicsScene::setSceneRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN14QGraphicsScene12setSceneRectEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsScene::setStyle(QStyle * style);
  fn _ZN14QGraphicsScene8setStyleEP6QStyle(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsScene::setPalette(const QPalette & palette);
  fn _ZN14QGraphicsScene10setPaletteERK8QPalette(arg0: *const c_void) -> i32;
  // proto: void QGraphicsScene::setMinimumRenderSize(qreal minSize);
  fn _ZN14QGraphicsScene20setMinimumRenderSizeEd(arg0: c_double) -> i32;
  // proto: void QGraphicsScene::NewQGraphicsScene(qreal x, qreal y, qreal width, qreal height, QObject * parent);
  fn _ZN14QGraphicsSceneC1EddddP7QObject(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void) -> i32;
  // proto: QGraphicsItem * QGraphicsScene::mouseGrabberItem();
  fn _ZNK14QGraphicsScene16mouseGrabberItemEv() -> i32;
  // proto: QGraphicsRectItem * QGraphicsScene::addRect(const QRectF & rect, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene7addRectERK6QRectFRK4QPenRK6QBrush(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: QGraphicsEllipseItem * QGraphicsScene::addEllipse(const QRectF & rect, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene10addEllipseERK6QRectFRK4QPenRK6QBrush(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: double QGraphicsScene::height();
  fn _ZNK14QGraphicsScene6heightEv() -> i32;
  // proto: void QGraphicsScene::setSelectionArea(const QPainterPath & path, const QTransform & deviceTransform);
  fn _ZN14QGraphicsScene16setSelectionAreaERK12QPainterPathRK10QTransform(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QFont QGraphicsScene::font();
  fn _ZNK14QGraphicsScene4fontEv() -> i32;
  // proto: void QGraphicsScene::clearSelection();
  fn _ZN14QGraphicsScene14clearSelectionEv() -> i32;
  // proto: void QGraphicsScene::NewQGraphicsScene(const QGraphicsScene & );
  fn _ZN14QGraphicsSceneC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QGraphicsScene::removeItem(QGraphicsItem * item);
  fn _ZN14QGraphicsScene10removeItemEP13QGraphicsItem(arg0: *mut c_void) -> i32;
  // proto: QGraphicsEllipseItem * QGraphicsScene::addEllipse(qreal x, qreal y, qreal w, qreal h, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene10addEllipseEddddRK4QPenRK6QBrush(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *const c_void, arg5: *const c_void) -> i32;
  // proto: void QGraphicsScene::setActiveWindow(QGraphicsWidget * widget);
  fn _ZN14QGraphicsScene15setActiveWindowEP15QGraphicsWidget(arg0: *mut c_void) -> i32;
  // proto: QGraphicsItem * QGraphicsScene::focusItem();
  fn _ZNK14QGraphicsScene9focusItemEv() -> i32;
  // proto: QGraphicsTextItem * QGraphicsScene::addText(const QString & text, const QFont & font);
  fn _ZN14QGraphicsScene7addTextERK7QStringRK5QFont(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QGraphicsScene::setSortCacheEnabled(bool enabled);
  fn _ZN14QGraphicsScene19setSortCacheEnabledEb(arg0: int8_t) -> i32;
  // proto: QGraphicsItem * QGraphicsScene::itemAt(const QPointF & pos, const QTransform & deviceTransform);
  fn _ZNK14QGraphicsScene6itemAtERK7QPointFRK10QTransform(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QGraphicsScene::destroyItemGroup(QGraphicsItemGroup * group);
  fn _ZN14QGraphicsScene16destroyItemGroupEP18QGraphicsItemGroup(arg0: *mut c_void) -> i32;
  // proto: double QGraphicsScene::width();
  fn _ZNK14QGraphicsScene5widthEv() -> i32;
  // proto: void QGraphicsScene::update(qreal x, qreal y, qreal w, qreal h);
  fn _ZN14QGraphicsScene6updateEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsScene::addItem(QGraphicsItem * item);
  fn _ZN14QGraphicsScene7addItemEP13QGraphicsItem(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsScene::setBackgroundBrush(const QBrush & brush);
  fn _ZN14QGraphicsScene18setBackgroundBrushERK6QBrush(arg0: *const c_void) -> i32;
  // proto: QGraphicsItem * QGraphicsScene::activePanel();
  fn _ZNK14QGraphicsScene11activePanelEv() -> i32;
  // proto: void QGraphicsScene::sceneRectChanged(const QRectF & rect);
  fn _ZN14QGraphicsScene16sceneRectChangedERK6QRectF(arg0: *const c_void) -> i32;
  // proto: QStyle * QGraphicsScene::style();
  fn _ZNK14QGraphicsScene5styleEv() -> i32;
  // proto: void QGraphicsScene::setFont(const QFont & font);
  fn _ZN14QGraphicsScene7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: QGraphicsPathItem * QGraphicsScene::addPath(const QPainterPath & path, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene7addPathERK12QPainterPathRK4QPenRK6QBrush(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: bool QGraphicsScene::stickyFocus();
  fn _ZNK14QGraphicsScene11stickyFocusEv() -> i32;
}

// body block begin
// class sizeof(QGraphicsScene)=1
pub struct QGraphicsScene {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsScene {
  pub fn setForegroundBrush<T: QGraphicsScene_setForegroundBrush>(&mut self, value: T) -> i32 {
    value.setForegroundBrush(self);
    return 1;
  }
}

pub trait QGraphicsScene_setForegroundBrush {
  fn setForegroundBrush(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::setForegroundBrush(const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_setForegroundBrush for (&'a  QBrush) {
  fn setForegroundBrush(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene18setForegroundBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene18setForegroundBrushERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setSceneRect<T: QGraphicsScene_setSceneRect>(&mut self, value: T) -> i32 {
    value.setSceneRect(self);
    return 1;
  }
}

pub trait QGraphicsScene_setSceneRect {
  fn setSceneRect(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::setSceneRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsScene_setSceneRect for (&'a  QRectF) {
  fn setSceneRect(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene12setSceneRectERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene12setSceneRectERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn isActive<T: QGraphicsScene_isActive>(&mut self, value: T) -> i32 {
    value.isActive(self);
    return 1;
  }
}

pub trait QGraphicsScene_isActive {
  fn isActive(self, this: &mut QGraphicsScene) -> i32;
}

// proto: bool QGraphicsScene::isActive();
impl<'a> /*trait*/ QGraphicsScene_isActive for () {
  fn isActive(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene8isActiveEv()};
    unsafe {_ZNK14QGraphicsScene8isActiveEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn hasFocus<T: QGraphicsScene_hasFocus>(&mut self, value: T) -> i32 {
    value.hasFocus(self);
    return 1;
  }
}

pub trait QGraphicsScene_hasFocus {
  fn hasFocus(self, this: &mut QGraphicsScene) -> i32;
}

// proto: bool QGraphicsScene::hasFocus();
impl<'a> /*trait*/ QGraphicsScene_hasFocus for () {
  fn hasFocus(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene8hasFocusEv()};
    unsafe {_ZNK14QGraphicsScene8hasFocusEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn itemsBoundingRect<T: QGraphicsScene_itemsBoundingRect>(&mut self, value: T) -> i32 {
    value.itemsBoundingRect(self);
    return 1;
  }
}

pub trait QGraphicsScene_itemsBoundingRect {
  fn itemsBoundingRect(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QRectF QGraphicsScene::itemsBoundingRect();
impl<'a> /*trait*/ QGraphicsScene_itemsBoundingRect for () {
  fn itemsBoundingRect(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene17itemsBoundingRectEv()};
    unsafe {_ZNK14QGraphicsScene17itemsBoundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn sendEvent<T: QGraphicsScene_sendEvent>(&mut self, value: T) -> i32 {
    value.sendEvent(self);
    return 1;
  }
}

pub trait QGraphicsScene_sendEvent {
  fn sendEvent(self, this: &mut QGraphicsScene) -> i32;
}

// proto: bool QGraphicsScene::sendEvent(QGraphicsItem * item, QEvent * event);
impl<'a> /*trait*/ QGraphicsScene_sendEvent for (&'a mut QGraphicsItem, &'a mut QEvent) {
  fn sendEvent(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene9sendEventEP13QGraphicsItemP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN14QGraphicsScene9sendEventEP13QGraphicsItemP6QEvent(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn minimumRenderSize<T: QGraphicsScene_minimumRenderSize>(&mut self, value: T) -> i32 {
    value.minimumRenderSize(self);
    return 1;
  }
}

pub trait QGraphicsScene_minimumRenderSize {
  fn minimumRenderSize(self, this: &mut QGraphicsScene) -> i32;
}

// proto: double QGraphicsScene::minimumRenderSize();
impl<'a> /*trait*/ QGraphicsScene_minimumRenderSize for () {
  fn minimumRenderSize(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene17minimumRenderSizeEv()};
    unsafe {_ZNK14QGraphicsScene17minimumRenderSizeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn selectionArea<T: QGraphicsScene_selectionArea>(&mut self, value: T) -> i32 {
    value.selectionArea(self);
    return 1;
  }
}

pub trait QGraphicsScene_selectionArea {
  fn selectionArea(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QPainterPath QGraphicsScene::selectionArea();
impl<'a> /*trait*/ QGraphicsScene_selectionArea for () {
  fn selectionArea(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene13selectionAreaEv()};
    unsafe {_ZNK14QGraphicsScene13selectionAreaEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn update<T: QGraphicsScene_update>(&mut self, value: T) -> i32 {
    value.update(self);
    return 1;
  }
}

pub trait QGraphicsScene_update {
  fn update(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::update(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsScene_update for (&'a  QRectF) {
  fn update(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene6updateERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene6updateERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addPolygon<T: QGraphicsScene_addPolygon>(&mut self, value: T) -> i32 {
    value.addPolygon(self);
    return 1;
  }
}

pub trait QGraphicsScene_addPolygon {
  fn addPolygon(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QGraphicsPolygonItem * QGraphicsScene::addPolygon(const QPolygonF & polygon, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addPolygon for (&'a  QPolygonF, &'a  QPen, &'a  QBrush) {
  fn addPolygon(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10addPolygonERK9QPolygonFRK4QPenRK6QBrush()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene10addPolygonERK9QPolygonFRK4QPenRK6QBrush(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addLine<T: QGraphicsScene_addLine>(&mut self, value: T) -> i32 {
    value.addLine(self);
    return 1;
  }
}

pub trait QGraphicsScene_addLine {
  fn addLine(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QGraphicsLineItem * QGraphicsScene::addLine(const QLineF & line, const QPen & pen);
impl<'a> /*trait*/ QGraphicsScene_addLine for (&'a  QLineF, &'a  QPen) {
  fn addLine(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addLineERK6QLineFRK4QPen()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene7addLineERK6QLineFRK4QPen(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn palette<T: QGraphicsScene_palette>(&mut self, value: T) -> i32 {
    value.palette(self);
    return 1;
  }
}

pub trait QGraphicsScene_palette {
  fn palette(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QPalette QGraphicsScene::palette();
impl<'a> /*trait*/ QGraphicsScene_palette for () {
  fn palette(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene7paletteEv()};
    unsafe {_ZNK14QGraphicsScene7paletteEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn isSortCacheEnabled<T: QGraphicsScene_isSortCacheEnabled>(&mut self, value: T) -> i32 {
    value.isSortCacheEnabled(self);
    return 1;
  }
}

pub trait QGraphicsScene_isSortCacheEnabled {
  fn isSortCacheEnabled(self, this: &mut QGraphicsScene) -> i32;
}

// proto: bool QGraphicsScene::isSortCacheEnabled();
impl<'a> /*trait*/ QGraphicsScene_isSortCacheEnabled for () {
  fn isSortCacheEnabled(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene18isSortCacheEnabledEv()};
    unsafe {_ZNK14QGraphicsScene18isSortCacheEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn NewQGraphicsScene<T: QGraphicsScene_NewQGraphicsScene>(value: T) -> QGraphicsScene {
    let rsthis = value.NewQGraphicsScene();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsScene_NewQGraphicsScene {
  fn NewQGraphicsScene(self) -> QGraphicsScene;
}

// proto: void QGraphicsScene::NewQGraphicsScene(const QRectF & sceneRect, QObject * parent);
impl<'a> /*trait*/ QGraphicsScene_NewQGraphicsScene for (&'a  QRectF, &'a mut QObject) {
  fn NewQGraphicsScene(self) -> QGraphicsScene {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsSceneC1ERK6QRectFP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN14QGraphicsSceneC1ERK6QRectFP7QObject(qthis, arg0, arg1)};
    let rsthis = QGraphicsScene{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QGraphicsScene::NewQGraphicsScene(QObject * parent);
impl<'a> /*trait*/ QGraphicsScene_NewQGraphicsScene for (&'a mut QObject) {
  fn NewQGraphicsScene(self) -> QGraphicsScene {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsSceneC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QGraphicsSceneC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsScene{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn clearFocus<T: QGraphicsScene_clearFocus>(&mut self, value: T) -> i32 {
    value.clearFocus(self);
    return 1;
  }
}

pub trait QGraphicsScene_clearFocus {
  fn clearFocus(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::clearFocus();
impl<'a> /*trait*/ QGraphicsScene_clearFocus for () {
  fn clearFocus(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10clearFocusEv()};
    unsafe {_ZN14QGraphicsScene10clearFocusEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn metaObject<T: QGraphicsScene_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGraphicsScene_metaObject {
  fn metaObject(self, this: &mut QGraphicsScene) -> i32;
}

// proto: const QMetaObject * QGraphicsScene::metaObject();
impl<'a> /*trait*/ QGraphicsScene_metaObject for () {
  fn metaObject(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene10metaObjectEv()};
    unsafe {_ZNK14QGraphicsScene10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addSimpleText<T: QGraphicsScene_addSimpleText>(&mut self, value: T) -> i32 {
    value.addSimpleText(self);
    return 1;
  }
}

pub trait QGraphicsScene_addSimpleText {
  fn addSimpleText(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QGraphicsSimpleTextItem * QGraphicsScene::addSimpleText(const QString & text, const QFont & font);
impl<'a> /*trait*/ QGraphicsScene_addSimpleText for (&'a  QString, &'a  QFont) {
  fn addSimpleText(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene13addSimpleTextERK7QStringRK5QFont()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene13addSimpleTextERK7QStringRK5QFont(arg0, arg1)};
    return 1;
  }
}

// proto: QGraphicsLineItem * QGraphicsScene::addLine(qreal x1, qreal y1, qreal x2, qreal y2, const QPen & pen);
impl<'a> /*trait*/ QGraphicsScene_addLine for (f64, f64, f64, f64, &'a  QPen) {
  fn addLine(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addLineEddddRK4QPen()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene7addLineEddddRK4QPen(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setBspTreeDepth<T: QGraphicsScene_setBspTreeDepth>(&mut self, value: T) -> i32 {
    value.setBspTreeDepth(self);
    return 1;
  }
}

pub trait QGraphicsScene_setBspTreeDepth {
  fn setBspTreeDepth(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::setBspTreeDepth(int depth);
impl<'a> /*trait*/ QGraphicsScene_setBspTreeDepth for (i32) {
  fn setBspTreeDepth(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene15setBspTreeDepthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QGraphicsScene15setBspTreeDepthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn sceneRect<T: QGraphicsScene_sceneRect>(&mut self, value: T) -> i32 {
    value.sceneRect(self);
    return 1;
  }
}

pub trait QGraphicsScene_sceneRect {
  fn sceneRect(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QRectF QGraphicsScene::sceneRect();
impl<'a> /*trait*/ QGraphicsScene_sceneRect for () {
  fn sceneRect(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene9sceneRectEv()};
    unsafe {_ZNK14QGraphicsScene9sceneRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn activeWindow<T: QGraphicsScene_activeWindow>(&mut self, value: T) -> i32 {
    value.activeWindow(self);
    return 1;
  }
}

pub trait QGraphicsScene_activeWindow {
  fn activeWindow(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QGraphicsWidget * QGraphicsScene::activeWindow();
impl<'a> /*trait*/ QGraphicsScene_activeWindow for () {
  fn activeWindow(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene12activeWindowEv()};
    unsafe {_ZNK14QGraphicsScene12activeWindowEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn backgroundBrush<T: QGraphicsScene_backgroundBrush>(&mut self, value: T) -> i32 {
    value.backgroundBrush(self);
    return 1;
  }
}

pub trait QGraphicsScene_backgroundBrush {
  fn backgroundBrush(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QBrush QGraphicsScene::backgroundBrush();
impl<'a> /*trait*/ QGraphicsScene_backgroundBrush for () {
  fn backgroundBrush(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene15backgroundBrushEv()};
    unsafe {_ZNK14QGraphicsScene15backgroundBrushEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn itemAt<T: QGraphicsScene_itemAt>(&mut self, value: T) -> i32 {
    value.itemAt(self);
    return 1;
  }
}

pub trait QGraphicsScene_itemAt {
  fn itemAt(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QGraphicsItem * QGraphicsScene::itemAt(qreal x, qreal y, const QTransform & deviceTransform);
impl<'a> /*trait*/ QGraphicsScene_itemAt for (f64, f64, &'a  QTransform) {
  fn itemAt(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene6itemAtEddRK10QTransform()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZNK14QGraphicsScene6itemAtEddRK10QTransform(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn advance<T: QGraphicsScene_advance>(&mut self, value: T) -> i32 {
    value.advance(self);
    return 1;
  }
}

pub trait QGraphicsScene_advance {
  fn advance(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::advance();
impl<'a> /*trait*/ QGraphicsScene_advance for () {
  fn advance(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7advanceEv()};
    unsafe {_ZN14QGraphicsScene7advanceEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setStickyFocus<T: QGraphicsScene_setStickyFocus>(&mut self, value: T) -> i32 {
    value.setStickyFocus(self);
    return 1;
  }
}

pub trait QGraphicsScene_setStickyFocus {
  fn setStickyFocus(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::setStickyFocus(bool enabled);
impl<'a> /*trait*/ QGraphicsScene_setStickyFocus for (i8) {
  fn setStickyFocus(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene14setStickyFocusEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QGraphicsScene14setStickyFocusEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn selectedItems<T: QGraphicsScene_selectedItems>(&mut self, value: T) -> i32 {
    value.selectedItems(self);
    return 1;
  }
}

pub trait QGraphicsScene_selectedItems {
  fn selectedItems(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QList<QGraphicsItem *> QGraphicsScene::selectedItems();
impl<'a> /*trait*/ QGraphicsScene_selectedItems for () {
  fn selectedItems(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene13selectedItemsEv()};
    unsafe {_ZNK14QGraphicsScene13selectedItemsEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn clear<T: QGraphicsScene_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QGraphicsScene_clear {
  fn clear(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::clear();
impl<'a> /*trait*/ QGraphicsScene_clear for () {
  fn clear(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene5clearEv()};
    unsafe {_ZN14QGraphicsScene5clearEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setActivePanel<T: QGraphicsScene_setActivePanel>(&mut self, value: T) -> i32 {
    value.setActivePanel(self);
    return 1;
  }
}

pub trait QGraphicsScene_setActivePanel {
  fn setActivePanel(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::setActivePanel(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsScene_setActivePanel for (&'a mut QGraphicsItem) {
  fn setActivePanel(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene14setActivePanelEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QGraphicsScene14setActivePanelEP13QGraphicsItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addPixmap<T: QGraphicsScene_addPixmap>(&mut self, value: T) -> i32 {
    value.addPixmap(self);
    return 1;
  }
}

pub trait QGraphicsScene_addPixmap {
  fn addPixmap(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QGraphicsPixmapItem * QGraphicsScene::addPixmap(const QPixmap & pixmap);
impl<'a> /*trait*/ QGraphicsScene_addPixmap for (&'a  QPixmap) {
  fn addPixmap(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene9addPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene9addPixmapERK7QPixmap(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn foregroundBrush<T: QGraphicsScene_foregroundBrush>(&mut self, value: T) -> i32 {
    value.foregroundBrush(self);
    return 1;
  }
}

pub trait QGraphicsScene_foregroundBrush {
  fn foregroundBrush(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QBrush QGraphicsScene::foregroundBrush();
impl<'a> /*trait*/ QGraphicsScene_foregroundBrush for () {
  fn foregroundBrush(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene15foregroundBrushEv()};
    unsafe {_ZNK14QGraphicsScene15foregroundBrushEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn selectionChanged<T: QGraphicsScene_selectionChanged>(&mut self, value: T) -> i32 {
    value.selectionChanged(self);
    return 1;
  }
}

pub trait QGraphicsScene_selectionChanged {
  fn selectionChanged(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::selectionChanged();
impl<'a> /*trait*/ QGraphicsScene_selectionChanged for () {
  fn selectionChanged(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene16selectionChangedEv()};
    unsafe {_ZN14QGraphicsScene16selectionChangedEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn views<T: QGraphicsScene_views>(&mut self, value: T) -> i32 {
    value.views(self);
    return 1;
  }
}

pub trait QGraphicsScene_views {
  fn views(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QList<QGraphicsView *> QGraphicsScene::views();
impl<'a> /*trait*/ QGraphicsScene_views for () {
  fn views(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene5viewsEv()};
    unsafe {_ZNK14QGraphicsScene5viewsEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn FreeQGraphicsScene<T: QGraphicsScene_FreeQGraphicsScene>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsScene(self);
    return 1;
  }
}

pub trait QGraphicsScene_FreeQGraphicsScene {
  fn FreeQGraphicsScene(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::FreeQGraphicsScene();
impl<'a> /*trait*/ QGraphicsScene_FreeQGraphicsScene for () {
  fn FreeQGraphicsScene(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsSceneD0Ev()};
    unsafe {_ZN14QGraphicsSceneD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addRect<T: QGraphicsScene_addRect>(&mut self, value: T) -> i32 {
    value.addRect(self);
    return 1;
  }
}

pub trait QGraphicsScene_addRect {
  fn addRect(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QGraphicsRectItem * QGraphicsScene::addRect(qreal x, qreal y, qreal w, qreal h, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addRect for (f64, f64, f64, f64, &'a  QPen, &'a  QBrush) {
  fn addRect(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addRectEddddRK4QPenRK6QBrush()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *const c_void;
    let arg5 = self.5.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene7addRectEddddRK4QPenRK6QBrush(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn bspTreeDepth<T: QGraphicsScene_bspTreeDepth>(&mut self, value: T) -> i32 {
    value.bspTreeDepth(self);
    return 1;
  }
}

pub trait QGraphicsScene_bspTreeDepth {
  fn bspTreeDepth(self, this: &mut QGraphicsScene) -> i32;
}

// proto: int QGraphicsScene::bspTreeDepth();
impl<'a> /*trait*/ QGraphicsScene_bspTreeDepth for () {
  fn bspTreeDepth(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene12bspTreeDepthEv()};
    unsafe {_ZNK14QGraphicsScene12bspTreeDepthEv()};
    return 1;
  }
}

// proto: void QGraphicsScene::setSceneRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsScene_setSceneRect for (f64, f64, f64, f64) {
  fn setSceneRect(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene12setSceneRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN14QGraphicsScene12setSceneRectEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setStyle<T: QGraphicsScene_setStyle>(&mut self, value: T) -> i32 {
    value.setStyle(self);
    return 1;
  }
}

pub trait QGraphicsScene_setStyle {
  fn setStyle(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::setStyle(QStyle * style);
impl<'a> /*trait*/ QGraphicsScene_setStyle for (&'a mut QStyle) {
  fn setStyle(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene8setStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QGraphicsScene8setStyleEP6QStyle(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setPalette<T: QGraphicsScene_setPalette>(&mut self, value: T) -> i32 {
    value.setPalette(self);
    return 1;
  }
}

pub trait QGraphicsScene_setPalette {
  fn setPalette(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::setPalette(const QPalette & palette);
impl<'a> /*trait*/ QGraphicsScene_setPalette for (&'a  QPalette) {
  fn setPalette(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene10setPaletteERK8QPalette(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setMinimumRenderSize<T: QGraphicsScene_setMinimumRenderSize>(&mut self, value: T) -> i32 {
    value.setMinimumRenderSize(self);
    return 1;
  }
}

pub trait QGraphicsScene_setMinimumRenderSize {
  fn setMinimumRenderSize(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::setMinimumRenderSize(qreal minSize);
impl<'a> /*trait*/ QGraphicsScene_setMinimumRenderSize for (f64) {
  fn setMinimumRenderSize(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene20setMinimumRenderSizeEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN14QGraphicsScene20setMinimumRenderSizeEd(arg0)};
    return 1;
  }
}

// proto: void QGraphicsScene::NewQGraphicsScene(qreal x, qreal y, qreal width, qreal height, QObject * parent);
impl<'a> /*trait*/ QGraphicsScene_NewQGraphicsScene for (f64, f64, f64, f64, &'a mut QObject) {
  fn NewQGraphicsScene(self) -> QGraphicsScene {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsSceneC1EddddP7QObject()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
    unsafe {_ZN14QGraphicsSceneC1EddddP7QObject(qthis, arg0, arg1, arg2, arg3, arg4)};
    let rsthis = QGraphicsScene{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn mouseGrabberItem<T: QGraphicsScene_mouseGrabberItem>(&mut self, value: T) -> i32 {
    value.mouseGrabberItem(self);
    return 1;
  }
}

pub trait QGraphicsScene_mouseGrabberItem {
  fn mouseGrabberItem(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QGraphicsItem * QGraphicsScene::mouseGrabberItem();
impl<'a> /*trait*/ QGraphicsScene_mouseGrabberItem for () {
  fn mouseGrabberItem(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene16mouseGrabberItemEv()};
    unsafe {_ZNK14QGraphicsScene16mouseGrabberItemEv()};
    return 1;
  }
}

// proto: QGraphicsRectItem * QGraphicsScene::addRect(const QRectF & rect, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addRect for (&'a  QRectF, &'a  QPen, &'a  QBrush) {
  fn addRect(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addRectERK6QRectFRK4QPenRK6QBrush()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene7addRectERK6QRectFRK4QPenRK6QBrush(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addEllipse<T: QGraphicsScene_addEllipse>(&mut self, value: T) -> i32 {
    value.addEllipse(self);
    return 1;
  }
}

pub trait QGraphicsScene_addEllipse {
  fn addEllipse(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QGraphicsEllipseItem * QGraphicsScene::addEllipse(const QRectF & rect, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addEllipse for (&'a  QRectF, &'a  QPen, &'a  QBrush) {
  fn addEllipse(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10addEllipseERK6QRectFRK4QPenRK6QBrush()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene10addEllipseERK6QRectFRK4QPenRK6QBrush(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn height<T: QGraphicsScene_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QGraphicsScene_height {
  fn height(self, this: &mut QGraphicsScene) -> i32;
}

// proto: double QGraphicsScene::height();
impl<'a> /*trait*/ QGraphicsScene_height for () {
  fn height(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene6heightEv()};
    unsafe {_ZNK14QGraphicsScene6heightEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setSelectionArea<T: QGraphicsScene_setSelectionArea>(&mut self, value: T) -> i32 {
    value.setSelectionArea(self);
    return 1;
  }
}

pub trait QGraphicsScene_setSelectionArea {
  fn setSelectionArea(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::setSelectionArea(const QPainterPath & path, const QTransform & deviceTransform);
impl<'a> /*trait*/ QGraphicsScene_setSelectionArea for (&'a  QPainterPath, &'a  QTransform) {
  fn setSelectionArea(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene16setSelectionAreaERK12QPainterPathRK10QTransform()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene16setSelectionAreaERK12QPainterPathRK10QTransform(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn font<T: QGraphicsScene_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QGraphicsScene_font {
  fn font(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QFont QGraphicsScene::font();
impl<'a> /*trait*/ QGraphicsScene_font for () {
  fn font(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene4fontEv()};
    unsafe {_ZNK14QGraphicsScene4fontEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn clearSelection<T: QGraphicsScene_clearSelection>(&mut self, value: T) -> i32 {
    value.clearSelection(self);
    return 1;
  }
}

pub trait QGraphicsScene_clearSelection {
  fn clearSelection(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::clearSelection();
impl<'a> /*trait*/ QGraphicsScene_clearSelection for () {
  fn clearSelection(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene14clearSelectionEv()};
    unsafe {_ZN14QGraphicsScene14clearSelectionEv()};
    return 1;
  }
}

// proto: void QGraphicsScene::NewQGraphicsScene(const QGraphicsScene & );
impl<'a> /*trait*/ QGraphicsScene_NewQGraphicsScene for (&'a  QGraphicsScene) {
  fn NewQGraphicsScene(self) -> QGraphicsScene {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsSceneC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsSceneC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsScene{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn removeItem<T: QGraphicsScene_removeItem>(&mut self, value: T) -> i32 {
    value.removeItem(self);
    return 1;
  }
}

pub trait QGraphicsScene_removeItem {
  fn removeItem(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::removeItem(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsScene_removeItem for (&'a mut QGraphicsItem) {
  fn removeItem(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10removeItemEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QGraphicsScene10removeItemEP13QGraphicsItem(arg0)};
    return 1;
  }
}

// proto: QGraphicsEllipseItem * QGraphicsScene::addEllipse(qreal x, qreal y, qreal w, qreal h, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addEllipse for (f64, f64, f64, f64, &'a  QPen, &'a  QBrush) {
  fn addEllipse(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10addEllipseEddddRK4QPenRK6QBrush()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *const c_void;
    let arg5 = self.5.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene10addEllipseEddddRK4QPenRK6QBrush(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setActiveWindow<T: QGraphicsScene_setActiveWindow>(&mut self, value: T) -> i32 {
    value.setActiveWindow(self);
    return 1;
  }
}

pub trait QGraphicsScene_setActiveWindow {
  fn setActiveWindow(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::setActiveWindow(QGraphicsWidget * widget);
impl<'a> /*trait*/ QGraphicsScene_setActiveWindow for (&'a mut QGraphicsWidget) {
  fn setActiveWindow(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene15setActiveWindowEP15QGraphicsWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QGraphicsScene15setActiveWindowEP15QGraphicsWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn focusItem<T: QGraphicsScene_focusItem>(&mut self, value: T) -> i32 {
    value.focusItem(self);
    return 1;
  }
}

pub trait QGraphicsScene_focusItem {
  fn focusItem(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QGraphicsItem * QGraphicsScene::focusItem();
impl<'a> /*trait*/ QGraphicsScene_focusItem for () {
  fn focusItem(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene9focusItemEv()};
    unsafe {_ZNK14QGraphicsScene9focusItemEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addText<T: QGraphicsScene_addText>(&mut self, value: T) -> i32 {
    value.addText(self);
    return 1;
  }
}

pub trait QGraphicsScene_addText {
  fn addText(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QGraphicsTextItem * QGraphicsScene::addText(const QString & text, const QFont & font);
impl<'a> /*trait*/ QGraphicsScene_addText for (&'a  QString, &'a  QFont) {
  fn addText(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addTextERK7QStringRK5QFont()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene7addTextERK7QStringRK5QFont(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setSortCacheEnabled<T: QGraphicsScene_setSortCacheEnabled>(&mut self, value: T) -> i32 {
    value.setSortCacheEnabled(self);
    return 1;
  }
}

pub trait QGraphicsScene_setSortCacheEnabled {
  fn setSortCacheEnabled(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::setSortCacheEnabled(bool enabled);
impl<'a> /*trait*/ QGraphicsScene_setSortCacheEnabled for (i8) {
  fn setSortCacheEnabled(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene19setSortCacheEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN14QGraphicsScene19setSortCacheEnabledEb(arg0)};
    return 1;
  }
}

// proto: QGraphicsItem * QGraphicsScene::itemAt(const QPointF & pos, const QTransform & deviceTransform);
impl<'a> /*trait*/ QGraphicsScene_itemAt for (&'a  QPointF, &'a  QTransform) {
  fn itemAt(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene6itemAtERK7QPointFRK10QTransform()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK14QGraphicsScene6itemAtERK7QPointFRK10QTransform(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn destroyItemGroup<T: QGraphicsScene_destroyItemGroup>(&mut self, value: T) -> i32 {
    value.destroyItemGroup(self);
    return 1;
  }
}

pub trait QGraphicsScene_destroyItemGroup {
  fn destroyItemGroup(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::destroyItemGroup(QGraphicsItemGroup * group);
impl<'a> /*trait*/ QGraphicsScene_destroyItemGroup for (&'a mut QGraphicsItemGroup) {
  fn destroyItemGroup(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene16destroyItemGroupEP18QGraphicsItemGroup()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QGraphicsScene16destroyItemGroupEP18QGraphicsItemGroup(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn width<T: QGraphicsScene_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QGraphicsScene_width {
  fn width(self, this: &mut QGraphicsScene) -> i32;
}

// proto: double QGraphicsScene::width();
impl<'a> /*trait*/ QGraphicsScene_width for () {
  fn width(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene5widthEv()};
    unsafe {_ZNK14QGraphicsScene5widthEv()};
    return 1;
  }
}

// proto: void QGraphicsScene::update(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsScene_update for (f64, f64, f64, f64) {
  fn update(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene6updateEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN14QGraphicsScene6updateEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addItem<T: QGraphicsScene_addItem>(&mut self, value: T) -> i32 {
    value.addItem(self);
    return 1;
  }
}

pub trait QGraphicsScene_addItem {
  fn addItem(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::addItem(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsScene_addItem for (&'a mut QGraphicsItem) {
  fn addItem(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addItemEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QGraphicsScene7addItemEP13QGraphicsItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setBackgroundBrush<T: QGraphicsScene_setBackgroundBrush>(&mut self, value: T) -> i32 {
    value.setBackgroundBrush(self);
    return 1;
  }
}

pub trait QGraphicsScene_setBackgroundBrush {
  fn setBackgroundBrush(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::setBackgroundBrush(const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_setBackgroundBrush for (&'a  QBrush) {
  fn setBackgroundBrush(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene18setBackgroundBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene18setBackgroundBrushERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn activePanel<T: QGraphicsScene_activePanel>(&mut self, value: T) -> i32 {
    value.activePanel(self);
    return 1;
  }
}

pub trait QGraphicsScene_activePanel {
  fn activePanel(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QGraphicsItem * QGraphicsScene::activePanel();
impl<'a> /*trait*/ QGraphicsScene_activePanel for () {
  fn activePanel(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene11activePanelEv()};
    unsafe {_ZNK14QGraphicsScene11activePanelEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn sceneRectChanged<T: QGraphicsScene_sceneRectChanged>(&mut self, value: T) -> i32 {
    value.sceneRectChanged(self);
    return 1;
  }
}

pub trait QGraphicsScene_sceneRectChanged {
  fn sceneRectChanged(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::sceneRectChanged(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsScene_sceneRectChanged for (&'a  QRectF) {
  fn sceneRectChanged(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene16sceneRectChangedERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene16sceneRectChangedERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn style<T: QGraphicsScene_style>(&mut self, value: T) -> i32 {
    value.style(self);
    return 1;
  }
}

pub trait QGraphicsScene_style {
  fn style(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QStyle * QGraphicsScene::style();
impl<'a> /*trait*/ QGraphicsScene_style for () {
  fn style(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene5styleEv()};
    unsafe {_ZNK14QGraphicsScene5styleEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setFont<T: QGraphicsScene_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QGraphicsScene_setFont {
  fn setFont(self, this: &mut QGraphicsScene) -> i32;
}

// proto: void QGraphicsScene::setFont(const QFont & font);
impl<'a> /*trait*/ QGraphicsScene_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene7setFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addPath<T: QGraphicsScene_addPath>(&mut self, value: T) -> i32 {
    value.addPath(self);
    return 1;
  }
}

pub trait QGraphicsScene_addPath {
  fn addPath(self, this: &mut QGraphicsScene) -> i32;
}

// proto: QGraphicsPathItem * QGraphicsScene::addPath(const QPainterPath & path, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addPath for (&'a  QPainterPath, &'a  QPen, &'a  QBrush) {
  fn addPath(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addPathERK12QPainterPathRK4QPenRK6QBrush()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScene7addPathERK12QPainterPathRK4QPenRK6QBrush(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn stickyFocus<T: QGraphicsScene_stickyFocus>(&mut self, value: T) -> i32 {
    value.stickyFocus(self);
    return 1;
  }
}

pub trait QGraphicsScene_stickyFocus {
  fn stickyFocus(self, this: &mut QGraphicsScene) -> i32;
}

// proto: bool QGraphicsScene::stickyFocus();
impl<'a> /*trait*/ QGraphicsScene_stickyFocus for () {
  fn stickyFocus(self, this: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene11stickyFocusEv()};
    unsafe {_ZNK14QGraphicsScene11stickyFocusEv()};
    return 1;
  }
}

