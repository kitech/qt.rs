// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
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
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStyleOptionComboBox_Class_Size() -> c_int;
  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox(const QStyleOptionComboBox & other);
  fn C_ZN20QStyleOptionComboBoxC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox();
  fn C_ZN20QStyleOptionComboBoxC2Ev() -> u64;
  fn QStyleOptionMenuItem_Class_Size() -> c_int;
  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem(const QStyleOptionMenuItem & other);
  fn C_ZN20QStyleOptionMenuItemC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem();
  fn C_ZN20QStyleOptionMenuItemC2Ev() -> u64;
  fn QStyleHintReturnVariant_Class_Size() -> c_int;
  // proto:  void QStyleHintReturnVariant::~QStyleHintReturnVariant();
  fn C_ZN23QStyleHintReturnVariantD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QStyleHintReturnVariant::QStyleHintReturnVariant();
  fn C_ZN23QStyleHintReturnVariantC2Ev() -> u64;
  fn QStyleOptionTitleBar_Class_Size() -> c_int;
  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(const QStyleOptionTitleBar & other);
  fn C_ZN20QStyleOptionTitleBarC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar();
  fn C_ZN20QStyleOptionTitleBarC2Ev() -> u64;
  fn QStyleOptionGraphicsItem_Class_Size() -> c_int;
  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem();
  fn C_ZN24QStyleOptionGraphicsItemC2Ev() -> u64;
  // proto: static qreal QStyleOptionGraphicsItem::levelOfDetailFromTransform(const QTransform & worldTransform);
  fn C_ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform(arg0: *mut c_void) -> c_double;
  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(const QStyleOptionGraphicsItem & other);
  fn C_ZN24QStyleOptionGraphicsItemC2ERKS_(arg0: *mut c_void) -> u64;
  fn QStyleOption_Class_Size() -> c_int;
  // proto:  void QStyleOption::~QStyleOption();
  fn C_ZN12QStyleOptionD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QStyleOption::init(const QWidget * w);
  fn C_ZN12QStyleOption4initEPK7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStyleOption::QStyleOption(const QStyleOption & other);
  fn C_ZN12QStyleOptionC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QStyleOption::QStyleOption(int version, int type);
  fn C_ZN12QStyleOptionC2Eii(arg0: c_int, arg1: c_int) -> u64;
  // proto:  void QStyleOption::initFrom(const QWidget * w);
  fn C_ZN12QStyleOption8initFromEPK7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QStyleOptionDockWidget_Class_Size() -> c_int;
  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget();
  fn C_ZN22QStyleOptionDockWidgetC2Ev() -> u64;
  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(const QStyleOptionDockWidget & other);
  fn C_ZN22QStyleOptionDockWidgetC2ERKS_(arg0: *mut c_void) -> u64;
  fn QStyleOptionProgressBar_Class_Size() -> c_int;
  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(const QStyleOptionProgressBar & other);
  fn C_ZN23QStyleOptionProgressBarC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar();
  fn C_ZN23QStyleOptionProgressBarC2Ev() -> u64;
  fn QStyleOptionSlider_Class_Size() -> c_int;
  // proto:  void QStyleOptionSlider::QStyleOptionSlider(const QStyleOptionSlider & other);
  fn C_ZN18QStyleOptionSliderC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QStyleOptionSlider::QStyleOptionSlider();
  fn C_ZN18QStyleOptionSliderC2Ev() -> u64;
  fn QStyleOptionFrame_Class_Size() -> c_int;
  // proto:  void QStyleOptionFrame::QStyleOptionFrame();
  fn C_ZN17QStyleOptionFrameC2Ev() -> u64;
  // proto:  void QStyleOptionFrame::QStyleOptionFrame(const QStyleOptionFrame & other);
  fn C_ZN17QStyleOptionFrameC2ERKS_(arg0: *mut c_void) -> u64;
  fn QStyleOptionComplex_Class_Size() -> c_int;
  // proto:  void QStyleOptionComplex::QStyleOptionComplex(int version, int type);
  fn C_ZN19QStyleOptionComplexC2Eii(arg0: c_int, arg1: c_int) -> u64;
  // proto:  void QStyleOptionComplex::QStyleOptionComplex(const QStyleOptionComplex & other);
  fn C_ZN19QStyleOptionComplexC2ERKS_(arg0: *mut c_void) -> u64;
  fn QStyleHintReturn_Class_Size() -> c_int;
  // proto:  void QStyleHintReturn::~QStyleHintReturn();
  fn C_ZN16QStyleHintReturnD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QStyleHintReturn::QStyleHintReturn(int version, int type);
  fn C_ZN16QStyleHintReturnC2Eii(arg0: c_int, arg1: c_int) -> u64;
  fn QStyleOptionHeader_Class_Size() -> c_int;
  // proto:  void QStyleOptionHeader::QStyleOptionHeader();
  fn C_ZN18QStyleOptionHeaderC2Ev() -> u64;
  // proto:  void QStyleOptionHeader::QStyleOptionHeader(const QStyleOptionHeader & other);
  fn C_ZN18QStyleOptionHeaderC2ERKS_(arg0: *mut c_void) -> u64;
  fn QStyleOptionToolBox_Class_Size() -> c_int;
  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox();
  fn C_ZN19QStyleOptionToolBoxC2Ev() -> u64;
  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox(const QStyleOptionToolBox & other);
  fn C_ZN19QStyleOptionToolBoxC2ERKS_(arg0: *mut c_void) -> u64;
  fn QStyleOptionFocusRect_Class_Size() -> c_int;
  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect();
  fn C_ZN21QStyleOptionFocusRectC2Ev() -> u64;
  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(const QStyleOptionFocusRect & other);
  fn C_ZN21QStyleOptionFocusRectC2ERKS_(arg0: *mut c_void) -> u64;
  fn QStyleOptionGroupBox_Class_Size() -> c_int;
  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(const QStyleOptionGroupBox & other);
  fn C_ZN20QStyleOptionGroupBoxC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox();
  fn C_ZN20QStyleOptionGroupBoxC2Ev() -> u64;
  fn QStyleOptionTab_Class_Size() -> c_int;
  // proto:  void QStyleOptionTab::QStyleOptionTab(const QStyleOptionTab & other);
  fn C_ZN15QStyleOptionTabC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QStyleOptionTab::QStyleOptionTab();
  fn C_ZN15QStyleOptionTabC2Ev() -> u64;
  fn QStyleOptionTabBarBase_Class_Size() -> c_int;
  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase();
  fn C_ZN22QStyleOptionTabBarBaseC2Ev() -> u64;
  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(const QStyleOptionTabBarBase & other);
  fn C_ZN22QStyleOptionTabBarBaseC2ERKS_(arg0: *mut c_void) -> u64;
  fn QStyleOptionRubberBand_Class_Size() -> c_int;
  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand();
  fn C_ZN22QStyleOptionRubberBandC2Ev() -> u64;
  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(const QStyleOptionRubberBand & other);
  fn C_ZN22QStyleOptionRubberBandC2ERKS_(arg0: *mut c_void) -> u64;
  fn QStyleOptionButton_Class_Size() -> c_int;
  // proto:  void QStyleOptionButton::QStyleOptionButton();
  fn C_ZN18QStyleOptionButtonC2Ev() -> u64;
  // proto:  void QStyleOptionButton::QStyleOptionButton(const QStyleOptionButton & other);
  fn C_ZN18QStyleOptionButtonC2ERKS_(arg0: *mut c_void) -> u64;
  fn QStyleHintReturnMask_Class_Size() -> c_int;
  // proto:  void QStyleHintReturnMask::QStyleHintReturnMask();
  fn C_ZN20QStyleHintReturnMaskC2Ev() -> u64;
  // proto:  void QStyleHintReturnMask::~QStyleHintReturnMask();
  fn C_ZN20QStyleHintReturnMaskD2Ev(qthis: u64 /* *mut c_void*/);
  fn QStyleOptionToolButton_Class_Size() -> c_int;
  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton();
  fn C_ZN22QStyleOptionToolButtonC2Ev() -> u64;
  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton(const QStyleOptionToolButton & other);
  fn C_ZN22QStyleOptionToolButtonC2ERKS_(arg0: *mut c_void) -> u64;
  fn QStyleOptionSizeGrip_Class_Size() -> c_int;
  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip();
  fn C_ZN20QStyleOptionSizeGripC2Ev() -> u64;
  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(const QStyleOptionSizeGrip & other);
  fn C_ZN20QStyleOptionSizeGripC2ERKS_(arg0: *mut c_void) -> u64;
  fn QStyleOptionViewItem_Class_Size() -> c_int;
  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem(const QStyleOptionViewItem & other);
  fn C_ZN20QStyleOptionViewItemC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem();
  fn C_ZN20QStyleOptionViewItemC2Ev() -> u64;
  fn QStyleOptionSpinBox_Class_Size() -> c_int;
  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox();
  fn C_ZN19QStyleOptionSpinBoxC2Ev() -> u64;
  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox(const QStyleOptionSpinBox & other);
  fn C_ZN19QStyleOptionSpinBoxC2ERKS_(arg0: *mut c_void) -> u64;
  fn QStyleOptionToolBar_Class_Size() -> c_int;
  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(const QStyleOptionToolBar & other);
  fn C_ZN19QStyleOptionToolBarC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar();
  fn C_ZN19QStyleOptionToolBarC2Ev() -> u64;
  fn QStyleOptionTabWidgetFrame_Class_Size() -> c_int;
  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame & other);
  fn C_ZN26QStyleOptionTabWidgetFrameC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame();
  fn C_ZN26QStyleOptionTabWidgetFrameC2Ev() -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QStyleOptionComboBox)=1
