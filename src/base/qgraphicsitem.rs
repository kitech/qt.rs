// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpainterpath::QPainterPath;
use super::qpointf::QPointF;
use super::qtransform::QTransform;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qpolygonf::QPolygonF;
use super::qrectf::QRectF;
use super::qgraphicseffect::QGraphicsEffect;
use super::qstring::QString;
use super::qvariant::QVariant;
use super::qgraphicsitemgroup::QGraphicsItemGroup;
use super::qmatrix::QMatrix;
use super::qcursor::QCursor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsItem::NewQGraphicsItem(const QGraphicsItem & );
  fn _ZN13QGraphicsItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QPainterPath QGraphicsItem::mapFromParent(const QPainterPath & path);
  fn _ZNK13QGraphicsItem13mapFromParentERK12QPainterPath(arg0: *const c_void) -> i32;
  // proto: QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPointF & point);
  fn _ZNK13QGraphicsItem11mapFromItemEPKS_RK7QPointF(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QGraphicsItem * QGraphicsItem::focusItem();
  fn _ZNK13QGraphicsItem9focusItemEv() -> i32;
  // proto: QGraphicsObject * QGraphicsItem::parentObject();
  fn _ZNK13QGraphicsItem12parentObjectEv() -> i32;
  // proto: void QGraphicsItem::setTransformOriginPoint(const QPointF & origin);
  fn _ZN13QGraphicsItem23setTransformOriginPointERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsItem::ungrabMouse();
  fn _ZN13QGraphicsItem11ungrabMouseEv() -> i32;
  // proto: int QGraphicsItem::type_();
  fn _ZNK13QGraphicsItem4typeEv() -> i32;
  // proto: bool QGraphicsItem::isSelected();
  fn _ZNK13QGraphicsItem10isSelectedEv() -> i32;
  // proto: QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem11mapFromItemEPKS_dddd(arg0: *const c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> i32;
  // proto: QGraphicsWidget * QGraphicsItem::parentWidget();
  fn _ZNK13QGraphicsItem12parentWidgetEv() -> i32;
  // proto: void QGraphicsItem::resetTransform();
  fn _ZN13QGraphicsItem14resetTransformEv() -> i32;
  // proto: QRegion QGraphicsItem::boundingRegion(const QTransform & itemToDeviceTransform);
  fn _ZNK13QGraphicsItem14boundingRegionERK10QTransform(arg0: *const c_void) -> i32;
  // proto: void QGraphicsItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN13QGraphicsItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0: *mut c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: bool QGraphicsItem::isActive();
  fn _ZNK13QGraphicsItem8isActiveEv() -> i32;
  // proto: void QGraphicsItem::NewQGraphicsItem(QGraphicsItem * parent);
  fn _ZN13QGraphicsItemC1EPS_(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QPolygonF QGraphicsItem::mapToParent(const QPolygonF & polygon);
  fn _ZNK13QGraphicsItem11mapToParentERK9QPolygonF(arg0: *const c_void) -> i32;
  // proto: bool QGraphicsItem::isWidget();
  fn _ZNK13QGraphicsItem8isWidgetEv() -> i32;
  // proto: void QGraphicsItem::setParentItem(QGraphicsItem * parent);
  fn _ZN13QGraphicsItem13setParentItemEPS_(arg0: *mut c_void) -> i32;
  // proto: QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QRectF & rect);
  fn _ZNK13QGraphicsItem9mapToItemEPKS_RK6QRectF(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QGraphicsWidget * QGraphicsItem::window();
  fn _ZNK13QGraphicsItem6windowEv() -> i32;
  // proto: QPointF QGraphicsItem::scenePos();
  fn _ZNK13QGraphicsItem8scenePosEv() -> i32;
  // proto: bool QGraphicsItem::handlesChildEvents();
  fn _ZNK13QGraphicsItem18handlesChildEventsEv() -> i32;
  // proto: void QGraphicsItem::setOpacity(qreal opacity);
  fn _ZN13QGraphicsItem10setOpacityEd(arg0: c_double) -> i32;
  // proto: QTransform QGraphicsItem::sceneTransform();
  fn _ZNK13QGraphicsItem14sceneTransformEv() -> i32;
  // proto: void QGraphicsItem::setZValue(qreal z);
  fn _ZN13QGraphicsItem9setZValueEd(arg0: c_double) -> i32;
  // proto: QPolygonF QGraphicsItem::mapFromParent(const QRectF & rect);
  fn _ZNK13QGraphicsItem13mapFromParentERK6QRectF(arg0: *const c_void) -> i32;
  // proto: QPolygonF QGraphicsItem::mapFromParent(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem13mapFromParentEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: bool QGraphicsItem::isObscured(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem10isObscuredEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsItem::installSceneEventFilter(QGraphicsItem * filterItem);
  fn _ZN13QGraphicsItem23installSceneEventFilterEPS_(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsItem::setY(qreal y);
  fn _ZN13QGraphicsItem4setYEd(arg0: c_double) -> i32;
  // proto: QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem13mapRectToItemEPKS_dddd(arg0: *const c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> i32;
  // proto: QGraphicsItem * QGraphicsItem::parentItem();
  fn _ZNK13QGraphicsItem10parentItemEv() -> i32;
  // proto: void QGraphicsItem::clearFocus();
  fn _ZN13QGraphicsItem10clearFocusEv() -> i32;
  // proto: bool QGraphicsItem::isWindow();
  fn _ZNK13QGraphicsItem8isWindowEv() -> i32;
  // proto: QPointF QGraphicsItem::transformOriginPoint();
  fn _ZNK13QGraphicsItem20transformOriginPointEv() -> i32;
  // proto: QRectF QGraphicsItem::boundingRect();
  fn _ZNK13QGraphicsItem12boundingRectEv() -> i32;
  // proto: QRectF QGraphicsItem::childrenBoundingRect();
  fn _ZNK13QGraphicsItem20childrenBoundingRectEv() -> i32;
  // proto: bool QGraphicsItem::isObscured(const QRectF & rect);
  fn _ZNK13QGraphicsItem10isObscuredERK6QRectF(arg0: *const c_void) -> i32;
  // proto: QPolygonF QGraphicsItem::mapFromScene(const QRectF & rect);
  fn _ZNK13QGraphicsItem12mapFromSceneERK6QRectF(arg0: *const c_void) -> i32;
  // proto: bool QGraphicsItem::hasCursor();
  fn _ZNK13QGraphicsItem9hasCursorEv() -> i32;
  // proto: void QGraphicsItem::setGraphicsEffect(QGraphicsEffect * effect);
  fn _ZN13QGraphicsItem17setGraphicsEffectEP15QGraphicsEffect(arg0: *mut c_void) -> i32;
  // proto: QPainterPath QGraphicsItem::mapToParent(const QPainterPath & path);
  fn _ZNK13QGraphicsItem11mapToParentERK12QPainterPath(arg0: *const c_void) -> i32;
  // proto: void QGraphicsItem::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
  fn _ZN13QGraphicsItem13ensureVisibleEddddii(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_int, arg5: c_int) -> i32;
  // proto: QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem9mapToItemEPKS_dddd(arg0: *const c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> i32;
  // proto: QPointF QGraphicsItem::mapToItem(const QGraphicsItem * item, qreal x, qreal y);
  fn _ZNK13QGraphicsItem9mapToItemEPKS_dd(arg0: *const c_void, arg1: c_double, arg2: c_double) -> i32;
  // proto: QRectF QGraphicsItem::mapRectToParent(const QRectF & rect);
  fn _ZNK13QGraphicsItem15mapRectToParentERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsItem::setToolTip(const QString & toolTip);
  fn _ZN13QGraphicsItem10setToolTipERK7QString(arg0: *const c_void) -> i32;
  // proto: double QGraphicsItem::rotation();
  fn _ZNK13QGraphicsItem8rotationEv() -> i32;
  // proto: QGraphicsScene * QGraphicsItem::scene();
  fn _ZNK13QGraphicsItem5sceneEv() -> i32;
  // proto: QPainterPath QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPainterPath & path);
  fn _ZNK13QGraphicsItem9mapToItemEPKS_RK12QPainterPath(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QRectF QGraphicsItem::mapRectToParent(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem15mapRectToParentEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QRectF & rect);
  fn _ZNK13QGraphicsItem11mapFromItemEPKS_RK6QRectF(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QRectF QGraphicsItem::mapRectFromParent(const QRectF & rect);
  fn _ZNK13QGraphicsItem17mapRectFromParentERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsItem::setFocusProxy(QGraphicsItem * item);
  fn _ZN13QGraphicsItem13setFocusProxyEPS_(arg0: *mut c_void) -> i32;
  // proto: bool QGraphicsItem::acceptDrops();
  fn _ZNK13QGraphicsItem11acceptDropsEv() -> i32;
  // proto: QPointF QGraphicsItem::mapToParent(const QPointF & point);
  fn _ZNK13QGraphicsItem11mapToParentERK7QPointF(arg0: *const c_void) -> i32;
  // proto: QRectF QGraphicsItem::mapRectFromScene(const QRectF & rect);
  fn _ZNK13QGraphicsItem16mapRectFromSceneERK6QRectF(arg0: *const c_void) -> i32;
  // proto: QGraphicsItem * QGraphicsItem::focusScopeItem();
  fn _ZNK13QGraphicsItem14focusScopeItemEv() -> i32;
  // proto: void QGraphicsItem::removeSceneEventFilter(QGraphicsItem * filterItem);
  fn _ZN13QGraphicsItem22removeSceneEventFilterEPS_(arg0: *mut c_void) -> i32;
  // proto: QGraphicsItem * QGraphicsItem::focusProxy();
  fn _ZNK13QGraphicsItem10focusProxyEv() -> i32;
  // proto: QPointF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPointF & point);
  fn _ZNK13QGraphicsItem9mapToItemEPKS_RK7QPointF(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QRectF QGraphicsItem::sceneBoundingRect();
  fn _ZNK13QGraphicsItem17sceneBoundingRectEv() -> i32;
  // proto: void QGraphicsItem::FreeQGraphicsItem();
  fn _ZN13QGraphicsItemD0Ev() -> i32;
  // proto: void QGraphicsItem::setX(qreal x);
  fn _ZN13QGraphicsItem4setXEd(arg0: c_double) -> i32;
  // proto: void QGraphicsItem::update(qreal x, qreal y, qreal width, qreal height);
  fn _ZN13QGraphicsItem6updateEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsItem::setSelected(bool selected);
  fn _ZN13QGraphicsItem11setSelectedEb(arg0: int8_t) -> i32;
  // proto: QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, const QRectF & rect);
  fn _ZNK13QGraphicsItem13mapRectToItemEPKS_RK6QRectF(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QGraphicsItem::stackBefore(const QGraphicsItem * sibling);
  fn _ZN13QGraphicsItem11stackBeforeEPKS_(arg0: *const c_void) -> i32;
  // proto: QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, qreal x, qreal y);
  fn _ZNK13QGraphicsItem11mapFromItemEPKS_dd(arg0: *const c_void, arg1: c_double, arg2: c_double) -> i32;
  // proto: void QGraphicsItem::resetMatrix();
  fn _ZN13QGraphicsItem11resetMatrixEv() -> i32;
  // proto: QPainterPath QGraphicsItem::opaqueArea();
  fn _ZNK13QGraphicsItem10opaqueAreaEv() -> i32;
  // proto: void QGraphicsItem::unsetCursor();
  fn _ZN13QGraphicsItem11unsetCursorEv() -> i32;
  // proto: QPointF QGraphicsItem::mapFromParent(qreal x, qreal y);
  fn _ZNK13QGraphicsItem13mapFromParentEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QRectF QGraphicsItem::mapRectToScene(const QRectF & rect);
  fn _ZNK13QGraphicsItem14mapRectToSceneERK6QRectF(arg0: *const c_void) -> i32;
  // proto: QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem15mapRectFromItemEPKS_dddd(arg0: *const c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> i32;
  // proto: double QGraphicsItem::scale();
  fn _ZNK13QGraphicsItem5scaleEv() -> i32;
  // proto: void QGraphicsItem::setBoundingRegionGranularity(qreal granularity);
  fn _ZN13QGraphicsItem28setBoundingRegionGranularityEd(arg0: c_double) -> i32;
  // proto: void QGraphicsItem::setAcceptDrops(bool on);
  fn _ZN13QGraphicsItem14setAcceptDropsEb(arg0: int8_t) -> i32;
  // proto: QPolygonF QGraphicsItem::mapFromScene(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem12mapFromSceneEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsItem::ungrabKeyboard();
  fn _ZN13QGraphicsItem14ungrabKeyboardEv() -> i32;
  // proto: void QGraphicsItem::setEnabled(bool enabled);
  fn _ZN13QGraphicsItem10setEnabledEb(arg0: int8_t) -> i32;
  // proto: QGraphicsEffect * QGraphicsItem::graphicsEffect();
  fn _ZNK13QGraphicsItem14graphicsEffectEv() -> i32;
  // proto: bool QGraphicsItem::acceptHoverEvents();
  fn _ZNK13QGraphicsItem17acceptHoverEventsEv() -> i32;
  // proto: QGraphicsWidget * QGraphicsItem::topLevelWidget();
  fn _ZNK13QGraphicsItem14topLevelWidgetEv() -> i32;
  // proto: QList<QGraphicsTransform *> QGraphicsItem::transformations();
  fn _ZNK13QGraphicsItem15transformationsEv() -> i32;
  // proto: QPolygonF QGraphicsItem::mapToScene(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem10mapToSceneEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: QPointF QGraphicsItem::mapToScene(qreal x, qreal y);
  fn _ZNK13QGraphicsItem10mapToSceneEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QRectF QGraphicsItem::mapRectFromScene(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem16mapRectFromSceneEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsItem::advance(int phase);
  fn _ZN13QGraphicsItem7advanceEi(arg0: c_int) -> i32;
  // proto: QMatrix QGraphicsItem::sceneMatrix();
  fn _ZNK13QGraphicsItem11sceneMatrixEv() -> i32;
  // proto: void QGraphicsItem::setFiltersChildEvents(bool enabled);
  fn _ZN13QGraphicsItem21setFiltersChildEventsEb(arg0: int8_t) -> i32;
  // proto: QPolygonF QGraphicsItem::mapToScene(const QPolygonF & polygon);
  fn _ZNK13QGraphicsItem10mapToSceneERK9QPolygonF(arg0: *const c_void) -> i32;
  // proto: QTransform QGraphicsItem::itemTransform(const QGraphicsItem * other, bool * ok);
  fn _ZNK13QGraphicsItem13itemTransformEPKS_Pb(arg0: *const c_void, arg1: *mut int8_t) -> i32;
  // proto: void QGraphicsItem::setTransformOriginPoint(qreal ax, qreal ay);
  fn _ZN13QGraphicsItem23setTransformOriginPointEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QGraphicsItem::moveBy(qreal dx, qreal dy);
  fn _ZN13QGraphicsItem6moveByEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QPolygonF QGraphicsItem::mapFromScene(const QPolygonF & polygon);
  fn _ZNK13QGraphicsItem12mapFromSceneERK9QPolygonF(arg0: *const c_void) -> i32;
  // proto: QGraphicsItemGroup * QGraphicsItem::group();
  fn _ZNK13QGraphicsItem5groupEv() -> i32;
  // proto: QPainterPath QGraphicsItem::shape();
  fn _ZNK13QGraphicsItem5shapeEv() -> i32;
  // proto: QPointF QGraphicsItem::mapFromScene(qreal x, qreal y);
  fn _ZNK13QGraphicsItem12mapFromSceneEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QGraphicsItem::scroll(qreal dx, qreal dy, const QRectF & rect);
  fn _ZN13QGraphicsItem6scrollEddRK6QRectF(arg0: c_double, arg1: c_double, arg2: *const c_void) -> i32;
  // proto: bool QGraphicsItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK13QGraphicsItem12isObscuredByEPKS_(arg0: *const c_void) -> i32;
  // proto: QPointF QGraphicsItem::mapFromParent(const QPointF & point);
  fn _ZNK13QGraphicsItem13mapFromParentERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsItem::setData(int key, const QVariant & value);
  fn _ZN13QGraphicsItem7setDataEiRK8QVariant(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: QGraphicsItem * QGraphicsItem::commonAncestorItem(const QGraphicsItem * other);
  fn _ZNK13QGraphicsItem18commonAncestorItemEPKS_(arg0: *const c_void) -> i32;
  // proto: QPainterPath QGraphicsItem::mapFromScene(const QPainterPath & path);
  fn _ZNK13QGraphicsItem12mapFromSceneERK12QPainterPath(arg0: *const c_void) -> i32;
  // proto: QPainterPath QGraphicsItem::mapToScene(const QPainterPath & path);
  fn _ZNK13QGraphicsItem10mapToSceneERK12QPainterPath(arg0: *const c_void) -> i32;
  // proto: QPolygonF QGraphicsItem::mapToParent(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem11mapToParentEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsItem::setGroup(QGraphicsItemGroup * group);
  fn _ZN13QGraphicsItem8setGroupEP18QGraphicsItemGroup(arg0: *mut c_void) -> i32;
  // proto: QRectF QGraphicsItem::mapRectFromParent(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem17mapRectFromParentEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsItem::show();
  fn _ZN13QGraphicsItem4showEv() -> i32;
  // proto: QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, const QRectF & rect);
  fn _ZNK13QGraphicsItem15mapRectFromItemEPKS_RK6QRectF(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: double QGraphicsItem::y();
  fn _ZNK13QGraphicsItem1yEv() -> i32;
  // proto: QPointF QGraphicsItem::mapFromScene(const QPointF & point);
  fn _ZNK13QGraphicsItem12mapFromSceneERK7QPointF(arg0: *const c_void) -> i32;
  // proto: bool QGraphicsItem::hasFocus();
  fn _ZNK13QGraphicsItem8hasFocusEv() -> i32;
  // proto: QPainterPath QGraphicsItem::clipPath();
  fn _ZNK13QGraphicsItem8clipPathEv() -> i32;
  // proto: void QGraphicsItem::setPos(qreal x, qreal y);
  fn _ZN13QGraphicsItem6setPosEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: bool QGraphicsItem::isEnabled();
  fn _ZNK13QGraphicsItem9isEnabledEv() -> i32;
  // proto: bool QGraphicsItem::contains(const QPointF & point);
  fn _ZNK13QGraphicsItem8containsERK7QPointF(arg0: *const c_void) -> i32;
  // proto: bool QGraphicsItem::isPanel();
  fn _ZNK13QGraphicsItem7isPanelEv() -> i32;
  // proto: bool QGraphicsItem::filtersChildEvents();
  fn _ZNK13QGraphicsItem18filtersChildEventsEv() -> i32;
  // proto: void QGraphicsItem::grabKeyboard();
  fn _ZN13QGraphicsItem12grabKeyboardEv() -> i32;
  // proto: QPainterPath QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPainterPath & path);
  fn _ZNK13QGraphicsItem11mapFromItemEPKS_RK12QPainterPath(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QGraphicsItem::setActive(bool active);
  fn _ZN13QGraphicsItem9setActiveEb(arg0: int8_t) -> i32;
  // proto: QGraphicsObject * QGraphicsItem::toGraphicsObject();
  fn _ZN13QGraphicsItem16toGraphicsObjectEv() -> i32;
  // proto: QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPolygonF & polygon);
  fn _ZNK13QGraphicsItem11mapFromItemEPKS_RK9QPolygonF(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QGraphicsItem::setHandlesChildEvents(bool enabled);
  fn _ZN13QGraphicsItem21setHandlesChildEventsEb(arg0: int8_t) -> i32;
  // proto: QPolygonF QGraphicsItem::mapFromParent(const QPolygonF & polygon);
  fn _ZNK13QGraphicsItem13mapFromParentERK9QPolygonF(arg0: *const c_void) -> i32;
  // proto: QPointF QGraphicsItem::mapToParent(qreal x, qreal y);
  fn _ZNK13QGraphicsItem11mapToParentEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QGraphicsItem::setMatrix(const QMatrix & matrix, bool combine);
  fn _ZN13QGraphicsItem9setMatrixERK7QMatrixb(arg0: *const c_void, arg1: int8_t) -> i32;
  // proto: void QGraphicsItem::update(const QRectF & rect);
  fn _ZN13QGraphicsItem6updateERK6QRectF(arg0: *const c_void) -> i32;
  // proto: QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPolygonF & polygon);
  fn _ZNK13QGraphicsItem9mapToItemEPKS_RK9QPolygonF(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QTransform QGraphicsItem::transform();
  fn _ZNK13QGraphicsItem9transformEv() -> i32;
  // proto: QVariant QGraphicsItem::data(int key);
  fn _ZNK13QGraphicsItem4dataEi(arg0: c_int) -> i32;
  // proto: void QGraphicsItem::hide();
  fn _ZN13QGraphicsItem4hideEv() -> i32;
  // proto: bool QGraphicsItem::isUnderMouse();
  fn _ZNK13QGraphicsItem12isUnderMouseEv() -> i32;
  // proto: void QGraphicsItem::setAcceptTouchEvents(bool enabled);
  fn _ZN13QGraphicsItem20setAcceptTouchEventsEb(arg0: int8_t) -> i32;
  // proto: void QGraphicsItem::setAcceptHoverEvents(bool enabled);
  fn _ZN13QGraphicsItem20setAcceptHoverEventsEb(arg0: int8_t) -> i32;
  // proto: QList<QGraphicsItem *> QGraphicsItem::childItems();
  fn _ZNK13QGraphicsItem10childItemsEv() -> i32;
  // proto: bool QGraphicsItem::isAncestorOf(const QGraphicsItem * child);
  fn _ZNK13QGraphicsItem12isAncestorOfEPKS_(arg0: *const c_void) -> i32;
  // proto: double QGraphicsItem::opacity();
  fn _ZNK13QGraphicsItem7opacityEv() -> i32;
  // proto: bool QGraphicsItem::isVisibleTo(const QGraphicsItem * parent);
  fn _ZNK13QGraphicsItem11isVisibleToEPKS_(arg0: *const c_void) -> i32;
  // proto: QString QGraphicsItem::toolTip();
  fn _ZNK13QGraphicsItem7toolTipEv() -> i32;
  // proto: QCursor QGraphicsItem::cursor();
  fn _ZNK13QGraphicsItem6cursorEv() -> i32;
  // proto: QPointF QGraphicsItem::mapToScene(const QPointF & point);
  fn _ZNK13QGraphicsItem10mapToSceneERK7QPointF(arg0: *const c_void) -> i32;
  // proto: double QGraphicsItem::zValue();
  fn _ZNK13QGraphicsItem6zValueEv() -> i32;
  // proto: QMatrix QGraphicsItem::matrix();
  fn _ZNK13QGraphicsItem6matrixEv() -> i32;
  // proto: QRectF QGraphicsItem::mapRectToScene(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem14mapRectToSceneEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsItem::setPos(const QPointF & pos);
  fn _ZN13QGraphicsItem6setPosERK7QPointF(arg0: *const c_void) -> i32;
  // proto: QGraphicsItem * QGraphicsItem::panel();
  fn _ZNK13QGraphicsItem5panelEv() -> i32;
  // proto: bool QGraphicsItem::isClipped();
  fn _ZNK13QGraphicsItem9isClippedEv() -> i32;
  // proto: QGraphicsItem * QGraphicsItem::topLevelItem();
  fn _ZNK13QGraphicsItem12topLevelItemEv() -> i32;
  // proto: QPolygonF QGraphicsItem::mapToScene(const QRectF & rect);
  fn _ZNK13QGraphicsItem10mapToSceneERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsItem::setScale(qreal scale);
  fn _ZN13QGraphicsItem8setScaleEd(arg0: c_double) -> i32;
  // proto: void QGraphicsItem::setCursor(const QCursor & cursor);
  fn _ZN13QGraphicsItem9setCursorERK7QCursor(arg0: *const c_void) -> i32;
  // proto: bool QGraphicsItem::isVisible();
  fn _ZNK13QGraphicsItem9isVisibleEv() -> i32;
  // proto: QPointF QGraphicsItem::pos();
  fn _ZNK13QGraphicsItem3posEv() -> i32;
  // proto: bool QGraphicsItem::isBlockedByModalPanel(QGraphicsItem ** blockingPanel);
  fn _ZNK13QGraphicsItem21isBlockedByModalPanelEPPS_(arg0: *mut c_void) -> i32;
  // proto: double QGraphicsItem::effectiveOpacity();
  fn _ZNK13QGraphicsItem16effectiveOpacityEv() -> i32;
  // proto: void QGraphicsItem::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
  fn _ZN13QGraphicsItem13ensureVisibleERK6QRectFii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: double QGraphicsItem::boundingRegionGranularity();
  fn _ZNK13QGraphicsItem25boundingRegionGranularityEv() -> i32;
  // proto: double QGraphicsItem::x();
  fn _ZNK13QGraphicsItem1xEv() -> i32;
  // proto: void QGraphicsItem::grabMouse();
  fn _ZN13QGraphicsItem9grabMouseEv() -> i32;
  // proto: void QGraphicsItem::setVisible(bool visible);
  fn _ZN13QGraphicsItem10setVisibleEb(arg0: int8_t) -> i32;
  // proto: void QGraphicsItem::setRotation(qreal angle);
  fn _ZN13QGraphicsItem11setRotationEd(arg0: c_double) -> i32;
  // proto: QTransform QGraphicsItem::deviceTransform(const QTransform & viewportTransform);
  fn _ZNK13QGraphicsItem15deviceTransformERK10QTransform(arg0: *const c_void) -> i32;
  // proto: bool QGraphicsItem::acceptTouchEvents();
  fn _ZNK13QGraphicsItem17acceptTouchEventsEv() -> i32;
  // proto: void QGraphicsItem::setTransform(const QTransform & matrix, bool combine);
  fn _ZN13QGraphicsItem12setTransformERK10QTransformb(arg0: *const c_void, arg1: int8_t) -> i32;
  // proto: QPolygonF QGraphicsItem::mapToParent(const QRectF & rect);
  fn _ZNK13QGraphicsItem11mapToParentERK6QRectF(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QGraphicsItem)=1
pub struct QGraphicsItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsItem {
  pub fn NewQGraphicsItem<T: QGraphicsItem_NewQGraphicsItem>(value: T) -> QGraphicsItem {
    let rsthis = value.NewQGraphicsItem();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsItem_NewQGraphicsItem {
  fn NewQGraphicsItem(self) -> QGraphicsItem;
}

// proto: void QGraphicsItem::NewQGraphicsItem(const QGraphicsItem & );
impl<'a> /*trait*/ QGraphicsItem_NewQGraphicsItem for (&'a  QGraphicsItem) {
  fn NewQGraphicsItem(self) -> QGraphicsItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItemC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapFromParent<T: QGraphicsItem_mapFromParent>(&mut self, value: T) -> i32 {
    value.mapFromParent(self);
    return 1;
  }
}

pub trait QGraphicsItem_mapFromParent {
  fn mapFromParent(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QPainterPath QGraphicsItem::mapFromParent(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent for (&'a  QPainterPath) {
  fn mapFromParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentERK12QPainterPath()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem13mapFromParentERK12QPainterPath(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapFromItem<T: QGraphicsItem_mapFromItem>(&mut self, value: T) -> i32 {
    value.mapFromItem(self);
    return 1;
  }
}

pub trait QGraphicsItem_mapFromItem {
  fn mapFromItem(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem for (&'a  QGraphicsItem, &'a  QPointF) {
  fn mapFromItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_RK7QPointF()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem11mapFromItemEPKS_RK7QPointF(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn focusItem<T: QGraphicsItem_focusItem>(&mut self, value: T) -> i32 {
    value.focusItem(self);
    return 1;
  }
}

pub trait QGraphicsItem_focusItem {
  fn focusItem(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsItem * QGraphicsItem::focusItem();
impl<'a> /*trait*/ QGraphicsItem_focusItem for () {
  fn focusItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9focusItemEv()};
    unsafe {_ZNK13QGraphicsItem9focusItemEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn parentObject<T: QGraphicsItem_parentObject>(&mut self, value: T) -> i32 {
    value.parentObject(self);
    return 1;
  }
}

pub trait QGraphicsItem_parentObject {
  fn parentObject(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsObject * QGraphicsItem::parentObject();
impl<'a> /*trait*/ QGraphicsItem_parentObject for () {
  fn parentObject(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12parentObjectEv()};
    unsafe {_ZNK13QGraphicsItem12parentObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setTransformOriginPoint<T: QGraphicsItem_setTransformOriginPoint>(&mut self, value: T) -> i32 {
    value.setTransformOriginPoint(self);
    return 1;
  }
}

pub trait QGraphicsItem_setTransformOriginPoint {
  fn setTransformOriginPoint(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setTransformOriginPoint(const QPointF & origin);
impl<'a> /*trait*/ QGraphicsItem_setTransformOriginPoint for (&'a  QPointF) {
  fn setTransformOriginPoint(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem23setTransformOriginPointERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsItem23setTransformOriginPointERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn ungrabMouse<T: QGraphicsItem_ungrabMouse>(&mut self, value: T) -> i32 {
    value.ungrabMouse(self);
    return 1;
  }
}

pub trait QGraphicsItem_ungrabMouse {
  fn ungrabMouse(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::ungrabMouse();
impl<'a> /*trait*/ QGraphicsItem_ungrabMouse for () {
  fn ungrabMouse(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11ungrabMouseEv()};
    unsafe {_ZN13QGraphicsItem11ungrabMouseEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn type_<T: QGraphicsItem_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QGraphicsItem_type_ {
  fn type_(self, this: &mut QGraphicsItem) -> i32;
}

// proto: int QGraphicsItem::type_();
impl<'a> /*trait*/ QGraphicsItem_type_ for () {
  fn type_(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem4typeEv()};
    unsafe {_ZNK13QGraphicsItem4typeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isSelected<T: QGraphicsItem_isSelected>(&mut self, value: T) -> i32 {
    value.isSelected(self);
    return 1;
  }
}

pub trait QGraphicsItem_isSelected {
  fn isSelected(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isSelected();
impl<'a> /*trait*/ QGraphicsItem_isSelected for () {
  fn isSelected(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10isSelectedEv()};
    unsafe {_ZNK13QGraphicsItem10isSelectedEv()};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem for (&'a  QGraphicsItem, f64, f64, f64, f64) {
  fn mapFromItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    unsafe {_ZNK13QGraphicsItem11mapFromItemEPKS_dddd(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn parentWidget<T: QGraphicsItem_parentWidget>(&mut self, value: T) -> i32 {
    value.parentWidget(self);
    return 1;
  }
}

pub trait QGraphicsItem_parentWidget {
  fn parentWidget(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsWidget * QGraphicsItem::parentWidget();
impl<'a> /*trait*/ QGraphicsItem_parentWidget for () {
  fn parentWidget(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12parentWidgetEv()};
    unsafe {_ZNK13QGraphicsItem12parentWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn resetTransform<T: QGraphicsItem_resetTransform>(&mut self, value: T) -> i32 {
    value.resetTransform(self);
    return 1;
  }
}

pub trait QGraphicsItem_resetTransform {
  fn resetTransform(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::resetTransform();
impl<'a> /*trait*/ QGraphicsItem_resetTransform for () {
  fn resetTransform(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem14resetTransformEv()};
    unsafe {_ZN13QGraphicsItem14resetTransformEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn boundingRegion<T: QGraphicsItem_boundingRegion>(&mut self, value: T) -> i32 {
    value.boundingRegion(self);
    return 1;
  }
}

pub trait QGraphicsItem_boundingRegion {
  fn boundingRegion(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QRegion QGraphicsItem::boundingRegion(const QTransform & itemToDeviceTransform);
impl<'a> /*trait*/ QGraphicsItem_boundingRegion for (&'a  QTransform) {
  fn boundingRegion(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14boundingRegionERK10QTransform()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem14boundingRegionERK10QTransform(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn paint<T: QGraphicsItem_paint>(&mut self, value: T) -> i32 {
    value.paint(self);
    return 1;
  }
}

pub trait QGraphicsItem_paint {
  fn paint(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsItem_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isActive<T: QGraphicsItem_isActive>(&mut self, value: T) -> i32 {
    value.isActive(self);
    return 1;
  }
}

pub trait QGraphicsItem_isActive {
  fn isActive(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isActive();
impl<'a> /*trait*/ QGraphicsItem_isActive for () {
  fn isActive(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8isActiveEv()};
    unsafe {_ZNK13QGraphicsItem8isActiveEv()};
    return 1;
  }
}

// proto: void QGraphicsItem::NewQGraphicsItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsItem_NewQGraphicsItem for (&'a mut QGraphicsItem) {
  fn NewQGraphicsItem(self) -> QGraphicsItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItemC1EPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsItemC1EPS_(qthis, arg0)};
    let rsthis = QGraphicsItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapToParent<T: QGraphicsItem_mapToParent>(&mut self, value: T) -> i32 {
    value.mapToParent(self);
    return 1;
  }
}

pub trait QGraphicsItem_mapToParent {
  fn mapToParent(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QPolygonF QGraphicsItem::mapToParent(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapToParent for (&'a  QPolygonF) {
  fn mapToParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentERK9QPolygonF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem11mapToParentERK9QPolygonF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isWidget<T: QGraphicsItem_isWidget>(&mut self, value: T) -> i32 {
    value.isWidget(self);
    return 1;
  }
}

pub trait QGraphicsItem_isWidget {
  fn isWidget(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isWidget();
impl<'a> /*trait*/ QGraphicsItem_isWidget for () {
  fn isWidget(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8isWidgetEv()};
    unsafe {_ZNK13QGraphicsItem8isWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setParentItem<T: QGraphicsItem_setParentItem>(&mut self, value: T) -> i32 {
    value.setParentItem(self);
    return 1;
  }
}

pub trait QGraphicsItem_setParentItem {
  fn setParentItem(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setParentItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsItem_setParentItem for (&'a mut QGraphicsItem) {
  fn setParentItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13setParentItemEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsItem13setParentItemEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapToItem<T: QGraphicsItem_mapToItem>(&mut self, value: T) -> i32 {
    value.mapToItem(self);
    return 1;
  }
}

pub trait QGraphicsItem_mapToItem {
  fn mapToItem(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapToItem for (&'a  QGraphicsItem, &'a  QRectF) {
  fn mapToItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem9mapToItemEPKS_RK6QRectF(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn window<T: QGraphicsItem_window>(&mut self, value: T) -> i32 {
    value.window(self);
    return 1;
  }
}

pub trait QGraphicsItem_window {
  fn window(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsWidget * QGraphicsItem::window();
impl<'a> /*trait*/ QGraphicsItem_window for () {
  fn window(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6windowEv()};
    unsafe {_ZNK13QGraphicsItem6windowEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn scenePos<T: QGraphicsItem_scenePos>(&mut self, value: T) -> i32 {
    value.scenePos(self);
    return 1;
  }
}

pub trait QGraphicsItem_scenePos {
  fn scenePos(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QPointF QGraphicsItem::scenePos();
impl<'a> /*trait*/ QGraphicsItem_scenePos for () {
  fn scenePos(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8scenePosEv()};
    unsafe {_ZNK13QGraphicsItem8scenePosEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn handlesChildEvents<T: QGraphicsItem_handlesChildEvents>(&mut self, value: T) -> i32 {
    value.handlesChildEvents(self);
    return 1;
  }
}

pub trait QGraphicsItem_handlesChildEvents {
  fn handlesChildEvents(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::handlesChildEvents();
impl<'a> /*trait*/ QGraphicsItem_handlesChildEvents for () {
  fn handlesChildEvents(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem18handlesChildEventsEv()};
    unsafe {_ZNK13QGraphicsItem18handlesChildEventsEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setOpacity<T: QGraphicsItem_setOpacity>(&mut self, value: T) -> i32 {
    value.setOpacity(self);
    return 1;
  }
}

pub trait QGraphicsItem_setOpacity {
  fn setOpacity(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setOpacity(qreal opacity);
impl<'a> /*trait*/ QGraphicsItem_setOpacity for (f64) {
  fn setOpacity(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setOpacityEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QGraphicsItem10setOpacityEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn sceneTransform<T: QGraphicsItem_sceneTransform>(&mut self, value: T) -> i32 {
    value.sceneTransform(self);
    return 1;
  }
}

pub trait QGraphicsItem_sceneTransform {
  fn sceneTransform(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QTransform QGraphicsItem::sceneTransform();
impl<'a> /*trait*/ QGraphicsItem_sceneTransform for () {
  fn sceneTransform(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14sceneTransformEv()};
    unsafe {_ZNK13QGraphicsItem14sceneTransformEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setZValue<T: QGraphicsItem_setZValue>(&mut self, value: T) -> i32 {
    value.setZValue(self);
    return 1;
  }
}

pub trait QGraphicsItem_setZValue {
  fn setZValue(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setZValue(qreal z);
impl<'a> /*trait*/ QGraphicsItem_setZValue for (f64) {
  fn setZValue(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setZValueEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QGraphicsItem9setZValueEd(arg0)};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapFromParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent for (&'a  QRectF) {
  fn mapFromParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem13mapFromParentERK6QRectF(arg0)};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapFromParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent for (f64, f64, f64, f64) {
  fn mapFromParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZNK13QGraphicsItem13mapFromParentEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isObscured<T: QGraphicsItem_isObscured>(&mut self, value: T) -> i32 {
    value.isObscured(self);
    return 1;
  }
}

pub trait QGraphicsItem_isObscured {
  fn isObscured(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isObscured(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_isObscured for (f64, f64, f64, f64) {
  fn isObscured(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10isObscuredEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZNK13QGraphicsItem10isObscuredEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn installSceneEventFilter<T: QGraphicsItem_installSceneEventFilter>(&mut self, value: T) -> i32 {
    value.installSceneEventFilter(self);
    return 1;
  }
}

pub trait QGraphicsItem_installSceneEventFilter {
  fn installSceneEventFilter(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::installSceneEventFilter(QGraphicsItem * filterItem);
impl<'a> /*trait*/ QGraphicsItem_installSceneEventFilter for (&'a mut QGraphicsItem) {
  fn installSceneEventFilter(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem23installSceneEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsItem23installSceneEventFilterEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setY<T: QGraphicsItem_setY>(&mut self, value: T) -> i32 {
    value.setY(self);
    return 1;
  }
}

pub trait QGraphicsItem_setY {
  fn setY(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setY(qreal y);
impl<'a> /*trait*/ QGraphicsItem_setY for (f64) {
  fn setY(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4setYEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QGraphicsItem4setYEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapRectToItem<T: QGraphicsItem_mapRectToItem>(&mut self, value: T) -> i32 {
    value.mapRectToItem(self);
    return 1;
  }
}

pub trait QGraphicsItem_mapRectToItem {
  fn mapRectToItem(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectToItem for (&'a  QGraphicsItem, f64, f64, f64, f64) {
  fn mapRectToItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapRectToItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    unsafe {_ZNK13QGraphicsItem13mapRectToItemEPKS_dddd(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn parentItem<T: QGraphicsItem_parentItem>(&mut self, value: T) -> i32 {
    value.parentItem(self);
    return 1;
  }
}

pub trait QGraphicsItem_parentItem {
  fn parentItem(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsItem * QGraphicsItem::parentItem();
impl<'a> /*trait*/ QGraphicsItem_parentItem for () {
  fn parentItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10parentItemEv()};
    unsafe {_ZNK13QGraphicsItem10parentItemEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn clearFocus<T: QGraphicsItem_clearFocus>(&mut self, value: T) -> i32 {
    value.clearFocus(self);
    return 1;
  }
}

pub trait QGraphicsItem_clearFocus {
  fn clearFocus(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::clearFocus();
impl<'a> /*trait*/ QGraphicsItem_clearFocus for () {
  fn clearFocus(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10clearFocusEv()};
    unsafe {_ZN13QGraphicsItem10clearFocusEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isWindow<T: QGraphicsItem_isWindow>(&mut self, value: T) -> i32 {
    value.isWindow(self);
    return 1;
  }
}

pub trait QGraphicsItem_isWindow {
  fn isWindow(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isWindow();
impl<'a> /*trait*/ QGraphicsItem_isWindow for () {
  fn isWindow(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8isWindowEv()};
    unsafe {_ZNK13QGraphicsItem8isWindowEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn transformOriginPoint<T: QGraphicsItem_transformOriginPoint>(&mut self, value: T) -> i32 {
    value.transformOriginPoint(self);
    return 1;
  }
}

pub trait QGraphicsItem_transformOriginPoint {
  fn transformOriginPoint(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QPointF QGraphicsItem::transformOriginPoint();
impl<'a> /*trait*/ QGraphicsItem_transformOriginPoint for () {
  fn transformOriginPoint(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem20transformOriginPointEv()};
    unsafe {_ZNK13QGraphicsItem20transformOriginPointEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn boundingRect<T: QGraphicsItem_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QGraphicsItem_boundingRect {
  fn boundingRect(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QRectF QGraphicsItem::boundingRect();
impl<'a> /*trait*/ QGraphicsItem_boundingRect for () {
  fn boundingRect(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12boundingRectEv()};
    unsafe {_ZNK13QGraphicsItem12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn childrenBoundingRect<T: QGraphicsItem_childrenBoundingRect>(&mut self, value: T) -> i32 {
    value.childrenBoundingRect(self);
    return 1;
  }
}

pub trait QGraphicsItem_childrenBoundingRect {
  fn childrenBoundingRect(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QRectF QGraphicsItem::childrenBoundingRect();
impl<'a> /*trait*/ QGraphicsItem_childrenBoundingRect for () {
  fn childrenBoundingRect(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem20childrenBoundingRectEv()};
    unsafe {_ZNK13QGraphicsItem20childrenBoundingRectEv()};
    return 1;
  }
}

// proto: bool QGraphicsItem::isObscured(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_isObscured for (&'a  QRectF) {
  fn isObscured(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10isObscuredERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem10isObscuredERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapFromScene<T: QGraphicsItem_mapFromScene>(&mut self, value: T) -> i32 {
    value.mapFromScene(self);
    return 1;
  }
}

pub trait QGraphicsItem_mapFromScene {
  fn mapFromScene(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QPolygonF QGraphicsItem::mapFromScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene for (&'a  QRectF) {
  fn mapFromScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem12mapFromSceneERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn hasCursor<T: QGraphicsItem_hasCursor>(&mut self, value: T) -> i32 {
    value.hasCursor(self);
    return 1;
  }
}

pub trait QGraphicsItem_hasCursor {
  fn hasCursor(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::hasCursor();
impl<'a> /*trait*/ QGraphicsItem_hasCursor for () {
  fn hasCursor(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9hasCursorEv()};
    unsafe {_ZNK13QGraphicsItem9hasCursorEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setGraphicsEffect<T: QGraphicsItem_setGraphicsEffect>(&mut self, value: T) -> i32 {
    value.setGraphicsEffect(self);
    return 1;
  }
}

pub trait QGraphicsItem_setGraphicsEffect {
  fn setGraphicsEffect(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setGraphicsEffect(QGraphicsEffect * effect);
impl<'a> /*trait*/ QGraphicsItem_setGraphicsEffect for (&'a mut QGraphicsEffect) {
  fn setGraphicsEffect(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem17setGraphicsEffectEP15QGraphicsEffect()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsItem17setGraphicsEffectEP15QGraphicsEffect(arg0)};
    return 1;
  }
}

// proto: QPainterPath QGraphicsItem::mapToParent(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapToParent for (&'a  QPainterPath) {
  fn mapToParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentERK12QPainterPath()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem11mapToParentERK12QPainterPath(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn ensureVisible<T: QGraphicsItem_ensureVisible>(&mut self, value: T) -> i32 {
    value.ensureVisible(self);
    return 1;
  }
}

pub trait QGraphicsItem_ensureVisible {
  fn ensureVisible(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsItem_ensureVisible for (f64, f64, f64, f64, i32, i32) {
  fn ensureVisible(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13ensureVisibleEddddii()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    unsafe {_ZN13QGraphicsItem13ensureVisibleEddddii(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapToItem for (&'a  QGraphicsItem, f64, f64, f64, f64) {
  fn mapToItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    unsafe {_ZNK13QGraphicsItem9mapToItemEPKS_dddd(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

// proto: QPointF QGraphicsItem::mapToItem(const QGraphicsItem * item, qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapToItem for (&'a  QGraphicsItem, f64, f64) {
  fn mapToItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_dd()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    unsafe {_ZNK13QGraphicsItem9mapToItemEPKS_dd(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapRectToParent<T: QGraphicsItem_mapRectToParent>(&mut self, value: T) -> i32 {
    value.mapRectToParent(self);
    return 1;
  }
}

pub trait QGraphicsItem_mapRectToParent {
  fn mapRectToParent(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QRectF QGraphicsItem::mapRectToParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectToParent for (&'a  QRectF) {
  fn mapRectToParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectToParentERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem15mapRectToParentERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setToolTip<T: QGraphicsItem_setToolTip>(&mut self, value: T) -> i32 {
    value.setToolTip(self);
    return 1;
  }
}

pub trait QGraphicsItem_setToolTip {
  fn setToolTip(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QGraphicsItem_setToolTip for (&'a  QString) {
  fn setToolTip(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsItem10setToolTipERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn rotation<T: QGraphicsItem_rotation>(&mut self, value: T) -> i32 {
    value.rotation(self);
    return 1;
  }
}

pub trait QGraphicsItem_rotation {
  fn rotation(self, this: &mut QGraphicsItem) -> i32;
}

// proto: double QGraphicsItem::rotation();
impl<'a> /*trait*/ QGraphicsItem_rotation for () {
  fn rotation(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8rotationEv()};
    unsafe {_ZNK13QGraphicsItem8rotationEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn scene<T: QGraphicsItem_scene>(&mut self, value: T) -> i32 {
    value.scene(self);
    return 1;
  }
}

pub trait QGraphicsItem_scene {
  fn scene(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsScene * QGraphicsItem::scene();
impl<'a> /*trait*/ QGraphicsItem_scene for () {
  fn scene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5sceneEv()};
    unsafe {_ZNK13QGraphicsItem5sceneEv()};
    return 1;
  }
}

// proto: QPainterPath QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapToItem for (&'a  QGraphicsItem, &'a  QPainterPath) {
  fn mapToItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_RK12QPainterPath()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem9mapToItemEPKS_RK12QPainterPath(arg0, arg1)};
    return 1;
  }
}

// proto: QRectF QGraphicsItem::mapRectToParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectToParent for (f64, f64, f64, f64) {
  fn mapRectToParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectToParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZNK13QGraphicsItem15mapRectToParentEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem for (&'a  QGraphicsItem, &'a  QRectF) {
  fn mapFromItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem11mapFromItemEPKS_RK6QRectF(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromParent<T: QGraphicsItem_mapRectFromParent>(&mut self, value: T) -> i32 {
    value.mapRectFromParent(self);
    return 1;
  }
}

pub trait QGraphicsItem_mapRectFromParent {
  fn mapRectFromParent(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QRectF QGraphicsItem::mapRectFromParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromParent for (&'a  QRectF) {
  fn mapRectFromParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17mapRectFromParentERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem17mapRectFromParentERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setFocusProxy<T: QGraphicsItem_setFocusProxy>(&mut self, value: T) -> i32 {
    value.setFocusProxy(self);
    return 1;
  }
}

pub trait QGraphicsItem_setFocusProxy {
  fn setFocusProxy(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setFocusProxy(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItem_setFocusProxy for (&'a mut QGraphicsItem) {
  fn setFocusProxy(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13setFocusProxyEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsItem13setFocusProxyEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn acceptDrops<T: QGraphicsItem_acceptDrops>(&mut self, value: T) -> i32 {
    value.acceptDrops(self);
    return 1;
  }
}

pub trait QGraphicsItem_acceptDrops {
  fn acceptDrops(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::acceptDrops();
impl<'a> /*trait*/ QGraphicsItem_acceptDrops for () {
  fn acceptDrops(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11acceptDropsEv()};
    unsafe {_ZNK13QGraphicsItem11acceptDropsEv()};
    return 1;
  }
}

// proto: QPointF QGraphicsItem::mapToParent(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapToParent for (&'a  QPointF) {
  fn mapToParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem11mapToParentERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromScene<T: QGraphicsItem_mapRectFromScene>(&mut self, value: T) -> i32 {
    value.mapRectFromScene(self);
    return 1;
  }
}

pub trait QGraphicsItem_mapRectFromScene {
  fn mapRectFromScene(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QRectF QGraphicsItem::mapRectFromScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromScene for (&'a  QRectF) {
  fn mapRectFromScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem16mapRectFromSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem16mapRectFromSceneERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn focusScopeItem<T: QGraphicsItem_focusScopeItem>(&mut self, value: T) -> i32 {
    value.focusScopeItem(self);
    return 1;
  }
}

pub trait QGraphicsItem_focusScopeItem {
  fn focusScopeItem(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsItem * QGraphicsItem::focusScopeItem();
impl<'a> /*trait*/ QGraphicsItem_focusScopeItem for () {
  fn focusScopeItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14focusScopeItemEv()};
    unsafe {_ZNK13QGraphicsItem14focusScopeItemEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn removeSceneEventFilter<T: QGraphicsItem_removeSceneEventFilter>(&mut self, value: T) -> i32 {
    value.removeSceneEventFilter(self);
    return 1;
  }
}

pub trait QGraphicsItem_removeSceneEventFilter {
  fn removeSceneEventFilter(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::removeSceneEventFilter(QGraphicsItem * filterItem);
impl<'a> /*trait*/ QGraphicsItem_removeSceneEventFilter for (&'a mut QGraphicsItem) {
  fn removeSceneEventFilter(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem22removeSceneEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsItem22removeSceneEventFilterEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn focusProxy<T: QGraphicsItem_focusProxy>(&mut self, value: T) -> i32 {
    value.focusProxy(self);
    return 1;
  }
}

pub trait QGraphicsItem_focusProxy {
  fn focusProxy(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsItem * QGraphicsItem::focusProxy();
impl<'a> /*trait*/ QGraphicsItem_focusProxy for () {
  fn focusProxy(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10focusProxyEv()};
    unsafe {_ZNK13QGraphicsItem10focusProxyEv()};
    return 1;
  }
}

// proto: QPointF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapToItem for (&'a  QGraphicsItem, &'a  QPointF) {
  fn mapToItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_RK7QPointF()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem9mapToItemEPKS_RK7QPointF(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn sceneBoundingRect<T: QGraphicsItem_sceneBoundingRect>(&mut self, value: T) -> i32 {
    value.sceneBoundingRect(self);
    return 1;
  }
}

pub trait QGraphicsItem_sceneBoundingRect {
  fn sceneBoundingRect(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QRectF QGraphicsItem::sceneBoundingRect();
impl<'a> /*trait*/ QGraphicsItem_sceneBoundingRect for () {
  fn sceneBoundingRect(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17sceneBoundingRectEv()};
    unsafe {_ZNK13QGraphicsItem17sceneBoundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn FreeQGraphicsItem<T: QGraphicsItem_FreeQGraphicsItem>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsItem(self);
    return 1;
  }
}

pub trait QGraphicsItem_FreeQGraphicsItem {
  fn FreeQGraphicsItem(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::FreeQGraphicsItem();
impl<'a> /*trait*/ QGraphicsItem_FreeQGraphicsItem for () {
  fn FreeQGraphicsItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItemD0Ev()};
    unsafe {_ZN13QGraphicsItemD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setX<T: QGraphicsItem_setX>(&mut self, value: T) -> i32 {
    value.setX(self);
    return 1;
  }
}

pub trait QGraphicsItem_setX {
  fn setX(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setX(qreal x);
impl<'a> /*trait*/ QGraphicsItem_setX for (f64) {
  fn setX(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4setXEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QGraphicsItem4setXEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn update<T: QGraphicsItem_update>(&mut self, value: T) -> i32 {
    value.update(self);
    return 1;
  }
}

pub trait QGraphicsItem_update {
  fn update(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::update(qreal x, qreal y, qreal width, qreal height);
impl<'a> /*trait*/ QGraphicsItem_update for (f64, f64, f64, f64) {
  fn update(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6updateEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN13QGraphicsItem6updateEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setSelected<T: QGraphicsItem_setSelected>(&mut self, value: T) -> i32 {
    value.setSelected(self);
    return 1;
  }
}

pub trait QGraphicsItem_setSelected {
  fn setSelected(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setSelected(bool selected);
impl<'a> /*trait*/ QGraphicsItem_setSelected for (i8) {
  fn setSelected(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11setSelectedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QGraphicsItem11setSelectedEb(arg0)};
    return 1;
  }
}

// proto: QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectToItem for (&'a  QGraphicsItem, &'a  QRectF) {
  fn mapRectToItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapRectToItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem13mapRectToItemEPKS_RK6QRectF(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn stackBefore<T: QGraphicsItem_stackBefore>(&mut self, value: T) -> i32 {
    value.stackBefore(self);
    return 1;
  }
}

pub trait QGraphicsItem_stackBefore {
  fn stackBefore(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::stackBefore(const QGraphicsItem * sibling);
impl<'a> /*trait*/ QGraphicsItem_stackBefore for (&'a  QGraphicsItem) {
  fn stackBefore(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11stackBeforeEPKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsItem11stackBeforeEPKS_(arg0)};
    return 1;
  }
}

// proto: QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem for (&'a  QGraphicsItem, f64, f64) {
  fn mapFromItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_dd()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    unsafe {_ZNK13QGraphicsItem11mapFromItemEPKS_dd(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn resetMatrix<T: QGraphicsItem_resetMatrix>(&mut self, value: T) -> i32 {
    value.resetMatrix(self);
    return 1;
  }
}

pub trait QGraphicsItem_resetMatrix {
  fn resetMatrix(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::resetMatrix();
impl<'a> /*trait*/ QGraphicsItem_resetMatrix for () {
  fn resetMatrix(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11resetMatrixEv()};
    unsafe {_ZN13QGraphicsItem11resetMatrixEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn opaqueArea<T: QGraphicsItem_opaqueArea>(&mut self, value: T) -> i32 {
    value.opaqueArea(self);
    return 1;
  }
}

pub trait QGraphicsItem_opaqueArea {
  fn opaqueArea(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QPainterPath QGraphicsItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsItem_opaqueArea for () {
  fn opaqueArea(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10opaqueAreaEv()};
    unsafe {_ZNK13QGraphicsItem10opaqueAreaEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn unsetCursor<T: QGraphicsItem_unsetCursor>(&mut self, value: T) -> i32 {
    value.unsetCursor(self);
    return 1;
  }
}

pub trait QGraphicsItem_unsetCursor {
  fn unsetCursor(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::unsetCursor();
impl<'a> /*trait*/ QGraphicsItem_unsetCursor for () {
  fn unsetCursor(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11unsetCursorEv()};
    unsafe {_ZN13QGraphicsItem11unsetCursorEv()};
    return 1;
  }
}

// proto: QPointF QGraphicsItem::mapFromParent(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent for (f64, f64) {
  fn mapFromParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZNK13QGraphicsItem13mapFromParentEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapRectToScene<T: QGraphicsItem_mapRectToScene>(&mut self, value: T) -> i32 {
    value.mapRectToScene(self);
    return 1;
  }
}

pub trait QGraphicsItem_mapRectToScene {
  fn mapRectToScene(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QRectF QGraphicsItem::mapRectToScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectToScene for (&'a  QRectF) {
  fn mapRectToScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14mapRectToSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem14mapRectToSceneERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromItem<T: QGraphicsItem_mapRectFromItem>(&mut self, value: T) -> i32 {
    value.mapRectFromItem(self);
    return 1;
  }
}

pub trait QGraphicsItem_mapRectFromItem {
  fn mapRectFromItem(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromItem for (&'a  QGraphicsItem, f64, f64, f64, f64) {
  fn mapRectFromItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectFromItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    unsafe {_ZNK13QGraphicsItem15mapRectFromItemEPKS_dddd(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn scale<T: QGraphicsItem_scale>(&mut self, value: T) -> i32 {
    value.scale(self);
    return 1;
  }
}

pub trait QGraphicsItem_scale {
  fn scale(self, this: &mut QGraphicsItem) -> i32;
}

// proto: double QGraphicsItem::scale();
impl<'a> /*trait*/ QGraphicsItem_scale for () {
  fn scale(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5scaleEv()};
    unsafe {_ZNK13QGraphicsItem5scaleEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setBoundingRegionGranularity<T: QGraphicsItem_setBoundingRegionGranularity>(&mut self, value: T) -> i32 {
    value.setBoundingRegionGranularity(self);
    return 1;
  }
}

pub trait QGraphicsItem_setBoundingRegionGranularity {
  fn setBoundingRegionGranularity(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setBoundingRegionGranularity(qreal granularity);
impl<'a> /*trait*/ QGraphicsItem_setBoundingRegionGranularity for (f64) {
  fn setBoundingRegionGranularity(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem28setBoundingRegionGranularityEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QGraphicsItem28setBoundingRegionGranularityEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setAcceptDrops<T: QGraphicsItem_setAcceptDrops>(&mut self, value: T) -> i32 {
    value.setAcceptDrops(self);
    return 1;
  }
}

pub trait QGraphicsItem_setAcceptDrops {
  fn setAcceptDrops(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setAcceptDrops(bool on);
impl<'a> /*trait*/ QGraphicsItem_setAcceptDrops for (i8) {
  fn setAcceptDrops(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem14setAcceptDropsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QGraphicsItem14setAcceptDropsEb(arg0)};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapFromScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene for (f64, f64, f64, f64) {
  fn mapFromScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZNK13QGraphicsItem12mapFromSceneEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn ungrabKeyboard<T: QGraphicsItem_ungrabKeyboard>(&mut self, value: T) -> i32 {
    value.ungrabKeyboard(self);
    return 1;
  }
}

pub trait QGraphicsItem_ungrabKeyboard {
  fn ungrabKeyboard(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::ungrabKeyboard();
impl<'a> /*trait*/ QGraphicsItem_ungrabKeyboard for () {
  fn ungrabKeyboard(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem14ungrabKeyboardEv()};
    unsafe {_ZN13QGraphicsItem14ungrabKeyboardEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setEnabled<T: QGraphicsItem_setEnabled>(&mut self, value: T) -> i32 {
    value.setEnabled(self);
    return 1;
  }
}

pub trait QGraphicsItem_setEnabled {
  fn setEnabled(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setEnabled(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setEnabled for (i8) {
  fn setEnabled(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QGraphicsItem10setEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn graphicsEffect<T: QGraphicsItem_graphicsEffect>(&mut self, value: T) -> i32 {
    value.graphicsEffect(self);
    return 1;
  }
}

pub trait QGraphicsItem_graphicsEffect {
  fn graphicsEffect(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsEffect * QGraphicsItem::graphicsEffect();
impl<'a> /*trait*/ QGraphicsItem_graphicsEffect for () {
  fn graphicsEffect(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14graphicsEffectEv()};
    unsafe {_ZNK13QGraphicsItem14graphicsEffectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn acceptHoverEvents<T: QGraphicsItem_acceptHoverEvents>(&mut self, value: T) -> i32 {
    value.acceptHoverEvents(self);
    return 1;
  }
}

pub trait QGraphicsItem_acceptHoverEvents {
  fn acceptHoverEvents(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::acceptHoverEvents();
impl<'a> /*trait*/ QGraphicsItem_acceptHoverEvents for () {
  fn acceptHoverEvents(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17acceptHoverEventsEv()};
    unsafe {_ZNK13QGraphicsItem17acceptHoverEventsEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn topLevelWidget<T: QGraphicsItem_topLevelWidget>(&mut self, value: T) -> i32 {
    value.topLevelWidget(self);
    return 1;
  }
}

pub trait QGraphicsItem_topLevelWidget {
  fn topLevelWidget(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsWidget * QGraphicsItem::topLevelWidget();
impl<'a> /*trait*/ QGraphicsItem_topLevelWidget for () {
  fn topLevelWidget(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14topLevelWidgetEv()};
    unsafe {_ZNK13QGraphicsItem14topLevelWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn transformations<T: QGraphicsItem_transformations>(&mut self, value: T) -> i32 {
    value.transformations(self);
    return 1;
  }
}

pub trait QGraphicsItem_transformations {
  fn transformations(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QList<QGraphicsTransform *> QGraphicsItem::transformations();
impl<'a> /*trait*/ QGraphicsItem_transformations for () {
  fn transformations(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15transformationsEv()};
    unsafe {_ZNK13QGraphicsItem15transformationsEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapToScene<T: QGraphicsItem_mapToScene>(&mut self, value: T) -> i32 {
    value.mapToScene(self);
    return 1;
  }
}

pub trait QGraphicsItem_mapToScene {
  fn mapToScene(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QPolygonF QGraphicsItem::mapToScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapToScene for (f64, f64, f64, f64) {
  fn mapToScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZNK13QGraphicsItem10mapToSceneEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: QPointF QGraphicsItem::mapToScene(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapToScene for (f64, f64) {
  fn mapToScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZNK13QGraphicsItem10mapToSceneEdd(arg0, arg1)};
    return 1;
  }
}

// proto: QRectF QGraphicsItem::mapRectFromScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromScene for (f64, f64, f64, f64) {
  fn mapRectFromScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem16mapRectFromSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZNK13QGraphicsItem16mapRectFromSceneEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn advance<T: QGraphicsItem_advance>(&mut self, value: T) -> i32 {
    value.advance(self);
    return 1;
  }
}

pub trait QGraphicsItem_advance {
  fn advance(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::advance(int phase);
impl<'a> /*trait*/ QGraphicsItem_advance for (i32) {
  fn advance(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem7advanceEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QGraphicsItem7advanceEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn sceneMatrix<T: QGraphicsItem_sceneMatrix>(&mut self, value: T) -> i32 {
    value.sceneMatrix(self);
    return 1;
  }
}

pub trait QGraphicsItem_sceneMatrix {
  fn sceneMatrix(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QMatrix QGraphicsItem::sceneMatrix();
impl<'a> /*trait*/ QGraphicsItem_sceneMatrix for () {
  fn sceneMatrix(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11sceneMatrixEv()};
    unsafe {_ZNK13QGraphicsItem11sceneMatrixEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setFiltersChildEvents<T: QGraphicsItem_setFiltersChildEvents>(&mut self, value: T) -> i32 {
    value.setFiltersChildEvents(self);
    return 1;
  }
}

pub trait QGraphicsItem_setFiltersChildEvents {
  fn setFiltersChildEvents(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setFiltersChildEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setFiltersChildEvents for (i8) {
  fn setFiltersChildEvents(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem21setFiltersChildEventsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QGraphicsItem21setFiltersChildEventsEb(arg0)};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapToScene(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapToScene for (&'a  QPolygonF) {
  fn mapToScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneERK9QPolygonF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem10mapToSceneERK9QPolygonF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn itemTransform<T: QGraphicsItem_itemTransform>(&mut self, value: T) -> i32 {
    value.itemTransform(self);
    return 1;
  }
}

pub trait QGraphicsItem_itemTransform {
  fn itemTransform(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QTransform QGraphicsItem::itemTransform(const QGraphicsItem * other, bool * ok);
impl<'a> /*trait*/ QGraphicsItem_itemTransform for (&'a  QGraphicsItem, &'a mut i8) {
  fn itemTransform(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13itemTransformEPKS_Pb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as *mut int8_t;
    unsafe {_ZNK13QGraphicsItem13itemTransformEPKS_Pb(arg0, arg1)};
    return 1;
  }
}

// proto: void QGraphicsItem::setTransformOriginPoint(qreal ax, qreal ay);
impl<'a> /*trait*/ QGraphicsItem_setTransformOriginPoint for (f64, f64) {
  fn setTransformOriginPoint(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem23setTransformOriginPointEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN13QGraphicsItem23setTransformOriginPointEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn moveBy<T: QGraphicsItem_moveBy>(&mut self, value: T) -> i32 {
    value.moveBy(self);
    return 1;
  }
}

pub trait QGraphicsItem_moveBy {
  fn moveBy(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::moveBy(qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsItem_moveBy for (f64, f64) {
  fn moveBy(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6moveByEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN13QGraphicsItem6moveByEdd(arg0, arg1)};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapFromScene(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene for (&'a  QPolygonF) {
  fn mapFromScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneERK9QPolygonF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem12mapFromSceneERK9QPolygonF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn group<T: QGraphicsItem_group>(&mut self, value: T) -> i32 {
    value.group(self);
    return 1;
  }
}

pub trait QGraphicsItem_group {
  fn group(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsItemGroup * QGraphicsItem::group();
impl<'a> /*trait*/ QGraphicsItem_group for () {
  fn group(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5groupEv()};
    unsafe {_ZNK13QGraphicsItem5groupEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn shape<T: QGraphicsItem_shape>(&mut self, value: T) -> i32 {
    value.shape(self);
    return 1;
  }
}

pub trait QGraphicsItem_shape {
  fn shape(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QPainterPath QGraphicsItem::shape();
impl<'a> /*trait*/ QGraphicsItem_shape for () {
  fn shape(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5shapeEv()};
    unsafe {_ZNK13QGraphicsItem5shapeEv()};
    return 1;
  }
}

// proto: QPointF QGraphicsItem::mapFromScene(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene for (f64, f64) {
  fn mapFromScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZNK13QGraphicsItem12mapFromSceneEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn scroll<T: QGraphicsItem_scroll>(&mut self, value: T) -> i32 {
    value.scroll(self);
    return 1;
  }
}

pub trait QGraphicsItem_scroll {
  fn scroll(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::scroll(qreal dx, qreal dy, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_scroll for (f64, f64, &'a  QRectF) {
  fn scroll(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6scrollEddRK6QRectF()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsItem6scrollEddRK6QRectF(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isObscuredBy<T: QGraphicsItem_isObscuredBy>(&mut self, value: T) -> i32 {
    value.isObscuredBy(self);
    return 1;
  }
}

pub trait QGraphicsItem_isObscuredBy {
  fn isObscuredBy(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItem_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12isObscuredByEPKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem12isObscuredByEPKS_(arg0)};
    return 1;
  }
}

// proto: QPointF QGraphicsItem::mapFromParent(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent for (&'a  QPointF) {
  fn mapFromParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem13mapFromParentERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setData<T: QGraphicsItem_setData>(&mut self, value: T) -> i32 {
    value.setData(self);
    return 1;
  }
}

pub trait QGraphicsItem_setData {
  fn setData(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setData(int key, const QVariant & value);
impl<'a> /*trait*/ QGraphicsItem_setData for (i32, &'a  QVariant) {
  fn setData(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem7setDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsItem7setDataEiRK8QVariant(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn commonAncestorItem<T: QGraphicsItem_commonAncestorItem>(&mut self, value: T) -> i32 {
    value.commonAncestorItem(self);
    return 1;
  }
}

pub trait QGraphicsItem_commonAncestorItem {
  fn commonAncestorItem(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsItem * QGraphicsItem::commonAncestorItem(const QGraphicsItem * other);
impl<'a> /*trait*/ QGraphicsItem_commonAncestorItem for (&'a  QGraphicsItem) {
  fn commonAncestorItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem18commonAncestorItemEPKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem18commonAncestorItemEPKS_(arg0)};
    return 1;
  }
}

// proto: QPainterPath QGraphicsItem::mapFromScene(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene for (&'a  QPainterPath) {
  fn mapFromScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneERK12QPainterPath()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem12mapFromSceneERK12QPainterPath(arg0)};
    return 1;
  }
}

// proto: QPainterPath QGraphicsItem::mapToScene(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapToScene for (&'a  QPainterPath) {
  fn mapToScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneERK12QPainterPath()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem10mapToSceneERK12QPainterPath(arg0)};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapToParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapToParent for (f64, f64, f64, f64) {
  fn mapToParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZNK13QGraphicsItem11mapToParentEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setGroup<T: QGraphicsItem_setGroup>(&mut self, value: T) -> i32 {
    value.setGroup(self);
    return 1;
  }
}

pub trait QGraphicsItem_setGroup {
  fn setGroup(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setGroup(QGraphicsItemGroup * group);
impl<'a> /*trait*/ QGraphicsItem_setGroup for (&'a mut QGraphicsItemGroup) {
  fn setGroup(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem8setGroupEP18QGraphicsItemGroup()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsItem8setGroupEP18QGraphicsItemGroup(arg0)};
    return 1;
  }
}

// proto: QRectF QGraphicsItem::mapRectFromParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromParent for (f64, f64, f64, f64) {
  fn mapRectFromParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17mapRectFromParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZNK13QGraphicsItem17mapRectFromParentEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn show<T: QGraphicsItem_show>(&mut self, value: T) -> i32 {
    value.show(self);
    return 1;
  }
}

pub trait QGraphicsItem_show {
  fn show(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::show();
impl<'a> /*trait*/ QGraphicsItem_show for () {
  fn show(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4showEv()};
    unsafe {_ZN13QGraphicsItem4showEv()};
    return 1;
  }
}

// proto: QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromItem for (&'a  QGraphicsItem, &'a  QRectF) {
  fn mapRectFromItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectFromItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem15mapRectFromItemEPKS_RK6QRectF(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn y<T: QGraphicsItem_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QGraphicsItem_y {
  fn y(self, this: &mut QGraphicsItem) -> i32;
}

// proto: double QGraphicsItem::y();
impl<'a> /*trait*/ QGraphicsItem_y for () {
  fn y(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem1yEv()};
    unsafe {_ZNK13QGraphicsItem1yEv()};
    return 1;
  }
}

// proto: QPointF QGraphicsItem::mapFromScene(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene for (&'a  QPointF) {
  fn mapFromScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem12mapFromSceneERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn hasFocus<T: QGraphicsItem_hasFocus>(&mut self, value: T) -> i32 {
    value.hasFocus(self);
    return 1;
  }
}

pub trait QGraphicsItem_hasFocus {
  fn hasFocus(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::hasFocus();
impl<'a> /*trait*/ QGraphicsItem_hasFocus for () {
  fn hasFocus(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8hasFocusEv()};
    unsafe {_ZNK13QGraphicsItem8hasFocusEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn clipPath<T: QGraphicsItem_clipPath>(&mut self, value: T) -> i32 {
    value.clipPath(self);
    return 1;
  }
}

pub trait QGraphicsItem_clipPath {
  fn clipPath(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QPainterPath QGraphicsItem::clipPath();
impl<'a> /*trait*/ QGraphicsItem_clipPath for () {
  fn clipPath(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8clipPathEv()};
    unsafe {_ZNK13QGraphicsItem8clipPathEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setPos<T: QGraphicsItem_setPos>(&mut self, value: T) -> i32 {
    value.setPos(self);
    return 1;
  }
}

pub trait QGraphicsItem_setPos {
  fn setPos(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setPos(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_setPos for (f64, f64) {
  fn setPos(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6setPosEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN13QGraphicsItem6setPosEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isEnabled<T: QGraphicsItem_isEnabled>(&mut self, value: T) -> i32 {
    value.isEnabled(self);
    return 1;
  }
}

pub trait QGraphicsItem_isEnabled {
  fn isEnabled(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isEnabled();
impl<'a> /*trait*/ QGraphicsItem_isEnabled for () {
  fn isEnabled(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9isEnabledEv()};
    unsafe {_ZNK13QGraphicsItem9isEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn contains<T: QGraphicsItem_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QGraphicsItem_contains {
  fn contains(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_contains for (&'a  QPointF) {
  fn contains(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem8containsERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isPanel<T: QGraphicsItem_isPanel>(&mut self, value: T) -> i32 {
    value.isPanel(self);
    return 1;
  }
}

pub trait QGraphicsItem_isPanel {
  fn isPanel(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isPanel();
impl<'a> /*trait*/ QGraphicsItem_isPanel for () {
  fn isPanel(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem7isPanelEv()};
    unsafe {_ZNK13QGraphicsItem7isPanelEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn filtersChildEvents<T: QGraphicsItem_filtersChildEvents>(&mut self, value: T) -> i32 {
    value.filtersChildEvents(self);
    return 1;
  }
}

pub trait QGraphicsItem_filtersChildEvents {
  fn filtersChildEvents(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::filtersChildEvents();
impl<'a> /*trait*/ QGraphicsItem_filtersChildEvents for () {
  fn filtersChildEvents(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem18filtersChildEventsEv()};
    unsafe {_ZNK13QGraphicsItem18filtersChildEventsEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn grabKeyboard<T: QGraphicsItem_grabKeyboard>(&mut self, value: T) -> i32 {
    value.grabKeyboard(self);
    return 1;
  }
}

pub trait QGraphicsItem_grabKeyboard {
  fn grabKeyboard(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::grabKeyboard();
impl<'a> /*trait*/ QGraphicsItem_grabKeyboard for () {
  fn grabKeyboard(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem12grabKeyboardEv()};
    unsafe {_ZN13QGraphicsItem12grabKeyboardEv()};
    return 1;
  }
}

// proto: QPainterPath QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem for (&'a  QGraphicsItem, &'a  QPainterPath) {
  fn mapFromItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_RK12QPainterPath()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem11mapFromItemEPKS_RK12QPainterPath(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setActive<T: QGraphicsItem_setActive>(&mut self, value: T) -> i32 {
    value.setActive(self);
    return 1;
  }
}

pub trait QGraphicsItem_setActive {
  fn setActive(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setActive(bool active);
impl<'a> /*trait*/ QGraphicsItem_setActive for (i8) {
  fn setActive(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setActiveEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QGraphicsItem9setActiveEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn toGraphicsObject<T: QGraphicsItem_toGraphicsObject>(&mut self, value: T) -> i32 {
    value.toGraphicsObject(self);
    return 1;
  }
}

pub trait QGraphicsItem_toGraphicsObject {
  fn toGraphicsObject(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsObject * QGraphicsItem::toGraphicsObject();
impl<'a> /*trait*/ QGraphicsItem_toGraphicsObject for () {
  fn toGraphicsObject(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem16toGraphicsObjectEv()};
    unsafe {_ZN13QGraphicsItem16toGraphicsObjectEv()};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem for (&'a  QGraphicsItem, &'a  QPolygonF) {
  fn mapFromItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_RK9QPolygonF()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem11mapFromItemEPKS_RK9QPolygonF(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setHandlesChildEvents<T: QGraphicsItem_setHandlesChildEvents>(&mut self, value: T) -> i32 {
    value.setHandlesChildEvents(self);
    return 1;
  }
}

pub trait QGraphicsItem_setHandlesChildEvents {
  fn setHandlesChildEvents(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setHandlesChildEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setHandlesChildEvents for (i8) {
  fn setHandlesChildEvents(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem21setHandlesChildEventsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QGraphicsItem21setHandlesChildEventsEb(arg0)};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapFromParent(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent for (&'a  QPolygonF) {
  fn mapFromParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentERK9QPolygonF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem13mapFromParentERK9QPolygonF(arg0)};
    return 1;
  }
}

// proto: QPointF QGraphicsItem::mapToParent(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapToParent for (f64, f64) {
  fn mapToParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZNK13QGraphicsItem11mapToParentEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setMatrix<T: QGraphicsItem_setMatrix>(&mut self, value: T) -> i32 {
    value.setMatrix(self);
    return 1;
  }
}

pub trait QGraphicsItem_setMatrix {
  fn setMatrix(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setMatrix(const QMatrix & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsItem_setMatrix for (&'a  QMatrix, i8) {
  fn setMatrix(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setMatrixERK7QMatrixb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN13QGraphicsItem9setMatrixERK7QMatrixb(arg0, arg1)};
    return 1;
  }
}

// proto: void QGraphicsItem::update(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_update for (&'a  QRectF) {
  fn update(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6updateERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsItem6updateERK6QRectF(arg0)};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapToItem for (&'a  QGraphicsItem, &'a  QPolygonF) {
  fn mapToItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_RK9QPolygonF()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem9mapToItemEPKS_RK9QPolygonF(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn transform<T: QGraphicsItem_transform>(&mut self, value: T) -> i32 {
    value.transform(self);
    return 1;
  }
}

pub trait QGraphicsItem_transform {
  fn transform(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QTransform QGraphicsItem::transform();
impl<'a> /*trait*/ QGraphicsItem_transform for () {
  fn transform(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9transformEv()};
    unsafe {_ZNK13QGraphicsItem9transformEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn data<T: QGraphicsItem_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QGraphicsItem_data {
  fn data(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QVariant QGraphicsItem::data(int key);
impl<'a> /*trait*/ QGraphicsItem_data for (i32) {
  fn data(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem4dataEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK13QGraphicsItem4dataEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn hide<T: QGraphicsItem_hide>(&mut self, value: T) -> i32 {
    value.hide(self);
    return 1;
  }
}

pub trait QGraphicsItem_hide {
  fn hide(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::hide();
impl<'a> /*trait*/ QGraphicsItem_hide for () {
  fn hide(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4hideEv()};
    unsafe {_ZN13QGraphicsItem4hideEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isUnderMouse<T: QGraphicsItem_isUnderMouse>(&mut self, value: T) -> i32 {
    value.isUnderMouse(self);
    return 1;
  }
}

pub trait QGraphicsItem_isUnderMouse {
  fn isUnderMouse(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isUnderMouse();
impl<'a> /*trait*/ QGraphicsItem_isUnderMouse for () {
  fn isUnderMouse(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12isUnderMouseEv()};
    unsafe {_ZNK13QGraphicsItem12isUnderMouseEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setAcceptTouchEvents<T: QGraphicsItem_setAcceptTouchEvents>(&mut self, value: T) -> i32 {
    value.setAcceptTouchEvents(self);
    return 1;
  }
}

pub trait QGraphicsItem_setAcceptTouchEvents {
  fn setAcceptTouchEvents(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setAcceptTouchEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setAcceptTouchEvents for (i8) {
  fn setAcceptTouchEvents(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem20setAcceptTouchEventsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QGraphicsItem20setAcceptTouchEventsEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setAcceptHoverEvents<T: QGraphicsItem_setAcceptHoverEvents>(&mut self, value: T) -> i32 {
    value.setAcceptHoverEvents(self);
    return 1;
  }
}

pub trait QGraphicsItem_setAcceptHoverEvents {
  fn setAcceptHoverEvents(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setAcceptHoverEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setAcceptHoverEvents for (i8) {
  fn setAcceptHoverEvents(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem20setAcceptHoverEventsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QGraphicsItem20setAcceptHoverEventsEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn childItems<T: QGraphicsItem_childItems>(&mut self, value: T) -> i32 {
    value.childItems(self);
    return 1;
  }
}

pub trait QGraphicsItem_childItems {
  fn childItems(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QList<QGraphicsItem *> QGraphicsItem::childItems();
impl<'a> /*trait*/ QGraphicsItem_childItems for () {
  fn childItems(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10childItemsEv()};
    unsafe {_ZNK13QGraphicsItem10childItemsEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isAncestorOf<T: QGraphicsItem_isAncestorOf>(&mut self, value: T) -> i32 {
    value.isAncestorOf(self);
    return 1;
  }
}

pub trait QGraphicsItem_isAncestorOf {
  fn isAncestorOf(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isAncestorOf(const QGraphicsItem * child);
impl<'a> /*trait*/ QGraphicsItem_isAncestorOf for (&'a  QGraphicsItem) {
  fn isAncestorOf(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12isAncestorOfEPKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem12isAncestorOfEPKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn opacity<T: QGraphicsItem_opacity>(&mut self, value: T) -> i32 {
    value.opacity(self);
    return 1;
  }
}

pub trait QGraphicsItem_opacity {
  fn opacity(self, this: &mut QGraphicsItem) -> i32;
}

// proto: double QGraphicsItem::opacity();
impl<'a> /*trait*/ QGraphicsItem_opacity for () {
  fn opacity(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem7opacityEv()};
    unsafe {_ZNK13QGraphicsItem7opacityEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isVisibleTo<T: QGraphicsItem_isVisibleTo>(&mut self, value: T) -> i32 {
    value.isVisibleTo(self);
    return 1;
  }
}

pub trait QGraphicsItem_isVisibleTo {
  fn isVisibleTo(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isVisibleTo(const QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsItem_isVisibleTo for (&'a  QGraphicsItem) {
  fn isVisibleTo(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11isVisibleToEPKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem11isVisibleToEPKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn toolTip<T: QGraphicsItem_toolTip>(&mut self, value: T) -> i32 {
    value.toolTip(self);
    return 1;
  }
}

pub trait QGraphicsItem_toolTip {
  fn toolTip(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QString QGraphicsItem::toolTip();
impl<'a> /*trait*/ QGraphicsItem_toolTip for () {
  fn toolTip(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem7toolTipEv()};
    unsafe {_ZNK13QGraphicsItem7toolTipEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn cursor<T: QGraphicsItem_cursor>(&mut self, value: T) -> i32 {
    value.cursor(self);
    return 1;
  }
}

pub trait QGraphicsItem_cursor {
  fn cursor(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QCursor QGraphicsItem::cursor();
impl<'a> /*trait*/ QGraphicsItem_cursor for () {
  fn cursor(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6cursorEv()};
    unsafe {_ZNK13QGraphicsItem6cursorEv()};
    return 1;
  }
}

// proto: QPointF QGraphicsItem::mapToScene(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapToScene for (&'a  QPointF) {
  fn mapToScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem10mapToSceneERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn zValue<T: QGraphicsItem_zValue>(&mut self, value: T) -> i32 {
    value.zValue(self);
    return 1;
  }
}

pub trait QGraphicsItem_zValue {
  fn zValue(self, this: &mut QGraphicsItem) -> i32;
}

// proto: double QGraphicsItem::zValue();
impl<'a> /*trait*/ QGraphicsItem_zValue for () {
  fn zValue(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6zValueEv()};
    unsafe {_ZNK13QGraphicsItem6zValueEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn matrix<T: QGraphicsItem_matrix>(&mut self, value: T) -> i32 {
    value.matrix(self);
    return 1;
  }
}

pub trait QGraphicsItem_matrix {
  fn matrix(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QMatrix QGraphicsItem::matrix();
impl<'a> /*trait*/ QGraphicsItem_matrix for () {
  fn matrix(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6matrixEv()};
    unsafe {_ZNK13QGraphicsItem6matrixEv()};
    return 1;
  }
}

// proto: QRectF QGraphicsItem::mapRectToScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectToScene for (f64, f64, f64, f64) {
  fn mapRectToScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14mapRectToSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZNK13QGraphicsItem14mapRectToSceneEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QGraphicsItem::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsItem_setPos for (&'a  QPointF) {
  fn setPos(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsItem6setPosERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn panel<T: QGraphicsItem_panel>(&mut self, value: T) -> i32 {
    value.panel(self);
    return 1;
  }
}

pub trait QGraphicsItem_panel {
  fn panel(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsItem * QGraphicsItem::panel();
impl<'a> /*trait*/ QGraphicsItem_panel for () {
  fn panel(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5panelEv()};
    unsafe {_ZNK13QGraphicsItem5panelEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isClipped<T: QGraphicsItem_isClipped>(&mut self, value: T) -> i32 {
    value.isClipped(self);
    return 1;
  }
}

pub trait QGraphicsItem_isClipped {
  fn isClipped(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isClipped();
impl<'a> /*trait*/ QGraphicsItem_isClipped for () {
  fn isClipped(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9isClippedEv()};
    unsafe {_ZNK13QGraphicsItem9isClippedEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn topLevelItem<T: QGraphicsItem_topLevelItem>(&mut self, value: T) -> i32 {
    value.topLevelItem(self);
    return 1;
  }
}

pub trait QGraphicsItem_topLevelItem {
  fn topLevelItem(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QGraphicsItem * QGraphicsItem::topLevelItem();
impl<'a> /*trait*/ QGraphicsItem_topLevelItem for () {
  fn topLevelItem(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12topLevelItemEv()};
    unsafe {_ZNK13QGraphicsItem12topLevelItemEv()};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapToScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapToScene for (&'a  QRectF) {
  fn mapToScene(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem10mapToSceneERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setScale<T: QGraphicsItem_setScale>(&mut self, value: T) -> i32 {
    value.setScale(self);
    return 1;
  }
}

pub trait QGraphicsItem_setScale {
  fn setScale(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setScale(qreal scale);
impl<'a> /*trait*/ QGraphicsItem_setScale for (f64) {
  fn setScale(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem8setScaleEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QGraphicsItem8setScaleEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setCursor<T: QGraphicsItem_setCursor>(&mut self, value: T) -> i32 {
    value.setCursor(self);
    return 1;
  }
}

pub trait QGraphicsItem_setCursor {
  fn setCursor(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setCursor(const QCursor & cursor);
impl<'a> /*trait*/ QGraphicsItem_setCursor for (&'a  QCursor) {
  fn setCursor(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsItem9setCursorERK7QCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isVisible<T: QGraphicsItem_isVisible>(&mut self, value: T) -> i32 {
    value.isVisible(self);
    return 1;
  }
}

pub trait QGraphicsItem_isVisible {
  fn isVisible(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isVisible();
impl<'a> /*trait*/ QGraphicsItem_isVisible for () {
  fn isVisible(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9isVisibleEv()};
    unsafe {_ZNK13QGraphicsItem9isVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn pos<T: QGraphicsItem_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QGraphicsItem_pos {
  fn pos(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QPointF QGraphicsItem::pos();
impl<'a> /*trait*/ QGraphicsItem_pos for () {
  fn pos(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem3posEv()};
    unsafe {_ZNK13QGraphicsItem3posEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isBlockedByModalPanel<T: QGraphicsItem_isBlockedByModalPanel>(&mut self, value: T) -> i32 {
    value.isBlockedByModalPanel(self);
    return 1;
  }
}

pub trait QGraphicsItem_isBlockedByModalPanel {
  fn isBlockedByModalPanel(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::isBlockedByModalPanel(QGraphicsItem ** blockingPanel);
impl<'a> /*trait*/ QGraphicsItem_isBlockedByModalPanel for (&'a mut QGraphicsItem) {
  fn isBlockedByModalPanel(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem21isBlockedByModalPanelEPPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK13QGraphicsItem21isBlockedByModalPanelEPPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn effectiveOpacity<T: QGraphicsItem_effectiveOpacity>(&mut self, value: T) -> i32 {
    value.effectiveOpacity(self);
    return 1;
  }
}

pub trait QGraphicsItem_effectiveOpacity {
  fn effectiveOpacity(self, this: &mut QGraphicsItem) -> i32;
}

// proto: double QGraphicsItem::effectiveOpacity();
impl<'a> /*trait*/ QGraphicsItem_effectiveOpacity for () {
  fn effectiveOpacity(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem16effectiveOpacityEv()};
    unsafe {_ZNK13QGraphicsItem16effectiveOpacityEv()};
    return 1;
  }
}

// proto: void QGraphicsItem::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsItem_ensureVisible for (&'a  QRectF, i32, i32) {
  fn ensureVisible(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13ensureVisibleERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN13QGraphicsItem13ensureVisibleERK6QRectFii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn boundingRegionGranularity<T: QGraphicsItem_boundingRegionGranularity>(&mut self, value: T) -> i32 {
    value.boundingRegionGranularity(self);
    return 1;
  }
}

pub trait QGraphicsItem_boundingRegionGranularity {
  fn boundingRegionGranularity(self, this: &mut QGraphicsItem) -> i32;
}

// proto: double QGraphicsItem::boundingRegionGranularity();
impl<'a> /*trait*/ QGraphicsItem_boundingRegionGranularity for () {
  fn boundingRegionGranularity(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem25boundingRegionGranularityEv()};
    unsafe {_ZNK13QGraphicsItem25boundingRegionGranularityEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn x<T: QGraphicsItem_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QGraphicsItem_x {
  fn x(self, this: &mut QGraphicsItem) -> i32;
}

// proto: double QGraphicsItem::x();
impl<'a> /*trait*/ QGraphicsItem_x for () {
  fn x(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem1xEv()};
    unsafe {_ZNK13QGraphicsItem1xEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn grabMouse<T: QGraphicsItem_grabMouse>(&mut self, value: T) -> i32 {
    value.grabMouse(self);
    return 1;
  }
}

pub trait QGraphicsItem_grabMouse {
  fn grabMouse(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::grabMouse();
impl<'a> /*trait*/ QGraphicsItem_grabMouse for () {
  fn grabMouse(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9grabMouseEv()};
    unsafe {_ZN13QGraphicsItem9grabMouseEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setVisible<T: QGraphicsItem_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QGraphicsItem_setVisible {
  fn setVisible(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setVisible(bool visible);
impl<'a> /*trait*/ QGraphicsItem_setVisible for (i8) {
  fn setVisible(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QGraphicsItem10setVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setRotation<T: QGraphicsItem_setRotation>(&mut self, value: T) -> i32 {
    value.setRotation(self);
    return 1;
  }
}

pub trait QGraphicsItem_setRotation {
  fn setRotation(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setRotation(qreal angle);
impl<'a> /*trait*/ QGraphicsItem_setRotation for (f64) {
  fn setRotation(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11setRotationEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QGraphicsItem11setRotationEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn deviceTransform<T: QGraphicsItem_deviceTransform>(&mut self, value: T) -> i32 {
    value.deviceTransform(self);
    return 1;
  }
}

pub trait QGraphicsItem_deviceTransform {
  fn deviceTransform(self, this: &mut QGraphicsItem) -> i32;
}

// proto: QTransform QGraphicsItem::deviceTransform(const QTransform & viewportTransform);
impl<'a> /*trait*/ QGraphicsItem_deviceTransform for (&'a  QTransform) {
  fn deviceTransform(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15deviceTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem15deviceTransformERK10QTransform(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn acceptTouchEvents<T: QGraphicsItem_acceptTouchEvents>(&mut self, value: T) -> i32 {
    value.acceptTouchEvents(self);
    return 1;
  }
}

pub trait QGraphicsItem_acceptTouchEvents {
  fn acceptTouchEvents(self, this: &mut QGraphicsItem) -> i32;
}

// proto: bool QGraphicsItem::acceptTouchEvents();
impl<'a> /*trait*/ QGraphicsItem_acceptTouchEvents for () {
  fn acceptTouchEvents(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17acceptTouchEventsEv()};
    unsafe {_ZNK13QGraphicsItem17acceptTouchEventsEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setTransform<T: QGraphicsItem_setTransform>(&mut self, value: T) -> i32 {
    value.setTransform(self);
    return 1;
  }
}

pub trait QGraphicsItem_setTransform {
  fn setTransform(self, this: &mut QGraphicsItem) -> i32;
}

// proto: void QGraphicsItem::setTransform(const QTransform & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsItem_setTransform for (&'a  QTransform, i8) {
  fn setTransform(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem12setTransformERK10QTransformb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN13QGraphicsItem12setTransformERK10QTransformb(arg0, arg1)};
    return 1;
  }
}

// proto: QPolygonF QGraphicsItem::mapToParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapToParent for (&'a  QRectF) {
  fn mapToParent(self, this: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsItem11mapToParentERK6QRectF(arg0)};
    return 1;
  }
}

