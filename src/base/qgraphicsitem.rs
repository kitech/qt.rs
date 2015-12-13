// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qtransform::QTransform;
use super::qregion::QRegion;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;
use super::qrectf::QRectF;
use super::qgraphicseffect::QGraphicsEffect;
use super::qstring::QString;
use super::qpainterpath::QPainterPath;
use super::qmatrix::QMatrix;
use super::qvariant::QVariant;
use super::qgraphicsitemgroup::QGraphicsItemGroup;
use super::qcursor::QCursor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsItem::NewQGraphicsItem(const QGraphicsItem & );
  fn _ZN13QGraphicsItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QGraphicsItem * QGraphicsItem::focusItem();
  fn _ZNK13QGraphicsItem9focusItemEv(qthis: *mut c_void) ;
  // proto:  QGraphicsObject * QGraphicsItem::parentObject();
  fn _ZNK13QGraphicsItem12parentObjectEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItem::setTransformOriginPoint(const QPointF & origin);
  fn _ZN13QGraphicsItem23setTransformOriginPointERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsItem::ungrabMouse();
  fn _ZN13QGraphicsItem11ungrabMouseEv(qthis: *mut c_void) ;
  // proto:  int QGraphicsItem::type_();
  fn _ZNK13QGraphicsItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QGraphicsItem::isSelected();
  fn _ZNK13QGraphicsItem10isSelectedEv(qthis: *mut c_void) -> int8_t;
  // proto:  QGraphicsWidget * QGraphicsItem::parentWidget();
  fn _ZNK13QGraphicsItem12parentWidgetEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItem::resetTransform();
  fn _ZN13QGraphicsItem14resetTransformEv(qthis: *mut c_void) ;
  // proto:  QRegion QGraphicsItem::boundingRegion(const QTransform & itemToDeviceTransform);
  fn _ZNK13QGraphicsItem14boundingRegionERK10QTransform(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN13QGraphicsItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  bool QGraphicsItem::isActive();
  fn _ZNK13QGraphicsItem8isActiveEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsItem::NewQGraphicsItem(QGraphicsItem * parent);
  fn _ZN13QGraphicsItemC1EPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QGraphicsItem::isWidget();
  fn _ZNK13QGraphicsItem8isWidgetEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsItem::setParentItem(QGraphicsItem * parent);
  fn _ZN13QGraphicsItem13setParentItemEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QGraphicsWidget * QGraphicsItem::window();
  fn _ZNK13QGraphicsItem6windowEv(qthis: *mut c_void) ;
  // proto:  QPointF QGraphicsItem::scenePos();
  fn _ZNK13QGraphicsItem8scenePosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::handlesChildEvents();
  fn _ZNK13QGraphicsItem18handlesChildEventsEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsItem::setOpacity(qreal opacity);
  fn _ZN13QGraphicsItem10setOpacityEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QTransform QGraphicsItem::sceneTransform();
  fn _ZNK13QGraphicsItem14sceneTransformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setZValue(qreal z);
  fn _ZN13QGraphicsItem9setZValueEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  bool QGraphicsItem::isObscured(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem10isObscuredEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> int8_t;
  // proto:  void QGraphicsItem::installSceneEventFilter(QGraphicsItem * filterItem);
  fn _ZN13QGraphicsItem23installSceneEventFilterEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsItem::setY(qreal y);
  fn _ZN13QGraphicsItem4setYEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem13mapRectToItemEPKS_dddd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsItem::parentItem();
  fn _ZNK13QGraphicsItem10parentItemEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItem::clearFocus();
  fn _ZN13QGraphicsItem10clearFocusEv(qthis: *mut c_void) ;
  // proto:  bool QGraphicsItem::isWindow();
  fn _ZNK13QGraphicsItem8isWindowEv(qthis: *mut c_void) -> int8_t;
  // proto:  QPointF QGraphicsItem::transformOriginPoint();
  fn _ZNK13QGraphicsItem20transformOriginPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::boundingRect();
  fn _ZNK13QGraphicsItem12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::childrenBoundingRect();
  fn _ZNK13QGraphicsItem20childrenBoundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::isObscured(const QRectF & rect);
  fn _ZNK13QGraphicsItem10isObscuredERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QGraphicsItem::hasCursor();
  fn _ZNK13QGraphicsItem9hasCursorEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsItem::setGraphicsEffect(QGraphicsEffect * effect);
  fn _ZN13QGraphicsItem17setGraphicsEffectEP15QGraphicsEffect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsItem::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
  fn _ZN13QGraphicsItem13ensureVisibleEddddii(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_int, arg5: c_int) ;
  // proto:  QRectF QGraphicsItem::mapRectToParent(const QRectF & rect);
  fn _ZNK13QGraphicsItem15mapRectToParentERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setToolTip(const QString & toolTip);
  fn _ZN13QGraphicsItem10setToolTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QGraphicsItem::rotation();
  fn _ZNK13QGraphicsItem8rotationEv(qthis: *mut c_void) -> c_double;
  // proto:  QGraphicsScene * QGraphicsItem::scene();
  fn _ZNK13QGraphicsItem5sceneEv(qthis: *mut c_void) ;
  // proto:  QRectF QGraphicsItem::mapRectToParent(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem15mapRectToParentEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectFromParent(const QRectF & rect);
  fn _ZNK13QGraphicsItem17mapRectFromParentERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setFocusProxy(QGraphicsItem * item);
  fn _ZN13QGraphicsItem13setFocusProxyEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QGraphicsItem::acceptDrops();
  fn _ZNK13QGraphicsItem11acceptDropsEv(qthis: *mut c_void) -> int8_t;
  // proto:  QRectF QGraphicsItem::mapRectFromScene(const QRectF & rect);
  fn _ZNK13QGraphicsItem16mapRectFromSceneERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsItem::focusScopeItem();
  fn _ZNK13QGraphicsItem14focusScopeItemEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItem::removeSceneEventFilter(QGraphicsItem * filterItem);
  fn _ZN13QGraphicsItem22removeSceneEventFilterEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QGraphicsItem * QGraphicsItem::focusProxy();
  fn _ZNK13QGraphicsItem10focusProxyEv(qthis: *mut c_void) ;
  // proto:  QRectF QGraphicsItem::sceneBoundingRect();
  fn _ZNK13QGraphicsItem17sceneBoundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::FreeQGraphicsItem();
  fn _ZN13QGraphicsItemD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsItem::setX(qreal x);
  fn _ZN13QGraphicsItem4setXEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsItem::update(qreal x, qreal y, qreal width, qreal height);
  fn _ZN13QGraphicsItem6updateEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  void QGraphicsItem::setSelected(bool selected);
  fn _ZN13QGraphicsItem11setSelectedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, const QRectF & rect);
  fn _ZNK13QGraphicsItem13mapRectToItemEPKS_RK6QRectF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::stackBefore(const QGraphicsItem * sibling);
  fn _ZN13QGraphicsItem11stackBeforeEPKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsItem::resetMatrix();
  fn _ZN13QGraphicsItem11resetMatrixEv(qthis: *mut c_void) ;
  // proto:  QPainterPath QGraphicsItem::opaqueArea();
  fn _ZNK13QGraphicsItem10opaqueAreaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::unsetCursor();
  fn _ZN13QGraphicsItem11unsetCursorEv(qthis: *mut c_void) ;
  // proto:  QRectF QGraphicsItem::mapRectToScene(const QRectF & rect);
  fn _ZNK13QGraphicsItem14mapRectToSceneERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem15mapRectFromItemEPKS_dddd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> *mut c_void;
  // proto:  double QGraphicsItem::scale();
  fn _ZNK13QGraphicsItem5scaleEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsItem::setBoundingRegionGranularity(qreal granularity);
  fn _ZN13QGraphicsItem28setBoundingRegionGranularityEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsItem::setAcceptDrops(bool on);
  fn _ZN13QGraphicsItem14setAcceptDropsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QGraphicsItem::ungrabKeyboard();
  fn _ZN13QGraphicsItem14ungrabKeyboardEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItem::setEnabled(bool enabled);
  fn _ZN13QGraphicsItem10setEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QGraphicsEffect * QGraphicsItem::graphicsEffect();
  fn _ZNK13QGraphicsItem14graphicsEffectEv(qthis: *mut c_void) ;
  // proto:  bool QGraphicsItem::acceptHoverEvents();
  fn _ZNK13QGraphicsItem17acceptHoverEventsEv(qthis: *mut c_void) -> int8_t;
  // proto:  QGraphicsWidget * QGraphicsItem::topLevelWidget();
  fn _ZNK13QGraphicsItem14topLevelWidgetEv(qthis: *mut c_void) ;
  // proto:  QList<QGraphicsTransform *> QGraphicsItem::transformations();
  fn _ZNK13QGraphicsItem15transformationsEv(qthis: *mut c_void) ;
  // proto:  QRectF QGraphicsItem::mapRectFromScene(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem16mapRectFromSceneEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::advance(int phase);
  fn _ZN13QGraphicsItem7advanceEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QMatrix QGraphicsItem::sceneMatrix();
  fn _ZNK13QGraphicsItem11sceneMatrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setFiltersChildEvents(bool enabled);
  fn _ZN13QGraphicsItem21setFiltersChildEventsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QTransform QGraphicsItem::itemTransform(const QGraphicsItem * other, bool * ok);
  fn _ZNK13QGraphicsItem13itemTransformEPKS_Pb(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut int8_t) -> *mut c_void;
  // proto:  void QGraphicsItem::setTransformOriginPoint(qreal ax, qreal ay);
  fn _ZN13QGraphicsItem23setTransformOriginPointEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QGraphicsItem::moveBy(qreal dx, qreal dy);
  fn _ZN13QGraphicsItem6moveByEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  QGraphicsItemGroup * QGraphicsItem::group();
  fn _ZNK13QGraphicsItem5groupEv(qthis: *mut c_void) ;
  // proto:  QPainterPath QGraphicsItem::shape();
  fn _ZNK13QGraphicsItem5shapeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::scroll(qreal dx, qreal dy, const QRectF & rect);
  fn _ZN13QGraphicsItem6scrollEddRK6QRectF(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: *mut c_void) ;
  // proto:  bool QGraphicsItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK13QGraphicsItem12isObscuredByEPKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QGraphicsItem::setData(int key, const QVariant & value);
  fn _ZN13QGraphicsItem7setDataEiRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QGraphicsItem * QGraphicsItem::commonAncestorItem(const QGraphicsItem * other);
  fn _ZNK13QGraphicsItem18commonAncestorItemEPKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsItem::setGroup(QGraphicsItemGroup * group);
  fn _ZN13QGraphicsItem8setGroupEP18QGraphicsItemGroup(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRectF QGraphicsItem::mapRectFromParent(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem17mapRectFromParentEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::show();
  fn _ZN13QGraphicsItem4showEv(qthis: *mut c_void) ;
  // proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, const QRectF & rect);
  fn _ZNK13QGraphicsItem15mapRectFromItemEPKS_RK6QRectF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  double QGraphicsItem::y();
  fn _ZNK13QGraphicsItem1yEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QGraphicsItem::hasFocus();
  fn _ZNK13QGraphicsItem8hasFocusEv(qthis: *mut c_void) -> int8_t;
  // proto:  QPainterPath QGraphicsItem::clipPath();
  fn _ZNK13QGraphicsItem8clipPathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setPos(qreal x, qreal y);
  fn _ZN13QGraphicsItem6setPosEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  bool QGraphicsItem::isEnabled();
  fn _ZNK13QGraphicsItem9isEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QGraphicsItem::contains(const QPointF & point);
  fn _ZNK13QGraphicsItem8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QGraphicsItem::isPanel();
  fn _ZNK13QGraphicsItem7isPanelEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QGraphicsItem::filtersChildEvents();
  fn _ZNK13QGraphicsItem18filtersChildEventsEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsItem::grabKeyboard();
  fn _ZN13QGraphicsItem12grabKeyboardEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItem::setActive(bool active);
  fn _ZN13QGraphicsItem9setActiveEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QGraphicsObject * QGraphicsItem::toGraphicsObject();
  fn _ZN13QGraphicsItem16toGraphicsObjectEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItem::setHandlesChildEvents(bool enabled);
  fn _ZN13QGraphicsItem21setHandlesChildEventsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QGraphicsItem::setMatrix(const QMatrix & matrix, bool combine);
  fn _ZN13QGraphicsItem9setMatrixERK7QMatrixb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  void QGraphicsItem::update(const QRectF & rect);
  fn _ZN13QGraphicsItem6updateERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTransform QGraphicsItem::transform();
  fn _ZNK13QGraphicsItem9transformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QGraphicsItem::data(int key);
  fn _ZNK13QGraphicsItem4dataEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QGraphicsItem::hide();
  fn _ZN13QGraphicsItem4hideEv(qthis: *mut c_void) ;
  // proto:  bool QGraphicsItem::isUnderMouse();
  fn _ZNK13QGraphicsItem12isUnderMouseEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsItem::setAcceptTouchEvents(bool enabled);
  fn _ZN13QGraphicsItem20setAcceptTouchEventsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QGraphicsItem::setAcceptHoverEvents(bool enabled);
  fn _ZN13QGraphicsItem20setAcceptHoverEventsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QList<QGraphicsItem *> QGraphicsItem::childItems();
  fn _ZNK13QGraphicsItem10childItemsEv(qthis: *mut c_void) ;
  // proto:  bool QGraphicsItem::isAncestorOf(const QGraphicsItem * child);
  fn _ZNK13QGraphicsItem12isAncestorOfEPKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  double QGraphicsItem::opacity();
  fn _ZNK13QGraphicsItem7opacityEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QGraphicsItem::isVisibleTo(const QGraphicsItem * parent);
  fn _ZNK13QGraphicsItem11isVisibleToEPKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QString QGraphicsItem::toolTip();
  fn _ZNK13QGraphicsItem7toolTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QCursor QGraphicsItem::cursor();
  fn _ZNK13QGraphicsItem6cursorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QGraphicsItem::zValue();
  fn _ZNK13QGraphicsItem6zValueEv(qthis: *mut c_void) -> c_double;
  // proto:  QMatrix QGraphicsItem::matrix();
  fn _ZNK13QGraphicsItem6matrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectToScene(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsItem14mapRectToSceneEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::setPos(const QPointF & pos);
  fn _ZN13QGraphicsItem6setPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QGraphicsItem * QGraphicsItem::panel();
  fn _ZNK13QGraphicsItem5panelEv(qthis: *mut c_void) ;
  // proto:  bool QGraphicsItem::isClipped();
  fn _ZNK13QGraphicsItem9isClippedEv(qthis: *mut c_void) -> int8_t;
  // proto:  QGraphicsItem * QGraphicsItem::topLevelItem();
  fn _ZNK13QGraphicsItem12topLevelItemEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItem::setScale(qreal scale);
  fn _ZN13QGraphicsItem8setScaleEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsItem::setCursor(const QCursor & cursor);
  fn _ZN13QGraphicsItem9setCursorERK7QCursor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QGraphicsItem::isVisible();
  fn _ZNK13QGraphicsItem9isVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  QPointF QGraphicsItem::pos();
  fn _ZNK13QGraphicsItem3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::isBlockedByModalPanel(QGraphicsItem ** blockingPanel);
  fn _ZNK13QGraphicsItem21isBlockedByModalPanelEPPS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  double QGraphicsItem::effectiveOpacity();
  fn _ZNK13QGraphicsItem16effectiveOpacityEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsItem::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
  fn _ZN13QGraphicsItem13ensureVisibleERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  double QGraphicsItem::boundingRegionGranularity();
  fn _ZNK13QGraphicsItem25boundingRegionGranularityEv(qthis: *mut c_void) -> c_double;
  // proto:  double QGraphicsItem::x();
  fn _ZNK13QGraphicsItem1xEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsItem::grabMouse();
  fn _ZN13QGraphicsItem9grabMouseEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItem::setVisible(bool visible);
  fn _ZN13QGraphicsItem10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QGraphicsItem::setRotation(qreal angle);
  fn _ZN13QGraphicsItem11setRotationEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QTransform QGraphicsItem::deviceTransform(const QTransform & viewportTransform);
  fn _ZNK13QGraphicsItem15deviceTransformERK10QTransform(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::acceptTouchEvents();
  fn _ZNK13QGraphicsItem17acceptTouchEventsEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsItem::setTransform(const QTransform & matrix, bool combine);
  fn _ZN13QGraphicsItem12setTransformERK10QTransformb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn focusItem<T: QGraphicsItem_focusItem>(&mut self, value: T)  {
     value.focusItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_focusItem {
  fn focusItem(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsItem * QGraphicsItem::focusItem();
impl<'a> /*trait*/ QGraphicsItem_focusItem for () {
  fn focusItem(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9focusItemEv()};
     unsafe {_ZNK13QGraphicsItem9focusItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn parentObject<T: QGraphicsItem_parentObject>(&mut self, value: T)  {
     value.parentObject(self);
    // return 1;
  }
}

pub trait QGraphicsItem_parentObject {
  fn parentObject(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsObject * QGraphicsItem::parentObject();
impl<'a> /*trait*/ QGraphicsItem_parentObject for () {
  fn parentObject(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12parentObjectEv()};
     unsafe {_ZNK13QGraphicsItem12parentObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setTransformOriginPoint<T: QGraphicsItem_setTransformOriginPoint>(&mut self, value: T)  {
     value.setTransformOriginPoint(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setTransformOriginPoint {
  fn setTransformOriginPoint(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setTransformOriginPoint(const QPointF & origin);
impl<'a> /*trait*/ QGraphicsItem_setTransformOriginPoint for (&'a  QPointF) {
  fn setTransformOriginPoint(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem23setTransformOriginPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem23setTransformOriginPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn ungrabMouse<T: QGraphicsItem_ungrabMouse>(&mut self, value: T)  {
     value.ungrabMouse(self);
    // return 1;
  }
}

pub trait QGraphicsItem_ungrabMouse {
  fn ungrabMouse(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::ungrabMouse();
impl<'a> /*trait*/ QGraphicsItem_ungrabMouse for () {
  fn ungrabMouse(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11ungrabMouseEv()};
     unsafe {_ZN13QGraphicsItem11ungrabMouseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn type_<T: QGraphicsItem_type_>(&mut self, value: T) -> i32 {
    return value.type_(self);
    // return 1;
  }
}

pub trait QGraphicsItem_type_ {
  fn type_(self, rsthis: &mut QGraphicsItem) -> i32;
}

// proto:  int QGraphicsItem::type_();
impl<'a> /*trait*/ QGraphicsItem_type_ for () {
  fn type_(self, rsthis: &mut QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem4typeEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isSelected<T: QGraphicsItem_isSelected>(&mut self, value: T) -> i8 {
    return value.isSelected(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isSelected {
  fn isSelected(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isSelected();
impl<'a> /*trait*/ QGraphicsItem_isSelected for () {
  fn isSelected(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10isSelectedEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem10isSelectedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn parentWidget<T: QGraphicsItem_parentWidget>(&mut self, value: T)  {
     value.parentWidget(self);
    // return 1;
  }
}

pub trait QGraphicsItem_parentWidget {
  fn parentWidget(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsWidget * QGraphicsItem::parentWidget();
impl<'a> /*trait*/ QGraphicsItem_parentWidget for () {
  fn parentWidget(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12parentWidgetEv()};
     unsafe {_ZNK13QGraphicsItem12parentWidgetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn resetTransform<T: QGraphicsItem_resetTransform>(&mut self, value: T)  {
     value.resetTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_resetTransform {
  fn resetTransform(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::resetTransform();
impl<'a> /*trait*/ QGraphicsItem_resetTransform for () {
  fn resetTransform(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem14resetTransformEv()};
     unsafe {_ZN13QGraphicsItem14resetTransformEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn boundingRegion<T: QGraphicsItem_boundingRegion>(&mut self, value: T) -> QRegion {
    return value.boundingRegion(self);
    // return 1;
  }
}

pub trait QGraphicsItem_boundingRegion {
  fn boundingRegion(self, rsthis: &mut QGraphicsItem) -> QRegion;
}

// proto:  QRegion QGraphicsItem::boundingRegion(const QTransform & itemToDeviceTransform);
impl<'a> /*trait*/ QGraphicsItem_boundingRegion for (&'a  QTransform) {
  fn boundingRegion(self, rsthis: &mut QGraphicsItem) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14boundingRegionERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem14boundingRegionERK10QTransform(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn paint<T: QGraphicsItem_paint>(&mut self, value: T)  {
     value.paint(self);
    // return 1;
  }
}

pub trait QGraphicsItem_paint {
  fn paint(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsItem_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isActive<T: QGraphicsItem_isActive>(&mut self, value: T) -> i8 {
    return value.isActive(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isActive {
  fn isActive(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isActive();
impl<'a> /*trait*/ QGraphicsItem_isActive for () {
  fn isActive(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8isActiveEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
  pub fn isWidget<T: QGraphicsItem_isWidget>(&mut self, value: T) -> i8 {
    return value.isWidget(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isWidget {
  fn isWidget(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isWidget();
impl<'a> /*trait*/ QGraphicsItem_isWidget for () {
  fn isWidget(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8isWidgetEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8isWidgetEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setParentItem<T: QGraphicsItem_setParentItem>(&mut self, value: T)  {
     value.setParentItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setParentItem {
  fn setParentItem(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setParentItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsItem_setParentItem for (&'a mut QGraphicsItem) {
  fn setParentItem(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13setParentItemEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem13setParentItemEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn window<T: QGraphicsItem_window>(&mut self, value: T)  {
     value.window(self);
    // return 1;
  }
}

pub trait QGraphicsItem_window {
  fn window(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsWidget * QGraphicsItem::window();
impl<'a> /*trait*/ QGraphicsItem_window for () {
  fn window(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6windowEv()};
     unsafe {_ZNK13QGraphicsItem6windowEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn scenePos<T: QGraphicsItem_scenePos>(&mut self, value: T) -> QPointF {
    return value.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsItem_scenePos {
  fn scenePos(self, rsthis: &mut QGraphicsItem) -> QPointF;
}

// proto:  QPointF QGraphicsItem::scenePos();
impl<'a> /*trait*/ QGraphicsItem_scenePos for () {
  fn scenePos(self, rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8scenePosEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn handlesChildEvents<T: QGraphicsItem_handlesChildEvents>(&mut self, value: T) -> i8 {
    return value.handlesChildEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_handlesChildEvents {
  fn handlesChildEvents(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::handlesChildEvents();
impl<'a> /*trait*/ QGraphicsItem_handlesChildEvents for () {
  fn handlesChildEvents(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem18handlesChildEventsEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem18handlesChildEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setOpacity<T: QGraphicsItem_setOpacity>(&mut self, value: T)  {
     value.setOpacity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setOpacity {
  fn setOpacity(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setOpacity(qreal opacity);
impl<'a> /*trait*/ QGraphicsItem_setOpacity for (f64) {
  fn setOpacity(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn sceneTransform<T: QGraphicsItem_sceneTransform>(&mut self, value: T) -> QTransform {
    return value.sceneTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_sceneTransform {
  fn sceneTransform(self, rsthis: &mut QGraphicsItem) -> QTransform;
}

// proto:  QTransform QGraphicsItem::sceneTransform();
impl<'a> /*trait*/ QGraphicsItem_sceneTransform for () {
  fn sceneTransform(self, rsthis: &mut QGraphicsItem) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14sceneTransformEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem14sceneTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setZValue<T: QGraphicsItem_setZValue>(&mut self, value: T)  {
     value.setZValue(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setZValue {
  fn setZValue(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setZValue(qreal z);
impl<'a> /*trait*/ QGraphicsItem_setZValue for (f64) {
  fn setZValue(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setZValueEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem9setZValueEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isObscured<T: QGraphicsItem_isObscured>(&mut self, value: T) -> i8 {
    return value.isObscured(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isObscured {
  fn isObscured(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isObscured(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_isObscured for (f64, f64, f64, f64) {
  fn isObscured(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10isObscuredEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem10isObscuredEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn installSceneEventFilter<T: QGraphicsItem_installSceneEventFilter>(&mut self, value: T)  {
     value.installSceneEventFilter(self);
    // return 1;
  }
}

pub trait QGraphicsItem_installSceneEventFilter {
  fn installSceneEventFilter(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::installSceneEventFilter(QGraphicsItem * filterItem);
impl<'a> /*trait*/ QGraphicsItem_installSceneEventFilter for (&'a mut QGraphicsItem) {
  fn installSceneEventFilter(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem23installSceneEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem23installSceneEventFilterEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setY<T: QGraphicsItem_setY>(&mut self, value: T)  {
     value.setY(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setY {
  fn setY(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setY(qreal y);
impl<'a> /*trait*/ QGraphicsItem_setY for (f64) {
  fn setY(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4setYEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem4setYEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapRectToItem<T: QGraphicsItem_mapRectToItem>(&mut self, value: T) -> QRectF {
    return value.mapRectToItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectToItem {
  fn mapRectToItem(self, rsthis: &mut QGraphicsItem) -> QRectF;
}

// proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectToItem for (&'a  QGraphicsItem, f64, f64, f64, f64) {
  fn mapRectToItem(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapRectToItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem13mapRectToItemEPKS_dddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn parentItem<T: QGraphicsItem_parentItem>(&mut self, value: T)  {
     value.parentItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_parentItem {
  fn parentItem(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsItem * QGraphicsItem::parentItem();
impl<'a> /*trait*/ QGraphicsItem_parentItem for () {
  fn parentItem(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10parentItemEv()};
     unsafe {_ZNK13QGraphicsItem10parentItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn clearFocus<T: QGraphicsItem_clearFocus>(&mut self, value: T)  {
     value.clearFocus(self);
    // return 1;
  }
}

pub trait QGraphicsItem_clearFocus {
  fn clearFocus(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::clearFocus();
impl<'a> /*trait*/ QGraphicsItem_clearFocus for () {
  fn clearFocus(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10clearFocusEv()};
     unsafe {_ZN13QGraphicsItem10clearFocusEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isWindow<T: QGraphicsItem_isWindow>(&mut self, value: T) -> i8 {
    return value.isWindow(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isWindow {
  fn isWindow(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isWindow();
impl<'a> /*trait*/ QGraphicsItem_isWindow for () {
  fn isWindow(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8isWindowEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8isWindowEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn transformOriginPoint<T: QGraphicsItem_transformOriginPoint>(&mut self, value: T) -> QPointF {
    return value.transformOriginPoint(self);
    // return 1;
  }
}

pub trait QGraphicsItem_transformOriginPoint {
  fn transformOriginPoint(self, rsthis: &mut QGraphicsItem) -> QPointF;
}

// proto:  QPointF QGraphicsItem::transformOriginPoint();
impl<'a> /*trait*/ QGraphicsItem_transformOriginPoint for () {
  fn transformOriginPoint(self, rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem20transformOriginPointEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem20transformOriginPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn boundingRect<T: QGraphicsItem_boundingRect>(&mut self, value: T) -> QRectF {
    return value.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_boundingRect {
  fn boundingRect(self, rsthis: &mut QGraphicsItem) -> QRectF;
}

// proto:  QRectF QGraphicsItem::boundingRect();
impl<'a> /*trait*/ QGraphicsItem_boundingRect for () {
  fn boundingRect(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12boundingRectEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn childrenBoundingRect<T: QGraphicsItem_childrenBoundingRect>(&mut self, value: T) -> QRectF {
    return value.childrenBoundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_childrenBoundingRect {
  fn childrenBoundingRect(self, rsthis: &mut QGraphicsItem) -> QRectF;
}

// proto:  QRectF QGraphicsItem::childrenBoundingRect();
impl<'a> /*trait*/ QGraphicsItem_childrenBoundingRect for () {
  fn childrenBoundingRect(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem20childrenBoundingRectEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem20childrenBoundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QGraphicsItem::isObscured(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_isObscured for (&'a  QRectF) {
  fn isObscured(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10isObscuredERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem10isObscuredERK6QRectF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn hasCursor<T: QGraphicsItem_hasCursor>(&mut self, value: T) -> i8 {
    return value.hasCursor(self);
    // return 1;
  }
}

pub trait QGraphicsItem_hasCursor {
  fn hasCursor(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::hasCursor();
impl<'a> /*trait*/ QGraphicsItem_hasCursor for () {
  fn hasCursor(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9hasCursorEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem9hasCursorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setGraphicsEffect<T: QGraphicsItem_setGraphicsEffect>(&mut self, value: T)  {
     value.setGraphicsEffect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setGraphicsEffect {
  fn setGraphicsEffect(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setGraphicsEffect(QGraphicsEffect * effect);
impl<'a> /*trait*/ QGraphicsItem_setGraphicsEffect for (&'a mut QGraphicsEffect) {
  fn setGraphicsEffect(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem17setGraphicsEffectEP15QGraphicsEffect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem17setGraphicsEffectEP15QGraphicsEffect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn ensureVisible<T: QGraphicsItem_ensureVisible>(&mut self, value: T)  {
     value.ensureVisible(self);
    // return 1;
  }
}

pub trait QGraphicsItem_ensureVisible {
  fn ensureVisible(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsItem_ensureVisible for (f64, f64, f64, f64, i32, i32) {
  fn ensureVisible(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13ensureVisibleEddddii()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
     unsafe {_ZN13QGraphicsItem13ensureVisibleEddddii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapRectToParent<T: QGraphicsItem_mapRectToParent>(&mut self, value: T) -> QRectF {
    return value.mapRectToParent(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectToParent {
  fn mapRectToParent(self, rsthis: &mut QGraphicsItem) -> QRectF;
}

// proto:  QRectF QGraphicsItem::mapRectToParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectToParent for (&'a  QRectF) {
  fn mapRectToParent(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectToParentERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem15mapRectToParentERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setToolTip<T: QGraphicsItem_setToolTip>(&mut self, value: T)  {
     value.setToolTip(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setToolTip {
  fn setToolTip(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QGraphicsItem_setToolTip for (&'a  QString) {
  fn setToolTip(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn rotation<T: QGraphicsItem_rotation>(&mut self, value: T) -> f64 {
    return value.rotation(self);
    // return 1;
  }
}

pub trait QGraphicsItem_rotation {
  fn rotation(self, rsthis: &mut QGraphicsItem) -> f64;
}

// proto:  double QGraphicsItem::rotation();
impl<'a> /*trait*/ QGraphicsItem_rotation for () {
  fn rotation(self, rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8rotationEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8rotationEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn scene<T: QGraphicsItem_scene>(&mut self, value: T)  {
     value.scene(self);
    // return 1;
  }
}

pub trait QGraphicsItem_scene {
  fn scene(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsScene * QGraphicsItem::scene();
impl<'a> /*trait*/ QGraphicsItem_scene for () {
  fn scene(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5sceneEv()};
     unsafe {_ZNK13QGraphicsItem5sceneEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QRectF QGraphicsItem::mapRectToParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectToParent for (f64, f64, f64, f64) {
  fn mapRectToParent(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectToParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem15mapRectToParentEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromParent<T: QGraphicsItem_mapRectFromParent>(&mut self, value: T) -> QRectF {
    return value.mapRectFromParent(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectFromParent {
  fn mapRectFromParent(self, rsthis: &mut QGraphicsItem) -> QRectF;
}

// proto:  QRectF QGraphicsItem::mapRectFromParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromParent for (&'a  QRectF) {
  fn mapRectFromParent(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17mapRectFromParentERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem17mapRectFromParentERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setFocusProxy<T: QGraphicsItem_setFocusProxy>(&mut self, value: T)  {
     value.setFocusProxy(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setFocusProxy {
  fn setFocusProxy(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setFocusProxy(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItem_setFocusProxy for (&'a mut QGraphicsItem) {
  fn setFocusProxy(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13setFocusProxyEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem13setFocusProxyEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn acceptDrops<T: QGraphicsItem_acceptDrops>(&mut self, value: T) -> i8 {
    return value.acceptDrops(self);
    // return 1;
  }
}

pub trait QGraphicsItem_acceptDrops {
  fn acceptDrops(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::acceptDrops();
impl<'a> /*trait*/ QGraphicsItem_acceptDrops for () {
  fn acceptDrops(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11acceptDropsEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem11acceptDropsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromScene<T: QGraphicsItem_mapRectFromScene>(&mut self, value: T) -> QRectF {
    return value.mapRectFromScene(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectFromScene {
  fn mapRectFromScene(self, rsthis: &mut QGraphicsItem) -> QRectF;
}

// proto:  QRectF QGraphicsItem::mapRectFromScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromScene for (&'a  QRectF) {
  fn mapRectFromScene(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem16mapRectFromSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem16mapRectFromSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn focusScopeItem<T: QGraphicsItem_focusScopeItem>(&mut self, value: T)  {
     value.focusScopeItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_focusScopeItem {
  fn focusScopeItem(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsItem * QGraphicsItem::focusScopeItem();
impl<'a> /*trait*/ QGraphicsItem_focusScopeItem for () {
  fn focusScopeItem(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14focusScopeItemEv()};
     unsafe {_ZNK13QGraphicsItem14focusScopeItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn removeSceneEventFilter<T: QGraphicsItem_removeSceneEventFilter>(&mut self, value: T)  {
     value.removeSceneEventFilter(self);
    // return 1;
  }
}

pub trait QGraphicsItem_removeSceneEventFilter {
  fn removeSceneEventFilter(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::removeSceneEventFilter(QGraphicsItem * filterItem);
impl<'a> /*trait*/ QGraphicsItem_removeSceneEventFilter for (&'a mut QGraphicsItem) {
  fn removeSceneEventFilter(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem22removeSceneEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem22removeSceneEventFilterEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn focusProxy<T: QGraphicsItem_focusProxy>(&mut self, value: T)  {
     value.focusProxy(self);
    // return 1;
  }
}

pub trait QGraphicsItem_focusProxy {
  fn focusProxy(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsItem * QGraphicsItem::focusProxy();
impl<'a> /*trait*/ QGraphicsItem_focusProxy for () {
  fn focusProxy(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10focusProxyEv()};
     unsafe {_ZNK13QGraphicsItem10focusProxyEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn sceneBoundingRect<T: QGraphicsItem_sceneBoundingRect>(&mut self, value: T) -> QRectF {
    return value.sceneBoundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_sceneBoundingRect {
  fn sceneBoundingRect(self, rsthis: &mut QGraphicsItem) -> QRectF;
}

// proto:  QRectF QGraphicsItem::sceneBoundingRect();
impl<'a> /*trait*/ QGraphicsItem_sceneBoundingRect for () {
  fn sceneBoundingRect(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17sceneBoundingRectEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem17sceneBoundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn FreeQGraphicsItem<T: QGraphicsItem_FreeQGraphicsItem>(&mut self, value: T)  {
     value.FreeQGraphicsItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_FreeQGraphicsItem {
  fn FreeQGraphicsItem(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::FreeQGraphicsItem();
impl<'a> /*trait*/ QGraphicsItem_FreeQGraphicsItem for () {
  fn FreeQGraphicsItem(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItemD0Ev()};
     unsafe {_ZN13QGraphicsItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setX<T: QGraphicsItem_setX>(&mut self, value: T)  {
     value.setX(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setX {
  fn setX(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setX(qreal x);
impl<'a> /*trait*/ QGraphicsItem_setX for (f64) {
  fn setX(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4setXEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem4setXEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn update<T: QGraphicsItem_update>(&mut self, value: T)  {
     value.update(self);
    // return 1;
  }
}

pub trait QGraphicsItem_update {
  fn update(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::update(qreal x, qreal y, qreal width, qreal height);
impl<'a> /*trait*/ QGraphicsItem_update for (f64, f64, f64, f64) {
  fn update(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6updateEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN13QGraphicsItem6updateEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setSelected<T: QGraphicsItem_setSelected>(&mut self, value: T)  {
     value.setSelected(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setSelected {
  fn setSelected(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setSelected(bool selected);
impl<'a> /*trait*/ QGraphicsItem_setSelected for (i8) {
  fn setSelected(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11setSelectedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QGraphicsItem11setSelectedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectToItem for (&'a  QGraphicsItem, &'a  QRectF) {
  fn mapRectToItem(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapRectToItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem13mapRectToItemEPKS_RK6QRectF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn stackBefore<T: QGraphicsItem_stackBefore>(&mut self, value: T)  {
     value.stackBefore(self);
    // return 1;
  }
}

pub trait QGraphicsItem_stackBefore {
  fn stackBefore(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::stackBefore(const QGraphicsItem * sibling);
impl<'a> /*trait*/ QGraphicsItem_stackBefore for (&'a  QGraphicsItem) {
  fn stackBefore(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11stackBeforeEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem11stackBeforeEPKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn resetMatrix<T: QGraphicsItem_resetMatrix>(&mut self, value: T)  {
     value.resetMatrix(self);
    // return 1;
  }
}

pub trait QGraphicsItem_resetMatrix {
  fn resetMatrix(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::resetMatrix();
impl<'a> /*trait*/ QGraphicsItem_resetMatrix for () {
  fn resetMatrix(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11resetMatrixEv()};
     unsafe {_ZN13QGraphicsItem11resetMatrixEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn opaqueArea<T: QGraphicsItem_opaqueArea>(&mut self, value: T) -> QPainterPath {
    return value.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsItem_opaqueArea {
  fn opaqueArea(self, rsthis: &mut QGraphicsItem) -> QPainterPath;
}

// proto:  QPainterPath QGraphicsItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsItem_opaqueArea for () {
  fn opaqueArea(self, rsthis: &mut QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10opaqueAreaEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn unsetCursor<T: QGraphicsItem_unsetCursor>(&mut self, value: T)  {
     value.unsetCursor(self);
    // return 1;
  }
}

pub trait QGraphicsItem_unsetCursor {
  fn unsetCursor(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::unsetCursor();
impl<'a> /*trait*/ QGraphicsItem_unsetCursor for () {
  fn unsetCursor(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11unsetCursorEv()};
     unsafe {_ZN13QGraphicsItem11unsetCursorEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapRectToScene<T: QGraphicsItem_mapRectToScene>(&mut self, value: T) -> QRectF {
    return value.mapRectToScene(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectToScene {
  fn mapRectToScene(self, rsthis: &mut QGraphicsItem) -> QRectF;
}

// proto:  QRectF QGraphicsItem::mapRectToScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectToScene for (&'a  QRectF) {
  fn mapRectToScene(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14mapRectToSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem14mapRectToSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromItem<T: QGraphicsItem_mapRectFromItem>(&mut self, value: T) -> QRectF {
    return value.mapRectFromItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectFromItem {
  fn mapRectFromItem(self, rsthis: &mut QGraphicsItem) -> QRectF;
}

// proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromItem for (&'a  QGraphicsItem, f64, f64, f64, f64) {
  fn mapRectFromItem(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectFromItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem15mapRectFromItemEPKS_dddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn scale<T: QGraphicsItem_scale>(&mut self, value: T) -> f64 {
    return value.scale(self);
    // return 1;
  }
}

pub trait QGraphicsItem_scale {
  fn scale(self, rsthis: &mut QGraphicsItem) -> f64;
}

// proto:  double QGraphicsItem::scale();
impl<'a> /*trait*/ QGraphicsItem_scale for () {
  fn scale(self, rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5scaleEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem5scaleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setBoundingRegionGranularity<T: QGraphicsItem_setBoundingRegionGranularity>(&mut self, value: T)  {
     value.setBoundingRegionGranularity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setBoundingRegionGranularity {
  fn setBoundingRegionGranularity(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setBoundingRegionGranularity(qreal granularity);
impl<'a> /*trait*/ QGraphicsItem_setBoundingRegionGranularity for (f64) {
  fn setBoundingRegionGranularity(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem28setBoundingRegionGranularityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem28setBoundingRegionGranularityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setAcceptDrops<T: QGraphicsItem_setAcceptDrops>(&mut self, value: T)  {
     value.setAcceptDrops(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setAcceptDrops {
  fn setAcceptDrops(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setAcceptDrops(bool on);
impl<'a> /*trait*/ QGraphicsItem_setAcceptDrops for (i8) {
  fn setAcceptDrops(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem14setAcceptDropsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QGraphicsItem14setAcceptDropsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn ungrabKeyboard<T: QGraphicsItem_ungrabKeyboard>(&mut self, value: T)  {
     value.ungrabKeyboard(self);
    // return 1;
  }
}

pub trait QGraphicsItem_ungrabKeyboard {
  fn ungrabKeyboard(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::ungrabKeyboard();
impl<'a> /*trait*/ QGraphicsItem_ungrabKeyboard for () {
  fn ungrabKeyboard(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem14ungrabKeyboardEv()};
     unsafe {_ZN13QGraphicsItem14ungrabKeyboardEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setEnabled<T: QGraphicsItem_setEnabled>(&mut self, value: T)  {
     value.setEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setEnabled {
  fn setEnabled(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setEnabled(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setEnabled for (i8) {
  fn setEnabled(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QGraphicsItem10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn graphicsEffect<T: QGraphicsItem_graphicsEffect>(&mut self, value: T)  {
     value.graphicsEffect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_graphicsEffect {
  fn graphicsEffect(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsEffect * QGraphicsItem::graphicsEffect();
impl<'a> /*trait*/ QGraphicsItem_graphicsEffect for () {
  fn graphicsEffect(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14graphicsEffectEv()};
     unsafe {_ZNK13QGraphicsItem14graphicsEffectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn acceptHoverEvents<T: QGraphicsItem_acceptHoverEvents>(&mut self, value: T) -> i8 {
    return value.acceptHoverEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_acceptHoverEvents {
  fn acceptHoverEvents(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::acceptHoverEvents();
impl<'a> /*trait*/ QGraphicsItem_acceptHoverEvents for () {
  fn acceptHoverEvents(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17acceptHoverEventsEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem17acceptHoverEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn topLevelWidget<T: QGraphicsItem_topLevelWidget>(&mut self, value: T)  {
     value.topLevelWidget(self);
    // return 1;
  }
}

pub trait QGraphicsItem_topLevelWidget {
  fn topLevelWidget(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsWidget * QGraphicsItem::topLevelWidget();
impl<'a> /*trait*/ QGraphicsItem_topLevelWidget for () {
  fn topLevelWidget(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14topLevelWidgetEv()};
     unsafe {_ZNK13QGraphicsItem14topLevelWidgetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn transformations<T: QGraphicsItem_transformations>(&mut self, value: T)  {
     value.transformations(self);
    // return 1;
  }
}

pub trait QGraphicsItem_transformations {
  fn transformations(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QList<QGraphicsTransform *> QGraphicsItem::transformations();
impl<'a> /*trait*/ QGraphicsItem_transformations for () {
  fn transformations(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15transformationsEv()};
     unsafe {_ZNK13QGraphicsItem15transformationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QRectF QGraphicsItem::mapRectFromScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromScene for (f64, f64, f64, f64) {
  fn mapRectFromScene(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem16mapRectFromSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem16mapRectFromSceneEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn advance<T: QGraphicsItem_advance>(&mut self, value: T)  {
     value.advance(self);
    // return 1;
  }
}

pub trait QGraphicsItem_advance {
  fn advance(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::advance(int phase);
impl<'a> /*trait*/ QGraphicsItem_advance for (i32) {
  fn advance(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem7advanceEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QGraphicsItem7advanceEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn sceneMatrix<T: QGraphicsItem_sceneMatrix>(&mut self, value: T) -> QMatrix {
    return value.sceneMatrix(self);
    // return 1;
  }
}

pub trait QGraphicsItem_sceneMatrix {
  fn sceneMatrix(self, rsthis: &mut QGraphicsItem) -> QMatrix;
}

// proto:  QMatrix QGraphicsItem::sceneMatrix();
impl<'a> /*trait*/ QGraphicsItem_sceneMatrix for () {
  fn sceneMatrix(self, rsthis: &mut QGraphicsItem) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11sceneMatrixEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem11sceneMatrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setFiltersChildEvents<T: QGraphicsItem_setFiltersChildEvents>(&mut self, value: T)  {
     value.setFiltersChildEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setFiltersChildEvents {
  fn setFiltersChildEvents(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setFiltersChildEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setFiltersChildEvents for (i8) {
  fn setFiltersChildEvents(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem21setFiltersChildEventsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QGraphicsItem21setFiltersChildEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn itemTransform<T: QGraphicsItem_itemTransform>(&mut self, value: T) -> QTransform {
    return value.itemTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_itemTransform {
  fn itemTransform(self, rsthis: &mut QGraphicsItem) -> QTransform;
}

// proto:  QTransform QGraphicsItem::itemTransform(const QGraphicsItem * other, bool * ok);
impl<'a> /*trait*/ QGraphicsItem_itemTransform for (&'a  QGraphicsItem, &'a mut i8) {
  fn itemTransform(self, rsthis: &mut QGraphicsItem) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13itemTransformEPKS_Pb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut int8_t;
    let mut ret = unsafe {_ZNK13QGraphicsItem13itemTransformEPKS_Pb(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsItem::setTransformOriginPoint(qreal ax, qreal ay);
impl<'a> /*trait*/ QGraphicsItem_setTransformOriginPoint for (f64, f64) {
  fn setTransformOriginPoint(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem23setTransformOriginPointEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN13QGraphicsItem23setTransformOriginPointEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn moveBy<T: QGraphicsItem_moveBy>(&mut self, value: T)  {
     value.moveBy(self);
    // return 1;
  }
}

pub trait QGraphicsItem_moveBy {
  fn moveBy(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::moveBy(qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsItem_moveBy for (f64, f64) {
  fn moveBy(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6moveByEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN13QGraphicsItem6moveByEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn group<T: QGraphicsItem_group>(&mut self, value: T)  {
     value.group(self);
    // return 1;
  }
}

pub trait QGraphicsItem_group {
  fn group(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsItemGroup * QGraphicsItem::group();
impl<'a> /*trait*/ QGraphicsItem_group for () {
  fn group(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5groupEv()};
     unsafe {_ZNK13QGraphicsItem5groupEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn shape<T: QGraphicsItem_shape>(&mut self, value: T) -> QPainterPath {
    return value.shape(self);
    // return 1;
  }
}

pub trait QGraphicsItem_shape {
  fn shape(self, rsthis: &mut QGraphicsItem) -> QPainterPath;
}

// proto:  QPainterPath QGraphicsItem::shape();
impl<'a> /*trait*/ QGraphicsItem_shape for () {
  fn shape(self, rsthis: &mut QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5shapeEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn scroll<T: QGraphicsItem_scroll>(&mut self, value: T)  {
     value.scroll(self);
    // return 1;
  }
}

pub trait QGraphicsItem_scroll {
  fn scroll(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::scroll(qreal dx, qreal dy, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_scroll for (f64, f64, &'a  QRectF) {
  fn scroll(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6scrollEddRK6QRectF()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem6scrollEddRK6QRectF(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isObscuredBy<T: QGraphicsItem_isObscuredBy>(&mut self, value: T) -> i8 {
    return value.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isObscuredBy {
  fn isObscuredBy(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItem_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12isObscuredByEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem12isObscuredByEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setData<T: QGraphicsItem_setData>(&mut self, value: T)  {
     value.setData(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setData {
  fn setData(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setData(int key, const QVariant & value);
impl<'a> /*trait*/ QGraphicsItem_setData for (i32, &'a  QVariant) {
  fn setData(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem7setDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem7setDataEiRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn commonAncestorItem<T: QGraphicsItem_commonAncestorItem>(&mut self, value: T)  {
     value.commonAncestorItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_commonAncestorItem {
  fn commonAncestorItem(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsItem * QGraphicsItem::commonAncestorItem(const QGraphicsItem * other);
impl<'a> /*trait*/ QGraphicsItem_commonAncestorItem for (&'a  QGraphicsItem) {
  fn commonAncestorItem(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem18commonAncestorItemEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK13QGraphicsItem18commonAncestorItemEPKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setGroup<T: QGraphicsItem_setGroup>(&mut self, value: T)  {
     value.setGroup(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setGroup {
  fn setGroup(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setGroup(QGraphicsItemGroup * group);
impl<'a> /*trait*/ QGraphicsItem_setGroup for (&'a mut QGraphicsItemGroup) {
  fn setGroup(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem8setGroupEP18QGraphicsItemGroup()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem8setGroupEP18QGraphicsItemGroup(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QRectF QGraphicsItem::mapRectFromParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromParent for (f64, f64, f64, f64) {
  fn mapRectFromParent(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17mapRectFromParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem17mapRectFromParentEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn show<T: QGraphicsItem_show>(&mut self, value: T)  {
     value.show(self);
    // return 1;
  }
}

pub trait QGraphicsItem_show {
  fn show(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::show();
impl<'a> /*trait*/ QGraphicsItem_show for () {
  fn show(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4showEv()};
     unsafe {_ZN13QGraphicsItem4showEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromItem for (&'a  QGraphicsItem, &'a  QRectF) {
  fn mapRectFromItem(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectFromItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem15mapRectFromItemEPKS_RK6QRectF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn y<T: QGraphicsItem_y>(&mut self, value: T) -> f64 {
    return value.y(self);
    // return 1;
  }
}

pub trait QGraphicsItem_y {
  fn y(self, rsthis: &mut QGraphicsItem) -> f64;
}

// proto:  double QGraphicsItem::y();
impl<'a> /*trait*/ QGraphicsItem_y for () {
  fn y(self, rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem1yEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem1yEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn hasFocus<T: QGraphicsItem_hasFocus>(&mut self, value: T) -> i8 {
    return value.hasFocus(self);
    // return 1;
  }
}

pub trait QGraphicsItem_hasFocus {
  fn hasFocus(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::hasFocus();
impl<'a> /*trait*/ QGraphicsItem_hasFocus for () {
  fn hasFocus(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8hasFocusEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8hasFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn clipPath<T: QGraphicsItem_clipPath>(&mut self, value: T) -> QPainterPath {
    return value.clipPath(self);
    // return 1;
  }
}

pub trait QGraphicsItem_clipPath {
  fn clipPath(self, rsthis: &mut QGraphicsItem) -> QPainterPath;
}

// proto:  QPainterPath QGraphicsItem::clipPath();
impl<'a> /*trait*/ QGraphicsItem_clipPath for () {
  fn clipPath(self, rsthis: &mut QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8clipPathEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem8clipPathEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setPos<T: QGraphicsItem_setPos>(&mut self, value: T)  {
     value.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setPos {
  fn setPos(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setPos(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_setPos for (f64, f64) {
  fn setPos(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6setPosEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN13QGraphicsItem6setPosEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isEnabled<T: QGraphicsItem_isEnabled>(&mut self, value: T) -> i8 {
    return value.isEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isEnabled {
  fn isEnabled(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isEnabled();
impl<'a> /*trait*/ QGraphicsItem_isEnabled for () {
  fn isEnabled(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9isEnabledEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn contains<T: QGraphicsItem_contains>(&mut self, value: T) -> i8 {
    return value.contains(self);
    // return 1;
  }
}

pub trait QGraphicsItem_contains {
  fn contains(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_contains for (&'a  QPointF) {
  fn contains(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isPanel<T: QGraphicsItem_isPanel>(&mut self, value: T) -> i8 {
    return value.isPanel(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isPanel {
  fn isPanel(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isPanel();
impl<'a> /*trait*/ QGraphicsItem_isPanel for () {
  fn isPanel(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem7isPanelEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem7isPanelEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn filtersChildEvents<T: QGraphicsItem_filtersChildEvents>(&mut self, value: T) -> i8 {
    return value.filtersChildEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_filtersChildEvents {
  fn filtersChildEvents(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::filtersChildEvents();
impl<'a> /*trait*/ QGraphicsItem_filtersChildEvents for () {
  fn filtersChildEvents(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem18filtersChildEventsEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem18filtersChildEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn grabKeyboard<T: QGraphicsItem_grabKeyboard>(&mut self, value: T)  {
     value.grabKeyboard(self);
    // return 1;
  }
}

pub trait QGraphicsItem_grabKeyboard {
  fn grabKeyboard(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::grabKeyboard();
impl<'a> /*trait*/ QGraphicsItem_grabKeyboard for () {
  fn grabKeyboard(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem12grabKeyboardEv()};
     unsafe {_ZN13QGraphicsItem12grabKeyboardEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setActive<T: QGraphicsItem_setActive>(&mut self, value: T)  {
     value.setActive(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setActive {
  fn setActive(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setActive(bool active);
impl<'a> /*trait*/ QGraphicsItem_setActive for (i8) {
  fn setActive(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setActiveEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QGraphicsItem9setActiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn toGraphicsObject<T: QGraphicsItem_toGraphicsObject>(&mut self, value: T)  {
     value.toGraphicsObject(self);
    // return 1;
  }
}

pub trait QGraphicsItem_toGraphicsObject {
  fn toGraphicsObject(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsObject * QGraphicsItem::toGraphicsObject();
impl<'a> /*trait*/ QGraphicsItem_toGraphicsObject for () {
  fn toGraphicsObject(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem16toGraphicsObjectEv()};
     unsafe {_ZN13QGraphicsItem16toGraphicsObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setHandlesChildEvents<T: QGraphicsItem_setHandlesChildEvents>(&mut self, value: T)  {
     value.setHandlesChildEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setHandlesChildEvents {
  fn setHandlesChildEvents(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setHandlesChildEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setHandlesChildEvents for (i8) {
  fn setHandlesChildEvents(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem21setHandlesChildEventsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QGraphicsItem21setHandlesChildEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setMatrix<T: QGraphicsItem_setMatrix>(&mut self, value: T)  {
     value.setMatrix(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setMatrix {
  fn setMatrix(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setMatrix(const QMatrix & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsItem_setMatrix for (&'a  QMatrix, i8) {
  fn setMatrix(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setMatrixERK7QMatrixb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN13QGraphicsItem9setMatrixERK7QMatrixb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QGraphicsItem::update(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_update for (&'a  QRectF) {
  fn update(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6updateERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem6updateERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn transform<T: QGraphicsItem_transform>(&mut self, value: T) -> QTransform {
    return value.transform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_transform {
  fn transform(self, rsthis: &mut QGraphicsItem) -> QTransform;
}

// proto:  QTransform QGraphicsItem::transform();
impl<'a> /*trait*/ QGraphicsItem_transform for () {
  fn transform(self, rsthis: &mut QGraphicsItem) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9transformEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn data<T: QGraphicsItem_data>(&mut self, value: T) -> QVariant {
    return value.data(self);
    // return 1;
  }
}

pub trait QGraphicsItem_data {
  fn data(self, rsthis: &mut QGraphicsItem) -> QVariant;
}

// proto:  QVariant QGraphicsItem::data(int key);
impl<'a> /*trait*/ QGraphicsItem_data for (i32) {
  fn data(self, rsthis: &mut QGraphicsItem) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QGraphicsItem4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn hide<T: QGraphicsItem_hide>(&mut self, value: T)  {
     value.hide(self);
    // return 1;
  }
}

pub trait QGraphicsItem_hide {
  fn hide(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::hide();
impl<'a> /*trait*/ QGraphicsItem_hide for () {
  fn hide(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4hideEv()};
     unsafe {_ZN13QGraphicsItem4hideEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isUnderMouse<T: QGraphicsItem_isUnderMouse>(&mut self, value: T) -> i8 {
    return value.isUnderMouse(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isUnderMouse {
  fn isUnderMouse(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isUnderMouse();
impl<'a> /*trait*/ QGraphicsItem_isUnderMouse for () {
  fn isUnderMouse(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12isUnderMouseEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem12isUnderMouseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setAcceptTouchEvents<T: QGraphicsItem_setAcceptTouchEvents>(&mut self, value: T)  {
     value.setAcceptTouchEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setAcceptTouchEvents {
  fn setAcceptTouchEvents(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setAcceptTouchEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setAcceptTouchEvents for (i8) {
  fn setAcceptTouchEvents(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem20setAcceptTouchEventsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QGraphicsItem20setAcceptTouchEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setAcceptHoverEvents<T: QGraphicsItem_setAcceptHoverEvents>(&mut self, value: T)  {
     value.setAcceptHoverEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setAcceptHoverEvents {
  fn setAcceptHoverEvents(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setAcceptHoverEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setAcceptHoverEvents for (i8) {
  fn setAcceptHoverEvents(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem20setAcceptHoverEventsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QGraphicsItem20setAcceptHoverEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn childItems<T: QGraphicsItem_childItems>(&mut self, value: T)  {
     value.childItems(self);
    // return 1;
  }
}

pub trait QGraphicsItem_childItems {
  fn childItems(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QList<QGraphicsItem *> QGraphicsItem::childItems();
impl<'a> /*trait*/ QGraphicsItem_childItems for () {
  fn childItems(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10childItemsEv()};
     unsafe {_ZNK13QGraphicsItem10childItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isAncestorOf<T: QGraphicsItem_isAncestorOf>(&mut self, value: T) -> i8 {
    return value.isAncestorOf(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isAncestorOf {
  fn isAncestorOf(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isAncestorOf(const QGraphicsItem * child);
impl<'a> /*trait*/ QGraphicsItem_isAncestorOf for (&'a  QGraphicsItem) {
  fn isAncestorOf(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12isAncestorOfEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem12isAncestorOfEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn opacity<T: QGraphicsItem_opacity>(&mut self, value: T) -> f64 {
    return value.opacity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_opacity {
  fn opacity(self, rsthis: &mut QGraphicsItem) -> f64;
}

// proto:  double QGraphicsItem::opacity();
impl<'a> /*trait*/ QGraphicsItem_opacity for () {
  fn opacity(self, rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem7opacityEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isVisibleTo<T: QGraphicsItem_isVisibleTo>(&mut self, value: T) -> i8 {
    return value.isVisibleTo(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isVisibleTo {
  fn isVisibleTo(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isVisibleTo(const QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsItem_isVisibleTo for (&'a  QGraphicsItem) {
  fn isVisibleTo(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11isVisibleToEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem11isVisibleToEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn toolTip<T: QGraphicsItem_toolTip>(&mut self, value: T) -> QString {
    return value.toolTip(self);
    // return 1;
  }
}

pub trait QGraphicsItem_toolTip {
  fn toolTip(self, rsthis: &mut QGraphicsItem) -> QString;
}

// proto:  QString QGraphicsItem::toolTip();
impl<'a> /*trait*/ QGraphicsItem_toolTip for () {
  fn toolTip(self, rsthis: &mut QGraphicsItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem7toolTipEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn cursor<T: QGraphicsItem_cursor>(&mut self, value: T) -> QCursor {
    return value.cursor(self);
    // return 1;
  }
}

pub trait QGraphicsItem_cursor {
  fn cursor(self, rsthis: &mut QGraphicsItem) -> QCursor;
}

// proto:  QCursor QGraphicsItem::cursor();
impl<'a> /*trait*/ QGraphicsItem_cursor for () {
  fn cursor(self, rsthis: &mut QGraphicsItem) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6cursorEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem6cursorEv(rsthis.qclsinst)};
    let mut ret1 = QCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn zValue<T: QGraphicsItem_zValue>(&mut self, value: T) -> f64 {
    return value.zValue(self);
    // return 1;
  }
}

pub trait QGraphicsItem_zValue {
  fn zValue(self, rsthis: &mut QGraphicsItem) -> f64;
}

// proto:  double QGraphicsItem::zValue();
impl<'a> /*trait*/ QGraphicsItem_zValue for () {
  fn zValue(self, rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6zValueEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem6zValueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn matrix<T: QGraphicsItem_matrix>(&mut self, value: T) -> QMatrix {
    return value.matrix(self);
    // return 1;
  }
}

pub trait QGraphicsItem_matrix {
  fn matrix(self, rsthis: &mut QGraphicsItem) -> QMatrix;
}

// proto:  QMatrix QGraphicsItem::matrix();
impl<'a> /*trait*/ QGraphicsItem_matrix for () {
  fn matrix(self, rsthis: &mut QGraphicsItem) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6matrixEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QRectF QGraphicsItem::mapRectToScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectToScene for (f64, f64, f64, f64) {
  fn mapRectToScene(self, rsthis: &mut QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14mapRectToSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsItem14mapRectToSceneEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsItem::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsItem_setPos for (&'a  QPointF) {
  fn setPos(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn panel<T: QGraphicsItem_panel>(&mut self, value: T)  {
     value.panel(self);
    // return 1;
  }
}

pub trait QGraphicsItem_panel {
  fn panel(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsItem * QGraphicsItem::panel();
impl<'a> /*trait*/ QGraphicsItem_panel for () {
  fn panel(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5panelEv()};
     unsafe {_ZNK13QGraphicsItem5panelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isClipped<T: QGraphicsItem_isClipped>(&mut self, value: T) -> i8 {
    return value.isClipped(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isClipped {
  fn isClipped(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isClipped();
impl<'a> /*trait*/ QGraphicsItem_isClipped for () {
  fn isClipped(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9isClippedEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem9isClippedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn topLevelItem<T: QGraphicsItem_topLevelItem>(&mut self, value: T)  {
     value.topLevelItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_topLevelItem {
  fn topLevelItem(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  QGraphicsItem * QGraphicsItem::topLevelItem();
impl<'a> /*trait*/ QGraphicsItem_topLevelItem for () {
  fn topLevelItem(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12topLevelItemEv()};
     unsafe {_ZNK13QGraphicsItem12topLevelItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setScale<T: QGraphicsItem_setScale>(&mut self, value: T)  {
     value.setScale(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setScale {
  fn setScale(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setScale(qreal scale);
impl<'a> /*trait*/ QGraphicsItem_setScale for (f64) {
  fn setScale(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem8setScaleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem8setScaleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setCursor<T: QGraphicsItem_setCursor>(&mut self, value: T)  {
     value.setCursor(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setCursor {
  fn setCursor(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setCursor(const QCursor & cursor);
impl<'a> /*trait*/ QGraphicsItem_setCursor for (&'a  QCursor) {
  fn setCursor(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsItem9setCursorERK7QCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isVisible<T: QGraphicsItem_isVisible>(&mut self, value: T) -> i8 {
    return value.isVisible(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isVisible {
  fn isVisible(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isVisible();
impl<'a> /*trait*/ QGraphicsItem_isVisible for () {
  fn isVisible(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9isVisibleEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn pos<T: QGraphicsItem_pos>(&mut self, value: T) -> QPointF {
    return value.pos(self);
    // return 1;
  }
}

pub trait QGraphicsItem_pos {
  fn pos(self, rsthis: &mut QGraphicsItem) -> QPointF;
}

// proto:  QPointF QGraphicsItem::pos();
impl<'a> /*trait*/ QGraphicsItem_pos for () {
  fn pos(self, rsthis: &mut QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem3posEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn isBlockedByModalPanel<T: QGraphicsItem_isBlockedByModalPanel>(&mut self, value: T) -> i8 {
    return value.isBlockedByModalPanel(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isBlockedByModalPanel {
  fn isBlockedByModalPanel(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::isBlockedByModalPanel(QGraphicsItem ** blockingPanel);
impl<'a> /*trait*/ QGraphicsItem_isBlockedByModalPanel for (&'a mut QGraphicsItem) {
  fn isBlockedByModalPanel(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem21isBlockedByModalPanelEPPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem21isBlockedByModalPanelEPPS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn effectiveOpacity<T: QGraphicsItem_effectiveOpacity>(&mut self, value: T) -> f64 {
    return value.effectiveOpacity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_effectiveOpacity {
  fn effectiveOpacity(self, rsthis: &mut QGraphicsItem) -> f64;
}

// proto:  double QGraphicsItem::effectiveOpacity();
impl<'a> /*trait*/ QGraphicsItem_effectiveOpacity for () {
  fn effectiveOpacity(self, rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem16effectiveOpacityEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem16effectiveOpacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QGraphicsItem::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsItem_ensureVisible for (&'a  QRectF, i32, i32) {
  fn ensureVisible(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13ensureVisibleERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN13QGraphicsItem13ensureVisibleERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn boundingRegionGranularity<T: QGraphicsItem_boundingRegionGranularity>(&mut self, value: T) -> f64 {
    return value.boundingRegionGranularity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_boundingRegionGranularity {
  fn boundingRegionGranularity(self, rsthis: &mut QGraphicsItem) -> f64;
}

// proto:  double QGraphicsItem::boundingRegionGranularity();
impl<'a> /*trait*/ QGraphicsItem_boundingRegionGranularity for () {
  fn boundingRegionGranularity(self, rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem25boundingRegionGranularityEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem25boundingRegionGranularityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn x<T: QGraphicsItem_x>(&mut self, value: T) -> f64 {
    return value.x(self);
    // return 1;
  }
}

pub trait QGraphicsItem_x {
  fn x(self, rsthis: &mut QGraphicsItem) -> f64;
}

// proto:  double QGraphicsItem::x();
impl<'a> /*trait*/ QGraphicsItem_x for () {
  fn x(self, rsthis: &mut QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem1xEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem1xEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn grabMouse<T: QGraphicsItem_grabMouse>(&mut self, value: T)  {
     value.grabMouse(self);
    // return 1;
  }
}

pub trait QGraphicsItem_grabMouse {
  fn grabMouse(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::grabMouse();
impl<'a> /*trait*/ QGraphicsItem_grabMouse for () {
  fn grabMouse(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9grabMouseEv()};
     unsafe {_ZN13QGraphicsItem9grabMouseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setVisible<T: QGraphicsItem_setVisible>(&mut self, value: T)  {
     value.setVisible(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setVisible {
  fn setVisible(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setVisible(bool visible);
impl<'a> /*trait*/ QGraphicsItem_setVisible for (i8) {
  fn setVisible(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QGraphicsItem10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setRotation<T: QGraphicsItem_setRotation>(&mut self, value: T)  {
     value.setRotation(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setRotation {
  fn setRotation(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setRotation(qreal angle);
impl<'a> /*trait*/ QGraphicsItem_setRotation for (f64) {
  fn setRotation(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11setRotationEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsItem11setRotationEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn deviceTransform<T: QGraphicsItem_deviceTransform>(&mut self, value: T) -> QTransform {
    return value.deviceTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_deviceTransform {
  fn deviceTransform(self, rsthis: &mut QGraphicsItem) -> QTransform;
}

// proto:  QTransform QGraphicsItem::deviceTransform(const QTransform & viewportTransform);
impl<'a> /*trait*/ QGraphicsItem_deviceTransform for (&'a  QTransform) {
  fn deviceTransform(self, rsthis: &mut QGraphicsItem) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15deviceTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsItem15deviceTransformERK10QTransform(rsthis.qclsinst, arg0)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn acceptTouchEvents<T: QGraphicsItem_acceptTouchEvents>(&mut self, value: T) -> i8 {
    return value.acceptTouchEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_acceptTouchEvents {
  fn acceptTouchEvents(self, rsthis: &mut QGraphicsItem) -> i8;
}

// proto:  bool QGraphicsItem::acceptTouchEvents();
impl<'a> /*trait*/ QGraphicsItem_acceptTouchEvents for () {
  fn acceptTouchEvents(self, rsthis: &mut QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17acceptTouchEventsEv()};
    let mut ret = unsafe {_ZNK13QGraphicsItem17acceptTouchEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn setTransform<T: QGraphicsItem_setTransform>(&mut self, value: T)  {
     value.setTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setTransform {
  fn setTransform(self, rsthis: &mut QGraphicsItem) ;
}

// proto:  void QGraphicsItem::setTransform(const QTransform & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsItem_setTransform for (&'a  QTransform, i8) {
  fn setTransform(self, rsthis: &mut QGraphicsItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem12setTransformERK10QTransformb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN13QGraphicsItem12setTransformERK10QTransformb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

