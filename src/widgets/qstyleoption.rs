// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtWidgets/qstyleoption.h
// dst-file: /src/widgets/qstyleoption.rs
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
// use super::qstyleoption::QStyleOptionComplex; // 773
use std::ops::Deref;
// use super::qstyleoption::QStyleOption; // 773
// use super::qstyleoption::QStyleHintReturn; // 773
use super::super::gui::qtransform::QTransform; // 771
use super::qwidget::QWidget; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox(const QStyleOptionComboBox & other);
  fn _ZN20QStyleOptionComboBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox();
  fn _ZN20QStyleOptionComboBoxC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox(int version);
  fn _ZN20QStyleOptionComboBoxC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem(const QStyleOptionMenuItem & other);
  fn _ZN20QStyleOptionMenuItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem();
  fn _ZN20QStyleOptionMenuItemC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem(int version);
  fn _ZN20QStyleOptionMenuItemC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleHintReturnVariant::~QStyleHintReturnVariant();
  fn _ZN23QStyleHintReturnVariantD0Ev(qthis: *mut c_void);
  // proto:  void QStyleHintReturnVariant::QStyleHintReturnVariant();
  fn _ZN23QStyleHintReturnVariantC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(int version);
  fn _ZN20QStyleOptionTitleBarC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(const QStyleOptionTitleBar & other);
  fn _ZN20QStyleOptionTitleBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar();
  fn _ZN20QStyleOptionTitleBarC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem();
  fn _ZN24QStyleOptionGraphicsItemC1Ev(qthis: *mut c_void);
  // proto: static qreal QStyleOptionGraphicsItem::levelOfDetailFromTransform(const QTransform & worldTransform);
  fn _ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform(arg0: *mut c_void) -> c_double;
  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(const QStyleOptionGraphicsItem & other);
  fn _ZN24QStyleOptionGraphicsItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(int version);
  fn _ZN24QStyleOptionGraphicsItemC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOption::~QStyleOption();
  fn _ZN12QStyleOptionD0Ev(qthis: *mut c_void);
  // proto:  void QStyleOption::init(const QWidget * w);
  fn _ZN12QStyleOption4initEPK7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOption::QStyleOption(const QStyleOption & other);
  fn _ZN12QStyleOptionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOption::QStyleOption(int version, int type);
  fn _ZN12QStyleOptionC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QStyleOption::initFrom(const QWidget * w);
  fn _ZN12QStyleOption8initFromEPK7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget();
  fn _ZN22QStyleOptionDockWidgetC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(int version);
  fn _ZN22QStyleOptionDockWidgetC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(const QStyleOptionDockWidget & other);
  fn _ZN22QStyleOptionDockWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(const QStyleOptionProgressBar & other);
  fn _ZN23QStyleOptionProgressBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(int version);
  fn _ZN23QStyleOptionProgressBarC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar();
  fn _ZN23QStyleOptionProgressBarC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionSlider::QStyleOptionSlider(const QStyleOptionSlider & other);
  fn _ZN18QStyleOptionSliderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionSlider::QStyleOptionSlider(int version);
  fn _ZN18QStyleOptionSliderC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionSlider::QStyleOptionSlider();
  fn _ZN18QStyleOptionSliderC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionFrame::QStyleOptionFrame();
  fn _ZN17QStyleOptionFrameC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionFrame::QStyleOptionFrame(const QStyleOptionFrame & other);
  fn _ZN17QStyleOptionFrameC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionFrame::QStyleOptionFrame(int version);
  fn _ZN17QStyleOptionFrameC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionComplex::QStyleOptionComplex(int version, int type);
  fn _ZN19QStyleOptionComplexC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QStyleOptionComplex::QStyleOptionComplex(const QStyleOptionComplex & other);
  fn _ZN19QStyleOptionComplexC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleHintReturn::~QStyleHintReturn();
  fn _ZN16QStyleHintReturnD0Ev(qthis: *mut c_void);
  // proto:  void QStyleHintReturn::QStyleHintReturn(int version, int type);
  fn _ZN16QStyleHintReturnC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QStyleOptionHeader::QStyleOptionHeader();
  fn _ZN18QStyleOptionHeaderC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionHeader::QStyleOptionHeader(const QStyleOptionHeader & other);
  fn _ZN18QStyleOptionHeaderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionHeader::QStyleOptionHeader(int version);
  fn _ZN18QStyleOptionHeaderC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox();
  fn _ZN19QStyleOptionToolBoxC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox(const QStyleOptionToolBox & other);
  fn _ZN19QStyleOptionToolBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox(int version);
  fn _ZN19QStyleOptionToolBoxC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(int version);
  fn _ZN21QStyleOptionFocusRectC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect();
  fn _ZN21QStyleOptionFocusRectC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(const QStyleOptionFocusRect & other);
  fn _ZN21QStyleOptionFocusRectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(int version);
  fn _ZN20QStyleOptionGroupBoxC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(const QStyleOptionGroupBox & other);
  fn _ZN20QStyleOptionGroupBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox();
  fn _ZN20QStyleOptionGroupBoxC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionTab::QStyleOptionTab(const QStyleOptionTab & other);
  fn _ZN15QStyleOptionTabC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionTab::QStyleOptionTab(int version);
  fn _ZN15QStyleOptionTabC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionTab::QStyleOptionTab();
  fn _ZN15QStyleOptionTabC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase();
  fn _ZN22QStyleOptionTabBarBaseC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(int version);
  fn _ZN22QStyleOptionTabBarBaseC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(const QStyleOptionTabBarBase & other);
  fn _ZN22QStyleOptionTabBarBaseC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(int version);
  fn _ZN22QStyleOptionRubberBandC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand();
  fn _ZN22QStyleOptionRubberBandC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(const QStyleOptionRubberBand & other);
  fn _ZN22QStyleOptionRubberBandC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionButton::QStyleOptionButton(int version);
  fn _ZN18QStyleOptionButtonC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionButton::QStyleOptionButton();
  fn _ZN18QStyleOptionButtonC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionButton::QStyleOptionButton(const QStyleOptionButton & other);
  fn _ZN18QStyleOptionButtonC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleHintReturnMask::QStyleHintReturnMask();
  fn _ZN20QStyleHintReturnMaskC1Ev(qthis: *mut c_void);
  // proto:  void QStyleHintReturnMask::~QStyleHintReturnMask();
  fn _ZN20QStyleHintReturnMaskD0Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton(int version);
  fn _ZN22QStyleOptionToolButtonC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton();
  fn _ZN22QStyleOptionToolButtonC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton(const QStyleOptionToolButton & other);
  fn _ZN22QStyleOptionToolButtonC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(int version);
  fn _ZN20QStyleOptionSizeGripC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip();
  fn _ZN20QStyleOptionSizeGripC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(const QStyleOptionSizeGrip & other);
  fn _ZN20QStyleOptionSizeGripC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem(const QStyleOptionViewItem & other);
  fn _ZN20QStyleOptionViewItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem(int version);
  fn _ZN20QStyleOptionViewItemC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem();
  fn _ZN20QStyleOptionViewItemC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox();
  fn _ZN19QStyleOptionSpinBoxC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox(const QStyleOptionSpinBox & other);
  fn _ZN19QStyleOptionSpinBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox(int version);
  fn _ZN19QStyleOptionSpinBoxC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(const QStyleOptionToolBar & other);
  fn _ZN19QStyleOptionToolBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(int version);
  fn _ZN19QStyleOptionToolBarC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar();
  fn _ZN19QStyleOptionToolBarC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame & other);
  fn _ZN26QStyleOptionTabWidgetFrameC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(int version);
  fn _ZN26QStyleOptionTabWidgetFrameC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame();
  fn _ZN26QStyleOptionTabWidgetFrameC1Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStyleOptionComboBox)=1
