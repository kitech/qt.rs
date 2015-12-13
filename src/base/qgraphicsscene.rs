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
use super::qpainterpath::QPainterPath;
use super::qpolygonf::QPolygonF;
use super::qpen::QPen;
use super::qlinef::QLineF;
use super::qpalette::QPalette;
use super::qobject::QObject;
use super::qstring::QString;
use super::qfont::QFont;
use super::qtransform::QTransform;
use super::qpixmap::QPixmap;
use super::qstyle::QStyle;
use super::qgraphicswidget::QGraphicsWidget;
use super::qpointf::QPointF;
use super::qgraphicsitemgroup::QGraphicsItemGroup;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsScene::setForegroundBrush(const QBrush & brush);
  fn _ZN14QGraphicsScene18setForegroundBrushERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsScene::setSceneRect(const QRectF & rect);
  fn _ZN14QGraphicsScene12setSceneRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QGraphicsScene::isActive();
  fn _ZNK14QGraphicsScene8isActiveEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QGraphicsScene::hasFocus();
  fn _ZNK14QGraphicsScene8hasFocusEv(qthis: *mut c_void) -> int8_t;
  // proto:  QRectF QGraphicsScene::itemsBoundingRect();
  fn _ZNK14QGraphicsScene17itemsBoundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsScene::sendEvent(QGraphicsItem * item, QEvent * event);
  fn _ZN14QGraphicsScene9sendEventEP13QGraphicsItemP6QEvent(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  double QGraphicsScene::minimumRenderSize();
  fn _ZNK14QGraphicsScene17minimumRenderSizeEv(qthis: *mut c_void) -> c_double;
  // proto:  QPainterPath QGraphicsScene::selectionArea();
  fn _ZNK14QGraphicsScene13selectionAreaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsScene::update(const QRectF & rect);
  fn _ZN14QGraphicsScene6updateERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QGraphicsPolygonItem * QGraphicsScene::addPolygon(const QPolygonF & polygon, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene10addPolygonERK9QPolygonFRK4QPenRK6QBrush(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  QGraphicsLineItem * QGraphicsScene::addLine(const QLineF & line, const QPen & pen);
  fn _ZN14QGraphicsScene7addLineERK6QLineFRK4QPen(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QPalette QGraphicsScene::palette();
  fn _ZNK14QGraphicsScene7paletteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsScene::isSortCacheEnabled();
  fn _ZNK14QGraphicsScene18isSortCacheEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsScene::NewQGraphicsScene(const QRectF & sceneRect, QObject * parent);
  fn _ZN14QGraphicsSceneC1ERK6QRectFP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QGraphicsScene::NewQGraphicsScene(QObject * parent);
  fn _ZN14QGraphicsSceneC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsScene::clearFocus();
  fn _ZN14QGraphicsScene10clearFocusEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QGraphicsScene::metaObject();
  fn _ZNK14QGraphicsScene10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QGraphicsSimpleTextItem * QGraphicsScene::addSimpleText(const QString & text, const QFont & font);
  fn _ZN14QGraphicsScene13addSimpleTextERK7QStringRK5QFont(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QGraphicsLineItem * QGraphicsScene::addLine(qreal x1, qreal y1, qreal x2, qreal y2, const QPen & pen);
  fn _ZN14QGraphicsScene7addLineEddddRK4QPen(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void) ;
  // proto:  void QGraphicsScene::setBspTreeDepth(int depth);
  fn _ZN14QGraphicsScene15setBspTreeDepthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QRectF QGraphicsScene::sceneRect();
  fn _ZNK14QGraphicsScene9sceneRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsWidget * QGraphicsScene::activeWindow();
  fn _ZNK14QGraphicsScene12activeWindowEv(qthis: *mut c_void) ;
  // proto:  QBrush QGraphicsScene::backgroundBrush();
  fn _ZNK14QGraphicsScene15backgroundBrushEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsScene::itemAt(qreal x, qreal y, const QTransform & deviceTransform);
  fn _ZNK14QGraphicsScene6itemAtEddRK10QTransform(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: *mut c_void) ;
  // proto:  void QGraphicsScene::advance();
  fn _ZN14QGraphicsScene7advanceEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsScene::setStickyFocus(bool enabled);
  fn _ZN14QGraphicsScene14setStickyFocusEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QList<QGraphicsItem *> QGraphicsScene::selectedItems();
  fn _ZNK14QGraphicsScene13selectedItemsEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsScene::clear();
  fn _ZN14QGraphicsScene5clearEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsScene::setActivePanel(QGraphicsItem * item);
  fn _ZN14QGraphicsScene14setActivePanelEP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QGraphicsPixmapItem * QGraphicsScene::addPixmap(const QPixmap & pixmap);
  fn _ZN14QGraphicsScene9addPixmapERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QBrush QGraphicsScene::foregroundBrush();
  fn _ZNK14QGraphicsScene15foregroundBrushEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsScene::selectionChanged();
  fn _ZN14QGraphicsScene16selectionChangedEv(qthis: *mut c_void) ;
  // proto:  QList<QGraphicsView *> QGraphicsScene::views();
  fn _ZNK14QGraphicsScene5viewsEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsScene::FreeQGraphicsScene();
  fn _ZN14QGraphicsSceneD0Ev(qthis: *mut c_void) ;
  // proto:  QGraphicsRectItem * QGraphicsScene::addRect(qreal x, qreal y, qreal w, qreal h, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene7addRectEddddRK4QPenRK6QBrush(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void, arg5: *mut c_void) ;
  // proto:  int QGraphicsScene::bspTreeDepth();
  fn _ZNK14QGraphicsScene12bspTreeDepthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGraphicsScene::setSceneRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN14QGraphicsScene12setSceneRectEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  void QGraphicsScene::setStyle(QStyle * style);
  fn _ZN14QGraphicsScene8setStyleEP6QStyle(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsScene::setPalette(const QPalette & palette);
  fn _ZN14QGraphicsScene10setPaletteERK8QPalette(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsScene::setMinimumRenderSize(qreal minSize);
  fn _ZN14QGraphicsScene20setMinimumRenderSizeEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsScene::NewQGraphicsScene(qreal x, qreal y, qreal width, qreal height, QObject * parent);
  fn _ZN14QGraphicsSceneC1EddddP7QObject(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void) ;
  // proto:  QGraphicsItem * QGraphicsScene::mouseGrabberItem();
  fn _ZNK14QGraphicsScene16mouseGrabberItemEv(qthis: *mut c_void) ;
  // proto:  QGraphicsRectItem * QGraphicsScene::addRect(const QRectF & rect, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene7addRectERK6QRectFRK4QPenRK6QBrush(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  QGraphicsEllipseItem * QGraphicsScene::addEllipse(const QRectF & rect, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene10addEllipseERK6QRectFRK4QPenRK6QBrush(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  double QGraphicsScene::height();
  fn _ZNK14QGraphicsScene6heightEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsScene::setSelectionArea(const QPainterPath & path, const QTransform & deviceTransform);
  fn _ZN14QGraphicsScene16setSelectionAreaERK12QPainterPathRK10QTransform(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QFont QGraphicsScene::font();
  fn _ZNK14QGraphicsScene4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsScene::clearSelection();
  fn _ZN14QGraphicsScene14clearSelectionEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsScene::NewQGraphicsScene(const QGraphicsScene & );
  fn _ZN14QGraphicsSceneC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsScene::removeItem(QGraphicsItem * item);
  fn _ZN14QGraphicsScene10removeItemEP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QGraphicsEllipseItem * QGraphicsScene::addEllipse(qreal x, qreal y, qreal w, qreal h, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene10addEllipseEddddRK4QPenRK6QBrush(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void, arg5: *mut c_void) ;
  // proto:  void QGraphicsScene::setActiveWindow(QGraphicsWidget * widget);
  fn _ZN14QGraphicsScene15setActiveWindowEP15QGraphicsWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QGraphicsItem * QGraphicsScene::focusItem();
  fn _ZNK14QGraphicsScene9focusItemEv(qthis: *mut c_void) ;
  // proto:  QGraphicsTextItem * QGraphicsScene::addText(const QString & text, const QFont & font);
  fn _ZN14QGraphicsScene7addTextERK7QStringRK5QFont(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QGraphicsScene::setSortCacheEnabled(bool enabled);
  fn _ZN14QGraphicsScene19setSortCacheEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QGraphicsItem * QGraphicsScene::itemAt(const QPointF & pos, const QTransform & deviceTransform);
  fn _ZNK14QGraphicsScene6itemAtERK7QPointFRK10QTransform(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QGraphicsScene::destroyItemGroup(QGraphicsItemGroup * group);
  fn _ZN14QGraphicsScene16destroyItemGroupEP18QGraphicsItemGroup(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QGraphicsScene::width();
  fn _ZNK14QGraphicsScene5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsScene::update(qreal x, qreal y, qreal w, qreal h);
  fn _ZN14QGraphicsScene6updateEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  void QGraphicsScene::addItem(QGraphicsItem * item);
  fn _ZN14QGraphicsScene7addItemEP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsScene::setBackgroundBrush(const QBrush & brush);
  fn _ZN14QGraphicsScene18setBackgroundBrushERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QGraphicsItem * QGraphicsScene::activePanel();
  fn _ZNK14QGraphicsScene11activePanelEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsScene::sceneRectChanged(const QRectF & rect);
  fn _ZN14QGraphicsScene16sceneRectChangedERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStyle * QGraphicsScene::style();
  fn _ZNK14QGraphicsScene5styleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsScene::setFont(const QFont & font);
  fn _ZN14QGraphicsScene7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QGraphicsPathItem * QGraphicsScene::addPath(const QPainterPath & path, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene7addPathERK12QPainterPathRK4QPenRK6QBrush(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  bool QGraphicsScene::stickyFocus();
  fn _ZNK14QGraphicsScene11stickyFocusEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QGraphicsScene)=1
pub struct QGraphicsScene {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsScene {
  pub fn setForegroundBrush<T: QGraphicsScene_setForegroundBrush>(&mut self, value: T)  {
     value.setForegroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setForegroundBrush {
  fn setForegroundBrush(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::setForegroundBrush(const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_setForegroundBrush for (&'a  QBrush) {
  fn setForegroundBrush(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene18setForegroundBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene18setForegroundBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setSceneRect<T: QGraphicsScene_setSceneRect>(&mut self, value: T)  {
     value.setSceneRect(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setSceneRect {
  fn setSceneRect(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::setSceneRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsScene_setSceneRect for (&'a  QRectF) {
  fn setSceneRect(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene12setSceneRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene12setSceneRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn isActive<T: QGraphicsScene_isActive>(&mut self, value: T) -> i8 {
    return value.isActive(self);
    // return 1;
  }
}

pub trait QGraphicsScene_isActive {
  fn isActive(self, rsthis: &mut QGraphicsScene) -> i8;
}

// proto:  bool QGraphicsScene::isActive();
impl<'a> /*trait*/ QGraphicsScene_isActive for () {
  fn isActive(self, rsthis: &mut QGraphicsScene) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene8isActiveEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn hasFocus<T: QGraphicsScene_hasFocus>(&mut self, value: T) -> i8 {
    return value.hasFocus(self);
    // return 1;
  }
}

pub trait QGraphicsScene_hasFocus {
  fn hasFocus(self, rsthis: &mut QGraphicsScene) -> i8;
}

// proto:  bool QGraphicsScene::hasFocus();
impl<'a> /*trait*/ QGraphicsScene_hasFocus for () {
  fn hasFocus(self, rsthis: &mut QGraphicsScene) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene8hasFocusEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene8hasFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn itemsBoundingRect<T: QGraphicsScene_itemsBoundingRect>(&mut self, value: T) -> QRectF {
    return value.itemsBoundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsScene_itemsBoundingRect {
  fn itemsBoundingRect(self, rsthis: &mut QGraphicsScene) -> QRectF;
}

// proto:  QRectF QGraphicsScene::itemsBoundingRect();
impl<'a> /*trait*/ QGraphicsScene_itemsBoundingRect for () {
  fn itemsBoundingRect(self, rsthis: &mut QGraphicsScene) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene17itemsBoundingRectEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene17itemsBoundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn sendEvent<T: QGraphicsScene_sendEvent>(&mut self, value: T) -> i8 {
    return value.sendEvent(self);
    // return 1;
  }
}

pub trait QGraphicsScene_sendEvent {
  fn sendEvent(self, rsthis: &mut QGraphicsScene) -> i8;
}

// proto:  bool QGraphicsScene::sendEvent(QGraphicsItem * item, QEvent * event);
impl<'a> /*trait*/ QGraphicsScene_sendEvent for (&'a mut QGraphicsItem, &'a mut QEvent) {
  fn sendEvent(self, rsthis: &mut QGraphicsScene) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene9sendEventEP13QGraphicsItemP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QGraphicsScene9sendEventEP13QGraphicsItemP6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn minimumRenderSize<T: QGraphicsScene_minimumRenderSize>(&mut self, value: T) -> f64 {
    return value.minimumRenderSize(self);
    // return 1;
  }
}

pub trait QGraphicsScene_minimumRenderSize {
  fn minimumRenderSize(self, rsthis: &mut QGraphicsScene) -> f64;
}

// proto:  double QGraphicsScene::minimumRenderSize();
impl<'a> /*trait*/ QGraphicsScene_minimumRenderSize for () {
  fn minimumRenderSize(self, rsthis: &mut QGraphicsScene) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene17minimumRenderSizeEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene17minimumRenderSizeEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn selectionArea<T: QGraphicsScene_selectionArea>(&mut self, value: T) -> QPainterPath {
    return value.selectionArea(self);
    // return 1;
  }
}

pub trait QGraphicsScene_selectionArea {
  fn selectionArea(self, rsthis: &mut QGraphicsScene) -> QPainterPath;
}

// proto:  QPainterPath QGraphicsScene::selectionArea();
impl<'a> /*trait*/ QGraphicsScene_selectionArea for () {
  fn selectionArea(self, rsthis: &mut QGraphicsScene) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene13selectionAreaEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene13selectionAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn update<T: QGraphicsScene_update>(&mut self, value: T)  {
     value.update(self);
    // return 1;
  }
}

pub trait QGraphicsScene_update {
  fn update(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::update(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsScene_update for (&'a  QRectF) {
  fn update(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene6updateERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene6updateERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addPolygon<T: QGraphicsScene_addPolygon>(&mut self, value: T)  {
     value.addPolygon(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addPolygon {
  fn addPolygon(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QGraphicsPolygonItem * QGraphicsScene::addPolygon(const QPolygonF & polygon, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addPolygon for (&'a  QPolygonF, &'a  QPen, &'a  QBrush) {
  fn addPolygon(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10addPolygonERK9QPolygonFRK4QPenRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene10addPolygonERK9QPolygonFRK4QPenRK6QBrush(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addLine<T: QGraphicsScene_addLine>(&mut self, value: T)  {
     value.addLine(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addLine {
  fn addLine(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QGraphicsLineItem * QGraphicsScene::addLine(const QLineF & line, const QPen & pen);
impl<'a> /*trait*/ QGraphicsScene_addLine for (&'a  QLineF, &'a  QPen) {
  fn addLine(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addLineERK6QLineFRK4QPen()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7addLineERK6QLineFRK4QPen(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn palette<T: QGraphicsScene_palette>(&mut self, value: T) -> QPalette {
    return value.palette(self);
    // return 1;
  }
}

pub trait QGraphicsScene_palette {
  fn palette(self, rsthis: &mut QGraphicsScene) -> QPalette;
}

// proto:  QPalette QGraphicsScene::palette();
impl<'a> /*trait*/ QGraphicsScene_palette for () {
  fn palette(self, rsthis: &mut QGraphicsScene) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene7paletteEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene7paletteEv(rsthis.qclsinst)};
    let mut ret1 = QPalette{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn isSortCacheEnabled<T: QGraphicsScene_isSortCacheEnabled>(&mut self, value: T) -> i8 {
    return value.isSortCacheEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsScene_isSortCacheEnabled {
  fn isSortCacheEnabled(self, rsthis: &mut QGraphicsScene) -> i8;
}

// proto:  bool QGraphicsScene::isSortCacheEnabled();
impl<'a> /*trait*/ QGraphicsScene_isSortCacheEnabled for () {
  fn isSortCacheEnabled(self, rsthis: &mut QGraphicsScene) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene18isSortCacheEnabledEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene18isSortCacheEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
    let arg0 = self.0.qclsinst  as *mut c_void;
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
  pub fn clearFocus<T: QGraphicsScene_clearFocus>(&mut self, value: T)  {
     value.clearFocus(self);
    // return 1;
  }
}

pub trait QGraphicsScene_clearFocus {
  fn clearFocus(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::clearFocus();
impl<'a> /*trait*/ QGraphicsScene_clearFocus for () {
  fn clearFocus(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10clearFocusEv()};
     unsafe {_ZN14QGraphicsScene10clearFocusEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn metaObject<T: QGraphicsScene_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsScene_metaObject {
  fn metaObject(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  const QMetaObject * QGraphicsScene::metaObject();
impl<'a> /*trait*/ QGraphicsScene_metaObject for () {
  fn metaObject(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene10metaObjectEv()};
     unsafe {_ZNK14QGraphicsScene10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addSimpleText<T: QGraphicsScene_addSimpleText>(&mut self, value: T)  {
     value.addSimpleText(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addSimpleText {
  fn addSimpleText(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QGraphicsSimpleTextItem * QGraphicsScene::addSimpleText(const QString & text, const QFont & font);
impl<'a> /*trait*/ QGraphicsScene_addSimpleText for (&'a  QString, &'a  QFont) {
  fn addSimpleText(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene13addSimpleTextERK7QStringRK5QFont()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene13addSimpleTextERK7QStringRK5QFont(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QGraphicsLineItem * QGraphicsScene::addLine(qreal x1, qreal y1, qreal x2, qreal y2, const QPen & pen);
impl<'a> /*trait*/ QGraphicsScene_addLine for (f64, f64, f64, f64, &'a  QPen) {
  fn addLine(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addLineEddddRK4QPen()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7addLineEddddRK4QPen(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setBspTreeDepth<T: QGraphicsScene_setBspTreeDepth>(&mut self, value: T)  {
     value.setBspTreeDepth(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setBspTreeDepth {
  fn setBspTreeDepth(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::setBspTreeDepth(int depth);
impl<'a> /*trait*/ QGraphicsScene_setBspTreeDepth for (i32) {
  fn setBspTreeDepth(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene15setBspTreeDepthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QGraphicsScene15setBspTreeDepthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn sceneRect<T: QGraphicsScene_sceneRect>(&mut self, value: T) -> QRectF {
    return value.sceneRect(self);
    // return 1;
  }
}

pub trait QGraphicsScene_sceneRect {
  fn sceneRect(self, rsthis: &mut QGraphicsScene) -> QRectF;
}

// proto:  QRectF QGraphicsScene::sceneRect();
impl<'a> /*trait*/ QGraphicsScene_sceneRect for () {
  fn sceneRect(self, rsthis: &mut QGraphicsScene) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene9sceneRectEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene9sceneRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn activeWindow<T: QGraphicsScene_activeWindow>(&mut self, value: T)  {
     value.activeWindow(self);
    // return 1;
  }
}

pub trait QGraphicsScene_activeWindow {
  fn activeWindow(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QGraphicsWidget * QGraphicsScene::activeWindow();
impl<'a> /*trait*/ QGraphicsScene_activeWindow for () {
  fn activeWindow(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene12activeWindowEv()};
     unsafe {_ZNK14QGraphicsScene12activeWindowEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn backgroundBrush<T: QGraphicsScene_backgroundBrush>(&mut self, value: T) -> QBrush {
    return value.backgroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsScene_backgroundBrush {
  fn backgroundBrush(self, rsthis: &mut QGraphicsScene) -> QBrush;
}

// proto:  QBrush QGraphicsScene::backgroundBrush();
impl<'a> /*trait*/ QGraphicsScene_backgroundBrush for () {
  fn backgroundBrush(self, rsthis: &mut QGraphicsScene) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene15backgroundBrushEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene15backgroundBrushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn itemAt<T: QGraphicsScene_itemAt>(&mut self, value: T)  {
     value.itemAt(self);
    // return 1;
  }
}

pub trait QGraphicsScene_itemAt {
  fn itemAt(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QGraphicsItem * QGraphicsScene::itemAt(qreal x, qreal y, const QTransform & deviceTransform);
impl<'a> /*trait*/ QGraphicsScene_itemAt for (f64, f64, &'a  QTransform) {
  fn itemAt(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene6itemAtEddRK10QTransform()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZNK14QGraphicsScene6itemAtEddRK10QTransform(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn advance<T: QGraphicsScene_advance>(&mut self, value: T)  {
     value.advance(self);
    // return 1;
  }
}

pub trait QGraphicsScene_advance {
  fn advance(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::advance();
impl<'a> /*trait*/ QGraphicsScene_advance for () {
  fn advance(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7advanceEv()};
     unsafe {_ZN14QGraphicsScene7advanceEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setStickyFocus<T: QGraphicsScene_setStickyFocus>(&mut self, value: T)  {
     value.setStickyFocus(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setStickyFocus {
  fn setStickyFocus(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::setStickyFocus(bool enabled);
impl<'a> /*trait*/ QGraphicsScene_setStickyFocus for (i8) {
  fn setStickyFocus(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene14setStickyFocusEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QGraphicsScene14setStickyFocusEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn selectedItems<T: QGraphicsScene_selectedItems>(&mut self, value: T)  {
     value.selectedItems(self);
    // return 1;
  }
}

pub trait QGraphicsScene_selectedItems {
  fn selectedItems(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QList<QGraphicsItem *> QGraphicsScene::selectedItems();
impl<'a> /*trait*/ QGraphicsScene_selectedItems for () {
  fn selectedItems(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene13selectedItemsEv()};
     unsafe {_ZNK14QGraphicsScene13selectedItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn clear<T: QGraphicsScene_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QGraphicsScene_clear {
  fn clear(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::clear();
impl<'a> /*trait*/ QGraphicsScene_clear for () {
  fn clear(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene5clearEv()};
     unsafe {_ZN14QGraphicsScene5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setActivePanel<T: QGraphicsScene_setActivePanel>(&mut self, value: T)  {
     value.setActivePanel(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setActivePanel {
  fn setActivePanel(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::setActivePanel(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsScene_setActivePanel for (&'a mut QGraphicsItem) {
  fn setActivePanel(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene14setActivePanelEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene14setActivePanelEP13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addPixmap<T: QGraphicsScene_addPixmap>(&mut self, value: T)  {
     value.addPixmap(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addPixmap {
  fn addPixmap(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QGraphicsPixmapItem * QGraphicsScene::addPixmap(const QPixmap & pixmap);
impl<'a> /*trait*/ QGraphicsScene_addPixmap for (&'a  QPixmap) {
  fn addPixmap(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene9addPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene9addPixmapERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn foregroundBrush<T: QGraphicsScene_foregroundBrush>(&mut self, value: T) -> QBrush {
    return value.foregroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsScene_foregroundBrush {
  fn foregroundBrush(self, rsthis: &mut QGraphicsScene) -> QBrush;
}

// proto:  QBrush QGraphicsScene::foregroundBrush();
impl<'a> /*trait*/ QGraphicsScene_foregroundBrush for () {
  fn foregroundBrush(self, rsthis: &mut QGraphicsScene) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene15foregroundBrushEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene15foregroundBrushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn selectionChanged<T: QGraphicsScene_selectionChanged>(&mut self, value: T)  {
     value.selectionChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScene_selectionChanged {
  fn selectionChanged(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::selectionChanged();
impl<'a> /*trait*/ QGraphicsScene_selectionChanged for () {
  fn selectionChanged(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene16selectionChangedEv()};
     unsafe {_ZN14QGraphicsScene16selectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn views<T: QGraphicsScene_views>(&mut self, value: T)  {
     value.views(self);
    // return 1;
  }
}

pub trait QGraphicsScene_views {
  fn views(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QList<QGraphicsView *> QGraphicsScene::views();
impl<'a> /*trait*/ QGraphicsScene_views for () {
  fn views(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene5viewsEv()};
     unsafe {_ZNK14QGraphicsScene5viewsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn FreeQGraphicsScene<T: QGraphicsScene_FreeQGraphicsScene>(&mut self, value: T)  {
     value.FreeQGraphicsScene(self);
    // return 1;
  }
}

pub trait QGraphicsScene_FreeQGraphicsScene {
  fn FreeQGraphicsScene(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::FreeQGraphicsScene();
impl<'a> /*trait*/ QGraphicsScene_FreeQGraphicsScene for () {
  fn FreeQGraphicsScene(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsSceneD0Ev()};
     unsafe {_ZN14QGraphicsSceneD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addRect<T: QGraphicsScene_addRect>(&mut self, value: T)  {
     value.addRect(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addRect {
  fn addRect(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QGraphicsRectItem * QGraphicsScene::addRect(qreal x, qreal y, qreal w, qreal h, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addRect for (f64, f64, f64, f64, &'a  QPen, &'a  QBrush) {
  fn addRect(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addRectEddddRK4QPenRK6QBrush()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7addRectEddddRK4QPenRK6QBrush(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn bspTreeDepth<T: QGraphicsScene_bspTreeDepth>(&mut self, value: T) -> i32 {
    return value.bspTreeDepth(self);
    // return 1;
  }
}

pub trait QGraphicsScene_bspTreeDepth {
  fn bspTreeDepth(self, rsthis: &mut QGraphicsScene) -> i32;
}

// proto:  int QGraphicsScene::bspTreeDepth();
impl<'a> /*trait*/ QGraphicsScene_bspTreeDepth for () {
  fn bspTreeDepth(self, rsthis: &mut QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene12bspTreeDepthEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene12bspTreeDepthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QGraphicsScene::setSceneRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsScene_setSceneRect for (f64, f64, f64, f64) {
  fn setSceneRect(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene12setSceneRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN14QGraphicsScene12setSceneRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setStyle<T: QGraphicsScene_setStyle>(&mut self, value: T)  {
     value.setStyle(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setStyle {
  fn setStyle(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::setStyle(QStyle * style);
impl<'a> /*trait*/ QGraphicsScene_setStyle for (&'a mut QStyle) {
  fn setStyle(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene8setStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene8setStyleEP6QStyle(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setPalette<T: QGraphicsScene_setPalette>(&mut self, value: T)  {
     value.setPalette(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setPalette {
  fn setPalette(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::setPalette(const QPalette & palette);
impl<'a> /*trait*/ QGraphicsScene_setPalette for (&'a  QPalette) {
  fn setPalette(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene10setPaletteERK8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setMinimumRenderSize<T: QGraphicsScene_setMinimumRenderSize>(&mut self, value: T)  {
     value.setMinimumRenderSize(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setMinimumRenderSize {
  fn setMinimumRenderSize(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::setMinimumRenderSize(qreal minSize);
impl<'a> /*trait*/ QGraphicsScene_setMinimumRenderSize for (f64) {
  fn setMinimumRenderSize(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene20setMinimumRenderSizeEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QGraphicsScene20setMinimumRenderSizeEd(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn mouseGrabberItem<T: QGraphicsScene_mouseGrabberItem>(&mut self, value: T)  {
     value.mouseGrabberItem(self);
    // return 1;
  }
}

pub trait QGraphicsScene_mouseGrabberItem {
  fn mouseGrabberItem(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QGraphicsItem * QGraphicsScene::mouseGrabberItem();
impl<'a> /*trait*/ QGraphicsScene_mouseGrabberItem for () {
  fn mouseGrabberItem(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene16mouseGrabberItemEv()};
     unsafe {_ZNK14QGraphicsScene16mouseGrabberItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QGraphicsRectItem * QGraphicsScene::addRect(const QRectF & rect, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addRect for (&'a  QRectF, &'a  QPen, &'a  QBrush) {
  fn addRect(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addRectERK6QRectFRK4QPenRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7addRectERK6QRectFRK4QPenRK6QBrush(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addEllipse<T: QGraphicsScene_addEllipse>(&mut self, value: T)  {
     value.addEllipse(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addEllipse {
  fn addEllipse(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QGraphicsEllipseItem * QGraphicsScene::addEllipse(const QRectF & rect, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addEllipse for (&'a  QRectF, &'a  QPen, &'a  QBrush) {
  fn addEllipse(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10addEllipseERK6QRectFRK4QPenRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene10addEllipseERK6QRectFRK4QPenRK6QBrush(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn height<T: QGraphicsScene_height>(&mut self, value: T) -> f64 {
    return value.height(self);
    // return 1;
  }
}

pub trait QGraphicsScene_height {
  fn height(self, rsthis: &mut QGraphicsScene) -> f64;
}

// proto:  double QGraphicsScene::height();
impl<'a> /*trait*/ QGraphicsScene_height for () {
  fn height(self, rsthis: &mut QGraphicsScene) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene6heightEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setSelectionArea<T: QGraphicsScene_setSelectionArea>(&mut self, value: T)  {
     value.setSelectionArea(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setSelectionArea {
  fn setSelectionArea(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::setSelectionArea(const QPainterPath & path, const QTransform & deviceTransform);
impl<'a> /*trait*/ QGraphicsScene_setSelectionArea for (&'a  QPainterPath, &'a  QTransform) {
  fn setSelectionArea(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene16setSelectionAreaERK12QPainterPathRK10QTransform()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene16setSelectionAreaERK12QPainterPathRK10QTransform(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn font<T: QGraphicsScene_font>(&mut self, value: T) -> QFont {
    return value.font(self);
    // return 1;
  }
}

pub trait QGraphicsScene_font {
  fn font(self, rsthis: &mut QGraphicsScene) -> QFont;
}

// proto:  QFont QGraphicsScene::font();
impl<'a> /*trait*/ QGraphicsScene_font for () {
  fn font(self, rsthis: &mut QGraphicsScene) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene4fontEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn clearSelection<T: QGraphicsScene_clearSelection>(&mut self, value: T)  {
     value.clearSelection(self);
    // return 1;
  }
}

pub trait QGraphicsScene_clearSelection {
  fn clearSelection(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::clearSelection();
impl<'a> /*trait*/ QGraphicsScene_clearSelection for () {
  fn clearSelection(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene14clearSelectionEv()};
     unsafe {_ZN14QGraphicsScene14clearSelectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QGraphicsScene::NewQGraphicsScene(const QGraphicsScene & );
impl<'a> /*trait*/ QGraphicsScene_NewQGraphicsScene for (&'a  QGraphicsScene) {
  fn NewQGraphicsScene(self) -> QGraphicsScene {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsSceneC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QGraphicsSceneC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsScene{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn removeItem<T: QGraphicsScene_removeItem>(&mut self, value: T)  {
     value.removeItem(self);
    // return 1;
  }
}

pub trait QGraphicsScene_removeItem {
  fn removeItem(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::removeItem(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsScene_removeItem for (&'a mut QGraphicsItem) {
  fn removeItem(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10removeItemEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene10removeItemEP13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QGraphicsEllipseItem * QGraphicsScene::addEllipse(qreal x, qreal y, qreal w, qreal h, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addEllipse for (f64, f64, f64, f64, &'a  QPen, &'a  QBrush) {
  fn addEllipse(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10addEllipseEddddRK4QPenRK6QBrush()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene10addEllipseEddddRK4QPenRK6QBrush(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setActiveWindow<T: QGraphicsScene_setActiveWindow>(&mut self, value: T)  {
     value.setActiveWindow(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setActiveWindow {
  fn setActiveWindow(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::setActiveWindow(QGraphicsWidget * widget);
impl<'a> /*trait*/ QGraphicsScene_setActiveWindow for (&'a mut QGraphicsWidget) {
  fn setActiveWindow(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene15setActiveWindowEP15QGraphicsWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene15setActiveWindowEP15QGraphicsWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn focusItem<T: QGraphicsScene_focusItem>(&mut self, value: T)  {
     value.focusItem(self);
    // return 1;
  }
}

pub trait QGraphicsScene_focusItem {
  fn focusItem(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QGraphicsItem * QGraphicsScene::focusItem();
impl<'a> /*trait*/ QGraphicsScene_focusItem for () {
  fn focusItem(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene9focusItemEv()};
     unsafe {_ZNK14QGraphicsScene9focusItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addText<T: QGraphicsScene_addText>(&mut self, value: T)  {
     value.addText(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addText {
  fn addText(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QGraphicsTextItem * QGraphicsScene::addText(const QString & text, const QFont & font);
impl<'a> /*trait*/ QGraphicsScene_addText for (&'a  QString, &'a  QFont) {
  fn addText(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addTextERK7QStringRK5QFont()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7addTextERK7QStringRK5QFont(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setSortCacheEnabled<T: QGraphicsScene_setSortCacheEnabled>(&mut self, value: T)  {
     value.setSortCacheEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setSortCacheEnabled {
  fn setSortCacheEnabled(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::setSortCacheEnabled(bool enabled);
impl<'a> /*trait*/ QGraphicsScene_setSortCacheEnabled for (i8) {
  fn setSortCacheEnabled(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene19setSortCacheEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QGraphicsScene19setSortCacheEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QGraphicsItem * QGraphicsScene::itemAt(const QPointF & pos, const QTransform & deviceTransform);
impl<'a> /*trait*/ QGraphicsScene_itemAt for (&'a  QPointF, &'a  QTransform) {
  fn itemAt(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene6itemAtERK7QPointFRK10QTransform()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZNK14QGraphicsScene6itemAtERK7QPointFRK10QTransform(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn destroyItemGroup<T: QGraphicsScene_destroyItemGroup>(&mut self, value: T)  {
     value.destroyItemGroup(self);
    // return 1;
  }
}

pub trait QGraphicsScene_destroyItemGroup {
  fn destroyItemGroup(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::destroyItemGroup(QGraphicsItemGroup * group);
impl<'a> /*trait*/ QGraphicsScene_destroyItemGroup for (&'a mut QGraphicsItemGroup) {
  fn destroyItemGroup(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene16destroyItemGroupEP18QGraphicsItemGroup()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene16destroyItemGroupEP18QGraphicsItemGroup(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn width<T: QGraphicsScene_width>(&mut self, value: T) -> f64 {
    return value.width(self);
    // return 1;
  }
}

pub trait QGraphicsScene_width {
  fn width(self, rsthis: &mut QGraphicsScene) -> f64;
}

// proto:  double QGraphicsScene::width();
impl<'a> /*trait*/ QGraphicsScene_width for () {
  fn width(self, rsthis: &mut QGraphicsScene) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene5widthEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QGraphicsScene::update(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsScene_update for (f64, f64, f64, f64) {
  fn update(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene6updateEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN14QGraphicsScene6updateEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addItem<T: QGraphicsScene_addItem>(&mut self, value: T)  {
     value.addItem(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addItem {
  fn addItem(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::addItem(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsScene_addItem for (&'a mut QGraphicsItem) {
  fn addItem(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addItemEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7addItemEP13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setBackgroundBrush<T: QGraphicsScene_setBackgroundBrush>(&mut self, value: T)  {
     value.setBackgroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setBackgroundBrush {
  fn setBackgroundBrush(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::setBackgroundBrush(const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_setBackgroundBrush for (&'a  QBrush) {
  fn setBackgroundBrush(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene18setBackgroundBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene18setBackgroundBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn activePanel<T: QGraphicsScene_activePanel>(&mut self, value: T)  {
     value.activePanel(self);
    // return 1;
  }
}

pub trait QGraphicsScene_activePanel {
  fn activePanel(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QGraphicsItem * QGraphicsScene::activePanel();
impl<'a> /*trait*/ QGraphicsScene_activePanel for () {
  fn activePanel(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene11activePanelEv()};
     unsafe {_ZNK14QGraphicsScene11activePanelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn sceneRectChanged<T: QGraphicsScene_sceneRectChanged>(&mut self, value: T)  {
     value.sceneRectChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScene_sceneRectChanged {
  fn sceneRectChanged(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::sceneRectChanged(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsScene_sceneRectChanged for (&'a  QRectF) {
  fn sceneRectChanged(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene16sceneRectChangedERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene16sceneRectChangedERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn style<T: QGraphicsScene_style>(&mut self, value: T) -> QStyle {
    return value.style(self);
    // return 1;
  }
}

pub trait QGraphicsScene_style {
  fn style(self, rsthis: &mut QGraphicsScene) -> QStyle;
}

// proto:  QStyle * QGraphicsScene::style();
impl<'a> /*trait*/ QGraphicsScene_style for () {
  fn style(self, rsthis: &mut QGraphicsScene) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene5styleEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene5styleEv(rsthis.qclsinst)};
    let mut ret1 = QStyle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn setFont<T: QGraphicsScene_setFont>(&mut self, value: T)  {
     value.setFont(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setFont {
  fn setFont(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  void QGraphicsScene::setFont(const QFont & font);
impl<'a> /*trait*/ QGraphicsScene_setFont for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn addPath<T: QGraphicsScene_addPath>(&mut self, value: T)  {
     value.addPath(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addPath {
  fn addPath(self, rsthis: &mut QGraphicsScene) ;
}

// proto:  QGraphicsPathItem * QGraphicsScene::addPath(const QPainterPath & path, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addPath for (&'a  QPainterPath, &'a  QPen, &'a  QBrush) {
  fn addPath(self, rsthis: &mut QGraphicsScene)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addPathERK12QPainterPathRK4QPenRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7addPathERK12QPainterPathRK4QPenRK6QBrush(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScene {
  pub fn stickyFocus<T: QGraphicsScene_stickyFocus>(&mut self, value: T) -> i8 {
    return value.stickyFocus(self);
    // return 1;
  }
}

pub trait QGraphicsScene_stickyFocus {
  fn stickyFocus(self, rsthis: &mut QGraphicsScene) -> i8;
}

// proto:  bool QGraphicsScene::stickyFocus();
impl<'a> /*trait*/ QGraphicsScene_stickyFocus for () {
  fn stickyFocus(self, rsthis: &mut QGraphicsScene) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene11stickyFocusEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene11stickyFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