#[derive(Default)]
pub struct QStyleOptionComboBox {
  qbase: QStyleOptionComplex,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionMenuItem)=1
#[derive(Default)]
pub struct QStyleOptionMenuItem {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleHintReturnVariant)=24
#[derive(Default)]
pub struct QStyleHintReturnVariant {
  qbase: QStyleHintReturn,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionTitleBar)=1
#[derive(Default)]
pub struct QStyleOptionTitleBar {
  qbase: QStyleOptionComplex,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionGraphicsItem)=1
#[derive(Default)]
pub struct QStyleOptionGraphicsItem {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOption)=1
#[derive(Default)]
pub struct QStyleOption {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionDockWidget)=1
#[derive(Default)]
pub struct QStyleOptionDockWidget {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionProgressBar)=1
#[derive(Default)]
pub struct QStyleOptionProgressBar {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionSlider)=1
#[derive(Default)]
pub struct QStyleOptionSlider {
  qbase: QStyleOptionComplex,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionFrame)=1
#[derive(Default)]
pub struct QStyleOptionFrame {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionComplex)=1
#[derive(Default)]
pub struct QStyleOptionComplex {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleHintReturn)=8
#[derive(Default)]
pub struct QStyleHintReturn {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionHeader)=1
#[derive(Default)]
pub struct QStyleOptionHeader {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionToolBox)=1
#[derive(Default)]
pub struct QStyleOptionToolBox {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionFocusRect)=1
#[derive(Default)]
pub struct QStyleOptionFocusRect {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionGroupBox)=1
#[derive(Default)]
pub struct QStyleOptionGroupBox {
  qbase: QStyleOptionComplex,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionTab)=1