pub struct QStyleOptionComboBox {
  qbase: QStyleOptionComplex,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionMenuItem)=1
pub struct QStyleOptionMenuItem {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleHintReturnVariant)=24
pub struct QStyleHintReturnVariant {
  qbase: QStyleHintReturn,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionTitleBar)=1
pub struct QStyleOptionTitleBar {
  qbase: QStyleOptionComplex,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionGraphicsItem)=1
pub struct QStyleOptionGraphicsItem {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOption)=1
pub struct QStyleOption {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionDockWidget)=1
pub struct QStyleOptionDockWidget {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionProgressBar)=1
pub struct QStyleOptionProgressBar {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionSlider)=1
pub struct QStyleOptionSlider {
  qbase: QStyleOptionComplex,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionFrame)=1
pub struct QStyleOptionFrame {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionComplex)=1
pub struct QStyleOptionComplex {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleHintReturn)=8
pub struct QStyleHintReturn {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionHeader)=1
pub struct QStyleOptionHeader {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionToolBox)=1
pub struct QStyleOptionToolBox {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionFocusRect)=1
pub struct QStyleOptionFocusRect {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionGroupBox)=1
pub struct QStyleOptionGroupBox {
  qbase: QStyleOptionComplex,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionTab)=1
pub struct QStyleOptionTab {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionTabBarBase)=1
pub struct QStyleOptionTabBarBase {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionRubberBand)=1
pub struct QStyleOptionRubberBand {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionButton)=1
pub struct QStyleOptionButton {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleHintReturnMask)=16
pub struct QStyleHintReturnMask {
  qbase: QStyleHintReturn,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionToolButton)=1
pub struct QStyleOptionToolButton {
  qbase: QStyleOptionComplex,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionSizeGrip)=1
pub struct QStyleOptionSizeGrip {
  qbase: QStyleOptionComplex,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionViewItem)=1
pub struct QStyleOptionViewItem {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionSpinBox)=1
pub struct QStyleOptionSpinBox {
  qbase: QStyleOptionComplex,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionToolBar)=1
