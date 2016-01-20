// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qquickitem.h
// dst-file: /src/quick/qquickitem.rs
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
// use super::qquickitem::QQuickItem; // 773
use super::super::gui::qmatrix4x4::QMatrix4x4; // 771
use super::super::core::qpoint::QPointF; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qrect::QRectF; // 771
use super::qsgtextureprovider::QSGTextureProvider; // 773
use super::super::gui::qcursor::QCursor; // 771
use super::qquickwindow::QQuickWindow; // 773
use super::super::core::qsize::QSizeF; // 771
use super::super::qml::qjsvalue::QJSValue; // 771
use super::super::core::qsize::QSize; // 771
use super::super::gui::qtransform::QTransform; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQuickTransform_Class_Size() -> c_int;
  // proto:  void QQuickTransform::~QQuickTransform();
  fn _ZN15QQuickTransformD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QQuickTransform::metaObject();
  fn _ZNK15QQuickTransform10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickTransform::QQuickTransform(QObject * parent);
  fn _ZN15QQuickTransformC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickTransform::appendToItem(QQuickItem * );
  fn _ZN15QQuickTransform12appendToItemEP10QQuickItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickTransform::applyTo(QMatrix4x4 * matrix);
  fn _ZNK15QQuickTransform7applyToEP10QMatrix4x4(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickTransform::prependToItem(QQuickItem * );
  fn _ZN15QQuickTransform13prependToItemEP10QQuickItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QQuickItem_Class_Size() -> c_int;
  // proto:  qreal QQuickItem::implicitWidth();
  fn _ZNK10QQuickItem13implicitWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QQuickItem::setImplicitWidth(qreal );
  fn _ZN10QQuickItem16setImplicitWidthEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QQuickItem::setBaselineOffset(qreal );
  fn _ZN10QQuickItem17setBaselineOffsetEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QQuickItem::resetWidth();
  fn _ZN10QQuickItem10resetWidthEv(qthis: u64 /* *mut c_void*/);
  // proto:  qreal QQuickItem::y();
  fn _ZNK10QQuickItem1yEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickItem::forceActiveFocus();
  fn _ZN10QQuickItem16forceActiveFocusEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickItem::setClip(bool );
  fn _ZN10QQuickItem7setClipEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QQuickItem::setHeight(qreal );
  fn _ZN10QQuickItem9setHeightEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QQuickItem::setY(qreal );
  fn _ZN10QQuickItem4setYEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  qreal QQuickItem::rotation();
  fn _ZNK10QQuickItem8rotationEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QQuickItem::opacity();
  fn _ZNK10QQuickItem7opacityEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QQuickItem::implicitHeight();
  fn _ZNK10QQuickItem14implicitHeightEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QQuickItem::isTextureProvider();
  fn _ZNK10QQuickItem17isTextureProviderEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQuickItem::smooth();
  fn _ZNK10QQuickItem6smoothEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QQmlListProperty<QQuickTransform> QQuickItem::transform();
  fn _ZN10QQuickItem9transformEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickItem::stackBefore(const QQuickItem * );
  fn _ZN10QQuickItem11stackBeforeEPKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickItem::setActiveFocusOnTab(bool );
  fn _ZN10QQuickItem19setActiveFocusOnTabEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  qreal QQuickItem::scale();
  fn _ZNK10QQuickItem5scaleEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QQuickItem::setEnabled(bool );
  fn _ZN10QQuickItem10setEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QQuickItem::setImplicitHeight(qreal );
  fn _ZN10QQuickItem17setImplicitHeightEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QQuickItem::setPosition(const QPointF & );
  fn _ZN10QQuickItem11setPositionERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickItem::setSmooth(bool );
  fn _ZN10QQuickItem9setSmoothEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QPointF QQuickItem::mapFromScene(const QPointF & point);
  fn _ZNK10QQuickItem12mapFromSceneERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QQuickItem::setAcceptHoverEvents(bool enabled);
  fn _ZN10QQuickItem20setAcceptHoverEventsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QPointF QQuickItem::transformOriginPoint();
  fn _ZNK10QQuickItem20transformOriginPointEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickItem::setState(const QString & );
  fn _ZN10QQuickItem8setStateERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QQuickItem * QQuickItem::parentItem();
  fn _ZNK10QQuickItem10parentItemEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QQuickItem::isFocusScope();
  fn _ZNK10QQuickItem12isFocusScopeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQuickItem::contains(const QPointF & point);
  fn _ZNK10QQuickItem8containsERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QRectF QQuickItem::boundingRect();
  fn _ZNK10QQuickItem12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickItem::resetAntialiasing();
  fn _ZN10QQuickItem17resetAntialiasingEv(qthis: u64 /* *mut c_void*/);
  // proto:  QRectF QQuickItem::mapRectToItem(const QQuickItem * item, const QRectF & rect);
  fn _ZNK10QQuickItem13mapRectToItemEPKS_RK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  qreal QQuickItem::x();
  fn _ZNK10QQuickItem1xEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPointF QQuickItem::mapFromItem(const QQuickItem * item, const QPointF & point);
  fn _ZNK10QQuickItem11mapFromItemEPKS_RK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QQuickItem::isVisible();
  fn _ZNK10QQuickItem9isVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSGTextureProvider * QQuickItem::textureProvider();
  fn _ZNK10QQuickItem15textureProviderEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickItem::setZ(qreal );
  fn _ZN10QQuickItem4setZEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QQuickItem::unsetCursor();
  fn _ZN10QQuickItem11unsetCursorEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQuickItem::acceptHoverEvents();
  fn _ZNK10QQuickItem17acceptHoverEventsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQuickItem::setFiltersChildMouseEvents(bool filter);
  fn _ZN10QQuickItem26setFiltersChildMouseEventsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QQuickItem::isEnabled();
  fn _ZNK10QQuickItem9isEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QPointF QQuickItem::mapToItem(const QQuickItem * item, const QPointF & point);
  fn _ZNK10QQuickItem9mapToItemEPKS_RK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  qreal QQuickItem::width();
  fn _ZNK10QQuickItem5widthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QPointF QQuickItem::mapToScene(const QPointF & point);
  fn _ZNK10QQuickItem10mapToSceneERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QQuickItem::keepTouchGrab();
  fn _ZNK10QQuickItem13keepTouchGrabEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRectF QQuickItem::childrenRect();
  fn _ZN10QQuickItem12childrenRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QQuickItem::metaObject();
  fn _ZNK10QQuickItem10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickItem::setKeepMouseGrab(bool );
  fn _ZN10QQuickItem16setKeepMouseGrabEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QQuickItem::filtersChildMouseEvents();
  fn _ZNK10QQuickItem23filtersChildMouseEventsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQuickItem::hasActiveFocus();
  fn _ZNK10QQuickItem14hasActiveFocusEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQuickItem::isUnderMouse();
  fn _ZNK10QQuickItem12isUnderMouseEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQuickItem::setTransformOriginPoint(const QPointF & );
  fn _ZN10QQuickItem23setTransformOriginPointERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickItem::polish();
  fn _ZN10QQuickItem6polishEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickItem::setParentItem(QQuickItem * parent);
  fn _ZN10QQuickItem13setParentItemEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickItem::QQuickItem(QQuickItem * parent);
  fn _ZN10QQuickItemC2EPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickItem::setWidth(qreal );
  fn _ZN10QQuickItem8setWidthEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QQuickItem * QQuickItem::nextItemInFocusChain(bool forward);
  fn _ZN10QQuickItem20nextItemInFocusChainEb(qthis: u64 /* *mut c_void*/, arg0: c_char) -> *mut c_void;
  // proto:  void QQuickItem::ungrabTouchPoints();
  fn _ZN10QQuickItem17ungrabTouchPointsEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQuickItem::keepMouseGrab();
  fn _ZNK10QQuickItem13keepMouseGrabEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRectF QQuickItem::mapRectFromScene(const QRectF & rect);
  fn _ZNK10QQuickItem16mapRectFromSceneERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPointF QQuickItem::position();
  fn _ZNK10QQuickItem8positionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QQuickItem::height();
  fn _ZNK10QQuickItem6heightEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QQuickItem * QQuickItem::childAt(qreal x, qreal y);
  fn _ZNK10QQuickItem7childAtEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  QCursor QQuickItem::cursor();
  fn _ZNK10QQuickItem6cursorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QQuickItem::clip();
  fn _ZNK10QQuickItem4clipEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQuickItem::antialiasing();
  fn _ZNK10QQuickItem12antialiasingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QQuickWindow * QQuickItem::window();
  fn _ZNK10QQuickItem6windowEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRectF QQuickItem::mapRectFromItem(const QQuickItem * item, const QRectF & rect);
  fn _ZNK10QQuickItem15mapRectFromItemEPKS_RK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QQuickItem::setVisible(bool );
  fn _ZN10QQuickItem10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QQuickItem::setFocus(bool );
  fn _ZN10QQuickItem8setFocusEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QQuickItem::activeFocusOnTab();
  fn _ZNK10QQuickItem16activeFocusOnTabEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qreal QQuickItem::z();
  fn _ZNK10QQuickItem1zEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QQuickItem::setOpacity(qreal );
  fn _ZN10QQuickItem10setOpacityEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QQuickItem::setAntialiasing(bool );
  fn _ZN10QQuickItem15setAntialiasingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QList<QQuickItem *> QQuickItem::childItems();
  fn _ZNK10QQuickItem10childItemsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickItem::setSize(const QSizeF & size);
  fn _ZN10QQuickItem7setSizeERK6QSizeF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickItem::~QQuickItem();
  fn _ZN10QQuickItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QQuickItem * QQuickItem::scopedFocusItem();
  fn _ZNK10QQuickItem15scopedFocusItemEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QQuickItem::grabToImage(const QJSValue & callback, const QSize & targetSize);
  fn _ZN10QQuickItem11grabToImageERK8QJSValueRK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  QTransform QQuickItem::itemTransform(QQuickItem * , bool * );
  fn _ZNK10QQuickItem13itemTransformEPS_Pb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> *mut c_void;
  // proto:  void QQuickItem::update();
  fn _ZN10QQuickItem6updateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickItem::setCursor(const QCursor & cursor);
  fn _ZN10QQuickItem9setCursorERK7QCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSharedPointer<QQuickItemGrabResult> QQuickItem::grabToImage(const QSize & targetSize);
  fn _ZN10QQuickItem11grabToImageERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRectF QQuickItem::clipRect();
  fn _ZNK10QQuickItem8clipRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickItem::ungrabMouse();
  fn _ZN10QQuickItem11ungrabMouseEv(qthis: u64 /* *mut c_void*/);
  // proto:  QRectF QQuickItem::mapRectToScene(const QRectF & rect);
  fn _ZNK10QQuickItem14mapRectToSceneERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QQuickItem::setRotation(qreal );
  fn _ZN10QQuickItem11setRotationEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  bool QQuickItem::hasFocus();
  fn _ZNK10QQuickItem8hasFocusEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQuickItem::setX(qreal );
  fn _ZN10QQuickItem4setXEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QString QQuickItem::state();
  fn _ZNK10QQuickItem5stateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickItem::setScale(qreal );
  fn _ZN10QQuickItem8setScaleEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QQuickItem::resetHeight();
  fn _ZN10QQuickItem11resetHeightEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickItem::QQuickItem(const QQuickItem & );
  fn _ZN10QQuickItemC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickItem::grabMouse();
  fn _ZN10QQuickItem9grabMouseEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickItem::setKeepTouchGrab(bool );
  fn _ZN10QQuickItem16setKeepTouchGrabEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QQuickItem::stackAfter(const QQuickItem * );
  fn _ZN10QQuickItem10stackAfterEPKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QQuickItem::baselineOffset();
  fn _ZNK10QQuickItem14baselineOffsetEv(qthis: u64 /* *mut c_void*/) -> c_double;
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem14opacityChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem14visibleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem13heightChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem19childrenRectChangedERK6QRectF(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem8yChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem19antialiasingChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem22visibleChildrenChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem12widthChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem15rotationChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem13smoothChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem15childrenChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem14enabledChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem21implicitHeightChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem13parentChangedEPS_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem8xChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem13windowChangedEP12QQuickWindow(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem11clipChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem12scaleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem12focusChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem12stateChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem20implicitWidthChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem18activeFocusChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem21baselineOffsetChangedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem23activeFocusOnTabChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickItem_SlotProxy_connect__ZN10QQuickItem8zChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQuickTransform)=1
#[derive(Default)]
pub struct QQuickTransform {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QQuickItem)=1
#[derive(Default)]
pub struct QQuickItem {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _childrenChanged: QQuickItem_childrenChanged_signal,
  pub _parentChanged: QQuickItem_parentChanged_signal,
  pub _stateChanged: QQuickItem_stateChanged_signal,
  pub _visibleChildrenChanged: QQuickItem_visibleChildrenChanged_signal,
  pub _transformOriginChanged: QQuickItem_transformOriginChanged_signal,
  pub _rotationChanged: QQuickItem_rotationChanged_signal,
  pub _antialiasingChanged: QQuickItem_antialiasingChanged_signal,
  pub _scaleChanged: QQuickItem_scaleChanged_signal,
  pub _heightChanged: QQuickItem_heightChanged_signal,
  pub _visibleChanged: QQuickItem_visibleChanged_signal,
  pub _clipChanged: QQuickItem_clipChanged_signal,
  pub _smoothChanged: QQuickItem_smoothChanged_signal,
  pub _widthChanged: QQuickItem_widthChanged_signal,
  pub _windowChanged: QQuickItem_windowChanged_signal,
  pub _opacityChanged: QQuickItem_opacityChanged_signal,
  pub _enabledChanged: QQuickItem_enabledChanged_signal,
  pub _xChanged: QQuickItem_xChanged_signal,
  pub _activeFocusOnTabChanged: QQuickItem_activeFocusOnTabChanged_signal,
  pub _implicitHeightChanged: QQuickItem_implicitHeightChanged_signal,
  pub _zChanged: QQuickItem_zChanged_signal,
  pub _focusChanged: QQuickItem_focusChanged_signal,
  pub _activeFocusChanged: QQuickItem_activeFocusChanged_signal,
  pub _yChanged: QQuickItem_yChanged_signal,
  pub _implicitWidthChanged: QQuickItem_implicitWidthChanged_signal,
  pub _childrenRectChanged: QQuickItem_childrenRectChanged_signal,
  pub _baselineOffsetChanged: QQuickItem_baselineOffsetChanged_signal,
}

impl /*struct*/ QQuickTransform {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQuickTransform {
    return QQuickTransform{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQuickTransform {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QQuickTransform {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QQuickTransform::~QQuickTransform();
impl /*struct*/ QQuickTransform {
  pub fn free<RetType, T: QQuickTransform_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQuickTransform_free<RetType> {
  fn free(self , rsthis: & QQuickTransform) -> RetType;
}

  // proto:  void QQuickTransform::~QQuickTransform();
impl<'a> /*trait*/ QQuickTransform_free<()> for () {
  fn free(self , rsthis: & QQuickTransform) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QQuickTransformD2Ev()};
     unsafe {_ZN15QQuickTransformD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QQuickTransform::metaObject();
impl /*struct*/ QQuickTransform {
  pub fn metaObject<RetType, T: QQuickTransform_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQuickTransform_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQuickTransform) -> RetType;
}

  // proto:  const QMetaObject * QQuickTransform::metaObject();
impl<'a> /*trait*/ QQuickTransform_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQuickTransform) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QQuickTransform10metaObjectEv()};
     unsafe {_ZNK15QQuickTransform10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickTransform::QQuickTransform(QObject * parent);
impl /*struct*/ QQuickTransform {
  pub fn new<T: QQuickTransform_new>(value: T) -> QQuickTransform {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQuickTransform_new {
  fn new(self) -> QQuickTransform;
}

  // proto:  void QQuickTransform::QQuickTransform(QObject * parent);
impl<'a> /*trait*/ QQuickTransform_new for (&'a QObject) {
  fn new(self) -> QQuickTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QQuickTransformC2EP7QObject()};
    let ctysz: c_int = unsafe{QQuickTransform_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QQuickTransformC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickTransform{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQuickTransform::appendToItem(QQuickItem * );
impl /*struct*/ QQuickTransform {
  pub fn appendToItem<RetType, T: QQuickTransform_appendToItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.appendToItem(self);
    // return 1;
  }
}

pub trait QQuickTransform_appendToItem<RetType> {
  fn appendToItem(self , rsthis: & QQuickTransform) -> RetType;
}

  // proto:  void QQuickTransform::appendToItem(QQuickItem * );
impl<'a> /*trait*/ QQuickTransform_appendToItem<()> for (&'a QQuickItem) {
  fn appendToItem(self , rsthis: & QQuickTransform) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QQuickTransform12appendToItemEP10QQuickItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QQuickTransform12appendToItemEP10QQuickItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickTransform::applyTo(QMatrix4x4 * matrix);
impl /*struct*/ QQuickTransform {
  pub fn applyTo<RetType, T: QQuickTransform_applyTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.applyTo(self);
    // return 1;
  }
}

pub trait QQuickTransform_applyTo<RetType> {
  fn applyTo(self , rsthis: & QQuickTransform) -> RetType;
}

  // proto:  void QQuickTransform::applyTo(QMatrix4x4 * matrix);
impl<'a> /*trait*/ QQuickTransform_applyTo<()> for (&'a QMatrix4x4) {
  fn applyTo(self , rsthis: & QQuickTransform) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QQuickTransform7applyToEP10QMatrix4x4()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK15QQuickTransform7applyToEP10QMatrix4x4(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickTransform::prependToItem(QQuickItem * );
impl /*struct*/ QQuickTransform {
  pub fn prependToItem<RetType, T: QQuickTransform_prependToItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prependToItem(self);
    // return 1;
  }
}

pub trait QQuickTransform_prependToItem<RetType> {
  fn prependToItem(self , rsthis: & QQuickTransform) -> RetType;
}

  // proto:  void QQuickTransform::prependToItem(QQuickItem * );
impl<'a> /*trait*/ QQuickTransform_prependToItem<()> for (&'a QQuickItem) {
  fn prependToItem(self , rsthis: & QQuickTransform) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QQuickTransform13prependToItemEP10QQuickItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QQuickTransform13prependToItemEP10QQuickItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QQuickItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQuickItem {
    return QQuickItem{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQuickItem {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QQuickItem {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  qreal QQuickItem::implicitWidth();
impl /*struct*/ QQuickItem {
  pub fn implicitWidth<RetType, T: QQuickItem_implicitWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.implicitWidth(self);
    // return 1;
  }
}

pub trait QQuickItem_implicitWidth<RetType> {
  fn implicitWidth(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  qreal QQuickItem::implicitWidth();
impl<'a> /*trait*/ QQuickItem_implicitWidth<f64> for () {
  fn implicitWidth(self , rsthis: & QQuickItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem13implicitWidthEv()};
    let mut ret = unsafe {_ZNK10QQuickItem13implicitWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QQuickItem::setImplicitWidth(qreal );
impl /*struct*/ QQuickItem {
  pub fn setImplicitWidth<RetType, T: QQuickItem_setImplicitWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setImplicitWidth(self);
    // return 1;
  }
}

pub trait QQuickItem_setImplicitWidth<RetType> {
  fn setImplicitWidth(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setImplicitWidth(qreal );
impl<'a> /*trait*/ QQuickItem_setImplicitWidth<()> for (f64) {
  fn setImplicitWidth(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem16setImplicitWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN10QQuickItem16setImplicitWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setBaselineOffset(qreal );
impl /*struct*/ QQuickItem {
  pub fn setBaselineOffset<RetType, T: QQuickItem_setBaselineOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBaselineOffset(self);
    // return 1;
  }
}

pub trait QQuickItem_setBaselineOffset<RetType> {
  fn setBaselineOffset(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setBaselineOffset(qreal );
impl<'a> /*trait*/ QQuickItem_setBaselineOffset<()> for (f64) {
  fn setBaselineOffset(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem17setBaselineOffsetEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN10QQuickItem17setBaselineOffsetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::resetWidth();
impl /*struct*/ QQuickItem {
  pub fn resetWidth<RetType, T: QQuickItem_resetWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetWidth(self);
    // return 1;
  }
}

pub trait QQuickItem_resetWidth<RetType> {
  fn resetWidth(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::resetWidth();
impl<'a> /*trait*/ QQuickItem_resetWidth<()> for () {
  fn resetWidth(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem10resetWidthEv()};
     unsafe {_ZN10QQuickItem10resetWidthEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QQuickItem::y();
impl /*struct*/ QQuickItem {
  pub fn y<RetType, T: QQuickItem_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QQuickItem_y<RetType> {
  fn y(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  qreal QQuickItem::y();
impl<'a> /*trait*/ QQuickItem_y<()> for () {
  fn y(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem1yEv()};
     unsafe {_ZNK10QQuickItem1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickItem::forceActiveFocus();
impl /*struct*/ QQuickItem {
  pub fn forceActiveFocus<RetType, T: QQuickItem_forceActiveFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.forceActiveFocus(self);
    // return 1;
  }
}

pub trait QQuickItem_forceActiveFocus<RetType> {
  fn forceActiveFocus(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::forceActiveFocus();
impl<'a> /*trait*/ QQuickItem_forceActiveFocus<()> for () {
  fn forceActiveFocus(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem16forceActiveFocusEv()};
     unsafe {_ZN10QQuickItem16forceActiveFocusEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setClip(bool );
impl /*struct*/ QQuickItem {
  pub fn setClip<RetType, T: QQuickItem_setClip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setClip(self);
    // return 1;
  }
}

pub trait QQuickItem_setClip<RetType> {
  fn setClip(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setClip(bool );
impl<'a> /*trait*/ QQuickItem_setClip<()> for (i8) {
  fn setClip(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem7setClipEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QQuickItem7setClipEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setHeight(qreal );
impl /*struct*/ QQuickItem {
  pub fn setHeight<RetType, T: QQuickItem_setHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHeight(self);
    // return 1;
  }
}

pub trait QQuickItem_setHeight<RetType> {
  fn setHeight(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setHeight(qreal );
impl<'a> /*trait*/ QQuickItem_setHeight<()> for (f64) {
  fn setHeight(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem9setHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN10QQuickItem9setHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setY(qreal );
impl /*struct*/ QQuickItem {
  pub fn setY<RetType, T: QQuickItem_setY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QQuickItem_setY<RetType> {
  fn setY(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setY(qreal );
impl<'a> /*trait*/ QQuickItem_setY<()> for (f64) {
  fn setY(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem4setYEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN10QQuickItem4setYEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QQuickItem::rotation();
impl /*struct*/ QQuickItem {
  pub fn rotation<RetType, T: QQuickItem_rotation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rotation(self);
    // return 1;
  }
}

pub trait QQuickItem_rotation<RetType> {
  fn rotation(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  qreal QQuickItem::rotation();
impl<'a> /*trait*/ QQuickItem_rotation<f64> for () {
  fn rotation(self , rsthis: & QQuickItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem8rotationEv()};
    let mut ret = unsafe {_ZNK10QQuickItem8rotationEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QQuickItem::opacity();
impl /*struct*/ QQuickItem {
  pub fn opacity<RetType, T: QQuickItem_opacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opacity(self);
    // return 1;
  }
}

pub trait QQuickItem_opacity<RetType> {
  fn opacity(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  qreal QQuickItem::opacity();
impl<'a> /*trait*/ QQuickItem_opacity<f64> for () {
  fn opacity(self , rsthis: & QQuickItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem7opacityEv()};
    let mut ret = unsafe {_ZNK10QQuickItem7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QQuickItem::implicitHeight();
impl /*struct*/ QQuickItem {
  pub fn implicitHeight<RetType, T: QQuickItem_implicitHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.implicitHeight(self);
    // return 1;
  }
}

pub trait QQuickItem_implicitHeight<RetType> {
  fn implicitHeight(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  qreal QQuickItem::implicitHeight();
impl<'a> /*trait*/ QQuickItem_implicitHeight<f64> for () {
  fn implicitHeight(self , rsthis: & QQuickItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem14implicitHeightEv()};
    let mut ret = unsafe {_ZNK10QQuickItem14implicitHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QQuickItem::isTextureProvider();
impl /*struct*/ QQuickItem {
  pub fn isTextureProvider<RetType, T: QQuickItem_isTextureProvider<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTextureProvider(self);
    // return 1;
  }
}

pub trait QQuickItem_isTextureProvider<RetType> {
  fn isTextureProvider(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::isTextureProvider();
impl<'a> /*trait*/ QQuickItem_isTextureProvider<i8> for () {
  fn isTextureProvider(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem17isTextureProviderEv()};
    let mut ret = unsafe {_ZNK10QQuickItem17isTextureProviderEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQuickItem::smooth();
impl /*struct*/ QQuickItem {
  pub fn smooth<RetType, T: QQuickItem_smooth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.smooth(self);
    // return 1;
  }
}

pub trait QQuickItem_smooth<RetType> {
  fn smooth(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::smooth();
impl<'a> /*trait*/ QQuickItem_smooth<i8> for () {
  fn smooth(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem6smoothEv()};
    let mut ret = unsafe {_ZNK10QQuickItem6smoothEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QQmlListProperty<QQuickTransform> QQuickItem::transform();
impl /*struct*/ QQuickItem {
  pub fn transform<RetType, T: QQuickItem_transform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transform(self);
    // return 1;
  }
}

pub trait QQuickItem_transform<RetType> {
  fn transform(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QQmlListProperty<QQuickTransform> QQuickItem::transform();
impl<'a> /*trait*/ QQuickItem_transform<()> for () {
  fn transform(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem9transformEv()};
     unsafe {_ZN10QQuickItem9transformEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickItem::stackBefore(const QQuickItem * );
impl /*struct*/ QQuickItem {
  pub fn stackBefore<RetType, T: QQuickItem_stackBefore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stackBefore(self);
    // return 1;
  }
}

pub trait QQuickItem_stackBefore<RetType> {
  fn stackBefore(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::stackBefore(const QQuickItem * );
impl<'a> /*trait*/ QQuickItem_stackBefore<()> for (&'a QQuickItem) {
  fn stackBefore(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem11stackBeforeEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQuickItem11stackBeforeEPKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setActiveFocusOnTab(bool );
impl /*struct*/ QQuickItem {
  pub fn setActiveFocusOnTab<RetType, T: QQuickItem_setActiveFocusOnTab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setActiveFocusOnTab(self);
    // return 1;
  }
}

pub trait QQuickItem_setActiveFocusOnTab<RetType> {
  fn setActiveFocusOnTab(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setActiveFocusOnTab(bool );
impl<'a> /*trait*/ QQuickItem_setActiveFocusOnTab<()> for (i8) {
  fn setActiveFocusOnTab(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem19setActiveFocusOnTabEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QQuickItem19setActiveFocusOnTabEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QQuickItem::scale();
impl /*struct*/ QQuickItem {
  pub fn scale<RetType, T: QQuickItem_scale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scale(self);
    // return 1;
  }
}

pub trait QQuickItem_scale<RetType> {
  fn scale(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  qreal QQuickItem::scale();
impl<'a> /*trait*/ QQuickItem_scale<f64> for () {
  fn scale(self , rsthis: & QQuickItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem5scaleEv()};
    let mut ret = unsafe {_ZNK10QQuickItem5scaleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QQuickItem::setEnabled(bool );
impl /*struct*/ QQuickItem {
  pub fn setEnabled<RetType, T: QQuickItem_setEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEnabled(self);
    // return 1;
  }
}

pub trait QQuickItem_setEnabled<RetType> {
  fn setEnabled(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setEnabled(bool );
impl<'a> /*trait*/ QQuickItem_setEnabled<()> for (i8) {
  fn setEnabled(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem10setEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QQuickItem10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setImplicitHeight(qreal );
impl /*struct*/ QQuickItem {
  pub fn setImplicitHeight<RetType, T: QQuickItem_setImplicitHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setImplicitHeight(self);
    // return 1;
  }
}

pub trait QQuickItem_setImplicitHeight<RetType> {
  fn setImplicitHeight(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setImplicitHeight(qreal );
impl<'a> /*trait*/ QQuickItem_setImplicitHeight<()> for (f64) {
  fn setImplicitHeight(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem17setImplicitHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN10QQuickItem17setImplicitHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setPosition(const QPointF & );
impl /*struct*/ QQuickItem {
  pub fn setPosition<RetType, T: QQuickItem_setPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPosition(self);
    // return 1;
  }
}

pub trait QQuickItem_setPosition<RetType> {
  fn setPosition(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setPosition(const QPointF & );
impl<'a> /*trait*/ QQuickItem_setPosition<()> for (&'a QPointF) {
  fn setPosition(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQuickItem11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setSmooth(bool );
impl /*struct*/ QQuickItem {
  pub fn setSmooth<RetType, T: QQuickItem_setSmooth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSmooth(self);
    // return 1;
  }
}

pub trait QQuickItem_setSmooth<RetType> {
  fn setSmooth(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setSmooth(bool );
impl<'a> /*trait*/ QQuickItem_setSmooth<()> for (i8) {
  fn setSmooth(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem9setSmoothEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QQuickItem9setSmoothEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QQuickItem::mapFromScene(const QPointF & point);
impl /*struct*/ QQuickItem {
  pub fn mapFromScene<RetType, T: QQuickItem_mapFromScene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene(self);
    // return 1;
  }
}

pub trait QQuickItem_mapFromScene<RetType> {
  fn mapFromScene(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QPointF QQuickItem::mapFromScene(const QPointF & point);
impl<'a> /*trait*/ QQuickItem_mapFromScene<QPointF> for (&'a QPointF) {
  fn mapFromScene(self , rsthis: & QQuickItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem12mapFromSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QQuickItem12mapFromSceneERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickItem::setAcceptHoverEvents(bool enabled);
impl /*struct*/ QQuickItem {
  pub fn setAcceptHoverEvents<RetType, T: QQuickItem_setAcceptHoverEvents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAcceptHoverEvents(self);
    // return 1;
  }
}

pub trait QQuickItem_setAcceptHoverEvents<RetType> {
  fn setAcceptHoverEvents(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setAcceptHoverEvents(bool enabled);
impl<'a> /*trait*/ QQuickItem_setAcceptHoverEvents<()> for (i8) {
  fn setAcceptHoverEvents(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem20setAcceptHoverEventsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QQuickItem20setAcceptHoverEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QQuickItem::transformOriginPoint();
impl /*struct*/ QQuickItem {
  pub fn transformOriginPoint<RetType, T: QQuickItem_transformOriginPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transformOriginPoint(self);
    // return 1;
  }
}

pub trait QQuickItem_transformOriginPoint<RetType> {
  fn transformOriginPoint(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QPointF QQuickItem::transformOriginPoint();
impl<'a> /*trait*/ QQuickItem_transformOriginPoint<QPointF> for () {
  fn transformOriginPoint(self , rsthis: & QQuickItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem20transformOriginPointEv()};
    let mut ret = unsafe {_ZNK10QQuickItem20transformOriginPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickItem::setState(const QString & );
impl /*struct*/ QQuickItem {
  pub fn setState<RetType, T: QQuickItem_setState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setState(self);
    // return 1;
  }
}

pub trait QQuickItem_setState<RetType> {
  fn setState(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setState(const QString & );
impl<'a> /*trait*/ QQuickItem_setState<()> for (&'a QString) {
  fn setState(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem8setStateERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQuickItem8setStateERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QQuickItem * QQuickItem::parentItem();
impl /*struct*/ QQuickItem {
  pub fn parentItem<RetType, T: QQuickItem_parentItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parentItem(self);
    // return 1;
  }
}

pub trait QQuickItem_parentItem<RetType> {
  fn parentItem(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QQuickItem * QQuickItem::parentItem();
impl<'a> /*trait*/ QQuickItem_parentItem<QQuickItem> for () {
  fn parentItem(self , rsthis: & QQuickItem) -> QQuickItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem10parentItemEv()};
    let mut ret = unsafe {_ZNK10QQuickItem10parentItemEv(rsthis.qclsinst)};
    let mut ret1 = QQuickItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QQuickItem::isFocusScope();
impl /*struct*/ QQuickItem {
  pub fn isFocusScope<RetType, T: QQuickItem_isFocusScope<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFocusScope(self);
    // return 1;
  }
}

pub trait QQuickItem_isFocusScope<RetType> {
  fn isFocusScope(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::isFocusScope();
impl<'a> /*trait*/ QQuickItem_isFocusScope<i8> for () {
  fn isFocusScope(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem12isFocusScopeEv()};
    let mut ret = unsafe {_ZNK10QQuickItem12isFocusScopeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQuickItem::contains(const QPointF & point);
impl /*struct*/ QQuickItem {
  pub fn contains<RetType, T: QQuickItem_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QQuickItem_contains<RetType> {
  fn contains(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::contains(const QPointF & point);
impl<'a> /*trait*/ QQuickItem_contains<i8> for (&'a QPointF) {
  fn contains(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QQuickItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRectF QQuickItem::boundingRect();
impl /*struct*/ QQuickItem {
  pub fn boundingRect<RetType, T: QQuickItem_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QQuickItem_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QRectF QQuickItem::boundingRect();
impl<'a> /*trait*/ QQuickItem_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QQuickItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem12boundingRectEv()};
    let mut ret = unsafe {_ZNK10QQuickItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickItem::resetAntialiasing();
impl /*struct*/ QQuickItem {
  pub fn resetAntialiasing<RetType, T: QQuickItem_resetAntialiasing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetAntialiasing(self);
    // return 1;
  }
}

pub trait QQuickItem_resetAntialiasing<RetType> {
  fn resetAntialiasing(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::resetAntialiasing();
impl<'a> /*trait*/ QQuickItem_resetAntialiasing<()> for () {
  fn resetAntialiasing(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem17resetAntialiasingEv()};
     unsafe {_ZN10QQuickItem17resetAntialiasingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QQuickItem::mapRectToItem(const QQuickItem * item, const QRectF & rect);
impl /*struct*/ QQuickItem {
  pub fn mapRectToItem<RetType, T: QQuickItem_mapRectToItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapRectToItem(self);
    // return 1;
  }
}

pub trait QQuickItem_mapRectToItem<RetType> {
  fn mapRectToItem(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QRectF QQuickItem::mapRectToItem(const QQuickItem * item, const QRectF & rect);
impl<'a> /*trait*/ QQuickItem_mapRectToItem<QRectF> for (&'a QQuickItem, &'a QRectF) {
  fn mapRectToItem(self , rsthis: & QQuickItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem13mapRectToItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QQuickItem13mapRectToItemEPKS_RK6QRectF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QQuickItem::x();
impl /*struct*/ QQuickItem {
  pub fn x<RetType, T: QQuickItem_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QQuickItem_x<RetType> {
  fn x(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  qreal QQuickItem::x();
impl<'a> /*trait*/ QQuickItem_x<()> for () {
  fn x(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem1xEv()};
     unsafe {_ZNK10QQuickItem1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QQuickItem::mapFromItem(const QQuickItem * item, const QPointF & point);
impl /*struct*/ QQuickItem {
  pub fn mapFromItem<RetType, T: QQuickItem_mapFromItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapFromItem(self);
    // return 1;
  }
}

pub trait QQuickItem_mapFromItem<RetType> {
  fn mapFromItem(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QPointF QQuickItem::mapFromItem(const QQuickItem * item, const QPointF & point);
impl<'a> /*trait*/ QQuickItem_mapFromItem<QPointF> for (&'a QQuickItem, &'a QPointF) {
  fn mapFromItem(self , rsthis: & QQuickItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem11mapFromItemEPKS_RK7QPointF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QQuickItem11mapFromItemEPKS_RK7QPointF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QQuickItem::isVisible();
impl /*struct*/ QQuickItem {
  pub fn isVisible<RetType, T: QQuickItem_isVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isVisible(self);
    // return 1;
  }
}

pub trait QQuickItem_isVisible<RetType> {
  fn isVisible(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::isVisible();
impl<'a> /*trait*/ QQuickItem_isVisible<i8> for () {
  fn isVisible(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem9isVisibleEv()};
    let mut ret = unsafe {_ZNK10QQuickItem9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSGTextureProvider * QQuickItem::textureProvider();
impl /*struct*/ QQuickItem {
  pub fn textureProvider<RetType, T: QQuickItem_textureProvider<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textureProvider(self);
    // return 1;
  }
}

pub trait QQuickItem_textureProvider<RetType> {
  fn textureProvider(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QSGTextureProvider * QQuickItem::textureProvider();
impl<'a> /*trait*/ QQuickItem_textureProvider<QSGTextureProvider> for () {
  fn textureProvider(self , rsthis: & QQuickItem) -> QSGTextureProvider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem15textureProviderEv()};
    let mut ret = unsafe {_ZNK10QQuickItem15textureProviderEv(rsthis.qclsinst)};
    let mut ret1 = QSGTextureProvider::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickItem::setZ(qreal );
impl /*struct*/ QQuickItem {
  pub fn setZ<RetType, T: QQuickItem_setZ<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setZ(self);
    // return 1;
  }
}

pub trait QQuickItem_setZ<RetType> {
  fn setZ(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setZ(qreal );
impl<'a> /*trait*/ QQuickItem_setZ<()> for (f64) {
  fn setZ(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem4setZEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN10QQuickItem4setZEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::unsetCursor();
impl /*struct*/ QQuickItem {
  pub fn unsetCursor<RetType, T: QQuickItem_unsetCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unsetCursor(self);
    // return 1;
  }
}

pub trait QQuickItem_unsetCursor<RetType> {
  fn unsetCursor(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::unsetCursor();
impl<'a> /*trait*/ QQuickItem_unsetCursor<()> for () {
  fn unsetCursor(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem11unsetCursorEv()};
     unsafe {_ZN10QQuickItem11unsetCursorEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QQuickItem::acceptHoverEvents();
impl /*struct*/ QQuickItem {
  pub fn acceptHoverEvents<RetType, T: QQuickItem_acceptHoverEvents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.acceptHoverEvents(self);
    // return 1;
  }
}

pub trait QQuickItem_acceptHoverEvents<RetType> {
  fn acceptHoverEvents(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::acceptHoverEvents();
impl<'a> /*trait*/ QQuickItem_acceptHoverEvents<i8> for () {
  fn acceptHoverEvents(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem17acceptHoverEventsEv()};
    let mut ret = unsafe {_ZNK10QQuickItem17acceptHoverEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQuickItem::setFiltersChildMouseEvents(bool filter);
impl /*struct*/ QQuickItem {
  pub fn setFiltersChildMouseEvents<RetType, T: QQuickItem_setFiltersChildMouseEvents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFiltersChildMouseEvents(self);
    // return 1;
  }
}

pub trait QQuickItem_setFiltersChildMouseEvents<RetType> {
  fn setFiltersChildMouseEvents(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setFiltersChildMouseEvents(bool filter);
impl<'a> /*trait*/ QQuickItem_setFiltersChildMouseEvents<()> for (i8) {
  fn setFiltersChildMouseEvents(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem26setFiltersChildMouseEventsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QQuickItem26setFiltersChildMouseEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QQuickItem::isEnabled();
impl /*struct*/ QQuickItem {
  pub fn isEnabled<RetType, T: QQuickItem_isEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEnabled(self);
    // return 1;
  }
}

pub trait QQuickItem_isEnabled<RetType> {
  fn isEnabled(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::isEnabled();
impl<'a> /*trait*/ QQuickItem_isEnabled<i8> for () {
  fn isEnabled(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem9isEnabledEv()};
    let mut ret = unsafe {_ZNK10QQuickItem9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPointF QQuickItem::mapToItem(const QQuickItem * item, const QPointF & point);
impl /*struct*/ QQuickItem {
  pub fn mapToItem<RetType, T: QQuickItem_mapToItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToItem(self);
    // return 1;
  }
}

pub trait QQuickItem_mapToItem<RetType> {
  fn mapToItem(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QPointF QQuickItem::mapToItem(const QQuickItem * item, const QPointF & point);
impl<'a> /*trait*/ QQuickItem_mapToItem<QPointF> for (&'a QQuickItem, &'a QPointF) {
  fn mapToItem(self , rsthis: & QQuickItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem9mapToItemEPKS_RK7QPointF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QQuickItem9mapToItemEPKS_RK7QPointF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QQuickItem::width();
impl /*struct*/ QQuickItem {
  pub fn width<RetType, T: QQuickItem_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QQuickItem_width<RetType> {
  fn width(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  qreal QQuickItem::width();
impl<'a> /*trait*/ QQuickItem_width<f64> for () {
  fn width(self , rsthis: & QQuickItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem5widthEv()};
    let mut ret = unsafe {_ZNK10QQuickItem5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPointF QQuickItem::mapToScene(const QPointF & point);
impl /*struct*/ QQuickItem {
  pub fn mapToScene<RetType, T: QQuickItem_mapToScene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToScene(self);
    // return 1;
  }
}

pub trait QQuickItem_mapToScene<RetType> {
  fn mapToScene(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QPointF QQuickItem::mapToScene(const QPointF & point);
impl<'a> /*trait*/ QQuickItem_mapToScene<QPointF> for (&'a QPointF) {
  fn mapToScene(self , rsthis: & QQuickItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem10mapToSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QQuickItem10mapToSceneERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QQuickItem::keepTouchGrab();
impl /*struct*/ QQuickItem {
  pub fn keepTouchGrab<RetType, T: QQuickItem_keepTouchGrab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keepTouchGrab(self);
    // return 1;
  }
}

pub trait QQuickItem_keepTouchGrab<RetType> {
  fn keepTouchGrab(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::keepTouchGrab();
impl<'a> /*trait*/ QQuickItem_keepTouchGrab<i8> for () {
  fn keepTouchGrab(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem13keepTouchGrabEv()};
    let mut ret = unsafe {_ZNK10QQuickItem13keepTouchGrabEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRectF QQuickItem::childrenRect();
impl /*struct*/ QQuickItem {
  pub fn childrenRect<RetType, T: QQuickItem_childrenRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childrenRect(self);
    // return 1;
  }
}

pub trait QQuickItem_childrenRect<RetType> {
  fn childrenRect(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QRectF QQuickItem::childrenRect();
impl<'a> /*trait*/ QQuickItem_childrenRect<QRectF> for () {
  fn childrenRect(self , rsthis: & QQuickItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem12childrenRectEv()};
    let mut ret = unsafe {_ZN10QQuickItem12childrenRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QQuickItem::metaObject();
impl /*struct*/ QQuickItem {
  pub fn metaObject<RetType, T: QQuickItem_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQuickItem_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  const QMetaObject * QQuickItem::metaObject();
impl<'a> /*trait*/ QQuickItem_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem10metaObjectEv()};
     unsafe {_ZNK10QQuickItem10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setKeepMouseGrab(bool );
impl /*struct*/ QQuickItem {
  pub fn setKeepMouseGrab<RetType, T: QQuickItem_setKeepMouseGrab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKeepMouseGrab(self);
    // return 1;
  }
}

pub trait QQuickItem_setKeepMouseGrab<RetType> {
  fn setKeepMouseGrab(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setKeepMouseGrab(bool );
impl<'a> /*trait*/ QQuickItem_setKeepMouseGrab<()> for (i8) {
  fn setKeepMouseGrab(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem16setKeepMouseGrabEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QQuickItem16setKeepMouseGrabEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QQuickItem::filtersChildMouseEvents();
impl /*struct*/ QQuickItem {
  pub fn filtersChildMouseEvents<RetType, T: QQuickItem_filtersChildMouseEvents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.filtersChildMouseEvents(self);
    // return 1;
  }
}

pub trait QQuickItem_filtersChildMouseEvents<RetType> {
  fn filtersChildMouseEvents(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::filtersChildMouseEvents();
impl<'a> /*trait*/ QQuickItem_filtersChildMouseEvents<i8> for () {
  fn filtersChildMouseEvents(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem23filtersChildMouseEventsEv()};
    let mut ret = unsafe {_ZNK10QQuickItem23filtersChildMouseEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQuickItem::hasActiveFocus();
impl /*struct*/ QQuickItem {
  pub fn hasActiveFocus<RetType, T: QQuickItem_hasActiveFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasActiveFocus(self);
    // return 1;
  }
}

pub trait QQuickItem_hasActiveFocus<RetType> {
  fn hasActiveFocus(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::hasActiveFocus();
impl<'a> /*trait*/ QQuickItem_hasActiveFocus<i8> for () {
  fn hasActiveFocus(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem14hasActiveFocusEv()};
    let mut ret = unsafe {_ZNK10QQuickItem14hasActiveFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQuickItem::isUnderMouse();
impl /*struct*/ QQuickItem {
  pub fn isUnderMouse<RetType, T: QQuickItem_isUnderMouse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUnderMouse(self);
    // return 1;
  }
}

pub trait QQuickItem_isUnderMouse<RetType> {
  fn isUnderMouse(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::isUnderMouse();
impl<'a> /*trait*/ QQuickItem_isUnderMouse<i8> for () {
  fn isUnderMouse(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem12isUnderMouseEv()};
    let mut ret = unsafe {_ZNK10QQuickItem12isUnderMouseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQuickItem::setTransformOriginPoint(const QPointF & );
impl /*struct*/ QQuickItem {
  pub fn setTransformOriginPoint<RetType, T: QQuickItem_setTransformOriginPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTransformOriginPoint(self);
    // return 1;
  }
}

pub trait QQuickItem_setTransformOriginPoint<RetType> {
  fn setTransformOriginPoint(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setTransformOriginPoint(const QPointF & );
impl<'a> /*trait*/ QQuickItem_setTransformOriginPoint<()> for (&'a QPointF) {
  fn setTransformOriginPoint(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem23setTransformOriginPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQuickItem23setTransformOriginPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::polish();
impl /*struct*/ QQuickItem {
  pub fn polish<RetType, T: QQuickItem_polish<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.polish(self);
    // return 1;
  }
}

pub trait QQuickItem_polish<RetType> {
  fn polish(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::polish();
impl<'a> /*trait*/ QQuickItem_polish<()> for () {
  fn polish(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem6polishEv()};
     unsafe {_ZN10QQuickItem6polishEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setParentItem(QQuickItem * parent);
impl /*struct*/ QQuickItem {
  pub fn setParentItem<RetType, T: QQuickItem_setParentItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setParentItem(self);
    // return 1;
  }
}

pub trait QQuickItem_setParentItem<RetType> {
  fn setParentItem(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setParentItem(QQuickItem * parent);
impl<'a> /*trait*/ QQuickItem_setParentItem<()> for (&'a QQuickItem) {
  fn setParentItem(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem13setParentItemEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQuickItem13setParentItemEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::QQuickItem(QQuickItem * parent);
impl /*struct*/ QQuickItem {
  pub fn new<T: QQuickItem_new>(value: T) -> QQuickItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQuickItem_new {
  fn new(self) -> QQuickItem;
}

  // proto:  void QQuickItem::QQuickItem(QQuickItem * parent);
impl<'a> /*trait*/ QQuickItem_new for (&'a QQuickItem) {
  fn new(self) -> QQuickItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItemC2EPS_()};
    let ctysz: c_int = unsafe{QQuickItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QQuickItemC2EPS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickItem{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQuickItem::setWidth(qreal );
impl /*struct*/ QQuickItem {
  pub fn setWidth<RetType, T: QQuickItem_setWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QQuickItem_setWidth<RetType> {
  fn setWidth(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setWidth(qreal );
impl<'a> /*trait*/ QQuickItem_setWidth<()> for (f64) {
  fn setWidth(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN10QQuickItem8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QQuickItem * QQuickItem::nextItemInFocusChain(bool forward);
impl /*struct*/ QQuickItem {
  pub fn nextItemInFocusChain<RetType, T: QQuickItem_nextItemInFocusChain<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nextItemInFocusChain(self);
    // return 1;
  }
}

pub trait QQuickItem_nextItemInFocusChain<RetType> {
  fn nextItemInFocusChain(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QQuickItem * QQuickItem::nextItemInFocusChain(bool forward);
impl<'a> /*trait*/ QQuickItem_nextItemInFocusChain<QQuickItem> for (i8) {
  fn nextItemInFocusChain(self , rsthis: & QQuickItem) -> QQuickItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem20nextItemInFocusChainEb()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZN10QQuickItem20nextItemInFocusChainEb(rsthis.qclsinst, arg0)};
    let mut ret1 = QQuickItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickItem::ungrabTouchPoints();
impl /*struct*/ QQuickItem {
  pub fn ungrabTouchPoints<RetType, T: QQuickItem_ungrabTouchPoints<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ungrabTouchPoints(self);
    // return 1;
  }
}

pub trait QQuickItem_ungrabTouchPoints<RetType> {
  fn ungrabTouchPoints(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::ungrabTouchPoints();
impl<'a> /*trait*/ QQuickItem_ungrabTouchPoints<()> for () {
  fn ungrabTouchPoints(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem17ungrabTouchPointsEv()};
     unsafe {_ZN10QQuickItem17ungrabTouchPointsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QQuickItem::keepMouseGrab();
impl /*struct*/ QQuickItem {
  pub fn keepMouseGrab<RetType, T: QQuickItem_keepMouseGrab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keepMouseGrab(self);
    // return 1;
  }
}

pub trait QQuickItem_keepMouseGrab<RetType> {
  fn keepMouseGrab(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::keepMouseGrab();
impl<'a> /*trait*/ QQuickItem_keepMouseGrab<i8> for () {
  fn keepMouseGrab(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem13keepMouseGrabEv()};
    let mut ret = unsafe {_ZNK10QQuickItem13keepMouseGrabEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRectF QQuickItem::mapRectFromScene(const QRectF & rect);
impl /*struct*/ QQuickItem {
  pub fn mapRectFromScene<RetType, T: QQuickItem_mapRectFromScene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromScene(self);
    // return 1;
  }
}

pub trait QQuickItem_mapRectFromScene<RetType> {
  fn mapRectFromScene(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QRectF QQuickItem::mapRectFromScene(const QRectF & rect);
impl<'a> /*trait*/ QQuickItem_mapRectFromScene<QRectF> for (&'a QRectF) {
  fn mapRectFromScene(self , rsthis: & QQuickItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem16mapRectFromSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QQuickItem16mapRectFromSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QQuickItem::position();
impl /*struct*/ QQuickItem {
  pub fn position<RetType, T: QQuickItem_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QQuickItem_position<RetType> {
  fn position(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QPointF QQuickItem::position();
impl<'a> /*trait*/ QQuickItem_position<QPointF> for () {
  fn position(self , rsthis: & QQuickItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem8positionEv()};
    let mut ret = unsafe {_ZNK10QQuickItem8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QQuickItem::height();
impl /*struct*/ QQuickItem {
  pub fn height<RetType, T: QQuickItem_height<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QQuickItem_height<RetType> {
  fn height(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  qreal QQuickItem::height();
impl<'a> /*trait*/ QQuickItem_height<f64> for () {
  fn height(self , rsthis: & QQuickItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem6heightEv()};
    let mut ret = unsafe {_ZNK10QQuickItem6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QQuickItem * QQuickItem::childAt(qreal x, qreal y);
impl /*struct*/ QQuickItem {
  pub fn childAt<RetType, T: QQuickItem_childAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childAt(self);
    // return 1;
  }
}

pub trait QQuickItem_childAt<RetType> {
  fn childAt(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QQuickItem * QQuickItem::childAt(qreal x, qreal y);
impl<'a> /*trait*/ QQuickItem_childAt<QQuickItem> for (f64, f64) {
  fn childAt(self , rsthis: & QQuickItem) -> QQuickItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem7childAtEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK10QQuickItem7childAtEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QQuickItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QCursor QQuickItem::cursor();
impl /*struct*/ QQuickItem {
  pub fn cursor<RetType, T: QQuickItem_cursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursor(self);
    // return 1;
  }
}

pub trait QQuickItem_cursor<RetType> {
  fn cursor(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QCursor QQuickItem::cursor();
impl<'a> /*trait*/ QQuickItem_cursor<QCursor> for () {
  fn cursor(self , rsthis: & QQuickItem) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem6cursorEv()};
    let mut ret = unsafe {_ZNK10QQuickItem6cursorEv(rsthis.qclsinst)};
    let mut ret1 = QCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QQuickItem::clip();
impl /*struct*/ QQuickItem {
  pub fn clip<RetType, T: QQuickItem_clip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clip(self);
    // return 1;
  }
}

pub trait QQuickItem_clip<RetType> {
  fn clip(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::clip();
impl<'a> /*trait*/ QQuickItem_clip<i8> for () {
  fn clip(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem4clipEv()};
    let mut ret = unsafe {_ZNK10QQuickItem4clipEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQuickItem::antialiasing();
impl /*struct*/ QQuickItem {
  pub fn antialiasing<RetType, T: QQuickItem_antialiasing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.antialiasing(self);
    // return 1;
  }
}

pub trait QQuickItem_antialiasing<RetType> {
  fn antialiasing(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::antialiasing();
impl<'a> /*trait*/ QQuickItem_antialiasing<i8> for () {
  fn antialiasing(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem12antialiasingEv()};
    let mut ret = unsafe {_ZNK10QQuickItem12antialiasingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QQuickWindow * QQuickItem::window();
impl /*struct*/ QQuickItem {
  pub fn window<RetType, T: QQuickItem_window<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QQuickItem_window<RetType> {
  fn window(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QQuickWindow * QQuickItem::window();
impl<'a> /*trait*/ QQuickItem_window<QQuickWindow> for () {
  fn window(self , rsthis: & QQuickItem) -> QQuickWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem6windowEv()};
    let mut ret = unsafe {_ZNK10QQuickItem6windowEv(rsthis.qclsinst)};
    let mut ret1 = QQuickWindow::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QQuickItem::mapRectFromItem(const QQuickItem * item, const QRectF & rect);
impl /*struct*/ QQuickItem {
  pub fn mapRectFromItem<RetType, T: QQuickItem_mapRectFromItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromItem(self);
    // return 1;
  }
}

pub trait QQuickItem_mapRectFromItem<RetType> {
  fn mapRectFromItem(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QRectF QQuickItem::mapRectFromItem(const QQuickItem * item, const QRectF & rect);
impl<'a> /*trait*/ QQuickItem_mapRectFromItem<QRectF> for (&'a QQuickItem, &'a QRectF) {
  fn mapRectFromItem(self , rsthis: & QQuickItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem15mapRectFromItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QQuickItem15mapRectFromItemEPKS_RK6QRectF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickItem::setVisible(bool );
impl /*struct*/ QQuickItem {
  pub fn setVisible<RetType, T: QQuickItem_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QQuickItem_setVisible<RetType> {
  fn setVisible(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setVisible(bool );
impl<'a> /*trait*/ QQuickItem_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QQuickItem10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setFocus(bool );
impl /*struct*/ QQuickItem {
  pub fn setFocus<RetType, T: QQuickItem_setFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFocus(self);
    // return 1;
  }
}

pub trait QQuickItem_setFocus<RetType> {
  fn setFocus(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setFocus(bool );
impl<'a> /*trait*/ QQuickItem_setFocus<()> for (i8) {
  fn setFocus(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem8setFocusEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QQuickItem8setFocusEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QQuickItem::activeFocusOnTab();
impl /*struct*/ QQuickItem {
  pub fn activeFocusOnTab<RetType, T: QQuickItem_activeFocusOnTab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeFocusOnTab(self);
    // return 1;
  }
}

pub trait QQuickItem_activeFocusOnTab<RetType> {
  fn activeFocusOnTab(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::activeFocusOnTab();
impl<'a> /*trait*/ QQuickItem_activeFocusOnTab<i8> for () {
  fn activeFocusOnTab(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem16activeFocusOnTabEv()};
    let mut ret = unsafe {_ZNK10QQuickItem16activeFocusOnTabEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QQuickItem::z();
impl /*struct*/ QQuickItem {
  pub fn z<RetType, T: QQuickItem_z<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.z(self);
    // return 1;
  }
}

pub trait QQuickItem_z<RetType> {
  fn z(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  qreal QQuickItem::z();
impl<'a> /*trait*/ QQuickItem_z<f64> for () {
  fn z(self , rsthis: & QQuickItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem1zEv()};
    let mut ret = unsafe {_ZNK10QQuickItem1zEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QQuickItem::setOpacity(qreal );
impl /*struct*/ QQuickItem {
  pub fn setOpacity<RetType, T: QQuickItem_setOpacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpacity(self);
    // return 1;
  }
}

pub trait QQuickItem_setOpacity<RetType> {
  fn setOpacity(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setOpacity(qreal );
impl<'a> /*trait*/ QQuickItem_setOpacity<()> for (f64) {
  fn setOpacity(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN10QQuickItem10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setAntialiasing(bool );
impl /*struct*/ QQuickItem {
  pub fn setAntialiasing<RetType, T: QQuickItem_setAntialiasing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAntialiasing(self);
    // return 1;
  }
}

pub trait QQuickItem_setAntialiasing<RetType> {
  fn setAntialiasing(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setAntialiasing(bool );
impl<'a> /*trait*/ QQuickItem_setAntialiasing<()> for (i8) {
  fn setAntialiasing(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem15setAntialiasingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QQuickItem15setAntialiasingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QQuickItem *> QQuickItem::childItems();
impl /*struct*/ QQuickItem {
  pub fn childItems<RetType, T: QQuickItem_childItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childItems(self);
    // return 1;
  }
}

pub trait QQuickItem_childItems<RetType> {
  fn childItems(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QList<QQuickItem *> QQuickItem::childItems();
impl<'a> /*trait*/ QQuickItem_childItems<()> for () {
  fn childItems(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem10childItemsEv()};
     unsafe {_ZNK10QQuickItem10childItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setSize(const QSizeF & size);
impl /*struct*/ QQuickItem {
  pub fn setSize<RetType, T: QQuickItem_setSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSize(self);
    // return 1;
  }
}

pub trait QQuickItem_setSize<RetType> {
  fn setSize(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setSize(const QSizeF & size);
impl<'a> /*trait*/ QQuickItem_setSize<()> for (&'a QSizeF) {
  fn setSize(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem7setSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQuickItem7setSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::~QQuickItem();
impl /*struct*/ QQuickItem {
  pub fn free<RetType, T: QQuickItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQuickItem_free<RetType> {
  fn free(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::~QQuickItem();
impl<'a> /*trait*/ QQuickItem_free<()> for () {
  fn free(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItemD2Ev()};
     unsafe {_ZN10QQuickItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QQuickItem * QQuickItem::scopedFocusItem();
impl /*struct*/ QQuickItem {
  pub fn scopedFocusItem<RetType, T: QQuickItem_scopedFocusItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scopedFocusItem(self);
    // return 1;
  }
}

pub trait QQuickItem_scopedFocusItem<RetType> {
  fn scopedFocusItem(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QQuickItem * QQuickItem::scopedFocusItem();
impl<'a> /*trait*/ QQuickItem_scopedFocusItem<QQuickItem> for () {
  fn scopedFocusItem(self , rsthis: & QQuickItem) -> QQuickItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem15scopedFocusItemEv()};
    let mut ret = unsafe {_ZNK10QQuickItem15scopedFocusItemEv(rsthis.qclsinst)};
    let mut ret1 = QQuickItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QQuickItem::grabToImage(const QJSValue & callback, const QSize & targetSize);
impl /*struct*/ QQuickItem {
  pub fn grabToImage<RetType, T: QQuickItem_grabToImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grabToImage(self);
    // return 1;
  }
}

pub trait QQuickItem_grabToImage<RetType> {
  fn grabToImage(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::grabToImage(const QJSValue & callback, const QSize & targetSize);
impl<'a> /*trait*/ QQuickItem_grabToImage<i8> for (&'a QJSValue, &'a QSize) {
  fn grabToImage(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem11grabToImageERK8QJSValueRK5QSize()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QQuickItem11grabToImageERK8QJSValueRK5QSize(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTransform QQuickItem::itemTransform(QQuickItem * , bool * );
impl /*struct*/ QQuickItem {
  pub fn itemTransform<RetType, T: QQuickItem_itemTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemTransform(self);
    // return 1;
  }
}

pub trait QQuickItem_itemTransform<RetType> {
  fn itemTransform(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QTransform QQuickItem::itemTransform(QQuickItem * , bool * );
impl<'a> /*trait*/ QQuickItem_itemTransform<QTransform> for (&'a QQuickItem, &'a mut Vec<i8>) {
  fn itemTransform(self , rsthis: & QQuickItem) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem13itemTransformEPS_Pb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK10QQuickItem13itemTransformEPS_Pb(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickItem::update();
impl /*struct*/ QQuickItem {
  pub fn update<RetType, T: QQuickItem_update<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.update(self);
    // return 1;
  }
}

pub trait QQuickItem_update<RetType> {
  fn update(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::update();
impl<'a> /*trait*/ QQuickItem_update<()> for () {
  fn update(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem6updateEv()};
     unsafe {_ZN10QQuickItem6updateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setCursor(const QCursor & cursor);
impl /*struct*/ QQuickItem {
  pub fn setCursor<RetType, T: QQuickItem_setCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCursor(self);
    // return 1;
  }
}

pub trait QQuickItem_setCursor<RetType> {
  fn setCursor(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setCursor(const QCursor & cursor);
impl<'a> /*trait*/ QQuickItem_setCursor<()> for (&'a QCursor) {
  fn setCursor(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem9setCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQuickItem9setCursorERK7QCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSharedPointer<QQuickItemGrabResult> QQuickItem::grabToImage(const QSize & targetSize);
impl<'a> /*trait*/ QQuickItem_grabToImage<()> for (&'a QSize) {
  fn grabToImage(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem11grabToImageERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQuickItem11grabToImageERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QQuickItem::clipRect();
impl /*struct*/ QQuickItem {
  pub fn clipRect<RetType, T: QQuickItem_clipRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clipRect(self);
    // return 1;
  }
}

pub trait QQuickItem_clipRect<RetType> {
  fn clipRect(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QRectF QQuickItem::clipRect();
impl<'a> /*trait*/ QQuickItem_clipRect<QRectF> for () {
  fn clipRect(self , rsthis: & QQuickItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem8clipRectEv()};
    let mut ret = unsafe {_ZNK10QQuickItem8clipRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickItem::ungrabMouse();
impl /*struct*/ QQuickItem {
  pub fn ungrabMouse<RetType, T: QQuickItem_ungrabMouse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ungrabMouse(self);
    // return 1;
  }
}

pub trait QQuickItem_ungrabMouse<RetType> {
  fn ungrabMouse(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::ungrabMouse();
impl<'a> /*trait*/ QQuickItem_ungrabMouse<()> for () {
  fn ungrabMouse(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem11ungrabMouseEv()};
     unsafe {_ZN10QQuickItem11ungrabMouseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QQuickItem::mapRectToScene(const QRectF & rect);
impl /*struct*/ QQuickItem {
  pub fn mapRectToScene<RetType, T: QQuickItem_mapRectToScene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapRectToScene(self);
    // return 1;
  }
}

pub trait QQuickItem_mapRectToScene<RetType> {
  fn mapRectToScene(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QRectF QQuickItem::mapRectToScene(const QRectF & rect);
impl<'a> /*trait*/ QQuickItem_mapRectToScene<QRectF> for (&'a QRectF) {
  fn mapRectToScene(self , rsthis: & QQuickItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem14mapRectToSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QQuickItem14mapRectToSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickItem::setRotation(qreal );
impl /*struct*/ QQuickItem {
  pub fn setRotation<RetType, T: QQuickItem_setRotation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRotation(self);
    // return 1;
  }
}

pub trait QQuickItem_setRotation<RetType> {
  fn setRotation(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setRotation(qreal );
impl<'a> /*trait*/ QQuickItem_setRotation<()> for (f64) {
  fn setRotation(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem11setRotationEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN10QQuickItem11setRotationEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QQuickItem::hasFocus();
impl /*struct*/ QQuickItem {
  pub fn hasFocus<RetType, T: QQuickItem_hasFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasFocus(self);
    // return 1;
  }
}

pub trait QQuickItem_hasFocus<RetType> {
  fn hasFocus(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  bool QQuickItem::hasFocus();
impl<'a> /*trait*/ QQuickItem_hasFocus<i8> for () {
  fn hasFocus(self , rsthis: & QQuickItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem8hasFocusEv()};
    let mut ret = unsafe {_ZNK10QQuickItem8hasFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQuickItem::setX(qreal );
impl /*struct*/ QQuickItem {
  pub fn setX<RetType, T: QQuickItem_setX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QQuickItem_setX<RetType> {
  fn setX(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setX(qreal );
impl<'a> /*trait*/ QQuickItem_setX<()> for (f64) {
  fn setX(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem4setXEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN10QQuickItem4setXEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QQuickItem::state();
impl /*struct*/ QQuickItem {
  pub fn state<RetType, T: QQuickItem_state<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.state(self);
    // return 1;
  }
}

pub trait QQuickItem_state<RetType> {
  fn state(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  QString QQuickItem::state();
impl<'a> /*trait*/ QQuickItem_state<QString> for () {
  fn state(self , rsthis: & QQuickItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem5stateEv()};
    let mut ret = unsafe {_ZNK10QQuickItem5stateEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickItem::setScale(qreal );
impl /*struct*/ QQuickItem {
  pub fn setScale<RetType, T: QQuickItem_setScale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScale(self);
    // return 1;
  }
}

pub trait QQuickItem_setScale<RetType> {
  fn setScale(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setScale(qreal );
impl<'a> /*trait*/ QQuickItem_setScale<()> for (f64) {
  fn setScale(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem8setScaleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN10QQuickItem8setScaleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::resetHeight();
impl /*struct*/ QQuickItem {
  pub fn resetHeight<RetType, T: QQuickItem_resetHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetHeight(self);
    // return 1;
  }
}

pub trait QQuickItem_resetHeight<RetType> {
  fn resetHeight(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::resetHeight();
impl<'a> /*trait*/ QQuickItem_resetHeight<()> for () {
  fn resetHeight(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem11resetHeightEv()};
     unsafe {_ZN10QQuickItem11resetHeightEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickItem::grabMouse();
impl /*struct*/ QQuickItem {
  pub fn grabMouse<RetType, T: QQuickItem_grabMouse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grabMouse(self);
    // return 1;
  }
}

pub trait QQuickItem_grabMouse<RetType> {
  fn grabMouse(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::grabMouse();
impl<'a> /*trait*/ QQuickItem_grabMouse<()> for () {
  fn grabMouse(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem9grabMouseEv()};
     unsafe {_ZN10QQuickItem9grabMouseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickItem::setKeepTouchGrab(bool );
impl /*struct*/ QQuickItem {
  pub fn setKeepTouchGrab<RetType, T: QQuickItem_setKeepTouchGrab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKeepTouchGrab(self);
    // return 1;
  }
}

pub trait QQuickItem_setKeepTouchGrab<RetType> {
  fn setKeepTouchGrab(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::setKeepTouchGrab(bool );
impl<'a> /*trait*/ QQuickItem_setKeepTouchGrab<()> for (i8) {
  fn setKeepTouchGrab(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem16setKeepTouchGrabEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QQuickItem16setKeepTouchGrabEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickItem::stackAfter(const QQuickItem * );
impl /*struct*/ QQuickItem {
  pub fn stackAfter<RetType, T: QQuickItem_stackAfter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stackAfter(self);
    // return 1;
  }
}

pub trait QQuickItem_stackAfter<RetType> {
  fn stackAfter(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  void QQuickItem::stackAfter(const QQuickItem * );
impl<'a> /*trait*/ QQuickItem_stackAfter<()> for (&'a QQuickItem) {
  fn stackAfter(self , rsthis: & QQuickItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickItem10stackAfterEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQuickItem10stackAfterEPKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QQuickItem::baselineOffset();
impl /*struct*/ QQuickItem {
  pub fn baselineOffset<RetType, T: QQuickItem_baselineOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.baselineOffset(self);
    // return 1;
  }
}

pub trait QQuickItem_baselineOffset<RetType> {
  fn baselineOffset(self , rsthis: & QQuickItem) -> RetType;
}

  // proto:  qreal QQuickItem::baselineOffset();
impl<'a> /*trait*/ QQuickItem_baselineOffset<f64> for () {
  fn baselineOffset(self , rsthis: & QQuickItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickItem14baselineOffsetEv()};
    let mut ret = unsafe {_ZNK10QQuickItem14baselineOffsetEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

#[derive(Default)] // for QQuickItem_childrenChanged
pub struct QQuickItem_childrenChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn childrenChanged(&self) -> QQuickItem_childrenChanged_signal {
     return QQuickItem_childrenChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_childrenChanged_signal {
  pub fn connect<T: QQuickItem_childrenChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_childrenChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_childrenChanged_signal);
}

#[derive(Default)] // for QQuickItem_parentChanged
pub struct QQuickItem_parentChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn parentChanged(&self) -> QQuickItem_parentChanged_signal {
     return QQuickItem_parentChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_parentChanged_signal {
  pub fn connect<T: QQuickItem_parentChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_parentChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_parentChanged_signal);
}

#[derive(Default)] // for QQuickItem_stateChanged
pub struct QQuickItem_stateChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn stateChanged(&self) -> QQuickItem_stateChanged_signal {
     return QQuickItem_stateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_stateChanged_signal {
  pub fn connect<T: QQuickItem_stateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_stateChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_stateChanged_signal);
}

#[derive(Default)] // for QQuickItem_visibleChildrenChanged
pub struct QQuickItem_visibleChildrenChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn visibleChildrenChanged(&self) -> QQuickItem_visibleChildrenChanged_signal {
     return QQuickItem_visibleChildrenChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_visibleChildrenChanged_signal {
  pub fn connect<T: QQuickItem_visibleChildrenChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_visibleChildrenChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_visibleChildrenChanged_signal);
}

#[derive(Default)] // for QQuickItem_transformOriginChanged
pub struct QQuickItem_transformOriginChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn transformOriginChanged(&self) -> QQuickItem_transformOriginChanged_signal {
     return QQuickItem_transformOriginChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_transformOriginChanged_signal {
  pub fn connect<T: QQuickItem_transformOriginChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_transformOriginChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_transformOriginChanged_signal);
}

#[derive(Default)] // for QQuickItem_rotationChanged
pub struct QQuickItem_rotationChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn rotationChanged(&self) -> QQuickItem_rotationChanged_signal {
     return QQuickItem_rotationChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_rotationChanged_signal {
  pub fn connect<T: QQuickItem_rotationChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_rotationChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_rotationChanged_signal);
}

#[derive(Default)] // for QQuickItem_antialiasingChanged
pub struct QQuickItem_antialiasingChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn antialiasingChanged(&self) -> QQuickItem_antialiasingChanged_signal {
     return QQuickItem_antialiasingChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_antialiasingChanged_signal {
  pub fn connect<T: QQuickItem_antialiasingChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_antialiasingChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_antialiasingChanged_signal);
}

#[derive(Default)] // for QQuickItem_scaleChanged
pub struct QQuickItem_scaleChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn scaleChanged(&self) -> QQuickItem_scaleChanged_signal {
     return QQuickItem_scaleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_scaleChanged_signal {
  pub fn connect<T: QQuickItem_scaleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_scaleChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_scaleChanged_signal);
}

#[derive(Default)] // for QQuickItem_heightChanged
pub struct QQuickItem_heightChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn heightChanged(&self) -> QQuickItem_heightChanged_signal {
     return QQuickItem_heightChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_heightChanged_signal {
  pub fn connect<T: QQuickItem_heightChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_heightChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_heightChanged_signal);
}

#[derive(Default)] // for QQuickItem_visibleChanged
pub struct QQuickItem_visibleChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn visibleChanged(&self) -> QQuickItem_visibleChanged_signal {
     return QQuickItem_visibleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_visibleChanged_signal {
  pub fn connect<T: QQuickItem_visibleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_visibleChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_visibleChanged_signal);
}

#[derive(Default)] // for QQuickItem_clipChanged
pub struct QQuickItem_clipChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn clipChanged(&self) -> QQuickItem_clipChanged_signal {
     return QQuickItem_clipChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_clipChanged_signal {
  pub fn connect<T: QQuickItem_clipChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_clipChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_clipChanged_signal);
}

#[derive(Default)] // for QQuickItem_smoothChanged
pub struct QQuickItem_smoothChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn smoothChanged(&self) -> QQuickItem_smoothChanged_signal {
     return QQuickItem_smoothChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_smoothChanged_signal {
  pub fn connect<T: QQuickItem_smoothChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_smoothChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_smoothChanged_signal);
}

#[derive(Default)] // for QQuickItem_widthChanged
pub struct QQuickItem_widthChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn widthChanged(&self) -> QQuickItem_widthChanged_signal {
     return QQuickItem_widthChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_widthChanged_signal {
  pub fn connect<T: QQuickItem_widthChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_widthChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_widthChanged_signal);
}

#[derive(Default)] // for QQuickItem_windowChanged
pub struct QQuickItem_windowChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn windowChanged(&self) -> QQuickItem_windowChanged_signal {
     return QQuickItem_windowChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_windowChanged_signal {
  pub fn connect<T: QQuickItem_windowChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_windowChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_windowChanged_signal);
}

#[derive(Default)] // for QQuickItem_opacityChanged
pub struct QQuickItem_opacityChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn opacityChanged(&self) -> QQuickItem_opacityChanged_signal {
     return QQuickItem_opacityChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_opacityChanged_signal {
  pub fn connect<T: QQuickItem_opacityChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_opacityChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_opacityChanged_signal);
}

#[derive(Default)] // for QQuickItem_enabledChanged
pub struct QQuickItem_enabledChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn enabledChanged(&self) -> QQuickItem_enabledChanged_signal {
     return QQuickItem_enabledChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_enabledChanged_signal {
  pub fn connect<T: QQuickItem_enabledChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_enabledChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_enabledChanged_signal);
}

#[derive(Default)] // for QQuickItem_xChanged
pub struct QQuickItem_xChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn xChanged(&self) -> QQuickItem_xChanged_signal {
     return QQuickItem_xChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_xChanged_signal {
  pub fn connect<T: QQuickItem_xChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_xChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_xChanged_signal);
}

#[derive(Default)] // for QQuickItem_activeFocusOnTabChanged
pub struct QQuickItem_activeFocusOnTabChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn activeFocusOnTabChanged(&self) -> QQuickItem_activeFocusOnTabChanged_signal {
     return QQuickItem_activeFocusOnTabChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_activeFocusOnTabChanged_signal {
  pub fn connect<T: QQuickItem_activeFocusOnTabChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_activeFocusOnTabChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_activeFocusOnTabChanged_signal);
}

#[derive(Default)] // for QQuickItem_implicitHeightChanged
pub struct QQuickItem_implicitHeightChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn implicitHeightChanged(&self) -> QQuickItem_implicitHeightChanged_signal {
     return QQuickItem_implicitHeightChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_implicitHeightChanged_signal {
  pub fn connect<T: QQuickItem_implicitHeightChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_implicitHeightChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_implicitHeightChanged_signal);
}

#[derive(Default)] // for QQuickItem_zChanged
pub struct QQuickItem_zChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn zChanged(&self) -> QQuickItem_zChanged_signal {
     return QQuickItem_zChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_zChanged_signal {
  pub fn connect<T: QQuickItem_zChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_zChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_zChanged_signal);
}

#[derive(Default)] // for QQuickItem_focusChanged
pub struct QQuickItem_focusChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn focusChanged(&self) -> QQuickItem_focusChanged_signal {
     return QQuickItem_focusChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_focusChanged_signal {
  pub fn connect<T: QQuickItem_focusChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_focusChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_focusChanged_signal);
}

#[derive(Default)] // for QQuickItem_activeFocusChanged
pub struct QQuickItem_activeFocusChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn activeFocusChanged(&self) -> QQuickItem_activeFocusChanged_signal {
     return QQuickItem_activeFocusChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_activeFocusChanged_signal {
  pub fn connect<T: QQuickItem_activeFocusChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_activeFocusChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_activeFocusChanged_signal);
}

#[derive(Default)] // for QQuickItem_yChanged
pub struct QQuickItem_yChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn yChanged(&self) -> QQuickItem_yChanged_signal {
     return QQuickItem_yChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_yChanged_signal {
  pub fn connect<T: QQuickItem_yChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_yChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_yChanged_signal);
}

#[derive(Default)] // for QQuickItem_implicitWidthChanged
pub struct QQuickItem_implicitWidthChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn implicitWidthChanged(&self) -> QQuickItem_implicitWidthChanged_signal {
     return QQuickItem_implicitWidthChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_implicitWidthChanged_signal {
  pub fn connect<T: QQuickItem_implicitWidthChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_implicitWidthChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_implicitWidthChanged_signal);
}

#[derive(Default)] // for QQuickItem_childrenRectChanged
pub struct QQuickItem_childrenRectChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn childrenRectChanged(&self) -> QQuickItem_childrenRectChanged_signal {
     return QQuickItem_childrenRectChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_childrenRectChanged_signal {
  pub fn connect<T: QQuickItem_childrenRectChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_childrenRectChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_childrenRectChanged_signal);
}

#[derive(Default)] // for QQuickItem_baselineOffsetChanged
pub struct QQuickItem_baselineOffsetChanged_signal{poi:u64}
impl /* struct */ QQuickItem {
  pub fn baselineOffsetChanged(&self) -> QQuickItem_baselineOffsetChanged_signal {
     return QQuickItem_baselineOffsetChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickItem_baselineOffsetChanged_signal {
  pub fn connect<T: QQuickItem_baselineOffsetChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickItem_baselineOffsetChanged_signal_connect {
  fn connect(self, sigthis: QQuickItem_baselineOffsetChanged_signal);
}

// opacityChanged()
extern fn QQuickItem_opacityChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_opacityChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_opacityChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_opacityChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_opacityChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem14opacityChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_opacityChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_opacityChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_opacityChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem14opacityChangedEv(arg0, arg1, arg2)};
  }
}
// visibleChanged()
extern fn QQuickItem_visibleChanged_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_visibleChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_visibleChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_visibleChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_visibleChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem14visibleChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_visibleChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_visibleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_visibleChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem14visibleChangedEv(arg0, arg1, arg2)};
  }
}
// heightChanged()
extern fn QQuickItem_heightChanged_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_heightChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_heightChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_heightChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_heightChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem13heightChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_heightChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_heightChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_heightChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem13heightChangedEv(arg0, arg1, arg2)};
  }
}
// childrenRectChanged(const class QRectF &)
extern fn QQuickItem_childrenRectChanged_signal_connect_cb_3(rsfptr:fn(QRectF), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QRectF::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QQuickItem_childrenRectChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QRectF)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QRectF::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickItem_childrenRectChanged_signal_connect for fn(QRectF) {
  fn connect(self, sigthis: QQuickItem_childrenRectChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_childrenRectChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem19childrenRectChangedERK6QRectF(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_childrenRectChanged_signal_connect for Box<Fn(QRectF)> {
  fn connect(self, sigthis: QQuickItem_childrenRectChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_childrenRectChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem19childrenRectChangedERK6QRectF(arg0, arg1, arg2)};
  }
}
// yChanged()
extern fn QQuickItem_yChanged_signal_connect_cb_4(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_yChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_yChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_yChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_yChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem8yChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_yChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_yChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_yChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem8yChangedEv(arg0, arg1, arg2)};
  }
}
// antialiasingChanged(_Bool)
extern fn QQuickItem_antialiasingChanged_signal_connect_cb_5(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QQuickItem_antialiasingChanged_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickItem_antialiasingChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QQuickItem_antialiasingChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_antialiasingChanged_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem19antialiasingChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_antialiasingChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QQuickItem_antialiasingChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_antialiasingChanged_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem19antialiasingChangedEb(arg0, arg1, arg2)};
  }
}
// visibleChildrenChanged()
extern fn QQuickItem_visibleChildrenChanged_signal_connect_cb_6(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_visibleChildrenChanged_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_visibleChildrenChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_visibleChildrenChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_visibleChildrenChanged_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem22visibleChildrenChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_visibleChildrenChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_visibleChildrenChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_visibleChildrenChanged_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem22visibleChildrenChangedEv(arg0, arg1, arg2)};
  }
}
// widthChanged()
extern fn QQuickItem_widthChanged_signal_connect_cb_7(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_widthChanged_signal_connect_cb_box_7(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_widthChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_widthChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_widthChanged_signal_connect_cb_7 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem12widthChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_widthChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_widthChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_widthChanged_signal_connect_cb_box_7 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem12widthChangedEv(arg0, arg1, arg2)};
  }
}
// rotationChanged()
extern fn QQuickItem_rotationChanged_signal_connect_cb_8(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_rotationChanged_signal_connect_cb_box_8(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_rotationChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_rotationChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_rotationChanged_signal_connect_cb_8 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem15rotationChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_rotationChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_rotationChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_rotationChanged_signal_connect_cb_box_8 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem15rotationChangedEv(arg0, arg1, arg2)};
  }
}
// smoothChanged(_Bool)
extern fn QQuickItem_smoothChanged_signal_connect_cb_9(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QQuickItem_smoothChanged_signal_connect_cb_box_9(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickItem_smoothChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QQuickItem_smoothChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_smoothChanged_signal_connect_cb_9 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem13smoothChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_smoothChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QQuickItem_smoothChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_smoothChanged_signal_connect_cb_box_9 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem13smoothChangedEb(arg0, arg1, arg2)};
  }
}
// childrenChanged()
extern fn QQuickItem_childrenChanged_signal_connect_cb_10(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_childrenChanged_signal_connect_cb_box_10(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_childrenChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_childrenChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_childrenChanged_signal_connect_cb_10 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem15childrenChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_childrenChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_childrenChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_childrenChanged_signal_connect_cb_box_10 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem15childrenChangedEv(arg0, arg1, arg2)};
  }
}
// enabledChanged()
extern fn QQuickItem_enabledChanged_signal_connect_cb_11(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_enabledChanged_signal_connect_cb_box_11(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_enabledChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_enabledChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_enabledChanged_signal_connect_cb_11 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem14enabledChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_enabledChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_enabledChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_enabledChanged_signal_connect_cb_box_11 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem14enabledChangedEv(arg0, arg1, arg2)};
  }
}
// implicitHeightChanged()
extern fn QQuickItem_implicitHeightChanged_signal_connect_cb_12(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_implicitHeightChanged_signal_connect_cb_box_12(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_implicitHeightChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_implicitHeightChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_implicitHeightChanged_signal_connect_cb_12 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem21implicitHeightChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_implicitHeightChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_implicitHeightChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_implicitHeightChanged_signal_connect_cb_box_12 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem21implicitHeightChangedEv(arg0, arg1, arg2)};
  }
}
// parentChanged(class QQuickItem *)
extern fn QQuickItem_parentChanged_signal_connect_cb_13(rsfptr:fn(QQuickItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QQuickItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QQuickItem_parentChanged_signal_connect_cb_box_13(rsfptr_raw:*mut Box<Fn(QQuickItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QQuickItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickItem_parentChanged_signal_connect for fn(QQuickItem) {
  fn connect(self, sigthis: QQuickItem_parentChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_parentChanged_signal_connect_cb_13 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem13parentChangedEPS_(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_parentChanged_signal_connect for Box<Fn(QQuickItem)> {
  fn connect(self, sigthis: QQuickItem_parentChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_parentChanged_signal_connect_cb_box_13 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem13parentChangedEPS_(arg0, arg1, arg2)};
  }
}
// xChanged()
extern fn QQuickItem_xChanged_signal_connect_cb_14(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_xChanged_signal_connect_cb_box_14(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_xChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_xChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_xChanged_signal_connect_cb_14 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem8xChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_xChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_xChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_xChanged_signal_connect_cb_box_14 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem8xChangedEv(arg0, arg1, arg2)};
  }
}
// windowChanged(class QQuickWindow *)
extern fn QQuickItem_windowChanged_signal_connect_cb_15(rsfptr:fn(QQuickWindow), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QQuickWindow::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QQuickItem_windowChanged_signal_connect_cb_box_15(rsfptr_raw:*mut Box<Fn(QQuickWindow)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QQuickWindow::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickItem_windowChanged_signal_connect for fn(QQuickWindow) {
  fn connect(self, sigthis: QQuickItem_windowChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_windowChanged_signal_connect_cb_15 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem13windowChangedEP12QQuickWindow(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_windowChanged_signal_connect for Box<Fn(QQuickWindow)> {
  fn connect(self, sigthis: QQuickItem_windowChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_windowChanged_signal_connect_cb_box_15 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem13windowChangedEP12QQuickWindow(arg0, arg1, arg2)};
  }
}
// clipChanged(_Bool)
extern fn QQuickItem_clipChanged_signal_connect_cb_16(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QQuickItem_clipChanged_signal_connect_cb_box_16(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickItem_clipChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QQuickItem_clipChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_clipChanged_signal_connect_cb_16 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem11clipChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_clipChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QQuickItem_clipChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_clipChanged_signal_connect_cb_box_16 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem11clipChangedEb(arg0, arg1, arg2)};
  }
}
// scaleChanged()
extern fn QQuickItem_scaleChanged_signal_connect_cb_17(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_scaleChanged_signal_connect_cb_box_17(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_scaleChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_scaleChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_scaleChanged_signal_connect_cb_17 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem12scaleChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_scaleChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_scaleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_scaleChanged_signal_connect_cb_box_17 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem12scaleChangedEv(arg0, arg1, arg2)};
  }
}
// focusChanged(_Bool)
extern fn QQuickItem_focusChanged_signal_connect_cb_18(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QQuickItem_focusChanged_signal_connect_cb_box_18(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickItem_focusChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QQuickItem_focusChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_focusChanged_signal_connect_cb_18 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem12focusChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_focusChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QQuickItem_focusChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_focusChanged_signal_connect_cb_box_18 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem12focusChangedEb(arg0, arg1, arg2)};
  }
}
// stateChanged(const class QString &)
extern fn QQuickItem_stateChanged_signal_connect_cb_19(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QQuickItem_stateChanged_signal_connect_cb_box_19(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickItem_stateChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QQuickItem_stateChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_stateChanged_signal_connect_cb_19 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem12stateChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_stateChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QQuickItem_stateChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_stateChanged_signal_connect_cb_box_19 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem12stateChangedERK7QString(arg0, arg1, arg2)};
  }
}
// implicitWidthChanged()
extern fn QQuickItem_implicitWidthChanged_signal_connect_cb_20(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_implicitWidthChanged_signal_connect_cb_box_20(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_implicitWidthChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_implicitWidthChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_implicitWidthChanged_signal_connect_cb_20 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem20implicitWidthChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_implicitWidthChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_implicitWidthChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_implicitWidthChanged_signal_connect_cb_box_20 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem20implicitWidthChangedEv(arg0, arg1, arg2)};
  }
}
// activeFocusChanged(_Bool)
extern fn QQuickItem_activeFocusChanged_signal_connect_cb_21(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QQuickItem_activeFocusChanged_signal_connect_cb_box_21(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickItem_activeFocusChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QQuickItem_activeFocusChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_activeFocusChanged_signal_connect_cb_21 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem18activeFocusChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_activeFocusChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QQuickItem_activeFocusChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_activeFocusChanged_signal_connect_cb_box_21 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem18activeFocusChangedEb(arg0, arg1, arg2)};
  }
}
// baselineOffsetChanged(qreal)
extern fn QQuickItem_baselineOffsetChanged_signal_connect_cb_22(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QQuickItem_baselineOffsetChanged_signal_connect_cb_box_22(rsfptr_raw:*mut Box<Fn(f64)>, arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickItem_baselineOffsetChanged_signal_connect for fn(f64) {
  fn connect(self, sigthis: QQuickItem_baselineOffsetChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_baselineOffsetChanged_signal_connect_cb_22 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem21baselineOffsetChangedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_baselineOffsetChanged_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QQuickItem_baselineOffsetChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_baselineOffsetChanged_signal_connect_cb_box_22 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem21baselineOffsetChangedEd(arg0, arg1, arg2)};
  }
}
// activeFocusOnTabChanged(_Bool)
extern fn QQuickItem_activeFocusOnTabChanged_signal_connect_cb_23(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QQuickItem_activeFocusOnTabChanged_signal_connect_cb_box_23(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickItem_activeFocusOnTabChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QQuickItem_activeFocusOnTabChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_activeFocusOnTabChanged_signal_connect_cb_23 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem23activeFocusOnTabChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_activeFocusOnTabChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QQuickItem_activeFocusOnTabChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_activeFocusOnTabChanged_signal_connect_cb_box_23 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem23activeFocusOnTabChangedEb(arg0, arg1, arg2)};
  }
}
// zChanged()
extern fn QQuickItem_zChanged_signal_connect_cb_24(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickItem_zChanged_signal_connect_cb_box_24(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickItem_zChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickItem_zChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_zChanged_signal_connect_cb_24 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem8zChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickItem_zChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickItem_zChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickItem_zChanged_signal_connect_cb_box_24 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickItem_SlotProxy_connect__ZN10QQuickItem8zChangedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