#[derive(Default)]
pub struct QStyleOptionTab {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionTabBarBase)=1
#[derive(Default)]
pub struct QStyleOptionTabBarBase {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionRubberBand)=1
#[derive(Default)]
pub struct QStyleOptionRubberBand {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionButton)=1
#[derive(Default)]
pub struct QStyleOptionButton {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleHintReturnMask)=16
#[derive(Default)]
pub struct QStyleHintReturnMask {
  qbase: QStyleHintReturn,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionToolButton)=1
#[derive(Default)]
pub struct QStyleOptionToolButton {
  qbase: QStyleOptionComplex,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionSizeGrip)=1
#[derive(Default)]
pub struct QStyleOptionSizeGrip {
  qbase: QStyleOptionComplex,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionViewItem)=1
#[derive(Default)]
pub struct QStyleOptionViewItem {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionSpinBox)=1
#[derive(Default)]
pub struct QStyleOptionSpinBox {
  qbase: QStyleOptionComplex,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionToolBar)=1
#[derive(Default)]
pub struct QStyleOptionToolBar {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStyleOptionTabWidgetFrame)=1
#[derive(Default)]
pub struct QStyleOptionTabWidgetFrame {
  qbase: QStyleOption,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStyleOptionComboBox {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionComboBox {
    return QStyleOptionComboBox{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionComboBox_new>(value: T) -> QStyleOptionComboBox {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionComboBox_new {
  fn new(self) -> QStyleOptionComboBox;
}

  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox(const QStyleOptionComboBox & other);
impl<'a> /*trait*/ QStyleOptionComboBox_new for (&'a QStyleOptionComboBox) {
  fn new(self) -> QStyleOptionComboBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionComboBoxC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionComboBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN20QStyleOptionComboBoxC2ERKS_(arg0)};
    let rsthis = QStyleOptionComboBox{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox();
impl<'a> /*trait*/ QStyleOptionComboBox_new for () {
  fn new(self) -> QStyleOptionComboBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionComboBoxC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionComboBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN20QStyleOptionComboBoxC2Ev()};
    let rsthis = QStyleOptionComboBox{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionMenuItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionMenuItem {
    return QStyleOptionMenuItem{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionMenuItem_new>(value: T) -> QStyleOptionMenuItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionMenuItem_new {
  fn new(self) -> QStyleOptionMenuItem;
}

  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem(const QStyleOptionMenuItem & other);
impl<'a> /*trait*/ QStyleOptionMenuItem_new for (&'a QStyleOptionMenuItem) {
  fn new(self) -> QStyleOptionMenuItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionMenuItemC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionMenuItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN20QStyleOptionMenuItemC2ERKS_(arg0)};
    let rsthis = QStyleOptionMenuItem{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem();
impl<'a> /*trait*/ QStyleOptionMenuItem_new for () {
  fn new(self) -> QStyleOptionMenuItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionMenuItemC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionMenuItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN20QStyleOptionMenuItemC2Ev()};
    let rsthis = QStyleOptionMenuItem{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleHintReturnVariant {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleHintReturnVariant {
    return QStyleHintReturnVariant{qbase: QStyleHintReturn::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn free<RetType, T: QStyleHintReturnVariant_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStyleHintReturnVariant_free<RetType> {
  fn free(self , rsthis: & QStyleHintReturnVariant) -> RetType;
}

  // proto:  void QStyleHintReturnVariant::~QStyleHintReturnVariant();
impl<'a> /*trait*/ QStyleHintReturnVariant_free<()> for () {
  fn free(self , rsthis: & QStyleHintReturnVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleHintReturnVariantD2Ev()};
     unsafe {C_ZN23QStyleHintReturnVariantD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyleHintReturnVariant::QStyleHintReturnVariant();
impl /*struct*/ QStyleHintReturnVariant {
  pub fn new<T: QStyleHintReturnVariant_new>(value: T) -> QStyleHintReturnVariant {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturnVariant_new {
  fn new(self) -> QStyleHintReturnVariant;
}

  // proto:  void QStyleHintReturnVariant::QStyleHintReturnVariant();
impl<'a> /*trait*/ QStyleHintReturnVariant_new for () {
  fn new(self) -> QStyleHintReturnVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleHintReturnVariantC2Ev()};
    let ctysz: c_int = unsafe{QStyleHintReturnVariant_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN23QStyleHintReturnVariantC2Ev()};
    let rsthis = QStyleHintReturnVariant{qbase: QStyleHintReturn::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionTitleBar {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionTitleBar {
    return QStyleOptionTitleBar{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(const QStyleOptionTitleBar & other);
impl /*struct*/ QStyleOptionTitleBar {
  pub fn new<T: QStyleOptionTitleBar_new>(value: T) -> QStyleOptionTitleBar {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTitleBar_new {
  fn new(self) -> QStyleOptionTitleBar;
}

  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(const QStyleOptionTitleBar & other);
impl<'a> /*trait*/ QStyleOptionTitleBar_new for (&'a QStyleOptionTitleBar) {
  fn new(self) -> QStyleOptionTitleBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionTitleBarC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionTitleBar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN20QStyleOptionTitleBarC2ERKS_(arg0)};
    let rsthis = QStyleOptionTitleBar{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar();
impl<'a> /*trait*/ QStyleOptionTitleBar_new for () {
  fn new(self) -> QStyleOptionTitleBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionTitleBarC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionTitleBar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN20QStyleOptionTitleBarC2Ev()};
    let rsthis = QStyleOptionTitleBar{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionGraphicsItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionGraphicsItem {
    return QStyleOptionGraphicsItem{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionGraphicsItem_new>(value: T) -> QStyleOptionGraphicsItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionGraphicsItem_new {
  fn new(self) -> QStyleOptionGraphicsItem;
}

  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem();
impl<'a> /*trait*/ QStyleOptionGraphicsItem_new for () {
  fn new(self) -> QStyleOptionGraphicsItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItemC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionGraphicsItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN24QStyleOptionGraphicsItemC2Ev()};
    let rsthis = QStyleOptionGraphicsItem{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform(arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(const QStyleOptionGraphicsItem & other);
impl<'a> /*trait*/ QStyleOptionGraphicsItem_new for (&'a QStyleOptionGraphicsItem) {
  fn new(self) -> QStyleOptionGraphicsItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItemC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionGraphicsItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN24QStyleOptionGraphicsItemC2ERKS_(arg0)};
    let rsthis = QStyleOptionGraphicsItem{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOption {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOption {
    return QStyleOption{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QStyleOption::~QStyleOption();
impl /*struct*/ QStyleOption {
  pub fn free<RetType, T: QStyleOption_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStyleOption_free<RetType> {
  fn free(self , rsthis: & QStyleOption) -> RetType;
}

  // proto:  void QStyleOption::~QStyleOption();
impl<'a> /*trait*/ QStyleOption_free<()> for () {
  fn free(self , rsthis: & QStyleOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOptionD2Ev()};
     unsafe {C_ZN12QStyleOptionD2Ev(rsthis.qclsinst)};
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
     unsafe {C_ZN12QStyleOption4initEPK7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyleOption::QStyleOption(const QStyleOption & other);
impl /*struct*/ QStyleOption {
  pub fn new<T: QStyleOption_new>(value: T) -> QStyleOption {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOption_new {
  fn new(self) -> QStyleOption;
}

  // proto:  void QStyleOption::QStyleOption(const QStyleOption & other);
impl<'a> /*trait*/ QStyleOption_new for (&'a QStyleOption) {
  fn new(self) -> QStyleOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOptionC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOption_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QStyleOptionC2ERKS_(arg0)};
    let rsthis = QStyleOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOption::QStyleOption(int version, int type);
impl<'a> /*trait*/ QStyleOption_new for (i32, i32) {
  fn new(self) -> QStyleOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOptionC2Eii()};
    let ctysz: c_int = unsafe{QStyleOption_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let qthis: u64 = unsafe {C_ZN12QStyleOptionC2Eii(arg0, arg1)};
    let rsthis = QStyleOption{qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZN12QStyleOption8initFromEPK7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStyleOptionDockWidget {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionDockWidget {
    return QStyleOptionDockWidget{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionDockWidget_new>(value: T) -> QStyleOptionDockWidget {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionDockWidget_new {
  fn new(self) -> QStyleOptionDockWidget;
}

  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget();
impl<'a> /*trait*/ QStyleOptionDockWidget_new for () {
  fn new(self) -> QStyleOptionDockWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionDockWidgetC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionDockWidget_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN22QStyleOptionDockWidgetC2Ev()};
    let rsthis = QStyleOptionDockWidget{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(const QStyleOptionDockWidget & other);
impl<'a> /*trait*/ QStyleOptionDockWidget_new for (&'a QStyleOptionDockWidget) {
  fn new(self) -> QStyleOptionDockWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionDockWidgetC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionDockWidget_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN22QStyleOptionDockWidgetC2ERKS_(arg0)};
    let rsthis = QStyleOptionDockWidget{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionProgressBar {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionProgressBar {
    return QStyleOptionProgressBar{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionProgressBar_new>(value: T) -> QStyleOptionProgressBar {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionProgressBar_new {
  fn new(self) -> QStyleOptionProgressBar;
}

  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(const QStyleOptionProgressBar & other);
impl<'a> /*trait*/ QStyleOptionProgressBar_new for (&'a QStyleOptionProgressBar) {
  fn new(self) -> QStyleOptionProgressBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleOptionProgressBarC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionProgressBar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN23QStyleOptionProgressBarC2ERKS_(arg0)};
    let rsthis = QStyleOptionProgressBar{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar();
impl<'a> /*trait*/ QStyleOptionProgressBar_new for () {
  fn new(self) -> QStyleOptionProgressBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleOptionProgressBarC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionProgressBar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN23QStyleOptionProgressBarC2Ev()};
    let rsthis = QStyleOptionProgressBar{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionSlider {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionSlider {
    return QStyleOptionSlider{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionSlider_new>(value: T) -> QStyleOptionSlider {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSlider_new {
  fn new(self) -> QStyleOptionSlider;
}

  // proto:  void QStyleOptionSlider::QStyleOptionSlider(const QStyleOptionSlider & other);
impl<'a> /*trait*/ QStyleOptionSlider_new for (&'a QStyleOptionSlider) {
  fn new(self) -> QStyleOptionSlider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionSliderC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionSlider_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QStyleOptionSliderC2ERKS_(arg0)};
    let rsthis = QStyleOptionSlider{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSlider::QStyleOptionSlider();
impl<'a> /*trait*/ QStyleOptionSlider_new for () {
  fn new(self) -> QStyleOptionSlider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionSliderC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionSlider_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN18QStyleOptionSliderC2Ev()};
    let rsthis = QStyleOptionSlider{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionFrame {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionFrame {
    return QStyleOptionFrame{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionFrame_new>(value: T) -> QStyleOptionFrame {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionFrame_new {
  fn new(self) -> QStyleOptionFrame;
}

  // proto:  void QStyleOptionFrame::QStyleOptionFrame();
impl<'a> /*trait*/ QStyleOptionFrame_new for () {
  fn new(self) -> QStyleOptionFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QStyleOptionFrameC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionFrame_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN17QStyleOptionFrameC2Ev()};
    let rsthis = QStyleOptionFrame{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFrame::QStyleOptionFrame(const QStyleOptionFrame & other);
impl<'a> /*trait*/ QStyleOptionFrame_new for (&'a QStyleOptionFrame) {
  fn new(self) -> QStyleOptionFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QStyleOptionFrameC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionFrame_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QStyleOptionFrameC2ERKS_(arg0)};
    let rsthis = QStyleOptionFrame{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionComplex {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionComplex {
    return QStyleOptionComplex{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionComplex_new>(value: T) -> QStyleOptionComplex {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionComplex_new {
  fn new(self) -> QStyleOptionComplex;
}

  // proto:  void QStyleOptionComplex::QStyleOptionComplex(int version, int type);
impl<'a> /*trait*/ QStyleOptionComplex_new for (i32, i32) {
  fn new(self) -> QStyleOptionComplex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionComplexC2Eii()};
    let ctysz: c_int = unsafe{QStyleOptionComplex_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let qthis: u64 = unsafe {C_ZN19QStyleOptionComplexC2Eii(arg0, arg1)};
    let rsthis = QStyleOptionComplex{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionComplex::QStyleOptionComplex(const QStyleOptionComplex & other);
impl<'a> /*trait*/ QStyleOptionComplex_new for (&'a QStyleOptionComplex) {
  fn new(self) -> QStyleOptionComplex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionComplexC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionComplex_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QStyleOptionComplexC2ERKS_(arg0)};
    let rsthis = QStyleOptionComplex{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleHintReturn {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleHintReturn {
    return QStyleHintReturn{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QStyleHintReturn::~QStyleHintReturn();
impl /*struct*/ QStyleHintReturn {
  pub fn free<RetType, T: QStyleHintReturn_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStyleHintReturn_free<RetType> {
  fn free(self , rsthis: & QStyleHintReturn) -> RetType;
}

  // proto:  void QStyleHintReturn::~QStyleHintReturn();
impl<'a> /*trait*/ QStyleHintReturn_free<()> for () {
  fn free(self , rsthis: & QStyleHintReturn) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStyleHintReturnD2Ev()};
     unsafe {C_ZN16QStyleHintReturnD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyleHintReturn::QStyleHintReturn(int version, int type);
impl /*struct*/ QStyleHintReturn {
  pub fn new<T: QStyleHintReturn_new>(value: T) -> QStyleHintReturn {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturn_new {
  fn new(self) -> QStyleHintReturn;
}

  // proto:  void QStyleHintReturn::QStyleHintReturn(int version, int type);
impl<'a> /*trait*/ QStyleHintReturn_new for (i32, i32) {
  fn new(self) -> QStyleHintReturn {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStyleHintReturnC2Eii()};
    let ctysz: c_int = unsafe{QStyleHintReturn_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let qthis: u64 = unsafe {C_ZN16QStyleHintReturnC2Eii(arg0, arg1)};
    let rsthis = QStyleHintReturn{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionHeader {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionHeader {
    return QStyleOptionHeader{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionHeader_new>(value: T) -> QStyleOptionHeader {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionHeader_new {
  fn new(self) -> QStyleOptionHeader;
}

  // proto:  void QStyleOptionHeader::QStyleOptionHeader();
impl<'a> /*trait*/ QStyleOptionHeader_new for () {
  fn new(self) -> QStyleOptionHeader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionHeaderC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionHeader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN18QStyleOptionHeaderC2Ev()};
    let rsthis = QStyleOptionHeader{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionHeader::QStyleOptionHeader(const QStyleOptionHeader & other);
impl<'a> /*trait*/ QStyleOptionHeader_new for (&'a QStyleOptionHeader) {
  fn new(self) -> QStyleOptionHeader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionHeaderC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionHeader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QStyleOptionHeaderC2ERKS_(arg0)};
    let rsthis = QStyleOptionHeader{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionToolBox {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionToolBox {
    return QStyleOptionToolBox{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionToolBox_new>(value: T) -> QStyleOptionToolBox {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolBox_new {
  fn new(self) -> QStyleOptionToolBox;
}

  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox();
impl<'a> /*trait*/ QStyleOptionToolBox_new for () {
  fn new(self) -> QStyleOptionToolBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBoxC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionToolBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN19QStyleOptionToolBoxC2Ev()};
    let rsthis = QStyleOptionToolBox{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox(const QStyleOptionToolBox & other);
impl<'a> /*trait*/ QStyleOptionToolBox_new for (&'a QStyleOptionToolBox) {
  fn new(self) -> QStyleOptionToolBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBoxC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionToolBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QStyleOptionToolBoxC2ERKS_(arg0)};
    let rsthis = QStyleOptionToolBox{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionFocusRect {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionFocusRect {
    return QStyleOptionFocusRect{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect();
impl /*struct*/ QStyleOptionFocusRect {
  pub fn new<T: QStyleOptionFocusRect_new>(value: T) -> QStyleOptionFocusRect {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionFocusRect_new {
  fn new(self) -> QStyleOptionFocusRect;
}

  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect();
impl<'a> /*trait*/ QStyleOptionFocusRect_new for () {
  fn new(self) -> QStyleOptionFocusRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QStyleOptionFocusRectC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionFocusRect_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN21QStyleOptionFocusRectC2Ev()};
    let rsthis = QStyleOptionFocusRect{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(const QStyleOptionFocusRect & other);
impl<'a> /*trait*/ QStyleOptionFocusRect_new for (&'a QStyleOptionFocusRect) {
  fn new(self) -> QStyleOptionFocusRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QStyleOptionFocusRectC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionFocusRect_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN21QStyleOptionFocusRectC2ERKS_(arg0)};
    let rsthis = QStyleOptionFocusRect{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionGroupBox {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionGroupBox {
    return QStyleOptionGroupBox{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(const QStyleOptionGroupBox & other);
impl /*struct*/ QStyleOptionGroupBox {
  pub fn new<T: QStyleOptionGroupBox_new>(value: T) -> QStyleOptionGroupBox {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionGroupBox_new {
  fn new(self) -> QStyleOptionGroupBox;
}

  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(const QStyleOptionGroupBox & other);
impl<'a> /*trait*/ QStyleOptionGroupBox_new for (&'a QStyleOptionGroupBox) {
  fn new(self) -> QStyleOptionGroupBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionGroupBoxC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionGroupBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN20QStyleOptionGroupBoxC2ERKS_(arg0)};
    let rsthis = QStyleOptionGroupBox{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox();
impl<'a> /*trait*/ QStyleOptionGroupBox_new for () {
  fn new(self) -> QStyleOptionGroupBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionGroupBoxC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionGroupBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN20QStyleOptionGroupBoxC2Ev()};
    let rsthis = QStyleOptionGroupBox{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionTab {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionTab {
    return QStyleOptionTab{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionTab_new>(value: T) -> QStyleOptionTab {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTab_new {
  fn new(self) -> QStyleOptionTab;
}

  // proto:  void QStyleOptionTab::QStyleOptionTab(const QStyleOptionTab & other);
impl<'a> /*trait*/ QStyleOptionTab_new for (&'a QStyleOptionTab) {
  fn new(self) -> QStyleOptionTab {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStyleOptionTabC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionTab_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN15QStyleOptionTabC2ERKS_(arg0)};
    let rsthis = QStyleOptionTab{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTab::QStyleOptionTab();
impl<'a> /*trait*/ QStyleOptionTab_new for () {
  fn new(self) -> QStyleOptionTab {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStyleOptionTabC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionTab_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN15QStyleOptionTabC2Ev()};
    let rsthis = QStyleOptionTab{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionTabBarBase {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionTabBarBase {
    return QStyleOptionTabBarBase{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionTabBarBase_new>(value: T) -> QStyleOptionTabBarBase {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTabBarBase_new {
  fn new(self) -> QStyleOptionTabBarBase;
}

  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase();
impl<'a> /*trait*/ QStyleOptionTabBarBase_new for () {
  fn new(self) -> QStyleOptionTabBarBase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionTabBarBaseC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionTabBarBase_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN22QStyleOptionTabBarBaseC2Ev()};
    let rsthis = QStyleOptionTabBarBase{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(const QStyleOptionTabBarBase & other);
impl<'a> /*trait*/ QStyleOptionTabBarBase_new for (&'a QStyleOptionTabBarBase) {
  fn new(self) -> QStyleOptionTabBarBase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionTabBarBaseC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionTabBarBase_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN22QStyleOptionTabBarBaseC2ERKS_(arg0)};
    let rsthis = QStyleOptionTabBarBase{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionRubberBand {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionRubberBand {
    return QStyleOptionRubberBand{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand();
impl /*struct*/ QStyleOptionRubberBand {
  pub fn new<T: QStyleOptionRubberBand_new>(value: T) -> QStyleOptionRubberBand {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionRubberBand_new {
  fn new(self) -> QStyleOptionRubberBand;
}

  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand();
impl<'a> /*trait*/ QStyleOptionRubberBand_new for () {
  fn new(self) -> QStyleOptionRubberBand {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionRubberBandC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionRubberBand_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN22QStyleOptionRubberBandC2Ev()};
    let rsthis = QStyleOptionRubberBand{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(const QStyleOptionRubberBand & other);
impl<'a> /*trait*/ QStyleOptionRubberBand_new for (&'a QStyleOptionRubberBand) {
  fn new(self) -> QStyleOptionRubberBand {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionRubberBandC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionRubberBand_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN22QStyleOptionRubberBandC2ERKS_(arg0)};
    let rsthis = QStyleOptionRubberBand{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionButton {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionButton {
    return QStyleOptionButton{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  // proto:  void QStyleOptionButton::QStyleOptionButton();
impl /*struct*/ QStyleOptionButton {
  pub fn new<T: QStyleOptionButton_new>(value: T) -> QStyleOptionButton {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionButton_new {
  fn new(self) -> QStyleOptionButton;
}

  // proto:  void QStyleOptionButton::QStyleOptionButton();
impl<'a> /*trait*/ QStyleOptionButton_new for () {
  fn new(self) -> QStyleOptionButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionButtonC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN18QStyleOptionButtonC2Ev()};
    let rsthis = QStyleOptionButton{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionButton::QStyleOptionButton(const QStyleOptionButton & other);
impl<'a> /*trait*/ QStyleOptionButton_new for (&'a QStyleOptionButton) {
  fn new(self) -> QStyleOptionButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionButtonC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QStyleOptionButtonC2ERKS_(arg0)};
    let rsthis = QStyleOptionButton{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleHintReturnMask {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleHintReturnMask {
    return QStyleHintReturnMask{qbase: QStyleHintReturn::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleHintReturnMask_new>(value: T) -> QStyleHintReturnMask {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturnMask_new {
  fn new(self) -> QStyleHintReturnMask;
}

  // proto:  void QStyleHintReturnMask::QStyleHintReturnMask();
impl<'a> /*trait*/ QStyleHintReturnMask_new for () {
  fn new(self) -> QStyleHintReturnMask {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleHintReturnMaskC2Ev()};
    let ctysz: c_int = unsafe{QStyleHintReturnMask_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN20QStyleHintReturnMaskC2Ev()};
    let rsthis = QStyleHintReturnMask{qbase: QStyleHintReturn::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleHintReturnMask::~QStyleHintReturnMask();
impl /*struct*/ QStyleHintReturnMask {
  pub fn free<RetType, T: QStyleHintReturnMask_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStyleHintReturnMask_free<RetType> {
  fn free(self , rsthis: & QStyleHintReturnMask) -> RetType;
}

  // proto:  void QStyleHintReturnMask::~QStyleHintReturnMask();
impl<'a> /*trait*/ QStyleHintReturnMask_free<()> for () {
  fn free(self , rsthis: & QStyleHintReturnMask) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleHintReturnMaskD2Ev()};
     unsafe {C_ZN20QStyleHintReturnMaskD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStyleOptionToolButton {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionToolButton {
    return QStyleOptionToolButton{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton();
impl /*struct*/ QStyleOptionToolButton {
  pub fn new<T: QStyleOptionToolButton_new>(value: T) -> QStyleOptionToolButton {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolButton_new {
  fn new(self) -> QStyleOptionToolButton;
}

  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton();
impl<'a> /*trait*/ QStyleOptionToolButton_new for () {
  fn new(self) -> QStyleOptionToolButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionToolButtonC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionToolButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN22QStyleOptionToolButtonC2Ev()};
    let rsthis = QStyleOptionToolButton{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton(const QStyleOptionToolButton & other);
impl<'a> /*trait*/ QStyleOptionToolButton_new for (&'a QStyleOptionToolButton) {
  fn new(self) -> QStyleOptionToolButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionToolButtonC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionToolButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN22QStyleOptionToolButtonC2ERKS_(arg0)};
    let rsthis = QStyleOptionToolButton{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionSizeGrip {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionSizeGrip {
    return QStyleOptionSizeGrip{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip();
impl /*struct*/ QStyleOptionSizeGrip {
  pub fn new<T: QStyleOptionSizeGrip_new>(value: T) -> QStyleOptionSizeGrip {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSizeGrip_new {
  fn new(self) -> QStyleOptionSizeGrip;
}

  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip();
impl<'a> /*trait*/ QStyleOptionSizeGrip_new for () {
  fn new(self) -> QStyleOptionSizeGrip {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionSizeGripC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionSizeGrip_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN20QStyleOptionSizeGripC2Ev()};
    let rsthis = QStyleOptionSizeGrip{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(const QStyleOptionSizeGrip & other);
impl<'a> /*trait*/ QStyleOptionSizeGrip_new for (&'a QStyleOptionSizeGrip) {
  fn new(self) -> QStyleOptionSizeGrip {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionSizeGripC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionSizeGrip_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN20QStyleOptionSizeGripC2ERKS_(arg0)};
    let rsthis = QStyleOptionSizeGrip{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionViewItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionViewItem {
    return QStyleOptionViewItem{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionViewItem_new>(value: T) -> QStyleOptionViewItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionViewItem_new {
  fn new(self) -> QStyleOptionViewItem;
}

  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem(const QStyleOptionViewItem & other);
impl<'a> /*trait*/ QStyleOptionViewItem_new for (&'a QStyleOptionViewItem) {
  fn new(self) -> QStyleOptionViewItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionViewItemC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionViewItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN20QStyleOptionViewItemC2ERKS_(arg0)};
    let rsthis = QStyleOptionViewItem{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem();
impl<'a> /*trait*/ QStyleOptionViewItem_new for () {
  fn new(self) -> QStyleOptionViewItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionViewItemC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionViewItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN20QStyleOptionViewItemC2Ev()};
    let rsthis = QStyleOptionViewItem{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionSpinBox {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionSpinBox {
    return QStyleOptionSpinBox{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionSpinBox_new>(value: T) -> QStyleOptionSpinBox {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSpinBox_new {
  fn new(self) -> QStyleOptionSpinBox;
}

  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox();
impl<'a> /*trait*/ QStyleOptionSpinBox_new for () {
  fn new(self) -> QStyleOptionSpinBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionSpinBoxC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionSpinBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN19QStyleOptionSpinBoxC2Ev()};
    let rsthis = QStyleOptionSpinBox{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox(const QStyleOptionSpinBox & other);
impl<'a> /*trait*/ QStyleOptionSpinBox_new for (&'a QStyleOptionSpinBox) {
  fn new(self) -> QStyleOptionSpinBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionSpinBoxC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionSpinBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QStyleOptionSpinBoxC2ERKS_(arg0)};
    let rsthis = QStyleOptionSpinBox{qbase: QStyleOptionComplex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionToolBar {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionToolBar {
    return QStyleOptionToolBar{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionToolBar_new>(value: T) -> QStyleOptionToolBar {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolBar_new {
  fn new(self) -> QStyleOptionToolBar;
}

  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(const QStyleOptionToolBar & other);
impl<'a> /*trait*/ QStyleOptionToolBar_new for (&'a QStyleOptionToolBar) {
  fn new(self) -> QStyleOptionToolBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBarC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionToolBar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QStyleOptionToolBarC2ERKS_(arg0)};
    let rsthis = QStyleOptionToolBar{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar();
impl<'a> /*trait*/ QStyleOptionToolBar_new for () {
  fn new(self) -> QStyleOptionToolBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBarC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionToolBar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN19QStyleOptionToolBarC2Ev()};
    let rsthis = QStyleOptionToolBar{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionTabWidgetFrame {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleOptionTabWidgetFrame {
    return QStyleOptionTabWidgetFrame{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStyleOptionTabWidgetFrame_new>(value: T) -> QStyleOptionTabWidgetFrame {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTabWidgetFrame_new {
  fn new(self) -> QStyleOptionTabWidgetFrame;
}

  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame & other);
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_new for (&'a QStyleOptionTabWidgetFrame) {
  fn new(self) -> QStyleOptionTabWidgetFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC2ERKS_()};
    let ctysz: c_int = unsafe{QStyleOptionTabWidgetFrame_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN26QStyleOptionTabWidgetFrameC2ERKS_(arg0)};
    let rsthis = QStyleOptionTabWidgetFrame{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame();
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_new for () {
  fn new(self) -> QStyleOptionTabWidgetFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC2Ev()};
    let ctysz: c_int = unsafe{QStyleOptionTabWidgetFrame_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN26QStyleOptionTabWidgetFrameC2Ev()};
    let rsthis = QStyleOptionTabWidgetFrame{qbase: QStyleOption::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