pub struct QStyleOptionToolBar {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionTabWidgetFrame)=1
pub struct QStyleOptionTabWidgetFrame {
  qbase: QStyleOption,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleOptionComboBox {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionComboBox {
    return QStyleOptionComboBox{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionComboBox {
  type Target = QStyleOptionComplex;

  fn deref(&self) -> &QStyleOptionComplex {
    return & self.qbase;
  }
}
impl AsRef<QStyleOptionComplex> for QStyleOptionComboBox {
  fn as_ref(& self) -> & QStyleOptionComplex {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox(const QStyleOptionComboBox & other);
impl /*struct*/ QStyleOptionComboBox {
  pub fn New<T: QStyleOptionComboBox_New>(value: T) -> QStyleOptionComboBox {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionComboBox_New {
  fn New(self) -> QStyleOptionComboBox;
}

  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox(const QStyleOptionComboBox & other);
impl<'a> /*trait*/ QStyleOptionComboBox_New for (&'a QStyleOptionComboBox) {
  fn New(self) -> QStyleOptionComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionComboBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionComboBoxC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionComboBox{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox();
impl<'a> /*trait*/ QStyleOptionComboBox_New for () {
  fn New(self) -> QStyleOptionComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionComboBoxC1Ev()};
    unsafe {_ZN20QStyleOptionComboBoxC1Ev(qthis)};
    let rsthis = QStyleOptionComboBox{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox(int version);
impl<'a> /*trait*/ QStyleOptionComboBox_New for (i32) {
  fn New(self) -> QStyleOptionComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionComboBoxC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionComboBoxC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionComboBox{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionMenuItem {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionMenuItem {
    return QStyleOptionMenuItem{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionMenuItem {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionMenuItem {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem(const QStyleOptionMenuItem & other);
impl /*struct*/ QStyleOptionMenuItem {
  pub fn New<T: QStyleOptionMenuItem_New>(value: T) -> QStyleOptionMenuItem {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionMenuItem_New {
  fn New(self) -> QStyleOptionMenuItem;
}

  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem(const QStyleOptionMenuItem & other);
impl<'a> /*trait*/ QStyleOptionMenuItem_New for (&'a QStyleOptionMenuItem) {
  fn New(self) -> QStyleOptionMenuItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionMenuItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionMenuItemC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionMenuItem{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem();
impl<'a> /*trait*/ QStyleOptionMenuItem_New for () {
  fn New(self) -> QStyleOptionMenuItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionMenuItemC1Ev()};
    unsafe {_ZN20QStyleOptionMenuItemC1Ev(qthis)};
    let rsthis = QStyleOptionMenuItem{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem(int version);
impl<'a> /*trait*/ QStyleOptionMenuItem_New for (i32) {
  fn New(self) -> QStyleOptionMenuItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionMenuItemC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionMenuItemC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionMenuItem{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleHintReturnVariant {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleHintReturnVariant {
    return QStyleHintReturnVariant{qbase: QStyleHintReturn::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleHintReturnVariant {
  type Target = QStyleHintReturn;

  fn deref(&self) -> &QStyleHintReturn {
    return & self.qbase;
  }
}
impl AsRef<QStyleHintReturn> for QStyleHintReturnVariant {
  fn as_ref(& self) -> & QStyleHintReturn {
    return & self.qbase;
  }
}
  // proto:  void QStyleHintReturnVariant::~QStyleHintReturnVariant();
impl /*struct*/ QStyleHintReturnVariant {
  pub fn Free<RetType, T: QStyleHintReturnVariant_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QStyleHintReturnVariant_Free<RetType> {
  fn Free(self , rsthis: & QStyleHintReturnVariant) -> RetType;
}

  // proto:  void QStyleHintReturnVariant::~QStyleHintReturnVariant();
impl<'a> /*trait*/ QStyleHintReturnVariant_Free<()> for () {
  fn Free(self , rsthis: & QStyleHintReturnVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleHintReturnVariantD0Ev()};
     unsafe {_ZN23QStyleHintReturnVariantD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyleHintReturnVariant::QStyleHintReturnVariant();
impl /*struct*/ QStyleHintReturnVariant {
  pub fn New<T: QStyleHintReturnVariant_New>(value: T) -> QStyleHintReturnVariant {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturnVariant_New {
  fn New(self) -> QStyleHintReturnVariant;
}

  // proto:  void QStyleHintReturnVariant::QStyleHintReturnVariant();
impl<'a> /*trait*/ QStyleHintReturnVariant_New for () {
  fn New(self) -> QStyleHintReturnVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleHintReturnVariantC1Ev()};
    unsafe {_ZN23QStyleHintReturnVariantC1Ev(qthis)};
    let rsthis = QStyleHintReturnVariant{/**/qbase: QStyleHintReturn::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionTitleBar {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionTitleBar {
    return QStyleOptionTitleBar{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionTitleBar {
  type Target = QStyleOptionComplex;

  fn deref(&self) -> &QStyleOptionComplex {
    return & self.qbase;
  }
}
impl AsRef<QStyleOptionComplex> for QStyleOptionTitleBar {
  fn as_ref(& self) -> & QStyleOptionComplex {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(int version);
impl /*struct*/ QStyleOptionTitleBar {
  pub fn New<T: QStyleOptionTitleBar_New>(value: T) -> QStyleOptionTitleBar {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTitleBar_New {
  fn New(self) -> QStyleOptionTitleBar;
}

  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(int version);
impl<'a> /*trait*/ QStyleOptionTitleBar_New for (i32) {
  fn New(self) -> QStyleOptionTitleBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionTitleBarC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionTitleBarC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionTitleBar{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(const QStyleOptionTitleBar & other);
impl<'a> /*trait*/ QStyleOptionTitleBar_New for (&'a QStyleOptionTitleBar) {
  fn New(self) -> QStyleOptionTitleBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionTitleBarC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionTitleBarC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionTitleBar{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar();
impl<'a> /*trait*/ QStyleOptionTitleBar_New for () {
  fn New(self) -> QStyleOptionTitleBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionTitleBarC1Ev()};
    unsafe {_ZN20QStyleOptionTitleBarC1Ev(qthis)};
    let rsthis = QStyleOptionTitleBar{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionGraphicsItem {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionGraphicsItem {
    return QStyleOptionGraphicsItem{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionGraphicsItem {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionGraphicsItem {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem();
impl /*struct*/ QStyleOptionGraphicsItem {
  pub fn New<T: QStyleOptionGraphicsItem_New>(value: T) -> QStyleOptionGraphicsItem {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionGraphicsItem_New {
  fn New(self) -> QStyleOptionGraphicsItem;
}

  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem();
impl<'a> /*trait*/ QStyleOptionGraphicsItem_New for () {
  fn New(self) -> QStyleOptionGraphicsItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItemC1Ev()};
    unsafe {_ZN24QStyleOptionGraphicsItemC1Ev(qthis)};
    let rsthis = QStyleOptionGraphicsItem{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static qreal QStyleOptionGraphicsItem::levelOfDetailFromTransform(const QTransform & worldTransform);
impl /*struct*/ QStyleOptionGraphicsItem {
  pub fn levelOfDetailFromTransform_s<RetType, T: QStyleOptionGraphicsItem_levelOfDetailFromTransform_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.levelOfDetailFromTransform_s();
    // return 1;
  }
}

pub trait QStyleOptionGraphicsItem_levelOfDetailFromTransform_s<RetType> {
  fn levelOfDetailFromTransform_s(self ) -> RetType;
}

  // proto: static qreal QStyleOptionGraphicsItem::levelOfDetailFromTransform(const QTransform & worldTransform);
impl<'a> /*trait*/ QStyleOptionGraphicsItem_levelOfDetailFromTransform_s<f64> for (&'a QTransform) {
  fn levelOfDetailFromTransform_s(self ) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform(arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(const QStyleOptionGraphicsItem & other);
impl<'a> /*trait*/ QStyleOptionGraphicsItem_New for (&'a QStyleOptionGraphicsItem) {
  fn New(self) -> QStyleOptionGraphicsItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QStyleOptionGraphicsItemC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionGraphicsItem{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(int version);
impl<'a> /*trait*/ QStyleOptionGraphicsItem_New for (i32) {
  fn New(self) -> QStyleOptionGraphicsItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItemC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN24QStyleOptionGraphicsItemC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionGraphicsItem{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOption {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOption {
    return QStyleOption{qclsinst: qthis};
  }
}
  // proto:  void QStyleOption::~QStyleOption();
impl /*struct*/ QStyleOption {
  pub fn Free<RetType, T: QStyleOption_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QStyleOption_Free<RetType> {
  fn Free(self , rsthis: & QStyleOption) -> RetType;
}

  // proto:  void QStyleOption::~QStyleOption();
impl<'a> /*trait*/ QStyleOption_Free<()> for () {
  fn Free(self , rsthis: & QStyleOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOptionD0Ev()};
     unsafe {_ZN12QStyleOptionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyleOption::init(const QWidget * w);
impl /*struct*/ QStyleOption {
  pub fn init<RetType, T: QStyleOption_init<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.init(self);
    // return 1;
  }
}

pub trait QStyleOption_init<RetType> {
  fn init(self , rsthis: & QStyleOption) -> RetType;
}

  // proto:  void QStyleOption::init(const QWidget * w);
impl<'a> /*trait*/ QStyleOption_init<()> for (&'a QWidget) {
  fn init(self , rsthis: & QStyleOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOption4initEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QStyleOption4initEPK7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyleOption::QStyleOption(const QStyleOption & other);
impl /*struct*/ QStyleOption {
  pub fn New<T: QStyleOption_New>(value: T) -> QStyleOption {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOption_New {
  fn New(self) -> QStyleOption;
}

  // proto:  void QStyleOption::QStyleOption(const QStyleOption & other);
impl<'a> /*trait*/ QStyleOption_New for (&'a QStyleOption) {
  fn New(self) -> QStyleOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOptionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QStyleOptionC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOption::QStyleOption(int version, int type);
impl<'a> /*trait*/ QStyleOption_New for (i32, i32) {
  fn New(self) -> QStyleOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOptionC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QStyleOptionC1Eii(qthis, arg0, arg1)};
    let rsthis = QStyleOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOption::initFrom(const QWidget * w);
impl /*struct*/ QStyleOption {
  pub fn initFrom<RetType, T: QStyleOption_initFrom<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.initFrom(self);
    // return 1;
  }
}

pub trait QStyleOption_initFrom<RetType> {
  fn initFrom(self , rsthis: & QStyleOption) -> RetType;
}

  // proto:  void QStyleOption::initFrom(const QWidget * w);
impl<'a> /*trait*/ QStyleOption_initFrom<()> for (&'a QWidget) {
  fn initFrom(self , rsthis: & QStyleOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOption8initFromEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QStyleOption8initFromEPK7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStyleOptionDockWidget {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionDockWidget {
    return QStyleOptionDockWidget{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionDockWidget {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionDockWidget {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget();
impl /*struct*/ QStyleOptionDockWidget {
  pub fn New<T: QStyleOptionDockWidget_New>(value: T) -> QStyleOptionDockWidget {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionDockWidget_New {
  fn New(self) -> QStyleOptionDockWidget;
}

  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget();
impl<'a> /*trait*/ QStyleOptionDockWidget_New for () {
  fn New(self) -> QStyleOptionDockWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionDockWidgetC1Ev()};
    unsafe {_ZN22QStyleOptionDockWidgetC1Ev(qthis)};
    let rsthis = QStyleOptionDockWidget{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(int version);
impl<'a> /*trait*/ QStyleOptionDockWidget_New for (i32) {
  fn New(self) -> QStyleOptionDockWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionDockWidgetC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN22QStyleOptionDockWidgetC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionDockWidget{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(const QStyleOptionDockWidget & other);
impl<'a> /*trait*/ QStyleOptionDockWidget_New for (&'a QStyleOptionDockWidget) {
  fn New(self) -> QStyleOptionDockWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionDockWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QStyleOptionDockWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionDockWidget{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionProgressBar {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionProgressBar {
    return QStyleOptionProgressBar{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionProgressBar {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionProgressBar {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(const QStyleOptionProgressBar & other);
impl /*struct*/ QStyleOptionProgressBar {
  pub fn New<T: QStyleOptionProgressBar_New>(value: T) -> QStyleOptionProgressBar {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionProgressBar_New {
  fn New(self) -> QStyleOptionProgressBar;
}

  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(const QStyleOptionProgressBar & other);
impl<'a> /*trait*/ QStyleOptionProgressBar_New for (&'a QStyleOptionProgressBar) {
  fn New(self) -> QStyleOptionProgressBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleOptionProgressBarC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QStyleOptionProgressBarC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionProgressBar{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(int version);
impl<'a> /*trait*/ QStyleOptionProgressBar_New for (i32) {
  fn New(self) -> QStyleOptionProgressBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleOptionProgressBarC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN23QStyleOptionProgressBarC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionProgressBar{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar();
impl<'a> /*trait*/ QStyleOptionProgressBar_New for () {
  fn New(self) -> QStyleOptionProgressBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleOptionProgressBarC1Ev()};
    unsafe {_ZN23QStyleOptionProgressBarC1Ev(qthis)};
    let rsthis = QStyleOptionProgressBar{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionSlider {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionSlider {
    return QStyleOptionSlider{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionSlider {
  type Target = QStyleOptionComplex;

  fn deref(&self) -> &QStyleOptionComplex {
    return & self.qbase;
  }
}
impl AsRef<QStyleOptionComplex> for QStyleOptionSlider {
  fn as_ref(& self) -> & QStyleOptionComplex {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionSlider::QStyleOptionSlider(const QStyleOptionSlider & other);
impl /*struct*/ QStyleOptionSlider {
  pub fn New<T: QStyleOptionSlider_New>(value: T) -> QStyleOptionSlider {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSlider_New {
  fn New(self) -> QStyleOptionSlider;
}

  // proto:  void QStyleOptionSlider::QStyleOptionSlider(const QStyleOptionSlider & other);
impl<'a> /*trait*/ QStyleOptionSlider_New for (&'a QStyleOptionSlider) {
  fn New(self) -> QStyleOptionSlider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionSliderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QStyleOptionSliderC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionSlider{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSlider::QStyleOptionSlider(int version);
impl<'a> /*trait*/ QStyleOptionSlider_New for (i32) {
  fn New(self) -> QStyleOptionSlider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionSliderC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QStyleOptionSliderC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionSlider{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSlider::QStyleOptionSlider();
impl<'a> /*trait*/ QStyleOptionSlider_New for () {
  fn New(self) -> QStyleOptionSlider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionSliderC1Ev()};
    unsafe {_ZN18QStyleOptionSliderC1Ev(qthis)};
    let rsthis = QStyleOptionSlider{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionFrame {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionFrame {
    return QStyleOptionFrame{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionFrame {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionFrame {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionFrame::QStyleOptionFrame();
impl /*struct*/ QStyleOptionFrame {
  pub fn New<T: QStyleOptionFrame_New>(value: T) -> QStyleOptionFrame {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionFrame_New {
  fn New(self) -> QStyleOptionFrame;
}

  // proto:  void QStyleOptionFrame::QStyleOptionFrame();
impl<'a> /*trait*/ QStyleOptionFrame_New for () {
  fn New(self) -> QStyleOptionFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QStyleOptionFrameC1Ev()};
    unsafe {_ZN17QStyleOptionFrameC1Ev(qthis)};
    let rsthis = QStyleOptionFrame{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFrame::QStyleOptionFrame(const QStyleOptionFrame & other);
impl<'a> /*trait*/ QStyleOptionFrame_New for (&'a QStyleOptionFrame) {
  fn New(self) -> QStyleOptionFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QStyleOptionFrameC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QStyleOptionFrameC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionFrame{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFrame::QStyleOptionFrame(int version);
impl<'a> /*trait*/ QStyleOptionFrame_New for (i32) {
  fn New(self) -> QStyleOptionFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QStyleOptionFrameC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN17QStyleOptionFrameC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionFrame{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionComplex {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionComplex {
    return QStyleOptionComplex{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionComplex {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionComplex {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionComplex::QStyleOptionComplex(int version, int type);
impl /*struct*/ QStyleOptionComplex {
  pub fn New<T: QStyleOptionComplex_New>(value: T) -> QStyleOptionComplex {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionComplex_New {
  fn New(self) -> QStyleOptionComplex;
}

  // proto:  void QStyleOptionComplex::QStyleOptionComplex(int version, int type);
impl<'a> /*trait*/ QStyleOptionComplex_New for (i32, i32) {
  fn New(self) -> QStyleOptionComplex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionComplexC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN19QStyleOptionComplexC1Eii(qthis, arg0, arg1)};
    let rsthis = QStyleOptionComplex{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionComplex::QStyleOptionComplex(const QStyleOptionComplex & other);
impl<'a> /*trait*/ QStyleOptionComplex_New for (&'a QStyleOptionComplex) {
  fn New(self) -> QStyleOptionComplex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionComplexC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyleOptionComplexC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionComplex{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleHintReturn {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleHintReturn {
    return QStyleHintReturn{qclsinst: qthis};
  }
}
  // proto:  void QStyleHintReturn::~QStyleHintReturn();
impl /*struct*/ QStyleHintReturn {
  pub fn Free<RetType, T: QStyleHintReturn_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QStyleHintReturn_Free<RetType> {
  fn Free(self , rsthis: & QStyleHintReturn) -> RetType;
}

  // proto:  void QStyleHintReturn::~QStyleHintReturn();
impl<'a> /*trait*/ QStyleHintReturn_Free<()> for () {
  fn Free(self , rsthis: & QStyleHintReturn) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStyleHintReturnD0Ev()};
     unsafe {_ZN16QStyleHintReturnD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyleHintReturn::QStyleHintReturn(int version, int type);
impl /*struct*/ QStyleHintReturn {
  pub fn New<T: QStyleHintReturn_New>(value: T) -> QStyleHintReturn {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturn_New {
  fn New(self) -> QStyleHintReturn;
}

  // proto:  void QStyleHintReturn::QStyleHintReturn(int version, int type);
impl<'a> /*trait*/ QStyleHintReturn_New for (i32, i32) {
  fn New(self) -> QStyleHintReturn {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStyleHintReturnC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN16QStyleHintReturnC1Eii(qthis, arg0, arg1)};
    let rsthis = QStyleHintReturn{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionHeader {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionHeader {
    return QStyleOptionHeader{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionHeader {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionHeader {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionHeader::QStyleOptionHeader();
impl /*struct*/ QStyleOptionHeader {
  pub fn New<T: QStyleOptionHeader_New>(value: T) -> QStyleOptionHeader {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionHeader_New {
  fn New(self) -> QStyleOptionHeader;
}

  // proto:  void QStyleOptionHeader::QStyleOptionHeader();
impl<'a> /*trait*/ QStyleOptionHeader_New for () {
  fn New(self) -> QStyleOptionHeader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionHeaderC1Ev()};
    unsafe {_ZN18QStyleOptionHeaderC1Ev(qthis)};
    let rsthis = QStyleOptionHeader{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionHeader::QStyleOptionHeader(const QStyleOptionHeader & other);
impl<'a> /*trait*/ QStyleOptionHeader_New for (&'a QStyleOptionHeader) {
  fn New(self) -> QStyleOptionHeader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionHeaderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QStyleOptionHeaderC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionHeader{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionHeader::QStyleOptionHeader(int version);
impl<'a> /*trait*/ QStyleOptionHeader_New for (i32) {
  fn New(self) -> QStyleOptionHeader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionHeaderC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QStyleOptionHeaderC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionHeader{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionToolBox {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionToolBox {
    return QStyleOptionToolBox{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionToolBox {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionToolBox {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox();
impl /*struct*/ QStyleOptionToolBox {
  pub fn New<T: QStyleOptionToolBox_New>(value: T) -> QStyleOptionToolBox {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolBox_New {
  fn New(self) -> QStyleOptionToolBox;
}

  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox();
impl<'a> /*trait*/ QStyleOptionToolBox_New for () {
  fn New(self) -> QStyleOptionToolBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBoxC1Ev()};
    unsafe {_ZN19QStyleOptionToolBoxC1Ev(qthis)};
    let rsthis = QStyleOptionToolBox{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox(const QStyleOptionToolBox & other);
impl<'a> /*trait*/ QStyleOptionToolBox_New for (&'a QStyleOptionToolBox) {
  fn New(self) -> QStyleOptionToolBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyleOptionToolBoxC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionToolBox{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox(int version);
impl<'a> /*trait*/ QStyleOptionToolBox_New for (i32) {
  fn New(self) -> QStyleOptionToolBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBoxC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN19QStyleOptionToolBoxC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionToolBox{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionFocusRect {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionFocusRect {
    return QStyleOptionFocusRect{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionFocusRect {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionFocusRect {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(int version);
impl /*struct*/ QStyleOptionFocusRect {
  pub fn New<T: QStyleOptionFocusRect_New>(value: T) -> QStyleOptionFocusRect {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionFocusRect_New {
  fn New(self) -> QStyleOptionFocusRect;
}

  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(int version);
impl<'a> /*trait*/ QStyleOptionFocusRect_New for (i32) {
  fn New(self) -> QStyleOptionFocusRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QStyleOptionFocusRectC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN21QStyleOptionFocusRectC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionFocusRect{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect();
impl<'a> /*trait*/ QStyleOptionFocusRect_New for () {
  fn New(self) -> QStyleOptionFocusRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QStyleOptionFocusRectC1Ev()};
    unsafe {_ZN21QStyleOptionFocusRectC1Ev(qthis)};
    let rsthis = QStyleOptionFocusRect{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(const QStyleOptionFocusRect & other);
impl<'a> /*trait*/ QStyleOptionFocusRect_New for (&'a QStyleOptionFocusRect) {
  fn New(self) -> QStyleOptionFocusRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QStyleOptionFocusRectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QStyleOptionFocusRectC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionFocusRect{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionGroupBox {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionGroupBox {
    return QStyleOptionGroupBox{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionGroupBox {
  type Target = QStyleOptionComplex;

  fn deref(&self) -> &QStyleOptionComplex {
    return & self.qbase;
  }
}
impl AsRef<QStyleOptionComplex> for QStyleOptionGroupBox {
  fn as_ref(& self) -> & QStyleOptionComplex {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(int version);
impl /*struct*/ QStyleOptionGroupBox {
  pub fn New<T: QStyleOptionGroupBox_New>(value: T) -> QStyleOptionGroupBox {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionGroupBox_New {
  fn New(self) -> QStyleOptionGroupBox;
}

  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(int version);
impl<'a> /*trait*/ QStyleOptionGroupBox_New for (i32) {
  fn New(self) -> QStyleOptionGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionGroupBoxC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionGroupBoxC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionGroupBox{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(const QStyleOptionGroupBox & other);
impl<'a> /*trait*/ QStyleOptionGroupBox_New for (&'a QStyleOptionGroupBox) {
  fn New(self) -> QStyleOptionGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionGroupBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionGroupBoxC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionGroupBox{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox();
impl<'a> /*trait*/ QStyleOptionGroupBox_New for () {
  fn New(self) -> QStyleOptionGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionGroupBoxC1Ev()};
    unsafe {_ZN20QStyleOptionGroupBoxC1Ev(qthis)};
    let rsthis = QStyleOptionGroupBox{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionTab {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionTab {
    return QStyleOptionTab{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionTab {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionTab {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionTab::QStyleOptionTab(const QStyleOptionTab & other);
impl /*struct*/ QStyleOptionTab {
  pub fn New<T: QStyleOptionTab_New>(value: T) -> QStyleOptionTab {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTab_New {
  fn New(self) -> QStyleOptionTab;
}

  // proto:  void QStyleOptionTab::QStyleOptionTab(const QStyleOptionTab & other);
impl<'a> /*trait*/ QStyleOptionTab_New for (&'a QStyleOptionTab) {
  fn New(self) -> QStyleOptionTab {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStyleOptionTabC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QStyleOptionTabC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionTab{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTab::QStyleOptionTab(int version);
impl<'a> /*trait*/ QStyleOptionTab_New for (i32) {
  fn New(self) -> QStyleOptionTab {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStyleOptionTabC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QStyleOptionTabC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionTab{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTab::QStyleOptionTab();
impl<'a> /*trait*/ QStyleOptionTab_New for () {
  fn New(self) -> QStyleOptionTab {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStyleOptionTabC1Ev()};
    unsafe {_ZN15QStyleOptionTabC1Ev(qthis)};
    let rsthis = QStyleOptionTab{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionTabBarBase {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionTabBarBase {
    return QStyleOptionTabBarBase{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionTabBarBase {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionTabBarBase {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase();
impl /*struct*/ QStyleOptionTabBarBase {
  pub fn New<T: QStyleOptionTabBarBase_New>(value: T) -> QStyleOptionTabBarBase {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTabBarBase_New {
  fn New(self) -> QStyleOptionTabBarBase;
}

  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase();
impl<'a> /*trait*/ QStyleOptionTabBarBase_New for () {
  fn New(self) -> QStyleOptionTabBarBase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionTabBarBaseC1Ev()};
    unsafe {_ZN22QStyleOptionTabBarBaseC1Ev(qthis)};
    let rsthis = QStyleOptionTabBarBase{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(int version);
impl<'a> /*trait*/ QStyleOptionTabBarBase_New for (i32) {
  fn New(self) -> QStyleOptionTabBarBase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionTabBarBaseC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN22QStyleOptionTabBarBaseC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionTabBarBase{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(const QStyleOptionTabBarBase & other);
impl<'a> /*trait*/ QStyleOptionTabBarBase_New for (&'a QStyleOptionTabBarBase) {
  fn New(self) -> QStyleOptionTabBarBase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionTabBarBaseC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QStyleOptionTabBarBaseC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionTabBarBase{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionRubberBand {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionRubberBand {
    return QStyleOptionRubberBand{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionRubberBand {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionRubberBand {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(int version);
impl /*struct*/ QStyleOptionRubberBand {
  pub fn New<T: QStyleOptionRubberBand_New>(value: T) -> QStyleOptionRubberBand {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionRubberBand_New {
  fn New(self) -> QStyleOptionRubberBand;
}

  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(int version);
impl<'a> /*trait*/ QStyleOptionRubberBand_New for (i32) {
  fn New(self) -> QStyleOptionRubberBand {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionRubberBandC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN22QStyleOptionRubberBandC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionRubberBand{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand();
impl<'a> /*trait*/ QStyleOptionRubberBand_New for () {
  fn New(self) -> QStyleOptionRubberBand {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionRubberBandC1Ev()};
    unsafe {_ZN22QStyleOptionRubberBandC1Ev(qthis)};
    let rsthis = QStyleOptionRubberBand{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(const QStyleOptionRubberBand & other);
impl<'a> /*trait*/ QStyleOptionRubberBand_New for (&'a QStyleOptionRubberBand) {
  fn New(self) -> QStyleOptionRubberBand {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionRubberBandC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QStyleOptionRubberBandC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionRubberBand{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionButton {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionButton {
    return QStyleOptionButton{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionButton {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionButton {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionButton::QStyleOptionButton(int version);
impl /*struct*/ QStyleOptionButton {
  pub fn New<T: QStyleOptionButton_New>(value: T) -> QStyleOptionButton {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionButton_New {
  fn New(self) -> QStyleOptionButton;
}

  // proto:  void QStyleOptionButton::QStyleOptionButton(int version);
impl<'a> /*trait*/ QStyleOptionButton_New for (i32) {
  fn New(self) -> QStyleOptionButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionButtonC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QStyleOptionButtonC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionButton{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionButton::QStyleOptionButton();
impl<'a> /*trait*/ QStyleOptionButton_New for () {
  fn New(self) -> QStyleOptionButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionButtonC1Ev()};
    unsafe {_ZN18QStyleOptionButtonC1Ev(qthis)};
    let rsthis = QStyleOptionButton{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionButton::QStyleOptionButton(const QStyleOptionButton & other);
impl<'a> /*trait*/ QStyleOptionButton_New for (&'a QStyleOptionButton) {
  fn New(self) -> QStyleOptionButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionButtonC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QStyleOptionButtonC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionButton{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleHintReturnMask {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleHintReturnMask {
    return QStyleHintReturnMask{qbase: QStyleHintReturn::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleHintReturnMask {
  type Target = QStyleHintReturn;

  fn deref(&self) -> &QStyleHintReturn {
    return & self.qbase;
  }
}
impl AsRef<QStyleHintReturn> for QStyleHintReturnMask {
  fn as_ref(& self) -> & QStyleHintReturn {
    return & self.qbase;
  }
}
  // proto:  void QStyleHintReturnMask::QStyleHintReturnMask();
impl /*struct*/ QStyleHintReturnMask {
  pub fn New<T: QStyleHintReturnMask_New>(value: T) -> QStyleHintReturnMask {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturnMask_New {
  fn New(self) -> QStyleHintReturnMask;
}

  // proto:  void QStyleHintReturnMask::QStyleHintReturnMask();
impl<'a> /*trait*/ QStyleHintReturnMask_New for () {
  fn New(self) -> QStyleHintReturnMask {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleHintReturnMaskC1Ev()};
    unsafe {_ZN20QStyleHintReturnMaskC1Ev(qthis)};
    let rsthis = QStyleHintReturnMask{/**/qbase: QStyleHintReturn::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleHintReturnMask::~QStyleHintReturnMask();
impl /*struct*/ QStyleHintReturnMask {
  pub fn Free<RetType, T: QStyleHintReturnMask_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QStyleHintReturnMask_Free<RetType> {
  fn Free(self , rsthis: & QStyleHintReturnMask) -> RetType;
}

  // proto:  void QStyleHintReturnMask::~QStyleHintReturnMask();
impl<'a> /*trait*/ QStyleHintReturnMask_Free<()> for () {
  fn Free(self , rsthis: & QStyleHintReturnMask) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleHintReturnMaskD0Ev()};
     unsafe {_ZN20QStyleHintReturnMaskD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStyleOptionToolButton {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionToolButton {
    return QStyleOptionToolButton{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionToolButton {
  type Target = QStyleOptionComplex;

  fn deref(&self) -> &QStyleOptionComplex {
    return & self.qbase;
  }
}
impl AsRef<QStyleOptionComplex> for QStyleOptionToolButton {
  fn as_ref(& self) -> & QStyleOptionComplex {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton(int version);
impl /*struct*/ QStyleOptionToolButton {
  pub fn New<T: QStyleOptionToolButton_New>(value: T) -> QStyleOptionToolButton {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolButton_New {
  fn New(self) -> QStyleOptionToolButton;
}

  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton(int version);
impl<'a> /*trait*/ QStyleOptionToolButton_New for (i32) {
  fn New(self) -> QStyleOptionToolButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionToolButtonC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN22QStyleOptionToolButtonC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionToolButton{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton();
impl<'a> /*trait*/ QStyleOptionToolButton_New for () {
  fn New(self) -> QStyleOptionToolButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionToolButtonC1Ev()};
    unsafe {_ZN22QStyleOptionToolButtonC1Ev(qthis)};
    let rsthis = QStyleOptionToolButton{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton(const QStyleOptionToolButton & other);
impl<'a> /*trait*/ QStyleOptionToolButton_New for (&'a QStyleOptionToolButton) {
  fn New(self) -> QStyleOptionToolButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionToolButtonC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QStyleOptionToolButtonC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionToolButton{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionSizeGrip {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionSizeGrip {
    return QStyleOptionSizeGrip{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionSizeGrip {
  type Target = QStyleOptionComplex;

  fn deref(&self) -> &QStyleOptionComplex {
    return & self.qbase;
  }
}
impl AsRef<QStyleOptionComplex> for QStyleOptionSizeGrip {
  fn as_ref(& self) -> & QStyleOptionComplex {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(int version);
impl /*struct*/ QStyleOptionSizeGrip {
  pub fn New<T: QStyleOptionSizeGrip_New>(value: T) -> QStyleOptionSizeGrip {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSizeGrip_New {
  fn New(self) -> QStyleOptionSizeGrip;
}

  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(int version);
impl<'a> /*trait*/ QStyleOptionSizeGrip_New for (i32) {
  fn New(self) -> QStyleOptionSizeGrip {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionSizeGripC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionSizeGripC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionSizeGrip{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip();
impl<'a> /*trait*/ QStyleOptionSizeGrip_New for () {
  fn New(self) -> QStyleOptionSizeGrip {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionSizeGripC1Ev()};
    unsafe {_ZN20QStyleOptionSizeGripC1Ev(qthis)};
    let rsthis = QStyleOptionSizeGrip{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(const QStyleOptionSizeGrip & other);
impl<'a> /*trait*/ QStyleOptionSizeGrip_New for (&'a QStyleOptionSizeGrip) {
  fn New(self) -> QStyleOptionSizeGrip {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionSizeGripC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionSizeGripC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionSizeGrip{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionViewItem {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionViewItem {
    return QStyleOptionViewItem{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionViewItem {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionViewItem {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem(const QStyleOptionViewItem & other);
impl /*struct*/ QStyleOptionViewItem {
  pub fn New<T: QStyleOptionViewItem_New>(value: T) -> QStyleOptionViewItem {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionViewItem_New {
  fn New(self) -> QStyleOptionViewItem;
}

  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem(const QStyleOptionViewItem & other);
impl<'a> /*trait*/ QStyleOptionViewItem_New for (&'a QStyleOptionViewItem) {
  fn New(self) -> QStyleOptionViewItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionViewItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionViewItemC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionViewItem{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem(int version);
impl<'a> /*trait*/ QStyleOptionViewItem_New for (i32) {
  fn New(self) -> QStyleOptionViewItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionViewItemC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionViewItemC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionViewItem{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem();
impl<'a> /*trait*/ QStyleOptionViewItem_New for () {
  fn New(self) -> QStyleOptionViewItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionViewItemC1Ev()};
    unsafe {_ZN20QStyleOptionViewItemC1Ev(qthis)};
    let rsthis = QStyleOptionViewItem{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionSpinBox {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionSpinBox {
    return QStyleOptionSpinBox{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionSpinBox {
  type Target = QStyleOptionComplex;

  fn deref(&self) -> &QStyleOptionComplex {
    return & self.qbase;
  }
}
impl AsRef<QStyleOptionComplex> for QStyleOptionSpinBox {
  fn as_ref(& self) -> & QStyleOptionComplex {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox();
impl /*struct*/ QStyleOptionSpinBox {
  pub fn New<T: QStyleOptionSpinBox_New>(value: T) -> QStyleOptionSpinBox {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSpinBox_New {
  fn New(self) -> QStyleOptionSpinBox;
}

  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox();
impl<'a> /*trait*/ QStyleOptionSpinBox_New for () {
  fn New(self) -> QStyleOptionSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionSpinBoxC1Ev()};
    unsafe {_ZN19QStyleOptionSpinBoxC1Ev(qthis)};
    let rsthis = QStyleOptionSpinBox{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox(const QStyleOptionSpinBox & other);
impl<'a> /*trait*/ QStyleOptionSpinBox_New for (&'a QStyleOptionSpinBox) {
  fn New(self) -> QStyleOptionSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionSpinBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyleOptionSpinBoxC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionSpinBox{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox(int version);
impl<'a> /*trait*/ QStyleOptionSpinBox_New for (i32) {
  fn New(self) -> QStyleOptionSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionSpinBoxC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN19QStyleOptionSpinBoxC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionSpinBox{/**/qbase: QStyleOptionComplex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionToolBar {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionToolBar {
    return QStyleOptionToolBar{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionToolBar {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionToolBar {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(const QStyleOptionToolBar & other);
impl /*struct*/ QStyleOptionToolBar {
  pub fn New<T: QStyleOptionToolBar_New>(value: T) -> QStyleOptionToolBar {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolBar_New {
  fn New(self) -> QStyleOptionToolBar;
}

  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(const QStyleOptionToolBar & other);
impl<'a> /*trait*/ QStyleOptionToolBar_New for (&'a QStyleOptionToolBar) {
  fn New(self) -> QStyleOptionToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBarC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyleOptionToolBarC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionToolBar{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(int version);
impl<'a> /*trait*/ QStyleOptionToolBar_New for (i32) {
  fn New(self) -> QStyleOptionToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBarC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN19QStyleOptionToolBarC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionToolBar{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar();
impl<'a> /*trait*/ QStyleOptionToolBar_New for () {
  fn New(self) -> QStyleOptionToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBarC1Ev()};
    unsafe {_ZN19QStyleOptionToolBarC1Ev(qthis)};
    let rsthis = QStyleOptionToolBar{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionTabWidgetFrame {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyleOptionTabWidgetFrame {
    return QStyleOptionTabWidgetFrame{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyleOptionTabWidgetFrame {
  type Target = QStyleOption;

  fn deref(&self) -> &QStyleOption {
    return & self.qbase;
  }
}
impl AsRef<QStyleOption> for QStyleOptionTabWidgetFrame {
  fn as_ref(& self) -> & QStyleOption {
    return & self.qbase;
  }
}
  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame & other);
impl /*struct*/ QStyleOptionTabWidgetFrame {
  pub fn New<T: QStyleOptionTabWidgetFrame_New>(value: T) -> QStyleOptionTabWidgetFrame {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTabWidgetFrame_New {
  fn New(self) -> QStyleOptionTabWidgetFrame;
}

  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame & other);
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_New for (&'a QStyleOptionTabWidgetFrame) {
  fn New(self) -> QStyleOptionTabWidgetFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN26QStyleOptionTabWidgetFrameC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionTabWidgetFrame{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(int version);
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_New for (i32) {
  fn New(self) -> QStyleOptionTabWidgetFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN26QStyleOptionTabWidgetFrameC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionTabWidgetFrame{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame();
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_New for () {
  fn New(self) -> QStyleOptionTabWidgetFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC1Ev()};
    unsafe {_ZN26QStyleOptionTabWidgetFrameC1Ev(qthis)};
    let rsthis = QStyleOptionTabWidgetFrame{/**/qbase: QStyleOption::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

